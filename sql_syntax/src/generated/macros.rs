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
                $crate::SqlSyntaxKind::PSQL_ALTER_COLUMN_USING => {
                    let $pattern = unsafe { $crate::PsqlAlterColumnUsing::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ANALYZE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlAnalyzeStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ARRAY_SUBQUERY_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlArraySubqueryExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlArraySubscriptExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ATTACH_PARTITION_ACTION => {
                    let $pattern =
                        unsafe { $crate::PsqlAttachPartitionAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CAST_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlCastExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_FILE => {
                    let $pattern = unsafe { $crate::PsqlCopyFile::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_OPTION => {
                    let $pattern = unsafe { $crate::PsqlCopyOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_OPTIONS => {
                    let $pattern = unsafe { $crate::PsqlCopyOptions::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlCopyStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_STDIO => {
                    let $pattern = unsafe { $crate::PsqlCopyStdio::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_COPY_TABLE_SOURCE => {
                    let $pattern = unsafe { $crate::PsqlCopyTableSource::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_EXTENSION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateExtensionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_FUNCTION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_MATERIALIZED_VIEW_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateMaterializedViewStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_POLICY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreatePolicyStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_CREATE_TABLE_PARTITION_OF_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlCreateTablePartitionOfStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_DETACH_PARTITION_ACTION => {
                    let $pattern =
                        unsafe { $crate::PsqlDetachPartitionAction::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_DO_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlDoStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DO_UPDATE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlDoUpdateClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_EXTENSION_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlDropExtensionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_FUNCTION_PARAMETERS => {
                    let $pattern =
                        unsafe { $crate::PsqlDropFunctionParameters::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_DROP_MATERIALIZED_VIEW_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlDropMaterializedViewStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_EXTENSION_SCHEMA_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlExtensionSchemaClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_EXTRACT_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlExtractExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_FIELD_SELECTION_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlFieldSelectionExpression::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_GENERATED_COLUMN_CONSTRAINT => {
                    let $pattern =
                        unsafe { $crate::PsqlGeneratedColumnConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_IDENTITY_CONSTRAINT => {
                    let $pattern = unsafe { $crate::PsqlIdentityConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_INDEX_METHOD_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlIndexMethodClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_INTERVAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlIntervalExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_INTO_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlIntoClause::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_LOCK_MODE => {
                    let $pattern = unsafe { $crate::PsqlLockMode::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_LOCK_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlLockStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_MULTI_COLUMN_SET_ITEM => {
                    let $pattern = unsafe { $crate::PsqlMultiColumnSetItem::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_NAMED_ARGUMENT => {
                    let $pattern = unsafe { $crate::PsqlNamedArgument::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_ON_COMMIT_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOnCommitClause::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_OVERRIDING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlOverridingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARAMETER_DEFAULT => {
                    let $pattern = unsafe { $crate::PsqlParameterDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_BY_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPartitionByClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_DEFAULT_BOUND => {
                    let $pattern =
                        unsafe { $crate::PsqlPartitionDefaultBound::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_FOR_VALUES_BOUND => {
                    let $pattern =
                        unsafe { $crate::PsqlPartitionForValuesBound::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_HASH_VALUES => {
                    let $pattern = unsafe { $crate::PsqlPartitionHashValues::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_IN_VALUES => {
                    let $pattern = unsafe { $crate::PsqlPartitionInValues::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_RANGE_VALUES => {
                    let $pattern = unsafe { $crate::PsqlPartitionRangeValues::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PARTITION_VALUE_LIST => {
                    let $pattern = unsafe { $crate::PsqlPartitionValueList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_ASSIGNMENT_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlPlAssignmentStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_BLOCK => {
                    let $pattern = unsafe { $crate::PsqlPlBlock::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_BODY => {
                    let $pattern = unsafe { $crate::PsqlPlBody::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_CASE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlCaseStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_CASE_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlCaseWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_CLOSE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlCloseStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_CURSOR_DECLARATION => {
                    let $pattern = unsafe { $crate::PsqlPlCursorDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_DECLARE_SECTION => {
                    let $pattern = unsafe { $crate::PsqlPlDeclareSection::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_DIAGNOSTICS_ITEM => {
                    let $pattern = unsafe { $crate::PsqlPlDiagnosticsItem::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_ELSE_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlElseClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_ELSIF_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlElsifClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXCEPTION_HANDLER => {
                    let $pattern = unsafe { $crate::PsqlPlExceptionHandler::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXCEPTION_SECTION => {
                    let $pattern = unsafe { $crate::PsqlPlExceptionSection::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXECUTE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlExecuteStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXIT_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlExitStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXIT_WHEN_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlExitWhenClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FETCH_DIRECTION => {
                    let $pattern = unsafe { $crate::PsqlPlFetchDirection::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FETCH_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlFetchStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOR_EXECUTE_SOURCE => {
                    let $pattern = unsafe { $crate::PsqlPlForExecuteSource::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOR_RANGE_SOURCE => {
                    let $pattern = unsafe { $crate::PsqlPlForRangeSource::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOR_RANGE_STEP => {
                    let $pattern = unsafe { $crate::PsqlPlForRangeStep::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOR_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlForStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOREACH_SLICE => {
                    let $pattern = unsafe { $crate::PsqlPlForeachSlice::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOREACH_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlForeachStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_GET_DIAGNOSTICS_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlPlGetDiagnosticsStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_IF_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlIfStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_LABEL => {
                    let $pattern = unsafe { $crate::PsqlPlLabel::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_LOOP_BODY => {
                    let $pattern = unsafe { $crate::PsqlPlLoopBody::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_LOOP_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlLoopStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_NULL_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlNullStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_OPEN_ARGUMENTS => {
                    let $pattern = unsafe { $crate::PsqlPlOpenArguments::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_OPEN_FOR_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlOpenForClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_OPEN_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlOpenStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_PERFORM_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlPerformStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RAISE_OPTION => {
                    let $pattern = unsafe { $crate::PsqlPlRaiseOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RAISE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlRaiseStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RAISE_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlRaiseUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_REPEATED_DECLARE => {
                    let $pattern = unsafe { $crate::PsqlPlRepeatedDeclare::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RETURN_QUERY_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::PsqlPlReturnQueryStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RETURN_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlReturnStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_SCROLL_OPTION => {
                    let $pattern = unsafe { $crate::PsqlPlScrollOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_SQLSTATE_CONDITION => {
                    let $pattern = unsafe { $crate::PsqlPlSqlstateCondition::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_USING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlPlUsingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_VARIABLE_DECLARATION => {
                    let $pattern =
                        unsafe { $crate::PsqlPlVariableDeclaration::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_VARIABLE_DEFAULT => {
                    let $pattern = unsafe { $crate::PsqlPlVariableDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_WHILE_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlPlWhileStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_POSITION_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlPositionExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_POSITIONAL_PARAMETER_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlPositionalParameterExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_REFRESH_MATERIALIZED_VIEW_STATEMENT => {
                    let $pattern = unsafe {
                        $crate::PsqlRefreshMaterializedViewStatement::new_unchecked(node)
                    };
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
                $crate::SqlSyntaxKind::PSQL_SEQUENCE_OPTION => {
                    let $pattern = unsafe { $crate::PsqlSequenceOption::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SEQUENCE_OPTIONS => {
                    let $pattern = unsafe { $crate::PsqlSequenceOptions::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_TABLE_AND_COLUMNS => {
                    let $pattern = unsafe { $crate::PsqlTableAndColumns::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TABLE_LIKE_ELEMENT => {
                    let $pattern = unsafe { $crate::PsqlTableLikeElement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TABLE_QUERY => {
                    let $pattern = unsafe { $crate::PsqlTableQuery::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_ARRAY_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlTildeArrayExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_ARRAY_SUBSCRIPT_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::PsqlTildeArraySubscriptExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTildeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TILDE_NAME_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlTildeNameExpression::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_TRIM_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlTrimExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TRUNCATE_IDENTITY_CLAUSE => {
                    let $pattern =
                        unsafe { $crate::PsqlTruncateIdentityClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TYPE_ARRAY_SUFFIX => {
                    let $pattern = unsafe { $crate::PsqlTypeArraySuffix::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_VACUUM_STATEMENT => {
                    let $pattern = unsafe { $crate::PsqlVacuumStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_WITH_DATA_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlWithDataClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_BY_MECHANISM => {
                    let $pattern = unsafe { $crate::PsqlXmlByMechanism::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_COLUMN_DEFAULT => {
                    let $pattern = unsafe { $crate::PsqlXmlColumnDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_COLUMN_PATH => {
                    let $pattern = unsafe { $crate::PsqlXmlColumnPath::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_EXISTS_EXPRESSION => {
                    let $pattern = unsafe { $crate::PsqlXmlExistsExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_NAMESPACE => {
                    let $pattern = unsafe { $crate::PsqlXmlNamespace::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_NAMESPACES_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlXmlNamespacesClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_PASSING_CLAUSE => {
                    let $pattern = unsafe { $crate::PsqlXmlPassingClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_TABLE_BINDING => {
                    let $pattern = unsafe { $crate::PsqlXmlTableBinding::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_TABLE_ORDINALITY_COLUMN => {
                    let $pattern =
                        unsafe { $crate::PsqlXmlTableOrdinalityColumn::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_TABLE_VALUE_COLUMN => {
                    let $pattern = unsafe { $crate::PsqlXmlTableValueColumn::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ADD_COLUMN_ACTION => {
                    let $pattern = unsafe { $crate::SqlAddColumnAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ADD_CONSTRAINT_ACTION => {
                    let $pattern = unsafe { $crate::SqlAddConstraintAction::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_ALL_TRIGGERS_TARGET => {
                    let $pattern = unsafe { $crate::SqlAllTriggersTarget::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_COLUMN_ACTION => {
                    let $pattern = unsafe { $crate::SqlAlterColumnAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_COLUMN_TYPE => {
                    let $pattern = unsafe { $crate::SqlAlterColumnType::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_INDEX_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlAlterIndexStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_TABLE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlAlterTableStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_TRIGGER_ACTION => {
                    let $pattern = unsafe { $crate::SqlAlterTriggerAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ANY_ALL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlAnyAllExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_BEGIN_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlBeginStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_CALL_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlCallStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_CHECK_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlCheckConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COL_REFERENCE => {
                    let $pattern = unsafe { $crate::SqlColReference::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLLATE_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlCollateConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLLATE_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlCollateExpression::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_COMMIT_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlCommitStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COMPOUND_SELECT_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::SqlCompoundSelectStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CONSTRAINT_NAME => {
                    let $pattern = unsafe { $crate::SqlConstraintName::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CREATE_INDEX_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlCreateIndexStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CREATE_TABLE_AS_STATEMENT => {
                    let $pattern =
                        unsafe { $crate::SqlCreateTableAsStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_DEFAULT_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlDefaultConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DELETE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDeleteStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_COLUMN_ACTION => {
                    let $pattern = unsafe { $crate::SqlDropColumnAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_COLUMN_DEFAULT => {
                    let $pattern = unsafe { $crate::SqlDropColumnDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_COLUMN_NOT_NULL => {
                    let $pattern = unsafe { $crate::SqlDropColumnNotNull::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_CONSTRAINT_ACTION => {
                    let $pattern = unsafe { $crate::SqlDropConstraintAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_FUNCTION_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDropFunctionStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_DROP_INDEX_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlDropIndexStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_FOREIGN_KEY_TABLE_CONSTRAINT => {
                    let $pattern =
                        unsafe { $crate::SqlForeignKeyTableConstraint::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_GO_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlGoStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_GRANT_OPTION_FOR => {
                    let $pattern = unsafe { $crate::SqlGrantOptionFor::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_INDEX_INCLUDE_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlIndexIncludeClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_INSERT_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlInsertStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_IS_DISTINCT_FROM_EXPRESSION => {
                    let $pattern =
                        unsafe { $crate::SqlIsDistinctFromExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_IS_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlIsExpression::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_NOT_NULL_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlNotNullConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NULL_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlNullConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NULL_LITERAL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlNullLiteralExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_NULLS_ORDER => {
                    let $pattern = unsafe { $crate::SqlNullsOrder::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_POSTFIX_NULL_EXPRESSION => {
                    let $pattern = unsafe { $crate::SqlPostfixNullExpression::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRECISION_MODIFIER => {
                    let $pattern = unsafe { $crate::SqlPrecisionModifier::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRIMARY_KEY_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlPrimaryKeyConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRIMARY_KEY_TABLE_CONSTRAINT => {
                    let $pattern =
                        unsafe { $crate::SqlPrimaryKeyTableConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRIVILEGE => {
                    let $pattern = unsafe { $crate::SqlPrivilege::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRIVILEGE_ALL_IN_SCHEMA => {
                    let $pattern = unsafe { $crate::SqlPrivilegeAllInSchema::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_PRIVILEGE_OBJECTS => {
                    let $pattern = unsafe { $crate::SqlPrivilegeObjects::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_REFERENCES_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlReferencesConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_RENAME_COLUMN_ACTION => {
                    let $pattern = unsafe { $crate::SqlRenameColumnAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_RENAME_CONSTRAINT_ACTION => {
                    let $pattern =
                        unsafe { $crate::SqlRenameConstraintAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_RENAME_TABLE_ACTION => {
                    let $pattern = unsafe { $crate::SqlRenameTableAction::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_REVOKE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlRevokeStatement::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ROLLBACK_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlRollbackStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_SET_COLUMN_DEFAULT => {
                    let $pattern = unsafe { $crate::SqlSetColumnDefault::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_SET_COLUMN_NOT_NULL => {
                    let $pattern = unsafe { $crate::SqlSetColumnNotNull::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_SET_SCHEMA_ACTION => {
                    let $pattern = unsafe { $crate::SqlSetSchemaAction::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_TABLE_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlTableConstraint::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_TRUNCATE_STATEMENT => {
                    let $pattern = unsafe { $crate::SqlTruncateStatement::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_UNIQUE_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlUniqueConstraint::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_UNIQUE_TABLE_CONSTRAINT => {
                    let $pattern = unsafe { $crate::SqlUniqueTableConstraint::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_WINDOW_FRAME_CLAUSE => {
                    let $pattern = unsafe { $crate::SqlWindowFrameClause::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_FRAME_CURRENT_ROW_BOUND => {
                    let $pattern =
                        unsafe { $crate::SqlWindowFrameCurrentRowBound::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_FRAME_OFFSET_BOUND => {
                    let $pattern =
                        unsafe { $crate::SqlWindowFrameOffsetBound::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_WINDOW_FRAME_UNBOUNDED_BOUND => {
                    let $pattern =
                        unsafe { $crate::SqlWindowFrameUnboundedBound::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_WITH_GRANT_OPTION => {
                    let $pattern = unsafe { $crate::SqlWithGrantOption::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_COPY_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlCopyOptionList::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_PL_CASE_WHEN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlCaseWhenClauseList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_DECLARATION_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlDeclarationList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_DIAGNOSTICS_ITEM_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlPlDiagnosticsItemList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_ELSIF_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlElsifClauseList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXCEPTION_CONDITION_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlPlExceptionConditionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_EXCEPTION_HANDLER_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlPlExceptionHandlerList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_FOR_TARGET_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlForTargetList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_RAISE_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlRaiseOptionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_PL_STATEMENT_LIST => {
                    let $pattern = unsafe { $crate::PsqlPlStatementList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_RETURNS_TABLE_COLUMN_LIST => {
                    let $pattern =
                        unsafe { $crate::PsqlReturnsTableColumnList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_SEQUENCE_OPTION_LIST => {
                    let $pattern = unsafe { $crate::PsqlSequenceOptionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_TABLE_AND_COLUMNS_LIST => {
                    let $pattern = unsafe { $crate::PsqlTableAndColumnsList::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::PSQL_XML_NAMESPACE_LIST => {
                    let $pattern = unsafe { $crate::PsqlXmlNamespaceList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::PSQL_XML_TABLE_COLUMN_LIST => {
                    let $pattern = unsafe { $crate::PsqlXmlTableColumnList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALIAS_COLUMN_DEFINITION_LIST => {
                    let $pattern =
                        unsafe { $crate::SqlAliasColumnDefinitionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_ALTER_TABLE_ACTION_LIST => {
                    let $pattern = unsafe { $crate::SqlAlterTableActionList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CALL_ARGUMENT_LIST => {
                    let $pattern = unsafe { $crate::SqlCallArgumentList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_CASE_WHEN_CLAUSE_LIST => {
                    let $pattern = unsafe { $crate::SqlCaseWhenClauseList::new_unchecked(node) };
                    $body
                }
                $crate::SqlSyntaxKind::SQL_COLUMN_CONSTRAINT_LIST => {
                    let $pattern = unsafe { $crate::SqlColumnConstraintList::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_INDEX_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::SqlIndexElementList::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_PRIVILEGE_LIST => {
                    let $pattern = unsafe { $crate::SqlPrivilegeList::new_unchecked(node) };
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
                $crate::SqlSyntaxKind::SQL_TABLE_ELEMENT_LIST => {
                    let $pattern = unsafe { $crate::SqlTableElementList::new_unchecked(node) };
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
