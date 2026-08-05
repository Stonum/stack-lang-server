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
            node => match $crate::PsqlSyntaxNode::kind(&node) {
                $crate::PsqlSyntaxKind::PSQL_ALIAS => {
                    let $pattern = unsafe { $crate::PsqlAlias::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ALIAS_COLUMN_DEFINITION => {
                    let $pattern =
                        unsafe { $crate::PsqlAliasColumnDefinition::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ALIAS_COLUMN_LIST => {
                    let $pattern = unsafe { $crate::PsqlAliasColumnList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ANY_ALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlAnyAllExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlArraySubscriptExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BETWEEN_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlBetweenExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BINARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlBinaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOOLEAN_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlBooleanLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlCallExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CASE_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlCaseElseClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CASE_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlCaseExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CASE_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlCaseWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CAST_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlCastExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CAST_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlCastFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::PsqlColReference::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COLUMN_DEFINITION => {
                    let $pattern = unsafe { $crate::PsqlColumnDefinition::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COLUMN_LIST => {
                    let $pattern = unsafe { $crate::PsqlColumnList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CREATE_FUNCTION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CREATE_POLICY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreatePolicyStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CREATE_TABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlCreateTableStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CREATE_TRIGGER_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateTriggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CREATE_VIEW_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlCreateViewStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CTE_DEFINITION => {
                    let $pattern = unsafe { $crate::PsqlCteDefinition::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CTE_MATERIALIZED_HINT => {
                    let $pattern = unsafe { $crate::PsqlCteMaterializedHint::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DATA_BASE_NAME => {
                    let $pattern = unsafe { $crate::PsqlDataBaseName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DELETE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDeleteStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DELETE_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDeleteUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DISTINCT_ON_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDistinctOnClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DO_NOTHING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDoNothingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DO_UPDATE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDoUpdateClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_FUNCTION_PARAMETERS => {
                    let $pattern =
                        unsafe { $crate::PsqlDropFunctionParameters::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_FUNCTION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlDropFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_POLICY_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropPolicyStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_TABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropTableStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_TRIGGER_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropTriggerStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_DROP_VIEW_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDropViewStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_EMPTY_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlEmptyStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_EXISTS_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlExistsExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FETCH_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlFetchClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FETCH_ONLY_TAIL => {
                    let $pattern = unsafe { $crate::PsqlFetchOnlyTail::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FETCH_WITH_TIES_TAIL => {
                    let $pattern = unsafe { $crate::PsqlFetchWithTiesTail::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FILTER_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlFilterClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlFromClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FROM_ITEM => {
                    let $pattern = unsafe { $crate::PsqlFromItem::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FUNCTION_BINDING => {
                    let $pattern = unsafe { $crate::PsqlFunctionBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FUNCTION_PARAMETER => {
                    let $pattern = unsafe { $crate::PsqlFunctionParameter::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GRANT_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlGrantStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GROUP_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlGroupByClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_HAVING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlHavingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_IN_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlInExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_IN_VALUE_LIST => {
                    let $pattern = unsafe { $crate::PsqlInValueList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_INSERT_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlInsertStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_INTERVAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlIntervalExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_IS_NULL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlIsNullExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_JOIN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlJoinClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_JOIN_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlJoinUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LANGUAGE_OPTION => {
                    let $pattern = unsafe { $crate::PsqlLanguageOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LIKE_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlLikeExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LIMIT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlLimitClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_LOGICAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlLogicalExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NAME => {
                    let $pattern = unsafe { $crate::PsqlName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NULL_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_NUMBER_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlNumberLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_OFFSET_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOffsetClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ON_CONFLICT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOnConflictClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ON_CONSTRAINT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOnConstraintClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOrderByClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlOrderByExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PARAMETER_DEFAULT => {
                    let $pattern = unsafe { $crate::PsqlParameterDefault::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PARAMETER_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlParameterExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PARENTHESIZED_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlParenthesizedExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PARENTHESIZED_JOIN_BINDING => {
                    let $pattern =
                        unsafe { $crate::PsqlParenthesizedJoinBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_POLICY_FOR_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPolicyForClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_POLICY_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPolicyUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_POLICY_WITH_CHECK_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlPolicyWithCheckClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_PRECISION_MODIFIER => {
                    let $pattern = unsafe { $crate::PsqlPrecisionModifier::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturningClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_NULL_OPTION => {
                    let $pattern = unsafe { $crate::PsqlReturnsNullOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_SETOF_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsSetofClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_TABLE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsTableClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN => {
                    let $pattern = unsafe { $crate::PsqlReturnsTableColumn::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_TRIGGER_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlReturnsTriggerClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ROOT => {
                    let $pattern = unsafe { $crate::PsqlRoot::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SECURITY_OPTION => {
                    let $pattern = unsafe { $crate::PsqlSecurityOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_ALL_QUANTIFIER => {
                    let $pattern = unsafe { $crate::PsqlSelectAllQuantifier::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSelectClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_DISTINCT_QUANTIFIER => {
                    let $pattern =
                        unsafe { $crate::PsqlSelectDistinctQuantifier::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlSelectExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlSelectStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SET_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSetClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SET_ITEM => {
                    let $pattern = unsafe { $crate::PsqlSetItem::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SET_OPERATION => {
                    let $pattern = unsafe { $crate::PsqlSetOperation::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SHEMA_NAME => {
                    let $pattern = unsafe { $crate::PsqlShemaName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STAR => {
                    let $pattern = unsafe { $crate::PsqlStar::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STRICT_OPTION => {
                    let $pattern = unsafe { $crate::PsqlStrictOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STRING_LITERAL_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlStringLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUBQUERY_BINDING => {
                    let $pattern = unsafe { $crate::PsqlSubqueryBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUBQUERY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlSubqueryExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUBSTRING_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlSubstringExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUBSTRING_FOR_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSubstringForClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SUBSTRING_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlSubstringFromClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_BINDING => {
                    let $pattern = unsafe { $crate::PsqlTableBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::PsqlTableColReference::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_NAME => {
                    let $pattern = unsafe { $crate::PsqlTableName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_STAR => {
                    let $pattern = unsafe { $crate::PsqlTableStar::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TILDE_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlTildeArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TILDE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTildeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TILDE_NAME => {
                    let $pattern = unsafe { $crate::PsqlTildeName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TIME_ZONE_MODIFIER => {
                    let $pattern = unsafe { $crate::PsqlTimeZoneModifier::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_EVENT => {
                    let $pattern = unsafe { $crate::PsqlTriggerEvent::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_FOR_EACH_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlTriggerForEachClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_REFERENCING_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingItem::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_UPDATE_OF_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerUpdateOfClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlTriggerWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TYPE_ARGUMENTS => {
                    let $pattern = unsafe { $crate::PsqlTypeArguments::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TYPE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTypeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TYPE_NAME => {
                    let $pattern = unsafe { $crate::PsqlTypeName::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_UNARY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlUnaryExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_UPDATE_FROM_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlUpdateFromClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_UPDATE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlUpdateStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VALUES_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlValuesClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VALUES_ROW => {
                    let $pattern = unsafe { $crate::PsqlValuesRow::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VARYING_MODIFIER => {
                    let $pattern = unsafe { $crate::PsqlVaryingModifier::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VIEW_OPTION => {
                    let $pattern = unsafe { $crate::PsqlViewOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VIEW_OPTIONS => {
                    let $pattern = unsafe { $crate::PsqlViewOptions::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VOLATILITY_OPTION => {
                    let $pattern = unsafe { $crate::PsqlVolatilityOption::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WHERE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlWhereClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WINDOW_FUNCTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlWindowFunctionExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WINDOW_PARTITION_BY_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlWindowPartitionByClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WINDOW_SPECIFICATION => {
                    let $pattern = unsafe { $crate::PsqlWindowSpecification::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WITH_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlWithClause::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS => {
                    let $pattern = unsafe { $crate::PsqlBogus::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_ASSIGNMENT => {
                    let $pattern = unsafe { $crate::PsqlBogusAssignment::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_BINDING => {
                    let $pattern = unsafe { $crate::PsqlBogusBinding::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlBogusExpression::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_MEMBER => {
                    let $pattern = unsafe { $crate::PsqlBogusMember::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_PARAMETER => {
                    let $pattern = unsafe { $crate::PsqlBogusParameter::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_BOGUS_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlBogusStatement::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ALIAS_COLUMN_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlAliasColumnDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CASE_WHEN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::PsqlCaseWhenClauseList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COLUMN_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::PsqlColumnDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_COLUMN_NAME_LIST => {
                    let $pattern = unsafe { $crate::PsqlColumnNameList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_CTE_DEFINITION_LIST => {
                    let $pattern = unsafe { $crate::PsqlCteDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_EXPRESSION_LIST => {
                    let $pattern = unsafe { $crate::PsqlExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FROM_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlFromItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FUNCTION_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlFunctionOptionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_FUNCTION_PARAMETER_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlFunctionParameterList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GRANTEE_LIST => {
                    let $pattern = unsafe { $crate::PsqlGranteeList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_GROUP_BY_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlGroupByItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_JOIN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::PsqlJoinClauseList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_ORDER_BY_EXPRESSION_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlOrderByExpressionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlReturnsTableColumnList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SELECT_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlSelectItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SET_ITEM_LIST => {
                    let $pattern = unsafe { $crate::PsqlSetItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_SET_OPERATION_LIST => {
                    let $pattern = unsafe { $crate::PsqlSetOperationList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlStatementList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TABLE_NAME_LIST => {
                    let $pattern = unsafe { $crate::PsqlTableNameList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_EVENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlTriggerEventList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TRIGGER_REFERENCING_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlTriggerReferencingItemList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TYPE_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlTypeArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_TYPE_NAME_LIST => {
                    let $pattern = unsafe { $crate::PsqlTypeNameList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VALUES_ROW_LIST => {
                    let $pattern = unsafe { $crate::PsqlValuesRowList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_VIEW_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlViewOptionList::new_unchecked(node) };
                    $body
                }
                $crate::PsqlSyntaxKind::PSQL_WINDOW_PARTITION_BY_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlWindowPartitionByItemList::new_unchecked(node) };
                    $body
                }
                _ => unreachable!(),
            },
        }
    };
}
pub(crate) use map_syntax_node;
