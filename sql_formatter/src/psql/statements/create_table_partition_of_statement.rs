use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCreateTablePartitionOfStatement;
use sql_syntax::PsqlCreateTablePartitionOfStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateTablePartitionOfStatement;
impl FormatNodeRule<PsqlCreateTablePartitionOfStatement>
    for FormatPsqlCreateTablePartitionOfStatement
{
    fn fmt_fields(
        &self,
        node: &PsqlCreateTablePartitionOfStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateTablePartitionOfStatementFields {
            create_token,
            persistence,
            table_token,
            if_token,
            not_token,
            exists_token,
            name,
            partition_token,
            of_token,
            parent,
            bound,
            partition_by,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format()])?;
        if let Some(persistence) = persistence {
            write!(f, [space(), persistence.format()])?;
        }
        write!(f, [space(), table_token.format()])?;
        for token in [&if_token, &not_token, &exists_token].into_iter().flatten() {
            write!(f, [space(), token.format()])?;
        }
        write!(
            f,
            [
                space(),
                name.format(),
                space(),
                partition_token.format(),
                space(),
                of_token.format(),
                space(),
                parent.format()
            ]
        )?;

        // A long header moves the bound and `PARTITION BY` to their own lines.
        write!(
            f,
            [group(&format_with(|f: &mut SqlFormatter| {
                write!(f, [soft_line_break_or_space(), bound.format()])?;
                if let Some(partition_by) = &partition_by {
                    write!(f, [soft_line_break_or_space(), partition_by.format()])?;
                }
                Ok(())
            }))]
        )?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
