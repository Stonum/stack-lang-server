use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlBlock;
use sql_syntax::PsqlPlBlockFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlBlock;
impl FormatNodeRule<PsqlPlBlock> for FormatPsqlPlBlock {
    fn fmt_fields(&self, node: &PsqlPlBlock, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlBlockFields {
            declare_section,
            begin_token,
            statements,
            end_token,
            semicolon_token,
        } = node.as_fields();

        if let Some(declare_section) = declare_section {
            write!(f, [declare_section.format(), hard_line_break()])?;
        }
        write!(
            f,
            [
                begin_token.format(),
                FormatPlBranchStatements::new(&statements),
                end_token.format()
            ]
        )?;
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
