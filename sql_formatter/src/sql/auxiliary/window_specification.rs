use crate::prelude::*;
use biome_formatter::{format_args, write};
use sql_syntax::SqlWindowSpecification;
use sql_syntax::SqlWindowSpecificationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlWindowSpecification;
impl FormatNodeRule<SqlWindowSpecification> for FormatSqlWindowSpecification {
    fn fmt_fields(&self, node: &SqlWindowSpecification, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlWindowSpecificationFields {
            l_paren_token,
            partition_by_clause,
            order_by_clause,
            frame_clause,
            r_paren_token,
        } = node.as_fields();

        // A window that doesn't fit puts each clause on its own line.
        let clauses = format_with(|f: &mut SqlFormatter| {
            let mut separate = false;
            if let Some(partition_by_clause) = &partition_by_clause {
                write!(f, [partition_by_clause.format()])?;
                separate = true;
            }
            if let Some(order_by_clause) = &order_by_clause {
                if separate {
                    write!(f, [soft_line_break_or_space()])?;
                }
                write!(f, [order_by_clause.format()])?;
                separate = true;
            }
            if let Some(frame_clause) = &frame_clause {
                if separate {
                    write!(f, [soft_line_break_or_space()])?;
                }
                write!(f, [frame_clause.format()])?;
            }
            Ok(())
        });

        write!(
            f,
            [group(&format_args![
                l_paren_token.format(),
                soft_block_indent(&clauses),
                r_paren_token.format()
            ])]
        )
    }
}
