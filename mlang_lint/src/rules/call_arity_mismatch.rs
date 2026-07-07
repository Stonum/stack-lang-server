use biome_rowan::AstSeparatedList;
use mlang_core::AnyMCoreDefinition;
use mlang_lsp_definition::CodeSymbolDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::{AnyMExpression, AstNode, MCallExpression, MSyntaxNode};

use crate::{Diagnostic, Severity};

pub const CODE: &str = "call-arity-mismatch";

pub fn check<'a>(
    root: &MSyntaxNode,
    core: &[AnyMCoreDefinition],
    definitions: impl Iterator<Item = &'a AnyMDefinition> + Clone,
) -> Vec<Diagnostic> {
    root.descendants()
        .filter_map(MCallExpression::cast)
        .filter_map(|call| check_call(&call, core, definitions.clone()))
        .collect()
}

fn check_call<'a>(
    call: &MCallExpression,
    core: &[AnyMCoreDefinition],
    definitions: impl Iterator<Item = &'a AnyMDefinition>,
) -> Option<Diagnostic> {
    let AnyMExpression::MIdentifierExpression(ident) = call.callee().ok()? else {
        return None;
    };
    let name = ident.name().ok()?.text();
    let arguments = call.arguments().ok()?;
    let count = arguments.args().len();

    let mut known = false;
    let mut accepted = false;

    for d in core
        .iter()
        .filter(|d| d.is_function() && d.compare_id_with(&name))
    {
        known = true;
        accepted |= d.can_be_called(count);
    }

    for d in definitions.filter(|d| d.is_function() && d.compare_id_with(&name)) {
        known = true;
        accepted |= d.can_be_called(count);
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
}
