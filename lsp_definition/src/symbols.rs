use tower_lsp::lsp_types::{CodeLens, Command, SymbolInformation, Url};

use crate::{CodeSymbolInformation, DefinitionKind, LocationDefinition};

pub fn get_symbols<'a, I, D>(uri: &Url, definitions: I) -> Vec<SymbolInformation>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolInformation + LocationDefinition + 'a,
{
    definitions
        .into_iter()
        .map(|def| {
            #[allow(deprecated)]
            SymbolInformation {
                name: def.symbol_name(),
                kind: def.symbol_kind(),
                tags: None,
                deprecated: None,
                location: def.location(uri.clone()),
                container_name: def.container().map(|c| c.symbol_name()),
            }
        })
        .collect::<Vec<_>>()
}

pub fn get_lens<'a, F, I, D>(command_builder: F, definitions: I) -> Vec<CodeLens>
where
    F: Fn(String, u32) -> Option<Command>,
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolInformation + LocationDefinition + 'a,
{
    definitions
        .into_iter()
        .filter(|def| def.kind() != DefinitionKind::Property)
        .filter_map(|def| {
            let container = def.container()?;
            let title = container.symbol_name();
            let line = container.range().start.line;

            Some(CodeLens {
                range: def.lsp_range(),
                command: command_builder(title, line),
                data: None,
            })
        })
        .collect()
}
