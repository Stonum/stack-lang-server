use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlNamespacesClause;
use sql_syntax::PsqlXmlNamespacesClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlNamespacesClause;
impl FormatNodeRule<PsqlXmlNamespacesClause> for FormatPsqlXmlNamespacesClause {
    fn fmt_fields(&self, node: &PsqlXmlNamespacesClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlNamespacesClauseFields {
            name_token,
            l_paren_token,
            items,
            r_paren_token,
            comma_token,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                l_paren_token.format(),
                items.format(),
                r_paren_token.format(),
                comma_token.format()
            ]
        )
    }
}
