//! XML functions with special argument syntax: `xmlexists`, `xmltable`.

use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::parse_error::postgres_only_syntax_error;
use crate::syntax_rules::expr::{parse_alias, parse_expression, parse_name, parse_type_name};
use crate::syntax_rules::parse_error::{
    expected_expression, expected_identifier, expected_type_name,
};
use crate::{SqlParser, SqlSyntaxFeature};
use sql_syntax::{SqlSyntaxKind::*, T};

fn is_at_ident_call(p: &mut SqlParser, name: &str) -> bool {
    p.at(T![ident]) && p.text(p.cur_range()).eq_ignore_ascii_case(name) && p.nth_at(1, T!['('])
}

/// `xmlexists(xpath PASSING [BY {REF | VALUE}] xml [BY ...])`
pub(crate) fn parse_xml_exists_expression(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![ident]);
            p.bump(T!['(']);
            parse_expression(p).or_add_diagnostic(p, expected_expression);
            parse_xml_passing_clause(p).or_add_diagnostic(p, expected_passing);
            p.expect(T![')']);
            Present(m.complete(p, PSQL_XML_EXISTS_EXPRESSION))
        },
        |p, marker| postgres_only_syntax_error(p, "`XMLEXISTS`", marker.range(p)),
    )
}

/// `PASSING [BY {REF | VALUE}] xml [BY {REF | VALUE}]`
fn parse_xml_passing_clause(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![passing]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![passing]);
    let _ = parse_xml_by_mechanism(p);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    let _ = parse_xml_by_mechanism(p);
    Present(m.complete(p, PSQL_XML_PASSING_CLAUSE))
}

fn parse_xml_by_mechanism(p: &mut SqlParser) -> ParsedSyntax {
    if !p.at(T![by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![by]);
    if p.at(T![ref]) || p.at(T![value]) {
        p.bump_any();
    } else {
        let range = p.cur_range();
        let err = p.err_builder("Expected `ref` or `value`", range);
        p.error(err);
    }
    Present(m.complete(p, PSQL_XML_BY_MECHANISM))
}

pub(crate) fn is_at_xml_table(p: &mut SqlParser) -> bool {
    is_at_ident_call(p, "xmltable")
}

/// `xmltable([xmlnamespaces(...),] row PASSING xml COLUMNS column, ...) [alias]`
pub(crate) fn parse_xml_table_binding(p: &mut SqlParser) -> ParsedSyntax {
    SqlSyntaxFeature::Postgres.parse_exclusive_syntax(
        p,
        |p| {
            let m = p.start();
            p.bump(T![ident]);
            p.bump(T!['(']);
            if is_at_ident_call(p, "xmlnamespaces") {
                let namespaces = p.start();
                p.bump(T![ident]);
                p.bump(T!['(']);
                let items = p.start();
                loop {
                    let item = p.start();
                    parse_expression(p).or_add_diagnostic(p, expected_expression);
                    p.expect(T![as]);
                    parse_name(p).or_add_diagnostic(p, expected_identifier);
                    item.complete(p, PSQL_XML_NAMESPACE);
                    if !p.eat(T![,]) {
                        break;
                    }
                }
                items.complete(p, PSQL_XML_NAMESPACE_LIST);
                p.expect(T![')']);
                p.expect(T![,]);
                namespaces.complete(p, PSQL_XML_NAMESPACES_CLAUSE);
            }
            parse_expression(p).or_add_diagnostic(p, expected_expression);
            parse_xml_passing_clause(p).or_add_diagnostic(p, expected_passing);
            p.expect(T![columns]);

            let columns = p.start();
            loop {
                parse_xml_table_column(p).or_add_diagnostic(p, expected_identifier);
                if !p.eat(T![,]) {
                    break;
                }
            }
            columns.complete(p, PSQL_XML_TABLE_COLUMN_LIST);

            p.expect(T![')']);
            parse_alias(p);
            Present(m.complete(p, PSQL_XML_TABLE_BINDING))
        },
        |p, marker| postgres_only_syntax_error(p, "`XMLTABLE`", marker.range(p)),
    )
}

/// `name type [PATH expr] [DEFAULT expr] [NOT NULL]` or `name FOR ORDINALITY`
fn parse_xml_table_column(p: &mut SqlParser) -> ParsedSyntax {
    let m = p.start();
    if parse_name(p).is_absent() {
        m.abandon(p);
        return Absent;
    }
    if p.at(T![for]) {
        p.bump(T![for]);
        p.expect(T![ordinality]);
        return Present(m.complete(p, PSQL_XML_TABLE_ORDINALITY_COLUMN));
    }

    parse_type_name(p).or_add_diagnostic(p, expected_type_name);
    if p.at(T![path]) {
        let path = p.start();
        p.bump(T![path]);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        path.complete(p, PSQL_XML_COLUMN_PATH);
    }
    if p.at(T![default]) {
        let default = p.start();
        p.bump(T![default]);
        parse_expression(p).or_add_diagnostic(p, expected_expression);
        default.complete(p, PSQL_XML_COLUMN_DEFAULT);
    }
    if p.at(T![not]) && p.nth_at(1, T![null]) {
        let not_null = p.start();
        p.bump(T![not]);
        p.bump(T![null]);
        not_null.complete(p, SQL_NOT_NULL_CONSTRAINT);
    }
    Present(m.complete(p, PSQL_XML_TABLE_VALUE_COLUMN))
}

fn expected_passing(p: &SqlParser, range: biome_rowan::TextRange) -> ParseDiagnostic {
    p.err_builder("Expected `passing`", range)
}
