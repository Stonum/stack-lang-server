use biome_rowan::AstSeparatedList;
use lsp_definition::CodeSymbolDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::{
    AnyMExpression, AstNode, MCallExpression, MClassDeclaration, MNewExpression, TextRange,
};

use super::SemanticIndex;
use crate::{Diagnostic, DiagnosticTag, Severity};

pub const CODE: &str = "deprecated";

/// Guards the `this.method()` lookup against cyclic `extends` chains.
const MAX_INHERITANCE_DEPTH: usize = 16;

fn method<'a>(
    index: &SemanticIndex<'a>,
    class: &str,
    name: &str,
    count: usize,
) -> Option<Deprecation<'a>> {
    let mut class = class;
    for _ in 0..MAX_INHERITANCE_DEPTH {
        let mut methods = index.methods(class, name).peekable();
        if methods.peek().is_some() {
            return deprecation(callable(methods, count));
        }
        class = index.classes(class).find_map(|c| c.parent())?;
    }
    None
}

/// Reason of a deprecated symbol usage.
struct Deprecation<'a>(&'a str);

fn callable<'a>(
    candidates: impl Iterator<Item = &'a AnyMDefinition>,
    count: usize,
) -> impl Iterator<Item = &'a AnyMDefinition> {
    candidates.filter(move |d| d.can_be_called(count))
}

/// A usage is deprecated only if every resolved candidate is deprecated,
/// so overloads and same-named definitions from other files don't produce false positives.
fn deprecation<'a>(
    mut candidates: impl Iterator<Item = &'a AnyMDefinition>,
) -> Option<Deprecation<'a>> {
    let mut reason = candidates.next()?.deprecated()?;
    for d in candidates {
        let other = d.deprecated()?;
        if reason.is_empty() {
            reason = other;
        }
    }
    Some(Deprecation(reason))
}

pub fn check_call(call: &MCallExpression, index: &SemanticIndex) -> Option<Diagnostic> {
    let count = call.arguments().ok()?.args().len();

    match call.callee().ok()? {
        AnyMExpression::MIdentifierExpression(ident) => {
            let name = ident.name().ok()?.text();
            let deprecation = deprecation(callable(index.functions(&name), count))?;
            Some(diagnostic(&name, deprecation, ident.range()))
        }
        AnyMExpression::MStaticMemberExpression(member) => {
            let AnyMExpression::MThisExpression(_) = member.object().ok()? else {
                return None;
            };
            let class = member
                .syntax()
                .ancestors()
                .find_map(MClassDeclaration::cast)?
                .id()
                .ok()?
                .text();
            let name = member.member().ok()?;
            let deprecation = method(index, &class, &name.text(), count)?;
            Some(diagnostic(&name.text(), deprecation, name.range()))
        }
        _ => None,
    }
}

pub fn check_new(new: &MNewExpression, index: &SemanticIndex) -> Option<Diagnostic> {
    let AnyMExpression::MIdentifierExpression(ident) = new.callee().ok()? else {
        return None;
    };
    let name = ident.name().ok()?.text();

    if let Some(deprecation) = deprecation(index.classes(&name)) {
        return Some(diagnostic(&name, deprecation, ident.range()));
    }

    let count = new.arguments().map_or(0, |args| args.args().len());
    let deprecation = deprecation(callable(index.constructors(&name), count))?;
    Some(diagnostic(&name, deprecation, ident.range()))
}

fn diagnostic(name: &str, Deprecation(reason): Deprecation, range: TextRange) -> Diagnostic {
    let message = if reason.is_empty() {
        format!("'{name}' is deprecated.")
    } else {
        format!("'{name}' is deprecated: {reason}")
    };

    Diagnostic {
        severity: Severity::Warning,
        code: CODE,
        message,
        range,
        tags: vec![DiagnosticTag::Deprecated],
    }
}

#[cfg(test)]
mod tests {
    use line_index::LineIndex;
    use mlang_parser::parse;
    use mlang_semantic::semantics;
    use mlang_syntax::MFileSource;

    use super::*;

    fn lint(text: &str) -> Vec<(String, String)> {
        let parsed = parse(text, MFileSource::module());
        let root = parsed.syntax();
        let model = semantics(&LineIndex::new(text), root.clone(), MFileSource::module());
        crate::tests::lint(&root, &model)
            .into_iter()
            .filter(|d| d.code == CODE)
            .map(|d| (text[d.range].to_string(), d.message))
            .collect()
    }

    #[test]
    fn flags_deprecated_function_call_with_reason() {
        let diagnostics = lint(
            r#"
# @deprecated use g()
func f() {}
f();
"#,
        );
        assert_eq!(
            diagnostics,
            vec![("f".into(), "'f' is deprecated: use g()".into())]
        );
    }

    #[test]
    fn flags_deprecated_function_from_doc_string() {
        let diagnostics = lint(
            r#"
func f(a) "
    about f
    @deprecated
" {}
var x = F(1);
"#,
        );
        assert_eq!(diagnostics, vec![("F".into(), "'F' is deprecated.".into())]);
    }

    #[test]
    fn ignores_not_deprecated_function() {
        let diagnostics = lint(
            r#"
# about f
func f() {}
f();
"#,
        );
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn resolves_overload_by_arity() {
        let diagnostics = lint(
            r#"
# @deprecated
func f(a) {}
func f(a, b) {}
f(1, 2);
f(1);
"#,
        );
        assert_eq!(diagnostics, vec![("f".into(), "'f' is deprecated.".into())]);
    }

    #[test]
    fn ignores_when_not_every_candidate_is_deprecated() {
        let diagnostics = lint(
            r#"
# @deprecated
func f(a) {}
func f(b) {}
f(1);
"#,
        );
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn flags_deprecated_class_and_constructor() {
        let diagnostics = lint(
            r#"
# @deprecated use B
class A {}

class B {
    constructor(a) "@deprecated pass two args" {}
    constructor(a, b) {}
}

var a = new A();
var b1 = new B(1);
var b2 = new B(1, 2);
"#,
        );
        assert_eq!(
            diagnostics,
            vec![
                ("A".into(), "'A' is deprecated: use B".into()),
                ("B".into(), "'B' is deprecated: pass two args".into()),
            ]
        );
    }

    #[test]
    fn flags_deprecated_method_called_on_this() {
        let diagnostics = lint(
            r#"
class A {
    # @deprecated use n()
    m() {}
}

class B extends A {
    run() {
        this.m();
        other.m();
    }
}
"#,
        );
        assert_eq!(
            diagnostics,
            vec![("m".into(), "'m' is deprecated: use n()".into())]
        );
    }
}
