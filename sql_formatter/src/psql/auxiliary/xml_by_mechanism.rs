use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlXmlByMechanism;
use sql_syntax::PsqlXmlByMechanismFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlByMechanism;
impl FormatNodeRule<PsqlXmlByMechanism> for FormatPsqlXmlByMechanism {
    fn fmt_fields(&self, node: &PsqlXmlByMechanism, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlByMechanismFields { by_token, kind } = node.as_fields();

        write!(f, [by_token.format(), space(), kind.format()])
    }
}
