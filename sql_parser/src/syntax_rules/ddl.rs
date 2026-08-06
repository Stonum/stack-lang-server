use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, count_dotted_name_segments, is_at_name_start, is_at_tilde_name_start,
    parse_any_name, parse_name, parse_table_name, parse_type_name,
};
use super::parse_error::*;
use super::postgres::ddl::{
    eat_cascade_or_restrict, parse_create_function_statement, parse_create_policy_statement,
    parse_create_trigger_statement, parse_drop_function_parameters, parse_drop_policy_statement,
    parse_drop_trigger_statement, parse_view_options,
};
use super::select::parse_select_statement;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind::*, T, *};

/// Dispatches `CREATE ...` to whichever DDL shape follows -- currently
/// `CREATE TABLE`, `CREATE FUNCTION`/`CREATE PROCEDURE`, `CREATE VIEW`,
/// `CREATE POLICY`, and `CREATE TRIGGER`. Any other `CREATE ...` falls
/// through to `Absent`, letting the caller's ordinary bogus-statement
/// recovery handle it, the same way an unimplemented statement always has.
pub(crate) fn parse_create_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![create]) {
        return Absent;
    }

    if p.nth_at(1, T![table]) {
        return parse_create_table_statement(p);
    }
    if p.nth_at(1, T![policy]) {
        return parse_create_policy_statement(p);
    }
    if p.nth_at(1, T![trigger]) {
        return parse_create_trigger_statement(p);
    }

    // `FUNCTION`/`PROCEDURE`/`VIEW` can all be preceded by an optional `OR
    // REPLACE`, so a single fixed-offset `nth_at` isn't enough to tell them
    // apart -- peek past it (without consuming anything for real) to see
    // which keyword actually follows.
    let target = p.lookahead(|p| {
        p.bump(T![create]);
        if p.eat(T![or]) {
            p.eat(T![replace]);
        }
        p.cur()
    });

    match target {
        T![function] | T![procedure] => parse_create_function_statement(p),
        T![view] => parse_create_view_statement(p),
        _ => Absent,
    }
}

/// `CREATE TABLE [IF NOT EXISTS] name (col type, col type, ...) [;]` --
/// column list only for now, no column-level or table-level constraints
/// yet (see the matching note in `codegen/sql.ungram`).
fn parse_create_table_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    p.bump(T![table]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![not]);
        p.expect(T![exists]);
    }

    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    p.expect(T!['(']);
    SqlColumnDefinitionList.parse_list(p);
    p.expect(T![')']);

    p.eat(T![;]);

    Present(m.complete(p, SQL_CREATE_TABLE_STATEMENT))
}

struct SqlColumnDefinitionList;

impl ParseSeparatedList for SqlColumnDefinitionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_COLUMN_DEFINITION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_column_definition(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![')'])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_column_definition(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    Present(m.complete(p, SQL_COLUMN_DEFINITION))
}

/// `CREATE [OR REPLACE] VIEW name [WITH (opt=val, ...)] AS <select> [;]` --
/// the view body is an ordinary `SELECT` statement, already fully
/// supported; only the thin wrapper is new here. `WITH (...)` storage
/// options are Postgres-only -- see [parse_view_options].
fn parse_create_view_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);

    if p.at(T![or]) {
        p.bump(T![or]);
        p.expect(T![replace]);
    }
    p.bump(T![view]);

    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
    let _ = parse_view_options(p);

    p.expect(T![as]);
    parse_select_statement(p).or_add_diagnostic(p, expected_statement);

    p.eat(T![;]);

    Present(m.complete(p, SQL_CREATE_VIEW_STATEMENT))
}

/// `GRANT ALL ON [TABLE] name [, name]* TO grantee [, grantee]* [;]` --
/// only the bare `ALL` privilege spec is modeled; see the grammar comment
/// on `SqlGrantStatement` for why.
pub(crate) fn parse_grant_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![grant]);
    p.expect(T![all]);
    p.expect(T![on]);
    p.eat(T![table]);

    SqlGrantObjectList.parse_list(p);

    p.expect(T![to]);

    SqlGranteeList.parse_list(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_GRANT_STATEMENT))
}

struct SqlGrantObjectList;

impl ParseSeparatedList for SqlGrantObjectList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_TABLE_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_table_name_for_ddl(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![to])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_table_binding,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

struct SqlGranteeList;

impl ParseSeparatedList for SqlGranteeList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_GRANTEE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// Dispatches `DROP ...` to whichever DDL shape follows -- `DROP TABLE`,
/// `DROP VIEW`, `DROP POLICY`, `DROP TRIGGER`, and `DROP FUNCTION`/`DROP
/// PROCEDURE`.
pub(crate) fn parse_drop_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![drop]) {
        return Absent;
    }

    if p.nth_at(1, T![table]) {
        return parse_drop_table_statement(p);
    }
    if p.nth_at(1, T![view]) {
        return parse_drop_view_statement(p);
    }
    if p.nth_at(1, T![policy]) {
        return parse_drop_policy_statement(p);
    }
    if p.nth_at(1, T![trigger]) {
        return parse_drop_trigger_statement(p);
    }

    parse_drop_function_statement(p)
}

/// `DROP POLICY [IF EXISTS] name ON table [;]`
fn parse_drop_view_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![view]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    SqlTableNameList.parse_list(p);

    eat_cascade_or_restrict(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_DROP_VIEW_STATEMENT))
}

/// `DROP TABLE [IF EXISTS] name (',' name)* [CASCADE|RESTRICT] [;]`
fn parse_drop_table_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![table]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    SqlTableNameList.parse_list(p);

    eat_cascade_or_restrict(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_DROP_TABLE_STATEMENT))
}

/// A plain or schema-qualified table name (`table`/`schema.table`), or a
/// mlang tilde name -- no alias, unlike [super::from::parse_table_binding].
pub(crate) fn parse_table_name_for_ddl(p: &mut SqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return Present(parse_table_name(p, 0));
    }

    if !p.at(T![ident]) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    Present(parse_table_name(p, segment_count))
}

struct SqlTableNameList;

impl ParseSeparatedList for SqlTableNameList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_TABLE_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_table_name_for_ddl(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;]) || p.at(T![cascade]) || p.at(T![restrict])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_table_binding,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// `DROP FUNCTION|PROCEDURE [IF EXISTS] name [(type, ...)] [CASCADE|RESTRICT] [;]`
/// -- Postgres DDL for removing a stored function/procedure, though the
/// bare `DROP FUNCTION name` shape is also valid T-SQL (Standard); the
/// parenthesized type list and `CASCADE`/`RESTRICT` are Postgres-only
/// extensions on top, gated individually rather than the whole statement
/// (see [parse_drop_function_parameters]/[eat_cascade_or_restrict]). `GO`
/// (a T-SQL batch separator some client scripts still carry over from
/// MSSQL) isn't part of this grammar at all; it already recovers fine as
/// ordinary bogus-statement content between two real statements.
fn parse_drop_function_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![drop]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![drop]);

    if !p.at(T![function]) && !p.at(T![procedure]) {
        let range = p.cur_range();
        let err = p
            .err_builder("Expected `function` or `procedure` after `drop`", range)
            .with_hint("Only `DROP FUNCTION`/`DROP PROCEDURE` is supported");
        p.error(err);
        return Present(m.complete(p, SQL_BOGUS_STATEMENT));
    }
    p.bump_any(); // 'function' | 'procedure'

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);
    let _ = parse_drop_function_parameters(p);

    eat_cascade_or_restrict(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_DROP_FUNCTION_STATEMENT))
}
