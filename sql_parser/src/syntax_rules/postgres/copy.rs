//! `COPY` -- Postgres-only.

use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::ddl::parse_table_name_for_ddl;
use crate::syntax_rules::expr::{
    EXPR_RECOVERY_SET, parse_column_name_list, parse_expression, parse_string_literal_expression,
    parse_subquery_expression,
};
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_table_binding,
};
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T};

/// `COPY {table [(col, ...)] | (query)} {FROM | TO} {'file' | PROGRAM 'cmd' |
/// STDIN | STDOUT} [[WITH] (option, ...)] [;]`
pub(crate) fn parse_copy_statement(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![copy]) {
        return Absent;
    }

    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![copy]);

            if p.at(T!['(']) {
                parse_subquery_expression(p).or_add_diagnostic(p, expected_expression);
            } else {
                let source = p.start();
                parse_table_name_for_ddl(p).or_add_diagnostic(p, expected_table_binding);
                let _ = parse_column_name_list(p);
                source.complete(p, PSQL_COPY_TABLE_SOURCE);
            }

            if p.at(T![from]) || p.at(T![to]) {
                p.bump_any();
            } else {
                let range = p.cur_range();
                let err = p.err_builder("Expected `from` or `to`", range);
                p.error(err);
            }

            parse_copy_target(p);
            let _ = parse_copy_options(p);

            p.eat(T![;]);
            Present(m.complete(p, PSQL_COPY_STATEMENT))
        },
        |p, marker| postgres_only_syntax_error(p, "`COPY`", marker.range(p)),
    )
}

fn parse_copy_target(p: &mut SqlParser) {
    if p.at(T![stdin]) || p.at(T![stdout]) {
        let m = p.start();
        p.bump_any();
        m.complete(p, PSQL_COPY_STDIO);
        return;
    }

    if p.at(T![program]) || p.at(SQL_STRING_LITERAL) {
        let m = p.start();
        p.eat(T![program]);
        parse_string_literal_expression(p).or_add_diagnostic(p, expected_expression);
        m.complete(p, PSQL_COPY_FILE);
        return;
    }

    let range = p.cur_range();
    let err = p.err_builder(
        "Expected a file name, `program`, `stdin` or `stdout`",
        range,
    );
    p.error(err);
}

fn parse_copy_options(p: &mut SqlParser) -> ParsedSyntax {
    if !(p.at(T!['(']) || (p.at(T![with]) && p.nth_at(1, T!['(']))) {
        return Absent;
    }

    let m = p.start();
    p.eat(T![with]);
    p.bump(T!['(']);
    PsqlCopyOptionList.parse_list(p);
    p.expect(T![')']);
    Present(m.complete(p, PSQL_COPY_OPTIONS))
}

/// `name [value]` -- the option name may be any word, keywords included
/// (`format`, `null`, `default`).
fn parse_copy_option(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![ident]) && !p.cur().is_keyword() {
        return Absent;
    }

    let m = p.start();
    let name = p.start();
    p.bump_remap(T![ident]);
    name.complete(p, SQL_NAME);

    if p.cur().is_keyword() && (p.nth_at(1, T![,]) || p.nth_at(1, T![')'])) {
        // A bare word value of any kind (`on_error ignore`, `log_verbosity verbose`).
        let value = p.start();
        let name = p.start();
        p.bump_remap(T![ident]);
        name.complete(p, SQL_NAME);
        value.complete(p, SQL_COL_REFERENCE);
    } else if !p.at(T![,]) && !p.at(T![')']) {
        parse_expression(p).or_add_diagnostic(p, expected_expression);
    }
    Present(m.complete(p, PSQL_COPY_OPTION))
}

struct PsqlCopyOptionList;

impl ParseSeparatedList for PsqlCopyOptionList {
    type Kind = SqlSyntaxKind;
    type Parser<'source> = SqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_COPY_OPTION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_copy_option(p)
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
