//! Post-parse pass that reclassifies string-literal nodes whose content
//! genuinely parses as embedded SQL, so `MSqlStringLiteralExpression`/
//! `MSqlLongStringLiteralExpression` is a real, dialect-independent property
//! of the syntax tree -- not a formatter-only heuristic keyed off a call's
//! name (see the plan this belongs to for the full rationale).

use std::iter;

use biome_rowan::{AstNode, SyntaxRewriter, VisitNodeSignal};
use mlang_syntax::concatenation::{
    build_placeholder_source, flatten_concatenation_chain, substitute_format_placeholders,
};
use mlang_syntax::{AnyMExpression, MBinaryExpression, MLanguage, MSyntaxKind, MSyntaxNode};

/// Cheap pre-filter before attempting a real parse: does `raw` start (after
/// leading whitespace) with one of these keywords, at a word boundary.
/// Keywords alone never decide anything -- only [sql_parser::parses_as_embedded_sql]
/// does -- this just avoids parsing every unrelated string literal in the
/// file.
const SQL_KEYWORDS: &[&str] = &["select", "insert", "update", "delete", "with"];

fn looks_like_sql(raw: &str) -> bool {
    let trimmed = raw.trim_start();
    SQL_KEYWORDS.iter().any(|keyword| {
        let Some(prefix) = trimmed.get(..keyword.len()) else {
            return false;
        };
        if !prefix.eq_ignore_ascii_case(keyword) {
            return false;
        }
        match trimmed[keyword.len()..].chars().next() {
            Some(c) => !c.is_alphanumeric() && c != '_',
            None => true,
        }
    })
}

/// Whether `raw` (a string literal's inner content, quotes already
/// stripped) qualifies as embedded SQL: passes the cheap keyword pre-filter,
/// then actually parses -- after substituting any `{...}` placeholders,
/// same as `try_format_embedded_sql` does for formatting -- via
/// `sql_parser`.
fn validate_as_sql(raw: &str) -> bool {
    if !looks_like_sql(raw) {
        return false;
    }

    match substitute_format_placeholders(raw) {
        Some((substituted, _originals)) => sql_parser::parses_as_embedded_sql(&substituted),
        None => sql_parser::parses_as_embedded_sql(raw),
    }
}

/// A string literal token's raw (still-escaped) inner content, with the
/// surrounding quotes stripped -- matches `try_format_embedded_sql`'s own
/// extraction in `mlang_formatter`.
fn raw_content(text: &str) -> Option<&str> {
    text.get(1..text.len().checked_sub(1)?)
}

/// Whether `node` (already known to be a `+`-[MSyntaxKind::M_BINARY_EXPRESSION])
/// is itself the left or right side of an *enclosing* `+`-chain -- if so, it's
/// not the top of its chain, and whichever ancestor *is* the top will
/// consider the whole chain when the rewriter reaches it (or already has,
/// in which case this node is unreachable and this check is moot).
fn is_nested_in_a_larger_plus_chain(node: &MSyntaxNode) -> bool {
    node.parent()
        .is_some_and(|parent| is_plus_binary_expression(&parent).is_some())
}

fn is_plus_binary_expression(node: &MSyntaxNode) -> Option<MBinaryExpression> {
    let binary = MBinaryExpression::cast_ref(node)?;
    let operator = binary.operator_token().ok()?;
    (operator.kind() == MSyntaxKind::PLUS).then_some(binary)
}

struct SqlLiteralRewriter;

impl SyntaxRewriter for SqlLiteralRewriter {
    type Language = MLanguage;

    fn visit_node(&mut self, node: MSyntaxNode) -> VisitNodeSignal<MLanguage> {
        match node.kind() {
            MSyntaxKind::M_STRING_LITERAL_EXPRESSION => {
                self.visit_string_literal(node, MSyntaxKind::M_SQL_STRING_LITERAL_EXPRESSION)
            }
            MSyntaxKind::M_LONG_STRING_LITERAL_EXPRESSION => {
                self.visit_string_literal(node, MSyntaxKind::M_SQL_LONG_STRING_LITERAL_EXPRESSION)
            }
            MSyntaxKind::M_BINARY_EXPRESSION => self.visit_binary_expression(node),
            _ => VisitNodeSignal::Traverse(node),
        }
    }
}

impl SqlLiteralRewriter {
    fn visit_string_literal(
        &mut self,
        node: MSyntaxNode,
        new_kind: MSyntaxKind,
    ) -> VisitNodeSignal<MLanguage> {
        let Some(token) = node.slots().next().and_then(|slot| slot.into_token()) else {
            return VisitNodeSignal::Traverse(node);
        };

        let Some(raw) = raw_content(token.text_trimmed()) else {
            return VisitNodeSignal::Traverse(node);
        };

        if validate_as_sql(raw) {
            let new_node = MSyntaxNode::new_detached(new_kind, iter::once(Some(token.into())));
            VisitNodeSignal::Replace(new_node)
        } else {
            VisitNodeSignal::Traverse(node)
        }
    }

