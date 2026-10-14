use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlColumnPath;
use sql_syntax::PsqlXmlColumnPathFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlColumnPath;
impl FormatNodeRule<PsqlXmlColumnPath> for FormatPsqlXmlColumnPath {
    fn fmt_fields(&self, node: &PsqlXmlColumnPath, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlColumnPathFields { path_token, value } = node.as_fields();

        write!(f, [path_token.format(), space(), value.format()])
    }
}
