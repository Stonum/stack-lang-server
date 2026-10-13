//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(clippy::redundant_closure)]
#![allow(clippy::too_many_arguments)]
use biome_rowan::AstNode;
use sql_syntax::{
    SqlSyntaxElement as SyntaxElement, SqlSyntaxNode as SyntaxNode, SqlSyntaxToken as SyntaxToken,
    *,
};
pub fn psql_alter_column_using(
    using_token: SyntaxToken,
    expression: AnySqlExpression,
) -> PsqlAlterColumnUsing {
    PsqlAlterColumnUsing::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_ALTER_COLUMN_USING,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
        ],
    ))
}
pub fn psql_analyze_statement(
    analyze_token: SyntaxToken,
    tables: PsqlTableAndColumnsList,
) -> PsqlAnalyzeStatementBuilder {
    PsqlAnalyzeStatementBuilder {
        analyze_token,
        tables,
        verbose_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlAnalyzeStatementBuilder {
    analyze_token: SyntaxToken,
    tables: PsqlTableAndColumnsList,
    verbose_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlAnalyzeStatementBuilder {
    pub fn with_verbose_token(mut self, verbose_token: SyntaxToken) -> Self {
        self.verbose_token = Some(verbose_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlAnalyzeStatement {
        PsqlAnalyzeStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_ANALYZE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.analyze_token)),
                self.verbose_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.tables.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
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
pub fn psql_attach_partition_action(
    attach_token: SyntaxToken,
    partition_token: SyntaxToken,
    partition: SqlTableName,
    bound: AnyPsqlPartitionBound,
) -> PsqlAttachPartitionAction {
    PsqlAttachPartitionAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_ATTACH_PARTITION_ACTION,
        [
            Some(SyntaxElement::Token(attach_token)),
            Some(SyntaxElement::Token(partition_token)),
            Some(SyntaxElement::Node(partition.into_syntax())),
            Some(SyntaxElement::Node(bound.into_syntax())),
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
pub fn psql_copy_file(file: SqlStringLiteralExpression) -> PsqlCopyFileBuilder {
    PsqlCopyFileBuilder {
        file,
        program_token: None,
    }
}
pub struct PsqlCopyFileBuilder {
    file: SqlStringLiteralExpression,
    program_token: Option<SyntaxToken>,
}
impl PsqlCopyFileBuilder {
    pub fn with_program_token(mut self, program_token: SyntaxToken) -> Self {
        self.program_token = Some(program_token);
        self
    }
    pub fn build(self) -> PsqlCopyFile {
        PsqlCopyFile::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_COPY_FILE,
            [
                self.program_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.file.into_syntax())),
            ],
        ))
    }
}
pub fn psql_copy_option(name: SqlName) -> PsqlCopyOptionBuilder {
    PsqlCopyOptionBuilder { name, value: None }
}
pub struct PsqlCopyOptionBuilder {
    name: SqlName,
    value: Option<AnySqlExpression>,
}
impl PsqlCopyOptionBuilder {
    pub fn with_value(mut self, value: AnySqlExpression) -> Self {
        self.value = Some(value);
        self
    }
    pub fn build(self) -> PsqlCopyOption {
        PsqlCopyOption::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_COPY_OPTION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.value
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_copy_options(
    l_paren_token: SyntaxToken,
    options: PsqlCopyOptionList,
    r_paren_token: SyntaxToken,
) -> PsqlCopyOptionsBuilder {
    PsqlCopyOptionsBuilder {
        l_paren_token,
        options,
        r_paren_token,
        with_token: None,
    }
}
pub struct PsqlCopyOptionsBuilder {
    l_paren_token: SyntaxToken,
    options: PsqlCopyOptionList,
    r_paren_token: SyntaxToken,
    with_token: Option<SyntaxToken>,
}
impl PsqlCopyOptionsBuilder {
    pub fn with_with_token(mut self, with_token: SyntaxToken) -> Self {
        self.with_token = Some(with_token);
        self
    }
    pub fn build(self) -> PsqlCopyOptions {
        PsqlCopyOptions::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_COPY_OPTIONS,
            [
                self.with_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.options.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn psql_copy_statement(
    copy_token: SyntaxToken,
    source: AnyPsqlCopySource,
    direction_token: SyntaxToken,
    target: AnyPsqlCopyTarget,
) -> PsqlCopyStatementBuilder {
    PsqlCopyStatementBuilder {
        copy_token,
        source,
        direction_token,
        target,
        options: None,
        semicolon_token: None,
    }
}
pub struct PsqlCopyStatementBuilder {
    copy_token: SyntaxToken,
    source: AnyPsqlCopySource,
    direction_token: SyntaxToken,
    target: AnyPsqlCopyTarget,
    options: Option<PsqlCopyOptions>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCopyStatementBuilder {
    pub fn with_options(mut self, options: PsqlCopyOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCopyStatement {
        PsqlCopyStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_COPY_STATEMENT,
            [
                Some(SyntaxElement::Token(self.copy_token)),
                Some(SyntaxElement::Node(self.source.into_syntax())),
                Some(SyntaxElement::Token(self.direction_token)),
                Some(SyntaxElement::Node(self.target.into_syntax())),
                self.options
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_copy_stdio(value_token: SyntaxToken) -> PsqlCopyStdio {
    PsqlCopyStdio::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_COPY_STDIO,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn psql_copy_table_source(table: SqlTableName) -> PsqlCopyTableSourceBuilder {
    PsqlCopyTableSourceBuilder {
        table,
        columns: None,
    }
}
pub struct PsqlCopyTableSourceBuilder {
    table: SqlTableName,
    columns: Option<SqlColumnList>,
}
impl PsqlCopyTableSourceBuilder {
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn build(self) -> PsqlCopyTableSource {
        PsqlCopyTableSource::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_COPY_TABLE_SOURCE,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_create_extension_statement(
    create_token: SyntaxToken,
    extension_token: SyntaxToken,
    name: SqlName,
) -> PsqlCreateExtensionStatementBuilder {
    PsqlCreateExtensionStatementBuilder {
        create_token,
        extension_token,
        name,
        if_token: None,
        not_token: None,
        exists_token: None,
        schema_clause: None,
        cascade_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreateExtensionStatementBuilder {
    create_token: SyntaxToken,
    extension_token: SyntaxToken,
    name: SqlName,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    schema_clause: Option<PsqlExtensionSchemaClause>,
    cascade_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateExtensionStatementBuilder {
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
    pub fn with_schema_clause(mut self, schema_clause: PsqlExtensionSchemaClause) -> Self {
        self.schema_clause = Some(schema_clause);
        self
    }
    pub fn with_cascade_token(mut self, cascade_token: SyntaxToken) -> Self {
        self.cascade_token = Some(cascade_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreateExtensionStatement {
        PsqlCreateExtensionStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_EXTENSION_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                Some(SyntaxElement::Token(self.extension_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.schema_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.cascade_token.map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn psql_create_materialized_view_statement(
    create_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: AnySqlCreateTableAsQuery,
) -> PsqlCreateMaterializedViewStatementBuilder {
    PsqlCreateMaterializedViewStatementBuilder {
        create_token,
        materialized_token,
        view_token,
        name,
        as_token,
        query,
        if_token: None,
        not_token: None,
        exists_token: None,
        columns: None,
        options: None,
        with_data: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreateMaterializedViewStatementBuilder {
    create_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: AnySqlCreateTableAsQuery,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    columns: Option<SqlColumnList>,
    options: Option<PsqlViewOptions>,
    with_data: Option<PsqlWithDataClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateMaterializedViewStatementBuilder {
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
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn with_options(mut self, options: PsqlViewOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn with_with_data(mut self, with_data: PsqlWithDataClause) -> Self {
        self.with_data = Some(with_data);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreateMaterializedViewStatement {
        PsqlCreateMaterializedViewStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_MATERIALIZED_VIEW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                Some(SyntaxElement::Token(self.materialized_token)),
                Some(SyntaxElement::Token(self.view_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.options
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                self.with_data
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
pub fn psql_create_table_partition_of_statement(
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    partition_token: SyntaxToken,
    of_token: SyntaxToken,
    parent: SqlTableName,
    bound: AnyPsqlPartitionBound,
) -> PsqlCreateTablePartitionOfStatementBuilder {
    PsqlCreateTablePartitionOfStatementBuilder {
        create_token,
        table_token,
        name,
        partition_token,
        of_token,
        parent,
        bound,
        persistence_token: None,
        if_token: None,
        not_token: None,
        exists_token: None,
        partition_by: None,
        semicolon_token: None,
    }
}
pub struct PsqlCreateTablePartitionOfStatementBuilder {
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    partition_token: SyntaxToken,
    of_token: SyntaxToken,
    parent: SqlTableName,
    bound: AnyPsqlPartitionBound,
    persistence_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    partition_by: Option<PsqlPartitionByClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateTablePartitionOfStatementBuilder {
    pub fn with_persistence_token(mut self, persistence_token: SyntaxToken) -> Self {
        self.persistence_token = Some(persistence_token);
        self
    }
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
    pub fn with_partition_by(mut self, partition_by: PsqlPartitionByClause) -> Self {
        self.partition_by = Some(partition_by);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlCreateTablePartitionOfStatement {
        PsqlCreateTablePartitionOfStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_CREATE_TABLE_PARTITION_OF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                self.persistence_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.partition_token)),
                Some(SyntaxElement::Token(self.of_token)),
                Some(SyntaxElement::Node(self.parent.into_syntax())),
                Some(SyntaxElement::Node(self.bound.into_syntax())),
                self.partition_by
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
        of_token: None,
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
    of_token: Option<SyntaxToken>,
    referencing_clause: Option<PsqlTriggerReferencingClause>,
    for_each_clause: Option<PsqlTriggerForEachClause>,
    when_clause: Option<PsqlTriggerWhenClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlCreateTriggerStatementBuilder {
    pub fn with_of_token(mut self, of_token: SyntaxToken) -> Self {
        self.of_token = Some(of_token);
        self
    }
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
                self.of_token.map(|token| SyntaxElement::Token(token)),
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
pub fn psql_detach_partition_action(
    detach_token: SyntaxToken,
    partition_token: SyntaxToken,
    partition: SqlTableName,
) -> PsqlDetachPartitionActionBuilder {
    PsqlDetachPartitionActionBuilder {
        detach_token,
        partition_token,
        partition,
        mode_token: None,
    }
}
pub struct PsqlDetachPartitionActionBuilder {
    detach_token: SyntaxToken,
    partition_token: SyntaxToken,
    partition: SqlTableName,
    mode_token: Option<SyntaxToken>,
}
impl PsqlDetachPartitionActionBuilder {
    pub fn with_mode_token(mut self, mode_token: SyntaxToken) -> Self {
        self.mode_token = Some(mode_token);
        self
    }
    pub fn build(self) -> PsqlDetachPartitionAction {
        PsqlDetachPartitionAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DETACH_PARTITION_ACTION,
            [
                Some(SyntaxElement::Token(self.detach_token)),
                Some(SyntaxElement::Token(self.partition_token)),
                Some(SyntaxElement::Node(self.partition.into_syntax())),
                self.mode_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn psql_do_statement(
    do_token: SyntaxToken,
    body: AnyPsqlRoutineBody,
) -> PsqlDoStatementBuilder {
    PsqlDoStatementBuilder {
        do_token,
        body,
        leading_language: None,
        trailing_language: None,
        semicolon_token: None,
    }
}
pub struct PsqlDoStatementBuilder {
    do_token: SyntaxToken,
    body: AnyPsqlRoutineBody,
    leading_language: Option<PsqlLanguageOption>,
    trailing_language: Option<PsqlLanguageOption>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDoStatementBuilder {
    pub fn with_leading_language(mut self, leading_language: PsqlLanguageOption) -> Self {
        self.leading_language = Some(leading_language);
        self
    }
    pub fn with_trailing_language(mut self, trailing_language: PsqlLanguageOption) -> Self {
        self.trailing_language = Some(trailing_language);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlDoStatement {
        PsqlDoStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DO_STATEMENT,
            [
                Some(SyntaxElement::Token(self.do_token)),
                self.leading_language
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.body.into_syntax())),
                self.trailing_language
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn psql_drop_extension_statement(
    drop_token: SyntaxToken,
    extension_token: SyntaxToken,
    extensions: SqlTableNameList,
) -> PsqlDropExtensionStatementBuilder {
    PsqlDropExtensionStatementBuilder {
        drop_token,
        extension_token,
        extensions,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlDropExtensionStatementBuilder {
    drop_token: SyntaxToken,
    extension_token: SyntaxToken,
    extensions: SqlTableNameList,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDropExtensionStatementBuilder {
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
    pub fn build(self) -> PsqlDropExtensionStatement {
        PsqlDropExtensionStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DROP_EXTENSION_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.extension_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.extensions.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
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
pub fn psql_drop_materialized_view_statement(
    drop_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    views: SqlTableNameList,
) -> PsqlDropMaterializedViewStatementBuilder {
    PsqlDropMaterializedViewStatementBuilder {
        drop_token,
        materialized_token,
        view_token,
        views,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlDropMaterializedViewStatementBuilder {
    drop_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    views: SqlTableNameList,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlDropMaterializedViewStatementBuilder {
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
    pub fn build(self) -> PsqlDropMaterializedViewStatement {
        PsqlDropMaterializedViewStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_DROP_MATERIALIZED_VIEW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.materialized_token)),
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
pub fn psql_extension_schema_clause(
    schema_token: SyntaxToken,
    schema: SqlName,
) -> PsqlExtensionSchemaClauseBuilder {
    PsqlExtensionSchemaClauseBuilder {
        schema_token,
        schema,
        with_token: None,
    }
}
pub struct PsqlExtensionSchemaClauseBuilder {
    schema_token: SyntaxToken,
    schema: SqlName,
    with_token: Option<SyntaxToken>,
}
impl PsqlExtensionSchemaClauseBuilder {
    pub fn with_with_token(mut self, with_token: SyntaxToken) -> Self {
        self.with_token = Some(with_token);
        self
    }
    pub fn build(self) -> PsqlExtensionSchemaClause {
        PsqlExtensionSchemaClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_EXTENSION_SCHEMA_CLAUSE,
            [
                self.with_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.schema_token)),
                Some(SyntaxElement::Node(self.schema.into_syntax())),
            ],
        ))
    }
}
pub fn psql_extract_expression(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    field: AnySqlExpression,
    from_token: SyntaxToken,
    source: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlExtractExpression {
    PsqlExtractExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_EXTRACT_EXPRESSION,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(field.into_syntax())),
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_field_selection_expression(
    expression: AnySqlExpression,
    dot_token: SyntaxToken,
    field: SqlName,
) -> PsqlFieldSelectionExpression {
    PsqlFieldSelectionExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_FIELD_SELECTION_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(dot_token)),
            Some(SyntaxElement::Node(field.into_syntax())),
        ],
    ))
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
pub fn psql_generated_column_constraint(
    generated_token: SyntaxToken,
    always_token: SyntaxToken,
    as_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    expression: AnySqlExpression,
    r_paren_token: SyntaxToken,
    stored_token: SyntaxToken,
) -> PsqlGeneratedColumnConstraint {
    PsqlGeneratedColumnConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_GENERATED_COLUMN_CONSTRAINT,
        [
            Some(SyntaxElement::Token(generated_token)),
            Some(SyntaxElement::Token(always_token)),
            Some(SyntaxElement::Token(as_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
            Some(SyntaxElement::Token(stored_token)),
        ],
    ))
}
pub fn psql_identity_constraint(
    generated_token: SyntaxToken,
    as_token: SyntaxToken,
    identity_token: SyntaxToken,
) -> PsqlIdentityConstraintBuilder {
    PsqlIdentityConstraintBuilder {
        generated_token,
        as_token,
        identity_token,
        always_token: None,
        by_token: None,
        default_token: None,
        options: None,
    }
}
pub struct PsqlIdentityConstraintBuilder {
    generated_token: SyntaxToken,
    as_token: SyntaxToken,
    identity_token: SyntaxToken,
    always_token: Option<SyntaxToken>,
    by_token: Option<SyntaxToken>,
    default_token: Option<SyntaxToken>,
    options: Option<PsqlSequenceOptions>,
}
impl PsqlIdentityConstraintBuilder {
    pub fn with_always_token(mut self, always_token: SyntaxToken) -> Self {
        self.always_token = Some(always_token);
        self
    }
    pub fn with_by_token(mut self, by_token: SyntaxToken) -> Self {
        self.by_token = Some(by_token);
        self
    }
    pub fn with_default_token(mut self, default_token: SyntaxToken) -> Self {
        self.default_token = Some(default_token);
        self
    }
    pub fn with_options(mut self, options: PsqlSequenceOptions) -> Self {
        self.options = Some(options);
        self
    }
    pub fn build(self) -> PsqlIdentityConstraint {
        PsqlIdentityConstraint::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_IDENTITY_CONSTRAINT,
            [
                Some(SyntaxElement::Token(self.generated_token)),
                self.always_token.map(|token| SyntaxElement::Token(token)),
                self.by_token.map(|token| SyntaxElement::Token(token)),
                self.default_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Token(self.identity_token)),
                self.options
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_index_method_clause(
    using_token: SyntaxToken,
    method: SqlName,
) -> PsqlIndexMethodClause {
    PsqlIndexMethodClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_INDEX_METHOD_CLAUSE,
        [
            Some(SyntaxElement::Token(using_token)),
            Some(SyntaxElement::Node(method.into_syntax())),
        ],
    ))
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
pub fn psql_into_clause(
    into_token: SyntaxToken,
    targets: SqlExpressionList,
) -> PsqlIntoClauseBuilder {
    PsqlIntoClauseBuilder {
        into_token,
        targets,
        strict_token: None,
    }
}
pub struct PsqlIntoClauseBuilder {
    into_token: SyntaxToken,
    targets: SqlExpressionList,
    strict_token: Option<SyntaxToken>,
}
impl PsqlIntoClauseBuilder {
    pub fn with_strict_token(mut self, strict_token: SyntaxToken) -> Self {
        self.strict_token = Some(strict_token);
        self
    }
    pub fn build(self) -> PsqlIntoClause {
        PsqlIntoClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_INTO_CLAUSE,
            [
                Some(SyntaxElement::Token(self.into_token)),
                self.strict_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.targets.into_syntax())),
            ],
        ))
    }
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
pub fn psql_language_option(
    language_token: SyntaxToken,
    name: AnyPsqlLanguageName,
) -> PsqlLanguageOption {
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
pub fn psql_lock_mode(
    in_token: SyntaxToken,
    lead_token: SyntaxToken,
    mode_token: SyntaxToken,
) -> PsqlLockModeBuilder {
    PsqlLockModeBuilder {
        in_token,
        lead_token,
        mode_token,
        middle_token: None,
        tail_token: None,
    }
}
pub struct PsqlLockModeBuilder {
    in_token: SyntaxToken,
    lead_token: SyntaxToken,
    mode_token: SyntaxToken,
    middle_token: Option<SyntaxToken>,
    tail_token: Option<SyntaxToken>,
}
impl PsqlLockModeBuilder {
    pub fn with_middle_token(mut self, middle_token: SyntaxToken) -> Self {
        self.middle_token = Some(middle_token);
        self
    }
    pub fn with_tail_token(mut self, tail_token: SyntaxToken) -> Self {
        self.tail_token = Some(tail_token);
        self
    }
    pub fn build(self) -> PsqlLockMode {
        PsqlLockMode::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_LOCK_MODE,
            [
                Some(SyntaxElement::Token(self.in_token)),
                Some(SyntaxElement::Token(self.lead_token)),
                self.middle_token.map(|token| SyntaxElement::Token(token)),
                self.tail_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.mode_token)),
            ],
        ))
    }
}
pub fn psql_lock_statement(
    lock_token: SyntaxToken,
    tables: SqlTableNameList,
) -> PsqlLockStatementBuilder {
    PsqlLockStatementBuilder {
        lock_token,
        tables,
        table_token: None,
        only_token: None,
        mode: None,
        nowait_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlLockStatementBuilder {
    lock_token: SyntaxToken,
    tables: SqlTableNameList,
    table_token: Option<SyntaxToken>,
    only_token: Option<SyntaxToken>,
    mode: Option<PsqlLockMode>,
    nowait_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlLockStatementBuilder {
    pub fn with_table_token(mut self, table_token: SyntaxToken) -> Self {
        self.table_token = Some(table_token);
        self
    }
    pub fn with_only_token(mut self, only_token: SyntaxToken) -> Self {
        self.only_token = Some(only_token);
        self
    }
    pub fn with_mode(mut self, mode: PsqlLockMode) -> Self {
        self.mode = Some(mode);
        self
    }
    pub fn with_nowait_token(mut self, nowait_token: SyntaxToken) -> Self {
        self.nowait_token = Some(nowait_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlLockStatement {
        PsqlLockStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_LOCK_STATEMENT,
            [
                Some(SyntaxElement::Token(self.lock_token)),
                self.table_token.map(|token| SyntaxElement::Token(token)),
                self.only_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.tables.into_syntax())),
                self.mode
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.nowait_token.map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_named_argument(
    name: SqlName,
    fat_arrow_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlNamedArgument {
    PsqlNamedArgument::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_NAMED_ARGUMENT,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Token(fat_arrow_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn psql_on_commit_clause(
    on_token: SyntaxToken,
    commit_token: SyntaxToken,
    action_token: SyntaxToken,
) -> PsqlOnCommitClauseBuilder {
    PsqlOnCommitClauseBuilder {
        on_token,
        commit_token,
        action_token,
        rows_token: None,
    }
}
pub struct PsqlOnCommitClauseBuilder {
    on_token: SyntaxToken,
    commit_token: SyntaxToken,
    action_token: SyntaxToken,
    rows_token: Option<SyntaxToken>,
}
impl PsqlOnCommitClauseBuilder {
    pub fn with_rows_token(mut self, rows_token: SyntaxToken) -> Self {
        self.rows_token = Some(rows_token);
        self
    }
    pub fn build(self) -> PsqlOnCommitClause {
        PsqlOnCommitClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_ON_COMMIT_CLAUSE,
            [
                Some(SyntaxElement::Token(self.on_token)),
                Some(SyntaxElement::Token(self.commit_token)),
                Some(SyntaxElement::Token(self.action_token)),
                self.rows_token.map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn psql_partition_by_clause(
    partition_by_token: SyntaxToken,
    strategy_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    keys: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlPartitionByClause {
    PsqlPartitionByClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_BY_CLAUSE,
        [
            Some(SyntaxElement::Token(partition_by_token)),
            Some(SyntaxElement::Token(strategy_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(keys.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_partition_default_bound(default_token: SyntaxToken) -> PsqlPartitionDefaultBound {
    PsqlPartitionDefaultBound::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_DEFAULT_BOUND,
        [Some(SyntaxElement::Token(default_token))],
    ))
}
pub fn psql_partition_for_values_bound(
    for_token: SyntaxToken,
    values_token: SyntaxToken,
    values: AnyPsqlPartitionValues,
) -> PsqlPartitionForValuesBound {
    PsqlPartitionForValuesBound::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_FOR_VALUES_BOUND,
        [
            Some(SyntaxElement::Token(for_token)),
            Some(SyntaxElement::Token(values_token)),
            Some(SyntaxElement::Node(values.into_syntax())),
        ],
    ))
}
pub fn psql_partition_hash_values(
    with_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    modulus_token: SyntaxToken,
    modulus: SqlNumberLiteralExpression,
    comma_token: SyntaxToken,
    remainder_token: SyntaxToken,
    remainder: SqlNumberLiteralExpression,
    r_paren_token: SyntaxToken,
) -> PsqlPartitionHashValues {
    PsqlPartitionHashValues::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_HASH_VALUES,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Token(modulus_token)),
            Some(SyntaxElement::Node(modulus.into_syntax())),
            Some(SyntaxElement::Token(comma_token)),
            Some(SyntaxElement::Token(remainder_token)),
            Some(SyntaxElement::Node(remainder.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_partition_in_values(
    in_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    values: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlPartitionInValues {
    PsqlPartitionInValues::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_IN_VALUES,
        [
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(values.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_partition_range_values(
    from_token: SyntaxToken,
    from: PsqlPartitionValueList,
    to_token: SyntaxToken,
    to: PsqlPartitionValueList,
) -> PsqlPartitionRangeValues {
    PsqlPartitionRangeValues::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_RANGE_VALUES,
        [
            Some(SyntaxElement::Token(from_token)),
            Some(SyntaxElement::Node(from.into_syntax())),
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(to.into_syntax())),
        ],
    ))
}
pub fn psql_partition_value_list(
    l_paren_token: SyntaxToken,
    values: SqlExpressionList,
    r_paren_token: SyntaxToken,
) -> PsqlPartitionValueList {
    PsqlPartitionValueList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PARTITION_VALUE_LIST,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(values.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_pl_assignment_statement(
    target: AnySqlExpression,
    operator_token: SyntaxToken,
    value: AnySqlExpression,
    semicolon_token: SyntaxToken,
) -> PsqlPlAssignmentStatement {
    PsqlPlAssignmentStatement::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_ASSIGNMENT_STATEMENT,
        [
            Some(SyntaxElement::Node(target.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
            Some(SyntaxElement::Token(semicolon_token)),
        ],
    ))
}
pub fn psql_pl_block(
    begin_token: SyntaxToken,
    statements: PsqlPlStatementList,
    end_token: SyntaxToken,
) -> PsqlPlBlockBuilder {
    PsqlPlBlockBuilder {
        begin_token,
        statements,
        end_token,
        declare_section: None,
        semicolon_token: None,
    }
}
pub struct PsqlPlBlockBuilder {
    begin_token: SyntaxToken,
    statements: PsqlPlStatementList,
    end_token: SyntaxToken,
    declare_section: Option<PsqlPlDeclareSection>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlPlBlockBuilder {
    pub fn with_declare_section(mut self, declare_section: PsqlPlDeclareSection) -> Self {
        self.declare_section = Some(declare_section);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlPlBlock {
        PsqlPlBlock::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_PL_BLOCK,
            [
                self.declare_section
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.begin_token)),
                Some(SyntaxElement::Node(self.statements.into_syntax())),
                Some(SyntaxElement::Token(self.end_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_pl_body(
    l_quote_token: SyntaxToken,
    block: PsqlPlBlock,
    r_quote_token: SyntaxToken,
) -> PsqlPlBody {
    PsqlPlBody::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_BODY,
        [
            Some(SyntaxElement::Token(l_quote_token)),
            Some(SyntaxElement::Node(block.into_syntax())),
            Some(SyntaxElement::Token(r_quote_token)),
        ],
    ))
}
pub fn psql_pl_declare_section(
    declare_token: SyntaxToken,
    declarations: PsqlPlDeclarationList,
) -> PsqlPlDeclareSection {
    PsqlPlDeclareSection::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_DECLARE_SECTION,
        [
            Some(SyntaxElement::Token(declare_token)),
            Some(SyntaxElement::Node(declarations.into_syntax())),
        ],
    ))
}
pub fn psql_pl_else_clause(
    else_token: SyntaxToken,
    statements: PsqlPlStatementList,
) -> PsqlPlElseClause {
    PsqlPlElseClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_ELSE_CLAUSE,
        [
            Some(SyntaxElement::Token(else_token)),
            Some(SyntaxElement::Node(statements.into_syntax())),
        ],
    ))
}
pub fn psql_pl_elsif_clause(
    elsif_token_token: SyntaxToken,
    condition: AnySqlExpression,
    then_token: SyntaxToken,
    statements: PsqlPlStatementList,
) -> PsqlPlElsifClause {
    PsqlPlElsifClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_ELSIF_CLAUSE,
        [
            Some(SyntaxElement::Token(elsif_token_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
            Some(SyntaxElement::Token(then_token)),
            Some(SyntaxElement::Node(statements.into_syntax())),
        ],
    ))
}
pub fn psql_pl_if_statement(
    if_token: SyntaxToken,
    condition: AnySqlExpression,
    then_token: SyntaxToken,
    statements: PsqlPlStatementList,
    elsif_clauses: PsqlPlElsifClauseList,
    end_token: SyntaxToken,
    end_if_token: SyntaxToken,
    semicolon_token: SyntaxToken,
) -> PsqlPlIfStatementBuilder {
    PsqlPlIfStatementBuilder {
        if_token,
        condition,
        then_token,
        statements,
        elsif_clauses,
        end_token,
        end_if_token,
        semicolon_token,
        else_clause: None,
    }
}
pub struct PsqlPlIfStatementBuilder {
    if_token: SyntaxToken,
    condition: AnySqlExpression,
    then_token: SyntaxToken,
    statements: PsqlPlStatementList,
    elsif_clauses: PsqlPlElsifClauseList,
    end_token: SyntaxToken,
    end_if_token: SyntaxToken,
    semicolon_token: SyntaxToken,
    else_clause: Option<PsqlPlElseClause>,
}
impl PsqlPlIfStatementBuilder {
    pub fn with_else_clause(mut self, else_clause: PsqlPlElseClause) -> Self {
        self.else_clause = Some(else_clause);
        self
    }
    pub fn build(self) -> PsqlPlIfStatement {
        PsqlPlIfStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_PL_IF_STATEMENT,
            [
                Some(SyntaxElement::Token(self.if_token)),
                Some(SyntaxElement::Node(self.condition.into_syntax())),
                Some(SyntaxElement::Token(self.then_token)),
                Some(SyntaxElement::Node(self.statements.into_syntax())),
                Some(SyntaxElement::Node(self.elsif_clauses.into_syntax())),
                self.else_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.end_token)),
                Some(SyntaxElement::Token(self.end_if_token)),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn psql_pl_perform_statement(
    perform_token: SyntaxToken,
    list: SqlSelectItemList,
    semicolon_token: SyntaxToken,
) -> PsqlPlPerformStatementBuilder {
    PsqlPlPerformStatementBuilder {
        perform_token,
        list,
        semicolon_token,
        from_clause: None,
        where_clause: None,
    }
}
pub struct PsqlPlPerformStatementBuilder {
    perform_token: SyntaxToken,
    list: SqlSelectItemList,
    semicolon_token: SyntaxToken,
    from_clause: Option<SqlFromClause>,
    where_clause: Option<SqlWhereClause>,
}
impl PsqlPlPerformStatementBuilder {
    pub fn with_from_clause(mut self, from_clause: SqlFromClause) -> Self {
        self.from_clause = Some(from_clause);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn build(self) -> PsqlPlPerformStatement {
        PsqlPlPerformStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_PL_PERFORM_STATEMENT,
            [
                Some(SyntaxElement::Token(self.perform_token)),
                Some(SyntaxElement::Node(self.list.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn psql_pl_variable_declaration(
    name: SqlName,
    ty: SqlTypeName,
    semicolon_token: SyntaxToken,
) -> PsqlPlVariableDeclarationBuilder {
    PsqlPlVariableDeclarationBuilder {
        name,
        ty,
        semicolon_token,
        constant_token: None,
        collation: None,
        not_null: None,
        default: None,
    }
}
pub struct PsqlPlVariableDeclarationBuilder {
    name: SqlName,
    ty: SqlTypeName,
    semicolon_token: SyntaxToken,
    constant_token: Option<SyntaxToken>,
    collation: Option<SqlCollateConstraint>,
    not_null: Option<SqlNotNullConstraint>,
    default: Option<PsqlPlVariableDefault>,
}
impl PsqlPlVariableDeclarationBuilder {
    pub fn with_constant_token(mut self, constant_token: SyntaxToken) -> Self {
        self.constant_token = Some(constant_token);
        self
    }
    pub fn with_collation(mut self, collation: SqlCollateConstraint) -> Self {
        self.collation = Some(collation);
        self
    }
    pub fn with_not_null(mut self, not_null: SqlNotNullConstraint) -> Self {
        self.not_null = Some(not_null);
        self
    }
    pub fn with_default(mut self, default: PsqlPlVariableDefault) -> Self {
        self.default = Some(default);
        self
    }
    pub fn build(self) -> PsqlPlVariableDeclaration {
        PsqlPlVariableDeclaration::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_PL_VARIABLE_DECLARATION,
            [
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.constant_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.collation
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.not_null
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.default
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.semicolon_token)),
            ],
        ))
    }
}
pub fn psql_pl_variable_default(
    operator_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlPlVariableDefault {
    PsqlPlVariableDefault::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_VARIABLE_DEFAULT,
        [
            Some(SyntaxElement::Token(operator_token)),
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
pub fn psql_position_expression(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    substring: AnySqlExpression,
    in_token: SyntaxToken,
    string: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlPositionExpression {
    PsqlPositionExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_POSITION_EXPRESSION,
        [
            Some(SyntaxElement::Token(name_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(substring.into_syntax())),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Node(string.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
        ],
    ))
}
pub fn psql_refresh_materialized_view_statement(
    refresh_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
) -> PsqlRefreshMaterializedViewStatementBuilder {
    PsqlRefreshMaterializedViewStatementBuilder {
        refresh_token,
        materialized_token,
        view_token,
        name,
        concurrently_token: None,
        with_data: None,
        semicolon_token: None,
    }
}
pub struct PsqlRefreshMaterializedViewStatementBuilder {
    refresh_token: SyntaxToken,
    materialized_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    concurrently_token: Option<SyntaxToken>,
    with_data: Option<PsqlWithDataClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlRefreshMaterializedViewStatementBuilder {
    pub fn with_concurrently_token(mut self, concurrently_token: SyntaxToken) -> Self {
        self.concurrently_token = Some(concurrently_token);
        self
    }
    pub fn with_with_data(mut self, with_data: PsqlWithDataClause) -> Self {
        self.with_data = Some(with_data);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlRefreshMaterializedViewStatement {
        PsqlRefreshMaterializedViewStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_REFRESH_MATERIALIZED_VIEW_STATEMENT,
            [
                Some(SyntaxElement::Token(self.refresh_token)),
                Some(SyntaxElement::Token(self.materialized_token)),
                Some(SyntaxElement::Token(self.view_token)),
                self.concurrently_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.with_data
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn psql_returning_clause(
    returning_token: SyntaxToken,
    items: SqlSelectItemList,
) -> PsqlReturningClauseBuilder {
    PsqlReturningClauseBuilder {
        returning_token,
        items,
        into_clause: None,
    }
}
pub struct PsqlReturningClauseBuilder {
    returning_token: SyntaxToken,
    items: SqlSelectItemList,
    into_clause: Option<PsqlIntoClause>,
}
impl PsqlReturningClauseBuilder {
    pub fn with_into_clause(mut self, into_clause: PsqlIntoClause) -> Self {
        self.into_clause = Some(into_clause);
        self
    }
    pub fn build(self) -> PsqlReturningClause {
        PsqlReturningClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_RETURNING_CLAUSE,
            [
                Some(SyntaxElement::Token(self.returning_token)),
                Some(SyntaxElement::Node(self.items.into_syntax())),
                self.into_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
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
pub fn psql_sequence_option(
    name_token: SyntaxToken,
    value: AnySqlExpression,
) -> PsqlSequenceOptionBuilder {
    PsqlSequenceOptionBuilder {
        name_token,
        value,
        joiner_token: None,
    }
}
pub struct PsqlSequenceOptionBuilder {
    name_token: SyntaxToken,
    value: AnySqlExpression,
    joiner_token: Option<SyntaxToken>,
}
impl PsqlSequenceOptionBuilder {
    pub fn with_joiner_token(mut self, joiner_token: SyntaxToken) -> Self {
        self.joiner_token = Some(joiner_token);
        self
    }
    pub fn build(self) -> PsqlSequenceOption {
        PsqlSequenceOption::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_SEQUENCE_OPTION,
            [
                Some(SyntaxElement::Token(self.name_token)),
                self.joiner_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.value.into_syntax())),
            ],
        ))
    }
}
pub fn psql_sequence_options(
    l_paren_token: SyntaxToken,
    items: PsqlSequenceOptionList,
    r_paren_token: SyntaxToken,
) -> PsqlSequenceOptions {
    PsqlSequenceOptions::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_SEQUENCE_OPTIONS,
        [
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(items.into_syntax())),
            Some(SyntaxElement::Token(r_paren_token)),
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
pub fn psql_table_and_columns(table: SqlTableName) -> PsqlTableAndColumnsBuilder {
    PsqlTableAndColumnsBuilder {
        table,
        columns: None,
    }
}
pub struct PsqlTableAndColumnsBuilder {
    table: SqlTableName,
    columns: Option<SqlColumnList>,
}
impl PsqlTableAndColumnsBuilder {
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn build(self) -> PsqlTableAndColumns {
        PsqlTableAndColumns::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_TABLE_AND_COLUMNS,
            [
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn psql_table_like_element(
    like_token: SyntaxToken,
    source: SqlTableName,
) -> PsqlTableLikeElement {
    PsqlTableLikeElement::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TABLE_LIKE_ELEMENT,
        [
            Some(SyntaxElement::Token(like_token)),
            Some(SyntaxElement::Node(source.into_syntax())),
        ],
    ))
}
pub fn psql_table_query(table_token: SyntaxToken, table: SqlTableName) -> PsqlTableQuery {
    PsqlTableQuery::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TABLE_QUERY,
        [
            Some(SyntaxElement::Token(table_token)),
            Some(SyntaxElement::Node(table.into_syntax())),
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
pub fn psql_trim_expression(
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    from_token: SyntaxToken,
    string: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> PsqlTrimExpressionBuilder {
    PsqlTrimExpressionBuilder {
        name_token,
        l_paren_token,
        from_token,
        string,
        r_paren_token,
        side_token: None,
        characters: None,
    }
}
pub struct PsqlTrimExpressionBuilder {
    name_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    from_token: SyntaxToken,
    string: AnySqlExpression,
    r_paren_token: SyntaxToken,
    side_token: Option<SyntaxToken>,
    characters: Option<AnySqlExpression>,
}
impl PsqlTrimExpressionBuilder {
    pub fn with_side_token(mut self, side_token: SyntaxToken) -> Self {
        self.side_token = Some(side_token);
        self
    }
    pub fn with_characters(mut self, characters: AnySqlExpression) -> Self {
        self.characters = Some(characters);
        self
    }
    pub fn build(self) -> PsqlTrimExpression {
        PsqlTrimExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_TRIM_EXPRESSION,
            [
                Some(SyntaxElement::Token(self.name_token)),
                Some(SyntaxElement::Token(self.l_paren_token)),
                self.side_token.map(|token| SyntaxElement::Token(token)),
                self.characters
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.string.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
            ],
        ))
    }
}
pub fn psql_truncate_identity_clause(
    action_token: SyntaxToken,
    identity_token: SyntaxToken,
) -> PsqlTruncateIdentityClause {
    PsqlTruncateIdentityClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TRUNCATE_IDENTITY_CLAUSE,
        [
            Some(SyntaxElement::Token(action_token)),
            Some(SyntaxElement::Token(identity_token)),
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
pub fn psql_vacuum_statement(
    vacuum_token: SyntaxToken,
    tables: PsqlTableAndColumnsList,
) -> PsqlVacuumStatementBuilder {
    PsqlVacuumStatementBuilder {
        vacuum_token,
        tables,
        full_token: None,
        freeze_token: None,
        verbose_token: None,
        analyze_token: None,
        semicolon_token: None,
    }
}
pub struct PsqlVacuumStatementBuilder {
    vacuum_token: SyntaxToken,
    tables: PsqlTableAndColumnsList,
    full_token: Option<SyntaxToken>,
    freeze_token: Option<SyntaxToken>,
    verbose_token: Option<SyntaxToken>,
    analyze_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl PsqlVacuumStatementBuilder {
    pub fn with_full_token(mut self, full_token: SyntaxToken) -> Self {
        self.full_token = Some(full_token);
        self
    }
    pub fn with_freeze_token(mut self, freeze_token: SyntaxToken) -> Self {
        self.freeze_token = Some(freeze_token);
        self
    }
    pub fn with_verbose_token(mut self, verbose_token: SyntaxToken) -> Self {
        self.verbose_token = Some(verbose_token);
        self
    }
    pub fn with_analyze_token(mut self, analyze_token: SyntaxToken) -> Self {
        self.analyze_token = Some(analyze_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> PsqlVacuumStatement {
        PsqlVacuumStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_VACUUM_STATEMENT,
            [
                Some(SyntaxElement::Token(self.vacuum_token)),
                self.full_token.map(|token| SyntaxElement::Token(token)),
                self.freeze_token.map(|token| SyntaxElement::Token(token)),
                self.verbose_token.map(|token| SyntaxElement::Token(token)),
                self.analyze_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.tables.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn psql_with_data_clause(
    with_token: SyntaxToken,
    data_token: SyntaxToken,
) -> PsqlWithDataClauseBuilder {
    PsqlWithDataClauseBuilder {
        with_token,
        data_token,
        no_token: None,
    }
}
pub struct PsqlWithDataClauseBuilder {
    with_token: SyntaxToken,
    data_token: SyntaxToken,
    no_token: Option<SyntaxToken>,
}
impl PsqlWithDataClauseBuilder {
    pub fn with_no_token(mut self, no_token: SyntaxToken) -> Self {
        self.no_token = Some(no_token);
        self
    }
    pub fn build(self) -> PsqlWithDataClause {
        PsqlWithDataClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::PSQL_WITH_DATA_CLAUSE,
            [
                Some(SyntaxElement::Token(self.with_token)),
                self.no_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.data_token)),
            ],
        ))
    }
}
pub fn sql_add_column_action(
    add_token: SyntaxToken,
    column: SqlColumnDefinition,
) -> SqlAddColumnActionBuilder {
    SqlAddColumnActionBuilder {
        add_token,
        column,
        column_token: None,
        if_token: None,
        not_token: None,
        exists_token: None,
    }
}
pub struct SqlAddColumnActionBuilder {
    add_token: SyntaxToken,
    column: SqlColumnDefinition,
    column_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
}
impl SqlAddColumnActionBuilder {
    pub fn with_column_token(mut self, column_token: SyntaxToken) -> Self {
        self.column_token = Some(column_token);
        self
    }
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
    pub fn build(self) -> SqlAddColumnAction {
        SqlAddColumnAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ADD_COLUMN_ACTION,
            [
                Some(SyntaxElement::Token(self.add_token)),
                self.column_token.map(|token| SyntaxElement::Token(token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.column.into_syntax())),
            ],
        ))
    }
}
pub fn sql_add_constraint_action(
    add_token: SyntaxToken,
    constraint: SqlTableConstraint,
) -> SqlAddConstraintAction {
    SqlAddConstraintAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ADD_CONSTRAINT_ACTION,
        [
            Some(SyntaxElement::Token(add_token)),
            Some(SyntaxElement::Node(constraint.into_syntax())),
        ],
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
pub fn sql_all_triggers_target(scope_token: SyntaxToken) -> SqlAllTriggersTarget {
    SqlAllTriggersTarget::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ALL_TRIGGERS_TARGET,
        [Some(SyntaxElement::Token(scope_token))],
    ))
}
pub fn sql_alter_column_action(
    alter_token: SyntaxToken,
    name: SqlName,
    operation: AnySqlAlterColumnOperation,
) -> SqlAlterColumnActionBuilder {
    SqlAlterColumnActionBuilder {
        alter_token,
        name,
        operation,
        column_token: None,
    }
}
pub struct SqlAlterColumnActionBuilder {
    alter_token: SyntaxToken,
    name: SqlName,
    operation: AnySqlAlterColumnOperation,
    column_token: Option<SyntaxToken>,
}
impl SqlAlterColumnActionBuilder {
    pub fn with_column_token(mut self, column_token: SyntaxToken) -> Self {
        self.column_token = Some(column_token);
        self
    }
    pub fn build(self) -> SqlAlterColumnAction {
        SqlAlterColumnAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALTER_COLUMN_ACTION,
            [
                Some(SyntaxElement::Token(self.alter_token)),
                self.column_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.operation.into_syntax())),
            ],
        ))
    }
}
pub fn sql_alter_column_type(
    type_token: SyntaxToken,
    ty: SqlTypeName,
) -> SqlAlterColumnTypeBuilder {
    SqlAlterColumnTypeBuilder {
        type_token,
        ty,
        set_token: None,
        data_token: None,
        using: None,
    }
}
pub struct SqlAlterColumnTypeBuilder {
    type_token: SyntaxToken,
    ty: SqlTypeName,
    set_token: Option<SyntaxToken>,
    data_token: Option<SyntaxToken>,
    using: Option<PsqlAlterColumnUsing>,
}
impl SqlAlterColumnTypeBuilder {
    pub fn with_set_token(mut self, set_token: SyntaxToken) -> Self {
        self.set_token = Some(set_token);
        self
    }
    pub fn with_data_token(mut self, data_token: SyntaxToken) -> Self {
        self.data_token = Some(data_token);
        self
    }
    pub fn with_using(mut self, using: PsqlAlterColumnUsing) -> Self {
        self.using = Some(using);
        self
    }
    pub fn build(self) -> SqlAlterColumnType {
        SqlAlterColumnType::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALTER_COLUMN_TYPE,
            [
                self.set_token.map(|token| SyntaxElement::Token(token)),
                self.data_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.type_token)),
                Some(SyntaxElement::Node(self.ty.into_syntax())),
                self.using
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_alter_index_statement(
    alter_token: SyntaxToken,
    index_token: SyntaxToken,
    name: SqlTableName,
    action: SqlRenameTableAction,
) -> SqlAlterIndexStatementBuilder {
    SqlAlterIndexStatementBuilder {
        alter_token,
        index_token,
        name,
        action,
        if_token: None,
        exists_token: None,
        semicolon_token: None,
    }
}
pub struct SqlAlterIndexStatementBuilder {
    alter_token: SyntaxToken,
    index_token: SyntaxToken,
    name: SqlTableName,
    action: SqlRenameTableAction,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlAlterIndexStatementBuilder {
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
    pub fn build(self) -> SqlAlterIndexStatement {
        SqlAlterIndexStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALTER_INDEX_STATEMENT,
            [
                Some(SyntaxElement::Token(self.alter_token)),
                Some(SyntaxElement::Token(self.index_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.action.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_alter_table_statement(
    alter_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    actions: SqlAlterTableActionList,
) -> SqlAlterTableStatementBuilder {
    SqlAlterTableStatementBuilder {
        alter_token,
        table_token,
        name,
        actions,
        if_token: None,
        exists_token: None,
        only_token: None,
        semicolon_token: None,
    }
}
pub struct SqlAlterTableStatementBuilder {
    alter_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    actions: SqlAlterTableActionList,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    only_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlAlterTableStatementBuilder {
    pub fn with_if_token(mut self, if_token: SyntaxToken) -> Self {
        self.if_token = Some(if_token);
        self
    }
    pub fn with_exists_token(mut self, exists_token: SyntaxToken) -> Self {
        self.exists_token = Some(exists_token);
        self
    }
    pub fn with_only_token(mut self, only_token: SyntaxToken) -> Self {
        self.only_token = Some(only_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlAlterTableStatement {
        SqlAlterTableStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ALTER_TABLE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.alter_token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                self.only_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Node(self.actions.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_alter_trigger_action(
    action_token: SyntaxToken,
    trigger_token: SyntaxToken,
    target: AnySqlTriggerTarget,
) -> SqlAlterTriggerAction {
    SqlAlterTriggerAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ALTER_TRIGGER_ACTION,
        [
            Some(SyntaxElement::Token(action_token)),
            Some(SyntaxElement::Token(trigger_token)),
            Some(SyntaxElement::Node(target.into_syntax())),
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
pub fn sql_begin_statement(kind_token: SyntaxToken) -> SqlBeginStatementBuilder {
    SqlBeginStatementBuilder {
        kind_token,
        transaction_kind_token: None,
        semicolon_token: None,
    }
}
pub struct SqlBeginStatementBuilder {
    kind_token: SyntaxToken,
    transaction_kind_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlBeginStatementBuilder {
    pub fn with_transaction_kind_token(mut self, transaction_kind_token: SyntaxToken) -> Self {
        self.transaction_kind_token = Some(transaction_kind_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlBeginStatement {
        SqlBeginStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_BEGIN_STATEMENT,
            [
                Some(SyntaxElement::Token(self.kind_token)),
                self.transaction_kind_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
    arguments: SqlCallArgumentList,
    r_paren_token: SyntaxToken,
) -> SqlCallExpressionBuilder {
    SqlCallExpressionBuilder {
        name,
        l_paren_token,
        arguments,
        r_paren_token,
        schema: None,
        quantifier_token: None,
        order_by: None,
        filter_clause: None,
    }
}
pub struct SqlCallExpressionBuilder {
    name: AnySqlName,
    l_paren_token: SyntaxToken,
    arguments: SqlCallArgumentList,
    r_paren_token: SyntaxToken,
    schema: Option<SqlShemaName>,
    quantifier_token: Option<SyntaxToken>,
    order_by: Option<SqlOrderByClause>,
    filter_clause: Option<PsqlFilterClause>,
}
impl SqlCallExpressionBuilder {
    pub fn with_schema(mut self, schema: SqlShemaName) -> Self {
        self.schema = Some(schema);
        self
    }
    pub fn with_quantifier_token(mut self, quantifier_token: SyntaxToken) -> Self {
        self.quantifier_token = Some(quantifier_token);
        self
    }
    pub fn with_order_by(mut self, order_by: SqlOrderByClause) -> Self {
        self.order_by = Some(order_by);
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
                self.quantifier_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.arguments.into_syntax())),
                self.order_by
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.filter_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_call_statement(
    call_token: SyntaxToken,
    procedure: SqlCallExpression,
) -> SqlCallStatementBuilder {
    SqlCallStatementBuilder {
        call_token,
        procedure,
        semicolon_token: None,
    }
}
pub struct SqlCallStatementBuilder {
    call_token: SyntaxToken,
    procedure: SqlCallExpression,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCallStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCallStatement {
        SqlCallStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CALL_STATEMENT,
            [
                Some(SyntaxElement::Token(self.call_token)),
                Some(SyntaxElement::Node(self.procedure.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
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
pub fn sql_check_constraint(
    check_token: SyntaxToken,
    l_paren_token: SyntaxToken,
    condition: AnySqlExpression,
    r_paren_token: SyntaxToken,
) -> SqlCheckConstraint {
    SqlCheckConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CHECK_CONSTRAINT,
        [
            Some(SyntaxElement::Token(check_token)),
            Some(SyntaxElement::Token(l_paren_token)),
            Some(SyntaxElement::Node(condition.into_syntax())),
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
pub fn sql_collate_constraint(
    collate_token: SyntaxToken,
    collation: SqlName,
) -> SqlCollateConstraint {
    SqlCollateConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLLATE_CONSTRAINT,
        [
            Some(SyntaxElement::Token(collate_token)),
            Some(SyntaxElement::Node(collation.into_syntax())),
        ],
    ))
}
pub fn sql_collate_expression(
    expression: AnySqlExpression,
    collate_token: SyntaxToken,
    collation: SqlName,
) -> SqlCollateExpression {
    SqlCollateExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLLATE_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(collate_token)),
            Some(SyntaxElement::Node(collation.into_syntax())),
        ],
    ))
}
pub fn sql_column_definition(
    name: SqlName,
    ty: SqlTypeName,
    constraints: SqlColumnConstraintList,
) -> SqlColumnDefinition {
    SqlColumnDefinition::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_DEFINITION,
        [
            Some(SyntaxElement::Node(name.into_syntax())),
            Some(SyntaxElement::Node(ty.into_syntax())),
            Some(SyntaxElement::Node(constraints.into_syntax())),
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
pub fn sql_commit_statement(commit_token: SyntaxToken) -> SqlCommitStatementBuilder {
    SqlCommitStatementBuilder {
        commit_token,
        transaction_kind_token: None,
        semicolon_token: None,
    }
}
pub struct SqlCommitStatementBuilder {
    commit_token: SyntaxToken,
    transaction_kind_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCommitStatementBuilder {
    pub fn with_transaction_kind_token(mut self, transaction_kind_token: SyntaxToken) -> Self {
        self.transaction_kind_token = Some(transaction_kind_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCommitStatement {
        SqlCommitStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_COMMIT_STATEMENT,
            [
                Some(SyntaxElement::Token(self.commit_token)),
                self.transaction_kind_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_compound_select_statement(
    first: SqlSubqueryExpression,
    set_operations: SqlSetOperationList,
) -> SqlCompoundSelectStatementBuilder {
    SqlCompoundSelectStatementBuilder {
        first,
        set_operations,
        with_clause: None,
        order_by_clause: None,
        limit_clause: None,
        offset_clause: None,
        fetch_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlCompoundSelectStatementBuilder {
    first: SqlSubqueryExpression,
    set_operations: SqlSetOperationList,
    with_clause: Option<SqlWithClause>,
    order_by_clause: Option<SqlOrderByClause>,
    limit_clause: Option<PsqlLimitClause>,
    offset_clause: Option<SqlOffsetClause>,
    fetch_clause: Option<SqlFetchClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCompoundSelectStatementBuilder {
    pub fn with_with_clause(mut self, with_clause: SqlWithClause) -> Self {
        self.with_clause = Some(with_clause);
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
    pub fn build(self) -> SqlCompoundSelectStatement {
        SqlCompoundSelectStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_COMPOUND_SELECT_STATEMENT,
            [
                self.with_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.first.into_syntax())),
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
pub fn sql_constraint_name(constraint_token: SyntaxToken, name: SqlName) -> SqlConstraintName {
    SqlConstraintName::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CONSTRAINT_NAME,
        [
            Some(SyntaxElement::Token(constraint_token)),
            Some(SyntaxElement::Node(name.into_syntax())),
        ],
    ))
}
pub fn sql_create_index_statement(
    create_token: SyntaxToken,
    index_token: SyntaxToken,
    on_token: SyntaxToken,
    table: SqlTableName,
    l_paren_token: SyntaxToken,
    elements: SqlIndexElementList,
    r_paren_token: SyntaxToken,
) -> SqlCreateIndexStatementBuilder {
    SqlCreateIndexStatementBuilder {
        create_token,
        index_token,
        on_token,
        table,
        l_paren_token,
        elements,
        r_paren_token,
        unique_token: None,
        concurrently_token: None,
        if_token: None,
        not_token: None,
        exists_token: None,
        name: None,
        only_token: None,
        method: None,
        include: None,
        where_clause: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateIndexStatementBuilder {
    create_token: SyntaxToken,
    index_token: SyntaxToken,
    on_token: SyntaxToken,
    table: SqlTableName,
    l_paren_token: SyntaxToken,
    elements: SqlIndexElementList,
    r_paren_token: SyntaxToken,
    unique_token: Option<SyntaxToken>,
    concurrently_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    name: Option<SqlName>,
    only_token: Option<SyntaxToken>,
    method: Option<PsqlIndexMethodClause>,
    include: Option<SqlIndexIncludeClause>,
    where_clause: Option<SqlWhereClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCreateIndexStatementBuilder {
    pub fn with_unique_token(mut self, unique_token: SyntaxToken) -> Self {
        self.unique_token = Some(unique_token);
        self
    }
    pub fn with_concurrently_token(mut self, concurrently_token: SyntaxToken) -> Self {
        self.concurrently_token = Some(concurrently_token);
        self
    }
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
    pub fn with_name(mut self, name: SqlName) -> Self {
        self.name = Some(name);
        self
    }
    pub fn with_only_token(mut self, only_token: SyntaxToken) -> Self {
        self.only_token = Some(only_token);
        self
    }
    pub fn with_method(mut self, method: PsqlIndexMethodClause) -> Self {
        self.method = Some(method);
        self
    }
    pub fn with_include(mut self, include: SqlIndexIncludeClause) -> Self {
        self.include = Some(include);
        self
    }
    pub fn with_where_clause(mut self, where_clause: SqlWhereClause) -> Self {
        self.where_clause = Some(where_clause);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCreateIndexStatement {
        SqlCreateIndexStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CREATE_INDEX_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                self.unique_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.index_token)),
                self.concurrently_token
                    .map(|token| SyntaxElement::Token(token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.on_token)),
                self.only_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.method
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.elements.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.include
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_create_table_as_statement(
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: AnySqlCreateTableAsQuery,
) -> SqlCreateTableAsStatementBuilder {
    SqlCreateTableAsStatementBuilder {
        create_token,
        table_token,
        name,
        as_token,
        query,
        persistence_token: None,
        if_token: None,
        not_token: None,
        exists_token: None,
        columns: None,
        on_commit: None,
        with_data: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateTableAsStatementBuilder {
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: AnySqlCreateTableAsQuery,
    persistence_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    columns: Option<SqlColumnList>,
    on_commit: Option<PsqlOnCommitClause>,
    with_data: Option<PsqlWithDataClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCreateTableAsStatementBuilder {
    pub fn with_persistence_token(mut self, persistence_token: SyntaxToken) -> Self {
        self.persistence_token = Some(persistence_token);
        self
    }
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
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn with_on_commit(mut self, on_commit: PsqlOnCommitClause) -> Self {
        self.on_commit = Some(on_commit);
        self
    }
    pub fn with_with_data(mut self, with_data: PsqlWithDataClause) -> Self {
        self.with_data = Some(with_data);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlCreateTableAsStatement {
        SqlCreateTableAsStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_CREATE_TABLE_AS_STATEMENT,
            [
                Some(SyntaxElement::Token(self.create_token)),
                self.persistence_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.on_commit
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.as_token)),
                Some(SyntaxElement::Node(self.query.into_syntax())),
                self.with_data
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_create_table_statement(
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    l_paren_token: SyntaxToken,
    elements: SqlTableElementList,
    r_paren_token: SyntaxToken,
) -> SqlCreateTableStatementBuilder {
    SqlCreateTableStatementBuilder {
        create_token,
        table_token,
        name,
        l_paren_token,
        elements,
        r_paren_token,
        persistence_token: None,
        if_token: None,
        not_token: None,
        exists_token: None,
        partition_by: None,
        on_commit: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateTableStatementBuilder {
    create_token: SyntaxToken,
    table_token: SyntaxToken,
    name: SqlTableName,
    l_paren_token: SyntaxToken,
    elements: SqlTableElementList,
    r_paren_token: SyntaxToken,
    persistence_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    not_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    partition_by: Option<PsqlPartitionByClause>,
    on_commit: Option<PsqlOnCommitClause>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlCreateTableStatementBuilder {
    pub fn with_persistence_token(mut self, persistence_token: SyntaxToken) -> Self {
        self.persistence_token = Some(persistence_token);
        self
    }
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
    pub fn with_partition_by(mut self, partition_by: PsqlPartitionByClause) -> Self {
        self.partition_by = Some(partition_by);
        self
    }
    pub fn with_on_commit(mut self, on_commit: PsqlOnCommitClause) -> Self {
        self.on_commit = Some(on_commit);
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
                self.persistence_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.table_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                Some(SyntaxElement::Token(self.l_paren_token)),
                Some(SyntaxElement::Node(self.elements.into_syntax())),
                Some(SyntaxElement::Token(self.r_paren_token)),
                self.partition_by
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.on_commit
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
    query: AnySqlCreateTableAsQuery,
) -> SqlCreateViewStatementBuilder {
    SqlCreateViewStatementBuilder {
        create_token,
        view_token,
        name,
        as_token,
        query,
        or_token: None,
        replace_token: None,
        columns: None,
        options: None,
        semicolon_token: None,
    }
}
pub struct SqlCreateViewStatementBuilder {
    create_token: SyntaxToken,
    view_token: SyntaxToken,
    name: SqlTableName,
    as_token: SyntaxToken,
    query: AnySqlCreateTableAsQuery,
    or_token: Option<SyntaxToken>,
    replace_token: Option<SyntaxToken>,
    columns: Option<SqlColumnList>,
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
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
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
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
pub fn sql_default_constraint(
    default_token: SyntaxToken,
    value: AnySqlExpression,
) -> SqlDefaultConstraint {
    SqlDefaultConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_DEFAULT_CONSTRAINT,
        [
            Some(SyntaxElement::Token(default_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
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
pub fn sql_drop_column_action(
    drop_token: SyntaxToken,
    name: SqlName,
) -> SqlDropColumnActionBuilder {
    SqlDropColumnActionBuilder {
        drop_token,
        name,
        column_token: None,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
    }
}
pub struct SqlDropColumnActionBuilder {
    drop_token: SyntaxToken,
    name: SqlName,
    column_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
}
impl SqlDropColumnActionBuilder {
    pub fn with_column_token(mut self, column_token: SyntaxToken) -> Self {
        self.column_token = Some(column_token);
        self
    }
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
    pub fn build(self) -> SqlDropColumnAction {
        SqlDropColumnAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_COLUMN_ACTION,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                self.column_token.map(|token| SyntaxElement::Token(token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_drop_column_default(
    drop_token: SyntaxToken,
    default_token: SyntaxToken,
) -> SqlDropColumnDefault {
    SqlDropColumnDefault::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_DROP_COLUMN_DEFAULT,
        [
            Some(SyntaxElement::Token(drop_token)),
            Some(SyntaxElement::Token(default_token)),
        ],
    ))
}
pub fn sql_drop_column_not_null(
    drop_token: SyntaxToken,
    not_token: SyntaxToken,
    null_token: SyntaxToken,
) -> SqlDropColumnNotNull {
    SqlDropColumnNotNull::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_DROP_COLUMN_NOT_NULL,
        [
            Some(SyntaxElement::Token(drop_token)),
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Token(null_token)),
        ],
    ))
}
pub fn sql_drop_constraint_action(
    drop_token: SyntaxToken,
    constraint_token: SyntaxToken,
    name: SqlName,
) -> SqlDropConstraintActionBuilder {
    SqlDropConstraintActionBuilder {
        drop_token,
        constraint_token,
        name,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
    }
}
pub struct SqlDropConstraintActionBuilder {
    drop_token: SyntaxToken,
    constraint_token: SyntaxToken,
    name: SqlName,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
}
impl SqlDropConstraintActionBuilder {
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
    pub fn build(self) -> SqlDropConstraintAction {
        SqlDropConstraintAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_CONSTRAINT_ACTION,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.constraint_token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.name.into_syntax())),
                self.drop_behavior_token
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
pub fn sql_drop_index_statement(
    drop_token: SyntaxToken,
    index_token: SyntaxToken,
    indexes: SqlTableNameList,
) -> SqlDropIndexStatementBuilder {
    SqlDropIndexStatementBuilder {
        drop_token,
        index_token,
        indexes,
        concurrently_token: None,
        if_token: None,
        exists_token: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlDropIndexStatementBuilder {
    drop_token: SyntaxToken,
    index_token: SyntaxToken,
    indexes: SqlTableNameList,
    concurrently_token: Option<SyntaxToken>,
    if_token: Option<SyntaxToken>,
    exists_token: Option<SyntaxToken>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlDropIndexStatementBuilder {
    pub fn with_concurrently_token(mut self, concurrently_token: SyntaxToken) -> Self {
        self.concurrently_token = Some(concurrently_token);
        self
    }
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
    pub fn build(self) -> SqlDropIndexStatement {
        SqlDropIndexStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_DROP_INDEX_STATEMENT,
            [
                Some(SyntaxElement::Token(self.drop_token)),
                Some(SyntaxElement::Token(self.index_token)),
                self.concurrently_token
                    .map(|token| SyntaxElement::Token(token)),
                self.if_token.map(|token| SyntaxElement::Token(token)),
                self.exists_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.indexes.into_syntax())),
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
pub fn sql_foreign_key_table_constraint(
    foreign_token: SyntaxToken,
    key_token: SyntaxToken,
    columns: SqlColumnList,
    references: SqlReferencesConstraint,
) -> SqlForeignKeyTableConstraint {
    SqlForeignKeyTableConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_FOREIGN_KEY_TABLE_CONSTRAINT,
        [
            Some(SyntaxElement::Token(foreign_token)),
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
            Some(SyntaxElement::Node(references.into_syntax())),
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
pub fn sql_go_statement(go_token: SyntaxToken) -> SqlGoStatementBuilder {
    SqlGoStatementBuilder {
        go_token,
        semicolon_token: None,
    }
}
pub struct SqlGoStatementBuilder {
    go_token: SyntaxToken,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlGoStatementBuilder {
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlGoStatement {
        SqlGoStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_GO_STATEMENT,
            [
                Some(SyntaxElement::Token(self.go_token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_grant_option_for(
    grant_token: SyntaxToken,
    option_token: SyntaxToken,
    for_token: SyntaxToken,
) -> SqlGrantOptionFor {
    SqlGrantOptionFor::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_GRANT_OPTION_FOR,
        [
            Some(SyntaxElement::Token(grant_token)),
            Some(SyntaxElement::Token(option_token)),
            Some(SyntaxElement::Token(for_token)),
        ],
    ))
}
pub fn sql_grant_statement(
    grant_token: SyntaxToken,
    privileges: SqlPrivilegeList,
    to_token: SyntaxToken,
    grantees: SqlGranteeList,
) -> SqlGrantStatementBuilder {
    SqlGrantStatementBuilder {
        grant_token,
        privileges,
        to_token,
        grantees,
        target: None,
        grant_option: None,
        semicolon_token: None,
    }
}
pub struct SqlGrantStatementBuilder {
    grant_token: SyntaxToken,
    privileges: SqlPrivilegeList,
    to_token: SyntaxToken,
    grantees: SqlGranteeList,
    target: Option<AnySqlPrivilegeTarget>,
    grant_option: Option<SqlWithGrantOption>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlGrantStatementBuilder {
    pub fn with_target(mut self, target: AnySqlPrivilegeTarget) -> Self {
        self.target = Some(target);
        self
    }
    pub fn with_grant_option(mut self, grant_option: SqlWithGrantOption) -> Self {
        self.grant_option = Some(grant_option);
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
                Some(SyntaxElement::Node(self.privileges.into_syntax())),
                self.target
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.to_token)),
                Some(SyntaxElement::Node(self.grantees.into_syntax())),
                self.grant_option
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
pub fn sql_index_include_clause(
    include_token: SyntaxToken,
    columns: SqlColumnList,
) -> SqlIndexIncludeClause {
    SqlIndexIncludeClause::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_INDEX_INCLUDE_CLAUSE,
        [
            Some(SyntaxElement::Token(include_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
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
pub fn sql_is_distinct_from_expression(
    left: AnySqlExpression,
    is_token: SyntaxToken,
    distinct_token: SyntaxToken,
    from_token: SyntaxToken,
    right: AnySqlExpression,
) -> SqlIsDistinctFromExpressionBuilder {
    SqlIsDistinctFromExpressionBuilder {
        left,
        is_token,
        distinct_token,
        from_token,
        right,
        not_token: None,
    }
}
pub struct SqlIsDistinctFromExpressionBuilder {
    left: AnySqlExpression,
    is_token: SyntaxToken,
    distinct_token: SyntaxToken,
    from_token: SyntaxToken,
    right: AnySqlExpression,
    not_token: Option<SyntaxToken>,
}
impl SqlIsDistinctFromExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlIsDistinctFromExpression {
        SqlIsDistinctFromExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_IS_DISTINCT_FROM_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.left.into_syntax())),
                Some(SyntaxElement::Token(self.is_token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.distinct_token)),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.right.into_syntax())),
            ],
        ))
    }
}
pub fn sql_is_expression(
    expression: AnySqlExpression,
    is_token: SyntaxToken,
    value_token: SyntaxToken,
) -> SqlIsExpressionBuilder {
    SqlIsExpressionBuilder {
        expression,
        is_token,
        value_token,
        not_token: None,
    }
}
pub struct SqlIsExpressionBuilder {
    expression: AnySqlExpression,
    is_token: SyntaxToken,
    value_token: SyntaxToken,
    not_token: Option<SyntaxToken>,
}
impl SqlIsExpressionBuilder {
    pub fn with_not_token(mut self, not_token: SyntaxToken) -> Self {
        self.not_token = Some(not_token);
        self
    }
    pub fn build(self) -> SqlIsExpression {
        SqlIsExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_IS_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.expression.into_syntax())),
                Some(SyntaxElement::Token(self.is_token)),
                self.not_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Token(self.value_token)),
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
pub fn sql_not_null_constraint(
    not_token: SyntaxToken,
    null_token: SyntaxToken,
) -> SqlNotNullConstraint {
    SqlNotNullConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NOT_NULL_CONSTRAINT,
        [
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Token(null_token)),
        ],
    ))
}
pub fn sql_null_constraint(null_token: SyntaxToken) -> SqlNullConstraint {
    SqlNullConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NULL_CONSTRAINT,
        [Some(SyntaxElement::Token(null_token))],
    ))
}
pub fn sql_null_literal_expression(value_token: SyntaxToken) -> SqlNullLiteralExpression {
    SqlNullLiteralExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NULL_LITERAL_EXPRESSION,
        [Some(SyntaxElement::Token(value_token))],
    ))
}
pub fn sql_nulls_order(nulls_token: SyntaxToken, position_token: SyntaxToken) -> SqlNullsOrder {
    SqlNullsOrder::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_NULLS_ORDER,
        [
            Some(SyntaxElement::Token(nulls_token)),
            Some(SyntaxElement::Token(position_token)),
        ],
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
        nulls_order: None,
    }
}
pub struct SqlOrderByExpressionBuilder {
    item: AnySqlExpression,
    order_token: Option<SyntaxToken>,
    nulls_order: Option<SqlNullsOrder>,
}
impl SqlOrderByExpressionBuilder {
    pub fn with_order_token(mut self, order_token: SyntaxToken) -> Self {
        self.order_token = Some(order_token);
        self
    }
    pub fn with_nulls_order(mut self, nulls_order: SqlNullsOrder) -> Self {
        self.nulls_order = Some(nulls_order);
        self
    }
    pub fn build(self) -> SqlOrderByExpression {
        SqlOrderByExpression::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ORDER_BY_EXPRESSION,
            [
                Some(SyntaxElement::Node(self.item.into_syntax())),
                self.order_token.map(|token| SyntaxElement::Token(token)),
                self.nulls_order
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
pub fn sql_postfix_null_expression(
    expression: AnySqlExpression,
    operator_token: SyntaxToken,
) -> SqlPostfixNullExpression {
    SqlPostfixNullExpression::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_POSTFIX_NULL_EXPRESSION,
        [
            Some(SyntaxElement::Node(expression.into_syntax())),
            Some(SyntaxElement::Token(operator_token)),
        ],
    ))
}
pub fn sql_precision_modifier(precision_token: SyntaxToken) -> SqlPrecisionModifier {
    SqlPrecisionModifier::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRECISION_MODIFIER,
        [Some(SyntaxElement::Token(precision_token))],
    ))
}
pub fn sql_primary_key_constraint(
    primary_token: SyntaxToken,
    key_token: SyntaxToken,
) -> SqlPrimaryKeyConstraint {
    SqlPrimaryKeyConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRIMARY_KEY_CONSTRAINT,
        [
            Some(SyntaxElement::Token(primary_token)),
            Some(SyntaxElement::Token(key_token)),
        ],
    ))
}
pub fn sql_primary_key_table_constraint(
    primary_token: SyntaxToken,
    key_token: SyntaxToken,
    columns: SqlColumnList,
) -> SqlPrimaryKeyTableConstraint {
    SqlPrimaryKeyTableConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRIMARY_KEY_TABLE_CONSTRAINT,
        [
            Some(SyntaxElement::Token(primary_token)),
            Some(SyntaxElement::Token(key_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
        ],
    ))
}
pub fn sql_privilege(kind_token: SyntaxToken) -> SqlPrivilegeBuilder {
    SqlPrivilegeBuilder {
        kind_token,
        privileges_token: None,
        columns: None,
    }
}
pub struct SqlPrivilegeBuilder {
    kind_token: SyntaxToken,
    privileges_token: Option<SyntaxToken>,
    columns: Option<SqlColumnList>,
}
impl SqlPrivilegeBuilder {
    pub fn with_privileges_token(mut self, privileges_token: SyntaxToken) -> Self {
        self.privileges_token = Some(privileges_token);
        self
    }
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn build(self) -> SqlPrivilege {
        SqlPrivilege::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_PRIVILEGE,
            [
                Some(SyntaxElement::Token(self.kind_token)),
                self.privileges_token
                    .map(|token| SyntaxElement::Token(token)),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_privilege_all_in_schema(
    on_token: SyntaxToken,
    all_token: SyntaxToken,
    object_kind_token: SyntaxToken,
    in_token: SyntaxToken,
    schema_token: SyntaxToken,
    schemas: SqlTableNameList,
) -> SqlPrivilegeAllInSchema {
    SqlPrivilegeAllInSchema::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRIVILEGE_ALL_IN_SCHEMA,
        [
            Some(SyntaxElement::Token(on_token)),
            Some(SyntaxElement::Token(all_token)),
            Some(SyntaxElement::Token(object_kind_token)),
            Some(SyntaxElement::Token(in_token)),
            Some(SyntaxElement::Token(schema_token)),
            Some(SyntaxElement::Node(schemas.into_syntax())),
        ],
    ))
}
pub fn sql_privilege_objects(
    on_token: SyntaxToken,
    objects: SqlTableNameList,
) -> SqlPrivilegeObjectsBuilder {
    SqlPrivilegeObjectsBuilder {
        on_token,
        objects,
        object_kind_token: None,
    }
}
pub struct SqlPrivilegeObjectsBuilder {
    on_token: SyntaxToken,
    objects: SqlTableNameList,
    object_kind_token: Option<SyntaxToken>,
}
impl SqlPrivilegeObjectsBuilder {
    pub fn with_object_kind_token(mut self, object_kind_token: SyntaxToken) -> Self {
        self.object_kind_token = Some(object_kind_token);
        self
    }
    pub fn build(self) -> SqlPrivilegeObjects {
        SqlPrivilegeObjects::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_PRIVILEGE_OBJECTS,
            [
                Some(SyntaxElement::Token(self.on_token)),
                self.object_kind_token
                    .map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.objects.into_syntax())),
            ],
        ))
    }
}
pub fn sql_references_constraint(
    references_token: SyntaxToken,
    table: SqlTableName,
) -> SqlReferencesConstraintBuilder {
    SqlReferencesConstraintBuilder {
        references_token,
        table,
        columns: None,
    }
}
pub struct SqlReferencesConstraintBuilder {
    references_token: SyntaxToken,
    table: SqlTableName,
    columns: Option<SqlColumnList>,
}
impl SqlReferencesConstraintBuilder {
    pub fn with_columns(mut self, columns: SqlColumnList) -> Self {
        self.columns = Some(columns);
        self
    }
    pub fn build(self) -> SqlReferencesConstraint {
        SqlReferencesConstraint::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_REFERENCES_CONSTRAINT,
            [
                Some(SyntaxElement::Token(self.references_token)),
                Some(SyntaxElement::Node(self.table.into_syntax())),
                self.columns
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_rename_column_action(
    rename_token: SyntaxToken,
    old_name: SqlName,
    to_token: SyntaxToken,
    new_name: SqlName,
) -> SqlRenameColumnActionBuilder {
    SqlRenameColumnActionBuilder {
        rename_token,
        old_name,
        to_token,
        new_name,
        column_token: None,
    }
}
pub struct SqlRenameColumnActionBuilder {
    rename_token: SyntaxToken,
    old_name: SqlName,
    to_token: SyntaxToken,
    new_name: SqlName,
    column_token: Option<SyntaxToken>,
}
impl SqlRenameColumnActionBuilder {
    pub fn with_column_token(mut self, column_token: SyntaxToken) -> Self {
        self.column_token = Some(column_token);
        self
    }
    pub fn build(self) -> SqlRenameColumnAction {
        SqlRenameColumnAction::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_RENAME_COLUMN_ACTION,
            [
                Some(SyntaxElement::Token(self.rename_token)),
                self.column_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.old_name.into_syntax())),
                Some(SyntaxElement::Token(self.to_token)),
                Some(SyntaxElement::Node(self.new_name.into_syntax())),
            ],
        ))
    }
}
pub fn sql_rename_constraint_action(
    rename_token: SyntaxToken,
    constraint_token: SyntaxToken,
    old_name: SqlName,
    to_token: SyntaxToken,
    new_name: SqlName,
) -> SqlRenameConstraintAction {
    SqlRenameConstraintAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_RENAME_CONSTRAINT_ACTION,
        [
            Some(SyntaxElement::Token(rename_token)),
            Some(SyntaxElement::Token(constraint_token)),
            Some(SyntaxElement::Node(old_name.into_syntax())),
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(new_name.into_syntax())),
        ],
    ))
}
pub fn sql_rename_table_action(
    rename_token: SyntaxToken,
    to_token: SyntaxToken,
    new_name: SqlName,
) -> SqlRenameTableAction {
    SqlRenameTableAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_RENAME_TABLE_ACTION,
        [
            Some(SyntaxElement::Token(rename_token)),
            Some(SyntaxElement::Token(to_token)),
            Some(SyntaxElement::Node(new_name.into_syntax())),
        ],
    ))
}
pub fn sql_revoke_statement(
    revoke_token: SyntaxToken,
    privileges: SqlPrivilegeList,
    from_token: SyntaxToken,
    grantees: SqlGranteeList,
) -> SqlRevokeStatementBuilder {
    SqlRevokeStatementBuilder {
        revoke_token,
        privileges,
        from_token,
        grantees,
        grant_option_for: None,
        target: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlRevokeStatementBuilder {
    revoke_token: SyntaxToken,
    privileges: SqlPrivilegeList,
    from_token: SyntaxToken,
    grantees: SqlGranteeList,
    grant_option_for: Option<SqlGrantOptionFor>,
    target: Option<AnySqlPrivilegeTarget>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlRevokeStatementBuilder {
    pub fn with_grant_option_for(mut self, grant_option_for: SqlGrantOptionFor) -> Self {
        self.grant_option_for = Some(grant_option_for);
        self
    }
    pub fn with_target(mut self, target: AnySqlPrivilegeTarget) -> Self {
        self.target = Some(target);
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
    pub fn build(self) -> SqlRevokeStatement {
        SqlRevokeStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_REVOKE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.revoke_token)),
                self.grant_option_for
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.privileges.into_syntax())),
                self.target
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Token(self.from_token)),
                Some(SyntaxElement::Node(self.grantees.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
}
pub fn sql_rollback_statement(rollback_token: SyntaxToken) -> SqlRollbackStatementBuilder {
    SqlRollbackStatementBuilder {
        rollback_token,
        transaction_kind_token: None,
        semicolon_token: None,
    }
}
pub struct SqlRollbackStatementBuilder {
    rollback_token: SyntaxToken,
    transaction_kind_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlRollbackStatementBuilder {
    pub fn with_transaction_kind_token(mut self, transaction_kind_token: SyntaxToken) -> Self {
        self.transaction_kind_token = Some(transaction_kind_token);
        self
    }
    pub fn with_semicolon_token(mut self, semicolon_token: SyntaxToken) -> Self {
        self.semicolon_token = Some(semicolon_token);
        self
    }
    pub fn build(self) -> SqlRollbackStatement {
        SqlRollbackStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_ROLLBACK_STATEMENT,
            [
                Some(SyntaxElement::Token(self.rollback_token)),
                self.transaction_kind_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
        into_clause: None,
    }
}
pub struct SqlSelectClauseBuilder {
    select_token: SyntaxToken,
    list: SqlSelectItemList,
    quantifier: Option<AnySqlSelectQuantifier>,
    into_clause: Option<PsqlIntoClause>,
}
impl SqlSelectClauseBuilder {
    pub fn with_quantifier(mut self, quantifier: AnySqlSelectQuantifier) -> Self {
        self.quantifier = Some(quantifier);
        self
    }
    pub fn with_into_clause(mut self, into_clause: PsqlIntoClause) -> Self {
        self.into_clause = Some(into_clause);
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
                self.into_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
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
pub fn sql_set_column_default(
    set_token: SyntaxToken,
    default_token: SyntaxToken,
    value: AnySqlExpression,
) -> SqlSetColumnDefault {
    SqlSetColumnDefault::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_COLUMN_DEFAULT,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Token(default_token)),
            Some(SyntaxElement::Node(value.into_syntax())),
        ],
    ))
}
pub fn sql_set_column_not_null(
    set_token: SyntaxToken,
    not_token: SyntaxToken,
    null_token: SyntaxToken,
) -> SqlSetColumnNotNull {
    SqlSetColumnNotNull::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_COLUMN_NOT_NULL,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Token(not_token)),
            Some(SyntaxElement::Token(null_token)),
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
pub fn sql_set_operation(operator_token_token: SyntaxToken) -> SqlSetOperationBuilder {
    SqlSetOperationBuilder {
        operator_token_token,
        quantifier_token: None,
        select_clause: None,
        from_clause: None,
        where_clause: None,
        group_by_clause: None,
        having_clause: None,
        subquery: None,
    }
}
pub struct SqlSetOperationBuilder {
    operator_token_token: SyntaxToken,
    quantifier_token: Option<SyntaxToken>,
    select_clause: Option<SqlSelectClause>,
    from_clause: Option<SqlFromClause>,
    where_clause: Option<SqlWhereClause>,
    group_by_clause: Option<SqlGroupByClause>,
    having_clause: Option<SqlHavingClause>,
    subquery: Option<SqlSubqueryExpression>,
}
impl SqlSetOperationBuilder {
    pub fn with_quantifier_token(mut self, quantifier_token: SyntaxToken) -> Self {
        self.quantifier_token = Some(quantifier_token);
        self
    }
    pub fn with_select_clause(mut self, select_clause: SqlSelectClause) -> Self {
        self.select_clause = Some(select_clause);
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
    pub fn with_subquery(mut self, subquery: SqlSubqueryExpression) -> Self {
        self.subquery = Some(subquery);
        self
    }
    pub fn build(self) -> SqlSetOperation {
        SqlSetOperation::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_SET_OPERATION,
            [
                Some(SyntaxElement::Token(self.operator_token_token)),
                self.quantifier_token
                    .map(|token| SyntaxElement::Token(token)),
                self.select_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.from_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.where_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.group_by_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.having_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.subquery
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_set_schema_action(
    set_token: SyntaxToken,
    schema_token: SyntaxToken,
    schema: SqlName,
) -> SqlSetSchemaAction {
    SqlSetSchemaAction::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_SET_SCHEMA_ACTION,
        [
            Some(SyntaxElement::Token(set_token)),
            Some(SyntaxElement::Token(schema_token)),
            Some(SyntaxElement::Node(schema.into_syntax())),
        ],
    ))
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
pub fn sql_table_constraint(constraint: AnySqlTableConstraint) -> SqlTableConstraintBuilder {
    SqlTableConstraintBuilder {
        constraint,
        name: None,
    }
}
pub struct SqlTableConstraintBuilder {
    constraint: AnySqlTableConstraint,
    name: Option<SqlConstraintName>,
}
impl SqlTableConstraintBuilder {
    pub fn with_name(mut self, name: SqlConstraintName) -> Self {
        self.name = Some(name);
        self
    }
    pub fn build(self) -> SqlTableConstraint {
        SqlTableConstraint::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_TABLE_CONSTRAINT,
            [
                self.name
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                Some(SyntaxElement::Node(self.constraint.into_syntax())),
            ],
        ))
    }
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
pub fn sql_truncate_statement(
    truncate_token: SyntaxToken,
    tables: SqlTableNameList,
) -> SqlTruncateStatementBuilder {
    SqlTruncateStatementBuilder {
        truncate_token,
        tables,
        table_token: None,
        identity_clause: None,
        drop_behavior_token: None,
        semicolon_token: None,
    }
}
pub struct SqlTruncateStatementBuilder {
    truncate_token: SyntaxToken,
    tables: SqlTableNameList,
    table_token: Option<SyntaxToken>,
    identity_clause: Option<PsqlTruncateIdentityClause>,
    drop_behavior_token: Option<SyntaxToken>,
    semicolon_token: Option<SyntaxToken>,
}
impl SqlTruncateStatementBuilder {
    pub fn with_table_token(mut self, table_token: SyntaxToken) -> Self {
        self.table_token = Some(table_token);
        self
    }
    pub fn with_identity_clause(mut self, identity_clause: PsqlTruncateIdentityClause) -> Self {
        self.identity_clause = Some(identity_clause);
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
    pub fn build(self) -> SqlTruncateStatement {
        SqlTruncateStatement::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_TRUNCATE_STATEMENT,
            [
                Some(SyntaxElement::Token(self.truncate_token)),
                self.table_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.tables.into_syntax())),
                self.identity_clause
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
                self.drop_behavior_token
                    .map(|token| SyntaxElement::Token(token)),
                self.semicolon_token
                    .map(|token| SyntaxElement::Token(token)),
            ],
        ))
    }
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
pub fn sql_unique_constraint(unique_token: SyntaxToken) -> SqlUniqueConstraint {
    SqlUniqueConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_UNIQUE_CONSTRAINT,
        [Some(SyntaxElement::Token(unique_token))],
    ))
}
pub fn sql_unique_table_constraint(
    unique_token: SyntaxToken,
    columns: SqlColumnList,
) -> SqlUniqueTableConstraint {
    SqlUniqueTableConstraint::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_UNIQUE_TABLE_CONSTRAINT,
        [
            Some(SyntaxElement::Token(unique_token)),
            Some(SyntaxElement::Node(columns.into_syntax())),
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
pub fn sql_window_frame_clause(
    unit_token: SyntaxToken,
    start: AnySqlWindowFrameBound,
) -> SqlWindowFrameClauseBuilder {
    SqlWindowFrameClauseBuilder {
        unit_token,
        start,
        between_token: None,
        and_token: None,
        end: None,
    }
}
pub struct SqlWindowFrameClauseBuilder {
    unit_token: SyntaxToken,
    start: AnySqlWindowFrameBound,
    between_token: Option<SyntaxToken>,
    and_token: Option<SyntaxToken>,
    end: Option<AnySqlWindowFrameBound>,
}
impl SqlWindowFrameClauseBuilder {
    pub fn with_between_token(mut self, between_token: SyntaxToken) -> Self {
        self.between_token = Some(between_token);
        self
    }
    pub fn with_and_token(mut self, and_token: SyntaxToken) -> Self {
        self.and_token = Some(and_token);
        self
    }
    pub fn with_end(mut self, end: AnySqlWindowFrameBound) -> Self {
        self.end = Some(end);
        self
    }
    pub fn build(self) -> SqlWindowFrameClause {
        SqlWindowFrameClause::unwrap_cast(SyntaxNode::new_detached(
            SqlSyntaxKind::SQL_WINDOW_FRAME_CLAUSE,
            [
                Some(SyntaxElement::Token(self.unit_token)),
                self.between_token.map(|token| SyntaxElement::Token(token)),
                Some(SyntaxElement::Node(self.start.into_syntax())),
                self.and_token.map(|token| SyntaxElement::Token(token)),
                self.end
                    .map(|token| SyntaxElement::Node(token.into_syntax())),
            ],
        ))
    }
}
pub fn sql_window_frame_current_row_bound(
    current_token: SyntaxToken,
    row_token: SyntaxToken,
) -> SqlWindowFrameCurrentRowBound {
    SqlWindowFrameCurrentRowBound::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_FRAME_CURRENT_ROW_BOUND,
        [
            Some(SyntaxElement::Token(current_token)),
            Some(SyntaxElement::Token(row_token)),
        ],
    ))
}
pub fn sql_window_frame_offset_bound(
    offset: AnySqlExpression,
    direction_token: SyntaxToken,
) -> SqlWindowFrameOffsetBound {
    SqlWindowFrameOffsetBound::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_FRAME_OFFSET_BOUND,
        [
            Some(SyntaxElement::Node(offset.into_syntax())),
            Some(SyntaxElement::Token(direction_token)),
        ],
    ))
}
pub fn sql_window_frame_unbounded_bound(
    unbounded_token: SyntaxToken,
    direction_token: SyntaxToken,
) -> SqlWindowFrameUnboundedBound {
    SqlWindowFrameUnboundedBound::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WINDOW_FRAME_UNBOUNDED_BOUND,
        [
            Some(SyntaxElement::Token(unbounded_token)),
            Some(SyntaxElement::Token(direction_token)),
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
        frame_clause: None,
    }
}
pub struct SqlWindowSpecificationBuilder {
    l_paren_token: SyntaxToken,
    r_paren_token: SyntaxToken,
    partition_by_clause: Option<SqlWindowPartitionByClause>,
    order_by_clause: Option<SqlOrderByClause>,
    frame_clause: Option<SqlWindowFrameClause>,
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
    pub fn with_frame_clause(mut self, frame_clause: SqlWindowFrameClause) -> Self {
        self.frame_clause = Some(frame_clause);
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
                self.frame_clause
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
pub fn sql_with_grant_option(
    with_token: SyntaxToken,
    grant_token: SyntaxToken,
    option_token: SyntaxToken,
) -> SqlWithGrantOption {
    SqlWithGrantOption::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_WITH_GRANT_OPTION,
        [
            Some(SyntaxElement::Token(with_token)),
            Some(SyntaxElement::Token(grant_token)),
            Some(SyntaxElement::Token(option_token)),
        ],
    ))
}
pub fn psql_copy_option_list<I, S>(items: I, separators: S) -> PsqlCopyOptionList
where
    I: IntoIterator<Item = PsqlCopyOption>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlCopyOptionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_COPY_OPTION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
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
pub fn psql_pl_declaration_list<I>(items: I) -> PsqlPlDeclarationList
where
    I: IntoIterator<Item = PsqlPlVariableDeclaration>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlPlDeclarationList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_DECLARATION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_pl_elsif_clause_list<I>(items: I) -> PsqlPlElsifClauseList
where
    I: IntoIterator<Item = PsqlPlElsifClause>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlPlElsifClauseList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_ELSIF_CLAUSE_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_pl_statement_list<I>(items: I) -> PsqlPlStatementList
where
    I: IntoIterator<Item = AnyPsqlPlStatement>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlPlStatementList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_PL_STATEMENT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
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
pub fn psql_sequence_option_list<I>(items: I) -> PsqlSequenceOptionList
where
    I: IntoIterator<Item = PsqlSequenceOption>,
    I::IntoIter: ExactSizeIterator,
{
    PsqlSequenceOptionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_SEQUENCE_OPTION_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
    ))
}
pub fn psql_table_and_columns_list<I, S>(items: I, separators: S) -> PsqlTableAndColumnsList
where
    I: IntoIterator<Item = PsqlTableAndColumns>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    PsqlTableAndColumnsList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::PSQL_TABLE_AND_COLUMNS_LIST,
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
pub fn sql_alter_table_action_list<I, S>(items: I, separators: S) -> SqlAlterTableActionList
where
    I: IntoIterator<Item = AnySqlAlterTableAction>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlAlterTableActionList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_ALTER_TABLE_ACTION_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
    ))
}
pub fn sql_call_argument_list<I, S>(items: I, separators: S) -> SqlCallArgumentList
where
    I: IntoIterator<Item = AnySqlCallArgument>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlCallArgumentList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_CALL_ARGUMENT_LIST,
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
pub fn sql_column_constraint_list<I>(items: I) -> SqlColumnConstraintList
where
    I: IntoIterator<Item = AnySqlColumnConstraint>,
    I::IntoIter: ExactSizeIterator,
{
    SqlColumnConstraintList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_COLUMN_CONSTRAINT_LIST,
        items
            .into_iter()
            .map(|item| Some(item.into_syntax().into())),
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
pub fn sql_index_element_list<I, S>(items: I, separators: S) -> SqlIndexElementList
where
    I: IntoIterator<Item = SqlOrderByExpression>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlIndexElementList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_INDEX_ELEMENT_LIST,
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
pub fn sql_privilege_list<I, S>(items: I, separators: S) -> SqlPrivilegeList
where
    I: IntoIterator<Item = SqlPrivilege>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlPrivilegeList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_PRIVILEGE_LIST,
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
pub fn sql_table_element_list<I, S>(items: I, separators: S) -> SqlTableElementList
where
    I: IntoIterator<Item = AnySqlTableElement>,
    I::IntoIter: ExactSizeIterator,
    S: IntoIterator<Item = SqlSyntaxToken>,
    S::IntoIter: ExactSizeIterator,
{
    let mut items = items.into_iter();
    let mut separators = separators.into_iter();
    let length = items.len() + separators.len();
    SqlTableElementList::unwrap_cast(SyntaxNode::new_detached(
        SqlSyntaxKind::SQL_TABLE_ELEMENT_LIST,
        (0..length).map(|index| {
            if index % 2 == 0 {
                Some(items.next()?.into_syntax().into())
            } else {
                Some(separators.next()?.into())
            }
        }),
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
