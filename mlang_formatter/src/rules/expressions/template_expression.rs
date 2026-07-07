use crate::prelude::*;

use biome_formatter::write;
use mlang_syntax::MTemplateExpression;
use mlang_syntax::MTemplateExpressionFields;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatMTemplateExpression;
impl_format_with_rule!(MTemplateExpression, FormatMTemplateExpression);

impl FormatNodeRule<MTemplateExpression> for FormatMTemplateExpression {
    fn fmt_fields(&self, node: &MTemplateExpression, f: &mut MFormatter) -> FormatResult<()> {
        let MTemplateExpressionFields { token, template } = node.as_fields();

        write![f, [token.format(), template.format()]]
    }
}
