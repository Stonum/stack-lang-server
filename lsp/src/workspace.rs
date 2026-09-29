use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use dashmap::DashMap;
use ini::Ini;
use line_index::{LineCol, LineIndex};
use log::info;
use serde_json::{Value, json};
use thiserror::Error;
use walkdir::WalkDir;

use lsp_definition::{
    CodeSymbolDefinition as _, DefinitionKind, LinkIndex, LocationDefinition as _, SemanticInfo,
    StringLowerCase, Symbol, Usage, get_completion, get_declaration, get_hover, get_lens,
    get_project_hover, get_reference, get_signatures, get_symbols, handler_resource,
};
use mlang_core::{AnyMCoreDefinition, load_core_api};
use mlang_parser::parse;
use mlang_semantic::{
    AnyMDefinition, SemanticModel, identifier_for_completion, identifier_for_offset,
    identifier_for_signature_help, semantics,
};
use mlang_syntax::MFileSource;
use xml_semantic::{
    RxDefinition, RxLink, RxLinkKind, RxSemanticModel, rx_expression_at, rx_identifier_for_offset,
    rx_semantics,
};
use xml_syntax::{TextSize, XmlFileSource, XmlSyntaxNode};

use tokio::runtime::Handle;
use tokio::sync::{OwnedRwLockReadGuard, RwLock, Semaphore};
use tokio::task::JoinError;

use tower_lsp::lsp_types::{
    CodeLens, Command, CompletionItem, CompletionResponse, Diagnostic, DocumentSymbolResponse,
    FormattingOptions, GotoDefinitionResponse, Hover, HoverContents, Location, MarkedString,
    Position, Range, SemanticTokens, SignatureHelp, SymbolInformation, TextDocumentItem, TextEdit,
    Url, WorkspaceFolder,
};

use crate::document::{CurrentDocument, DocumentKind, Semantics};
use crate::format::format;
use crate::tokens::semantic_tokens;

#[derive(Debug, Error)]
pub enum WorkspaceInitializationError {
    #[error("{0}")]
    Ini(#[from] ini::Error),
    #[error("Section {0} not found")]
    SectionNotFound(String),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("Folders not found")]
    FoldersNotFound,
    #[error("{0}")]
    Join(#[from] JoinError),
}

#[derive(Debug, Error)]
pub enum WorkspaceError {
    #[error("Failed convert Url to file path {0}")]
    UrlConversion(Url),
    #[error("{0}")]
    FileSource(#[from] mlang_syntax::FileSourceError),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    JoinHandle(#[from] JoinError),
}

impl WorkspaceError {
    /// True when this error just means "this document's language can't be
    /// determined" (e.g. no/unknown file extension) rather than an actual
    /// server malfunction. Callers should handle these quietly -- e.g. a
    /// random file the user switched to the `stack` language in the editor
    /// -- instead of logging them as errors on every request.
    pub fn is_unsupported_document(&self) -> bool {
        matches!(self, WorkspaceError::FileSource(_))
    }
}

pub struct Workspace {
    opened_files: DashMap<Url, Arc<RwLock<CurrentDocument>>>,
    mlang_semantics: DashMap<PathBuf, Option<Arc<SemanticModel>>>,
    rx_semantics: DashMap<PathBuf, Option<Arc<RxSemanticModel>>>,
    /// Where paths are shown relative to: the editor's workspace folders and the
    /// common ancestor of the indexed folders, for files outside of them.
    workspace_folders: std::sync::RwLock<Vec<PathBuf>>,
    root: std::sync::RwLock<Option<PathBuf>>,
    // `Arc<[_]>` so it can be cheaply handed to `spawn_blocking` closures.
    core: Arc<[AnyMCoreDefinition]>,
}

/// Semantic model of a file for the cross-file index; `None` for files that don't take
/// part in cross-file navigation.
fn index_semantics(path: &Path, text: &str) -> Option<Semantics> {
    if is_resource(path) {
        let parsed = xml_parser::parse(text);
        return Some(Semantics::Resource(rx_semantics(text, &parsed.syntax())));
    }

    let file_source = MFileSource::try_from(path).ok()?;
    if !(file_source.is_module() || file_source.is_handler()) {
        return None;
    }
    let parsed = parse(text, file_source);
    Some(Semantics::Mlang(semantics(
        text,
        parsed.syntax(),
        file_source,
    )))
}

impl Workspace {
    pub fn new() -> Workspace {
        Workspace {
            opened_files: DashMap::new(),
            mlang_semantics: DashMap::new(),
            rx_semantics: DashMap::new(),
            workspace_folders: Default::default(),
            root: Default::default(),
            core: load_core_api().into(),
        }
    }

