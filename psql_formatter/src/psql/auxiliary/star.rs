use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlStar;
use psql_syntax::PsqlStarFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlStar;
impl FormatNodeRule<PsqlStar> for FormatPsqlStar {
    fn fmt_fields(&self, node: &PsqlStar, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlStarFields { value_token } = node.as_fields();

        write!(f, [value_token.format()])
    }
}
