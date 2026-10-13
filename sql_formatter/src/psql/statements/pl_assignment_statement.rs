use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlAssignmentStatement;
use sql_syntax::PsqlPlAssignmentStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlAssignmentStatement;
impl FormatNodeRule<PsqlPlAssignmentStatement> for FormatPsqlPlAssignmentStatement {
    fn fmt_fields(
        &self,
        node: &PsqlPlAssignmentStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPlAssignmentStatementFields {
            target,
            operator,
            value,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                target.format(),
                space(),
                operator.format(),
                space(),
                value.format(),
                semicolon_token.format()
            ]
        )
    }
}
