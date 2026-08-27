use biome_formatter::{TransformSourceMap, TransformSourceMapBuilder};
use biome_rowan::{
    AstNode, SyntaxKind, SyntaxToken, TextSize, VisitNodeSignal, chain_trivia_pieces,
};
use sql_syntax::{
    AnySqlExpression, SqlLanguage, SqlLogicalExpression, SqlParenthesizedExpression, SqlSyntaxKind,
    SqlSyntaxNode, SqlSyntaxTrivia,
};
use std::collections::BTreeSet;

use biome_rowan::SyntaxRewriter;

pub(super) fn transform(root: SqlSyntaxNode) -> (SqlSyntaxNode, TransformSourceMap) {
    let mut rewriter = SqlFormatSyntaxRewriter::with_offset(root.text_range().start());
    let transformed = rewriter.transform(root);
    (transformed, rewriter.finish())
}

/// Strips redundant `SqlParenthesizedExpression` nodes from the tree
/// before formatting, and re-balances right-recursive `and`/`or` chains
/// (which only occur as a byproduct of that stripping) into left-recursive
/// ones -- see [sql_syntax::NeedsParentheses] for the other half (deciding
/// where the formatter re-inserts the parentheses this removes). Ported
/// from `mlang_formatter::syntax_rewriter`, trimmed to sql's single
/// `SqlParenthesizedExpression` kind (no parenthesized-assignment
/// variant, since SQL has nothing analogous to a JS assignment
/// expression).
#[derive(Default)]
struct SqlFormatSyntaxRewriter {
    source_map: TransformSourceMapBuilder,

    /// Positions at which a `(` has been removed -- needed to correctly
    /// compute source offsets for nested parenthesized expressions with
    /// trivia between them (see the equivalent field's doc comment in
    /// `mlang_formatter::syntax_rewriter` for a worked example).
    l_paren_source_position: BTreeSet<TextSize>,
}

impl SqlFormatSyntaxRewriter {
    fn with_offset(offset: TextSize) -> Self {
        SqlFormatSyntaxRewriter {
            source_map: TransformSourceMapBuilder::with_offset(offset),
            ..Default::default()
        }
    }
}

