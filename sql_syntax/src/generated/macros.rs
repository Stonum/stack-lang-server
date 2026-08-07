//! Generated file, do not edit by hand, see `xtask/codegen`

#[doc = r" Reconstruct an AstNode from a SyntaxNode"]
#[doc = r""]
#[doc = r" This macros performs a match over the [kind](biome_rowan::SyntaxNode::kind)"]
#[doc = r" of the provided [biome_rowan::SyntaxNode] and constructs the appropriate"]
#[doc = r" AstNode type for it, then execute the provided expression over it."]
#[doc = r""]
#[doc = r" # Examples"]
#[doc = r""]
#[doc = r" ```ignore"]
#[doc = r" map_syntax_node!(syntax_node, node => node.format())"]
#[doc = r" ```"]
#[macro_export]
macro_rules! map_syntax_node {
    ($ node : expr , $ pattern : pat => $ body : expr) => {
        match $node {
            node => match $crate::SqlSyntaxNode::kind(&node) {
                $crate::SqlSyntaxKind::PSQL_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlArraySubscriptExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CAST_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlCastExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_FUNCTION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_POLICY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreatePolicyStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_TRIGGER_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateTriggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CTE_MATERIALIZED_HINT => {
                    let $pattern = unsafe { $crate::PsqlCteMaterializedHint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DELETE_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDeleteUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DISTINCT_ON_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDistinctOnClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DO_NOTHING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDoNothingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DO_UPDATE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDoUpdateClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_FUNCTION_PARAMETERS => {
                    let $pattern =
                        unsafe { $crate::PsqlDropFunctionParameters::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_POLICY_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropPolicyStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_TRIGGER_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropTriggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_FILTER_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlFilterClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_FUNCTION_PARAMETER => {
                    let $pattern = unsafe { $crate::PsqlFunctionParameter::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_INTERVAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlIntervalExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_JOIN_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlJoinUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_LANGUAGE_OPTION => {
                    let $pattern = unsafe { $crate::PsqlLanguageOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_LIMIT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlLimitClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ON_CONFLICT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOnConflictClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ON_CONSTRAINT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOnConstraintClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARAMETER_DEFAULT => {
                    let $pattern = unsafe { $crate::PsqlParameterDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_POLICY_FOR_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPolicyForClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_POLICY_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPolicyUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_POLICY_WITH_CHECK_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlPolicyWithCheckClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturningClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_NULL_OPTION => {
                    let $pattern = unsafe { $crate::PsqlReturnsNullOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_SETOF_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsSetofClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_TABLE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsTableClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN => {
                    let $pattern = unsafe { $crate::PsqlReturnsTableColumn::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_TRIGGER_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsTriggerClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SECURITY_OPTION => {
                    let $pattern = unsafe { $crate::PsqlSecurityOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_STRICT_OPTION => {
                    let $pattern = unsafe { $crate::PsqlStrictOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SUBSTRING_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlSubstringExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SUBSTRING_FOR_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSubstringForClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SUBSTRING_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSubstringFromClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlTildeArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTildeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_EVENT => {
                    let $pattern = unsafe { $crate::PsqlTriggerEvent::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_FOR_EACH_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlTriggerForEachClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingItem::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_UPDATE_OF_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerUpdateOfClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlTriggerWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TYPE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTypeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_VIEW_OPTION => {
                    let $pattern = unsafe { $crate::PsqlViewOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_VIEW_OPTIONS => {
                    let $pattern = unsafe { $crate::PsqlViewOptions::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_VOLATILITY_OPTION => {
                    let $pattern = unsafe { $crate::PsqlVolatilityOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALIAS => {
                    let $pattern = unsafe { $crate::SqlAlias::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALIAS_COLUMN_DEFINITION => {
                    let $pattern = unsafe { $crate::SqlAliasColumnDefinition::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALIAS_COLUMN_LIST => {
                    let $pattern = unsafe { $crate::SqlAliasColumnList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ANY_ALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlAnyAllExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BETWEEN_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlBetweenExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CASE_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlCaseElseClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CASE_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlCaseExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CASE_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlCaseWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CAST_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlCastFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::SqlColReference::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLUMN_DEFINITION => {
                    let $pattern = unsafe { $crate::SqlColumnDefinition::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLUMN_LIST => {
                    let $pattern = unsafe { $crate::SqlColumnList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CREATE_TABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlCreateTableStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CREATE_VIEW_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlCreateViewStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CTE_DEFINITION => {
                    let $pattern = unsafe { $crate::SqlCteDefinition::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DATA_BASE_NAME => {
                    let $pattern = unsafe { $crate::SqlDataBaseName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DELETE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDeleteStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_FUNCTION_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDropFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_TABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDropTableStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_VIEW_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDropViewStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_EXISTS_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlExistsExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FETCH_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlFetchClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FETCH_ONLY_TAIL => {
                    let $pattern = unsafe { $crate::SqlFetchOnlyTail::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FETCH_WITH_TIES_TAIL => {
                    let $pattern = unsafe { $crate::SqlFetchWithTiesTail::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlFromClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FROM_ITEM => {
                    let $pattern = unsafe { $crate::SqlFromItem::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FUNCTION_BINDING => {
                    let $pattern = unsafe { $crate::SqlFunctionBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_GRANT_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlGrantStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_GROUP_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlGroupByClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_HAVING_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlHavingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlInExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_IN_VALUE_LIST => {
                    let $pattern = unsafe { $crate::SqlInValueList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_INSERT_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlInsertStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_IS_NULL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlIsNullExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_JOIN_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlJoinClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_LIKE_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlLikeExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NAME => {
                    let $pattern = unsafe { $crate::SqlName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_OFFSET_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlOffsetClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ORDER_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlOrderByClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ORDER_BY_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlOrderByExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PARAMETER_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlParameterExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PARENTHESIZED_JOIN_BINDING => {
                    let $pattern =
                        unsafe { $crate::SqlParenthesizedJoinBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRECISION_MODIFIER => {
                    let $pattern = unsafe { $crate::SqlPrecisionModifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ROOT => {
                    let $pattern = unsafe { $crate::SqlRoot::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_ALL_QUANTIFIER => {
                    let $pattern = unsafe { $crate::SqlSelectAllQuantifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlSelectClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_DISTINCT_QUANTIFIER => {
                    let $pattern =
                        unsafe { $crate::SqlSelectDistinctQuantifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlSelectExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlSelectStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlSetClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_ITEM => {
                    let $pattern = unsafe { $crate::SqlSetItem::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_OPERATION => {
                    let $pattern = unsafe { $crate::SqlSetOperation::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SHEMA_NAME => {
                    let $pattern = unsafe { $crate::SqlShemaName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_STAR => {
                    let $pattern = unsafe { $crate::SqlStar::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SUBQUERY_BINDING => {
                    let $pattern = unsafe { $crate::SqlSubqueryBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SUBQUERY_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlSubqueryExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TABLE_BINDING => {
                    let $pattern = unsafe { $crate::SqlTableBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TABLE_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::SqlTableColReference::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TABLE_NAME => {
                    let $pattern = unsafe { $crate::SqlTableName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TABLE_STAR => {
                    let $pattern = unsafe { $crate::SqlTableStar::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TILDE_NAME => {
                    let $pattern = unsafe { $crate::SqlTildeName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TIME_ZONE_MODIFIER => {
                    let $pattern = unsafe { $crate::SqlTimeZoneModifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { $crate::SqlTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TYPE_NAME => {
                    let $pattern = unsafe { $crate::SqlTypeName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_UPDATE_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlUpdateFromClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_UPDATE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlUpdateStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_VALUES_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlValuesClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_VALUES_ROW => {
                    let $pattern = unsafe { $crate::SqlValuesRow::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_VARYING_MODIFIER => {
                    let $pattern = unsafe { $crate::SqlVaryingModifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WHERE_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlWhereClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlWindowFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_PARTITION_BY_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::SqlWindowPartitionByClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_SPECIFICATION => {
                    let $pattern = unsafe { $crate::SqlWindowSpecification::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WITH_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlWithClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS => {
                    let $pattern = unsafe { $crate::SqlBogus::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::SqlBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::SqlBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::SqlBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::SqlBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_FUNCTION_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlFunctionOptionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_FUNCTION_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlFunctionParameterList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlReturnsTableColumnList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_EVENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlTriggerEventList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TYPE_NAME_LIST => {
                    let $pattern = unsafe { $crate::PsqlTypeNameList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_VIEW_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlViewOptionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALIAS_COLUMN_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::SqlAliasColumnDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CASE_WHEN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::SqlCaseWhenClauseList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLUMN_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::SqlColumnDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLUMN_NAME_LIST => {
                    let $pattern = unsafe { $crate::SqlColumnNameList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CTE_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::SqlCteDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_EXPRESSION_LIST => {
                    let $pattern = unsafe { $crate::SqlExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_FROM_ITEM_LIST => {
                    let $pattern = unsafe { $crate::SqlFromItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_GRANTEE_LIST => {
                    let $pattern = unsafe { $crate::SqlGranteeList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_GROUP_BY_ITEM_LIST => {
                    let $pattern = unsafe { $crate::SqlGroupByItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_JOIN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::SqlJoinClauseList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ORDER_BY_EXPRESSION_LIST => {
                    let $pattern = unsafe { $crate::SqlOrderByExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SELECT_ITEM_LIST => {
                    let $pattern = unsafe { $crate::SqlSelectItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_ITEM_LIST => {
                    let $pattern = unsafe { $crate::SqlSetItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_OPERATION_LIST => {
                    let $pattern = unsafe { $crate::SqlSetOperationList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::SqlStatementList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TABLE_NAME_LIST => {
                    let $pattern = unsafe { $crate::SqlTableNameList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_TYPE_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::SqlTypeArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_VALUES_ROW_LIST => {
                    let $pattern = unsafe { $crate::SqlValuesRowList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_PARTITION_BY_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::SqlWindowPartitionByItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
