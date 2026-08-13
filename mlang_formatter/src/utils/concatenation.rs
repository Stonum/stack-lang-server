use super::string_utils::{escape_for_string_literal, format_sql_source, quote_as_static_str};
use crate::prelude::*;

use biome_formatter::write;
use biome_rowan::TextSize;
use mlang_syntax::AnyMExpression;
pub(crate) use mlang_syntax::concatenation::{
    ConcatenationChain, ConcatenationPart, build_placeholder_source, flatten_concatenation_chain,
    hole_placeholder,
};

/// Whether any node bypassed by [FormatConcatenatedQuery] (every literal's
/// own string-expression node, every intermediate `+`-binary-expression
/// node that gets flattened away) carries a leading or trailing comment.
/// [FormatConcatenatedQuery] never calls these nodes' own `.format()` (it
/// reconstructs the visible output from scratch instead), so a comment
/// attached to one of them would otherwise silently vanish -- safer to
/// bail out and leave the whole call verbatim, matching this feature's
/// "never risk corrupting/losing what we don't fully handle" stance
/// elsewhere (see `string_utils.rs::try_format_embedded_sql`).
///
/// This also marks every one of those nodes as "checked for a suppression
/// comment" (`biome_formatter`'s own bookkeeping, via
/// `Comments::mark_suppression_checked`) regardless of the outcome --
/// required because they never go through the normal per-node `.format()`
/// dispatch that would otherwise do this automatically, or a debug build
/// panics ("formatted without checking if it has suppression comments").
pub(crate) fn chain_has_comments(chain: &ConcatenationChain, comments: &MComments) -> bool {
    let literal_nodes = chain.parts.iter().filter_map(|part| match part {
        ConcatenationPart::Literal(token) => token.parent(),
        ConcatenationPart::Hole(_) => None,
    });
    let operator_nodes = chain.operators.iter().filter_map(|token| token.parent());

    let mut has_comments = false;
    for node in literal_nodes.chain(operator_nodes) {
        comments.mark_suppression_checked(&node);
        has_comments |= comments.has_comments(&node);
    }

    has_comments
}

/// One piece of a reformatted concatenation chain, in output order: either
/// a freshly SQL-formatted literal text segment, or the index (into the
/// original `parts` passed to [format_concatenation_chain]) of a hole that
/// appeared at that position -- the caller re-formats that original
/// expression itself ([ConcatenationPart::Hole]'s own `Format` impl), this
/// module never touches hole content.
pub(crate) enum FormattedPiece {
    Literal(String),
    Hole(usize),
}

/// Parses and reformats a flattened concatenation chain as one SQL
/// document (see [build_placeholder_source]), then splits the result back
/// into pieces at the placeholder positions. Returns `None` if the joined
/// text doesn't parse/format cleanly, or if placeholder substitution itself
/// was refused -- in either case the caller must fall back to leaving the
/// whole call verbatim (see `call_arguments.rs`), not a structural
/// `+`-chain reformat: a hole landing flush against SQL keyword text with
/// no separator, for instance, might genuinely be invalid at runtime, or
/// might be fine if the hole's own value supplies the missing separator --
/// unknowable from the source text alone, so nothing here should be
/// second-guessed by reformatting the surrounding structure either.
pub(crate) fn format_concatenation_chain(
    parts: &[ConcatenationPart],
    f: &MFormatter,
) -> Option<Vec<FormattedPiece>> {
    let joined = build_placeholder_source(parts)?;
    let formatted = format_sql_source(&joined, f)?;

    let pieces = split_formatted_concatenation(&formatted, parts)?;

    // Bail out only for the specific case that actually reads as broken: a
    // *multi-line* piece where a hole lands inside a still-open nested SQL
    // string/identifier literal (an odd number of `'`/`"` in the piece) --
    // e.g. `"select a\nfrom t\nwhere x = '"` followed by a hole, then
    // `"'\n   and y = '"` for the next one. The dangling quote ends up
    // isolated on its own line, disconnected from the text it belongs to.
    // A plain multi-line piece with *balanced* quotes (a hole substituting
    // for a bare identifier/expression, not sitting inside a nested string)
    // is a perfectly normal, readable result and must NOT be rejected --
    // see `concatenation_multiline_sql_with_holes_at_clause_boundaries`.
    if pieces.iter().any(|piece| match piece {
        FormattedPiece::Literal(text) if text.contains('\n') => !has_balanced_quotes(text),
        _ => false,
    }) {
        return None;
    }

    Some(pieces)
}

