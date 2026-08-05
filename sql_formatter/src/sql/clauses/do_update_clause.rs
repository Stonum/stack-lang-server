use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDoUpdateClause;
use sql_syntax::SqlDoUpdateClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDoUpdateClause;
impl FormatNodeRule<SqlDoUpdateClause> for FormatSqlDoUpdateClause {
    fn fmt_fields(&self, node: &SqlDoUpdateClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDoUpdateClauseFields {
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
