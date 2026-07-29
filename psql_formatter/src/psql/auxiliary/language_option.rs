use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlLanguageOption;
use psql_syntax::PsqlLanguageOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLanguageOption;
impl FormatNodeRule<PsqlLanguageOption> for FormatPsqlLanguageOption {
    fn fmt_fields(&self, node: &PsqlLanguageOption, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlLanguageOptionFields {
            language_token,
            name,
        } = node.as_fields();

        write!(f, [language_token.format(), space(), name.format()])
    }
}