impl SqlFormatSyntaxRewriter {
    /// Replaces a parenthesized expression that has no syntax error (no
    /// missing required child, no skipped token trivia attached to either
    /// paren) and whose inner expression isn't a bogus node, with the
    /// inner expression -- reattaching the parens' own trivia (comments,
    /// whitespace) onto the inner expression's first/last token. See
    /// `mlang_formatter::syntax_rewriter::visit_parenthesized`'s doc
    /// comment for the full trivia-handling rationale; this is a direct
    /// port, simplified for sql's single parenthesized-expression kind.
    fn visit_parenthesized(
        &mut self,
        parenthesized: SqlParenthesizedExpression,
    ) -> VisitNodeSignal<SqlLanguage> {
        let (l_paren, inner, r_paren) = match (
            parenthesized.l_paren_token(),
            parenthesized.expression(),
            parenthesized.r_paren_token(),
        ) {
            (Ok(l_paren), Ok(inner), Ok(r_paren)) => {
                let prev_token = l_paren.prev_token();
                // Keep parentheses around unknown expressions -- the
                // formatter can't know the precedence.
                if inner.syntax().kind().is_bogus()
                    // Don't remove parentheses with skipped trivia -- the
                    // intended syntax isn't certain.
                    || has_skipped_comment(&l_paren.leading_trivia())
                    || prev_token
                        .is_some_and(|prev| has_skipped_comment(&prev.trailing_trivia()))
                    || r_paren.leading_trivia().has_skipped()
                    // `any (...)`/`all (...)`/`some (...)` requires literal
                    // parens as part of its own grammar (its `source` field
                    // is typed `AnySqlAnyAllSource = SqlSubqueryExpression
                    // | SqlParenthesizedExpression`, not the general
                    // `AnySqlExpression`) -- unlike every other
                    // parenthesized expression, these parens aren't a
                    // redundant grouping that can be safely stripped and
                    // reinserted by `NeedsParentheses`; removing them would
                    // make the tree fail to cast back into that union.
                    || parenthesized
                        .syntax()
                        .parent()
                        .is_some_and(|parent| parent.kind() == SqlSyntaxKind::SQL_ANY_ALL_EXPRESSION)
                {
                    return VisitNodeSignal::Traverse(parenthesized.into_syntax());
                } else {
                    (l_paren, inner.into_syntax(), r_paren)
                }
            }
            _ => {
                // At least one missing child -- handle as a regular node.
                return VisitNodeSignal::Traverse(parenthesized.into_syntax());
            }
        };

        self.source_map.push_source_text(l_paren.text());

        let inner_trimmed_range = inner.text_trimmed_range();
        let original_inner_offset = inner.text_range().start();
        let inner = self.transform(inner);
        let inner_offset = original_inner_offset - inner.text_range().start();

        match inner.first_token() {
            // Can only happen for `()`, which is never valid syntax here
            // anyway -- keep the parenthesized expression as is, it'll be
            // formatted verbatim.
            None => {
                let updated = parenthesized
                    .with_expression(AnySqlExpression::unwrap_cast(inner))
                    .into_syntax();

                self.source_map.push_source_text(r_paren.text());

                VisitNodeSignal::Replace(updated)
            }

            Some(first_token) => {
                self.source_map.extend_trimmed_node_range(
                    inner_trimmed_range,
                    parenthesized.syntax().text_trimmed_range(),
                );

                let l_paren_trimmed_range = l_paren.text_trimmed_range();
                self.source_map.add_deleted_range(l_paren_trimmed_range);
                self.l_paren_source_position
                    .insert(l_paren_trimmed_range.start());

                let mut l_paren_trailing = l_paren.trailing_trivia().pieces().peekable();

                // Skip over leading whitespace.
                while let Some(piece) = l_paren_trailing.peek() {
                    if piece.is_whitespace() {
                        self.source_map.add_deleted_range(piece.text_range());
                        l_paren_trailing.next();
                    } else {
                        break;
                    }
                }

                let l_paren_trailing_non_whitespace_trivia = l_paren_trailing
                    .peek()
                    .is_some_and(|piece| piece.is_skipped() || piece.is_comments());

                let l_paren_trivia =
                    chain_trivia_pieces(l_paren.leading_trivia().pieces(), l_paren_trailing);

                let mut leading_trivia = first_token.leading_trivia().pieces().peekable();
                let mut first_new_line = None;

                let mut inner_offset = inner_offset;

                // The leading whitespace before the opening paren replaces
                // the whitespace before the node.
                while let Some(trivia) = leading_trivia.peek() {
                    if self
                        .l_paren_source_position
                        .contains(&(trivia.text_range().start() + inner_offset))
                    {
                        inner_offset += TextSize::from(1);
                    }

                    if trivia.is_newline() && first_new_line.is_none() {
                        first_new_line = Some((
                            trivia.text_range() + inner_offset,
                            leading_trivia.next().unwrap(),
                        ));
                    } else if trivia.is_whitespace() || trivia.is_newline() {
                        self.source_map
                            .add_deleted_range(trivia.text_range() + inner_offset);
                        leading_trivia.next();
                    } else {
                        break;
                    }
                }

                // Remove all leading new lines directly in front of the
                // token, but keep one if it precedes a skipped token or a
                // comment.
                if !l_paren_trailing_non_whitespace_trivia
                    && leading_trivia.peek().is_none()
                    && first_new_line.is_some()
                {
                    let (inner_offset, _) = first_new_line.take().unwrap();

                    self.source_map.add_deleted_range(inner_offset);
                }

                let leading_trivia = chain_trivia_pieces(
                    first_new_line.map(|(_, trivia)| trivia).into_iter(),
                    leading_trivia,
                );

                let new_leading = chain_trivia_pieces(l_paren_trivia, leading_trivia);
                let new_first = first_token.with_leading_trivia_pieces(new_leading);

                // SAFETY: `inner_first` is part of the `inner` subtree.
                let updated = inner
                    .replace_child(first_token.into(), new_first.into())
                    .unwrap();

                let r_paren_trivia = chain_trivia_pieces(
                    r_paren.leading_trivia().pieces(),
                    r_paren.trailing_trivia().pieces(),
                );

                // SAFETY: `last_token` only returns `None` if the subtree
                // has no tokens at all, but it contains at least
                // `first_token`.
                let last_token = updated.last_token().unwrap();

                let new_last = last_token.append_trivia_pieces(r_paren_trivia);

                self.source_map
                    .add_deleted_range(r_paren.text_trimmed_range());

                self.source_map.push_source_text(r_paren.text());

                // SAFETY: `last_token` is part of the `updated` subtree.
                VisitNodeSignal::Replace(
                    updated
                        .replace_child(last_token.into(), new_last.into())
                        .unwrap(),
                )
            }
        }
    }

