//! Parser functions for constructs that only exist under
//! [crate::SqlSyntaxFeature::Postgres] -- mirrors how `biome_js_parser`
//! keeps every TypeScript-only parse function under its own
//! `syntax/typescript` module rather than scattered through the shared JS
//! grammar files. Each module here still parses unconditionally (see
//! [biome_parser::SyntaxFeature::parse_exclusive_syntax]/[biome_parser::SyntaxFeature::exclusive_syntax])
//! and is called from a thin dispatch site in the matching shared module
//! (e.g. `postgres::returning_clause::parse_returning_clause` is called
//! from `insert.rs`/`update.rs`/`delete.rs`), so callers elsewhere in the
//! grammar don't need their own per-dialect branches.

pub(crate) mod ddl;
pub(crate) mod delete_using_clause;
pub(crate) mod expr;
pub(crate) mod from;
pub(crate) mod on_conflict_clause;
pub(crate) mod parse_error;
pub(crate) mod returning_clause;
pub(crate) mod select;
pub(crate) mod with_clause;
