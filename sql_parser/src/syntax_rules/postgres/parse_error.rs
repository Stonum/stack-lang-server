use crate::SqlParser;

use biome_parser::prelude::*;
use biome_rowan::TextRange;

/// Shared diagnostic for every [crate::SqlSyntaxFeature::Postgres]-gated
/// construct (mirrors `biome_js_parser`'s own `ts_only_syntax_error`) --
/// `syntax` names the specific construct just parsed (e.g. `` "`RETURNING`
/// clauses" ``, `` "`ON CONFLICT`" ``), `range` is the already-completed
/// node's own range.
pub(crate) fn postgres_only_syntax_error(
    p: &SqlParser,
    syntax: &str,
    range: TextRange,
) -> ParseDiagnostic {
    p.err_builder(
        std::format!(
            "{syntax} is a Postgres-only feature, not supported by the Standard or Mssql dialect."
        ),
        range,
    )
    .with_hint("Postgres-only syntax")
}
