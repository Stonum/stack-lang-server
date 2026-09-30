use biome_rowan::{SyntaxNode, TextRange};
use line_index::{LineCol, LineIndex};
use mlang_syntax::{MLanguage, MSyntaxKind, T};
use tower_lsp::lsp_types::{Range, SemanticToken, SemanticTokenModifier, SemanticTokenType};

pub const SEMANTIC_TOKEN_MAP: [SemanticTokenType; 1] = [SemanticTokenType::KEYWORD];

pub const SEMANTIC_TOKEN_MODIFIERS: [SemanticTokenModifier; 0] = [];

#[inline]
fn kind_to_type(token: MSyntaxKind) -> Option<(u32, u32)> {
    match token {
        // Don't override tm grammar
        T![function] | T![class] | T![get] | T![set] => None,
        T![var] => None,
        T![new] | T![this] | T![super] | T![extends] => None,
        T![true] | T![false] | T![null] => None,

        // keywords
        _ if token.is_keyword() => Some((0, 0)),
        _ => None,
    }
}

/// Text of the whole lines `range` spans, clamped to the document.
fn lines_range(syntax: &SyntaxNode<MLanguage>, line_index: &LineIndex, range: Range) -> TextRange {
    let document = syntax.text_range();
    let line_start = |line: u32| line_index.offset(LineCol { line, col: 0 });
    let start = line_start(range.start.line).unwrap_or(document.end());
    let end = range
        .end
        .line
        .checked_add(1)
        .and_then(line_start)
        .unwrap_or(document.end());
    TextRange::new(start, end.max(start))
}

pub fn semantic_tokens(
    syntax: SyntaxNode<MLanguage>,
    line_index: &LineIndex,
    tokens_range: Option<Range>,
) -> Vec<SemanticToken> {
    let range = match tokens_range {
        Some(range) => lines_range(&syntax, line_index, range),
        None => syntax.text_range(),
    };

    let mut result = vec![];
    let (mut prev_line, mut prev_col) = (0, 0);

    // only the tokens of the requested lines are visited
    let mut next = syntax.token_at_offset(range.start()).right_biased();
    while let Some(token) = next {
        let token_range = token.text_trimmed_range();
        if token_range.start() >= range.end() {
            break;
        }
        next = token.next_token();

        if !range.contains_range(token_range) {
            continue;
        }
        let Some((token_type, token_modifiers)) = kind_to_type(token.kind()) else {
            continue;
        };
        let Some(token_range) = line_index.line_col_range(token_range) else {
            continue;
        };

        let (line, col) = (token_range.start.line, token_range.start.col);
        let length = token_range.end.col - col; // we has not supported multiline tokens

        let delta_line = line.saturating_sub(prev_line);
        let delta_start = if line == prev_line {
            col.saturating_sub(prev_col)
        } else {
            col
        };

        (prev_line, prev_col) = (line, col);
        result.push(SemanticToken {
            delta_line,
            delta_start,
            length,
            token_type,
            token_modifiers_bitset: token_modifiers,
        });
    }

    result
}

#[cfg(test)]
mod tests {
    use mlang_syntax::MFileSource;
    use tower_lsp::lsp_types::Position;

    use super::*;

    const SRC: &str = r#"func f(a) {
   if (a) {
      return 1;
   }
   else {
      return 2;
   }
}
"#;

    /// Keywords of [SRC] as `(line, col, length)`.
    const KEYWORDS: [(u32, u32, u32); 4] = [(1, 3, 2), (2, 6, 6), (4, 3, 4), (5, 6, 6)];

    fn tokens(text: &str, range: Option<(u32, u32)>) -> Vec<SemanticToken> {
        let parsed = mlang_parser::parse(text, MFileSource::module());
        let range =
            range.map(|(start, end)| Range::new(Position::new(start, 0), Position::new(end, 0)));
        semantic_tokens(parsed.syntax(), &LineIndex::new(text), range)
    }

    /// Absolute `(line, col, length)` of delta-encoded tokens.
    fn decode(tokens: &[SemanticToken]) -> Vec<(u32, u32, u32)> {
        let (mut line, mut col) = (0, 0);
        tokens
            .iter()
            .map(|token| {
                if token.delta_line > 0 {
                    col = 0;
                }
                line += token.delta_line;
                col += token.delta_start;
                (line, col, token.length)
            })
            .collect()
    }

    #[test]
    fn full_document_tokens() {
        assert_eq!(decode(&tokens(SRC, None)), KEYWORDS);
    }

    #[test]
    fn range_skips_tokens_outside_of_it() {
        assert_eq!(decode(&tokens(SRC, Some((2, 4)))), [(2, 6, 6), (4, 3, 4)]);
        assert_eq!(decode(&tokens(SRC, Some((3, 3)))), []);
        assert_eq!(decode(&tokens(SRC, Some((5, 100)))), [(5, 6, 6)]);
        assert_eq!(decode(&tokens(SRC, Some((100, 200)))), []);
    }

    #[test]
    fn range_deltas_start_from_the_document_start() {
        let data = tokens(SRC, Some((4, 5)));
        assert_eq!(
            data.iter()
                .map(|t| (t.delta_line, t.delta_start, t.length))
                .collect::<Vec<_>>(),
            [(4, 3, 4), (1, 6, 6)]
        );
    }

    #[test]
    fn same_line_tokens_are_relative_to_each_other() {
        let text = "if (a) return 1; else return 2;\n";
        let data = tokens(text, Some((0, 0)));
        assert_eq!(
            data.iter()
                .map(|t| (t.delta_line, t.delta_start, t.length))
                .collect::<Vec<_>>(),
            [(0, 0, 2), (0, 7, 6), (0, 10, 4), (0, 5, 6)]
        );
    }

    #[test]
    fn every_range_agrees_with_the_full_document() {
        let full = decode(&tokens(SRC, None));
        for start in 0..10 {
            for end in start..10 {
                let expected: Vec<_> = full
                    .iter()
                    .copied()
                    .filter(|(line, ..)| (start..=end).contains(line))
                    .collect();
                assert_eq!(
                    decode(&tokens(SRC, Some((start, end)))),
                    expected,
                    "lines {start}..={end}"
                );
            }
        }
    }
}
