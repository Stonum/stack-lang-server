//! Shared helper for the `verbatim_attributes` formatting mode: emit a tag's
//! attributes exactly as authored (spacing, line breaks, order) instead of
//! re-laying them out.

use crate::prelude::*;
use biome_formatter::{LINE_TERMINATORS, normalize_newlines, write};
use biome_rowan::AstNode;
use xml_syntax::{XmlAttributeList, XmlName};

/// Writes the whitespace after the tag name and every attribute verbatim (only
/// newlines are normalized), tracking the attribute tokens so the formatter's
/// "all tokens seen" check stays happy.
///
/// Returns `true` when that region spans more than one line, so the caller can
/// drop the closing `>` / `/>` onto its own line.
pub(crate) fn write_verbatim_attributes(
    name: Option<XmlName>,
    attributes: &XmlAttributeList,
    f: &mut XmlFormatter,
) -> FormatResult<bool> {
    let trimmed_start = attributes.syntax().text_trimmed_range().start();

    // The gap between the tag name and the first attribute lives half in the
    // name token's trailing trivia and half in the first attribute token's
    // leading trivia -- concatenating both reconstructs it exactly.
    let mut gap = String::new();
    if let Some(token) = name.and_then(|n| n.syntax().last_token()) {
        gap.push_str(token.trailing_trivia().text());
    }
    if let Some(token) = attributes.syntax().first_token() {
        gap.push_str(token.leading_trivia().text());
    }
    if gap.is_empty() {
        gap.push(' ');
    }

    let attrs_text = attributes.syntax().text_trimmed().to_string();
    let multiline = gap.contains('\n') || attrs_text.contains('\n');

    let gap = normalize_newlines(&gap, LINE_TERMINATORS);
    write!(
        f,
        [
            dynamic_text(&gap, trimmed_start),
            format_verbatim_node(attributes.syntax()),
        ]
    )?;

    Ok(multiline)
}
