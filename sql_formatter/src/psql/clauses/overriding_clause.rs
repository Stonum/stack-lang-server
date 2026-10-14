use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlOverridingClause;
use sql_syntax::PsqlOverridingClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlOverridingClause;
impl FormatNodeRule<PsqlOverridingClause> for FormatPsqlOverridingClause {
    fn fmt_fields(&self, node: &PsqlOverridingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlOverridingClauseFields {
            overriding_token,
            kind,
            value_token,
        } = node.as_fields();

        write!(
            f,
            [
                overriding_token.format(),
                space(),
                kind.format(),
                space(),
                value_token.format()
            ]
        )
    }
}
