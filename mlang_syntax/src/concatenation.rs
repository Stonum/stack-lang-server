use crate::{AnyMExpression, AnyMLiteralExpression, MSyntaxKind, MSyntaxToken};
use biome_rowan::SyntaxResult;
use std::ops::Range;

/// One piece of a flattened top-level `+`-concatenation chain
pub enum ConcatenationPart {
    Literal(MSyntaxToken),
    Hole(AnyMExpression),
}

/// A flattened `+`-concatenation chain: the alternating literal/hole
/// [ConcatenationPart]s, plus the `+` operator token that originally sat
/// between each adjacent pair (`operators[i]` is between `parts[i]` and
/// `parts[i + 1]`, so `operators.len() == parts.len() - 1`) -- needed so
/// every original token in the chain can be accounted for by callers that
/// reconstruct the visible output from scratch instead of formatting each
/// node normally.
pub struct ConcatenationChain {
    pub parts: Vec<ConcatenationPart>,
    pub operators: Vec<MSyntaxToken>,
}

/// Flattens a left-associative `+`-chain (`"a" + x + "b"` parses as
/// `MBinaryExpression(MBinaryExpression("a", x), "b")`
/// Only top-level `+` is unwrapped -- a non-`+` operator, or any expression
/// kind other than a string literal, becomes an opaque hole and its own
/// internal structure is never inspected.
///
/// Returns `None` unless the parts strictly alternate `Literal`, `Hole`,
/// `Literal`, `Hole`, ...:
/// - Two adjacent holes would merge their placeholders
///   ([build_placeholder_source]) into a single token for the SQL lexer,
///   which is unrecoverable.
/// - Two adjacent literals are deliberately unsupported too, even though
///   they could in principle be joined -- keeping the alternation strict
///   guarantees a 1:1 correspondence between input literal parts and
///   output pieces, which is what lets a caller account for every original
///   token without having to work out which of several original literals a
///   merged output run came from.
pub fn flatten_concatenation_chain(expression: &AnyMExpression) -> Option<ConcatenationChain> {
    let mut parts = Vec::new();
    let mut operators = Vec::new();
    collect_concatenation_parts(expression, &mut parts, &mut operators).ok()?;

    if !parts_strictly_alternate(&parts) {
        return None;
    }

    Some(ConcatenationChain { parts, operators })
}

fn collect_concatenation_parts(
    expression: &AnyMExpression,
    parts: &mut Vec<ConcatenationPart>,
    operators: &mut Vec<MSyntaxToken>,
) -> SyntaxResult<()> {
    if let AnyMExpression::MBinaryExpression(binary) = expression {
        let operator_token = binary.operator_token()?;
        if operator_token.kind() == MSyntaxKind::PLUS {
            collect_concatenation_parts(&binary.left()?, parts, operators)?;
            operators.push(operator_token);
            collect_concatenation_parts(&binary.right()?, parts, operators)?;
            return Ok(());
        }
    }

    parts.push(match expression {
        AnyMExpression::AnyMLiteralExpression(AnyMLiteralExpression::MStringLiteralExpression(
            string,
        )) => ConcatenationPart::Literal(string.value_token()?),
        AnyMExpression::AnyMLiteralExpression(
            AnyMLiteralExpression::MLongStringLiteralExpression(string),
        ) => ConcatenationPart::Literal(string.value_token()?),
        _ => ConcatenationPart::Hole(expression.clone()),
    });

    Ok(())
}

fn parts_strictly_alternate(parts: &[ConcatenationPart]) -> bool {
    parts.windows(2).all(|pair| {
        !matches!(
            pair,
            [ConcatenationPart::Hole(_), ConcatenationPart::Hole(_)]
                | [ConcatenationPart::Literal(_), ConcatenationPart::Literal(_)]
        )
    })
}

/// The positional placeholder substituted for the hole at `index` --
/// syntactically a plain identifier, valid almost anywhere in the SQL
/// grammar (column/table/function name); if the hole actually sits inside
/// an already-open SQL string literal (the typical case, e.g.
/// `"...where name = '" + userName + "'"`), it simply becomes part of that
/// literal's text content instead, which the lexer doesn't care about
/// either way.
pub fn hole_placeholder(index: usize) -> String {
    format!("__mlang_hole_{index}__")
}

/// A string literal token's raw (still-escaped, exactly as written in
/// source) inner content, with the surrounding quotes stripped.
fn literal_raw_content(token: &MSyntaxToken) -> Option<&str> {
    let content = token.text_trimmed();
    content.get(1..content.len().checked_sub(1)?)
}

