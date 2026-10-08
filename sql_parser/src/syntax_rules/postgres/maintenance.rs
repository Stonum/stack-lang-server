//! `VACUUM` / `ANALYZE` -- Postgres-only maintenance statements.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::parse_table_name_for_ddl;
use crate::syntax_rules::expr::{EXPR_RECOVERY_SET, parse_column_name_list};
use crate::syntax_rules::parse_error::expected_table_binding;
use crate::syntax_rules::stmt::STMT_RECOVERY_SET;
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

/// `VACUUM [FULL] [FREEZE] [VERBOSE] [ANALYZE] [table [(col, ...)], ...] [;]`
pub(crate) fn parse_vacuum_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![vacuum]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![vacuum]);
            p.eat(T![full]);
            p.eat(T![freeze]);
            p.eat(T![verbose]);
            if is_at_analyze(p) {
                p.bump_any();
            }
            PsqlTableAndColumnsList.parse_list(p);
            p.eat(T![;]);
            Present(m.complete(p, PSQL_VACUUM_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`VACUUM`", marker.range(p)),
    )
}

/// `ANALYZE [VERBOSE] [table [(col, ...)], ...] [;]` (also spelled `ANALYSE`)
pub(crate) fn parse_analyze_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !is_at_analyze(p) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump_any();
            p.eat(T![verbose]);
            PsqlTableAndColumnsList.parse_list(p);
            p.eat(T![;]);
            Present(m.complete(p, PSQL_ANALYZE_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`ANALYZE`", marker.range(p)),
    )
}

fn is_at_analyze(p: &mut SqlParser) -> bool {
    p.at(T![analyze]) || p.at(T![analyse])
}

/// `table [(col, ...)], ...` -- may be empty (the whole database).
struct PsqlTableAndColumnsList;

impl ParseSeparatedList for PsqlTableAndColumnsList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_TABLE_AND_COLUMNS_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        let table = parse_table_name_for_ddl(p);
        if table.is_absent() {
            return Absent;
        }

        let m = table.precede(p);
        let _ = parse_column_name_list(p);
        Present(m.complete(p, PSQL_TABLE_AND_COLUMNS))
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        // The next statement may follow without a `;` in mlang command batches.
        p.at(EOF) || p.at_ts(STMT_RECOVERY_SET)
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