    /// Re-balances right-recursive `and`/`or` chains with the same
    /// operator to be left-recursive (only reachable after parentheses
    /// removal, e.g. `a and (b and c)` -> `a and b and c`), since
    /// `logical_expression.rs`'s chain-flattening only ever walks down
    /// `left`. A parenthesized group of 3+ conditions
    /// (`a and (b and c and d)`) parses its own insides left-recursively
    /// too, so the tree is `and(a, and(and(b,c), d))` -- the group's `left`
    /// (`and(b,c)`) is itself a chain, not a single atom, so pulling up
    /// only the immediate `right.left()`/`right.right()` pair (as a single
    /// rotation would) just relocates the imbalance one level down instead
    /// of removing it. Flattening both sides down to their individual
    /// atoms first and folding them back left-associatively handles any
    /// nesting depth in one pass.
    fn visit_logical_expression(
        &mut self,
        logical: SqlLogicalExpression,
    ) -> VisitNodeSignal<SqlLanguage> {
        match (logical.left(), logical.operator_token(), logical.right()) {
            (Ok(left), Ok(operator), Ok(right)) => {
                let left_key = left.syntax().key();
                let operator_key = operator.key();
                let right_key = right.syntax().key();

                let left = AnySqlExpression::unwrap_cast(self.transform(left.into_syntax()));
                let operator = self.visit_token(operator);
                let right = AnySqlExpression::unwrap_cast(self.transform(right.into_syntax()));

                let operator_kind = operator.kind();
                let mut atoms = Vec::new();
                let mut operators = Vec::new();
                flatten_same_operator_chain(left, operator_kind, &mut atoms, &mut operators);
                operators.push(operator);
                flatten_same_operator_chain(right, operator_kind, &mut atoms, &mut operators);

                let updated: AnySqlExpression = if atoms.len() > 2 {
                    let mut atoms = atoms.into_iter();
                    let mut operators = operators.into_iter();
                    let mut acc = atoms.next().expect("flatten always keeps the left operand");
                    for atom in atoms {
                        let op = operators
                            .next()
                            .expect("one operator between each pair of flattened atoms");
                        acc = sql_factory::make::sql_logical_expression(acc, op, atom).into();
                    }
                    acc
                } else {
                    // Neither side was itself a same-operator chain --
                    // nothing to flatten. Avoid updating the node if none
                    // of the children changed, to avoid re-spinning all
                    // parents.
                    let mut atoms = atoms.into_iter();
                    let new_left = atoms.next().expect("exactly two atoms in this branch");
                    let new_right = atoms.next().expect("exactly two atoms in this branch");
                    let new_operator = operators
                        .into_iter()
                        .next()
                        .expect("exactly one operator in this branch");

                    if new_left.syntax().key() != left_key
                        || new_operator.key() != operator_key
                        || new_right.syntax().key() != right_key
                    {
                        logical
                            .with_left(new_left)
                            .with_operator_token_token(new_operator)
                            .with_right(new_right)
                            .into()
                    } else {
                        logical.into()
                    }
                };

                VisitNodeSignal::Replace(updated.into_syntax())
            }
            _ => VisitNodeSignal::Traverse(logical.into_syntax()),
        }
    }

