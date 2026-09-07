//! XML parser for Stack `.rx` resources and `.xdic` dictionaries.
//!
//! Built on the same `biome_rowan` / `biome_parser` toolkit as `sql_parser`
//! and `mlang_parser`.

// The lexer lands before the parser/token source that will drive it.
#[allow(dead_code)]
pub(crate) mod lexer;
