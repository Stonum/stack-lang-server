//! Aggregates every test file that's *entirely* about a
//! [sql_syntax::SqlDialect::Postgres]-gated construct (every test in it
//! parses under `.with_dialect(SqlDialect::Postgres)`, never bare
//! `SqlFileSource::script()`) into one binary, mirroring
//! `syntax_rules/postgres/` on the source side. Files that mix Standard
//! and Postgres-only tests for the same statement (e.g. `drop_table.rs`,
//! `create_view.rs`) stay flat in `tests/` instead -- splitting those
//! would separate tests about the same construct into two files for no
//! real benefit.

#[macro_use]
mod helper;

#[path = "postgres/create_function.rs"]
mod create_function;
#[path = "postgres/create_policy.rs"]
mod create_policy;
#[path = "postgres/create_trigger.rs"]
mod create_trigger;
#[path = "postgres/on_conflict_clause.rs"]
mod on_conflict_clause;
#[path = "postgres/returning_clause.rs"]
mod returning_clause;
