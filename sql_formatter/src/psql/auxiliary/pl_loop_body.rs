use crate::prelude::*;
use crate::psql::lists::pl_statement_list::FormatPlBranchStatements;
use biome_formatter::write;
use sql_syntax::PsqlPlLoopBody;
use sql_syntax::PsqlPlLoopBodyFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlLoopBody;
impl FormatNodeRule<PsqlPlLoopBody> for FormatPsqlPlLoopBody {
    fn fmt_fields(&self, node: &PsqlPlLoopBody, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlLoopBodyFields {
            loop_token,
            statements,
            end_token,
            end_loop_token,
            end_label,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                loop_token.format(),
                FormatPlBranchStatements::new(&statements),
                end_token.format(),
                space(),
                end_loop_token.format()
            ]
        )?;
        if let Some(end_label) = end_label {
            write!(f, [space(), end_label.format()])?;
        }
        write!(f, [semicolon_token.format()])
    }
}
