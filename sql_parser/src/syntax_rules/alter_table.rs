use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::{parse_column_definition, parse_table_name_for_ddl};
use super::expr::{EXPR_RECOVERY_SET, parse_name};
use super::parse_error::{
    expected_alter_table_action, expected_column_definition, expected_identifier,
    expected_table_binding,
};
use super::postgres::ddl::eat_cascade_or_restrict;
use super::stmt::STMT_RECOVERY_SET;
use crate::SqlParser;
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

/// `ALTER TABLE [IF EXISTS] [ONLY] name action, ... [;]`
pub(crate) fn parse_alter_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![alter]) || !p.nth_at(1, T![table]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![alter]);
    p.bump(T![table]);

    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }
    p.eat(T![only]);

    parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);

    SqlAlterTableActionList.parse_list(p);

    p.eat(T![;]);

    Present(m.complete(p, SQL_ALTER_TABLE_STATEMENT))
}

struct SqlAlterTableActionList;

impl ParseSeparatedList for SqlAlterTableActionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = SQL_ALTER_TABLE_ACTION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        match p.cur() {
            T![enable] | T![disable] => parse_alter_trigger_action(p),
            T![add] => parse_add_column_action(p),
            T![drop] => parse_drop_column_action(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // The next statement may follow without a `;` in mlang command
        // batches; `drop` starts both a statement and an action.
        p.at(EOF) || (p.at_ts(STMT_RECOVERY_SET) && !is_at_drop_action(p))
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(SQL_BOGUS, EXPR_RECOVERY_SET),
            expected_alter_table_action,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_empty(&self) -> bool {
        false
    }
}

/// `drop column ...` / `drop if exists ...` / `drop name`, as opposed to a
/// following `drop table ...` statement.
fn is_at_drop_action(p: &mut SqlParser) -> bool {
    p.at(T![drop]) && (p.nth_at(1, T![column]) || p.nth_at(1, T![if]) || p.nth_at(1, T![ident]))
}

/// `ENABLE | DISABLE TRIGGER name | ALL | USER`
fn parse_alter_trigger_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump_any();
    p.expect(T![trigger]);

    if p.at(T![all]) || p.at(T![user]) {
        let target = p.start();
        p.bump_any();
        target.complete(p, SQL_ALL_TRIGGERS_TARGET);
    } else {
        parse_name(p).or_add_diagnostic(p, expected_identifier);
    }

    Present(m.complete(p, SQL_ALTER_TRIGGER_ACTION))
}

/// `ADD [COLUMN] [IF NOT EXISTS] column definition`
fn parse_add_column_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![add]);
    p.eat(T![column]);
    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![not]);
        p.expect(T![exists]);
    }
    parse_column_definition(p).or_add_diagnostic(p, expected_column_definition);
    Present(m.complete(p, SQL_ADD_COLUMN_ACTION))
}

/// `DROP [COLUMN] [IF EXISTS] name [CASCADE | RESTRICT]`
fn parse_drop_column_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.eat(T![column]);
    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    eat_cascade_or_restrict(p);
    Present(m.complete(p, SQL_DROP_COLUMN_ACTION))
}
