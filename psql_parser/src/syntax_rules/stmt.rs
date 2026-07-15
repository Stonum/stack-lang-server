use biome_parser::ParserProgress;
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::parse_recovery::{ParseRecoveryTokenSet, RecoveryResult};
use biome_parser::parsed_syntax::ParsedSyntax::{Absent, Present};
use biome_parser::prelude::*;

use super::expr::{
    EXPR_RECOVERY_SET, parse_alias, parse_expression, parse_number_literal_expression,
};
use super::from::parse_from_clause;
use super::parse_error::*;
use crate::PsqlParser;
use psql_syntax::{PsqlSyntaxKind::*, T, *};

pub const STMT_RECOVERY_SET: TokenSet<PsqlSyntaxKind> = token_set![T![;]];

pub(crate) fn parse_statements(p: &mut PsqlParser, statement_list: Marker) {
    let mut progress = ParserProgress::default();

    while !p.at(EOF) {
        progress.assert_progressing(p);

        if parse_statement(p, StatementContext::StatementList)
            .or_recover_with_token_set(
                p,
                &ParseRecoveryTokenSet::new(PSQL_BOGUS_STATEMENT, STMT_RECOVERY_SET),
                expected_statement,
            )
            .is_err()
        {
            break;
        }
    }

    statement_list.complete(p, PSQL_STATEMENT_LIST);
}

pub(crate) fn parse_statement(p: &mut PsqlParser, _context: StatementContext) -> ParsedSyntax {
    match p.cur() {
        T![select] => parse_select_statement(p),
        // T![:] => parse_annotation_statement(p, context),
        // T![;] => parse_empty_statement(p),
        // T!['{'] => parse_block_stmt(p),
        // T![if] => parse_if_statement(p),
        // T![while] => parse_while_statement(p),

        // T![var] => parse_variable_statement(p),
        // T![for] => parse_for_statement(p),
        // T![forall] => parse_forall_statement(p),

        // T![switch] => parse_switch_statement(p),
        // T![try] => parse_try_statement(p),
        // T![return] => parse_return_statement(p),
        // T![break] => parse_break_statement(p),
        // T![continue] => parse_continue_statement(p),
        // T![throw] => parse_throw_statement(p),
        // T![debug] => parse_debugger_statement(p),
        // // function
        // T![inline] | T![function] => parse_function_declaration(p, context, None),

        // // class
        // T![class] => parse_class_declaration(p, context, None),

        // T![.] => parse_global_statement(p),

        // _ if is_at_expression(p) => parse_expression_statement(p),
        _ => Absent,
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum StatementContext {
    StatementList,
}

impl StatementContext {
    pub(crate) fn is_single_statement(&self) -> bool {
        !matches!(self, StatementContext::StatementList)
    }
}

fn parse_select_statement(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![select]) {
        return Absent;
    }

    let select_stmt = p.start();

    let select_clause = p.start();
    p.expect(T![select]);
    PsqlSelectItemList.parse_list(p);
    select_clause.complete(p, PSQL_SELECT_CLAUSE);

    let _ = parse_from_clause(p);
    let _ = parse_where_clause(p);
    let _ = parse_group_by_clause(p);
    let _ = parse_having_clause(p);
    let _ = parse_order_by_clause(p);
    let _ = parse_limit_clause(p);
    let _ = parse_offset_clause(p);
    p.eat(T![;]);

    Present(select_stmt.complete(p, PSQL_SELECT_STATEMENT))
}

fn parse_where_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![where]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![where]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_WHERE_CLAUSE))
}

fn parse_group_by_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![group_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![group_by]);
    PsqlGroupByItemList.parse_list(p);
    Present(m.complete(p, PSQL_GROUP_BY_CLAUSE))
}

struct PsqlGroupByItemList;

impl ParseSeparatedList for PsqlGroupByItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_GROUP_BY_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![;])
            || p.at(T![having])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

fn parse_having_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![having]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![having]);
    parse_expression(p).or_add_diagnostic(p, expected_expression);
    Present(m.complete(p, PSQL_HAVING_CLAUSE))
}

fn parse_order_by_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![order_by]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![order_by]);
    PsqlOrderByExpressionList.parse_list(p);
    Present(m.complete(p, PSQL_ORDER_BY_CLAUSE))
}

struct PsqlOrderByExpressionList;

impl ParseSeparatedList for PsqlOrderByExpressionList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_ORDER_BY_EXPRESSION_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_order_by_expression(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF) || p.at(T![;]) || p.at(T![limit]) || p.at(T![offset])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_order_by_expression(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    if parse_expression(p).is_present() && (p.at(T![asc]) || p.at(T![desc])) {
        p.bump_any();
    }
    Present(m.complete(p, PSQL_ORDER_BY_EXPRESSION))
}

fn parse_limit_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![limit]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![limit]);
    parse_number_literal_expression(p).or_add_diagnostic(p, expected_number_literal);
    Present(m.complete(p, PSQL_LIMIT_CLAUSE))
}

fn parse_offset_clause(p: &mut PsqlParser) -> ParsedSyntax {
    if !p.at(T![offset]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![offset]);
    parse_number_literal_expression(p).or_add_diagnostic(p, expected_number_literal);
    Present(m.complete(p, PSQL_OFFSET_CLAUSE))
}

struct PsqlSelectItemList;

impl ParseSeparatedList for PsqlSelectItemList {
    type Kind = PsqlSyntaxKind;
    type Parser<'source> = PsqlParser<'source>;
    const LIST_KIND: Self::Kind = PSQL_SELECT_ITEM_LIST;

    fn parse_element(&mut self, p: &mut Self::Parser<'_>) -> ParsedSyntax {
        parse_select_item(p)
    }

    fn is_at_list_end(&self, p: &mut Self::Parser<'_>) -> bool {
        p.at(EOF)
            || p.at(T![from])
            || p.at(T![where])
            || p.at(T![group_by])
            || p.at(T![having])
            || p.at(T![order_by])
            || p.at(T![limit])
            || p.at(T![offset])
    }

    fn recover(
        &mut self,
        p: &mut Self::Parser<'_>,
        parsed_element: ParsedSyntax,
    ) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(PSQL_BOGUS_EXPRESSION, EXPR_RECOVERY_SET),
            expected_expression,
        )
    }

    fn separating_element_kind(&mut self) -> Self::Kind {
        T![,]
    }
}

fn parse_select_item(p: &mut PsqlParser) -> ParsedSyntax {
    let m = p.start();
    if p.at(T![*]) {
        p.bump(T![*]);
        Present(m.complete(p, PSQL_STAR))
    } else {
        if parse_expression(p).is_present() {
            parse_alias(p);
        }
        Present(m.complete(p, PSQL_SELECT_EXPRESSION))
    }
}
