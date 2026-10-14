use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCteCycleClause;
use sql_syntax::PsqlCteCycleClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCteCycleClause;
impl FormatNodeRule<PsqlCteCycleClause> for FormatPsqlCteCycleClause {
    fn fmt_fields(&self, node: &PsqlCteCycleClause, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCteCycleClauseFields {
            cycle_token,
            columns,
            set_token,
            mark,
            mark_values,
            using_token,
            path,
        } = node.as_fields();

        write!(
            f,
            [
                cycle_token.format(),
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
                mark.format()
            ]
        )?;
        if let Some(mark_values) = mark_values {
            write!(f, [space(), mark_values.format()])?;
        }
        write!(f, [space(), using_token.format(), space(), path.format()])
    }
}
