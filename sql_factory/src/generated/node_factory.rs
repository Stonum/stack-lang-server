//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use sql_syntax::{
    SqlSyntaxElement as SyntaxElement, SqlSyntaxNode as SyntaxNode, SqlSyntaxToken as SyntaxToken,
    *,
};
pub fn psql_array_expression(
    array_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    items: SqlExpressionList,
    r_brack_token: SyntaxToken,
) -> PsqlArrayExpression {
    PsqlArrayExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_ARRAY_EXPRESSION,
        [
            Some(SyntaxElement::Token(array_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn psql_array_subscript_expression(
    expression: AnySqlExpression,
    l_brack_token: SyntaxToken,
    index: AnySqlExpression,
    r_brack_token: SyntaxToken,
) -> PsqlArraySubscriptExpression {
    PsqlArraySubscriptExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_ARRAY_SUBSCRIPT_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(index.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn psql_cast_expression(
    expression: AnySqlExpression,
    double_colon_token: SyntaxToken,
    ty: SqlTypeName,
) -> PsqlCastExpression {
    PsqlCastExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_CAST_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(double_colon_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn psql_create_function_statement(
    create_token: SyntaxToken,
    kind_token: SyntaxToken,
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    parameters: PsqlFunctionParameterList,
    r_paren_token: SyntaxToken,
    leading_options: PsqlFunctionOptionList,
    as_token: SyntaxToken,
    body: SqlStringLiteralExpression,
    trailing_options: PsqlFunctionOptionList,
) -> PsqlCreateFunctionStatementBuilder {
    PsqlCreateFunctionStatementBuilder {
        create_token,
        kind_token,
        name,
        l_paren_token,
        parameters,
        r_paren_token,
        leading_options,
        as_token,
        body,
        trailing_options,
        or_token: None,
        replace_token: None,
        returns_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreateFunctionStatementBuilder {
    create_token: SyntaxToken,
    kind_token: SyntaxToken,
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    parameters: PsqlFunctionParameterList,
    r_paren_token: SyntaxToken,
    leading_options: PsqlFunctionOptionList,
    as_token: SyntaxToken,
    body: SqlStringLiteralExpression,
    trailing_options: PsqlFunctionOptionList,
    or_token: Option<SyntaxToken>,
    replace_token: Option<SyntaxToken>,
    returns_clause: Option<PsqlReturnsClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateFunctionStatementBuilder {
    pub fn with_or_token(mut self, or_token: SyntaxToken) -> Self {
        self.or_token = Some(or_token);
        self
    }
    pub fn with_replace_token(mut self, replace_token: SyntaxToken) -> Self {
        self.replace_token = Some(replace_token);
        self
    }
    pub fn with_returns_clause(mut self, returns_clause: PsqlReturnsClause) -> Self {
        self.returns_clause = Some(returns_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreateFunctionStatement {
        PsqlCreateFunctionStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_FUNCTION_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                self.or_token.map(|token| SyntaxElement::Token(token)),
                self.replace_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.kind_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.parameters.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.returns_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.leading_options.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                Some(SyntaxElement::Node(self.trailing_options.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_create_policy_statement(
    create_token: SyntaxToken,
    policy_token: SyntaxToken,
    name: SqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
) -> PsqlCreatePolicyStatementBuilder {
    PsqlCreatePolicyStatementBuilder {
        create_token,
        policy_token,
        name,
        on_token,
        table,
        for_clause: None,
        using_clause: None,
        with_check_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreatePolicyStatementBuilder {
    create_token: SyntaxToken,
    policy_token: SyntaxToken,
    name: SqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
    for_clause: Option<PsqlPolicyForClause>,
    using_clause: Option<PsqlPolicyUsingClause>,
    with_check_clause: Option<PsqlPolicyWithCheckClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreatePolicyStatementBuilder {
    pub fn with_for_clause(mut self, for_clause: PsqlPolicyForClause) -> Self {
        self.for_clause = Some(for_clause);
        self
    }
    pub fn with_using_clause(mut self, using_clause: PsqlPolicyUsingClause) -> Self {
        self.using_clause = Some(using_clause);
        self
    }
    pub fn with_with_check_clause(mut self, with_check_clause: PsqlPolicyWithCheckClause) -> Self {
        self.with_check_clause = Some(with_check_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreatePolicyStatement {
        PsqlCreatePolicyStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_POLICY_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                Some(SyntaxElement::Token(self.policy_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.for_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.using_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.with_check_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_create_trigger_statement(
    create_token: SyntaxToken,
    trigger_token: SyntaxToken,
    name: AnySqlName,
    timing_token: SyntaxToken,
    events: PsqlTriggerEventList,
    on_token: SyntaxToken,
    table: SqlTableName,
    execute_token: SyntaxToken,
    function_kind_token: SyntaxToken,
    function: SqlCallExpression,
) -> PsqlCreateTriggerStatementBuilder {
    PsqlCreateTriggerStatementBuilder {
        create_token,
        trigger_token,
        name,
        timing_token,
        events,
        on_token,
        table,
        execute_token,
        function_kind_token,
        function,
        referencing_clause: None,
        for_each_clause: None,
        when_clause: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreateTriggerStatementBuilder {
    create_token: SyntaxToken,
    trigger_token: SyntaxToken,
    name: AnySqlName,
    timing_token: SyntaxToken,
    events: PsqlTriggerEventList,
    on_token: SyntaxToken,
    table: SqlTableName,
    execute_token: SyntaxToken,
    function_kind_token: SyntaxToken,
    function: SqlCallExpression,
    referencing_clause: Option<PsqlTriggerReferencingClause>,
    for_each_clause: Option<PsqlTriggerForEachClause>,
    when_clause: Option<PsqlTriggerWhenClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateTriggerStatementBuilder {
    pub fn with_referencing_clause(
        mut self,
        referencing_clause: PsqlTriggerReferencingClause,
    ) -> Self {
        self.referencing_clause = Some(referencing_clause);
        self
    }
    pub fn with_for_each_clause(mut self, for_each_clause: PsqlTriggerForEachClause) -> Self {
        self.for_each_clause = Some(for_each_clause);
        self
    }
    pub fn with_when_clause(mut self, when_clause: PsqlTriggerWhenClause) -> Self {
        self.when_clause = Some(when_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreateTriggerStatement {
        PsqlCreateTriggerStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_TRIGGER_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                Some(SyntaxElement::Token(self.trigger_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.timing_token)),
                Some(SyntaxElement::Node(self.events.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.referencing_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.for_each_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.when_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.execute_token)),
                Some(SyntaxElement::Token(self.function_kind_token)),
                Some(SyntaxElement::Node(self.function.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_cte_materialized_hint(
    materialized_token: SyntaxToken,
) -> PsqlCteMaterializedHintBuilder {
    PsqlCteMaterializedHintBuilder {
        materialized_token,
        not_token: None,
    }
}
pub struct PsqlCteMaterializedHintBuilder {
    materialized_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl PsqlCteMaterializedHintBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> PsqlCteMaterializedHint {
        PsqlCteMaterializedHint::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CTE_MATERIALIZED_HINT,
            [
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.materialized_token)),
            ],
        ))
    }
}
pub fn psql_delete_using_clause(
    using_token: SyntaxToken,
    items: SqlFromItemList,
) -> PsqlDeleteUsingClause {
    PsqlDeleteUsingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_DELETE_USING_CLAUSE,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_distinct_on_clause(
    on_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    items: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlDistinctOnClause {
    PsqlDistinctOnClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_DISTINCT_ON_CLAUSE,
        [
            Some(SyntaxElement::Token(on_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_do_nothing_clause(
    do_token: SyntaxToken,
    nothing_token: SyntaxToken,
) -> PsqlDoNothingClause {
    PsqlDoNothingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_DO_NOTHING_CLAUSE,
        [
            Some(SyntaxElement::Token(do_token)),
            Some(SyntaxElement::Token(nothing_token)),
        ],
    ))
}
pub fn psql_do_update_clause(
    do_token: SyntaxToken,
    update_token: SyntaxToken,
    set_clause: SqlSetClause,
) -> PsqlDoUpdateClauseBuilder {
    PsqlDoUpdateClauseBuilder {
        do_token,
        update_token,
        set_clause,
        where_clause: None,
    }
}
pub struct PsqlDoUpdateClauseBuilder {
    do_token: SyntaxToken,
    update_token: SyntaxToken,
    set_clause: SqlSetClause,
    where_clause: Option<SqlWhereClause>,
}
impl PsqlDoUpdateClauseBuilder {
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn build(self) -> PsqlDoUpdateClause {
        PsqlDoUpdateClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DO_UPDATE_CLAUSE,
            [
                Some(SyntaxElement::Token(self.do_token)),
                Some(SyntaxElement::Token(self.update_token)),
                Some(SyntaxElement::Node(self.set_clause.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_drop_function_parameters(
    l_paren_token: SyntaxToken,
    items: PsqlTypeNameList,
    r_paren_token: SyntaxToken,
) -> PsqlDropFunctionParameters {
    PsqlDropFunctionParameters::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_DROP_FUNCTION_PARAMETERS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_drop_policy_statement(
    drop_token: SyntaxToken,
    policy_token: SyntaxToken,
    name: SqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
) -> PsqlDropPolicyStatementBuilder {
    PsqlDropPolicyStatementBuilder {
        drop_token,
        policy_token,
        name,
        on_token,
        table,
        if_token: None,
        exists_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlDropPolicyStatementBuilder {
    drop_token: SyntaxToken,
    policy_token: SyntaxToken,
    name: SqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDropPolicyStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlDropPolicyStatement {
        PsqlDropPolicyStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DROP_POLICY_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.policy_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_drop_trigger_statement(
    drop_token: SyntaxToken,
    trigger_token: SyntaxToken,
    name: AnySqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
) -> PsqlDropTriggerStatementBuilder {
    PsqlDropTriggerStatementBuilder {
        drop_token,
        trigger_token,
        name,
        on_token,
        table,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlDropTriggerStatementBuilder {
    drop_token: SyntaxToken,
    trigger_token: SyntaxToken,
    name: AnySqlName,
    on_token: SyntaxToken,
    table: SqlTableName,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDropTriggerStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_drop_behavior_token(mut self, drop_behavior_token: SyntaxToken) -> Self {
        self.drop_behavior_token = Some(drop_behavior_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlDropTriggerStatement {
        PsqlDropTriggerStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DROP_TRIGGER_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.trigger_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_filter_clause(
    filter_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    where_token: SyntaxToken,
    condition: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlFilterClause {
    PsqlFilterClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_FILTER_CLAUSE,
        [
            Some(SyntaxElement::Token(filter_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Token(where_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_function_parameter(ty: SqlTypeName) -> PsqlFunctionParameterBuilder {
    PsqlFunctionParameterBuilder {
        ty,
        mode_token: None,
        name: None,
        default: None,
    }
}
pub struct PsqlFunctionParameterBuilder {
    ty: SqlTypeName,
    mode_token: Option<SyntaxToken>,
    name: Option<SqlName>,
    default: Option<PsqlParameterDefault>,
}
impl PsqlFunctionParameterBuilder {
    pub fn with_mode_token(mut self, mode_token: SyntaxToken) -> Self {
        self.mode_token = Some(mode_token);
        self
    }
    pub fn with_name(mut self, name: SqlName) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_default(mut self, default: PsqlParameterDefault) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> PsqlFunctionParameter {
        PsqlFunctionParameter::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_FUNCTION_PARAMETER,
            [
                self.mode_token.map(|token| SyntaxElement::Token(token)),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_interval_expression(
    interval_token: SyntaxToken,
    value: SqlStringLiteralExpression,
) -> PsqlIntervalExpression {
    PsqlIntervalExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_INTERVAL_EXPRESSION,
        [
            Some(SyntaxElement::Token(interval_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_join_using_clause(
    using_token: SyntaxToken,
    columns: SqlColumnList,
) -> PsqlJoinUsingClause {
    PsqlJoinUsingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_JOIN_USING_CLAUSE,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
        ],
    ))
}
pub fn psql_language_option(language_token: SyntaxToken, name: SqlName) -> PsqlLanguageOption {
    PsqlLanguageOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_LANGUAGE_OPTION,
        [
            Some(SyntaxElement::Token(language_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn psql_limit_clause(
    limit_token: SyntaxToken,
    limit_count: AnySqlLimitValue,
) -> PsqlLimitClause {
    PsqlLimitClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_LIMIT_CLAUSE,
        [
            Some(SyntaxElement::Token(limit_token)),
            Some(SyntaxElement::Node(limit_count.into_syntax())),
        ],
    ))
}
pub fn psql_on_conflict_clause(
    on_token: SyntaxToken,
    conflict_token: SyntaxToken,
    action: AnySqlConflictAction,
) -> PsqlOnConflictClauseBuilder {
    PsqlOnConflictClauseBuilder {
        on_token,
        conflict_token,
        action,
        target: None,
    }
}
pub struct PsqlOnConflictClauseBuilder {
    on_token: SyntaxToken,
    conflict_token: SyntaxToken,
    action: AnySqlConflictAction,
    target: Option<AnySqlConflictTarget>,
}
impl PsqlOnConflictClauseBuilder {
    pub fn with_target(mut self, target: AnySqlConflictTarget) -> Self {
        self.target = Some(target);
        self
    }
    pub fn build(self) -> PsqlOnConflictClause {
        PsqlOnConflictClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_ON_CONFLICT_CLAUSE,
            [
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Token(self.conflict_token)),
                self.target
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.action.into_syntax())),
            ],
        ))
    }
}
pub fn psql_on_constraint_clause(
    on_token: SyntaxToken,
    constraint_token: SyntaxToken,
    name: SqlName,
) -> PsqlOnConstraintClause {
    PsqlOnConstraintClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_ON_CONSTRAINT_CLAUSE,
        [
            Some(SyntaxElement::Token(on_token)),
            Some(SyntaxElement::Token(constraint_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn psql_parameter_default(
    marker_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlParameterDefault {
    PsqlParameterDefault::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARAMETER_DEFAULT,
        [
            Some(SyntaxElement::Token(marker_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_policy_for_clause(
    for_token: SyntaxToken,
    command_token: SyntaxToken,
) -> PsqlPolicyForClause {
    PsqlPolicyForClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_POLICY_FOR_CLAUSE,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(command_token)),
        ],
    ))
}
pub fn psql_policy_using_clause(
    using_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlPolicyUsingClause {
    PsqlPolicyUsingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_POLICY_USING_CLAUSE,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_policy_with_check_clause(
    with_token: SyntaxToken,
    check_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlPolicyWithCheckClause {
    PsqlPolicyWithCheckClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_POLICY_WITH_CHECK_CLAUSE,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(check_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_returning_clause(
    returning_token: SyntaxToken,
    items: SqlSelectItemList,
) -> PsqlReturningClause {
    PsqlReturningClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNING_CLAUSE,
        [
            Some(SyntaxElement::Token(returning_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_returns_clause(returns_token: SyntaxToken, ty: AnySqlReturnsType) -> PsqlReturnsClause {
    PsqlReturnsClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_CLAUSE,
        [
            Some(SyntaxElement::Token(returns_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn psql_returns_null_option(
    returns_token: SyntaxToken,
    first_null_token: SyntaxToken,
    on_token: SyntaxToken,
    second_null_token: SyntaxToken,
    input_token: SyntaxToken,
) -> PsqlReturnsNullOption {
    PsqlReturnsNullOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_NULL_OPTION,
        [
            Some(SyntaxElement::Token(returns_token)),
            Some(SyntaxElement::Token(first_null_token)),
            Some(SyntaxElement::Token(on_token)),
            Some(SyntaxElement::Token(second_null_token)),
            Some(SyntaxElement::Token(input_token)),
        ],
    ))
}
pub fn psql_returns_setof_clause(
    setof_token: SyntaxToken,
    ty: SqlTypeName,
) -> PsqlReturnsSetofClause {
    PsqlReturnsSetofClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_SETOF_CLAUSE,
        [
            Some(SyntaxElement::Token(setof_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn psql_returns_table_clause(
    table_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    columns: PsqlReturnsTableColumnList,
    r_paren_token: SyntaxToken,
) -> PsqlReturnsTableClause {
    PsqlReturnsTableClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_TABLE_CLAUSE,
        [
            Some(SyntaxElement::Token(table_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_returns_table_column(name: SqlName, ty: SqlTypeName) -> PsqlReturnsTableColumn {
    PsqlReturnsTableColumn::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn psql_returns_trigger_clause(trigger_token: SyntaxToken) -> PsqlReturnsTriggerClause {
    PsqlReturnsTriggerClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_TRIGGER_CLAUSE,
        [Some(SyntaxElement::Token(trigger_token))],
    ))
}
pub fn psql_security_option(
    security_token: SyntaxToken,
    value_token: SyntaxToken,
) -> PsqlSecurityOption {
    PsqlSecurityOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_SECURITY_OPTION,
        [
            Some(SyntaxElement::Token(security_token)),
            Some(SyntaxElement::Token(value_token)),
        ],
    ))
}
pub fn psql_strict_option(strict_token: SyntaxToken) -> PsqlStrictOption {
    PsqlStrictOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_STRICT_OPTION,
        [Some(SyntaxElement::Token(strict_token))],
    ))
}
pub fn psql_substring_expression(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    expression: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlSubstringExpressionBuilder {
    PsqlSubstringExpressionBuilder {
        name_token,
        l_paren_token,
        expression,
        r_paren_token,
        from_clause: None,
        for_clause: None,
    }
}
pub struct PsqlSubstringExpressionBuilder {
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    expression: AnySqlExpression,
    r_paren_token: SyntaxToken,
    from_clause: Option<PsqlSubstringFromClause>,
    for_clause: Option<PsqlSubstringForClause>,
}
impl PsqlSubstringExpressionBuilder {
    pub fn with_from_clause(mut self, from_clause: PsqlSubstringFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_for_clause(mut self, for_clause: PsqlSubstringForClause) -> Self {
        self.for_clause = Some(for_clause);
        self
    }
    pub fn build(self) -> PsqlSubstringExpression {
        PsqlSubstringExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_SUBSTRING_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.for_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn psql_substring_for_clause(
    for_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlSubstringForClause {
    PsqlSubstringForClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_SUBSTRING_FOR_CLAUSE,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_substring_from_clause(
    from_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlSubstringFromClause {
    PsqlSubstringFromClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_SUBSTRING_FROM_CLAUSE,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_tilde_array_expression(
    array_token: SyntaxToken,
    open_tilde_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    items: SqlExpressionList,
    r_brack_token: SyntaxToken,
    close_tilde_token: SyntaxToken,
) -> PsqlTildeArrayExpression {
    PsqlTildeArrayExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TILDE_ARRAY_EXPRESSION,
        [
            Some(SyntaxElement::Token(array_token)),
            Some(SyntaxElement::Token(open_tilde_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_brack_token)),
            Some(SyntaxElement::Token(close_tilde_token)),
        ],
    ))
}
pub fn psql_tilde_array_suffix(
    open_tilde_token: SyntaxToken,
    l_brack_token: SyntaxToken,
    r_brack_token: SyntaxToken,
    close_tilde_token: SyntaxToken,
) -> PsqlTildeArraySuffix {
    PsqlTildeArraySuffix::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TILDE_ARRAY_SUFFIX,
        [
            Some(SyntaxElement::Token(open_tilde_token)),
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(r_brack_token)),
            Some(SyntaxElement::Token(close_tilde_token)),
        ],
    ))
}
pub fn psql_trigger_event(kind_token: SyntaxToken) -> PsqlTriggerEventBuilder {
    PsqlTriggerEventBuilder {
        kind_token,
        or_token: None,
        of_clause: None,
    }
}
pub struct PsqlTriggerEventBuilder {
    kind_token: SyntaxToken,
    or_token: Option<SyntaxToken>,
    of_clause: Option<PsqlTriggerUpdateOfClause>,
}
impl PsqlTriggerEventBuilder {
    pub fn with_or_token(mut self, or_token: SyntaxToken) -> Self {
        self.or_token = Some(or_token);
        self
    }
    pub fn with_of_clause(mut self, of_clause: PsqlTriggerUpdateOfClause) -> Self {
        self.of_clause = Some(of_clause);
        self
    }
    pub fn build(self) -> PsqlTriggerEvent {
        PsqlTriggerEvent::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_TRIGGER_EVENT,
            [
                self.or_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.kind_token)),
                self.of_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_trigger_for_each_clause(
    for_token: SyntaxToken,
    each_token: SyntaxToken,
    granularity_token: SyntaxToken,
) -> PsqlTriggerForEachClause {
    PsqlTriggerForEachClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_FOR_EACH_CLAUSE,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(each_token)),
            Some(SyntaxElement::Token(granularity_token)),
        ],
    ))
}
pub fn psql_trigger_referencing_clause(
    referencing_token: SyntaxToken,
    items: PsqlTriggerReferencingItemList,
) -> PsqlTriggerReferencingClause {
    PsqlTriggerReferencingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_CLAUSE,
        [
            Some(SyntaxElement::Token(referencing_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn psql_trigger_referencing_item(
    which_token: SyntaxToken,
    table_token: SyntaxToken,
    as_token: SyntaxToken,
    name: SqlName,
) -> PsqlTriggerReferencingItem {
    PsqlTriggerReferencingItem::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM,
        [
            Some(SyntaxElement::Token(which_token)),
            Some(SyntaxElement::Token(table_token)),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn psql_trigger_update_of_clause(
    of_token: SyntaxToken,
    columns: SqlColumnNameList,
) -> PsqlTriggerUpdateOfClause {
    PsqlTriggerUpdateOfClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_UPDATE_OF_CLAUSE,
        [
            Some(SyntaxElement::Token(of_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
        ],
    ))
}
pub fn psql_trigger_when_clause(
    when_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlTriggerWhenClause {
    PsqlTriggerWhenClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_WHEN_CLAUSE,
        [
            Some(SyntaxElement::Token(when_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_type_array_suffix(
    l_brack_token: SyntaxToken,
    r_brack_token: SyntaxToken,
) -> PsqlTypeArraySuffix {
    PsqlTypeArraySuffix::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TYPE_ARRAY_SUFFIX,
        [
            Some(SyntaxElement::Token(l_brack_token)),
            Some(SyntaxElement::Token(r_brack_token)),
        ],
    ))
}
pub fn psql_view_option(
    name: SqlName,
    eq_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlViewOption {
    PsqlViewOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_VIEW_OPTION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_view_options(
    with_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    items: PsqlViewOptionList,
    r_paren_token: SyntaxToken,
) -> PsqlViewOptions {
    PsqlViewOptions::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_VIEW_OPTIONS,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_volatility_option(value_token: SyntaxToken) -> PsqlVolatilityOption {
    PsqlVolatilityOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_VOLATILITY_OPTION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_alias(value: SqlName) -> SqlAliasBuilder {
    SqlAliasBuilder {
        value,
        as_token: None,
        columns: None,
    }
}
pub struct SqlAliasBuilder {
    value: SqlName,
    as_token: Option<SyntaxToken>,
    columns: Option<SqlAliasColumnList>,
}
impl SqlAliasBuilder {
    pub fn with_as_token(mut self, as_token: SyntaxToken) -> Self {
        self.as_token = Some(as_token);
        self
    }
    pub fn with_columns(mut self, columns: SqlAliasColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn build(self) -> SqlAlias {
        SqlAlias::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALIAS,
            [
                self.as_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_alias_column_definition(name: SqlName) -> SqlAliasColumnDefinitionBuilder {
    SqlAliasColumnDefinitionBuilder { name, ty: None }
}
pub struct SqlAliasColumnDefinitionBuilder {
    name: SqlName,
    ty: Option<SqlTypeName>,
}
impl SqlAliasColumnDefinitionBuilder {
    pub fn with_ty(mut self, ty: SqlTypeName) -> Self {
        self.ty = Some(ty);
        self
    }
    pub fn build(self) -> SqlAliasColumnDefinition {
        SqlAliasColumnDefinition::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALIAS_COLUMN_DEFINITION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.ty
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_alias_column_list(
    l_paren_token: SyntaxToken,
    items: SqlAliasColumnDefinitionList,
    r_paren_token: SyntaxToken,
) -> SqlAliasColumnList {
    SqlAliasColumnList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ALIAS_COLUMN_LIST,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_any_all_expression(
    quantifier_token: SyntaxToken,
    source: AnySqlAnyAllSource,
) -> SqlAnyAllExpression {
    SqlAnyAllExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ANY_ALL_EXPRESSION,
        [
            Some(SyntaxElement::Token(quantifier_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
        ],
    ))
}
pub fn sql_between_expression(
    expression: AnySqlExpression,
    between_token: SyntaxToken,
    low: AnySqlExpression,
    and_token: SyntaxToken,
    high: AnySqlExpression,
) -> SqlBetweenExpressionBuilder {
    SqlBetweenExpressionBuilder {
        expression,
        between_token,
        low,
        and_token,
        high,
        not_token: None,
    }
}
pub struct SqlBetweenExpressionBuilder {
    expression: AnySqlExpression,
    between_token: SyntaxToken,
    low: AnySqlExpression,
    and_token: SyntaxToken,
    high: AnySqlExpression,
    not_token: Option<SyntaxToken>,
}
impl SqlBetweenExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlBetweenExpression {
        SqlBetweenExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_BETWEEN_EXPRESSION,
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
pub fn sql_binary_expression(
    left: AnySqlExpression,
    operator_token_token: SyntaxToken,
    right: AnySqlExpression,
) -> SqlBinaryExpression {
    SqlBinaryExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BINARY_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn sql_boolean_literal_expression(value_token: SyntaxToken) -> SqlBooleanLiteralExpression {
    SqlBooleanLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOOLEAN_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_call_expression(
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    arguments: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> SqlCallExpressionBuilder {
    SqlCallExpressionBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        schema: None,
        filter_clause: None,
    }
}
pub struct SqlCallExpressionBuilder {
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    arguments: SqlExpressionList,
    r_paren_token: SyntaxToken,
    schema: Option<SqlShemaName>,
    filter_clause: Option<PsqlFilterClause>,
}
impl SqlCallExpressionBuilder {
    pub fn with_schema(mut self, schema: SqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_filter_clause(mut self, filter_clause: PsqlFilterClause) -> Self {
        self.filter_clause = Some(filter_clause);
        self
    }
    pub fn build(self) -> SqlCallExpression {
        SqlCallExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CALL_EXPRESSION,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.filter_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_case_else_clause(
    else_token: SyntaxToken,
    result: AnySqlExpression,
) -> SqlCaseElseClause {
    SqlCaseElseClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CASE_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(result.into_syntax())),
        ],
    ))
}
pub fn sql_case_expression(
    case_token: SyntaxToken,
    when_clauses: SqlCaseWhenClauseList,
    end_token: SyntaxToken,
) -> SqlCaseExpressionBuilder {
    SqlCaseExpressionBuilder {
        case_token,
        when_clauses,
        end_token,
        expression: None,
        else_clause: None,
    }
}
pub struct SqlCaseExpressionBuilder {
    case_token: SyntaxToken,
    when_clauses: SqlCaseWhenClauseList,
    end_token: SyntaxToken,
    expression: Option<AnySqlExpression>,
    else_clause: Option<SqlCaseElseClause>,
}
impl SqlCaseExpressionBuilder {
    pub fn with_expression(mut self, expression: AnySqlExpression) -> Self {
        self.expression = Some(expression);
        self
    }
    pub fn with_else_clause(mut self, else_clause: SqlCaseElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> SqlCaseExpression {
        SqlCaseExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CASE_EXPRESSION,
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
pub fn sql_case_when_clause(
    when_token: SyntaxToken,
    condition: AnySqlExpression,
    then_token: SyntaxToken,
    result: AnySqlExpression,
) -> SqlCaseWhenClause {
    SqlCaseWhenClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CASE_WHEN_CLAUSE,
        [
            Some(SyntaxElement::Token(when_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(then_token)),
            Some(SyntaxElement::Node(result.into_syntax())),
        ],
    ))
}
pub fn sql_cast_function_expression(
    cast_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    expression: AnySqlExpression,
    as_token: SyntaxToken,
    ty: SqlTypeName,
    r_paren_token: SyntaxToken,
) -> SqlCastFunctionExpression {
    SqlCastFunctionExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CAST_FUNCTION_EXPRESSION,
        [
            Some(SyntaxElement::Token(cast_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_col_reference(name: SqlName) -> SqlColReference {
    SqlColReference::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COL_REFERENCE,
        [Some(SyntaxElement::Node(name.into_syntax()))],
    ))
}
pub fn sql_column_definition(name: SqlName, ty: SqlTypeName) -> SqlColumnDefinition {
    SqlColumnDefinition::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_DEFINITION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(ty.into_syntax())),
        ],
    ))
}
pub fn sql_column_list(
    l_paren_token: SyntaxToken,
    items: SqlColumnNameList,
    r_paren_token: SyntaxToken,
) -> SqlColumnList {
    SqlColumnList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_LIST,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_create_table_statement(
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    l_paren_token: SyntaxToken,
    columns: SqlColumnDefinitionList,
    r_paren_token: SyntaxToken,
) -> SqlCreateTableStatementBuilder {
    SqlCreateTableStatementBuilder {
        create_token,
        table_token,
        name,
        l_paren_token,
        columns,
        r_paren_token,
        if_token: None,
        not_token: None,
        exists_token: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateTableStatementBuilder {
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    l_paren_token: SyntaxToken,
    columns: SqlColumnDefinitionList,
    r_paren_token: SyntaxToken,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCreateTableStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCreateTableStatement {
        SqlCreateTableStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CREATE_TABLE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.columns.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_create_view_statement(
    create_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: SqlSelectStatement,
) -> SqlCreateViewStatementBuilder {
    SqlCreateViewStatementBuilder {
        create_token,
        view_token,
        name,
        as_token,
        query,
        or_token: None,
        replace_token: None,
        options: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateViewStatementBuilder {
    create_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: SqlSelectStatement,
    or_token: Option<SyntaxToken>,
    replace_token: Option<SyntaxToken>,
    options: Option<PsqlViewOptions>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCreateViewStatementBuilder {
    pub fn with_or_token(mut self, or_token: SyntaxToken) -> Self {
        self.or_token = Some(or_token);
        self
    }
    pub fn with_replace_token(mut self, replace_token: SyntaxToken) -> Self {
        self.replace_token = Some(replace_token);
        self
    }
    pub fn with_options(mut self, options: PsqlViewOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCreateViewStatement {
        SqlCreateViewStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CREATE_VIEW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                self.or_token.map(|token| SyntaxElement::Token(token)),
                self.replace_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.view_token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.options
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_cte_definition(
    name: SqlName,
    as_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    query: AnySqlStatement,
    r_paren_token: SyntaxToken,
) -> SqlCteDefinitionBuilder {
    SqlCteDefinitionBuilder {
        name,
        as_token,
        l_paren_token,
        query,
        r_paren_token,
        columns: None,
        materialized: None,
    }
}
pub struct SqlCteDefinitionBuilder {
    name: SqlName,
    as_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    query: AnySqlStatement,
    r_paren_token: SyntaxToken,
    columns: Option<SqlColumnList>,
    materialized: Option<PsqlCteMaterializedHint>,
}
impl SqlCteDefinitionBuilder {
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn with_materialized(mut self, materialized: PsqlCteMaterializedHint) -> Self {
        self.materialized = Some(materialized);
        self
    }
    pub fn build(self) -> SqlCteDefinition {
        SqlCteDefinition::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CTE_DEFINITION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                self.materialized
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn sql_data_base_name(name: SqlName, dot_token: SyntaxToken) -> SqlDataBaseName {
    SqlDataBaseName::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_DATA_BASE_NAME,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
        ],
    ))
}
pub fn sql_delete_statement(
    delete_token: SyntaxToken,
    from_token: SyntaxToken,
    table: SqlTableBinding,
) -> SqlDeleteStatementBuilder {
    SqlDeleteStatementBuilder {
        delete_token,
        from_token,
        table,
        with_clause: None,
        using: None,
        where_clause: None,
        returning_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlDeleteStatementBuilder {
    delete_token: SyntaxToken,
    from_token: SyntaxToken,
    table: SqlTableBinding,
    with_clause: Option<SqlWithClause>,
    using: Option<PsqlDeleteUsingClause>,
    where_clause: Option<SqlWhereClause>,
    returning_clause: Option<PsqlReturningClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlDeleteStatementBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
        self
    }
    pub fn with_using(mut self, using: PsqlDeleteUsingClause) -> Self {
        self.using = Some(using);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_returning_clause(mut self, returning_clause: PsqlReturningClause) -> Self {
        self.returning_clause = Some(returning_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlDeleteStatement {
        SqlDeleteStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DELETE_STATEMENT,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.delete_token)),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.using
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.returning_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_drop_function_statement(
    drop_token: SyntaxToken,
    kind_token: SyntaxToken,
    name: AnySqlName,
) -> SqlDropFunctionStatementBuilder {
    SqlDropFunctionStatementBuilder {
        drop_token,
        kind_token,
        name,
        if_token: None,
        exists_token: None,
        parameters: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlDropFunctionStatementBuilder {
    drop_token: SyntaxToken,
    kind_token: SyntaxToken,
    name: AnySqlName,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    parameters: Option<PsqlDropFunctionParameters>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlDropFunctionStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_parameters(mut self, parameters: PsqlDropFunctionParameters) -> Self {
        self.parameters = Some(parameters);
        self
    }
    pub fn with_drop_behavior_token(mut self, drop_behavior_token: SyntaxToken) -> Self {
        self.drop_behavior_token = Some(drop_behavior_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlDropFunctionStatement {
        SqlDropFunctionStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_FUNCTION_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.kind_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.parameters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_drop_table_statement(
    drop_token: SyntaxToken,
    table_token: SyntaxToken,
    tables: SqlTableNameList,
) -> SqlDropTableStatementBuilder {
    SqlDropTableStatementBuilder {
        drop_token,
        table_token,
        tables,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlDropTableStatementBuilder {
    drop_token: SyntaxToken,
    table_token: SyntaxToken,
    tables: SqlTableNameList,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlDropTableStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_drop_behavior_token(mut self, drop_behavior_token: SyntaxToken) -> Self {
        self.drop_behavior_token = Some(drop_behavior_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlDropTableStatement {
        SqlDropTableStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_TABLE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.tables.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_drop_view_statement(
    drop_token: SyntaxToken,
    view_token: SyntaxToken,
    views: SqlTableNameList,
) -> SqlDropViewStatementBuilder {
    SqlDropViewStatementBuilder {
        drop_token,
        view_token,
        views,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlDropViewStatementBuilder {
    drop_token: SyntaxToken,
    view_token: SyntaxToken,
    views: SqlTableNameList,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlDropViewStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_drop_behavior_token(mut self, drop_behavior_token: SyntaxToken) -> Self {
        self.drop_behavior_token = Some(drop_behavior_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlDropViewStatement {
        SqlDropViewStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_VIEW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.view_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.views.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_empty_statement(semicolon_token: SyntaxToken) -> SqlEmptyStatement {
    SqlEmptyStatement::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_EMPTY_STATEMENT,
        [Some(SyntaxElement::Token(semicolon_token))],
    ))
}
pub fn sql_exists_expression(
    exists_token: SyntaxToken,
    subquery: SqlSubqueryExpression,
) -> SqlExistsExpression {
    SqlExistsExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_EXISTS_EXPRESSION,
        [
            Some(SyntaxElement::Token(exists_token)),
            Some(SyntaxElement::Node(subquery.into_syntax())),
        ],
    ))
}
pub fn sql_fetch_clause(
    fetch_token: SyntaxToken,
    quantifier_token: SyntaxToken,
    row_or_rows_token: SyntaxToken,
    tail: AnySqlFetchTail,
) -> SqlFetchClauseBuilder {
    SqlFetchClauseBuilder {
        fetch_token,
        quantifier_token,
        row_or_rows_token,
        tail,
        count: None,
    }
}
pub struct SqlFetchClauseBuilder {
    fetch_token: SyntaxToken,
    quantifier_token: SyntaxToken,
    row_or_rows_token: SyntaxToken,
    tail: AnySqlFetchTail,
    count: Option<AnySqlLimitValue>,
}
impl SqlFetchClauseBuilder {
    pub fn with_count(mut self, count: AnySqlLimitValue) -> Self {
        self.count = Some(count);
        self
    }
    pub fn build(self) -> SqlFetchClause {
        SqlFetchClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_FETCH_CLAUSE,
            [
                Some(SyntaxElement::Token(self.fetch_token)),
                Some(SyntaxElement::Token(self.quantifier_token)),
                self.count
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.row_or_rows_token)),
                Some(SyntaxElement::Node(self.tail.into_syntax())),
            ],
        ))
    }
}
pub fn sql_fetch_only_tail(only_token: SyntaxToken) -> SqlFetchOnlyTail {
    SqlFetchOnlyTail::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FETCH_ONLY_TAIL,
        [Some(SyntaxElement::Token(only_token))],
    ))
}
pub fn sql_fetch_with_ties_tail(
    with_token: SyntaxToken,
    ties_token: SyntaxToken,
) -> SqlFetchWithTiesTail {
    SqlFetchWithTiesTail::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FETCH_WITH_TIES_TAIL,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(ties_token)),
        ],
    ))
}
pub fn sql_from_clause(from_token: SyntaxToken, items: SqlFromItemList) -> SqlFromClause {
    SqlFromClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FROM_CLAUSE,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_from_item(source: AnySqlFromExpression, joins: SqlJoinClauseList) -> SqlFromItem {
    SqlFromItem::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FROM_ITEM,
        [
            Some(SyntaxElement::Node(source.into_syntax())),
            Some(SyntaxElement::Node(joins.into_syntax())),
        ],
    ))
}
pub fn sql_function_binding(
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    arguments: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> SqlFunctionBindingBuilder {
    SqlFunctionBindingBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        lateral_token: None,
        schema: None,
        alias: None,
    }
}
pub struct SqlFunctionBindingBuilder {
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    arguments: SqlExpressionList,
    r_paren_token: SyntaxToken,
    lateral_token: Option<SyntaxToken>,
    schema: Option<SqlShemaName>,
    alias: Option<SqlAlias>,
}
impl SqlFunctionBindingBuilder {
    pub fn with_lateral_token(mut self, lateral_token: SyntaxToken) -> Self {
        self.lateral_token = Some(lateral_token);
        self
    }
    pub fn with_schema(mut self, schema: SqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_alias(mut self, alias: SqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> SqlFunctionBinding {
        SqlFunctionBinding::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_FUNCTION_BINDING,
            [
                self.lateral_token.map(|token| SyntaxElement::Token(token)),
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
pub fn sql_grant_statement(
    grant_token: SyntaxToken,
    all_token: SyntaxToken,
    on_token: SyntaxToken,
    objects: SqlTableNameList,
    to_token: SyntaxToken,
    grantees: SqlGranteeList,
) -> SqlGrantStatementBuilder {
    SqlGrantStatementBuilder {
        grant_token,
        all_token,
        on_token,
        objects,
        to_token,
        grantees,
        table_token: None,
        semicolon_token: None,
    }
}
pub struct SqlGrantStatementBuilder {
    grant_token: SyntaxToken,
    all_token: SyntaxToken,
    on_token: SyntaxToken,
    objects: SqlTableNameList,
    to_token: SyntaxToken,
    grantees: SqlGranteeList,
    table_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlGrantStatementBuilder {
    pub fn with_table_token(mut self, table_token: SyntaxToken) -> Self {
        self.table_token = Some(table_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlGrantStatement {
        SqlGrantStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_GRANT_STATEMENT,
            [
                Some(SyntaxElement::Token(self.grant_token)),
                Some(SyntaxElement::Token(self.all_token)),
                Some(SyntaxElement::Token(self.on_token)),
                self.table_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.objects.into_syntax())),
                Some(SyntaxElement::Token(self.to_token)),
                Some(SyntaxElement::Node(self.grantees.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_group_by_clause(
    group_by_token: SyntaxToken,
    items: SqlGroupByItemList,
) -> SqlGroupByClause {
    SqlGroupByClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_GROUP_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(group_by_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_having_clause(
    having_token: SyntaxToken,
    condition: AnySqlExpression,
) -> SqlHavingClause {
    SqlHavingClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_HAVING_CLAUSE,
        [
            Some(SyntaxElement::Token(having_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn sql_in_expression(
    expression: AnySqlExpression,
    in_token: SyntaxToken,
    source: AnySqlInSource,
) -> SqlInExpressionBuilder {
    SqlInExpressionBuilder {
        expression,
        in_token,
        source,
        not_token: None,
    }
}
pub struct SqlInExpressionBuilder {
    expression: AnySqlExpression,
    in_token: SyntaxToken,
    source: AnySqlInSource,
    not_token: Option<SyntaxToken>,
}
impl SqlInExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlInExpression {
        SqlInExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_IN_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.in_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
            ],
        ))
    }
}
pub fn sql_in_value_list(
    l_paren_token: SyntaxToken,
    items: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> SqlInValueList {
    SqlInValueList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_IN_VALUE_LIST,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_insert_statement(
    insert_token: SyntaxToken,
    into_token: SyntaxToken,
    table: SqlTableBinding,
    source: AnySqlInsertSource,
) -> SqlInsertStatementBuilder {
    SqlInsertStatementBuilder {
        insert_token,
        into_token,
        table,
        source,
        with_clause: None,
        columns: None,
        on_conflict_clause: None,
        returning_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlInsertStatementBuilder {
    insert_token: SyntaxToken,
    into_token: SyntaxToken,
    table: SqlTableBinding,
    source: AnySqlInsertSource,
    with_clause: Option<SqlWithClause>,
    columns: Option<SqlColumnList>,
    on_conflict_clause: Option<PsqlOnConflictClause>,
    returning_clause: Option<PsqlReturningClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlInsertStatementBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
        self
    }
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn with_on_conflict_clause(mut self, on_conflict_clause: PsqlOnConflictClause) -> Self {
        self.on_conflict_clause = Some(on_conflict_clause);
        self
    }
    pub fn with_returning_clause(mut self, returning_clause: PsqlReturningClause) -> Self {
        self.returning_clause = Some(returning_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlInsertStatement {
        SqlInsertStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_INSERT_STATEMENT,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.insert_token)),
                Some(SyntaxElement::Token(self.into_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.on_conflict_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.returning_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_is_null_expression(
    expression: AnySqlExpression,
    is_token: SyntaxToken,
    null_token: SyntaxToken,
) -> SqlIsNullExpressionBuilder {
    SqlIsNullExpressionBuilder {
        expression,
        is_token,
        null_token,
        not_token: None,
    }
}
pub struct SqlIsNullExpressionBuilder {
    expression: AnySqlExpression,
    is_token: SyntaxToken,
    null_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl SqlIsNullExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlIsNullExpression {
        SqlIsNullExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_IS_NULL_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                Some(SyntaxElement::Token(self.is_token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.null_token)),
            ],
        ))
    }
}
pub fn sql_join_clause(
    join_token: SyntaxToken,
    source: AnySqlFromExpression,
) -> SqlJoinClauseBuilder {
    SqlJoinClauseBuilder {
        join_token,
        source,
        join_type_token: None,
        outer_token: None,
        on_token: None,
        condition: None,
        using_clause: None,
    }
}
pub struct SqlJoinClauseBuilder {
    join_token: SyntaxToken,
    source: AnySqlFromExpression,
    join_type_token: Option<SyntaxToken>,
    outer_token: Option<SyntaxToken>,
    on_token: Option<SyntaxToken>,
    condition: Option<AnySqlExpression>,
    using_clause: Option<PsqlJoinUsingClause>,
}
impl SqlJoinClauseBuilder {
    pub fn with_join_type_token(mut self, join_type_token: SyntaxToken) -> Self {
        self.join_type_token = Some(join_type_token);
        self
    }
    pub fn with_outer_token(mut self, outer_token: SyntaxToken) -> Self {
        self.outer_token = Some(outer_token);
        self
    }
    pub fn with_on_token(mut self, on_token: SyntaxToken) -> Self {
        self.on_token = Some(on_token);
        self
    }
    pub fn with_condition(mut self, condition: AnySqlExpression) -> Self {
        self.condition = Some(condition);
        self
    }
    pub fn with_using_clause(mut self, using_clause: PsqlJoinUsingClause) -> Self {
        self.using_clause = Some(using_clause);
        self
    }
    pub fn build(self) -> SqlJoinClause {
        SqlJoinClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_JOIN_CLAUSE,
            [
                self.join_type_token
                    .map(|token| SyntaxElement::Token(token)),
                self.outer_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.join_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                self.on_token.map(|token| SyntaxElement::Token(token)),
                self.condition
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.using_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_like_expression(
    expression: AnySqlExpression,
    operator_token_token: SyntaxToken,
    pattern: AnySqlExpression,
) -> SqlLikeExpressionBuilder {
    SqlLikeExpressionBuilder {
        expression,
        operator_token_token,
        pattern,
        not_token: None,
    }
}
pub struct SqlLikeExpressionBuilder {
    expression: AnySqlExpression,
    operator_token_token: SyntaxToken,
    pattern: AnySqlExpression,
    not_token: Option<SyntaxToken>,
}
impl SqlLikeExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlLikeExpression {
        SqlLikeExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_LIKE_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.operator_token_token)),
                Some(SyntaxElement::Node(self.pattern.into_syntax())),
            ],
        ))
    }
}
pub fn sql_logical_expression(
    left: AnySqlExpression,
    operator_token_token: SyntaxToken,
    right: AnySqlExpression,
) -> SqlLogicalExpression {
    SqlLogicalExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_LOGICAL_EXPRESSION,
        [
            Some(SyntaxElement::Node(left.into_syntax())),
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(right.into_syntax())),
        ],
    ))
}
pub fn sql_name(value_token: SyntaxToken) -> SqlName {
    SqlName::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_null_literal_expression(value_token: SyntaxToken) -> SqlNullLiteralExpression {
    SqlNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_number_literal_expression(value_token: SyntaxToken) -> SqlNumberLiteralExpression {
    SqlNumberLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NUMBER_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_offset_clause(offset_token: SyntaxToken, start: AnySqlLimitValue) -> SqlOffsetClause {
    SqlOffsetClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_OFFSET_CLAUSE,
        [
            Some(SyntaxElement::Token(offset_token)),
            Some(SyntaxElement::Node(start.into_syntax())),
        ],
    ))
}
pub fn sql_order_by_clause(
    order_by_token: SyntaxToken,
    items: SqlOrderByExpressionList,
) -> SqlOrderByClause {
    SqlOrderByClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ORDER_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(order_by_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_order_by_expression(item: AnySqlExpression) -> SqlOrderByExpressionBuilder {
    SqlOrderByExpressionBuilder {
        item,
        order_token: None,
    }
}
pub struct SqlOrderByExpressionBuilder {
    item: AnySqlExpression,
    order_token: Option<SyntaxToken>,
}
impl SqlOrderByExpressionBuilder {
    pub fn with_order_token(mut self, order_token: SyntaxToken) -> Self {
        self.order_token = Some(order_token);
        self
    }
    pub fn build(self) -> SqlOrderByExpression {
        SqlOrderByExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ORDER_BY_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.item.into_syntax())),
                self.order_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_parameter_expression(
    colon_token: SyntaxToken,
    name_token: SyntaxToken,
) -> SqlParameterExpression {
    SqlParameterExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PARAMETER_EXPRESSION,
        [
            Some(SyntaxElement::Token(colon_token)),
            Some(SyntaxElement::Token(name_token)),
        ],
    ))
}
pub fn sql_parenthesized_expression(
    l_paren_token: SyntaxToken,
    expression: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> SqlParenthesizedExpression {
    SqlParenthesizedExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PARENTHESIZED_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_parenthesized_join_binding(
    l_paren_token: SyntaxToken,
    source: AnySqlFromExpression,
    joins: SqlJoinClauseList,
    r_paren_token: SyntaxToken,
) -> SqlParenthesizedJoinBindingBuilder {
    SqlParenthesizedJoinBindingBuilder {
        l_paren_token,
        source,
        joins,
        r_paren_token,
        alias: None,
    }
}
pub struct SqlParenthesizedJoinBindingBuilder {
    l_paren_token: SyntaxToken,
    source: AnySqlFromExpression,
    joins: SqlJoinClauseList,
    r_paren_token: SyntaxToken,
    alias: Option<SqlAlias>,
}
impl SqlParenthesizedJoinBindingBuilder {
    pub fn with_alias(mut self, alias: SqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> SqlParenthesizedJoinBinding {
        SqlParenthesizedJoinBinding::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_PARENTHESIZED_JOIN_BINDING,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                Some(SyntaxElement::Node(self.joins.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_precision_modifier(precision_token: SyntaxToken) -> SqlPrecisionModifier {
    SqlPrecisionModifier::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRECISION_MODIFIER,
        [Some(SyntaxElement::Token(precision_token))],
    ))
}
pub fn sql_root(stmt: SqlStatementList, eof_token: SyntaxToken) -> SqlRootBuilder {
    SqlRootBuilder {
        stmt,
        eof_token,
        bom_token: None,
    }
}
pub struct SqlRootBuilder {
    stmt: SqlStatementList,
    eof_token: SyntaxToken,
    bom_token: Option<SyntaxToken>,
}
impl SqlRootBuilder {
    pub fn with_bom_token(mut self, bom_token: SyntaxToken) -> Self {
        self.bom_token = Some(bom_token);
        self
    }
    pub fn build(self) -> SqlRoot {
        SqlRoot::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ROOT,
            [
                self.bom_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.stmt.into_syntax())),
                Some(SyntaxElement::Token(self.eof_token)),
            ],
        ))
    }
}
pub fn sql_select_all_quantifier(all_token: SyntaxToken) -> SqlSelectAllQuantifier {
    SqlSelectAllQuantifier::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SELECT_ALL_QUANTIFIER,
        [Some(SyntaxElement::Token(all_token))],
    ))
}
pub fn sql_select_clause(
    select_token: SyntaxToken,
    list: SqlSelectItemList,
) -> SqlSelectClauseBuilder {
    SqlSelectClauseBuilder {
        select_token,
        list,
        quantifier: None,
    }
}
pub struct SqlSelectClauseBuilder {
    select_token: SyntaxToken,
    list: SqlSelectItemList,
    quantifier: Option<AnySqlSelectQuantifier>,
}
impl SqlSelectClauseBuilder {
    pub fn with_quantifier(mut self, quantifier: AnySqlSelectQuantifier) -> Self {
        self.quantifier = Some(quantifier);
        self
    }
    pub fn build(self) -> SqlSelectClause {
        SqlSelectClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SELECT_CLAUSE,
            [
                Some(SyntaxElement::Token(self.select_token)),
                self.quantifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.list.into_syntax())),
            ],
        ))
    }
}
pub fn sql_select_distinct_quantifier(
    distinct_token: SyntaxToken,
) -> SqlSelectDistinctQuantifierBuilder {
    SqlSelectDistinctQuantifierBuilder {
        distinct_token,
        on_clause: None,
    }
}
pub struct SqlSelectDistinctQuantifierBuilder {
    distinct_token: SyntaxToken,
    on_clause: Option<PsqlDistinctOnClause>,
}
impl SqlSelectDistinctQuantifierBuilder {
    pub fn with_on_clause(mut self, on_clause: PsqlDistinctOnClause) -> Self {
        self.on_clause = Some(on_clause);
        self
    }
    pub fn build(self) -> SqlSelectDistinctQuantifier {
        SqlSelectDistinctQuantifier::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SELECT_DISTINCT_QUANTIFIER,
            [
                Some(SyntaxElement::Token(self.distinct_token)),
                self.on_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_select_expression(expr: AnySqlExpression) -> SqlSelectExpressionBuilder {
    SqlSelectExpressionBuilder { expr, alias: None }
}
pub struct SqlSelectExpressionBuilder {
    expr: AnySqlExpression,
    alias: Option<SqlAlias>,
}
impl SqlSelectExpressionBuilder {
    pub fn with_alias(mut self, alias: SqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> SqlSelectExpression {
        SqlSelectExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SELECT_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expr.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_select_statement(
    select_clause: SqlSelectClause,
    set_operations: SqlSetOperationList,
) -> SqlSelectStatementBuilder {
    SqlSelectStatementBuilder {
        select_clause,
        set_operations,
        with_clause: None,
        from_clause: None,
        where_clause: None,
        group_by_clause: None,
        having_clause: None,
        order_by_clause: None,
        limit_clause: None,
        offset_clause: None,
        fetch_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlSelectStatementBuilder {
    select_clause: SqlSelectClause,
    set_operations: SqlSetOperationList,
    with_clause: Option<SqlWithClause>,
    from_clause: Option<SqlFromClause>,
    where_clause: Option<SqlWhereClause>,
    group_by_clause: Option<SqlGroupByClause>,
    having_clause: Option<SqlHavingClause>,
    order_by_clause: Option<SqlOrderByClause>,
    limit_clause: Option<PsqlLimitClause>,
    offset_clause: Option<SqlOffsetClause>,
    fetch_clause: Option<SqlFetchClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlSelectStatementBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
        self
    }
    pub fn with_from_clause(mut self, from_clause: SqlFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_group_by_clause(mut self, group_by_clause: SqlGroupByClause) -> Self {
        self.group_by_clause = Some(group_by_clause);
        self
    }
    pub fn with_having_clause(mut self, having_clause: SqlHavingClause) -> Self {
        self.having_clause = Some(having_clause);
        self
    }
    pub fn with_order_by_clause(mut self, order_by_clause: SqlOrderByClause) -> Self {
        self.order_by_clause = Some(order_by_clause);
        self
    }
    pub fn with_limit_clause(mut self, limit_clause: PsqlLimitClause) -> Self {
        self.limit_clause = Some(limit_clause);
        self
    }
    pub fn with_offset_clause(mut self, offset_clause: SqlOffsetClause) -> Self {
        self.offset_clause = Some(offset_clause);
        self
    }
    pub fn with_fetch_clause(mut self, fetch_clause: SqlFetchClause) -> Self {
        self.fetch_clause = Some(fetch_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlSelectStatement {
        SqlSelectStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SELECT_STATEMENT,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.select_clause.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.group_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.having_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.set_operations.into_syntax())),
                self.order_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.limit_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.offset_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.fetch_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_set_clause(set_token: SyntaxToken, items: SqlSetItemList) -> SqlSetClause {
    SqlSetClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_CLAUSE,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_set_item(column: SqlName, eq_token: SyntaxToken, expr: AnySqlExpression) -> SqlSetItem {
    SqlSetItem::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_ITEM,
        [
            Some(SyntaxElement::Node(column.into_syntax())),
            Some(SyntaxElement::Token(eq_token)),
            Some(SyntaxElement::Node(expr.into_syntax())),
        ],
    ))
}
pub fn sql_set_operation(
    operator_token_token: SyntaxToken,
    select_clause: SqlSelectClause,
) -> SqlSetOperationBuilder {
    SqlSetOperationBuilder {
        operator_token_token,
        select_clause,
        quantifier_token: None,
        from_clause: None,
        where_clause: None,
        group_by_clause: None,
        having_clause: None,
    }
}
pub struct SqlSetOperationBuilder {
    operator_token_token: SyntaxToken,
    select_clause: SqlSelectClause,
    quantifier_token: Option<SyntaxToken>,
    from_clause: Option<SqlFromClause>,
    where_clause: Option<SqlWhereClause>,
    group_by_clause: Option<SqlGroupByClause>,
    having_clause: Option<SqlHavingClause>,
}
impl SqlSetOperationBuilder {
    pub fn with_quantifier_token(mut self, quantifier_token: SyntaxToken) -> Self {
        self.quantifier_token = Some(quantifier_token);
        self
    }
    pub fn with_from_clause(mut self, from_clause: SqlFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_group_by_clause(mut self, group_by_clause: SqlGroupByClause) -> Self {
        self.group_by_clause = Some(group_by_clause);
        self
    }
    pub fn with_having_clause(mut self, having_clause: SqlHavingClause) -> Self {
        self.having_clause = Some(having_clause);
        self
    }
    pub fn build(self) -> SqlSetOperation {
        SqlSetOperation::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SET_OPERATION,
            [
                Some(SyntaxElement::Token(self.operator_token_token)),
                self.quantifier_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.select_clause.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.group_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.having_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_shema_name(name: SqlName, dot_token: SyntaxToken) -> SqlShemaNameBuilder {
    SqlShemaNameBuilder {
        name,
        dot_token,
        base: None,
    }
}
pub struct SqlShemaNameBuilder {
    name: SqlName,
    dot_token: SyntaxToken,
    base: Option<SqlDataBaseName>,
}
impl SqlShemaNameBuilder {
    pub fn with_base(mut self, base: SqlDataBaseName) -> Self {
        self.base = Some(base);
        self
    }
    pub fn build(self) -> SqlShemaName {
        SqlShemaName::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SHEMA_NAME,
            [
                self.base
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.dot_token)),
            ],
        ))
    }
}
pub fn sql_star(value_token: SyntaxToken) -> SqlStar {
    SqlStar::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_STAR,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_string_literal_expression(value_token: SyntaxToken) -> SqlStringLiteralExpression {
    SqlStringLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_STRING_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_subquery_binding(
    l_paren_token: SyntaxToken,
    query: AnySqlSubqueryBody,
    r_paren_token: SyntaxToken,
) -> SqlSubqueryBindingBuilder {
    SqlSubqueryBindingBuilder {
        l_paren_token,
        query,
        r_paren_token,
        lateral_token: None,
        alias: None,
    }
}
pub struct SqlSubqueryBindingBuilder {
    l_paren_token: SyntaxToken,
    query: AnySqlSubqueryBody,
    r_paren_token: SyntaxToken,
    lateral_token: Option<SyntaxToken>,
    alias: Option<SqlAlias>,
}
impl SqlSubqueryBindingBuilder {
    pub fn with_lateral_token(mut self, lateral_token: SyntaxToken) -> Self {
        self.lateral_token = Some(lateral_token);
        self
    }
    pub fn with_alias(mut self, alias: SqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> SqlSubqueryBinding {
        SqlSubqueryBinding::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SUBQUERY_BINDING,
            [
                self.lateral_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_subquery_expression(
    l_paren_token: SyntaxToken,
    query: AnySqlSubqueryBody,
    r_paren_token: SyntaxToken,
) -> SqlSubqueryExpression {
    SqlSubqueryExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SUBQUERY_EXPRESSION,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(query.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_table_binding(table: SqlTableName) -> SqlTableBindingBuilder {
    SqlTableBindingBuilder { table, alias: None }
}
pub struct SqlTableBindingBuilder {
    table: SqlTableName,
    alias: Option<SqlAlias>,
}
impl SqlTableBindingBuilder {
    pub fn with_alias(mut self, alias: SqlAlias) -> Self {
        self.alias = Some(alias);
        self
    }
    pub fn build(self) -> SqlTableBinding {
        SqlTableBinding::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_TABLE_BINDING,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.alias
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_table_col_reference(
    table: SqlTableName,
    dot_token: SyntaxToken,
    name: SqlName,
) -> SqlTableColReference {
    SqlTableColReference::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TABLE_COL_REFERENCE,
        [
            Some(SyntaxElement::Node(table.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn sql_table_name(name: AnySqlName) -> SqlTableNameBuilder {
    SqlTableNameBuilder { name, schema: None }
}
pub struct SqlTableNameBuilder {
    name: AnySqlName,
    schema: Option<SqlShemaName>,
}
impl SqlTableNameBuilder {
    pub fn with_schema(mut self, schema: SqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn build(self) -> SqlTableName {
        SqlTableName::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_TABLE_NAME,
            [
                self.schema
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.name.into_syntax())),
            ],
        ))
    }
}
pub fn sql_table_star(table: SqlTableName, dot_token: SyntaxToken, star: SqlStar) -> SqlTableStar {
    SqlTableStar::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TABLE_STAR,
        [
            Some(SyntaxElement::Node(table.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(star.into_syntax())),
        ],
    ))
}
pub fn sql_tilde_name(value_token: SyntaxToken) -> SqlTildeName {
    SqlTildeName::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TILDE_NAME,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_time_zone_modifier(
    with_or_without_token: SyntaxToken,
    time_token: SyntaxToken,
    zone_token: SyntaxToken,
) -> SqlTimeZoneModifier {
    SqlTimeZoneModifier::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TIME_ZONE_MODIFIER,
        [
            Some(SyntaxElement::Token(with_or_without_token)),
            Some(SyntaxElement::Token(time_token)),
            Some(SyntaxElement::Token(zone_token)),
        ],
    ))
}
pub fn sql_type_arguments(
    l_paren_token: SyntaxToken,
    items: SqlTypeArgumentList,
    r_paren_token: SyntaxToken,
) -> SqlTypeArguments {
    SqlTypeArguments::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TYPE_ARGUMENTS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_type_name(name_token: SyntaxToken) -> SqlTypeNameBuilder {
    SqlTypeNameBuilder {
        name_token,
        args: None,
        modifier: None,
        array_suffix: None,
    }
}
pub struct SqlTypeNameBuilder {
    name_token: SyntaxToken,
    args: Option<SqlTypeArguments>,
    modifier: Option<AnySqlTypeModifier>,
    array_suffix: Option<AnySqlTypeArraySuffix>,
}
impl SqlTypeNameBuilder {
    pub fn with_args(mut self, args: SqlTypeArguments) -> Self {
        self.args = Some(args);
        self
    }
    pub fn with_modifier(mut self, modifier: AnySqlTypeModifier) -> Self {
        self.modifier = Some(modifier);
        self
    }
    pub fn with_array_suffix(mut self, array_suffix: AnySqlTypeArraySuffix) -> Self {
        self.array_suffix = Some(array_suffix);
        self
    }
    pub fn build(self) -> SqlTypeName {
        SqlTypeName::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_TYPE_NAME,
            [
                Some(SyntaxElement::Token(self.name_token)),
                self.args
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.modifier
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.array_suffix
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_unary_expression(
    operator_token_token: SyntaxToken,
    expression: AnySqlExpression,
) -> SqlUnaryExpression {
    SqlUnaryExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_UNARY_EXPRESSION,
        [
            Some(SyntaxElement::Token(operator_token_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn sql_update_from_clause(
    from_token: SyntaxToken,
    items: SqlFromItemList,
) -> SqlUpdateFromClause {
    SqlUpdateFromClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_UPDATE_FROM_CLAUSE,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_update_statement(
    update_token: SyntaxToken,
    table: SqlTableBinding,
    set_clause: SqlSetClause,
) -> SqlUpdateStatementBuilder {
    SqlUpdateStatementBuilder {
        update_token,
        table,
        set_clause,
        with_clause: None,
        from_clause: None,
        where_clause: None,
        returning_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlUpdateStatementBuilder {
    update_token: SyntaxToken,
    table: SqlTableBinding,
    set_clause: SqlSetClause,
    with_clause: Option<SqlWithClause>,
    from_clause: Option<SqlUpdateFromClause>,
    where_clause: Option<SqlWhereClause>,
    returning_clause: Option<PsqlReturningClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlUpdateStatementBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
        self
    }
    pub fn with_from_clause(mut self, from_clause: SqlUpdateFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_returning_clause(mut self, returning_clause: PsqlReturningClause) -> Self {
        self.returning_clause = Some(returning_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlUpdateStatement {
        SqlUpdateStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_UPDATE_STATEMENT,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.update_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                Some(SyntaxElement::Node(self.set_clause.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.returning_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_values_clause(
    values_token: SyntaxToken,
    rows: SqlValuesRowList,
) -> SqlValuesClauseBuilder {
    SqlValuesClauseBuilder {
        values_token,
        rows,
        with_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlValuesClauseBuilder {
    values_token: SyntaxToken,
    rows: SqlValuesRowList,
    with_clause: Option<SqlWithClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlValuesClauseBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlValuesClause {
        SqlValuesClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_VALUES_CLAUSE,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.values_token)),
                Some(SyntaxElement::Node(self.rows.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_values_row(
    l_paren_token: SyntaxToken,
    items: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> SqlValuesRow {
    SqlValuesRow::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_VALUES_ROW,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn sql_varying_modifier(varying_token: SyntaxToken) -> SqlVaryingModifier {
    SqlVaryingModifier::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_VARYING_MODIFIER,
        [Some(SyntaxElement::Token(varying_token))],
    ))
}
pub fn sql_where_clause(where_token: SyntaxToken, condition: AnySqlExpression) -> SqlWhereClause {
    SqlWhereClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WHERE_CLAUSE,
        [
            Some(SyntaxElement::Token(where_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
        ],
    ))
}
pub fn sql_window_function_expression(
    call: SqlCallExpression,
    over_token: SyntaxToken,
    window: SqlWindowSpecification,
) -> SqlWindowFunctionExpression {
    SqlWindowFunctionExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_FUNCTION_EXPRESSION,
        [
            Some(SyntaxElement::Node(call.into_syntax())),
            Some(SyntaxElement::Token(over_token)),
            Some(SyntaxElement::Node(window.into_syntax())),
        ],
    ))
}
pub fn sql_window_partition_by_clause(
    partition_by_token: SyntaxToken,
    items: SqlWindowPartitionByItemList,
) -> SqlWindowPartitionByClause {
    SqlWindowPartitionByClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_PARTITION_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(partition_by_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
        ],
    ))
}
pub fn sql_window_specification(
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
) -> SqlWindowSpecificationBuilder {
    SqlWindowSpecificationBuilder {
        l_paren_token,
        r_paren_token,
        partition_by_clause: None,
        order_by_clause: None,
    }
}
pub struct SqlWindowSpecificationBuilder {
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    partition_by_clause: Option<SqlWindowPartitionByClause>,
    order_by_clause: Option<SqlOrderByClause>,
}
impl SqlWindowSpecificationBuilder {
    pub fn with_partition_by_clause(
        mut self,
        partition_by_clause: SqlWindowPartitionByClause,
    ) -> Self {
        self.partition_by_clause = Some(partition_by_clause);
        self
    }
    pub fn with_order_by_clause(mut self, order_by_clause: SqlOrderByClause) -> Self {
        self.order_by_clause = Some(order_by_clause);
        self
    }
    pub fn build(self) -> SqlWindowSpecification {
        SqlWindowSpecification::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_WINDOW_SPECIFICATION,
            [
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.partition_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.order_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn sql_with_clause(
    with_token: SyntaxToken,
    ctes: SqlCteDefinitionList,
) -> SqlWithClauseBuilder {
    SqlWithClauseBuilder {
        with_token,
        ctes,
        recursive_token: None,
    }
}
pub struct SqlWithClauseBuilder {
    with_token: SyntaxToken,
    ctes: SqlCteDefinitionList,
    recursive_token: Option<SyntaxToken>,
}
impl SqlWithClauseBuilder {
    pub fn with_recursive_token(mut self, recursive_token: SyntaxToken) -> Self {
        self.recursive_token = Some(recursive_token);
        self
    }
    pub fn build(self) -> SqlWithClause {
        SqlWithClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_WITH_CLAUSE,
            [
                Some(SyntaxElement::Token(self.with_token)),
                self.recursive_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ctes.into_syntax())),
            ],
        ))
    }
}
pub fn psql_function_option_list<I>(items: I) -> PsqlFunctionOptionList
where
    I: IntoIterator<Item = AnySqlFunctionOption>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlFunctionOptionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_FUNCTION_OPTION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_function_parameter_list<I, S>(items: I, separators: S) -> PsqlFunctionParameterList
where
    I: IntoIterator<Item = PsqlFunctionParameter>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlFunctionParameterList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_FUNCTION_PARAMETER_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_returns_table_column_list<I, S>(items: I, separators: S) -> PsqlReturnsTableColumnList
where
    I: IntoIterator<Item = PsqlReturnsTableColumn>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlReturnsTableColumnList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_trigger_event_list<I>(items: I) -> PsqlTriggerEventList
where
    I: IntoIterator<Item = PsqlTriggerEvent>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlTriggerEventList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_EVENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_trigger_referencing_item_list<I>(items: I) -> PsqlTriggerReferencingItemList
where
    I: IntoIterator<Item = PsqlTriggerReferencingItem>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlTriggerReferencingItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_type_name_list<I, S>(items: I, separators: S) -> PsqlTypeNameList
where
    I: IntoIterator<Item = SqlTypeName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlTypeNameList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TYPE_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn psql_view_option_list<I, S>(items: I, separators: S) -> PsqlViewOptionList
where
    I: IntoIterator<Item = PsqlViewOption>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlViewOptionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_VIEW_OPTION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_alias_column_definition_list<I, S>(
    items: I,
    separators: S,
) -> SqlAliasColumnDefinitionList
where
    I: IntoIterator<Item = SqlAliasColumnDefinition>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlAliasColumnDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ALIAS_COLUMN_DEFINITION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_case_when_clause_list<I>(items: I) -> SqlCaseWhenClauseList
where
    I: IntoIterator<Item = SqlCaseWhenClause>,
    I::IntoIter: ExactSizeIterator,
{
    SqlCaseWhenClauseList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CASE_WHEN_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn sql_column_definition_list<I, S>(items: I, separators: S) -> SqlColumnDefinitionList
where
    I: IntoIterator<Item = SqlColumnDefinition>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlColumnDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_DEFINITION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_column_name_list<I, S>(items: I, separators: S) -> SqlColumnNameList
where
    I: IntoIterator<Item = SqlName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlColumnNameList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_cte_definition_list<I, S>(items: I, separators: S) -> SqlCteDefinitionList
where
    I: IntoIterator<Item = SqlCteDefinition>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlCteDefinitionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CTE_DEFINITION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_expression_list<I, S>(items: I, separators: S) -> SqlExpressionList
where
    I: IntoIterator<Item = AnySqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlExpressionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_EXPRESSION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_from_item_list<I, S>(items: I, separators: S) -> SqlFromItemList
where
    I: IntoIterator<Item = SqlFromItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlFromItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FROM_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_grantee_list<I, S>(items: I, separators: S) -> SqlGranteeList
where
    I: IntoIterator<Item = SqlName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlGranteeList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_GRANTEE_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_group_by_item_list<I, S>(items: I, separators: S) -> SqlGroupByItemList
where
    I: IntoIterator<Item = AnySqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlGroupByItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_GROUP_BY_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_join_clause_list<I>(items: I) -> SqlJoinClauseList
where
    I: IntoIterator<Item = SqlJoinClause>,
    I::IntoIter: ExactSizeIterator,
{
    SqlJoinClauseList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_JOIN_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn sql_order_by_expression_list<I, S>(items: I, separators: S) -> SqlOrderByExpressionList
where
    I: IntoIterator<Item = SqlOrderByExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlOrderByExpressionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ORDER_BY_EXPRESSION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_select_item_list<I, S>(items: I, separators: S) -> SqlSelectItemList
where
    I: IntoIterator<Item = AnySqlSelectItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlSelectItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SELECT_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_set_item_list<I, S>(items: I, separators: S) -> SqlSetItemList
where
    I: IntoIterator<Item = SqlSetItem>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlSetItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_set_operation_list<I>(items: I) -> SqlSetOperationList
where
    I: IntoIterator<Item = SqlSetOperation>,
    I::IntoIter: ExactSizeIterator,
{
    SqlSetOperationList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_OPERATION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn sql_statement_list<I>(items: I) -> SqlStatementList
where
    I: IntoIterator<Item = AnySqlStatement>,
    I::IntoIter: ExactSizeIterator,
{
    SqlStatementList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn sql_table_name_list<I, S>(items: I, separators: S) -> SqlTableNameList
where
    I: IntoIterator<Item = SqlTableName>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlTableNameList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TABLE_NAME_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_type_argument_list<I, S>(items: I, separators: S) -> SqlTypeArgumentList
where
    I: IntoIterator<Item = SqlNumberLiteralExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlTypeArgumentList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TYPE_ARGUMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_values_row_list<I, S>(items: I, separators: S) -> SqlValuesRowList
where
    I: IntoIterator<Item = SqlValuesRow>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlValuesRowList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_VALUES_ROW_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_window_partition_by_item_list<I, S>(
    items: I,
    separators: S,
) -> SqlWindowPartitionByItemList
where
    I: IntoIterator<Item = AnySqlExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlWindowPartitionByItemList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_PARTITION_BY_ITEM_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_bogus<I>(slots: I) -> SqlBogus
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogus::unwrap_cast(SyntaxNode::new_detached(SqlSyntaxKind::SQL_BOGUS, slots))
}
pub fn sql_bogus_assignment<I>(slots: I) -> SqlBogusAssignment
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusAssignment::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_ASSIGNMENT,
        slots,
    ))
}
pub fn sql_bogus_binding<I>(slots: I) -> SqlBogusBinding
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusBinding::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_BINDING,
        slots,
    ))
}
pub fn sql_bogus_expression<I>(slots: I) -> SqlBogusExpression
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_EXPRESSION,
        slots,
    ))
}
pub fn sql_bogus_member<I>(slots: I) -> SqlBogusMember
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusMember::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_MEMBER,
        slots,
    ))
}
pub fn sql_bogus_parameter<I>(slots: I) -> SqlBogusParameter
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusParameter::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_PARAMETER,
        slots,
    ))
}
pub fn sql_bogus_statement<I>(slots: I) -> SqlBogusStatement
where
    I: IntoIterator<Item = Option<SyntaxElement>>,
    I::IntoIter: ExactSizeIterator,
{
    SqlBogusStatement::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_BOGUS_STATEMENT,
        slots,
    ))
}
