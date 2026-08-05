use crate::prelude::*;
use crate::utils::write_select_body_clauses;
use biome_formatter::write;
use sql_syntax::SqlSetOperation;
use sql_syntax::SqlSetOperationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSetOperation;
impl FormatNodeRule<SqlSetOperation> for FormatSqlSetOperation {
    fn fmt_fields(&self, node: &SqlSetOperation, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlSetOperationFields {
            operator_token,
            quantifier,
            select_clause,
            from_clause,
            where_clause,
            group_by_clause,
            having_clause,
        } = node.as_fields();

        // The operator (`union`/`intersect`/`except`) sits on its own line,
        // indented one level deeper than the branches on either side of it
        // -- which are themselves siblings at the same (unindented) level,
        // same as any other SELECT's clauses.
        write!(
            f,
            [indent(&format_once(|f| {
                write!(f, [hard_line_break(), operator_token.format()])?;
                if let Some(quantifier) = &quantifier {
                    write!(f, [space(), quantifier.format()])?;
                }
                Ok(())
            }))]
        )?;

        write!(f, [hard_line_break()])?;
        write!(
            f,
            [group(&format_once(|f| {
                write_select_body_clauses(
                    select_clause,
                    from_clause,
                    where_clause,
                    group_by_clause,
                    having_clause,
                    f,
                )
            }))]
        )
    }
}
