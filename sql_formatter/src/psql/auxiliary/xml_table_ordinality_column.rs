use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlTableOrdinalityColumn;
use sql_syntax::PsqlXmlTableOrdinalityColumnFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlTableOrdinalityColumn;
impl FormatNodeRule<PsqlXmlTableOrdinalityColumn> for FormatPsqlXmlTableOrdinalityColumn {
    fn fmt_fields(
        &self,
        node: &PsqlXmlTableOrdinalityColumn,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlXmlTableOrdinalityColumnFields {
            name,
            for_token,
            ordinality_token,
        } = node.as_fields();

        write!(
            f,
            [
                name.format(),
                space(),
                for_token.format(),
                space(),
                ordinality_token.format()
            ]
        )
    }
}
