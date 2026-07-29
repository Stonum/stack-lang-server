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
            kind,
            name,
            l_paren_token,
            parameters,
            r_paren_token,
            returns_clause,
            as_token,
            body,
            language_option,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [
                create_token.format(),
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

        write!(f, [space(), as_token.format(), space(), body.format()])?;

        if let Some(language_option) = language_option {
            write!(f, [space(), language_option.format()])?;
        }
        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
