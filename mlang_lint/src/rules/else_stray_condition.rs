use mlang_syntax::{AnyMExpression, AnyMStatement, AstNode, MIfStatement, MSyntaxNode};

use crate::{Diagnostic, Severity};

pub const CODE: &str = "no-condition-in-else";

/// Detects `else(cond) body` — a missing `if` after `else`. `else` does not
/// take a condition: the parenthesized expression parses as the *entire*
/// body of the else-branch (a no-op expression statement), and whatever
/// follows it runs unconditionally, silently defeating the intended guard.
pub fn check(root: &MSyntaxNode) -> Vec<Diagnostic> {
    root.descendants()
        .filter_map(MIfStatement::cast)
        .filter_map(|if_stmt| {
            let alternate = if_stmt.else_clause()?.alternate().ok()?;
            let AnyMStatement::MExpressionStatement(expr_stmt) = alternate else {
                return None;
            };
            let AnyMExpression::MParenthesizedExpression(paren) = expr_stmt.expression().ok()?
            else {
                return None;
            };

            Some(Diagnostic {
                severity: Severity::Error,
                code: CODE,
                message: "'else' does not support conditions — did you mean 'else if (...)'? \
                          As written, this parenthesized expression is the entire body of \
                          the 'else' branch, and any following statement runs unconditionally."
                    .to_string(),
                range: paren.range(),
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
    fn flags_stray_condition_after_else() {
        let diagnostics = lint("if (true) todo() else (false) todo()");
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, Severity::Error);
        assert_eq!(diagnostics[0].code, CODE);
    }

    #[test]
    fn does_not_flag_else_if() {
        let diagnostics = lint("if (true) todo() else if (false) todo()");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn does_not_flag_normal_else() {
        let diagnostics = lint("if (true) { todo() } else { todo() }");
        assert!(diagnostics.is_empty());
    }

    #[test]
    fn does_not_flag_if_without_else() {
        let diagnostics = lint("if (true) todo()");
        assert!(diagnostics.is_empty());
    }
}