/// Whether `text` has an even number of `'` and of `"` characters. An odd
/// count means the text ends (or starts) inside a still-open quoted
/// string/identifier -- Postgres represents a literal `'` inside a string
/// by doubling it (`''`), which conveniently still counts as an even,
/// balance-preserving pair here.
fn has_balanced_quotes(text: &str) -> bool {
    text.chars().filter(|&c| c == '\'').count() % 2 == 0
        && text.chars().filter(|&c| c == '"').count() % 2 == 0
}

/// Locates each hole's placeholder in `formatted`, in order (the formatter
/// never reorders tokens, so the placeholders can only appear in the same
/// left-to-right order they were substituted in), and slices the text
/// between them into literal pieces. Returns `None` if a placeholder that
/// was substituted can't be found in the formatted output -- which would
/// mean `sql_formatter` altered or dropped it, an unexpected condition this
/// code must not paper over.
///
/// Because [flatten_concatenation_chain] only ever produces strictly
/// alternating parts, every [FormattedPiece::Literal] pushed here
/// corresponds to exactly one input [ConcatenationPart::Literal], in the
/// same order -- there's never a run of several original literals merged
/// into one output piece.
fn split_formatted_concatenation(
    formatted: &str,
    parts: &[ConcatenationPart],
) -> Option<Vec<FormattedPiece>> {
    let mut pieces = Vec::new();
    let mut cursor = 0usize;

    for (index, part) in parts.iter().enumerate() {
        if matches!(part, ConcatenationPart::Hole(_)) {
            let placeholder = hole_placeholder(index);
            let found_at = cursor + formatted.get(cursor..)?.find(&placeholder)?;

            if found_at > cursor {
                pieces.push(FormattedPiece::Literal(
                    formatted[cursor..found_at].to_string(),
                ));
            }
            pieces.push(FormattedPiece::Hole(index));
            cursor = found_at + placeholder.len();
        }
    }

    if cursor < formatted.len() {
        pieces.push(FormattedPiece::Literal(formatted[cursor..].to_string()));
    }

    Some(pieces)
}

/// A concatenation chain that's been successfully reformatted end to end
/// (flattened, joined, parsed, SQL-formatted, and split back into pieces),
/// ready to be written back out via its `Format` impl.
pub(crate) struct ConcatenatedQuery {
    chain: ConcatenationChain,
    pieces: Vec<FormattedPiece>,
}

impl ConcatenatedQuery {
    /// Attempts the whole pipeline for `expression` (assumed to already be
    /// a top-level `+` [AnyMExpression::MBinaryExpression] -- this doesn't
    /// check that itself). Returns `None` at the first failure -- not a
    /// `+`-concatenation shape we understand, adjacent same-kind parts,
    /// stray comments, unparseable/unformattable joined SQL -- in which
    /// case the caller must leave the whole call verbatim
    /// (`call_arguments.rs`), the same as a single query-like string
    /// argument that fails to parse: a `+`-chain we couldn't confidently
    /// reformat as SQL might still be a perfectly valid *runtime* query
    /// (e.g. a hole's own value could supply a separator the source text
    /// doesn't show), so reformatting just the surrounding mlang structure
    /// while leaving the content untouched would be actively misleading,
    /// not merely unhelpful.
    pub(crate) fn try_new(expression: &AnyMExpression, f: &MFormatter) -> Option<Self> {
        let chain = flatten_concatenation_chain(expression)?;
        if chain_has_comments(&chain, f.comments()) {
            return None;
        }
        if wrapper_has_comments(expression.syntax(), f.comments()) {
            return None;
        }

        let pieces = format_concatenation_chain(&chain.parts, f)?;

        Some(Self { chain, pieces })
    }
}

/// Whether an ancestor co-starting with `node` (the `MSqlConcatenationExpression`
/// wrapper) carries a leading comment -- biome attaches it there, not to
/// the chain's first literal piece, so [chain_has_comments] alone misses it.
fn wrapper_has_comments(node: &mlang_syntax::MSyntaxNode, comments: &MComments) -> bool {
    let start = node.text_range().start();
    let mut ancestor = node.parent();
    while let Some(current) = ancestor {
        if current.text_range().start() != start {
            break;
        }
        if comments.has_comments(&current) {
            return true;
        }
        ancestor = current.parent();
    }
    false
}

