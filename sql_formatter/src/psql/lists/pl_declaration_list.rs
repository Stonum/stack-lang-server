use crate::prelude::*;
use sql_syntax::PsqlPlDeclarationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlDeclarationList;
impl FormatRule<PsqlPlDeclarationList> for FormatPsqlPlDeclarationList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlDeclarationList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}
