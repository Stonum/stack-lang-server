use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::parse_table_name_for_ddl;
use crate::syntax_rules::expr::{
    TYPE_NAME_TOKEN_SET, count_dotted_name_segments, is_at_name_start, is_at_tilde_name_start,
    parse_any_name, parse_call_expression, parse_expression, parse_name,
    parse_string_literal_expression, parse_type_name,
};
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_table_binding, expected_type_name,
};
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

/// `CASCADE`/`RESTRICT` after `DROP TABLE`/`DROP VIEW`/`DROP FUNCTION` --
/// Postgres-only, no equivalent at all in T-SQL's `DROP` grammar (`DROP
/// POLICY`/`DROP TRIGGER` don't need their own call site here since the
/// whole statement is already Postgres-gated -- see
/// [parse_drop_policy_statement]/[parse_drop_trigger_statement]). Bumped
/// unconditionally once seen so the parser always makes progress, then
/// diagnosed afterward if the active dialect doesn't support it -- there's
/// no dedicated node to mark bogus for a single modifier keyword, so this
/// reports the diagnostic directly rather than going through
/// [SyntaxFeature::exclusive_syntax].
pub(crate) fn eat_cascade_or_restrict(p: &mut SqlParser) {
    if !(p.at(T![cascade]) || p.at(T![restrict])) {
        return;
    }

    let range = p.cur_range();
    p.bump_any();
    if !SqlSyntaxFeature::Postgres.is_supported(p) {
        p.error(postgres_only_syntax_error(p, "`CASCADE`/`RESTRICT`", range));
    }
}

/// `CREATE [OR REPLACE] FUNCTION|PROCEDURE name(param type, ...) [RETURNS
/// type] <options>* AS body <options>* [;]` (see the matching note in
/// `codegen/sql.ungram` for the option-list shape). Postgres-only: T-SQL's
/// `CREATE FUNCTION`/`CREATE PROCEDURE` use an entirely different shape
/// (`AS BEGIN ... END`, no `LANGUAGE`, no `STRICT`/volatility/`SECURITY
/// DEFINER` options), so the whole statement is gated rather than any one
/// piece of it. Only ever reached once [crate::syntax_rules::ddl::parse_create_statement]'s
/// own lookahead has already confirmed `function`/`procedure` follows, so
/// it parses unconditionally rather than re-checking its own "is at" gate.
pub(crate) fn parse_create_function_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_create_function_statement_body,
        |p, marker| {
            postgres_only_syntax_error(p, "`CREATE FUNCTION`/`CREATE PROCEDURE`", marker.range(p))
        },
    )
}

fn parse_create_function_statement_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);

    if p.at(T![or]) {
        p.bump(T![or]);
        p.expect(T![replace]);
    }

    if !p.at(T![function]) && !p.at(T![procedure]) {
        let range = p.cur_range();
        let err = p
            .err_builder("Expected `function` or `procedure` after `create`", range)
            .with_hint("Only `CREATE FUNCTION`/`CREATE PROCEDURE` is supported");
        p.error(err);
        return Present(m.complete(p, SQL_BOGUS_STATEMENT));
    }
    p.bump_any(); // 'function' | 'procedure'

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T!['(']);
    SqlFunctionParameterList.parse_list(p);
    p.expect(T![')']);

    let _ = parse_returns_clause(p);
    parse_function_option_list(p);

    p.expect(T![as]);
    parse_string_literal_expression(p).or_add_diagnostic(p, expected_expression);

    parse_function_option_list(p);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_CREATE_FUNCTION_STATEMENT))
}

/// Zero or more options (`LANGUAGE`, volatility, `SECURITY DEFINER`/
/// `INVOKER`, `STRICT`) in any order -- real scripts scatter these both
/// before and after `AS`, so this is called from both positions.
fn parse_function_option_list(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    while is_at_function_option(p) {
        let _ = parse_function_option(p);
    }
    m.complete(p, PSQL_FUNCTION_OPTION_LIST)
}