impl Format<MFormatContext> for ConcatenatedQuery {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        FormatConcatenatedQuery {
            chain: &self.chain,
            pieces: &self.pieces,
        }
        .fmt(f)
    }
}

/// Writes a reformatted concatenation chain back out as mlang source: each
/// literal piece becomes its own string literal (reusing the same
/// delimiter-picking and escaping rules as a single reformatted query, see
/// `string_utils.rs`), each hole is re-formatted via its own `Format` impl,
/// and all pieces are joined with a plain, unbreakable `+` -- see the
/// comment in [FormatConcatenatedQuery::fmt] for why no line-wrapping is
/// attempted between pieces.
///
/// Every original token consumed by the chain (literal value tokens, `+`
/// operator tokens) is marked "removed" for the formatter's own
/// token-tracking bookkeeping first: their text is never reproduced
/// verbatim, only their skipped trivia, since this struct reconstructs the
/// visible output itself from scratch.
struct FormatConcatenatedQuery<'a> {
    chain: &'a ConcatenationChain,
    pieces: &'a [FormattedPiece],
}

impl Format<MFormatContext> for FormatConcatenatedQuery<'_> {
    fn fmt(&self, f: &mut MFormatter) -> FormatResult<()> {
        for part in &self.chain.parts {
            if let ConcatenationPart::Literal(token) = part {
                write!(f, [format_removed(token)])?;
            }
        }
        for operator in &self.chain.operators {
            write!(f, [format_removed(operator)])?;
        }

        // The delimiter for every new literal segment: the first original
        // literal's own opening quote (mlang's `` ` `` and `"` delimiters
        // are interchangeable, see `quote_as_static_str`'s doc comment in
        // `string_utils.rs`).
        let preferred_quote = self
            .chain
            .parts
            .iter()
            .find_map(|part| match part {
                ConcatenationPart::Literal(token) => token.text_trimmed().chars().next(),
                ConcatenationPart::Hole(_) => None,
            })
            .unwrap_or('`');

        // The start position of each literal piece's original token, in
        // order -- computed up front so the `format_with` closure below
        // (which may run more than once while the printer measures
        // different layouts, so it must be `Fn`, not `FnMut`) only ever
        // needs to index into this fixed list rather than advance a shared
        // mutable iterator.
        let literal_positions: Vec<TextSize> = self
            .chain
            .parts
            .iter()
            .filter_map(|part| match part {
                ConcatenationPart::Literal(token) => Some(token.text_trimmed_range().start()),
                ConcatenationPart::Hole(_) => None,
            })
            .collect();

        // Pieces are glued together with a plain, unbreakable ` + ` -- no
        // `soft_line_break_or_space()`/`group()` here on purpose. Any
        // literal piece with reformatted multi-line SQL already contains
        // its own `hard_line_break()`s (see `write_sql_piece_as_string_literal`),
        // and a hard break inside a `group` always forces the *whole*
        // group to expand -- so a soft break between, say, "from " and a
        // hole would *always* render as an actual line break the moment
        // any OTHER piece in the chain is multi-line, pushing every hole
        // onto its own line regardless of whether it needs to. What the
        // hole should do instead is sit exactly where the original author
        // put it -- immediately after "from ", not shoved onto a separate
        // line -- so pieces are simply written back to back.
        //
        // Only the very first and very last piece get the "start on a
        // fresh indented line" / "dedent before the closing quote" margin
        // treatment (matching the single-literal reformatted-SQL
        // convention, `FormatSqlStringToken::format_reformatted_multi_line_query`)
        // -- an *interior* piece boundary (adjacent to a hole on one or
        // both sides) never gets a margin break, so a hole's surrounding
        // text stays glued to it instead of being pushed onto its own
        // line. This is gated on whether *any* piece is multi-line, not
        // just the first/last one, so the whole chain still uses one
        // consistent indent level even when, say, only a middle piece
        // happens to be the one that wraps.
        let last_index = self.pieces.len() - 1;
        let overall_is_multiline = self
            .pieces
            .iter()
            .any(|piece| matches!(piece, FormattedPiece::Literal(text) if text.contains('\n')));

        write!(
            f,
            [&format_with(|f| {
                let mut literal_index = 0usize;
                for (index, piece) in self.pieces.iter().enumerate() {
                    if index > 0 {
                        write!(f, [text(" + ")])?;
                    }

                    match piece {
                        FormattedPiece::Literal(text_content) => {
                            let start = literal_positions[literal_index];
                            literal_index += 1;
                            write_sql_piece_as_string_literal(
                                text_content,
                                preferred_quote,
                                start,
                                overall_is_multiline && index == 0,
                                overall_is_multiline && index == last_index,
                                f,
                            )?;
                        }
                        FormattedPiece::Hole(part_index) => {
                            let ConcatenationPart::Hole(expression) =
                                &self.chain.parts[*part_index]
                            else {
                                return Err(FormatError::SyntaxError);
                            };
                            write!(f, [expression.format()])?;
                        }
                    }
                }

                Ok(())
            })]
        )
    }
}

