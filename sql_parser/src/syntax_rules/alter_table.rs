use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::ddl::{
    is_at_table_constraint_start, parse_column_definition, parse_table_constraint,
    parse_table_name_for_ddl,
};
use super::expr::{EXPR_RECOVERY_SET, parse_expression, parse_name, parse_type_name};
use super::index::parse_alter_index_statement;
use super::parse_error::{
    expected_alter_column_operation, expected_alter_table_action, expected_column_definition,
    expected_expression, expected_identifier, expected_table_binding, expected_type_name,
};
use super::postgres::ddl::eat_cascade_or_restrict;
use super::postgres::parse_error::postgres_only_syntax_error;
use super::stmt::STMT_RECOVERY_SET;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

/// `ALTER TABLE [IF EXISTS] [ONLY] name action, ... [;]` or `ALTER INDEX ...`
pub(crate) fn parse_alter_statement(p: &mut SqlParser) -> ParsedSyntax {
    if p.at(T![alter]) && p.nth_at(1, T![index]) {
        return parse_alter_index_statement(p);
    }
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
            T![add] => parse_add_action(p),
            T![drop] if p.nth_at(1, T![constraint]) => parse_drop_constraint_action(p),
            T![drop] => parse_drop_column_action(p),
            T![alter] => parse_alter_column_action(p),
            T![rename] => parse_rename_action(p),
            T![set] if p.nth_at(1, T![schema]) => parse_set_schema_action(p),
            _ => Absent,
        }
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // The next statement may follow without a `;` in mlang command
        // batches; `drop`/`alter` start both a statement and an action.
        p.at(EOF) || (p.at_ts(STMT_RECOVERY_SET) && !is_at_column_action(p))
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

/// `drop column ...` / `drop if exists ...` / `drop name` or `alter
/// [column] name ...`, as opposed to a following `drop table ...` /
/// `alter table ...` statement.
fn is_at_column_action(p: &mut SqlParser) -> bool {
    match p.cur() {
        T![drop] => {
            p.nth_at(1, T![column])
                || p.nth_at(1, T![constraint])
                || p.nth_at(1, T![if])
                || p.nth_at(1, T![ident])
        }
        T![alter] => p.nth_at(1, T![column]) || p.nth_at(1, T![ident]),
        _ => false,
    }
}

/// `ALTER [COLUMN] name {[SET DATA] TYPE ... | SET/DROP DEFAULT | SET/DROP NOT NULL}`
fn parse_alter_column_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![alter]);
    p.eat(T![column]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    parse_alter_column_operation(p).or_add_diagnostic(p, expected_alter_column_operation);
    Present(m.complete(p, SQL_ALTER_COLUMN_ACTION))
}

fn parse_alter_column_operation(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    let kind = match (p.cur(), p.nth(1)) {
        (T![type], _) | (T![set], T![data]) => {
            if p.at(T![set]) {
                p.bump(T![set]);
                p.bump(T![data]);
            }
            p.expect(T![type]);
            parse_type_name(p).or_add_diagnostic(p, expected_type_name);
            let _ = parse_alter_column_using(p);
            SQL_ALTER_COLUMN_TYPE
        }
        (T![set], T![default]) => {
            p.bump(T![set]);
            p.bump(T![default]);
            parse_expression(p).or_add_diagnostic(p, expected_expression);
            SQL_SET_COLUMN_DEFAULT
        }
        (T![drop], T![default]) => {
            p.bump(T![drop]);
            p.bump(T![default]);
            SQL_DROP_COLUMN_DEFAULT
        }
        (T![set], T![not]) => {
            p.bump(T![set]);
            p.bump(T![not]);
            p.expect(T![null]);
            SQL_SET_COLUMN_NOT_NULL
        }
        (T![drop], T![not]) => {
            p.bump(T![drop]);
            p.bump(T![not]);
            p.expect(T![null]);
            SQL_DROP_COLUMN_NOT_NULL
        }
        _ => {
            m.abandon(p);
            return Absent;
        }
    };
    Present(m.complete(p, kind))
}

/// `RENAME TO new`, `RENAME CONSTRAINT old TO new` or `RENAME [COLUMN] old TO new`
fn parse_rename_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![rename]);

    if p.eat(T![to]) {
        parse_name(p).or_add_diagnostic(p, expected_identifier);
        return Present(m.complete(p, SQL_RENAME_TABLE_ACTION));
    }

    let kind = if p.eat(T![constraint]) {
        SQL_RENAME_CONSTRAINT_ACTION
    } else {
        p.eat(T![column]);
        SQL_RENAME_COLUMN_ACTION
    };
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    p.expect(T![to]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, kind))
}

/// `SET SCHEMA schema`
fn parse_set_schema_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![set]);
    p.bump(T![schema]);
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    Present(m.complete(p, SQL_SET_SCHEMA_ACTION))
}

/// `USING expr` after `ALTER COLUMN ... TYPE ...`; Postgres-only.
fn parse_alter_column_using(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![using]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![using]);
            parse_expression(p).or_add_diagnostic(p, expected_expression);
            Present(m.complete(p, PSQL_ALTER_COLUMN_USING))
        },
        |p, marker| postgres_only_syntax_error(p, "`USING` in `ALTER COLUMN`", marker.range(p)),
    )
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
/// `ADD <table constraint>` or `ADD [COLUMN] ...`
fn parse_add_action(p: &mut SqlParser) -> ParsedSyntax {
    let is_constraint = p.lookahead(|p| {
        p.bump(T![add]);
        is_at_table_constraint_start(p)
    });
    if !is_constraint {
        return parse_add_column_action(p);
    }

    let m = p.start();
    p.bump(T![add]);
    parse_table_constraint(p).ok();
    Present(m.complete(p, SQL_ADD_CONSTRAINT_ACTION))
}

/// `DROP CONSTRAINT [IF EXISTS] name [CASCADE | RESTRICT]`
fn parse_drop_constraint_action(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    p.bump(T![drop]);
    p.bump(T![constraint]);
    if p.at(T![if]) {
        p.bump(T![if]);
        p.expect(T![exists]);
    }
    parse_name(p).or_add_diagnostic(p, expected_identifier);
    eat_cascade_or_restrict(p);
    Present(m.complete(p, SQL_DROP_CONSTRAINT_ACTION))
}

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
