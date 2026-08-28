use biome_rowan::AstSeparatedList;
use mlang_core::AnyMCoreDefinition;
use mlang_lsp_definition::CodeSymbolDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::{AnyMExpression, AstNode, MCallExpression, MSyntaxNode};
use std::collections::HashMap;
use unicase::UniCase;

use crate::{Diagnostic, Severity};

pub const CODE: &str = "call-arity-mismatch";

pub fn check<'a>(
    root: &MSyntaxNode,
    core: &'a [AnyMCoreDefinition],
    definitions: impl Iterator<Item = &'a AnyMDefinition>,
) -> Vec<Diagnostic> {
    let core_index = build_index(core.iter());
    let definitions_index = build_index(definitions);

    root.descendants()
        .filter_map(MCallExpression::cast)
        .filter_map(|call| check_call(&call, &core_index, &definitions_index))
        .collect()
}

/// Indexes `items` by case-folded name (matching
/// [CodeSymbolDefinition::compare_id_with]'s `unicase::eq` comparison) so a
/// lookup by name doesn't have to linearly rescan every definition for
/// every call site in the file -- `check` used to do exactly that,
/// making the whole lint O(calls x definitions).
fn build_index<'a, T: CodeSymbolDefinition>(
    items: impl Iterator<Item = &'a T>,
) -> HashMap<UniCase<String>, Vec<&'a T>> {
    let mut index: HashMap<UniCase<String>, Vec<&T>> = HashMap::new();
    for item in items.filter(|d| d.is_function()) {
        index
            .entry(UniCase::new(item.id().to_string()))
            .or_default()
            .push(item);
    }
    index
}

fn check_call(
    call: &MCallExpression,
    core_index: &HashMap<UniCase<String>, Vec<&AnyMCoreDefinition>>,
    definitions_index: &HashMap<UniCase<String>, Vec<&AnyMDefinition>>,
) -> Option<Diagnostic> {
    let AnyMExpression::MIdentifierExpression(ident) = call.callee().ok()? else {
        return None;
    };
    let name = ident.name().ok()?.text();
    let arguments = call.arguments().ok()?;
    let count = arguments.args().len();

    let mut known = false;
    let mut accepted = false;

    let key = UniCase::new(name.to_string());

    if let Some(matches) = core_index.get(&key) {
        for d in matches {
            known = true;
            accepted |= d.can_be_called(count);
        }
    }

    if let Some(matches) = definitions_index.get(&key) {
        for d in matches {
            known = true;
            accepted |= d.can_be_called(count);
        }
    }

    if !known || accepted {
        return None;
    }

    Some(Diagnostic {
        severity: Severity::Error,
        code: CODE,
        message: format!("'{name}' cannot be called with {count} argument(s)."),
        range: call.range(),
    })
}

#[cfg(test)]
mod tests {
    use mlang_core::load_core_api;
    use mlang_parser::parse;
    use mlang_semantic::semantics;
    use mlang_syntax::MFileSource;

    use super::*;

    fn lint_with_core(text: &str) -> Vec<Diagnostic> {
        let core = load_core_api();
        let parsed = parse(text, MFileSource::module());
        let root = parsed.syntax();
        let model = semantics(text, root.clone(), MFileSource::module());
        check(&root, &core, model.definitions())
    }

    #[test]
    fn flags_builtin_call_with_wrong_arity() {
        let diagnostics = lint_with_core("var x = Извлечь(1)");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert_eq!(diagnostics[0].code, CODE);
    }

    #[test]
    fn accepts_builtin_call_with_rest_args() {
        // Извлечь is annotated with argsCount: 2, hasRest: true — 2 required, plus any more.
        let diagnostics = lint_with_core("var x = Извлечь(1, 2, 3)");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn accepts_builtin_call_with_correct_arity() {
        let diagnostics = lint_with_core(r#"var x = Извлечь(1, 2)"#);
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn ignores_unknown_function() {
        let diagnostics = lint_with_core("var x = TotallyUnknown(1, 2, 3)");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn flags_user_function_call_with_wrong_arity() {
        let diagnostics = lint_with_core("func f(a, b) {} f(1)");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert_eq!(diagnostics[0].code, CODE);
    }

    #[test]
    fn accepts_user_function_call_with_correct_arity() {
        let diagnostics = lint_with_core("func f(a, b) {} f(1, 2)");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn matches_user_function_regardless_of_call_site_casing() {
        // The index is keyed by case-folded name -- confirms it still
        // unifies differently-cased spellings the same way
        // `compare_id_with`'s `unicase::eq` did before this was indexed.
        let diagnostics = lint_with_core("func Foo(a, b) {} foo(1)");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, CODE);
    }
}
