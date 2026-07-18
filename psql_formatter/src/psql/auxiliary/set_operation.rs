use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSetOperation;
use psql_syntax::PsqlSetOperationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSetOperation;
impl FormatNodeRule<PsqlSetOperation> for FormatPsqlSetOperation {
    fn fmt_fields(&self, node: &PsqlSetOperation, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlSetOperationFields {
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

        write!(f, [hard_line_break(), select_clause.format()])?;
        if let Some(from_clause) = from_clause {
            write!(f, [hard_line_break(), from_clause.format()])?;
        }
        if let Some(where_clause) = where_clause {
            write!(f, [hard_line_break(), where_clause.format()])?;
        }
        if let Some(group_by_clause) = group_by_clause {
            write!(f, [hard_line_break(), group_by_clause.format()])?;
        }
        if let Some(having_clause) = having_clause {
            write!(f, [hard_line_break(), having_clause.format()])?;
        }
        Ok(())
    }
}
