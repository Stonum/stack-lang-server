use crate::prelude::*;
use crate::utils::object::can_unquote_member_name;
use crate::utils::{FormatLiteralStringToken, StringLiteralParentKind};

use biome_formatter::token::number::format_number_token;
use biome_formatter::write;
use mlang_syntax::MLiteralMemberNameFields;
use mlang_syntax::{MLiteralMemberName, MSyntaxKind};
use std::borrow::Cow;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLiteralMemberName;
impl_format_with_rule!(MLiteralMemberName, FormatMLiteralMemberName);

impl FormatNodeRule<MLiteralMemberName> for FormatMLiteralMemberName {
    fn fmt_fields(&self, node: &MLiteralMemberName, f: &mut MFormatter) -> FormatResult<()> {
        let MLiteralMemberNameFields { value } = node.as_fields();

        let value = value?;

        match value.kind() {
            MSyntaxKind::M_STRING_LITERAL => {
                if let Some(inner) = can_unquote_member_name(value.text_trimmed()) {
                    return write!(
                        f,
                        [format_replaced(
                            &value,
                            &syntax_token_cow_slice(
                                Cow::Owned(inner.to_owned()),
                                &value,
                                value.text_trimmed_range().start(),
                            )
                        )]
                    );
                }
                write![
                    f,
                    [FormatLiteralStringToken::new(
                        &value,
                        StringLiteralParentKind::Member
                    )]
                ]
            }
            MSyntaxKind::M_NUMBER_LITERAL => format_number_token(&value).fmt(f),
            _ => write![f, [value.format()]],
        }
    }
}