fn is_at_function_option(p: &mut SqlParser) -> bool {
    p.at(T![language])
        || p.at(T![immutable])
        || p.at(T![stable])
        || p.at(T![volatile])
        || p.at(T![security])
        || p.at(T![strict])
        // The primary `RETURNS <type>` clause is always parsed separately,
        // before the option list is ever reached, so a `returns` seen here
        // can only be the start of `RETURNS NULL ON NULL INPUT`.
        || p.at(T![returns])
}

fn parse_function_option(p: &mut SqlParser) -> ParsedSyntax {
    match p.cur() {
        T![language] => parse_language_option(p),
        T![immutable] | T![stable] | T![volatile] => parse_volatility_option(p),
        T![security] => parse_security_option(p),
        T![strict] => parse_strict_option(p),
        T![returns] => parse_returns_null_option(p),
        _ => Absent,
    }
}

/// `RETURNS NULL ON NULL INPUT` -- a synonym for `STRICT`.
fn parse_returns_null_option(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![returns]);
    p.expect(T![null]);
    p.expect(T![on]);
    p.expect(T![null]);
    p.expect(T![input]);
    Present(m.complete(p, PSQL_RETURNS_NULL_OPTION))
}

fn parse_volatility_option(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_any(); // 'immutable' | 'stable' | 'volatile'
    Present(m.complete(p, PSQL_VOLATILITY_OPTION))
}

fn parse_security_option(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![security]);
    if p.at(T![definer]) || p.at(T![invoker]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `definer` or `invoker` after `security`", range);
        p.error(err);
    }
    Present(m.complete(p, PSQL_SECURITY_OPTION))
}

fn parse_strict_option(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![strict]);
    Present(m.complete(p, PSQL_STRICT_OPTION))
}

fn parse_returns_clause(p: &mut SqlParser) -> ParsedSyntax {
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
    } else if p.at(T![setof]) {
        let setof_clause = p.start();
        p.bump(T![setof]);
        parse_type_name(p).or_add_diagnostic(p, expected_type_name);
        setof_clause.complete(p, PSQL_RETURNS_SETOF_CLAUSE);
    } else {
        parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    }
    Present(m.complete(p, PSQL_RETURNS_CLAUSE))
}

/// `TABLE(col type, col type, ...)` -- a set-returning function's
/// result-row shape.
fn parse_returns_table_clause(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    p.bump(T![table]);
    p.expect(T!['(']);
    SqlReturnsTableColumnList.parse_list(p);
    p.expect(T![')']);
    m.complete(p, PSQL_RETURNS_TABLE_CLAUSE)
}

struct SqlReturnsTableColumnList;

impl ParseSeparatedList for SqlReturnsTableColumnList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_returns_table_column(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    Present(m.complete(p, PSQL_RETURNS_TABLE_COLUMN))
}

fn parse_language_option(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![language]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![language]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, PSQL_LANGUAGE_OPTION))
}

struct SqlFunctionParameterList;

impl ParseSeparatedList for SqlFunctionParameterList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![')']]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn is_at_parameter_mode(p: &mut SqlParser) -> bool {
    p.at(T![in]) || p.at(T![out]) || p.at(T![inout])
}

/// `name type` is only unambiguous when the token after the candidate name
/// also starts a type name -- a lone `ident` (e.g. `create function
/// f(a)`) is Postgres's anonymous-parameter shorthand (`a` is the *type*,
/// a user-defined type name), not a name with a missing type.
fn is_at_named_parameter(p: &mut SqlParser) -> bool {
    is_at_name_start(p) && TYPE_NAME_TOKEN_SET.contains(p.nth(1))
}

