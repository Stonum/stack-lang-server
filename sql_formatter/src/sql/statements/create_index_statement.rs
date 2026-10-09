use crate::prelude::*;
use crate::utils::{is_simple_expression, write_bracketed_fill_list};
use biome_formatter::{format_args, write};
use sql_syntax::SqlCreateIndexStatement;
use sql_syntax::SqlCreateIndexStatementFields;
use sql_syntax::SqlOrderByExpression;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlCreateIndexStatement;
impl FormatNodeRule<SqlCreateIndexStatement> for FormatSqlCreateIndexStatement {
    fn fmt_fields(&self, node: &SqlCreateIndexStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlCreateIndexStatementFields {
            create_token,
            unique_token,
            index_token,
            concurrently_token,
            if_token,
            not_token,
            exists_token,
            name,
            on_token,
            only_token,
            table,
            method,
            l_paren_token,
            elements,
            r_paren_token,
            include,
            where_clause,
            semicolon_token,
        } = node.as_fields();

        let head = format_with(|f| {
            write!(f, [create_token.format()])?;
            if let Some(unique_token) = &unique_token {
                write!(f, [space(), unique_token.format()])?;
            }
            write!(f, [space(), index_token.format()])?;
            for token in [&concurrently_token, &if_token, &not_token, &exists_token]
                .into_iter()
                .flatten()
            {
                write!(f, [space(), token.format()])?;
            }
            if let Some(name) = &name {
                write!(f, [space(), name.format()])?;
            }
            write!(f, [space(), on_token.format()])?;
            if let Some(only_token) = &only_token {
                write!(f, [space(), only_token.format()])?;
            }
            write!(f, [space(), table.format()])?;
            if let Some(method) = &method {
                write!(f, [space(), method.format()])?;
            }
            write!(f, [space()])?;
            write_bracketed_fill_list(
                l_paren_token.clone(),
                &elements,
                r_paren_token.clone(),
                |element: &SqlOrderByExpression| {
                    !element
                        .item()
                        .is_ok_and(|item| is_simple_expression(&item, 0))
                },
                f,
            )
        });

        // `include`/`where` move to their own lines once the index doesn't fit.
        let tail = format_with(|f| {
            if let Some(include) = &include {
                write!(f, [soft_line_break_or_space(), include.format()])?;
            }
            if let Some(where_clause) = &where_clause {
                write!(f, [soft_line_break_or_space(), where_clause.format()])?;
            }
            Ok(())
        });

        write!(f, [group(&format_args![head, tail])])?;

        if let Some(semicolon_token) = semicolon_token {
            write!(f, [semicolon_token.format()])?;
        }
        Ok(())
    }
}
