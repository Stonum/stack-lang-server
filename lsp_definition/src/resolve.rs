use std::path::{Path, PathBuf};

use itertools::Itertools;
use tower_lsp::lsp_types::{
    Documentation, Location, MarkedString, ParameterInformation, ParameterLabel,
    SignatureInformation, Url,
};

use crate::members::class_members;
use crate::resource::resolve_linked;
use crate::{
    CodeSymbolDefinition, DefinitionKind, LocationDefinition, MarkupDefinition, SemanticInfo,
    SignatureParameters, Symbol, Usage,
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mode {
    /// Only definitions that accept the call's argument count.
    Goto,
    /// All candidates, best arity match first; a class is shown with its constructors.
    Hover,
}

pub fn get_declaration<'a, I, D>(info: &SemanticInfo, definitions: I) -> Vec<Location>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + LocationDefinition + 'a,
{
    resolve(info, definitions, Mode::Goto)
        .into_iter()
        .map(|(uri, d)| d.id_location(uri))
        .collect()
}

pub fn get_hover<'a, I, D>(info: &SemanticInfo, definitions: I) -> Vec<MarkedString>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + 'a,
{
    resolve(info, definitions, Mode::Hover)
        .into_iter()
        .map(|(_, d)| MarkedString::String(d.full_markdown()))
        .collect()
}

/// [get_hover] for definitions from project files; for symbols linked across files
/// each one also links to where it is declared, shown relative to the deepest of `roots`.
pub fn get_project_hover<'a, I, D>(
    info: &SemanticInfo,
    definitions: I,
    roots: &[PathBuf],
) -> Vec<MarkedString>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + LocationDefinition + 'a,
{
    let located = info.symbol.is_linked();
    resolve(info, definitions, Mode::Hover)
        .into_iter()
        .map(|(uri, d)| {
            let mut markdown = d.full_markdown();
            if located {
                markdown.push_str("  \n");
                markdown.push_str(&location_link(&uri, d.id_range().start.line, roots));
            }
            MarkedString::String(markdown)
        })
        .collect()
}

/// `[dir/file.rx:12](file:///…/dir/file.rx#L12)`: the path relative to `root`, or just
/// the file name outside of it; `line` is zero-based.
fn location_link(uri: &Url, line: u32, roots: &[PathBuf]) -> String {
    let line = line + 1;
    let name = uri
        .to_file_path()
        .ok()
        .and_then(|path| {
            let relative = roots
                .iter()
                .filter_map(|root| relative_path(&path, root))
                .min_by_key(|relative| relative.len());
            match relative {
                Some(relative) => Some(relative.join("/")),
                None => Some(path.file_name()?.to_string_lossy().into_owned()),
            }
        })
        .unwrap_or_else(|| uri.path().to_string());
    format!("[{name}:{line}]({uri}#L{line})")
}

/// Components of `path` below `root`, compared ignoring case as Windows paths are.
fn relative_path(path: &Path, root: &Path) -> Option<Vec<String>> {
    let mut components = path.components();
    for root in root.components() {
        let component = components.next()?;
        let same = unicase::eq(
            component.as_os_str().to_string_lossy().as_ref(),
            root.as_os_str().to_string_lossy().as_ref(),
        );
        if !same {
            return None;
        }
    }
    let relative: Vec<String> = components
        .map(|c| c.as_os_str().to_string_lossy().into_owned())
        .collect();
    (!relative.is_empty()).then_some(relative)
}

pub fn get_signatures<'a, I, D>(
    info: &SemanticInfo,
    definitions: I,
    current_argument: u32,
) -> Vec<SignatureInformation>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + MarkupDefinition + SignatureParameters + 'a,
{
    let callables = match (&info.symbol, info.usage) {
        (Symbol::Function(_), Usage::Call(_))
        | (Symbol::Member { class: Some(_), .. }, Usage::Call(_))
        | (Symbol::Class(_), Usage::New(_) | Usage::Super(_)) => {
            resolve(info, definitions, Mode::Hover)
        }
        _ => vec![],
    };

    callables
        .into_iter()
        .filter(|(_, d)| d.kind() != DefinitionKind::Class)
        .map(|(_, d)| signature(d, current_argument))
        .collect()
}

fn signature<D: SignatureParameters>(d: &D, current_argument: u32) -> SignatureInformation {
    let offsets = d.offsets();
    let mut parameters = offsets
        .iter()
        .map(|offset| ParameterInformation {
            label: ParameterLabel::LabelOffsets(*offset),
            documentation: None,
        })
        .collect::<Vec<_>>();

    if d.has_rest() {
        if let Some(rest) = parameters.last_mut() {
            rest.documentation = Some(Documentation::String(String::from("... arg 1")));
        }

        if let Some(offset) = offsets.last() {
            for i in 2..100 {
                parameters.push(ParameterInformation {
                    label: ParameterLabel::LabelOffsets(*offset),
                    documentation: Some(Documentation::String(format!("... arg {i}"))),
                });
            }
        }
    }

    SignatureInformation {
        label: d.text(),
        parameters: Some(parameters),
        documentation: None,
        active_parameter: Some(current_argument),
    }
}

