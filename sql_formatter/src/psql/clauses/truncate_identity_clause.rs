use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlTruncateIdentityClause;
use sql_syntax::PsqlTruncateIdentityClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTruncateIdentityClause;
impl FormatNodeRule<PsqlTruncateIdentityClause> for FormatPsqlTruncateIdentityClause {
    fn fmt_fields(
        &self,
        node: &PsqlTruncateIdentityClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlTruncateIdentityClauseFields {
            action,
            identity_token,
        } = node.as_fields();

        write!(f, [action.format(), space(), identity_token.format()])
    }
}
