use crate::prelude::*;
use psql_syntax::PsqlSyntaxToken;

use biome_formatter::trivia::format_skipped_token_trivia;
use biome_formatter::write;

use crate::PsqlFormatContext;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSyntaxToken;
impl_format!(PsqlSyntaxToken, FormatPsqlSyntaxToken);

impl FormatRule<PsqlSyntaxToken> for FormatPsqlSyntaxToken {
    type Context = PsqlFormatContext;

    fn fmt(&self, token: &PsqlSyntaxToken, f: &mut PsqlFormatter) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(f, [format_skipped_token_trivia(token)])?;

        if token.kind().is_keyword() {
            let lowercased = token.text_trimmed().to_lowercase();
            let start = token.text_trimmed_range().start();
            return write!(f, [dynamic_text(&lowercased, start)]);
        }
        write!(f, [format_trimmed_token(token)])
    }
}