    fn finish(self) -> TransformSourceMap {
        self.source_map.finish()
    }
}

/// Recursively flattens `expr` into its individual atoms wherever it (and
/// its descendants) is a `SqlLogicalExpression` with `operator_kind`,
/// appending the operator between each adjacent pair to `operators` --
/// so `atoms.len() == operators.len() + 1` once both `left` and `right` of
/// a node have been flattened this way. Stops descending (treats the
/// subtree as one opaque atom) at a different operator, a non-logical
/// expression, or a logical expression with a syntax error, since none of
/// those can be safely re-associated.
fn flatten_same_operator_chain(
    expr: AnySqlExpression,
    operator_kind: SqlSyntaxKind,
    atoms: &mut Vec<AnySqlExpression>,
    operators: &mut Vec<SyntaxToken<SqlLanguage>>,
) {
    if let AnySqlExpression::SqlLogicalExpression(ref logical) = expr
        && let (Ok(left), Ok(operator), Ok(right)) =
            (logical.left(), logical.operator_token(), logical.right())
        && operator.kind() == operator_kind
    {
        flatten_same_operator_chain(left, operator_kind, atoms, operators);
        operators.push(operator);
        flatten_same_operator_chain(right, operator_kind, atoms, operators);
        return;
    }

    atoms.push(expr);
}

impl SyntaxRewriter for SqlFormatSyntaxRewriter {
    type Language = SqlLanguage;

    fn visit_node(&mut self, node: SqlSyntaxNode) -> VisitNodeSignal<Self::Language> {
        match node.kind() {
            SqlSyntaxKind::SQL_PARENTHESIZED_EXPRESSION => {
                self.visit_parenthesized(SqlParenthesizedExpression::unwrap_cast(node))
            }
            SqlSyntaxKind::SQL_LOGICAL_EXPRESSION => {
                self.visit_logical_expression(SqlLogicalExpression::unwrap_cast(node))
            }
            _ => VisitNodeSignal::Traverse(node),
        }
    }

    fn visit_token(&mut self, token: SyntaxToken<Self::Language>) -> SyntaxToken<Self::Language> {
        self.source_map.push_source_text(token.text());
        token
    }
}

fn has_skipped_comment(trivia: &SqlSyntaxTrivia) -> bool {
    trivia.pieces().any(|piece| piece.is_skipped())
}

#[cfg(test)]
mod tests {
    use super::SqlFormatSyntaxRewriter;
    use biome_formatter::TransformSourceMap;
    use biome_rowan::{AstNode, SyntaxRewriter};
    use sql_parser::parse;
    use sql_syntax::{SqlFileSource, SqlLogicalExpression, SqlSyntaxNode};

    fn source_map_test(input: &str) -> (SqlSyntaxNode, TransformSourceMap) {
        let tree = parse(input, SqlFileSource::script()).syntax();

        let mut rewriter = SqlFormatSyntaxRewriter::default();
        let transformed = rewriter.transform(tree);
        let source_map = rewriter.finish();

        (transformed, source_map)
    }

