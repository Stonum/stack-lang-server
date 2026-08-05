use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlLanguageOption;
use sql_syntax::SqlLanguageOptionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlLanguageOption;
impl FormatNodeRule<SqlLanguageOption> for FormatSqlLanguageOption {
    fn fmt_fields(&self, node: &SqlLanguageOption, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlLanguageOptionFields {
            language_token,
            name,
        } = node.as_fields();

        write!(f, [language_token.format(), space(), name.format()])
    }
}
