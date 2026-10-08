use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::PsqlAnalyzeStatement;
use sql_syntax::PsqlAnalyzeStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlAnalyzeStatement;
impl FormatNodeRule<PsqlAnalyzeStatement> for FormatPsqlAnalyzeStatement {
    fn fmt_fields(&self, node: &PsqlAnalyzeStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlAnalyzeStatementFields {
            analyze,
            verbose_token,
            tables,
            semicolon_token,
        } = node.as_fields();

        let keyword = format_with(move |f| {
            write!(f, [analyze.format()])?;
            if let Some(verbose_token) = &verbose_token {
                write!(f, [space(), verbose_token.format()])?;
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
