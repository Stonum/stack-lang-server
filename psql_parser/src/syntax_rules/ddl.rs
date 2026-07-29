use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{parse_any_name, parse_type_name};
use super::parse_error::*;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

/// `DROP FUNCTION|PROCEDURE [IF EXISTS] name [(type, ...)] [CASCADE|RESTRICT] [;]`
/// -- Postgres DDL for removing a stored function/procedure. `GO` (a T-SQL
/// batch separator some client scripts still carry over from MSSQL) isn't
/// part of this grammar at all; it already recovers fine as ordinary
/// bogus-statement content between two real statements.
pub(crate) fn parse_drop_function_statement(p: &mut PsqlParser) -> ParsedSyntax {
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
