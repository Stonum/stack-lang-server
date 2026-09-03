//! Post-parse heuristic: when the braces don't balance, the parser reports the
//! problem far from where it was actually made — a missing `}` collapses to the
//! opening brace of the *outermost* block (or the end of the file), a missing
//! `{` trips on some stray `}` many lines below. This module re-anchors that one
//! diagnostic at the likely edit site using indentation.
//!
//! It is deliberately narrow to avoid the false-positive storms a global
//! re-pairing produces on real code (object literals, one-liners, K&R braces and
//! continuation indents all break a naive indentation match):
//!
//! * it only runs when a parse actually produced errors *and* the curly braces
//!   are unbalanced;
//! * it reports the *first* spot the nesting breaks and stops — at most one
//!   extra diagnostic, ever.

use biome_parser::diagnostic::ParseDiagnostic;
use biome_rowan::{Direction, TextRange, TextSize};
use mlang_syntax::MSyntaxKind::{
    L_CURLY, M_BLOCK_STATEMENT, M_CLASS_DECLARATION, M_CLASS_EXPRESSION, M_FUNCTION_BODY,
    M_SWITCH_STATEMENT, R_CURLY,
};
use mlang_syntax::{MSyntaxKind, MSyntaxNode, MSyntaxToken};

/// The parser's message for a `{` block that ran out of tokens before its `}`
/// (see `m_parse_error::expect_closing_delimiter`).
const MISSING_CURLY: &str = "Missing closing `}`";
/// Fragment of the parser's message for a `}` with no block to close. No
/// trailing quote: biome sometimes prints following tokens/trivia after the `}`.
const STRAY_CURLY: &str = "instead found '}";
/// Cascade fallout once a `{` is dropped and later code drifts out of its function.
const RETURN_OUTSIDE_FN: &str = "Illegal return statement outside of a function";

pub(crate) fn refine(source: &str, root: &MSyntaxNode, diagnostics: &mut Vec<ParseDiagnostic>) {
    if diagnostics.is_empty() {
        return;
    }

    let braces = collect_braces(root);
    let opens = braces.iter().filter(|(is_open, _)| *is_open).count();
    let closes = braces.len() - opens;

    let lines = Lines::new(source);

    if opens > closes {
        refine_missing_close(source, root, &lines, diagnostics);
    } else if closes > opens {
        refine_missing_open(source, &braces, &lines, diagnostics);
    }
}

/// A `}` went missing. The parser blames the outermost unclosed block and marks
/// only its `{` glyph. Re-anchor on the *whole header line* so it stands out,
/// and — walking that block's braces — move the blame to the first inner `{`
/// whose `}` landed to the left of it, if any.
fn refine_missing_close(
    source: &str,
    root: &MSyntaxNode,
    lines: &Lines,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    if !diagnostics
        .iter()
        .any(|d| d.message.to_string() == MISSING_CURLY)
    {
        return;
    }

    let Some(unclosed) = root
        .descendants()
        .find(|node| is_curly_block(node) && curly_token(node, R_CURLY).is_none())
    else {
        return;
    };
    let own_opener = curly_token(&unclosed, L_CURLY).map(|t| t.text_trimmed_range());
    let region = unclosed.text_range();

    // The culprit: an inner `{` whose `}` is dedented past it (so it stole an
    // outer `}`), else the flagged block itself.
    let mut culprit = own_opener;
    let mut orphan_close = None;
    let mut stack: Vec<(usize, TextRange)> = Vec::new();
    for token in unclosed
        .descendants_with_tokens(Direction::Next)
        .filter_map(|element| element.into_token())
    {
        let range = token.text_trimmed_range();
        if !region.contains_range(range) {
            continue;
        }
        match token.kind() {
            L_CURLY => stack.push((lines.indent_before(source, range.start()), range)),
            R_CURLY => {
                if let Some(&(opener_indent, opener_range)) = stack.last()
                    && lines.column(range.start()) < opener_indent
                    && Some(opener_range) != own_opener
                {
                    culprit = Some(opener_range);
                    orphan_close = Some(range);
                    break;
                }
                stack.pop();
            }
            _ => {}
        }
    }

    let Some(opener) = culprit else { return };
    let header = block_header_range(source, lines, opener);

    let mut diagnostic = ParseDiagnostic::new("This `{` is never closed", header)
        .with_hint("a `}` is missing inside this block");
    if let Some(orphan) = orphan_close {
        diagnostic = diagnostic.with_detail(orphan, "this `}` closes an outer block, not this one");
    }

    diagnostics.retain(|d| d.message.to_string() != MISSING_CURLY);
    diagnostics.push(diagnostic);
}

/// A `{` went missing: the parser tripped on a stray `}` far below. Find the
/// first `}` that is more indented than the block it closes, walk up past its
/// body to the `if` / `while` / ... header that should have opened a block, and
/// blame that.
fn refine_missing_open(
    source: &str,
    braces: &[(bool, TextRange)],
    lines: &Lines,
    diagnostics: &mut Vec<ParseDiagnostic>,
) {
    if !diagnostics
        .iter()
        .any(|d| d.message.to_string().contains(STRAY_CURLY))
    {
        return;
    }

    let mut stack: Vec<usize> = Vec::new();
    for &(is_open, range) in braces {
        if is_open {
            stack.push(lines.indent_before(source, range.start()));
            continue;
        }
        let column = lines.column(range.start());
        let more_indented_than_opener = stack.last().is_none_or(|&indent| column > indent);
        if more_indented_than_opener
            && let Some(header) = phantom_header(source, lines, range.start(), column)
        {
            diagnostics.retain(|d| {
                let m = d.message.to_string();
                !m.contains(STRAY_CURLY)
                    && m != RETURN_OUTSIDE_FN
                    && !m.starts_with("Expected a class method body")
            });
            diagnostics.push(
                ParseDiagnostic::new("This block is missing its opening `{`", header)
                    .with_detail(range, "this `}` has no `{` to match")
                    .with_hint("add `{` to open the block"),
            );
            return;
        }
        stack.pop();
    }
}

