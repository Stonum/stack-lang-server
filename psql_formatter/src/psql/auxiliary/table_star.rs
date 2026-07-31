use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTableStar;
use psql_syntax::PsqlTableStarFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTableStar;
impl FormatNodeRule<PsqlTableStar> for FormatPsqlTableStar {
    fn fmt_fields(&self, node: &PsqlTableStar, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTableStarFields {
            table,
            dot_token,
            star,
        } = node.as_fields();

        write!(f, [table.format(), dot_token.format(), star.format()])
    }
}
