# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project rules

- Never commit on your own. Implementing a task (even when the user says "implement and commit") does not authorize the actual `git commit` — always stop and get explicit confirmation immediately before committing.
- Never hand-edit files under any `generated/` directory (`{mlang,sql,xml}_syntax/src/generated`, `{mlang,sql,xml}_factory/src/generated`). These are produced by codegen from the `.ungram` grammar files in `codegen/` (`m.ungram`, `sql.ungram`, `xml.ungram`). If generated code needs to change (including lint/warning fixes), fix the grammar or codegen tooling and regenerate — or flag the issue to the user instead of patching the generated output directly.
  - The codegen tool is **not** in this repo: it is an external fork of biome's `xtask_codegen` (grammar + formatter-scaffold subcommands). Claude cannot run it — ask the user to regenerate.
  - Exception to know about: `mlang_syntax/src/generated/kind.rs` contains a hand-authored bilingual (EN/RU) keyword table that the tool does not produce; after a `grammar m` regen, new kinds are patched into it by hand and its keyword section is left untouched.
  - After any grammar regen, run the full parser test suite (`cargo test -p mlang_parser` etc.) — slot-validation mismatches silently collapse nodes to `*_BOGUS` without diagnostics.
- `.ungram` files must stay LF (enforced via `.gitattributes`); the grammar parser rejects CRLF.

## Commands

Toolchain is pinned to Rust 1.90.0, edition 2024. On Windows the linker is `rust-lld.exe` (see `.cargo/`).

```bash
cargo build                                  # binary: stack-lang-server (crate `lsp`)
cargo fmt
cargo clippy --all-targets --all-features -- -D warnings   # same as CI and the pre-commit hook
cargo test                                   # whole workspace
cargo test -p sql_formatter                  # one crate
cargo test -p sql_formatter --test select_statement        # one integration test file
cargo test -p mlang_formatter --test formatter some_test_name   # one test
```

The pre-commit hook runs `cargo fmt -- --check` and clippy with `-D warnings`; run both after every change. CI (`.github/workflows/test.yml`) only triggers on `lsp/`, `line_index/`, `mlang_*` paths. Release builds are triggered by a commit whose message starts with `bump version ` on `main` (version lives in `[workspace.package]` of the root `Cargo.toml`).

## Architecture

An LSP server for the Stack platform's languages, built on biome's infrastructure (`biome_rowan` CST, `biome_parser`, a forked `biome_formatter`). Three languages, each split into the same biome-style crate stack:

| Layer | mlang (`.prg`, scripts) | SQL (`.sql`) | XML (`.rx*`, `.xdic`) |
|---|---|---|---|
| `*_syntax` — node/kind types (mostly generated) + `FileSource` | `mlang_syntax` | `sql_syntax` | `xml_syntax` |
| `*_factory` — node constructors (generated) | `mlang_factory` | `sql_factory` | `xml_factory` |
| `*_parser` — hand-written lexer + recursive-descent `syntax_rules` | `mlang_parser` | `sql_parser` | `xml_parser` |
| `*_formatter` — one `FormatNodeRule` per node under `rules/`/per-language dirs, glued by `generated.rs` | `mlang_formatter` | `sql_formatter` | `xml_formatter` |
| semantic / lint | `mlang_semantic`, `mlang_lint`, `mlang_lsp_definition` | — | `xml_semantic`, `xml_lint` |

Other crates: `mlang_core` (JSON/YAML data for the built-in mlang API), `line_index` (offset ↔ LSP position), `lsp` (tower-lsp server: `main.rs` = `Backend`/handlers, `workspace.rs` = indexed files and symbols, `document.rs` = `DocumentKind` dispatch by file extension, `format.rs`, `tokens.rs` = semantic tokens).

Key cross-cutting points:

- **mlang is bilingual** — keywords have English and Russian spellings (and several synonyms); the formatter can normalize keyword language. Parser rules often accept synonyms that the grammar must model as labelled alternations.
- **Embedded SQL in mlang**: `mlang_parser/src/sql_literal_rewriter.rs` is a post-parse pass that reclassifies string literals which actually parse as SQL (via `sql_parser`) into `MSqlStringLiteralExpression`/`MSqlLongStringLiteralExpression`; `mlang_formatter` then formats them with `sql_formatter`. SQL parsed from `.sql` files uses the Postgres dialect with the mlang extension enabled (`SqlFileSource::with_mlang_extension`). `SqlDialect` = Standard / Postgres / Mssql.
- Formatter rules not yet implemented fall back to `format_verbatim_node`, so a statement-level format test may not exercise a node's own rule — `sql_formatter` tests use `assert_fmt_node!` to format a specific node kind directly.

## Tests

Tests live in each crate's `tests/` as integration tests with shared macros in `tests/helper` (`assert_fmt!`, `assert_fmt_eq!`, `assert_fmt_node!`). `assert_fmt!` checks round-trip *and* idempotency, so it only proves anything when paired with cases whose input is actually mis-formatted (use `assert_fmt_eq!` for those). Prefer raw strings (`r#"..."#`) for fixtures. Don't put real client names or paths in committed code, tests or comments.
