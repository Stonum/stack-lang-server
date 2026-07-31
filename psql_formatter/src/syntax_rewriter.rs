use biome_formatter::{TransformSourceMap, TransformSourceMapBuilder};
use biome_rowan::{
    AstNode, SyntaxKind, SyntaxToken, TextSize, VisitNodeSignal, chain_trivia_pieces,
};
use psql_syntax::{
    AnyPsqlExpression, PsqlLanguage, PsqlLogicalExpression, PsqlParenthesizedExpression,
    PsqlSyntaxKind, PsqlSyntaxNode, PsqlSyntaxTrivia,
};
use std::collections::BTreeSet;

use biome_rowan::SyntaxRewriter;

pub(super) fn transform(root: PsqlSyntaxNode) -> (PsqlSyntaxNode, TransformSourceMap) {
    let mut rewriter = PsqlFormatSyntaxRewriter::with_offset(root.text_range().start());
    let transformed = rewriter.transform(root);
    (transformed, rewriter.finish())
}

/// Strips redundant `PsqlParenthesizedExpression` nodes from the tree
/// before formatting, and re-balances right-recursive `and`/`or` chains
/// (which only occur as a byproduct of that stripping) into left-recursive
/// ones -- see [psql_syntax::NeedsParentheses] for the other half (deciding
/// where the formatter re-inserts the parentheses this removes). Ported
/// from `mlang_formatter::syntax_rewriter`, trimmed to psql's single
/// `PsqlParenthesizedExpression` kind (no parenthesized-assignment
/// variant, since SQL has nothing analogous to a JS assignment
/// expression).
#[derive(Default)]
struct PsqlFormatSyntaxRewriter {
    source_map: TransformSourceMapBuilder,

    /// Positions at which a `(` has been removed -- needed to correctly
    /// compute source offsets for nested parenthesized expressions with
    /// trivia between them (see the equivalent field's doc comment in
    /// `mlang_formatter::syntax_rewriter` for a worked example).
    l_paren_source_position: BTreeSet<TextSize>,
}

impl PsqlFormatSyntaxRewriter {
    fn with_offset(offset: TextSize) -> Self {
        PsqlFormatSyntaxRewriter {
            source_map: TransformSourceMapBuilder::with_offset(offset),
            ..Default::default()
        }
    }
}

