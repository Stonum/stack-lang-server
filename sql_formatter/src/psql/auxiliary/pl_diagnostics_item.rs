use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlDiagnosticsItem;
use sql_syntax::PsqlPlDiagnosticsItemFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlDiagnosticsItem;
impl FormatNodeRule<PsqlPlDiagnosticsItem> for FormatPsqlPlDiagnosticsItem {
    fn fmt_fields(&self, node: &PsqlPlDiagnosticsItem, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPlDiagnosticsItemFields {
            target,
            operator,
            item,
        } = node.as_fields();

        write!(
            f,
            [
                target.format(),
                space(),
                operator.format(),
                space(),
                item.format()
            ]
        )
    }
}