    #[test]
    fn rebalances_a_parenthesized_group_of_three_or_more_conditions() {
        // Regression test: a single rotation only pulls the parenthesized
        // group's immediate `right.left()`/`right.right()` pair up to the
        // top -- for a group of 3+ conditions, that `right.left()` is
        // itself a chain (`b and c`), not a single atom, so one rotation
        // just relocates the imbalance instead of removing it. The fully
        // flattened, left-recursive shape is `and(and(and(a,b),c),d)`,
        // where every node's `left` accumulates the chain so far and
        // `right` is always a single atom.
        let root = parse("select a and (b and c and d)", SqlFileSource::script()).syntax();
        let transformed = SqlFormatSyntaxRewriter::default().transform(root);

        assert_eq!(
            &transformed.text().to_string(),
            "select a and b and c and d"
        );

        let mut logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(SqlLogicalExpression::cast)
            .collect();
        assert_eq!(logical_expressions.len(), 3);

        let ab = logical_expressions.pop().unwrap();
        let abc = logical_expressions.pop().unwrap();
        let abcd = logical_expressions.pop().unwrap();

        assert_eq!(ab.left().unwrap().text(), "a");
        assert_eq!(ab.right().unwrap().text(), "b");

        assert_eq!(abc.left().unwrap().syntax(), ab.syntax());
        assert_eq!(abc.right().unwrap().text(), "c");

        assert_eq!(abcd.left().unwrap().syntax(), abc.syntax());
        assert_eq!(abcd.right().unwrap().text(), "d");
    }

    #[test]
    fn rebalances_logical_expressions() {
        let root = parse("select a and (b and c)", SqlFileSource::script()).syntax();

        let transformed = SqlFormatSyntaxRewriter::default().transform(root.clone());

        assert_ne!(&transformed, &root);
        assert_eq!(&transformed.text().to_string(), "select a and b and c");

        let mut logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(SqlLogicalExpression::cast)
            .collect();

        assert_eq!(logical_expressions.len(), 2);

        let left = logical_expressions.pop().unwrap();
        let top = logical_expressions.pop().unwrap();

        assert_eq!(top.left().unwrap().syntax(), left.syntax());
        assert_eq!(&top.right().unwrap().text(), "c");

        assert_eq!(left.left().unwrap().text(), "a");
        assert_eq!(left.right().unwrap().text(), "b");
    }

    #[test]
    fn only_rebalances_logical_expressions_with_same_operator() {
        let root = parse("select a and (b or c)", SqlFileSource::script()).syntax();
        let transformed = SqlFormatSyntaxRewriter::default().transform(root);

        assert_eq!(&transformed.text().to_string(), "select a and b or c");

        let logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(SqlLogicalExpression::cast)
            .collect();

        assert_eq!(logical_expressions.len(), 2);

        let top = logical_expressions.first().unwrap();
        let right = logical_expressions.last().unwrap();

        assert_eq!(top.left().unwrap().text(), "a");
        assert_eq!(top.right().unwrap().syntax(), right.syntax());
        assert_eq!(right.left().unwrap().text(), "b");
        assert_eq!(right.right().unwrap().text(), "c");
    }

    #[test]
    fn single_parentheses() {
        let (transformed, source_map) = source_map_test("select (a)");

        assert_eq!(&transformed.text(), "select a");

        let binary = transformed
            .descendants()
            .find(|node| node.text() == "(a)" || node.text() == "a")
            .unwrap();

        assert_eq!(source_map.trimmed_source_text(&binary), "(a)");
    }

    #[test]
    fn nested_parentheses() {
        let (transformed, source_map) = source_map_test("select ((a))");

        assert_eq!(&transformed.text(), "select a");

        let node = transformed
            .descendants()
            .find(|node| node.text() == "a")
            .unwrap();

        assert_eq!(source_map.trimmed_source_text(&node), "((a))");
    }

    #[test]
    fn adjacent_nodes() {
        let (transformed, source_map) = source_map_test("select (a + b)");

        assert_eq!(&transformed.text(), "select a + b");

        let binary = transformed
            .descendants()
            .find(|node| node.text() == "a + b")
            .unwrap();
        assert_eq!(source_map.trimmed_source_text(&binary), "(a + b)");
    }
}