fn parse_function_parameter(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_name_start(p) && !is_at_parameter_mode(p) && !p.at_ts(TYPE_NAME_TOKEN_SET) {
        return Absent;
    }

    let m = p.start();
    if is_at_parameter_mode(p) {
        p.bump_any();
    }
    if is_at_named_parameter(p) {
        parse_name(p).or_add_diagnostic(p, expected_identifier);
    }
    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    let _ = parse_parameter_default(p);
    Present(m.complete(p, PSQL_FUNCTION_PARAMETER))
}

/// Postgres allows either spelling for a parameter default -- `DEFAULT
/// expr` or the shorthand `= expr` -- both seen in real scripts.
fn parse_parameter_default(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![default]) && !p.at(T![=]) {
        return Absent;
    }

    let m = p.start();
    p.bump_any(); // 'default' | '='
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_PARAMETER_DEFAULT))
}

/// `WITH (opt=val, ...)` view storage options -- Postgres-only, T-SQL's
/// `CREATE VIEW` has an unrelated `WITH SCHEMABINDING`/`WITH ENCRYPTION`
/// options list instead.
pub(crate) fn parse_view_options(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(p, parse_view_options_body, |p, marker| {
        postgres_only_syntax_error(p, "View storage options (`WITH (...)`)", marker.range(p))
    })
}

fn parse_view_options_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![with]);
    p.expect(T!['(']);
    SqlViewOptionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_VIEW_OPTIONS))
}

struct SqlViewOptionList;

impl ParseSeparatedList for SqlViewOptionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_VIEW_OPTION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_view_option(p)
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

fn parse_view_option(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) {
        return Absent;
    }

    let m = p.start();
    parse_name(p).unwrap();
    p.expect(T![=]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_VIEW_OPTION))
}

/// `CREATE POLICY name ON table [FOR ALL|SELECT|INSERT|UPDATE|DELETE]
/// [USING (condition)] [WITH CHECK (condition)] [;]` -- both conditions are
/// ordinary boolean expressions, already fully supported. Postgres-only:
/// row-level security policies have no T-SQL equivalent. Only ever reached
/// once [crate::syntax_rules::ddl::parse_create_statement]'s own
/// `nth_at(1, policy)` check has already confirmed the shape, so it parses
/// unconditionally rather than re-checking its own "is at" gate.
pub(crate) fn parse_create_policy_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_create_policy_statement_body,
        |p, marker| postgres_only_syntax_error(p, "`CREATE POLICY`", marker.range(p)),
    )
}

fn parse_create_policy_statement_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    p.bump(T![policy]);

    parse_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T![on]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_policy_for_clause(p);
    let _ = parse_policy_using_clause(p);
    let _ = parse_policy_with_check_clause(p);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_CREATE_POLICY_STATEMENT))
}

fn parse_policy_for_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![for]);
    if p.at(T![all]) || p.at(T![select]) || p.at(T![insert]) || p.at(T![update]) || p.at(T![delete])
    {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder(
            "Expected `all`, `select`, `insert`, `update` or `delete` after `for`",
            range,
        );
        p.error(err);
    }
    Present(m.complete(p, PSQL_POLICY_FOR_CLAUSE))
}

fn parse_policy_using_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![using]);
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_POLICY_USING_CLAUSE))
}

fn parse_policy_with_check_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![with]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![with]);
    p.expect(T![check]);
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_POLICY_WITH_CHECK_CLAUSE))
}

/// `CREATE TRIGGER name {BEFORE|AFTER} event [OR event]* ON table
/// [REFERENCING ...] [FOR EACH ROW|STATEMENT] EXECUTE FUNCTION|PROCEDURE
/// name(args) [;]` -- the trigger *function* itself is a separate
/// `CREATE FUNCTION ... RETURNS TRIGGER`; this is just the attachment.
/// Postgres-only: T-SQL triggers use a different shape entirely (no
/// `BEFORE`, only `AFTER`/`INSTEAD OF`, no `REFERENCING`, the trigger body
/// is inlined via `inserted`/`deleted` pseudo-tables instead of a separate
/// `EXECUTE FUNCTION`). Only ever reached once
/// [crate::syntax_rules::ddl::parse_create_statement]'s own `nth_at(1,
/// trigger)` check has already confirmed the shape, so it parses
/// unconditionally rather than re-checking its own "is at" gate.
pub(crate) fn parse_create_trigger_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_create_trigger_statement_body,
        |p, marker| postgres_only_syntax_error(p, "`CREATE TRIGGER`", marker.range(p)),
    )
}

