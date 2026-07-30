use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, count_dotted_name_segments, is_at_tilde_name_start, parse_any_name,
    parse_expression, parse_name, parse_string_literal_expression, parse_table_name,
    parse_type_name,
};
use super::parse_error::*;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

/// Dispatches `CREATE ...` to whichever DDL shape follows -- currently
/// `CREATE TABLE` and `CREATE FUNCTION`/`CREATE PROCEDURE`. Any other
/// `CREATE ...` falls through to `Absent`, letting the caller's ordinary
/// bogus-statement recovery handle it, the same way an unimplemented
/// statement always has.
pub(crate) fn parse_create_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![create]) {
        return Absent;
    }

    if p.nth_at(1, T![table]) {
        return parse_create_table_statement(p);
    }
    if p.nth_at(1, T![function]) || p.nth_at(1, T![procedure]) {
        return parse_create_function_statement(p);
    }

    Absent
}

/// `CREATE FUNCTION|PROCEDURE name(param type, ...) [RETURNS type] AS body
/// [LANGUAGE name] [;]` -- minimal skeleton (see the matching note in
/// `codegen/psql.ungram` for what's deliberately not supported yet).
fn parse_create_function_statement(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    p.bump_any(); // 'function' | 'procedure'

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T!['(']);
    PsqlFunctionParameterList.parse_list(p);
    p.expect(T![')']);

    let _ = parse_returns_clause(p);

    p.expect(T![as]);
    parse_string_literal_expression(p).or_add_diagnostic(p, expected_expression);

    let _ = parse_language_option(p);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_CREATE_FUNCTION_STATEMENT))
}

fn parse_returns_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![returns]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![returns]);
    if p.at(T![table]) {
        parse_returns_table_clause(p);
    } else if p.at(T![trigger]) {
        let trigger_clause = p.start();
        p.bump(T![trigger]);
        trigger_clause.complete(p, PSQL_RETURNS_TRIGGER_CLAUSE);
    } else {
        parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    }
    Present(m.complete(p, PSQL_RETURNS_CLAUSE))
}

/// `TABLE(col type, col type, ...)` -- a set-returning function's
/// result-row shape.
fn parse_returns_table_clause(p: &mut PsqlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![table]);
    p.expect(T!['(']);
    PsqlReturnsTableColumnList.parse_list(p);
    p.expect(T![')']);
    m.complete(p, PSQL_RETURNS_TABLE_CLAUSE)
}

struct PsqlReturnsTableColumnList;

impl ParseSeparatedList for PsqlReturnsTableColumnList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_RETURNS_TABLE_COLUMN_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_returns_table_column(p)
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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_returns_table_column(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    Present(m.complete(p, PSQL_RETURNS_TABLE_COLUMN))
}

fn parse_language_option(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![language]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![language]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, PSQL_LANGUAGE_OPTION))
}

struct PsqlFunctionParameterList;

impl ParseSeparatedList for PsqlFunctionParameterList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_FUNCTION_PARAMETER_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_function_parameter(p)
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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn is_at_parameter_mode(p: &mut PsqlParser) -> bool {
    p.at(T![in]) || p.at(T![out]) || p.at(T![inout])
}

fn parse_function_parameter(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) && !is_at_parameter_mode(p) {
        return Absent;
    }

    let m = p.start();
    if is_at_parameter_mode(p) {
        p.bump_any();
    }
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    let _ = parse_parameter_default(p);
    Present(m.complete(p, PSQL_FUNCTION_PARAMETER))
}

fn parse_parameter_default(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![default]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![default]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_PARAMETER_DEFAULT))
}

/// `CREATE TABLE [IF NOT EXISTS] name (col type, col type, ...) [;]` --
/// column list only for now, no column-level or table-level constraints
/// yet (see the matching note in `codegen/psql.ungram`).
fn parse_create_table_statement(p: &mut PsqlParser) -> ParsedSyntax {
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
    PsqlColumnDefinitionList.parse_list(p);
    p.expect(T![')']);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_CREATE_TABLE_STATEMENT))
}

struct PsqlColumnDefinitionList;

impl ParseSeparatedList for PsqlColumnDefinitionList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_COLUMN_DEFINITION_LIST;

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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_column_definition(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    Present(m.complete(p, PSQL_COLUMN_DEFINITION))
}

/// Dispatches `DROP ...` to whichever DDL shape follows -- currently `DROP
/// TABLE` and `DROP FUNCTION`/`DROP PROCEDURE`, the only two seen in real
/// client scripts so far.
pub(crate) fn parse_drop_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![drop]) {
        return Absent;
    }

    if p.nth_at(1, T![table]) {
        return parse_drop_table_statement(p);
    }

    parse_drop_function_statement(p)
}

/// `DROP TABLE [IF EXISTS] name (',' name)* [CASCADE|RESTRICT] [;]`
fn parse_drop_table_statement(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![table]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    PsqlTableNameList.parse_list(p);

    if p.at(T![cascade]) || p.at(T![restrict]) {
        p.bump_any();
    }

    p.eat(T![;]);

    Present(m.complete(p, PSQL_DROP_TABLE_STATEMENT))
}

/// A plain or schema-qualified table name (`table`/`schema.table`), or a
/// mlang tilde name -- no alias, unlike [super::from::parse_table_binding].
fn parse_table_name_for_ddl(p: &mut PsqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return Present(parse_table_name(p, 0));
    }

    if !p.at(T![ident]) {
        return Absent;
    }

    let segment_count = count_dotted_name_segments(p).min(3);
    Present(parse_table_name(p, segment_count))
}

struct PsqlTableNameList;

impl ParseSeparatedList for PsqlTableNameList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_TABLE_NAME_LIST;

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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, EXPR_RECOVERY_SET),
            expected_table_binding,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// `DROP FUNCTION|PROCEDURE [IF EXISTS] name [(type, ...)] [CASCADE|RESTRICT] [;]`
/// -- Postgres DDL for removing a stored function/procedure. `GO` (a T-SQL
/// batch separator some client scripts still carry over from MSSQL) isn't
/// part of this grammar at all; it already recovers fine as ordinary
/// bogus-statement content between two real statements.
fn parse_drop_function_statement(p: &mut PsqlParser) -> ParsedSyntax {
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
        return Present(m.complete(p, PSQL_BOGUS_STATEMENT));
    }
    p.bump_any(); // 'function' | 'procedure'

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);
    let _ = parse_drop_function_parameters(p);

    if p.at(T![cascade]) || p.at(T![restrict]) {
        p.bump_any();
    }

    p.eat(T![;]);

    Present(m.complete(p, PSQL_DROP_FUNCTION_STATEMENT))
}

fn parse_drop_function_parameters(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    let m = p.start();
    p.bump(T!['(']);
    PsqlTypeNameList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_DROP_FUNCTION_PARAMETERS))
}

struct PsqlTypeNameList;

impl ParseSeparatedList for PsqlTypeNameList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_TYPE_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_type_name(p)
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
            &ParseRecoveryTokenSet::new(PSQL_BOGUS, token_set![T![')']]),
            expected_type_name,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
