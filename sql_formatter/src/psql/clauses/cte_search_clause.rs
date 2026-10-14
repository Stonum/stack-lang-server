use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCteSearchClause;
use sql_syntax::PsqlCteSearchClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteSearchClause;
impl FormatNodeRule<PsqlCteSearchClause> for FormatPsqlCteSearchClause {
    fn fmt_fields(&self, node: &PsqlCteSearchClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCteSearchClauseFields {
            search_token,
            order,
            first_token,
            by_token,
            columns,
            set_token,
            column,
        } = node.as_fields();

        write!(
            f,
            [
                search_token.format(),
                space(),
                order.format(),
                space(),
                first_token.format(),
                space(),
                by_token.format(),
                space(),
                // A short column list: kept on one line.
                format_with(|f| f
                    .join_with(space())
                    .entries(
                        columns
                            .format_separated(",")
                            .with_trailing_separator(TrailingSeparator::Omit)
                    )
                    .finish()),
                space(),
                set_token.format(),
                space(),
                column.format()
            ]
        )
    }
}