fn parse_create_trigger_statement_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![create]);
    p.bump(T![trigger]);

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    if p.at(T![before]) || p.at(T![after]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `before` or `after`", range);
        p.error(err);
    }
    parse_trigger_event_list(p);

    p.expect(T![on]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    let _ = parse_trigger_referencing_clause(p);
    let _ = parse_trigger_for_each_clause(p);
    let _ = parse_trigger_when_clause(p);

    p.expect(T![execute]);
    if p.at(T![function]) || p.at(T![procedure]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `function` or `procedure` after `execute`", range);
        p.error(err);
    }
    parse_trigger_function(p).or_add_diagnostic(p, expected_expression);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_CREATE_TRIGGER_STATEMENT))
}

fn parse_trigger_event_list(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    let mut first = true;
    while first || p.at(T![or]) {
        let event = p.start();
        if !first {
            p.bump(T![or]);
        }
        if p.at(T![update]) {
            p.bump(T![update]);
            let _ = parse_trigger_update_of_clause(p);
        } else if p.at(T![insert]) || p.at(T![delete]) {
            p.bump_any();
        } else {
            let range = p.cur_range();
            let err = p.err_builder("Expected `insert`, `update` or `delete`", range);
            p.error(err);
            event.abandon(p);
            break;
        }
        event.complete(p, PSQL_TRIGGER_EVENT);
        first = false;
    }
    m.complete(p, PSQL_TRIGGER_EVENT_LIST)
}

/// `UPDATE OF col, col, ...` -- narrows an `UPDATE` trigger event to
/// specific columns.
fn parse_trigger_update_of_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![of]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![of]);
    SqlTriggerUpdateOfColumnList.parse_list(p);
    Present(m.complete(p, PSQL_TRIGGER_UPDATE_OF_CLAUSE))
}

/// Unlike `INSERT`'s parenthesized column list, `UPDATE OF col, col` has no
/// enclosing parens -- it ends at whatever follows (`ON`, another `OR`
/// event, `;`, EOF), not at a `)`.
struct SqlTriggerUpdateOfColumnList;

impl ParseSeparatedList for SqlTriggerUpdateOfColumnList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_COLUMN_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![on]) || p.at(T![or]) || p.at(T![;])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![on], T![or], T![;]]),
            expected_identifier,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

/// `true` if the parser is at an `ident` spelled `old`/`new` (case
/// insensitive) -- unlike `~name~`, real Postgres never reserves these as
/// keywords, so they're recognized by text, not by a dedicated token kind,
/// keeping them usable as ordinary identifiers everywhere else (e.g.
/// `old.some_column` in a `WHEN (...)` condition or trigger body).
fn is_at_old_or_new(p: &mut SqlParser) -> bool {
    if !p.at(T![ident]) {
        return false;
    }
    let text = p.text(p.cur_range());
    text.eq_ignore_ascii_case("old") || text.eq_ignore_ascii_case("new")
}

fn parse_trigger_referencing_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![referencing]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![referencing]);

    let items = p.start();
    while is_at_old_or_new(p) {
        let item = p.start();
        p.bump(T![ident]);
        p.expect(T![table]);
        p.expect(T![as]);
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        item.complete(p, PSQL_TRIGGER_REFERENCING_ITEM);
    }
    items.complete(p, PSQL_TRIGGER_REFERENCING_ITEM_LIST);

    Present(m.complete(p, PSQL_TRIGGER_REFERENCING_CLAUSE))
}

