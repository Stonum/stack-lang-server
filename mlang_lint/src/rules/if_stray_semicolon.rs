use mlang_syntax::{AnyMStatement, AstNode, MIfStatement, MSyntaxNode};

use crate::{Diagnostic, Severity};

pub const CODE: &str = "no-empty-if";

pub fn check(root: &MSyntaxNode) -> Vec<Diagnostic> {
    root.descendants()
        .filter_map(MIfStatement::cast)
        .filter_map(|if_stmt| {
            let consequent = if_stmt.consequent().ok()?;
            let AnyMStatement::MEmptyStatement(_) = consequent else {
                return None;
            };

            Some(Diagnostic {
                severity: Severity::Warning,
                code: CODE,
                message: "Empty statement (';') right after 'if (...)' \
                          — the intended statement is never executed."
                    .to_string(),
                range: if_stmt.range(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    use super::*;

    fn lint(text: &str) -> Vec<Diagnostic> {
        let parsed = parse(text, MFileSource::script());
        check(&parsed.syntax())
    }

    #[test]
    fn flags_stray_semicolon_after_if() {
        let diagnostics = lint("if (x == 1);\n f();");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Warning);
        assert_eq!(diagnostics[0].code, CODE);
    }

    #[test]
    fn does_not_flag_normal_if() {
        let diagnostics = lint("if (x == 1) { f(); }");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn does_not_flag_if_without_body() {
        let diagnostics = lint("if (x == 1) f();");
        assert!(diagnostics.is_empty());
    }
}