/// Wraps `formatted_sql` back into a valid mlang string literal delimited
/// by `preferred_quote` (switching to the other allowed delimiter if the
/// text itself contains a conflicting quote character, same rule
/// `FormatSqlStringToken` uses for the single-literal case), splicing the
/// text in as new, source-position-tagged content at `start` (used only
/// for `dynamic_text`'s sourcemap bookkeeping -- there's no single original
/// token this piece corresponds to one-for-one once it's been reformatted,
/// so unlike `FormatSqlStringToken` this never goes through
/// `format_replaced`; token-tracking for the piece's source token is
/// already handled separately, see [FormatConcatenatedQuery]).
///
/// `formatted_sql` may itself be multi-line -- [format_concatenation_chain]
/// only bails out on the specific multi-line shape that reads as broken (a
/// hole landing inside a still-open nested string/identifier literal); an
/// ordinary multi-line piece with balanced quotes (e.g. `"select
/// a\nfrom "` followed by a hole substituting for a bare table/function
/// name) is fine.
///
/// `leading_margin`/`trailing_margin` control whether this piece opens with
/// a forced break-then-indent right after the quote, and closes with a
/// forced dedent-then-break right before the quote -- the same "bracket
/// hugs the paren, closing bracket dedents onto its own line" margin
/// `FormatSqlStringToken::format_reformatted_multi_line_query` always
/// applies for a single reformatted literal. Only the very first and very
/// last piece of a chain get these (set by the caller,
/// [FormatConcatenatedQuery]) -- an interior piece, immediately adjacent to
/// a hole on one or both sides, must stay glued to it instead of being
/// pushed onto its own line. All pieces still share the same indent level
/// for their own internal line breaks regardless of whether they carry a
/// margin, via the same ambient `indent()` nesting.
fn write_sql_piece_as_string_literal(
    formatted_sql: &str,
    preferred_quote: char,
    start: TextSize,
    leading_margin: bool,
    trailing_margin: bool,
    f: &mut MFormatter,
) -> FormatResult<()> {
    let effective_quote = if preferred_quote == '"' && formatted_sql.contains('"') {
        '`'
    } else {
        preferred_quote
    };
    let quote_text = quote_as_static_str(effective_quote);

    write!(
        f,
        [
            text(quote_text),
            indent(&format_with(|f| {
                if leading_margin {
                    write!(f, [hard_line_break()])?;
                }

                let mut lines = formatted_sql.lines();
                if let Some(first_line) = lines.next() {
                    write!(
                        f,
                        [dynamic_text(
                            &escape_for_string_literal(first_line, effective_quote),
                            start
                        )]
                    )?;
                    for line in lines {
                        write!(
                            f,
                            [
                                hard_line_break(),
                                dynamic_text(
                                    &escape_for_string_literal(line, effective_quote),
                                    start
                                )
                            ]
                        )?;
                    }
                }
                Ok(())
            })),
        ]
    )?;

    if trailing_margin {
        write!(f, [hard_line_break()])?;
    }

    write!(f, [text(quote_text)])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::comments::{MCommentStyle, MComments};
    use crate::{MFormatContext, MFormatOptions};
    use biome_formatter::{FormatState, VecBuffer};
    use biome_rowan::AstNode;
    use mlang_parser::parse;
    use mlang_syntax::{MBinaryExpression, MFileSource};

    fn parse_binary_expression(src: &str) -> AnyMExpression {
        let full = std::format!("#\nvar x = {src};");
        let tree = parse(&full, MFileSource::script());
        let binary = tree
            .syntax()
            .descendants()
            .find_map(MBinaryExpression::cast)
            .expect("expected a top-level MBinaryExpression");
        AnyMExpression::MBinaryExpression(binary)
    }

    /// Builds a throwaway [MFormatter] usable for a single call, following
    /// the low-level construction shown in `biome_formatter`'s own
    /// `write`/`format` free functions (`FormatState` -> `VecBuffer` ->
    /// `Formatter`) -- there's no ready-made "just give me a formatter"
    /// helper elsewhere in this crate since production code always reaches
    /// one via the normal node-formatting traversal.
    macro_rules! with_test_formatter {
        (|$f:ident| $body:expr) => {{
            let comments_tree = parse("#\n;", MFileSource::script());
            let options = MFormatOptions::new(MFileSource::script());
            let comments = MComments::from_node(&comments_tree.syntax(), &MCommentStyle, None);
            let context = MFormatContext::new(options, comments);
            let mut state = FormatState::new(context);
            let mut buffer = VecBuffer::new(&mut state);
            let $f = Formatter::new(&mut buffer);
            $body
        }};
    }

    #[test]
    fn flattens_literal_hole_literal_chain() {
        let expr = parse_binary_expression(r#""select * from t where a = '" + x + "'""#);
        let chain = flatten_concatenation_chain(&expr).expect("should flatten");

        assert_eq!(chain.parts.len(), 3);
        assert_eq!(chain.operators.len(), 2);
        assert!(matches!(chain.parts[0], ConcatenationPart::Literal(_)));
        assert!(matches!(chain.parts[1], ConcatenationPart::Hole(_)));
        assert!(matches!(chain.parts[2], ConcatenationPart::Literal(_)));
    }

    #[test]
    fn bails_out_on_adjacent_holes() {
        let expr = parse_binary_expression("x + y");

        assert!(flatten_concatenation_chain(&expr).is_none());
    }

    #[test]
    fn bails_out_on_adjacent_literals() {
        let expr = parse_binary_expression(r#""a" + "b" + x"#);

        assert!(flatten_concatenation_chain(&expr).is_none());
    }

    #[test]
    fn stops_at_non_plus_operator() {
        // `-` isn't concatenation -- the whole parenthesized right side
        // becomes a single opaque hole, its inner `-` never inspected.
        let expr = parse_binary_expression(r#""a" + (x - y)"#);
        let chain = flatten_concatenation_chain(&expr).expect("should flatten");

        assert_eq!(chain.parts.len(), 2);
        assert!(matches!(chain.parts[0], ConcatenationPart::Literal(_)));
        assert!(matches!(chain.parts[1], ConcatenationPart::Hole(_)));
    }

    #[test]
    fn builds_placeholder_source_substituting_holes() {
        let expr = parse_binary_expression(r#""select * from t where a = '" + x + "'""#);
        let chain = flatten_concatenation_chain(&expr).unwrap();

        let source = build_placeholder_source(&chain.parts).unwrap();

        assert_eq!(source, "select * from t where a = '__mlang_hole_1__'");
    }

    #[test]
    fn bails_out_when_literal_already_contains_a_placeholder() {
        let expr = parse_binary_expression(r#""__mlang_hole_1__" + x"#);
        let chain = flatten_concatenation_chain(&expr).unwrap();

        assert!(build_placeholder_source(&chain.parts).is_none());
    }

    #[test]
    fn formats_concatenation_chain_and_splits_holes_back() {
        let expr =
            parse_binary_expression(r#""select   *   from   t   where   a   =   '" + x + "'""#);
        let chain = flatten_concatenation_chain(&expr).unwrap();

        let pieces = with_test_formatter!(|f| format_concatenation_chain(&chain.parts, &f))
            .expect("should format");

        assert_eq!(pieces.len(), 3);
        assert!(matches!(
            &pieces[0],
            FormattedPiece::Literal(text) if text == "select * from t where a = '"
        ));
        assert!(matches!(pieces[1], FormattedPiece::Hole(1)));
        assert!(matches!(&pieces[2], FormattedPiece::Literal(text) if text == "'"));
    }

    #[test]
    fn bails_out_when_joined_text_does_not_parse_as_sql() {
        let expr = parse_binary_expression(r#""not valid   sql !!!" + x"#);
        let chain = flatten_concatenation_chain(&expr).unwrap();

        let result = with_test_formatter!(|f| format_concatenation_chain(&chain.parts, &f));

        assert!(result.is_none());
    }
}
