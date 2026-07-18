use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDoUpdateClause;
use psql_syntax::PsqlDoUpdateClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDoUpdateClause;
impl FormatNodeRule<PsqlDoUpdateClause> for FormatPsqlDoUpdateClause {
    fn fmt_fields(&self, node: &PsqlDoUpdateClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlDoUpdateClauseFields {
            do_token,
            update_token,
            set_clause,
            where_clause,
        } = node.as_fields();

        write!(
            f,
            [
                do_token.format(),
                space(),
                update_token.format(),
                space(),
                set_clause.format()
            ]
        )?;
        if let Some(where_clause) = where_clause {
            write!(f, [hard_line_break(), where_clause.format()])?;
        }
        Ok(())
    }
}
