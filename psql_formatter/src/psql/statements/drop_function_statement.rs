use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlDropFunctionStatement;
use psql_syntax::PsqlDropFunctionStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlDropFunctionStatement;
impl FormatNodeRule<PsqlDropFunctionStatement> for FormatPsqlDropFunctionStatement {
    fn fmt_fields(
        &self,
        node: &PsqlDropFunctionStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlDropFunctionStatementFields {
            drop_token,
            kind,
            if_token,
            exists_token,
            name,
            parameters,
            drop_behavior,
            semicolon_token,
        } = node.as_fields();

        write!(f, [drop_token.format(), space(), kind.format()])?;

        if let Some(if_token) = if_token {
            write!(f, [space(), if_token.format()])?;
        }
        if let Some(exists_token) = exists_token {
            write!(f, [space(), exists_token.format()])?;
        }

        write!(f, [space(), name.format()])?;

        if let Some(parameters) = parameters {
            write!(f, [parameters.format()])?;
        }
        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
