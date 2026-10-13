use crate::prelude::*;
use crate::utils::{is_simple_expression, write_wrapping_fill_clause};
use biome_formatter::write;
use sql_syntax::AnySqlExpression;
use sql_syntax::PsqlIntoClause;
use sql_syntax::PsqlIntoClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIntoClause;
impl FormatNodeRule<PsqlIntoClause> for FormatPsqlIntoClause {
    fn fmt_fields(&self, node: &PsqlIntoClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlIntoClauseFields {
            into_token,
            strict_token,
            targets,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [into_token.format()])?;
            if let Some(strict_token) = &strict_token {
                write!(f, [space(), strict_token.format()])?;
            }
            Ok(())
        });

        write_wrapping_fill_clause(
            keyword,
            &targets,
            |target: &AnySqlExpression| !is_simple_expression(target, 0),
            f,
        )
    }
}