    fn visit_binary_expression(&mut self, node: MSyntaxNode) -> VisitNodeSignal<MLanguage> {
        let Some(binary) = is_plus_binary_expression(&node) else {
            return VisitNodeSignal::Traverse(node);
        };
        if is_nested_in_a_larger_plus_chain(&node) {
            // Not the top of my chain -- leave the outermost `+` in the
            // chain to consider the whole thing (see [is_plus_binary_expression]).
            return VisitNodeSignal::Traverse(node);
        }

        let expression = AnyMExpression::from(binary);
        let Some(chain) = flatten_concatenation_chain(&expression) else {
            return VisitNodeSignal::Traverse(node);
        };
        let Some(joined) = build_placeholder_source(&chain.parts) else {
            return VisitNodeSignal::Traverse(node);
        };

        if validate_as_sql(&joined) {
            let new_node = MSyntaxNode::new_detached(
                MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION,
                iter::once(Some(node.into())),
            );
            VisitNodeSignal::Replace(new_node)
        } else {
            VisitNodeSignal::Traverse(node)
        }
    }
}

/// Runs the SQL-literal-reclassification pass over an already-parsed tree.
pub(crate) fn rewrite_sql_literals(root: MSyntaxNode) -> MSyntaxNode {
    SqlLiteralRewriter.transform(root)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recognizes_all_five_keywords() {
        for keyword in SQL_KEYWORDS {
            assert!(looks_like_sql(&format!("{keyword} * from t")), "{keyword}");
            assert!(
                looks_like_sql(&format!("{} * from t", keyword.to_uppercase())),
                "{keyword} uppercase"
            );
        }
    }

    #[test]
    fn requires_a_word_boundary_after_the_keyword() {
        assert!(!looks_like_sql("selection of items"));
        assert!(!looks_like_sql("withdraw the request"));
        assert!(!looks_like_sql("insertable rows"));
    }

    #[test]
    fn ignores_leading_whitespace() {
        assert!(looks_like_sql("\n   select a from t"));
    }

    #[test]
    fn rejects_unrelated_text() {
        assert!(!looks_like_sql("just a plain string"));
    }

    #[test]
    fn validates_real_sql_not_just_the_keyword() {
        assert!(validate_as_sql("select a from t"));
        assert!(!validate_as_sql("select from where"));
    }

    #[test]
    fn validates_after_substituting_format_placeholders() {
        assert!(validate_as_sql("select a from t where b = {0}"));
    }

    /// End to end through `crate::parse` -- confirms the rewrite is wired
    /// into the real parse pipeline, not just callable in isolation.
    #[test]
    fn rewrites_a_qualifying_string_literal() {
        let tree = crate::parse(
            "#\nvar q = \"select a from t\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            root.descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_STRING_LITERAL_EXPRESSION)
        );
        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_STRING_LITERAL_EXPRESSION)
        );
    }

    #[test]
    fn leaves_an_unrelated_string_literal_untouched() {
        let tree = crate::parse(
            "#\nvar q = \"just a plain string\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            root.descendants()
                .any(|node| node.kind() == MSyntaxKind::M_STRING_LITERAL_EXPRESSION)
        );
        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_STRING_LITERAL_EXPRESSION)
        );
    }

    #[test]
    fn leaves_a_string_that_fails_to_parse_as_sql_untouched() {
        let tree = crate::parse(
            "#\nvar q = \"select from where\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_STRING_LITERAL_EXPRESSION)
        );
    }

    #[test]
    fn rewrites_a_qualifying_concatenation_chain() {
        let tree = crate::parse(
            "#\nvar q = \"select * from t where a = '\" + x + \"'\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            root.descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION)
        );
        // The wrapped chain's own pieces stay ordinary string literals --
        // only the outermost `+`-chain node itself gets reclassified.
        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_STRING_LITERAL_EXPRESSION)
        );
    }

    #[test]
    fn leaves_an_ordinary_string_concatenation_untouched() {
        let tree = crate::parse(
            "#\nvar q = \"hello \" + name + \"!\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION)
        );
    }

    #[test]
    fn leaves_a_non_plus_binary_expression_untouched() {
        let tree = crate::parse(
            "#\nvar q = (\"select 1\" - x);",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION)
        );
    }

    #[test]
    fn only_wraps_the_outermost_node_of_a_longer_chain() {
        let tree = crate::parse(
            "#\nvar q = \"select * from t where a = '\" + x + \"' and b = '\" + y + \"'\";",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        let wrapped: Vec<_> = root
            .descendants()
            .filter(|node| node.kind() == MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION)
            .collect();
        assert_eq!(wrapped.len(), 1);
    }

    #[test]
    fn leaves_a_chain_that_does_not_validate_as_sql_untouched() {
        let tree = crate::parse(
            "#\nvar q = \"not valid   sql !!!\" + x;",
            mlang_syntax::MFileSource::script(),
        );
        let root = tree.syntax();

        assert!(
            !root
                .descendants()
                .any(|node| node.kind() == MSyntaxKind::M_SQL_CONCATENATION_EXPRESSION)
        );
    }
}