    fn register_files(&self, folders: &[PathBuf], files: Vec<PathBuf>) {
        if let Ok(mut root) = self.root.write() {
            *root = common_ancestor(folders);
        }

        self.mlang_semantics.clear();
        self.rx_semantics.clear();
        for path in files {
            if is_resource(&path) {
                self.rx_semantics.insert(path, None);
            } else {
                self.mlang_semantics.insert(path, None);
            }
        }

        info!(
            "Found {} program and {} resource files",
            self.mlang_semantics.len(),
            self.rx_semantics.len()
        );
    }

    fn project(&self) -> Project {
        Project {
            mlang: models(&self.mlang_semantics),
            rx: models(&self.rx_semantics),
            roots: self.roots(),
        }
    }

    /// The editor's workspace folders, even when files are found through `stack.ini`.
    pub fn set_workspace_folders(&self, folders: Option<&[WorkspaceFolder]>) {
        let folders = folders
            .unwrap_or_default()
            .iter()
            .filter_map(|f| f.uri.to_file_path().ok())
            .collect();
        if let Ok(mut workspace_folders) = self.workspace_folders.write() {
            *workspace_folders = folders;
        }
    }

    fn roots(&self) -> Vec<PathBuf> {
        let mut roots = self
            .workspace_folders
            .read()
            .map(|folders| folders.clone())
            .unwrap_or_default();
        roots.extend(self.root.read().ok().and_then(|root| root.clone()));
        roots
    }

    fn insert_model(&self, path: PathBuf, model: Semantics) {
        match model {
            Semantics::Mlang(model) => {
                self.mlang_semantics.insert(path, Some(Arc::new(model)));
            }
            Semantics::Resource(model) => {
                self.rx_semantics.insert(path, Some(Arc::new(model)));
            }
        }
    }

    pub async fn init_with_workspace_folders(
        &self,
        folders: Option<Vec<WorkspaceFolder>>,
    ) -> Result<(), WorkspaceInitializationError> {
        let folders = folders.ok_or(WorkspaceInitializationError::FoldersNotFound)?;
        info!("Get files from workspace folders!");
        self.set_workspace_folders(Some(&folders));

        let folders = folders
            .into_iter()
            .filter_map(|f| f.uri.to_file_path().ok())
            .map(|path| (path, true)) // recursively all folders in workspace
            .collect::<Vec<_>>();
        let roots: Vec<PathBuf> = folders.iter().map(|(path, _)| path.clone()).collect();

        // The directory walk is blocking synchronous I/O -- keep it off the
        // async worker so it can't stall unrelated request handling.
        let files = tokio::task::spawn_blocking(move || get_files(folders)).await??;
        self.register_files(&roots, files);
        Ok(())
    }

