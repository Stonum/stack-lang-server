use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::PsqlVacuumStatement;
use sql_syntax::PsqlVacuumStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlVacuumStatement;
impl FormatNodeRule<PsqlVacuumStatement> for FormatPsqlVacuumStatement {
    fn fmt_fields(&self, node: &PsqlVacuumStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlVacuumStatementFields {
            vacuum_token,
            full_token,
            freeze_token,
            verbose_token,
            analyze,
            tables,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [vacuum_token.format()])?;
            for token in [&full_token, &freeze_token, &verbose_token, &analyze]
                .into_iter()
                .flatten()
            {
                write!(f, [space(), token.format()])?;
            }
            Ok(())
        });

        if tables.is_empty() {
            write!(f, [keyword])?;
        } else {
            write_wrapping_fill_clause(keyword, &tables, |_| false, f)?;
        }

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
