use crate::prelude::*;
use psql_syntax::PsqlSelectAllQuantifier;
use psql_syntax::PsqlSelectAllQuantifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSelectAllQuantifier;
impl FormatNodeRule<PsqlSelectAllQuantifier> for FormatPsqlSelectAllQuantifier {
    fn fmt_fields(
        &self,
        node: &PsqlSelectAllQuantifier,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlSelectAllQuantifierFields { all_token } = node.as_fields();

        all_token.format().fmt(f)
    }
}