    pub async fn init_with_settings_file(
        &self,
        path: &str,
    ) -> Result<(), WorkspaceInitializationError> {
        let path = PathBuf::from(path);

        // Reading/parsing the ini file and walking the directories it points
        // at is all blocking synchronous I/O -- run it on a blocking worker.
        let (roots, files) = tokio::task::spawn_blocking(move || {
            let mut path = path;
            if !path.is_file() {
                path.push("stack.ini");
            }

            info!(
                "Get files from ini file {}!",
                path.to_str().unwrap_or_default()
            );

            let ini = Ini::load_from_file_noescape(path)?;
            let app_path = ini.section(Some("AppPath")).ok_or(
                WorkspaceInitializationError::SectionNotFound("AppPath".to_string()),
            )?;

            // programs and resources
            let folders = app_path
                .get_all("PRG")
                .chain(app_path.get_all("RS"))
                .map(|s| {
                    let mut path = PathBuf::from(s);

                    // recursively only folders ends with **
                    let recursively = path.ends_with("**");
                    if recursively {
                        path.pop();
                    }
                    (path, recursively)
                })
                .collect::<Vec<_>>();

            let roots: Vec<PathBuf> = folders.iter().map(|(path, _)| path.clone()).collect();
            let files = get_files(folders)?;
            Ok::<_, WorkspaceInitializationError>((roots, files))
        })
        .await??;

        self.register_files(&roots, files);
        Ok(())
    }

    pub async fn update_semantic_information(&self) {
        let mut handles = vec![];

        let num_cores = std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(1);

        let current = Handle::current();
        let semaphore = Arc::new(Semaphore::new(num_cores * 2));

        // Snapshot the keys up front: holding a `DashMap` shard guard (what
        // `.iter()` yields) across the `.await` below would block any other
        // task touching that shard for the whole warm-up.
        let paths: Vec<PathBuf> = self
            .mlang_semantics
            .iter()
            .map(|entry| entry.key().to_path_buf())
            .chain(
                self.rx_semantics
                    .iter()
                    .map(|entry| entry.key().to_path_buf()),
            )
            .collect();

        for path in paths {
            let permit = semaphore.clone().acquire_owned().await.unwrap();

            let handle = current.spawn_blocking(move || {
                let _ = permit;
                let text = std::fs::read_to_string(&path).ok()?;
                let model = index_semantics(&path, &text)?;
                Some((path, model))
            });

            handles.push(handle);
        }

        for handle in handles {
            if let Ok(Some((path, model))) = handle.await {
                self.insert_model(path, model);
            }
        }
    }

    pub async fn get_opened_document(
        &self,
        uri: &Url,
    ) -> Result<OwnedRwLockReadGuard<CurrentDocument>, WorkspaceError> {
        if let Some(document) = self.opened_files.get(uri) {
            return Ok(Arc::clone(document.value()).read_owned().await);
        }

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConversion(uri.clone())))?;

        let text = tokio::fs::read_to_string(&path).await?;
        let file_uri = uri.clone();
        let document =
            tokio::task::spawn_blocking(move || CurrentDocument::new(file_uri, &path, &text))
                .await??;

        let document = Arc::new(RwLock::new(document));
        self.opened_files.insert(uri.clone(), Arc::clone(&document));

        Ok(document.read_owned().await)
    }

