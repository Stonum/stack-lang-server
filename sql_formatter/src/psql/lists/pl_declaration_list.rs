use crate::prelude::*;
use biome_rowan::AstNode;
use sql_syntax::PsqlPlDeclarationList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlDeclarationList;
impl FormatRule<PsqlPlDeclarationList> for FormatPsqlPlDeclarationList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlDeclarationList, f: &mut SqlFormatter) -> FormatResult<()> {
        // Keeps (at most) one blank line where the source has one.
        let mut join = f.join_nodes_with_hardline();
        for item in node.iter() {
            join.entry(item.syntax(), &item.format());
        }
        join.finish()
    }
}
