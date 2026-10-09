use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTableLikeElement;
use sql_syntax::PsqlTableLikeElementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableLikeElement;
impl FormatNodeRule<PsqlTableLikeElement> for FormatPsqlTableLikeElement {
    fn fmt_fields(&self, node: &PsqlTableLikeElement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlTableLikeElementFields { like_token, source } = node.as_fields();

        write!(f, [like_token.format(), space(), source.format()])
    }
}