fn resolve<'a, I, D>(info: &SemanticInfo, definitions: I, mode: Mode) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let SemanticInfo { symbol, usage } = info;
    let arguments = usage.arguments();

    let Some(name) = symbol.name() else {
        return vec![];
    };

    match symbol {
        Symbol::Select(_)
        | Symbol::ApiBrowser(_)
        | Symbol::Handler(_)
        | Symbol::ExtraHandler(_)
        | Symbol::HandlerEvent { .. } => return resolve_linked(symbol, definitions),
        Symbol::Class(_)
            if matches!(usage, Usage::Declaration | Usage::New(_) | Usage::Super(_)) =>
        {
            return class_with_constructors(name, arguments, definitions, mode);
        }
        _ => {}
    }

    let pool = match symbol {
        Symbol::Member {
            class: Some(class), ..
        } => class_members(definitions, class),
        _ => definitions.into_iter().collect(),
    };

    let candidates = pool
        .into_iter()
        .filter(|(_, d)| accepts(symbol, *usage, d.kind()) && d.compare_id_with(name))
        .collect();

    by_arity(candidates, arguments, mode)
}

fn accepts(symbol: &Symbol, usage: Usage, kind: DefinitionKind) -> bool {
    match symbol {
        Symbol::Function(_) => kind == DefinitionKind::Function,
        Symbol::Class(_) => kind == DefinitionKind::Class,
        Symbol::AnyClass => false,
        Symbol::Member { .. } if usage == Usage::Access => kind.is_accessor(),
        Symbol::Member { .. } => kind == DefinitionKind::Method,
        // resolved by name links in `resolve_linked`
        Symbol::Select(_)
        | Symbol::ApiBrowser(_)
        | Symbol::Handler(_)
        | Symbol::ExtraHandler(_)
        | Symbol::HandlerEvent { .. } => false,
    }
}

fn by_arity<D>(candidates: Vec<(Url, &D)>, arguments: Option<usize>, mode: Mode) -> Vec<(Url, &D)>
where
    D: CodeSymbolDefinition,
{
    let Some(count) = arguments else {
        return candidates;
    };

    match mode {
        Mode::Goto => candidates
            .into_iter()
            .filter(|(_, d)| d.can_be_called(count))
            .collect(),
        Mode::Hover => candidates
            .into_iter()
            .sorted_by(|(_, a), (_, b)| a.call_priority(b, count))
            .collect(),
    }
}

/// Goto: matching constructors, or the class itself when there are none.
/// Hover: the class followed by its constructors.
fn class_with_constructors<'a, I, D>(
    class_name: &str,
    arguments: Option<usize>,
    definitions: I,
    mode: Mode,
) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let definitions = Vec::from_iter(definitions);
    let mut result = vec![];

    let classes = definitions
        .iter()
        .filter(|(_, d)| d.kind() == DefinitionKind::Class && d.compare_id_with(class_name));

    for (uri, class) in classes {
        let constructors = definitions
            .iter()
            .filter(|(_, d)| {
                d.kind() == DefinitionKind::Constructor && d.container().as_ref() == Some(*class)
            })
            .cloned()
            .collect();
        let constructors = by_arity(constructors, arguments, mode);

        if mode == Mode::Hover || constructors.is_empty() {
            result.push((uri.clone(), *class));
        }
        result.extend(constructors);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn location_link_shows_file_name_and_one_based_line() {
        let uri = Url::from_file_path(std::env::temp_dir().join("Модуль.rx")).unwrap();
        let link = location_link(&uri, 4, &[]);
        assert!(link.starts_with("[Модуль.rx:5]("), "{link}");
        assert!(link.ends_with(&format!("{uri}#L5)")), "{link}");
    }

    #[test]
    fn location_link_shows_path_relative_to_the_deepest_root() {
        let temp = std::env::temp_dir();
        let project = temp.join("project");
        let uri = Url::from_file_path(project.join("one").join("RX").join("stack.rx")).unwrap();

        let link = location_link(&uri, 0, &[temp.clone(), project.clone()]);
        assert!(link.starts_with("[one/RX/stack.rx:1]("), "{link}");

        let link = location_link(&uri, 0, &[temp.join("other")]);
        assert!(link.starts_with("[stack.rx:1]("), "{link}");
    }

    #[test]
    fn relative_path_ignores_case() {
        let root = std::env::temp_dir().join("Project");
        let path = std::env::temp_dir().join("PROJECT").join("rx").join("a.rx");
        assert_eq!(
            relative_path(&path, &root),
            Some(vec!["rx".into(), "a.rx".into()])
        );
        assert_eq!(relative_path(&root, &root), None);
    }
}
