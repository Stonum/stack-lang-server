use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlCreateFunctionStatement;
use psql_syntax::PsqlCreateFunctionStatementFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCreateFunctionStatement;
impl FormatNodeRule<PsqlCreateFunctionStatement> for FormatPsqlCreateFunctionStatement {
    fn fmt_fields(
        &self,
        node: &PsqlCreateFunctionStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlCreateFunctionStatementFields {
            create_token,
            or_token,
            replace_token,
            kind,
            name,
            l_paren_token,
            parameters,
            r_paren_token,
            returns_clause,
            leading_options,
            as_token,
            body,
            trailing_options,
            semicolon_token,
        } = node.as_fields();

        write!(f, [create_token.format()])?;

        if let Some(or_token) = or_token {
            write!(f, [space(), or_token.format()])?;
        }
        if let Some(replace_token) = replace_token {
            write!(f, [space(), replace_token.format()])?;
        }

        write!(
            f,
            [
                space(),
                kind.format(),
                space(),
                name.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&parameters.format())),
                r_paren_token.format(),
            ]
        )?;

        if let Some(returns_clause) = returns_clause {
            write!(f, [space(), returns_clause.format()])?;
        }
        if !leading_options.is_empty() {
            write!(f, [space(), group(&leading_options.format())])?;
        }

        write!(f, [space(), as_token.format(), space(), body.format()])?;

        if !trailing_options.is_empty() {
            write!(f, [space(), group(&trailing_options.format())])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
