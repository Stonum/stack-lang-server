use std::collections::HashMap;

use tower_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, Documentation, MarkupContent, MarkupKind, Url,
};

use crate::members::class_members;
use crate::{
    CodeSymbolDefinition, CodeSymbolInformation, DefinitionKind, LocationDefinition,
    MarkupDefinition, SemanticInfo, Symbol, Usage,
};

pub fn get_completion<'a, I, D>(info: &SemanticInfo, definitions: I) -> Vec<CompletionItem>
where
    I: IntoIterator<Item = (Url, &'a D)>,
    D: CodeSymbolDefinition + CodeSymbolInformation + MarkupDefinition + LocationDefinition + 'a,
{
    match (&info.symbol, info.usage) {
        (Symbol::Class(class), Usage::Instance | Usage::Super(_) | Usage::New(_)) => {
            let members = class_members(definitions, class);
            completion_items(members.into_iter().map(|(_, d)| d))
        }
        (Symbol::AnyClass, _) => {
            let classes = definitions
                .into_iter()
                .filter_map(|(_, d)| (d.kind() == DefinitionKind::Class).then_some(d));
            completion_items(classes)
        }
        _ => vec![],
    }
}

fn completion_items<'a, I, D>(definitions: I) -> Vec<CompletionItem>
where
    I: IntoIterator<Item = &'a D>,
    D: CodeSymbolDefinition + CodeSymbolInformation + MarkupDefinition + 'a,
{
    let mut def_groups: HashMap<&str, Vec<&D>> = HashMap::new();
    for d in definitions.into_iter().filter(|d| d.kind().is_member()) {
        def_groups.entry(d.id()).or_default().push(d);
    }

    def_groups
        .into_iter()
        .map(|def_group| {
            let first_def = def_group.1.first().unwrap();
            let kind = first_def.kind();
            let completion_label = first_def.symbol_name();
            let mut completion_item = CompletionItem::new_simple(
                completion_label.to_string(),
                first_def.parent().unwrap_or_default().to_string(),
            );

            completion_item.kind = match kind {
                DefinitionKind::Method => Some(CompletionItemKind::METHOD),
                DefinitionKind::Getter | DefinitionKind::Setter => {
                    Some(CompletionItemKind::PROPERTY)
                }
                DefinitionKind::Property => Some(CompletionItemKind::VARIABLE),
                DefinitionKind::Class => Some(CompletionItemKind::CLASS),
                _ => None,
            };

            if completion_label.starts_with("_") {
                completion_item.sort_text = Some(format!("я{}", completion_label));
            } else if kind == DefinitionKind::Property {
                completion_item.sort_text = Some(format!("яя{}", completion_label));
            }

            let markdown_strs: Vec<String> =
                def_group.1.iter().map(|d| d.full_markdown()).collect();
            let markdown = MarkupContent {
                kind: MarkupKind::Markdown,
                value: markdown_strs.join("  \n"),
            };
            completion_item.documentation = Some(Documentation::MarkupContent(markdown));
            completion_item
        })
        .collect()
}