    pub async fn hover(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<Hover>, WorkspaceError> {
        let semantic_info = self.identifier_from_position(uri, position).await?;
        let Some(semantic_info) = semantic_info else {
            return Ok(None);
        };

        let mut markups = get_hover(&semantic_info, self.core.iter().map(|d| (uri.clone(), d)));
        if markups.is_empty() {
            markups = self.project().hover(&semantic_info);
        }

        Ok(Some(Hover {
            contents: HoverContents::Array(markups),
            range: None,
        }))
    }

    pub async fn goto_definition(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<GotoDefinitionResponse>, WorkspaceError> {
        let semantic_info = self.identifier_from_position(uri, position).await?;
        let Some(semantic_info) = semantic_info else {
            return Ok(None);
        };

        let locations = self.project().declarations(&semantic_info);
        Ok(Some(GotoDefinitionResponse::Array(locations)))
    }

    pub async fn references(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<Vec<Location>>, WorkspaceError> {
        let semantic_info = self.identifier_from_position(uri, position).await?;
        let Some(semantic_info) = semantic_info else {
            return Ok(None);
        };

        let mlang = models(&self.mlang_semantics)
            .into_iter()
            .flat_map(|(uri, model)| get_reference(&semantic_info, &uri, model.references()));
        let rx = models(&self.rx_semantics)
            .into_iter()
            .flat_map(|(uri, model)| get_reference(&semantic_info, &uri, model.references()));
        let locations = mlang.chain(rx).collect::<Vec<_>>();

        Ok(Some(locations))
    }

    pub async fn document_symbol_response(
        &self,
        uri: &Url,
    ) -> Result<Option<DocumentSymbolResponse>, WorkspaceError> {
        let document = self.get_opened_document(uri).await?;

        if let Some(symbols) = document.xml_document_symbols() {
            return Ok(Some(DocumentSymbolResponse::Nested(symbols)));
        }

        let definitions = document.definitions();
        let response = get_symbols(uri, definitions);

        Ok(Some(DocumentSymbolResponse::Flat(response)))
    }

    pub async fn symbol_information(&self, query: &str) -> Option<Vec<SymbolInformation>> {
        let semantics = self.mlang_semantics.iter().filter_map(|r| match r.pair() {
            (path, Some(definitions)) => {
                let uri = Url::from_file_path(path).ok()?;
                Some((uri, Arc::clone(definitions)))
            }
            _ => None,
        });

        let information = semantics
            .flat_map(|(uri, semantics)| {
                if !query.is_empty() {
                    let query = StringLowerCase::new(query);
                    get_symbols(
                        &uri,
                        semantics
                            .definitions()
                            .filter(|d| d.partial_compare_id_with(&query)),
                    )
                } else {
                    get_symbols(&uri, semantics.definitions())
                }
            })
            .collect::<Vec<_>>();

        Some(information)
    }

    pub async fn code_lens(&self, uri: &Url) -> Result<Option<Vec<CodeLens>>, WorkspaceError> {
        let document = self.get_opened_document(uri).await?;
        let command = String::from("stack.movetoLine");

        let command_builder = |title: String, line_number: u32| {
            let args = vec![Value::Number(line_number.into())];
            Some(Command::new(title, command.clone(), Some(args)))
        };

        // the project lookup runs without holding the document lock
        if let Some(model) = document.rx_semantics() {
            // above the opening tag, even when attributes are on their own lines
            let resources: Vec<_> = model
                .definitions()
                .map(|d| (d.lsp_range(), d.links()))
                .collect();
            drop(document);

            let project = self.project();
            let uri = uri.clone();
            let lenses = tokio::task::spawn_blocking(move || {
                let links = project.links();
                resources
                    .into_iter()
                    .flat_map(|(range, own)| links.resource_lenses(&uri, range, own))
                    .collect()
            })
            .await?;
            return Ok(Some(lenses));
        }

        let mut response = get_lens(command_builder, document.definitions());

        if document.kind() == DocumentKind::Handler {
            let handlers: Vec<_> = document
                .definitions()
                .filter(|d| d.kind() == DefinitionKind::Handler)
                .map(|d| (d.lsp_range(), d.id().to_string()))
                .collect();
            drop(document);

            let project = self.project();
            let uri = uri.clone();
            let lenses = tokio::task::spawn_blocking(move || {
                let links = project.links();
                handlers
                    .into_iter()
                    .flat_map(|(range, handler)| {
                        links
                            .handler_links(&handler)
                            .into_iter()
                            .map(|(label, resource)| links.lens(&uri, range, label, resource))
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>()
            })
            .await?;
            response.extend(lenses);
        }

        Ok(Some(response))
    }

    pub async fn semantic_tokens(
        &self,
        uri: &Url,
        tokens_range: Option<Range>,
    ) -> Result<Option<SemanticTokens>, WorkspaceError> {
        let document = self.get_opened_document(uri).await?;
        let Some(syntax) = document.mlang_syntax() else {
            return Ok(None);
        };
        let line_index = document.line_index();

        Ok(Some(SemanticTokens {
            result_id: None,
            data: semantic_tokens(syntax, line_index, tokens_range),
        }))
    }

    pub async fn completion(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<CompletionResponse>, WorkspaceError> {
        // get position before trigger
        let line = position.line;
        let Some(char) = position.character.checked_sub(1) else {
            return Ok(None);
        };
        let position = Position::new(line, char);

        let document = self.get_opened_document(uri).await?;
        let semantic_info = async {
            let syntax = document.mlang_syntax()?;
            let text = syntax.text().to_string();

            let line_index = LineIndex::new(&text);
            let offset = line_index.offset(LineCol {
                line: position.line,
                col: position.character,
            })?;

            identifier_for_completion(syntax, offset)
        }
        .await;

        let Some(semantic_info) = semantic_info else {
            return Ok(None);
        };

        let semantics = self
            .mlang_semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (path, Some(semantics)) => {
                    let uri = Url::from_file_path(path).ok()?;
                    let semantics = Arc::clone(semantics);
                    Some((uri, semantics))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let definitions = semantics
            .iter()
            .flat_map(|(uri, arc)| arc.definitions().map(|d| (uri.clone(), d)));

        let completions: Vec<CompletionItem> = get_completion(&semantic_info, definitions);
        Ok(Some(CompletionResponse::Array(completions)))
    }

    pub async fn signature_help(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<SignatureHelp>, WorkspaceError> {
        let position = Position::new(position.line, position.character);

        let document = self.get_opened_document(uri).await?;
        let semantic_data = async {
            let syntax = document.mlang_syntax()?;
            let text = syntax.text().to_string();

            let line_index = LineIndex::new(&text);
            let offset = line_index.offset(LineCol {
                line: position.line,
                col: position.character,
            })?;

            identifier_for_signature_help(syntax, offset)
        }
        .await;

        let Some(semantic_data) = semantic_data else {
            return Ok(None);
        };

        let semantics = self
            .mlang_semantics
            .iter()
            .filter_map(|r| match r.pair() {
                (path, Some(semantics)) => {
                    let uri = Url::from_file_path(path).ok()?;
                    let semantics = Arc::clone(semantics);
                    Some((uri, semantics))
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        let definitions = semantics
            .iter()
            .flat_map(|(uri, arc)| arc.definitions().map(|d| (uri.clone(), d)));

        let (semantic_info, current_argument) = semantic_data;

        let signatures = get_signatures(&semantic_info, definitions, current_argument);

        Ok(Some(SignatureHelp {
            signatures,
            active_signature: None,
            active_parameter: None,
        }))
    }

    pub async fn format(
        &self,
        uri: &Url,
        range: Range,
        options: FormattingOptions,
    ) -> Result<Option<Vec<TextEdit>>, WorkspaceError> {
        let document = self.get_opened_document(uri).await?;

        let handle = tokio::task::spawn_blocking(move || format(&document, options, range));
        let edited = handle.await?;
        Ok(edited)
    }
}

/// Lints `document` against the core API and the workspace's cross-file definitions
fn semantic_lint(
    core: &[AnyMCoreDefinition],
    workspace_semantics: &[Arc<SemanticModel>],
    document: &CurrentDocument,
) -> Vec<mlang_lint::Diagnostic> {
    let Some(root) = document.mlang_syntax() else {
        return Vec::new();
    };

    let definitions: Vec<&AnyMDefinition> = document
        .definitions()
        .chain(workspace_semantics.iter().flat_map(|s| s.definitions()))
        .collect();

    let mut diagnostics = mlang_lint::syntax_diagnostics(&root);

    diagnostics.extend(mlang_lint::semantic_diagnostics(
        &root,
        core,
        definitions.into_iter(),
    ));

    diagnostics
}

impl Workspace {
    /// Snapshot of the cross-file semantic models to lint an open document
    /// against. Cheap `Arc` clones; take it before moving work into
    /// `spawn_blocking`.
    fn workspace_semantics_snapshot(&self) -> Vec<Arc<SemanticModel>> {
        self.mlang_semantics
            .iter()
            .filter_map(|r| r.value().clone())
            .collect()
    }

    pub async fn open_document(
        &self,
        document: TextDocumentItem,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let uri = document.uri;

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConversion(uri.clone())))?;

        let document_uri = uri.clone();
        let core = Arc::clone(&self.core);
        let workspace_semantics = self.workspace_semantics_snapshot();

        let handle = tokio::task::spawn_blocking(move || {
            let document = CurrentDocument::new(document_uri, &path, &document.text)?;
            let lint = semantic_lint(&core, &workspace_semantics, &document);
            let diagnostics = document.diagnostics(&lint);
            Ok::<_, mlang_syntax::FileSourceError>((document, diagnostics))
        });

        let (document, diagnostics) = handle.await??;

        self.opened_files
            .insert(uri, Arc::new(RwLock::new(document)));

        Ok(diagnostics)
    }

    pub async fn close_document(&self, document_url: &Url) {
        self.opened_files.remove(document_url);
    }

    pub async fn change_document(
        &self,
        document: TextDocumentItem,
    ) -> Result<Vec<Diagnostic>, WorkspaceError> {
        let uri = document.uri;

        let path = uri
            .to_file_path()
            .or(Err(WorkspaceError::UrlConversion(uri.clone())))?;

        // lock file for read
        let guard = self.opened_files.get(&uri);
        let opened_file = if let Some(guard) = guard {
            Some(Arc::clone(guard.value()).write_owned().await)
        } else {
            None
        };

        let document_uri = uri.clone();
        let path_for_blocking = path.clone();
        let handle = tokio::task::spawn_blocking(move || {
            // only programs, handlers and resources take part in the
            // cross-file index -- e.g. a `.sql` file has no model at all
            let model = index_semantics(&path_for_blocking, &document.text);

            let current_document =
                CurrentDocument::new(document_uri, &path_for_blocking, &document.text)?;

            Ok::<_, mlang_syntax::FileSourceError>((current_document, model))
        });

        let (document, model) = handle.await??;

        if let Some(model) = model {
            self.insert_model(path, model);
        }

        if let Some(mut opened_file) = opened_file {
            let core = Arc::clone(&self.core);
            let workspace_semantics = self.workspace_semantics_snapshot();

            let (document, diagnostics) = tokio::task::spawn_blocking(move || {
                let lint = semantic_lint(&core, &workspace_semantics, &document);
                let diagnostics = document.diagnostics(&lint);
                (document, diagnostics)
            })
            .await?;

            *opened_file = document;

            // show diagnostics only for opened files
            return Ok(diagnostics);
        }

        Ok(vec![])
    }
    pub async fn delete_document(&self, document_url: &Url) {
        self.opened_files.remove(document_url);

        let path = document_url.to_file_path();
        if let Ok(path) = path {
            self.mlang_semantics.remove(&path);
            self.rx_semantics.remove(&path);
        }
    }
}

impl Workspace {
    async fn identifier_from_position(
        &self,
        uri: &Url,
        position: Position,
    ) -> Result<Option<SemanticInfo>, WorkspaceError> {
        let document = self.get_opened_document(uri).await?;

        let offset = document.line_index().offset(LineCol {
            line: position.line,
            col: position.character,
        });
        let Some(offset) = offset else {
            return Ok(None);
        };

        let kind = document.kind();
        let identifier = if let Some(file_source) = kind.mlang_file_source() {
            document
                .mlang_syntax()
                .and_then(|syntax| identifier_for_offset(syntax, offset, file_source))
        } else if kind == DocumentKind::Resource {
            document.xml_syntax().and_then(|syntax| {
                rx_identifier_for_offset(&syntax, offset)
                    .or_else(|| expression_identifier(&syntax, offset))
            })
        } else {
            None
        };

        Ok(identifier)
    }
}

fn common_ancestor(paths: &[PathBuf]) -> Option<PathBuf> {
    let (first, rest) = paths.split_first()?;
    let mut ancestor = first.clone();
    for path in rest {
        while !path.starts_with(&ancestor) {
            if !ancestor.pop() {
                return None;
            }
        }
    }
    Some(ancestor)
}

/// What the cursor points at inside an API browser's `Выражение`: the value is mlang code,
/// a bare name there is a function.
fn expression_identifier(root: &XmlSyntaxNode, offset: TextSize) -> Option<SemanticInfo> {
    let expression = rx_expression_at(root, offset)?;
    let source = MFileSource::script();
    let parsed = parse(&expression.code, source);
    identifier_for_offset(parsed.syntax(), expression.offset, source).or_else(|| {
        let name = expression.code.trim().trim_end_matches(';').trim_end();
        let is_name = !name.is_empty() && name.chars().all(|c| c.is_alphanumeric() || c == '_');
        is_name.then(|| SemanticInfo::new(Symbol::Function(name.to_string()), Usage::Reference))
    })
}

fn is_resource(path: &Path) -> bool {
    XmlFileSource::try_from(path).is_ok_and(|source| source.variant().is_resource())
}

/// Snapshot of the cross-file index.
struct Project {
    mlang: Vec<(Url, Arc<SemanticModel>)>,
    rx: Vec<(Url, Arc<RxSemanticModel>)>,
    roots: Vec<PathBuf>,
}

impl Project {
    fn declarations(&self, info: &SemanticInfo) -> Vec<Location> {
        let mut locations = get_declaration(info, definitions(&self.mlang, |m| m.definitions()));
        locations.extend(get_declaration(
            info,
            definitions(&self.rx, |m| m.definitions()),
        ));
        locations
    }

    fn links(&self) -> Links<'_> {
        Links {
            mlang: LinkIndex::new(
                self.mlang
                    .iter()
                    .flat_map(|(uri, m)| m.definitions().map(move |d| (uri, d))),
            ),
            rx: LinkIndex::new(
                self.rx
                    .iter()
                    .flat_map(|(uri, m)| m.definitions().map(move |d| (uri, d))),
            ),
        }
    }

    fn hover(&self, info: &SemanticInfo) -> Vec<MarkedString> {
        let roots = &self.roots;
        let mut markups =
            get_project_hover(info, definitions(&self.mlang, |m| m.definitions()), roots);
        markups.extend(get_project_hover(
            info,
            definitions(&self.rx, |m| m.definitions()),
            roots,
        ));
        markups
    }
}

/// Resources and handlers of the project indexed by name, for lenses.
struct Links<'a> {
    mlang: LinkIndex<'a, &'a Url, AnyMDefinition>,
    rx: LinkIndex<'a, &'a Url, RxDefinition>,
}

impl Links<'_> {
    fn declarations(&self, symbol: &Symbol) -> Vec<Location> {
        let mlang = self.mlang.resolve(symbol).unwrap_or_default();
        let mlang = mlang.into_iter().map(|(uri, d)| d.id_location(uri.clone()));
        let rx = self.rx.resolve(symbol).unwrap_or_default();
        let rx = rx.into_iter().map(|(uri, d)| d.id_location(uri.clone()));
        mlang.chain(rx).collect()
    }

    fn lens(&self, uri: &Url, range: Range, label: &str, target: Symbol) -> CodeLens {
        let locations = self.declarations(&target);
        link_lens(uri, range, label, &target, locations)
    }

    /// Lenses of a resource with `own` links: its handlers first, then, for a browser,
    /// the handlers of its select instead of the select itself.
    fn resource_lenses(&self, uri: &Url, range: Range, own: Vec<RxLink>) -> Vec<CodeLens> {
        let (selects, mut links): (Vec<_>, Vec<_>) = own
            .into_iter()
            .partition(|link| link.kind == RxLinkKind::Select);

        for link in selects {
            if let Symbol::Select(select) = &link.symbol {
                let handlers = self.select_handlers(select).into_iter();
                links.extend(handlers.map(|symbol| RxLink {
                    kind: RxLinkKind::SelectHandler,
                    symbol,
                }));
            }
        }

        links
            .into_iter()
            .map(|link| {
                let label = match link.kind {
                    RxLinkKind::Select => "Select",
                    RxLinkKind::SelectHandler => "Select handler",
                    RxLinkKind::Handler => "Handler",
                };
                self.lens(uri, range, label, link.symbol)
            })
            .collect()
    }

    /// Resources a handler serves: a select handler its select, an API handler its browser
    /// and the browser's select.
    fn handler_links(&self, handler: &str) -> Vec<(&'static str, Symbol)> {
        let browser = match handler_resource(handler) {
            Symbol::ApiBrowser(browser) => browser,
            select => return vec![("Select", select)],
        };

        let mut links = vec![];
        let browsers = self.rx.named(DefinitionKind::ApiBrowser, &browser);
        for select in browsers.filter_map(|d| d.select.as_ref()) {
            let select = ("Select", Symbol::Select(select.text.clone()));
            if !links.contains(&select) {
                links.push(select);
            }
        }
        links.insert(0, ("API browser", Symbol::ApiBrowser(browser)));
        links
    }

    /// The handler named after the select and the `Обработчик` of each select with this name.
    fn select_handlers(&self, select: &str) -> Vec<Symbol> {
        let mut handlers = vec![Symbol::Handler(select.to_string())];
        let selects = self.rx.named(DefinitionKind::Select, select);
        for handler in selects.filter_map(|d| d.handler.as_ref()) {
            let handler = Symbol::ExtraHandler(handler.text.clone());
            if !handlers.contains(&handler) {
                handlers.push(handler);
            }
        }
        handlers
    }
}

/// A lens leading to `target`, or telling it is not found.
fn link_lens(
    uri: &Url,
    range: Range,
    label: &str,
    target: &Symbol,
    locations: Vec<Location>,
) -> CodeLens {
    let name = target.name().unwrap_or_default();

    // a command without id is shown as plain text
    let command = match locations.len() {
        0 => Command::new(format!("{label}: {name} (not found)"), String::new(), None),
        count => {
            let count = match count {
                1 => String::new(),
                n => format!(" ({n})"),
            };
            let arguments = vec![json!(uri), json!(range.start), json!(locations)];
            Command::new(
                format!("{label}: {name}{count}"),
                String::from("stack.goToLocations"),
                Some(arguments),
            )
        }
    };

    CodeLens {
        range,
        command: Some(command),
        data: None,
    }
}

/// Loaded models with their file urls; cheap `Arc` clones, no map guards held.
fn models<T>(map: &DashMap<PathBuf, Option<Arc<T>>>) -> Vec<(Url, Arc<T>)> {
    map.iter()
        .filter_map(|r| match r.pair() {
            (path, Some(model)) => Some((Url::from_file_path(path).ok()?, Arc::clone(model))),
            _ => None,
        })
        .collect()
}

fn definitions<'a, T, D: 'a, I>(
    models: &'a [(Url, Arc<T>)],
    of: impl Fn(&'a T) -> I + 'a,
) -> impl Iterator<Item = (Url, &'a D)>
where
    I: Iterator<Item = &'a D> + 'a,
{
    models
        .iter()
        .flat_map(move |(uri, model)| of(model).map(move |d| (uri.clone(), d)))
}

fn get_files(to_visit: Vec<(PathBuf, bool)>) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::with_capacity(1000);

    for (path, recursively) in to_visit {
        let depth = if recursively { usize::MAX } else { 1 };
        let walker = WalkDir::new(path).max_depth(depth);

        let entries = walker
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.file_type().is_file())
            .filter(|entry| {
                let path = entry.path();
                MFileSource::try_from(path).is_ok_and(|m| m.is_module() || m.is_handler())
                    || is_resource(path)
            });

        for entry in entries {
            files.push(entry.into_path());
        }
    }

    Ok(files)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn common_ancestor_of_folders() {
        let root = std::env::temp_dir().join("master");
        let folders = [
            root.join("one").join("RX"),
            root.join("two").join("rx"),
            root.join("one").join("prg"),
        ];
        assert_eq!(common_ancestor(&folders), Some(root.clone()));
        assert_eq!(common_ancestor(&[root.join("prg")]), Some(root.join("prg")));
        assert_eq!(common_ancestor(&[]), None);
    }
}
