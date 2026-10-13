use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, count_dotted_name_segments, is_at_name_segment_start, is_at_name_start,
    is_at_tilde_name_start, parse_any_name, parse_column_name_list, parse_expression, parse_name,
    parse_subquery_expression, parse_table_name, parse_type_name,
};
use super::index::{parse_create_index_statement, parse_drop_index_statement};
use super::parse_error::*;
use super::postgres::ddl::{
    eat_cascade_or_restrict, eat_table_persistence, parse_create_function_statement,
    parse_create_policy_statement, parse_create_trigger_statement, parse_drop_function_parameters,
    parse_drop_policy_statement, parse_drop_trigger_statement, parse_generated_constraint,
    parse_on_commit_clause, parse_table_query, parse_truncate_identity_clause, parse_view_options,
    parse_with_data_clause,
};
use super::postgres::extension::{
    parse_create_extension_statement, parse_drop_extension_statement,
};
use super::postgres::materialized_view::{
    parse_create_materialized_view_statement, parse_drop_materialized_view_statement,
};
use super::postgres::partition::{
    complete_create_table_partition_of, parse_partition_by_clause, parse_table_like_element,
};
use super::stmt::STMT_RECOVERY_SET;
use super::with_clause::parse_with_prefixed_select_statement;
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

    if p.nth_at(1, T![table]) || (is_at_table_persistence(p, 1) && p.nth_at(2, T![table])) {
        return parse_create_table_statement(p);
    }
    if p.nth_at(1, T![index]) || (p.nth_at(1, T![unique]) && p.nth_at(2, T![index])) {
        return parse_create_index_statement(p);
    }
    if p.nth_at(1, T![materialized]) {
        return parse_create_materialized_view_statement(p);
    }
    if p.nth_at(1, T![extension]) {
        return parse_create_extension_statement(p);
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

/// `CREATE [TEMP|TEMPORARY|UNLOGGED] TABLE [IF NOT EXISTS] name
/// (column | table constraint, ...) [ON COMMIT ...] [;]`, or the
/// `... name [(col, ...)] [ON COMMIT ...] AS query [WITH [NO] DATA]` form.
fn parse_create_table_statement(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    eat_table_persistence(p);
    p.bump(T![table]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![not]);
        p.expect(T![exists]);
    }

    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    if p.at(T![partition]) && p.nth_at(1, T![of]) {
        return Present(complete_create_table_partition_of(p, m));
    }

    if is_at_create_table_as(p) {
        let _ = parse_column_name_list(p);
        let _ = parse_on_commit_clause(p);
        p.expect(T![as]);
        parse_create_table_as_query(p).or_add_diagnostic(p, expected_statement);
        let _ = parse_with_data_clause(p);
        p.eat(T![;]);
        return Present(m.complete(p, SQL_CREATE_TABLE_AS_STATEMENT));
    }

    p.expect(T!['(']);
    SqlTableElementList.parse_list(p);
    p.expect(T![')']);

    let _ = parse_partition_by_clause(p);
    let _ = parse_on_commit_clause(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_CREATE_TABLE_STATEMENT))
}

/// After the table name: `AS` follows, possibly behind a parenthesized
/// column list and an `ON COMMIT` clause.
fn is_at_create_table_as(p: &mut SqlParser) -> bool {
    p.lookahead(|p| {
        if p.at(T!['(']) {
            let mut depth = 0;
            loop {
                match p.cur() {
                    T!['('] => depth += 1,
                    T![')'] => depth -= 1,
                    EOF => return false,
                    _ => {}
                }
                p.bump_any();
                if depth == 0 {
                    break;
                }
            }
        }
        if p.at(T![on]) && p.nth_at(1, T![commit]) {
            p.bump_any();
            p.bump_any();
            if !p.at(EOF) {
                p.bump_any();
            }
            p.eat(T![rows]);
        }
        p.at(T![as])
    })
}

/// `select ...`, `with ... select ...`, `(select ...)` or `TABLE name`.
pub(crate) fn parse_create_table_as_query(p: &mut SqlParser) -> ParsedSyntax {
    match p.cur() {
        T![select] | T![with] => parse_with_prefixed_select_statement(p),
        T!['('] => parse_subquery_expression(p),
        T![table] => parse_table_query(p),
        _ => Absent,
    }
}

fn is_at_table_persistence(p: &mut SqlParser, n: usize) -> bool {
    p.nth_at(n, T![temp]) || p.nth_at(n, T![temporary]) || p.nth_at(n, T![unlogged])
}

struct SqlTableElementList;

impl ParseSeparatedList for SqlTableElementList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_TABLE_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        if is_at_table_constraint_start(p) {
            parse_table_constraint(p)
        } else if p.at(T![like]) {
            parse_table_like_element(p)
        } else {
            parse_column_definition(p)
        }
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

pub(crate) fn parse_column_definition(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);

    let constraints = p.start();
    while parse_column_constraint(p).is_present() {}
    constraints.complete(p, SQL_COLUMN_CONSTRAINT_LIST);

    Present(m.complete(p, SQL_COLUMN_DEFINITION))
}

pub(crate) fn is_at_table_constraint_start(p: &mut SqlParser) -> bool {
    p.at(T![constraint])
        || p.at(T![primary])
        || p.at(T![unique])
        || p.at(T![check])
        || p.at(T![foreign])
}

