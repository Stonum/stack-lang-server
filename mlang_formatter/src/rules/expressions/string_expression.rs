use crate::prelude::*;
use crate::utils::{FormatLiteralStringToken, FormatSqlStringToken, StringLiteralParentKind};
use biome_rowan::declare_node_union;

use mlang_syntax::parentheses::NeedsParentheses;
use mlang_syntax::{
    MLongStringLiteralExpression, MLongStringLiteralExpressionFields, MStringLiteralExpression,
    MStringLiteralExpressionFields, MSyntaxToken,
};

/// Whether `token` was explicitly, wholly selected by the requesting
/// `textDocument/rangeFormatting` call -- i.e. the selection lies entirely
/// inside this single string token, not merely overlapping it. Containment
/// (rather than mere overlap) is what distinguishes a genuine partial
/// selection of this string from whole-document formatting, which always
/// sends a range spanning the entire file and essentially never fits inside
/// one token.
fn is_explicitly_selected(token: &MSyntaxToken, f: &MFormatter) -> bool {
    f.options()
        .selected_range()
        .is_some_and(|selected| token.text_trimmed_range().contains_range(selected))
}

declare_node_union! {
    pub(crate) FormatString =
        MStringLiteralExpression |
        MLongStringLiteralExpression
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMStringLiteralExpression;
impl_format_with_rule!(MStringLiteralExpression, FormatMStringLiteralExpression);

impl FormatNodeRule<MStringLiteralExpression> for FormatMStringLiteralExpression {
    fn fmt_fields(&self, node: &MStringLiteralExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        if is_explicitly_selected(&value_token, f) {
            FormatSqlStringToken::new(&value_token).fmt(f)
        } else {
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression).fmt(f)
        }
    }

    fn needs_parentheses(&self, item: &MStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMLongStringLiteralExpression;
impl_format_with_rule!(
    MLongStringLiteralExpression,
    FormatMLongStringLiteralExpression
);

impl FormatNodeRule<MLongStringLiteralExpression> for FormatMLongStringLiteralExpression {
    fn fmt_fields(
        &self,
        node: &MLongStringLiteralExpression,
        f: &mut MFormatter,
    ) -> FormatResult<()> {
        let MLongStringLiteralExpressionFields { value_token } = node.as_fields();

        let value_token = value_token?;
        if is_explicitly_selected(&value_token, f) {
            FormatSqlStringToken::new(&value_token).fmt(f)
        } else {
            FormatLiteralStringToken::new(&value_token, StringLiteralParentKind::Expression).fmt(f)
        }
    }

    fn needs_parentheses(&self, item: &MLongStringLiteralExpression) -> bool {
        item.needs_parentheses()
    }
}
