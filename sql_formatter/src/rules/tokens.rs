use crate::prelude::*;
use sql_syntax::{SqlSyntaxKind, SqlSyntaxToken};

use biome_formatter::trivia::format_skipped_token_trivia;
use biome_formatter::write;

use crate::SqlFormatContext;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlSyntaxToken;
impl_format!(SqlSyntaxToken, FormatSqlSyntaxToken);

impl FormatRule<SqlSyntaxToken> for FormatSqlSyntaxToken {
    type Context = SqlFormatContext;

    fn fmt(&self, token: &SqlSyntaxToken, f: &mut SqlFormatter) -> FormatResult<()> {
        f.state_mut().track_token(token);

        write!(f, [format_skipped_token_trivia(token)])?;

        let start = token.text_trimmed_range().start();
        if token.kind().is_keyword() {
            let canonical = token
                .kind()
                .to_string()
                .expect("every keyword kind has a canonical to_string() spelling");
            return write!(f, [dynamic_text(canonical, start)]);
        }

        let text = token.text_trimmed();
        // The mlang dialect's SQL-Server-style `[identifier]` bracket
        // quoting (re-lexed to plain `IDENT`, see
        // `SqlReLexContext::BracketName`) isn't valid Postgres syntax --
        // canonicalize it to Postgres's own `"identifier"` quoting on
        // output, same spirit as keyword canonicalization above. `""`
        // escapes any literal `"` in the content, matching how Postgres
        // itself escapes a quote inside a quoted identifier.
        if token.kind() == SqlSyntaxKind::IDENT
            && text.len() >= 2
            && text.starts_with('[')
            && text.ends_with(']')
        {
            let inner = &text[1..text.len() - 1];
            let canonical = std::format!("\"{}\"", inner.replace('"', "\"\""));
            return write!(f, [dynamic_text(&canonical, start)]);
        }
        if text.contains('\r') {
            // A multi-line token (only ever a string/dollar-quoted literal
            // in this grammar -- e.g. a verbatim PL/pgSQL function body)
            // keeps its original text as-is on a CRLF source file, `\r\n`
            // line endings embedded in the token itself included (unlike
            // between-token trivia, which is already normalized
            // elsewhere). biome_formatter's text builder asserts against
            // raw `\r` in printed content, so normalize to `\n` for what's
            // actually written -- Postgres doesn't care about the
            // line-ending style inside a string.
            let normalized = text.replace("\r\n", "\n").replace('\r', "\n");
            let start = token.text_trimmed_range().start();
            return write!(f, [dynamic_text(&normalized, start)]);
        }

        write!(f, [format_trimmed_token(token)])
    }
}