fn parse_trigger_for_each_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![for]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![for]);
    p.expect(T![each]);
    if p.at(T![row]) || p.at(T![statement]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `row` or `statement` after `for each`", range);
        p.error(err);
    }
    Present(m.complete(p, PSQL_TRIGGER_FOR_EACH_CLAUSE))
}

/// `WHEN (condition)` -- restricts a row-level trigger to fire only when
/// `condition` holds.
fn parse_trigger_when_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![when]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![when]);
    p.expect(T!['(']);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_TRIGGER_WHEN_CLAUSE))
}

/// The `name(args)` called by `EXECUTE FUNCTION`/`EXECUTE PROCEDURE` --
/// structurally identical to an ordinary function call.
fn parse_trigger_function(p: &mut SqlParser) -> ParsedSyntax {
    if is_at_tilde_name_start(p) {
        return parse_call_expression(p, 0);
    }
    if !p.at(T![ident]) {
        return Absent;
    }
    let segment_count = count_dotted_name_segments(p).min(3);
    parse_call_expression(p, segment_count)
}

/// `DROP POLICY [IF EXISTS] name ON table [;]` -- Postgres-only, mirrors
/// [parse_create_policy_statement]. Only ever reached once
/// [crate::syntax_rules::ddl::parse_drop_statement]'s own `nth_at(1,
/// policy)` check has already confirmed the shape, so it parses
/// unconditionally rather than re-checking its own "is at" gate.
pub(crate) fn parse_drop_policy_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_drop_policy_statement_body,
        |p, marker| postgres_only_syntax_error(p, "`DROP POLICY`", marker.range(p)),
    )
}

fn parse_drop_policy_statement_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![policy]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    parse_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T![on]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    p.eat(T![;]);

    Present(m.complete(p, PSQL_DROP_POLICY_STATEMENT))
}

/// `DROP TRIGGER [IF EXISTS] name ON table [CASCADE|RESTRICT] [;]` --
/// Postgres-only, mirrors [parse_create_trigger_statement]. Only ever
/// reached once [crate::syntax_rules::ddl::parse_drop_statement]'s own
/// `nth_at(1, trigger)` check has already confirmed the shape, so it
/// parses unconditionally rather than re-checking its own "is at" gate.
pub(crate) fn parse_drop_trigger_statement(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_drop_trigger_statement_body,
        |p, marker| postgres_only_syntax_error(p, "`DROP TRIGGER`", marker.range(p)),
    )
}

fn parse_drop_trigger_statement_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![trigger]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }

    parse_any_name(p).or_add_diagnostic(p, expected_identifier);

    p.expect(T![on]);
    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    // The whole statement is already Postgres-gated above, so `CASCADE`/
    // `RESTRICT` here doesn't need its own separate gate the way
    // `DROP TABLE`/`DROP VIEW`/`DROP FUNCTION` do (see
    // [eat_cascade_or_restrict]).
    if p.at(T![cascade]) || p.at(T![restrict]) {
        p.bump_any();
    }

    p.eat(T![;]);

    Present(m.complete(p, PSQL_DROP_TRIGGER_STATEMENT))
}

/// `(type, ...)` after `DROP FUNCTION|PROCEDURE name` -- disambiguates
/// between overloads by parameter type. Postgres-only: T-SQL's `DROP
/// FUNCTION`/`DROP PROCEDURE` take just a bare name, no parenthesized type
/// list.
pub(crate) fn parse_drop_function_parameters(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T!['(']) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        parse_drop_function_parameters_body,
        |p, marker| {
            postgres_only_syntax_error(p, "`DROP FUNCTION`'s parameter type list", marker.range(p))
        },
    )
}

fn parse_drop_function_parameters_body(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T!['(']);
    SqlTypeNameList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_DROP_FUNCTION_PARAMETERS))
}

struct SqlTypeNameList;

impl ParseSeparatedList for SqlTypeNameList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
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
            &ParseRecoveryTokenSet::new(SQL_BOGUS, token_set![T![')']]),
            expected_type_name,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}
