//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
use biome_rowan::AstNode;
use psql_syntax::{
    PsqlSyntaxElement as SyntaxElement, PsqlSyntaxNode as SyntaxNode,
    PsqlSyntaxToken as SyntaxToken, *,
};
pub fn psql_alias(value: PsqlName) -> PsqlAliasBuilder {
    PsqlAliasBuilder {
        value,
        as_token: None,
    }
}
pub struct PsqlAliasBuilder {
    value: PsqlName,
    as_token: Option<SyntaxToken>,
}
impl PsqlAliasBuilder {
    pub fn with_as_token(mut self, as_token: SyntaxToken) -> Self {
        self.as_token = Some(as_token);
        self
    }
    pub fn build(self) -> PsqlAlias {
        PsqlAlias::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_ALIAS,
            [
                self.as_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn psql_between_expression(
    expression: AnyPsqlExpression,
    between_token: SyntaxToken,
    low: AnyPsqlExpression,
    and_token: SyntaxToken,
    high: AnyPsqlExpression,
) -> PsqlBetweenExpressionBuilder {
    PsqlBetweenExpressionBuilder {
        expression,
        between_token,
        low,
        and_token,
        high,
        not_token: None,
    }
}
pub struct PsqlBetweenExpressionBuilder {
    expression: AnyPsqlExpression,
    between_token: SyntaxToken,
    low: AnyPsqlExpression,
    and_token: SyntaxToken,
    high: AnyPsqlExpression,
    not_token: Option<SyntaxToken>,
}
impl PsqlBetweenExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> PsqlBetweenExpression {
        PsqlBetweenExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_BETWEEN_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.between_token)),
                Some(SyntaxElement::Node(self.low.into_syntax())),
                Some(SyntaxElement::Token(self.and_token)),
                Some(SyntaxElement::Node(self.high.into_syntax())),
            ],
        ))
    }
}
pub fn psql_binary_expression(
    left: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    right: AnyPsqlExpression,
) -> PsqlBinaryExpression {
    PsqlBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn psql_boolean_literal_expression(value_token: SyntaxToken) -> PsqlBooleanLiteralExpression {
    PsqlBooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOOLEAN_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_call_expression(
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlCallExpressionBuilder {
    PsqlCallExpressionBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        schema: None,
    }
}
pub struct PsqlCallExpressionBuilder {
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlExpressionList,
    r_paren_token: SyntaxToken,
    schema: Option<PsqlShemaName>,
}
impl PsqlCallExpressionBuilder {
    pub fn with_schema(mut self, schema: PsqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn build(self) -> PsqlCallExpression {
        PsqlCallExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_CALL_EXPRESSION,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn psql_case_else_clause(
    else_token: SyntaxToken,
    result: AnyPsqlExpression,
) -> PsqlCaseElseClause {
    PsqlCaseElseClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_CASE_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(result.into_syntax())),
        ],
    ))
}
pub fn psql_case_expression(
    case_token: SyntaxToken,
    when_clauses: PsqlCaseWhenClauseList,
    end_token: SyntaxToken,
) -> PsqlCaseExpressionBuilder {
    PsqlCaseExpressionBuilder {
        case_token,
        when_clauses,
        end_token,
        expression: None,
        else_clause: None,
    }
}
pub struct PsqlCaseExpressionBuilder {
    case_token: SyntaxToken,
    when_clauses: PsqlCaseWhenClauseList,
    end_token: SyntaxToken,
    expression: Option<AnyPsqlExpression>,
    else_clause: Option<PsqlCaseElseClause>,
}
impl PsqlCaseExpressionBuilder {
    pub fn with_expression(mut self, expression: AnyPsqlExpression) -> Self {
        self.expression = Some(expression);
        self
    }
    pub fn with_else_clause(mut self, else_clause: PsqlCaseElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> PsqlCaseExpression {
        PsqlCaseExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_CASE_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.case_token)),
                self.expression
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.when_clauses.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.end_token)),
            ],
        ))
    }
}
pub fn psql_case_when_clause(
    when_token: SyntaxToken,
    condition: AnyPsqlExpression,
    then_token: SyntaxToken,
    result: AnyPsqlExpression,
) -> PsqlCaseWhenClause {
    PsqlCaseWhenClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_CASE_WHEN_CLAUSE,
        [
            Some(SyntaxElement::Token(when_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(then_token)),
            Some(SyntaxElement::Node(result.into_syntax())),
        ],
    ))
}
pub fn psql_col_reference(name: PsqlName) -> PsqlColReference {
    PsqlColReference::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_COL_REFERENCE,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn psql_data_base_name(name: PsqlName, dot_token: SyntaxToken) -> PsqlDataBaseName {
    PsqlDataBaseName::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_DATA_BASE_NAME,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
        ],
    ))
}
pub fn psql_delete_statement(
    delete_token: SyntaxToken,
    from_token: SyntaxToken,
    table: PsqlTableBinding,
) -> PsqlDeleteStatementBuilder {
    PsqlDeleteStatementBuilder {
        delete_token,
        from_token,
        table,
        using: None,
        where_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlDeleteStatementBuilder {
    delete_token: SyntaxToken,
    from_token: SyntaxToken,
    table: PsqlTableBinding,
    using: Option<PsqlDeleteUsingClause>,
    where_clause: Option<PsqlWhereClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDeleteStatementBuilder {
    pub fn with_using(mut self, using: PsqlDeleteUsingClause) -> Self {
        self.using = Some(using);
        self
    }
    pub fn with_where_clause(mut self, where_clause: PsqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlDeleteStatement {
        PsqlDeleteStatement::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_DELETE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.delete_token)),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.using
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_delete_using_clause(
    using_token: SyntaxToken,
    any_psql_from_expression: AnyPsqlFromExpression,
) -> PsqlDeleteUsingClause {
    PsqlDeleteUsingClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_DELETE_USING_CLAUSE,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Node(any_psql_from_expression.into_syntax())),
        ],
    ))
}
pub fn psql_from_clause(
    from_token: SyntaxToken,
    source: AnyPsqlFromExpression,
    joins: PsqlJoinClauseList,
) -> PsqlFromClause {
    PsqlFromClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_FROM_CLAUSE,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
            Some(SyntaxElement::Node(joins.into_syntax())),
        ],
    ))
}
pub fn psql_function_binding(
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlFunctionBindingBuilder {
    PsqlFunctionBindingBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        schema: None,
        alias: None,
    }
}
pub struct PsqlFunctionBindingBuilder {
    name: PsqlName,
    l_paren_token: SyntaxToken,
    arguments: PsqlExpressionList,
    r_paren_token: SyntaxToken,
    schema: Option<PsqlShemaName>,
    alias: Option<PsqlAlias>,
}
impl PsqlFunctionBindingBuilder {
    pub fn with_schema(mut self, schema: PsqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlFunctionBinding {
        PsqlFunctionBinding::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_FUNCTION_BINDING,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_group_by_clause(
    group_by_token: SyntaxToken,
    items: PsqlGroupByItemList,
) -> PsqlGroupByClause {
    PsqlGroupByClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_GROUP_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(group_by_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_having_clause(
    having_token: SyntaxToken,
    condition: AnyPsqlExpression,
) -> PsqlHavingClause {
    PsqlHavingClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_HAVING_CLAUSE,
        [
            Some(SyntaxElement::Token(having_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn psql_in_expression(
    expression: AnyPsqlExpression,
    in_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    items: PsqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlInExpressionBuilder {
    PsqlInExpressionBuilder {
        expression,
        in_token,
        l_paren_token,
        items,
        r_paren_token,
        not_token: None,
    }
}
pub struct PsqlInExpressionBuilder {
    expression: AnyPsqlExpression,
    in_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    items: PsqlExpressionList,
    r_paren_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl PsqlInExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> PsqlInExpression {
        PsqlInExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_IN_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.in_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.items.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn psql_insert_columns(
    l_paren_token: SyntaxToken,
    items: PsqlInsertColumnList,
    r_paren_token: SyntaxToken,
) -> PsqlInsertColumns {
    PsqlInsertColumns::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_INSERT_COLUMNS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_insert_statement(
    insert_token: SyntaxToken,
    into_token: SyntaxToken,
    table: PsqlTableBinding,
    source: AnyPsqlInsertSource,
) -> PsqlInsertStatementBuilder {
    PsqlInsertStatementBuilder {
        insert_token,
        into_token,
        table,
        source,
        columns: None,
        semicolon_token: None,
    }
}
pub struct PsqlInsertStatementBuilder {
    insert_token: SyntaxToken,
    into_token: SyntaxToken,
    table: PsqlTableBinding,
    source: AnyPsqlInsertSource,
    columns: Option<PsqlInsertColumns>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlInsertStatementBuilder {
    pub fn with_columns(mut self, columns: PsqlInsertColumns) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlInsertStatement {
        PsqlInsertStatement::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_INSERT_STATEMENT,
            [
                Some(SyntaxElement::Token(self.insert_token)),
                Some(SyntaxElement::Token(self.into_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_insert_values(
    values_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    items: PsqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlInsertValues {
    PsqlInsertValues::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_INSERT_VALUES,
        [
            Some(SyntaxElement::Token(values_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_is_null_expression(
    expression: AnyPsqlExpression,
    is_token: SyntaxToken,
    null_token: SyntaxToken,
) -> PsqlIsNullExpressionBuilder {
    PsqlIsNullExpressionBuilder {
        expression,
        is_token,
        null_token,
        not_token: None,
    }
}
pub struct PsqlIsNullExpressionBuilder {
    expression: AnyPsqlExpression,
    is_token: SyntaxToken,
    null_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl PsqlIsNullExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> PsqlIsNullExpression {
        PsqlIsNullExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_IS_NULL_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                Some(SyntaxElement::Token(self.is_token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.null_token)),
            ],
        ))
    }
}
pub fn psql_join_clause(
    join_token: SyntaxToken,
    source: AnyPsqlFromExpression,
    on_token: SyntaxToken,
    condition: AnyPsqlExpression,
) -> PsqlJoinClauseBuilder {
    PsqlJoinClauseBuilder {
        join_token,
        source,
        on_token,
        condition,
        join_type_token: None,
        outer_token: None,
    }
}
pub struct PsqlJoinClauseBuilder {
    join_token: SyntaxToken,
    source: AnyPsqlFromExpression,
    on_token: SyntaxToken,
    condition: AnyPsqlExpression,
    join_type_token: Option<SyntaxToken>,
    outer_token: Option<SyntaxToken>,
}
impl PsqlJoinClauseBuilder {
    pub fn with_join_type_token(mut self, join_type_token: SyntaxToken) -> Self {
        self.join_type_token = Some(join_type_token);
        self
    }
    pub fn with_outer_token(mut self, outer_token: SyntaxToken) -> Self {
        self.outer_token = Some(outer_token);
        self
    }
    pub fn build(self) -> PsqlJoinClause {
        PsqlJoinClause::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_JOIN_CLAUSE,
            [
                self.join_type_token
                    .map(|token| SyntaxElement::Token(token)),
                self.outer_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.join_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Node(self.condition.into_syntax())),
            ],
        ))
    }
}
pub fn psql_like_expression(
    expression: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    pattern: AnyPsqlExpression,
) -> PsqlLikeExpressionBuilder {
    PsqlLikeExpressionBuilder {
        expression,
        operator_token_token,
        pattern,
        not_token: None,
    }
}
pub struct PsqlLikeExpressionBuilder {
    expression: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    pattern: AnyPsqlExpression,
    not_token: Option<SyntaxToken>,
}
impl PsqlLikeExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> PsqlLikeExpression {
        PsqlLikeExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_LIKE_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.operator_token_token)),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
            ],
        ))
    }
}
pub fn psql_limit_clause(
    limit_token: SyntaxToken,
    limit_count: PsqlNumberLiteralExpression,
) -> PsqlLimitClause {
    PsqlLimitClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_LIMIT_CLAUSE,
        [
            Some(SyntaxElement::Token(limit_token)),
            Some(SyntaxElement::Node(limit_count.into_syntax())),
        ],
    ))
}
pub fn psql_logical_expression(
    left: AnyPsqlExpression,
    operator_token_token: SyntaxToken,
    right: AnyPsqlExpression,
) -> PsqlLogicalExpression {
    PsqlLogicalExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn psql_name(value_token: SyntaxToken) -> PsqlName {
    PsqlName::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_null_literal_expression(value_token: SyntaxToken) -> PsqlNullLiteralExpression {
    PsqlNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_number_literal_expression(value_token: SyntaxToken) -> PsqlNumberLiteralExpression {
    PsqlNumberLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_NUMBER_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_offset_clause(
    offset_token: SyntaxToken,
    start: PsqlNumberLiteralExpression,
) -> PsqlOffsetClause {
    PsqlOffsetClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_OFFSET_CLAUSE,
        [
            Some(SyntaxElement::Token(offset_token)),
            Some(SyntaxElement::Node(start.into_syntax())),
        ],
    ))
}
pub fn psql_order_by_clause(
    order_by_token: SyntaxToken,
    items: PsqlOrderByExpressionList,
) -> PsqlOrderByClause {
    PsqlOrderByClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ORDER_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(order_by_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_order_by_expression(item: AnyPsqlExpression) -> PsqlOrderByExpressionBuilder {
    PsqlOrderByExpressionBuilder {
        item,
        order_token: None,
    }
}
pub struct PsqlOrderByExpressionBuilder {
    item: AnyPsqlExpression,
    order_token: Option<SyntaxToken>,
}
impl PsqlOrderByExpressionBuilder {
    pub fn with_order_token(mut self, order_token: SyntaxToken) -> Self {
        self.order_token = Some(order_token);
        self
    }
    pub fn build(self) -> PsqlOrderByExpression {
        PsqlOrderByExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.item.into_syntax())),
                self.order_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: AnyPsqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlParenthesizedExpression {
    PsqlParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_root(stmt: PsqlStatementList, eof_token: SyntaxToken) -> PsqlRoot {
    PsqlRoot::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ROOT,
        [
            Some(SyntaxElement::Node(stmt.into_syntax())),
            Some(SyntaxElement::Token(eof_token)),
        ],
    ))
}
pub fn psql_select_clause(select_token: SyntaxToken, list: PsqlSelectItemList) -> PsqlSelectClause {
    PsqlSelectClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SELECT_CLAUSE,
        [
            Some(SyntaxElement::Token(select_token)),
            Some(SyntaxElement::Node(list.into_syntax())),
        ],
    ))
}
pub fn psql_select_expression(expr: AnyPsqlExpression) -> PsqlSelectExpressionBuilder {
    PsqlSelectExpressionBuilder { expr, alias: None }
}
pub struct PsqlSelectExpressionBuilder {
    expr: AnyPsqlExpression,
    alias: Option<PsqlAlias>,
}
impl PsqlSelectExpressionBuilder {
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlSelectExpression {
        PsqlSelectExpression::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_SELECT_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expr.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_select_statement(select_clause: PsqlSelectClause) -> PsqlSelectStatementBuilder {
    PsqlSelectStatementBuilder {
        select_clause,
        from_clause: None,
        where_clause: None,
        group_by_clause: None,
        having_clause: None,
        order_by_clause: None,
        limit_clause: None,
        offset_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlSelectStatementBuilder {
    select_clause: PsqlSelectClause,
    from_clause: Option<PsqlFromClause>,
    where_clause: Option<PsqlWhereClause>,
    group_by_clause: Option<PsqlGroupByClause>,
    having_clause: Option<PsqlHavingClause>,
    order_by_clause: Option<PsqlOrderByClause>,
    limit_clause: Option<PsqlLimitClause>,
    offset_clause: Option<PsqlOffsetClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlSelectStatementBuilder {
    pub fn with_from_clause(mut self, from_clause: PsqlFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: PsqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_group_by_clause(mut self, group_by_clause: PsqlGroupByClause) -> Self {
        self.group_by_clause = Some(group_by_clause);
        self
    }
    pub fn with_having_clause(mut self, having_clause: PsqlHavingClause) -> Self {
        self.having_clause = Some(having_clause);
        self
    }
    pub fn with_order_by_clause(mut self, order_by_clause: PsqlOrderByClause) -> Self {
        self.order_by_clause = Some(order_by_clause);
        self
    }
    pub fn with_limit_clause(mut self, limit_clause: PsqlLimitClause) -> Self {
        self.limit_clause = Some(limit_clause);
        self
    }
    pub fn with_offset_clause(mut self, offset_clause: PsqlOffsetClause) -> Self {
        self.offset_clause = Some(offset_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlSelectStatement {
        PsqlSelectStatement::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_SELECT_STATEMENT,
            [
                Some(SyntaxElement::Node(self.select_clause.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.group_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.having_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.order_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.limit_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.offset_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_set_clause(set_token: SyntaxToken, items: PsqlSetItemList) -> PsqlSetClause {
    PsqlSetClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SET_CLAUSE,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_set_item(
    column: PsqlName,
    eq_token: SyntaxToken,
    expr: AnyPsqlExpression,
) -> PsqlSetItem {
    PsqlSetItem::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SET_ITEM,
        [
            Some(SyntaxElement::Node(column.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(expr.into_syntax())),
        ],
    ))
}
pub fn psql_shema_name(name: PsqlName, dot_token: SyntaxToken) -> PsqlShemaNameBuilder {
    PsqlShemaNameBuilder {
        name,
        dot_token,
        base: None,
    }
}
pub struct PsqlShemaNameBuilder {
    name: PsqlName,
    dot_token: SyntaxToken,
    base: Option<PsqlDataBaseName>,
}
impl PsqlShemaNameBuilder {
    pub fn with_base(mut self, base: PsqlDataBaseName) -> Self {
        self.base = Some(base);
        self
    }
    pub fn build(self) -> PsqlShemaName {
        PsqlShemaName::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_SHEMA_NAME,
            [
                self.base
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.dot_token)),
            ],
        ))
    }
}
pub fn psql_star(value_token: SyntaxToken) -> PsqlStar {
    PsqlStar::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_string_literal_expression(value_token: SyntaxToken) -> PsqlStringLiteralExpression {
    PsqlStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_table_binding(table: PsqlTableName) -> PsqlTableBindingBuilder {
    PsqlTableBindingBuilder { table, alias: None }
}
pub struct PsqlTableBindingBuilder {
    table: PsqlTableName,
    alias: Option<PsqlAlias>,
}
impl PsqlTableBindingBuilder {
    pub fn with_alias(mut self, alias: PsqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> PsqlTableBinding {
        PsqlTableBinding::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_TABLE_BINDING,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_table_col_reference(
    table: PsqlTableName,
    dot_token: SyntaxToken,
    name: PsqlName,
) -> PsqlTableColReference {
    PsqlTableColReference::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_TABLE_COL_REFERENCE,
        [
            Some(SyntaxElement::Node(table.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn psql_table_name(name: PsqlName) -> PsqlTableNameBuilder {
    PsqlTableNameBuilder { name, schema: None }
}
pub struct PsqlTableNameBuilder {
    name: PsqlName,
    schema: Option<PsqlShemaName>,
}
impl PsqlTableNameBuilder {
    pub fn with_schema(mut self, schema: PsqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn build(self) -> PsqlTableName {
        PsqlTableName::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_TABLE_NAME,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn psql_unary_expression(
    operator_token_token: SyntaxToken,
    expression: AnyPsqlExpression,
) -> PsqlUnaryExpression {
    PsqlUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn psql_update_statement(
    update_token: SyntaxToken,
    table: PsqlTableBinding,
    set_clause: PsqlSetClause,
) -> PsqlUpdateStatementBuilder {
    PsqlUpdateStatementBuilder {
        update_token,
        table,
        set_clause,
        where_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlUpdateStatementBuilder {
    update_token: SyntaxToken,
    table: PsqlTableBinding,
    set_clause: PsqlSetClause,
    where_clause: Option<PsqlWhereClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlUpdateStatementBuilder {
    pub fn with_where_clause(mut self, where_clause: PsqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlUpdateStatement {
        PsqlUpdateStatement::unwrap_cast(SyntaxNode::new_detached(
            PsqlSyntaxKind::PSQL_UPDATE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.update_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                Some(SyntaxElement::Node(self.set_clause.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_where_clause(
    where_token: SyntaxToken,
    condition: AnyPsqlExpression,
) -> PsqlWhereClause {
    PsqlWhereClause::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_WHERE_CLAUSE,
        [
            Some(SyntaxElement::Token(where_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn psql_case_when_clause_list<I>(items: I) -> PsqlCaseWhenClauseList
where
    I: IntoIterator<Item = PsqlCaseWhenClause>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlCaseWhenClauseList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_CASE_WHEN_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_expression_list<I, S>(items: I, separators: S) -> PsqlExpressionList
where
    I: IntoIterator<Item = AnyPsqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlExpressionList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_EXPRESSION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_group_by_item_list<I, S>(items: I, separators: S) -> PsqlGroupByItemList
where
    I: IntoIterator<Item = AnyPsqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlGroupByItemList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_GROUP_BY_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_insert_column_list<I, S>(items: I, separators: S) -> PsqlInsertColumnList
where
    I: IntoIterator<Item = PsqlName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlInsertColumnList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_INSERT_COLUMN_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_join_clause_list<I>(items: I) -> PsqlJoinClauseList
where
    I: IntoIterator<Item = PsqlJoinClause>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlJoinClauseList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_JOIN_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_order_by_expression_list<I, S>(items: I, separators: S) -> PsqlOrderByExpressionList
where
    I: IntoIterator<Item = PsqlOrderByExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlOrderByExpressionList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_select_item_list<I, S>(items: I, separators: S) -> PsqlSelectItemList
where
    I: IntoIterator<Item = AnyPsqlSelectItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlSelectItemList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SELECT_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_set_item_list<I, S>(items: I, separators: S) -> PsqlSetItemList
where
    I: IntoIterator<Item = PsqlSetItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = PsqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlSetItemList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_SET_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_statement_list<I>(items: I) -> PsqlStatementList
where
    I: IntoIterator<Item = AnyPsqlStatement>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlStatementList::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_bogus<I>(slots: I) -> PsqlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogus::unwrap_cast(SyntaxNode::new_detached(PsqlSyntaxKind::PSQL_BOGUS, slots))
}
pub fn psql_bogus_assignment<I>(slots: I) -> PsqlBogusAssignment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusAssignment::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_ASSIGNMENT,
        slots,
    ))
}
pub fn psql_bogus_binding<I>(slots: I) -> PsqlBogusBinding
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusBinding::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_BINDING,
        slots,
    ))
}
pub fn psql_bogus_expression<I>(slots: I) -> PsqlBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn psql_bogus_member<I>(slots: I) -> PsqlBogusMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusMember::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_MEMBER,
        slots,
    ))
}
pub fn psql_bogus_parameter<I>(slots: I) -> PsqlBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn psql_bogus_statement<I>(slots: I) -> PsqlBogusStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlBogusStatement::unwrap_cast(SyntaxNode::new_detached(
        PsqlSyntaxKind::PSQL_BOGUS_STATEMENT,
        slots,
    ))
}
