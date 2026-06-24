use super::FormatLiteralStringToken;
use super::StringLiteralParentKind;

use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::AstNode;
use biome_unicode_table::{is_js_id_continue, is_js_id_start};
use mlang_syntax::AnyMObjectMemberName;
use mlang_syntax::MSyntaxKind;
use mlang_syntax::MSyntaxKind::M_STRING_LITERAL;
use std::borrow::Cow;

impl Format<MFormatContext> for AnyMObjectMemberName {
    fn fmt(&self, f: &mut Formatter<MFormatContext>) -> FormatResult<()> {
        self.format().fmt(f)
    }
}

/// If `quoted` is a double-quoted string whose inner content is a valid, non-keyword JS
/// identifier, returns the inner content (without quotes). Otherwise returns `None`.
pub(crate) fn can_unquote_member_name(quoted: &str) -> Option<&str> {
    let inner = quoted.strip_prefix('"').and_then(|s| s.strip_suffix('"'))?;
    let mut chars = inner.chars();
    let first = chars.next()?;
    if !is_js_id_start(first) || !chars.all(is_js_id_continue) {
        return None;
    }
    if MSyntaxKind::from_keyword(inner).is_some_and(|k| k.is_keyword()) {
        return None;
    }
    Some(inner)
}

pub(crate) fn write_member_name(
    name: &AnyMObjectMemberName,
    f: &mut MFormatter,
) -> FormatResult<usize> {
    match name {
        AnyMObjectMemberName::MLiteralMemberName(literal) => {
            let value = literal.value()?;

            if value.kind() == M_STRING_LITERAL {
                if let Some(inner) = can_unquote_member_name(value.text_trimmed()) {
                    write!(
                        f,
                        [
                            format_leading_comments(name.syntax()),
                            format_replaced(
                                &value,
                                &syntax_token_cow_slice(
                                    Cow::Owned(inner.to_owned()),
                                    &value,
                                    value.text_trimmed_range().start(),
                                )
                            ),
                            format_trailing_comments(name.syntax())
                        ]
                    )?;
                    return Ok(inner.len());
                }

                let format = FormatLiteralStringToken::new(&value, StringLiteralParentKind::Member);
                let cleaned = format.clean_text();

                write!(
                    f,
                    [
                        format_leading_comments(name.syntax()),
                        cleaned,
                        format_trailing_comments(name.syntax())
                    ]
                )?;

                Ok(cleaned.width())
            } else {
                write!(f, [name])?;

                Ok(value.text_trimmed().len())
            }
        }
        name => {
            write!(f, [&name])?;
            Ok(name.text().len())
        }
    }
}
