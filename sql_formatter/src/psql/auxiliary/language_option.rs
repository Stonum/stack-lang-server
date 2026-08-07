use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlLanguageOption;
use sql_syntax::PsqlLanguageOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLanguageOption;
impl FormatNodeRule<PsqlLanguageOption> for FormatPsqlLanguageOption {
    fn fmt_fields(&self, node: &PsqlLanguageOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlLanguageOptionFields {
            language_token,
            name,
        } = node.as_fields();

        write!(f, [language_token.format(), space(), name.format()])
    }
}