/// From a stray `}`, walk up over the more-indented lines that form the block's
/// body, then over blank lines, and return the header line if it looks like one
/// (`if (...)`, `Если(...)`, a bare `else`, ...). `None` if the shape doesn't fit
/// — then it is probably just an extra `}`, left to the parser.
fn phantom_header(
    source: &str,
    lines: &Lines,
    close_start: TextSize,
    close_column: usize,
) -> Option<TextRange> {
    let close_line = lines.line_of(close_start.into());

    // Walk up over the block body: lines more indented than the `}`, and blank
    // lines between them (a blank line doesn't end a block). Stop at the first
    // real line at the `}`'s indentation or shallower — that is the header.
    let mut body_top = close_line;
    let mut run_of_blanks = 0;
    while body_top > 0 {
        let above = body_top - 1;
        if lines.content(source, above).is_none() {
            run_of_blanks += 1;
            if run_of_blanks > 3 {
                return None; // a big gap — don't reach across it
            }
        } else if lines.indent(source, above) > close_column {
            run_of_blanks = 0;
        } else {
            break;
        }
        body_top -= 1;
    }
    if body_top == close_line {
        return None; // no body above the `}`
    }

    let header = body_top.checked_sub(1)?;
    if lines.indent(source, header) > close_column {
        return None;
    }

    let content = lines.content(source, header)?;
    let text = source[usize::from(content.start())..usize::from(content.end())].trim();
    let looks_like_header = text.ends_with(')')
        || matches!(
            text.to_lowercase().as_str(),
            "else" | "do" | "иначе" | "тогда"
        );
    looks_like_header.then_some(content)
}

/// The range to squiggle for a block: its header. If the `{` shares its line
/// with code (`while (a) {`), that line's content; if the `{` sits alone on its
/// line, the nearest non-blank line above it (`while (a)` \n `{`).
fn block_header_range(source: &str, lines: &Lines, opener: TextRange) -> TextRange {
    let opener_line = lines.line_of(opener.start().into());

    if let Some(content) = lines.content(source, opener_line)
        && content.start() < opener.start()
    {
        return content;
    }

    let mut line = opener_line;
    while line > 0 {
        line -= 1;
        if let Some(content) = lines.content(source, line) {
            return content;
        }
    }

    lines.content(source, opener_line).unwrap_or(opener)
}

fn collect_braces(root: &MSyntaxNode) -> Vec<(bool, TextRange)> {
    root.descendants_with_tokens(Direction::Next)
        .filter_map(|element| {
            let token = element.into_token()?;
            match token.kind() {
                L_CURLY => Some((true, token.text_trimmed_range())),
                R_CURLY => Some((false, token.text_trimmed_range())),
                _ => None,
            }
        })
        .collect()
}

fn is_curly_block(node: &MSyntaxNode) -> bool {
    matches!(
        node.kind(),
        M_BLOCK_STATEMENT
            | M_FUNCTION_BODY
            | M_CLASS_DECLARATION
            | M_CLASS_EXPRESSION
            | M_SWITCH_STATEMENT
    )
}

fn curly_token(node: &MSyntaxNode, kind: MSyntaxKind) -> Option<MSyntaxToken> {
    node.children_with_tokens()
        .filter_map(|element| element.into_token())
        .find(|token| token.kind() == kind)
}

struct Lines {
    starts: Vec<usize>,
}

impl Lines {
    fn new(source: &str) -> Self {
        let mut starts = vec![0];
        starts.extend(source.match_indices('\n').map(|(i, _)| i + 1));
        Self { starts }
    }

    fn line_of(&self, offset: usize) -> usize {
        self.starts
            .partition_point(|&start| start <= offset)
            .saturating_sub(1)
    }

    fn column(&self, offset: TextSize) -> usize {
        let offset = usize::from(offset);
        offset - self.starts[self.line_of(offset)]
    }

    fn indent(&self, source: &str, line: usize) -> usize {
        source[self.starts[line]..]
            .bytes()
            .take_while(|b| *b == b' ' || *b == b'\t')
            .count()
    }

    fn indent_before(&self, source: &str, offset: TextSize) -> usize {
        self.indent(source, self.line_of(usize::from(offset)))
    }

    /// Range from the first non-whitespace byte of the line to its trimmed end.
    /// `None` for a blank line.
    fn content(&self, source: &str, line: usize) -> Option<TextRange> {
        let start = self.starts[line];
        let end = self
            .starts
            .get(line + 1)
            .map(|&next| next - 1)
            .unwrap_or(source.len());
        let content_start = start + self.indent(source, line);
        let content_end = source[..end].trim_end().len();
        (content_start < content_end).then(|| {
            TextRange::new(
                TextSize::try_from(content_start).unwrap(),
                TextSize::try_from(content_end).unwrap(),
            )
        })
    }
}
