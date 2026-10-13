use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlExceptionConditionList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlExceptionConditionList;
impl FormatRule<PsqlPlExceptionConditionList> for FormatPsqlPlExceptionConditionList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlExceptionConditionList, f: &mut SqlFormatter) -> FormatResult<()> {
        // `a or b or c` -- the separators are `or` keywords, spaced on both sides.
        for element in node.elements() {
            write!(f, [element.node().format()])?;
            if let Some(separator) = element.trailing_separator()? {
                write!(f, [space(), separator.format(), space()])?;
            }
        }
        Ok(())
    }
}