impl PsqlFormatSyntaxRewriter {
    /// Replaces a parenthesized expression that has no syntax error (no
    /// missing required child, no skipped token trivia attached to either
    /// paren) and whose inner expression isn't a bogus node, with the
    /// inner expression -- reattaching the parens' own trivia (comments,
    /// whitespace) onto the inner expression's first/last token. See
    /// `mlang_formatter::syntax_rewriter::visit_parenthesized`'s doc
    /// comment for the full trivia-handling rationale; this is a direct
    /// port, simplified for psql's single parenthesized-expression kind.
    fn visit_parenthesized(
        &mut self,
        parenthesized: PsqlParenthesizedExpression,
    ) -> VisitNodeSignal<PsqlLanguage> {
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
                    // is typed `AnyPsqlAnyAllSource = PsqlSubqueryExpression
                    // | PsqlParenthesizedExpression`, not the general
                    // `AnyPsqlExpression`) -- unlike every other
                    // parenthesized expression, these parens aren't a
                    // redundant grouping that can be safely stripped and
                    // reinserted by `NeedsParentheses`; removing them would
                    // make the tree fail to cast back into that union.
                    || parenthesized
                        .syntax()
                        .parent()
                        .is_some_and(|parent| parent.kind() == PsqlSyntaxKind::PSQL_ANY_ALL_EXPRESSION)
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
                    .with_expression(AnyPsqlExpression::unwrap_cast(inner))
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
    /// `left`. Direct port of
    /// `mlang_formatter::syntax_rewriter::visit_logical_expression`.
    fn visit_logical_expression(
        &mut self,
        logical: PsqlLogicalExpression,
    ) -> VisitNodeSignal<PsqlLanguage> {
        match (logical.left(), logical.operator_token(), logical.right()) {
            (Ok(left), Ok(operator), Ok(right)) => {
                let left_key = left.syntax().key();
                let operator_key = operator.key();
                let right_key = right.syntax().key();

                let left = AnyPsqlExpression::unwrap_cast(self.transform(left.into_syntax()));
                let operator = self.visit_token(operator);
                let right = AnyPsqlExpression::unwrap_cast(self.transform(right.into_syntax()));

                let updated = match right {
                    AnyPsqlExpression::PsqlLogicalExpression(right_logical) => {
                        match (
                            right_logical.left(),
                            right_logical.operator_token(),
                            right_logical.right(),
                        ) {
                            (Ok(right_left), Ok(right_operator), Ok(right_right))
                                if right_operator.kind() == operator.kind() =>
                            {
                                logical
                                    .with_left(
                                        psql_factory::make::psql_logical_expression(
                                            left, operator, right_left,
                                        )
                                        .into(),
                                    )
                                    .with_operator_token_token(right_operator)
                                    .with_right(right_right)
                            }

                            // Don't re-balance a logical expression that
                            // has syntax errors.
                            _ => logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right_logical.into()),
                        }
                    }

                    // Don't re-balance logical expressions with different
                    // operators. Avoid updating the node if none of the
                    // children changed, to avoid re-spinning all parents.
                    right => {
                        if left.syntax().key() != left_key
                            || operator.key() != operator_key
                            || right.syntax().key() != right_key
                        {
                            logical
                                .with_left(left)
                                .with_operator_token_token(operator)
                                .with_right(right)
                        } else {
                            logical
                        }
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

impl SyntaxRewriter for PsqlFormatSyntaxRewriter {
    type Language = PsqlLanguage;

    fn visit_node(&mut self, node: PsqlSyntaxNode) -> VisitNodeSignal<Self::Language> {
        match node.kind() {
            PsqlSyntaxKind::PSQL_PARENTHESIZED_EXPRESSION => {
                self.visit_parenthesized(PsqlParenthesizedExpression::unwrap_cast(node))
            }
            PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION => {
                self.visit_logical_expression(PsqlLogicalExpression::unwrap_cast(node))
            }
            _ => VisitNodeSignal::Traverse(node),
        }
    }

    fn visit_token(&mut self, token: SyntaxToken<Self::Language>) -> SyntaxToken<Self::Language> {
        self.source_map.push_source_text(token.text());
        token
    }
}

fn has_skipped_comment(trivia: &PsqlSyntaxTrivia) -> bool {
    trivia.pieces().any(|piece| piece.is_skipped())
}

#[cfg(test)]
mod tests {
    use super::PsqlFormatSyntaxRewriter;
    use biome_formatter::TransformSourceMap;
    use biome_rowan::{AstNode, SyntaxRewriter};
    use psql_parser::parse;
    use psql_syntax::{PsqlFileSource, PsqlLogicalExpression, PsqlSyntaxNode};

    fn source_map_test(input: &str) -> (PsqlSyntaxNode, TransformSourceMap) {
        let tree = parse(input, PsqlFileSource::script()).syntax();

        let mut rewriter = PsqlFormatSyntaxRewriter::default();
        let transformed = rewriter.transform(tree);
        let source_map = rewriter.finish();

        (transformed, source_map)
    }

    #[test]
    fn rebalances_logical_expressions() {
        let root = parse("select a and (b and c)", PsqlFileSource::script()).syntax();

        let transformed = PsqlFormatSyntaxRewriter::default().transform(root.clone());

        assert_ne!(&transformed, &root);
        assert_eq!(&transformed.text().to_string(), "select a and b and c");

        let mut logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(PsqlLogicalExpression::cast)
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
        let root = parse("select a and (b or c)", PsqlFileSource::script()).syntax();
        let transformed = PsqlFormatSyntaxRewriter::default().transform(root);

        assert_eq!(&transformed.text().to_string(), "select a and b or c");

        let logical_expressions: Vec<_> = transformed
            .descendants()
            .filter_map(PsqlLogicalExpression::cast)
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