/// `[CONSTRAINT name] PRIMARY KEY (...) | UNIQUE (...) | CHECK (...) |
/// FOREIGN KEY (...) REFERENCES ...`
pub(crate) fn parse_table_constraint(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    if p.at(T![constraint]) {
        let name = p.start();
        p.bump(T![constraint]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        name.complete(p, SQL_CONSTRAINT_NAME);
    }

    let constraint = p.start();
    let kind = match p.cur() {
        T![primary] => {
            p.bump(T![primary]);
            p.expect(T![key]);
            parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
            SQL_PRIMARY_KEY_TABLE_CONSTRAINT
        }
        T![unique] => {
            p.bump(T![unique]);
            parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
            SQL_UNIQUE_TABLE_CONSTRAINT
        }
        T![check] => {
            bump_check_constraint(p);
            SQL_CHECK_CONSTRAINT
        }
        T![foreign] => {
            p.bump(T![foreign]);
            p.expect(T![key]);
            parse_column_name_list(p).or_add_diagnostic(p, expected_column_list);
            parse_references_constraint(p).or_add_diagnostic(p, expected_references_clause);
            SQL_FOREIGN_KEY_TABLE_CONSTRAINT
        }
        _ => {
            constraint.abandon(p);
            let range = p.cur_range();
            p.error(expected_table_constraint(p, range));
            return Present(m.complete(p, SQL_TABLE_CONSTRAINT));
        }
    };
    constraint.complete(p, kind);

    Present(m.complete(p, SQL_TABLE_CONSTRAINT))
}

/// `CHECK (condition)`; the parser is at `check`.
fn bump_check_constraint(p: &mut SqlParser) {
    p.bump(T![check]);
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![')']);
}

/// `REFERENCES table [(column, ...)]`
fn parse_references_constraint(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![references]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![references]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
    let _ = parse_column_name_list(p);
    Present(m.complete(p, SQL_REFERENCES_CONSTRAINT))
}

/// One column constraint after the column's type, or a `CONSTRAINT name`
/// naming the one that follows.
fn parse_column_constraint(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    let kind = match p.cur() {
        T![constraint] => {
            p.bump(T![constraint]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            SQL_CONSTRAINT_NAME
        }
        T![primary] => {
            p.bump(T![primary]);
            p.expect(T![key]);
            SQL_PRIMARY_KEY_CONSTRAINT
        }
        T![unique] => {
            p.bump(T![unique]);
            SQL_UNIQUE_CONSTRAINT
        }
        T![check] => {
            bump_check_constraint(p);
            SQL_CHECK_CONSTRAINT
        }
        T![references] => {
            m.abandon(p);
            return parse_references_constraint(p);
        }
        T![generated] => {
            m.abandon(p);
            return parse_generated_constraint(p);
        }
        T![null] => {
            p.bump(T![null]);
            SQL_NULL_CONSTRAINT
        }
        T![not] if p.nth_at(1, T![null]) => {
            p.bump(T![not]);
            p.bump(T![null]);
            SQL_NOT_NULL_CONSTRAINT
        }
        T![default] => {
            p.bump(T![default]);
            parse_expression(p).or_add_diagnostic(p, expected_expression);
            SQL_DEFAULT_CONSTRAINT
        }
        T![collate] => {
            p.bump(T![collate]);
            parse_name(p).or_add_diagnostic(p, expected_identifier);
            SQL_COLLATE_CONSTRAINT
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };
    Present(m.complete(p, kind))
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
    let _ = parse_column_name_list(p);
    let _ = parse_view_options(p);

    p.expect(T![as]);
    parse_create_table_as_query(p).or_add_diagnostic(p, expected_statement);

    p.eat(T![;]);

    Present(m.complete(p, SQL_CREATE_VIEW_STATEMENT))
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
    if p.nth_at(1, T![index]) {
        return parse_drop_index_statement(p);
    }
    if p.nth_at(1, T![materialized]) {
        return parse_drop_materialized_view_statement(p);
    }
    if p.nth_at(1, T![extension]) {
        return parse_drop_extension_statement(p);
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

/// `TRUNCATE [TABLE] name, ... [RESTART|CONTINUE IDENTITY] [CASCADE|RESTRICT] [;]`
pub(crate) fn parse_truncate_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![truncate]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![truncate]);
    p.eat(T![table]);

    SqlTableNameList.parse_list(p);

    let _ = parse_truncate_identity_clause(p);
    eat_cascade_or_restrict(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_TRUNCATE_STATEMENT))
}

/// A plain or schema-qualified table name (`table`/`schema.table`), or a
/// mlang tilde name -- no alias, unlike [super::from::parse_table_binding].
pub(crate) fn parse_table_name_for_ddl(p: &mut SqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return Present(parse_table_name(p, 0));
    }

    if !is_at_name_segment_start(p) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    Present(parse_table_name(p, segment_count))
}

pub(crate) struct SqlTableNameList;

impl ParseSeparatedList for SqlTableNameList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_TABLE_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_table_name_for_ddl(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // The next statement may follow without a `;` in mlang command batches.
        p.at(EOF)
            || p.at_ts(STMT_RECOVERY_SET)
            || p.at(T![cascade])
            || p.at(T![restrict])
            || p.at(T![restart])
            || p.at(T![continue])
            || p.at(T![in])
            || p.at(T![nowait])
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

    fn allow_empty(&self) -> bool {
        false
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