/// Joins a flattened concatenation chain into one SQL source string,
/// substituting each hole with its own placeholder ([hole_placeholder]).
///
/// Returns `None` if any literal segment already contains a placeholder
/// that's about to be substituted -- an extremely unlikely coincidence, but
/// one that would otherwise silently corrupt the reconstructed query (a
/// placeholder accidentally embedded in a literal segment would later be
/// mistaken for a hole when splitting the formatted output back apart).
pub fn build_placeholder_source(parts: &[ConcatenationPart]) -> Option<String> {
    let placeholders: Vec<String> = (0..parts.len()).map(hole_placeholder).collect();

    for part in parts {
        if let ConcatenationPart::Literal(token) = part {
            let raw = literal_raw_content(token)?;
            if placeholders
                .iter()
                .any(|placeholder| raw.contains(placeholder.as_str()))
            {
                return None;
            }
        }
    }

    let mut joined = String::new();
    for (index, part) in parts.iter().enumerate() {
        match part {
            ConcatenationPart::Literal(token) => joined.push_str(literal_raw_content(token)?),
            ConcatenationPart::Hole(_) => joined.push_str(&placeholders[index]),
        }
    }

    Some(joined)
}

/// Byte ranges of every non-nested `{...}` in `raw` (`.format()`-style
/// template placeholders, e.g. `{0}`, `{}`, `{name}` -- content is opaque,
/// not required to be numeric).
fn find_format_placeholders(raw: &str) -> Vec<Range<usize>> {
    let mut ranges = Vec::new();
    let mut cursor = 0;

    while let Some(open) = raw[cursor..].find('{') {
        let open = cursor + open;
        match raw[open + 1..].find(['{', '}']) {
            Some(rel) if raw.as_bytes()[open + 1 + rel] == b'}' => {
                let close = open + 1 + rel;
                ranges.push(open..close + 1);
                cursor = close + 1;
            }
            _ => cursor = open + 1,
        }
    }

    ranges
}

/// Replaces each `{...}` in `raw` with [hole_placeholder], returning the
/// substituted text and the original `{...}` text of each occurrence, in
/// order. `None` if there's nothing to substitute, or if `raw` already
/// contains a placeholder we're about to introduce.
pub fn substitute_format_placeholders(raw: &str) -> Option<(String, Vec<String>)> {
    let placeholder_ranges = find_format_placeholders(raw);
    if placeholder_ranges.is_empty() {
        return None;
    }

    let placeholders: Vec<String> = (0..placeholder_ranges.len())
        .map(hole_placeholder)
        .collect();
    if placeholders.iter().any(|p| raw.contains(p.as_str())) {
        return None;
    }

    let mut substituted = String::with_capacity(raw.len());
    let mut originals = Vec::with_capacity(placeholder_ranges.len());
    let mut cursor = 0;
    for (index, range) in placeholder_ranges.iter().enumerate() {
        substituted.push_str(&raw[cursor..range.start]);
        substituted.push_str(&placeholders[index]);
        originals.push(raw[range.clone()].to_string());
        cursor = range.end;
    }
    substituted.push_str(&raw[cursor..]);

    Some((substituted, originals))
}

#[cfg(test)]
mod format_placeholder_tests {
    use super::*;

    #[test]
    fn finds_single_placeholder() {
        let ranges = find_format_placeholders("select {0} from t");
        assert_eq!(ranges, vec![7..10]);
    }

    #[test]
    fn finds_empty_placeholder() {
        let ranges = find_format_placeholders("select {} from t");
        assert_eq!(ranges, vec![7..9]);
    }

    #[test]
    fn finds_non_numeric_placeholder() {
        let ranges = find_format_placeholders("select {name} from t");
        assert_eq!(ranges, vec![7..13]);
    }

    #[test]
    fn finds_repeated_placeholder_occurrences() {
        let ranges = find_format_placeholders("{0} and {0}");
        assert_eq!(ranges, vec![0..3, 8..11]);
    }

    #[test]
    fn skips_unmatched_opening_brace() {
        let ranges = find_format_placeholders("select { from t where a = {0}");
        assert_eq!(ranges, vec![26..29]);
    }

    #[test]
    fn substitutes_and_restores_round_trip() {
        let raw = "select {0} from t where a = {1}";
        let (substituted, originals) = substitute_format_placeholders(raw).unwrap();

        assert_eq!(originals, vec!["{0}", "{1}"]);
        assert!(!substituted.contains('{'));
        assert_eq!(
            substituted,
            "select __mlang_hole_0__ from t where a = __mlang_hole_1__"
        );
    }

    #[test]
    fn no_placeholders_returns_none() {
        assert!(substitute_format_placeholders("select * from t").is_none());
    }

    #[test]
    fn bails_out_on_placeholder_collision() {
        let raw = format!("select {{0}} from t where a = '{}'", hole_placeholder(0));
        assert!(substitute_format_placeholders(&raw).is_none());
    }
}
