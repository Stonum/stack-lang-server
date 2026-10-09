//! `GRANT` / `REVOKE`.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::parse_table_name_for_ddl;
use super::expr::{EXPR_RECOVERY_SET, parse_column_name_list, parse_name};
use super::parse_error::{expected_identifier, expected_table_binding};
use super::postgres::ddl::eat_cascade_or_restrict;
use super::stmt::STMT_RECOVERY_SET;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

const PRIVILEGE_KINDS: TokenSet<SqlSyntaxKind> = token_set![
    T![all],
    T![select],
    T![insert],
    T![update],
    T![delete],
    T![truncate],
    T![references],
    T![trigger],
    T![usage],
    T![execute],
    T![create],
    T![connect],
    T![temporary],
    T![temp],
];

const OBJECT_KINDS: TokenSet<SqlSyntaxKind> = token_set![
    T![table],
    T![sequence],
    T![function],
    T![procedure],
    T![routine],
    T![schema],
    T![database],
    T![language],
];

const ALL_IN_SCHEMA_KINDS: TokenSet<SqlSyntaxKind> = token_set![
    T![tables],
    T![sequences],
    T![functions],
    T![procedures],
    T![routines],
];

/// `GRANT privilege, ... [ON target] TO grantee, ... [WITH GRANT OPTION] [;]`
pub(crate) fn parse_grant_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![grant]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![grant]);

    SqlPrivilegeList.parse_list(p);
    let _ = parse_privilege_target(p);

    p.expect(T![to]);
    SqlGranteeList.parse_list(p);

    if p.at(T![with]) && p.nth_at(1, T![grant]) {
        let option = p.start();
        p.bump(T![with]);
        p.bump(T![grant]);
        p.expect(T![option]);
        option.complete(p, SQL_WITH_GRANT_OPTION);
    }

    p.eat(T![;]);
    Present(m.complete(p, SQL_GRANT_STATEMENT))
}

/// `REVOKE [GRANT OPTION FOR] privilege, ... [ON target] FROM grantee, ...
/// [CASCADE | RESTRICT] [;]`
pub(crate) fn parse_revoke_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![revoke]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![revoke]);

    if p.at(T![grant]) {
        let option_for = p.start();
        p.bump(T![grant]);
        p.expect(T![option]);
        p.expect(T![for]);
        option_for.complete(p, SQL_GRANT_OPTION_FOR);
    }

    SqlPrivilegeList.parse_list(p);
    let _ = parse_privilege_target(p);

    p.expect(T![from]);
    SqlGranteeList.parse_list(p);
    eat_cascade_or_restrict(p);

    p.eat(T![;]);
    Present(m.complete(p, SQL_REVOKE_STATEMENT))
}

/// A privilege keyword (`ALL [PRIVILEGES]`, `SELECT (col, ...)`, ...) or a
/// role name.
fn parse_privilege(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    if p.at_ts(PRIVILEGE_KINDS) || p.at(T![ident]) {
        p.bump_any();
    } else if p.cur().is_non_reserved_keyword() {
        p.bump_remap(T![ident]);
    } else {
        m.abandon(p);
        return Absent;
    }
    p.eat(T![privileges]);
    let _ = parse_column_name_list(p);
    Present(m.complete(p, SQL_PRIVILEGE))
}

fn parse_privilege_target(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![on]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![on]);

    if p.at(T![all]) {
        p.bump(T![all]);
        if p.at_ts(ALL_IN_SCHEMA_KINDS) {
            p.bump_any();
        } else {
            let range = p.cur_range();
            let err = p.err_builder(
                "Expected `tables`, `sequences`, `functions`, `procedures` or `routines` after `all`",
                range,
            );
            p.error(err);
        }
        p.expect(T![in]);
        p.expect(T![schema]);
        SqlPrivilegeObjectList.parse_list(p);
        Present(m.complete(p, SQL_PRIVILEGE_ALL_IN_SCHEMA))
    } else {
        if p.at_ts(OBJECT_KINDS) {
            p.bump_any();
        }
        SqlPrivilegeObjectList.parse_list(p);
        Present(m.complete(p, SQL_PRIVILEGE_OBJECTS))
    }
}

struct SqlPrivilegeList;

impl ParseSeparatedList for SqlPrivilegeList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_PRIVILEGE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_privilege(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;]) || p.at(T![on]) || p.at(T![to]) || p.at(T![from])
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

    fn allow_empty(&self) -> bool {
        false
    }
}

struct SqlPrivilegeObjectList;

impl ParseSeparatedList for SqlPrivilegeObjectList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_TABLE_NAME_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_table_name_for_ddl(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;]) || p.at(T![to]) || p.at(T![from])
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

struct SqlGranteeList;

impl ParseSeparatedList for SqlGranteeList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_GRANTEE_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_name(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at_ts(STMT_RECOVERY_SET) || p.at(T![cascade]) || p.at(T![restrict])
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

    fn allow_empty(&self) -> bool {
        false
    }
}
