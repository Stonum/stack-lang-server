use std::ops::Not;

use tower_lsp::lsp_types::Url;

use crate::{CodeSymbolDefinition, DefinitionKind};

/// Members of `class_name` and its ancestors; overridden ancestor members are skipped.
pub(crate) fn class_members<'a, I, D>(definitions: I, class_name: &str) -> Vec<(&'a Url, &'a D)>
where
    I: IntoIterator<Item = (&'a Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let definitions = Vec::from_iter(definitions);
    let classes = definitions
        .iter()
        .filter_map(|(_, d)| (d.kind() == DefinitionKind::Class).then_some(d))
        .collect::<Vec<_>>();

    // collect hierarchy
    let mut classes_hier = vec![class_name];

    let mut stack = vec![class_name];
    while let Some(current_class) = stack.pop() {
        for class in classes.iter().filter(|c| c.compare_id_with(current_class)) {
            if let Some(parent) = class.parent() {
                classes_hier.push(parent);
                stack.push(parent);
            }
        }
    }

    classes_hier.dedup(); // remove duplicates

    // collect all class and super class methods
    let mut members: Vec<(&Url, &D)> = vec![];
    for current_class in classes_hier {
        let current_members = definitions
            .iter()
            // find by class name
            .filter(|(_uri, member)| {
                member
                    .container()
                    .is_some_and(|c| c.compare_id_with(current_class))
            })
            // filter out already added members
            .filter(|(_uri, current_member)| {
                members
                    .iter()
                    .any(|(_uri, member)| member.can_be_overridden(*current_member))
                    .not()
            })
            .copied()
            .collect::<Vec<_>>();

        members.extend(current_members);
    }

    members
}
