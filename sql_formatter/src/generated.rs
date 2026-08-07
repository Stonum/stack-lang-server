//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, SqlFormatContext, SqlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<sql_syntax::PsqlArrayExpression>
    for crate::psql::expressions::array_expression::FormatPsqlArrayExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlArrayExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlArrayExpression,
        crate::psql::expressions::array_expression::FormatPsqlArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::array_expression::FormatPsqlArrayExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlArrayExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlArrayExpression,
        crate::psql::expressions::array_expression::FormatPsqlArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::array_expression::FormatPsqlArrayExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlArraySubscriptExpression>
    for crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlArraySubscriptExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlArraySubscriptExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlArraySubscriptExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlArraySubscriptExpression,
        crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: array_subscript_expression :: FormatPsqlArraySubscriptExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlArraySubscriptExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlArraySubscriptExpression,
        crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: array_subscript_expression :: FormatPsqlArraySubscriptExpression :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlCastExpression>
    for crate::psql::expressions::cast_expression::FormatPsqlCastExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCastExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCastExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCastExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCastExpression,
        crate::psql::expressions::cast_expression::FormatPsqlCastExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::cast_expression::FormatPsqlCastExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCastExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCastExpression,
        crate::psql::expressions::cast_expression::FormatPsqlCastExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::cast_expression::FormatPsqlCastExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCreateFunctionStatement>
    for crate::psql::statements::create_function_statement::FormatPsqlCreateFunctionStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCreateFunctionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCreateFunctionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreateFunctionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCreateFunctionStatement,
        crate::psql::statements::create_function_statement::FormatPsqlCreateFunctionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_function_statement :: FormatPsqlCreateFunctionStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreateFunctionStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCreateFunctionStatement,
        crate::psql::statements::create_function_statement::FormatPsqlCreateFunctionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_function_statement :: FormatPsqlCreateFunctionStatement :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlCreatePolicyStatement>
    for crate::psql::statements::create_policy_statement::FormatPsqlCreatePolicyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCreatePolicyStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCreatePolicyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreatePolicyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCreatePolicyStatement,
        crate::psql::statements::create_policy_statement::FormatPsqlCreatePolicyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_policy_statement :: FormatPsqlCreatePolicyStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreatePolicyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCreatePolicyStatement,
        crate::psql::statements::create_policy_statement::FormatPsqlCreatePolicyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_policy_statement :: FormatPsqlCreatePolicyStatement :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlCreateTriggerStatement>
    for crate::psql::statements::create_trigger_statement::FormatPsqlCreateTriggerStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCreateTriggerStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCreateTriggerStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreateTriggerStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCreateTriggerStatement,
        crate::psql::statements::create_trigger_statement::FormatPsqlCreateTriggerStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_trigger_statement :: FormatPsqlCreateTriggerStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreateTriggerStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCreateTriggerStatement,
        crate::psql::statements::create_trigger_statement::FormatPsqlCreateTriggerStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_trigger_statement :: FormatPsqlCreateTriggerStatement :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlCteMaterializedHint>
    for crate::psql::auxiliary::cte_materialized_hint::FormatPsqlCteMaterializedHint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCteMaterializedHint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCteMaterializedHint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCteMaterializedHint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCteMaterializedHint,
        crate::psql::auxiliary::cte_materialized_hint::FormatPsqlCteMaterializedHint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::cte_materialized_hint::FormatPsqlCteMaterializedHint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCteMaterializedHint {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCteMaterializedHint,
        crate::psql::auxiliary::cte_materialized_hint::FormatPsqlCteMaterializedHint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::cte_materialized_hint::FormatPsqlCteMaterializedHint::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDeleteUsingClause>
    for crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDeleteUsingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDeleteUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDeleteUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDeleteUsingClause,
        crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDeleteUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDeleteUsingClause,
        crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDistinctOnClause>
    for crate::psql::clauses::distinct_on_clause::FormatPsqlDistinctOnClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDistinctOnClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDistinctOnClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDistinctOnClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDistinctOnClause,
        crate::psql::clauses::distinct_on_clause::FormatPsqlDistinctOnClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::distinct_on_clause::FormatPsqlDistinctOnClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDistinctOnClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDistinctOnClause,
        crate::psql::clauses::distinct_on_clause::FormatPsqlDistinctOnClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::distinct_on_clause::FormatPsqlDistinctOnClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDoNothingClause>
    for crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDoNothingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDoNothingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDoNothingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDoNothingClause,
        crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDoNothingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDoNothingClause,
        crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDoUpdateClause>
    for crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlDoUpdateClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDoUpdateClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDoUpdateClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDoUpdateClause,
        crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDoUpdateClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDoUpdateClause,
        crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDropFunctionParameters>
    for crate::psql::auxiliary::drop_function_parameters::FormatPsqlDropFunctionParameters
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDropFunctionParameters,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDropFunctionParameters>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDropFunctionParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDropFunctionParameters,
        crate::psql::auxiliary::drop_function_parameters::FormatPsqlDropFunctionParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: drop_function_parameters :: FormatPsqlDropFunctionParameters :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDropFunctionParameters {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDropFunctionParameters,
        crate::psql::auxiliary::drop_function_parameters::FormatPsqlDropFunctionParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: drop_function_parameters :: FormatPsqlDropFunctionParameters :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlDropPolicyStatement>
    for crate::psql::statements::drop_policy_statement::FormatPsqlDropPolicyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDropPolicyStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDropPolicyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDropPolicyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDropPolicyStatement,
        crate::psql::statements::drop_policy_statement::FormatPsqlDropPolicyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::drop_policy_statement::FormatPsqlDropPolicyStatement::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDropPolicyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDropPolicyStatement,
        crate::psql::statements::drop_policy_statement::FormatPsqlDropPolicyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::drop_policy_statement::FormatPsqlDropPolicyStatement::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::PsqlDropTriggerStatement>
    for crate::psql::statements::drop_trigger_statement::FormatPsqlDropTriggerStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDropTriggerStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDropTriggerStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDropTriggerStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDropTriggerStatement,
        crate::psql::statements::drop_trigger_statement::FormatPsqlDropTriggerStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: drop_trigger_statement :: FormatPsqlDropTriggerStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDropTriggerStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDropTriggerStatement,
        crate::psql::statements::drop_trigger_statement::FormatPsqlDropTriggerStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: drop_trigger_statement :: FormatPsqlDropTriggerStatement :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlFilterClause>
    for crate::psql::clauses::filter_clause::FormatPsqlFilterClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlFilterClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlFilterClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlFilterClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlFilterClause,
        crate::psql::clauses::filter_clause::FormatPsqlFilterClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::filter_clause::FormatPsqlFilterClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlFilterClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlFilterClause,
        crate::psql::clauses::filter_clause::FormatPsqlFilterClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::filter_clause::FormatPsqlFilterClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlFunctionParameter>
    for crate::psql::auxiliary::function_parameter::FormatPsqlFunctionParameter
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlFunctionParameter,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlFunctionParameter>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlFunctionParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlFunctionParameter,
        crate::psql::auxiliary::function_parameter::FormatPsqlFunctionParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::function_parameter::FormatPsqlFunctionParameter::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlFunctionParameter {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlFunctionParameter,
        crate::psql::auxiliary::function_parameter::FormatPsqlFunctionParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::function_parameter::FormatPsqlFunctionParameter::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlIntervalExpression>
    for crate::psql::expressions::interval_expression::FormatPsqlIntervalExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlIntervalExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlIntervalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlIntervalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlIntervalExpression,
        crate::psql::expressions::interval_expression::FormatPsqlIntervalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::interval_expression::FormatPsqlIntervalExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlIntervalExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlIntervalExpression,
        crate::psql::expressions::interval_expression::FormatPsqlIntervalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::interval_expression::FormatPsqlIntervalExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlJoinUsingClause>
    for crate::psql::clauses::join_using_clause::FormatPsqlJoinUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlJoinUsingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlJoinUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlJoinUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlJoinUsingClause,
        crate::psql::clauses::join_using_clause::FormatPsqlJoinUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::join_using_clause::FormatPsqlJoinUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlJoinUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlJoinUsingClause,
        crate::psql::clauses::join_using_clause::FormatPsqlJoinUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::join_using_clause::FormatPsqlJoinUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlLanguageOption>
    for crate::psql::auxiliary::language_option::FormatPsqlLanguageOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlLanguageOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlLanguageOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlLanguageOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlLanguageOption,
        crate::psql::auxiliary::language_option::FormatPsqlLanguageOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::language_option::FormatPsqlLanguageOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlLanguageOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlLanguageOption,
        crate::psql::auxiliary::language_option::FormatPsqlLanguageOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::language_option::FormatPsqlLanguageOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlLimitClause>
    for crate::psql::clauses::limit_clause::FormatPsqlLimitClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlLimitClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlLimitClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlLimitClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlLimitClause,
        crate::psql::clauses::limit_clause::FormatPsqlLimitClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::limit_clause::FormatPsqlLimitClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlLimitClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlLimitClause,
        crate::psql::clauses::limit_clause::FormatPsqlLimitClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::limit_clause::FormatPsqlLimitClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlOnConflictClause>
    for crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlOnConflictClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlOnConflictClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlOnConflictClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlOnConflictClause,
        crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlOnConflictClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlOnConflictClause,
        crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlOnConstraintClause>
    for crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlOnConstraintClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlOnConstraintClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlOnConstraintClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlOnConstraintClause,
        crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlOnConstraintClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlOnConstraintClause,
        crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlParameterDefault>
    for crate::psql::auxiliary::parameter_default::FormatPsqlParameterDefault
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlParameterDefault,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlParameterDefault>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlParameterDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlParameterDefault,
        crate::psql::auxiliary::parameter_default::FormatPsqlParameterDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::parameter_default::FormatPsqlParameterDefault::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlParameterDefault {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlParameterDefault,
        crate::psql::auxiliary::parameter_default::FormatPsqlParameterDefault,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::parameter_default::FormatPsqlParameterDefault::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPolicyForClause>
    for crate::psql::clauses::policy_for_clause::FormatPsqlPolicyForClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPolicyForClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPolicyForClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPolicyForClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPolicyForClause,
        crate::psql::clauses::policy_for_clause::FormatPsqlPolicyForClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::policy_for_clause::FormatPsqlPolicyForClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPolicyForClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPolicyForClause,
        crate::psql::clauses::policy_for_clause::FormatPsqlPolicyForClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::policy_for_clause::FormatPsqlPolicyForClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPolicyUsingClause>
    for crate::psql::clauses::policy_using_clause::FormatPsqlPolicyUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPolicyUsingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPolicyUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPolicyUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPolicyUsingClause,
        crate::psql::clauses::policy_using_clause::FormatPsqlPolicyUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::policy_using_clause::FormatPsqlPolicyUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPolicyUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPolicyUsingClause,
        crate::psql::clauses::policy_using_clause::FormatPsqlPolicyUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::policy_using_clause::FormatPsqlPolicyUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPolicyWithCheckClause>
    for crate::psql::clauses::policy_with_check_clause::FormatPsqlPolicyWithCheckClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPolicyWithCheckClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPolicyWithCheckClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPolicyWithCheckClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPolicyWithCheckClause,
        crate::psql::clauses::policy_with_check_clause::FormatPsqlPolicyWithCheckClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: clauses :: policy_with_check_clause :: FormatPsqlPolicyWithCheckClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPolicyWithCheckClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPolicyWithCheckClause,
        crate::psql::clauses::policy_with_check_clause::FormatPsqlPolicyWithCheckClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: clauses :: policy_with_check_clause :: FormatPsqlPolicyWithCheckClause :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlReturningClause>
    for crate::psql::clauses::returning_clause::FormatPsqlReturningClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturningClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturningClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturningClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturningClause,
        crate::psql::clauses::returning_clause::FormatPsqlReturningClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returning_clause::FormatPsqlReturningClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturningClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturningClause,
        crate::psql::clauses::returning_clause::FormatPsqlReturningClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returning_clause::FormatPsqlReturningClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsClause>
    for crate::psql::clauses::returns_clause::FormatPsqlReturnsClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlReturnsClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsClause,
        crate::psql::clauses::returns_clause::FormatPsqlReturnsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returns_clause::FormatPsqlReturnsClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsClause,
        crate::psql::clauses::returns_clause::FormatPsqlReturnsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returns_clause::FormatPsqlReturnsClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsNullOption>
    for crate::psql::auxiliary::returns_null_option::FormatPsqlReturnsNullOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturnsNullOption,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsNullOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsNullOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsNullOption,
        crate::psql::auxiliary::returns_null_option::FormatPsqlReturnsNullOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::returns_null_option::FormatPsqlReturnsNullOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsNullOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsNullOption,
        crate::psql::auxiliary::returns_null_option::FormatPsqlReturnsNullOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::returns_null_option::FormatPsqlReturnsNullOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsSetofClause>
    for crate::psql::clauses::returns_setof_clause::FormatPsqlReturnsSetofClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturnsSetofClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsSetofClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsSetofClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsSetofClause,
        crate::psql::clauses::returns_setof_clause::FormatPsqlReturnsSetofClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returns_setof_clause::FormatPsqlReturnsSetofClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsSetofClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsSetofClause,
        crate::psql::clauses::returns_setof_clause::FormatPsqlReturnsSetofClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returns_setof_clause::FormatPsqlReturnsSetofClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsTableClause>
    for crate::psql::clauses::returns_table_clause::FormatPsqlReturnsTableClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturnsTableClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsTableClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsTableClause,
        crate::psql::clauses::returns_table_clause::FormatPsqlReturnsTableClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returns_table_clause::FormatPsqlReturnsTableClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsTableClause,
        crate::psql::clauses::returns_table_clause::FormatPsqlReturnsTableClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returns_table_clause::FormatPsqlReturnsTableClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsTableColumn>
    for crate::psql::auxiliary::returns_table_column::FormatPsqlReturnsTableColumn
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturnsTableColumn,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsTableColumn>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableColumn {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsTableColumn,
        crate::psql::auxiliary::returns_table_column::FormatPsqlReturnsTableColumn,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::returns_table_column::FormatPsqlReturnsTableColumn::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableColumn {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsTableColumn,
        crate::psql::auxiliary::returns_table_column::FormatPsqlReturnsTableColumn,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::returns_table_column::FormatPsqlReturnsTableColumn::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlReturnsTriggerClause>
    for crate::psql::clauses::returns_trigger_clause::FormatPsqlReturnsTriggerClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlReturnsTriggerClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlReturnsTriggerClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTriggerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsTriggerClause,
        crate::psql::clauses::returns_trigger_clause::FormatPsqlReturnsTriggerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returns_trigger_clause::FormatPsqlReturnsTriggerClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTriggerClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsTriggerClause,
        crate::psql::clauses::returns_trigger_clause::FormatPsqlReturnsTriggerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returns_trigger_clause::FormatPsqlReturnsTriggerClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlSecurityOption>
    for crate::psql::auxiliary::security_option::FormatPsqlSecurityOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlSecurityOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSecurityOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSecurityOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSecurityOption,
        crate::psql::auxiliary::security_option::FormatPsqlSecurityOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::security_option::FormatPsqlSecurityOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSecurityOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSecurityOption,
        crate::psql::auxiliary::security_option::FormatPsqlSecurityOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::security_option::FormatPsqlSecurityOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlStrictOption>
    for crate::psql::auxiliary::strict_option::FormatPsqlStrictOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlStrictOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlStrictOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlStrictOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlStrictOption,
        crate::psql::auxiliary::strict_option::FormatPsqlStrictOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::strict_option::FormatPsqlStrictOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlStrictOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlStrictOption,
        crate::psql::auxiliary::strict_option::FormatPsqlStrictOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::strict_option::FormatPsqlStrictOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlSubstringExpression>
    for crate::psql::expressions::substring_expression::FormatPsqlSubstringExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlSubstringExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSubstringExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSubstringExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSubstringExpression,
        crate::psql::expressions::substring_expression::FormatPsqlSubstringExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::substring_expression::FormatPsqlSubstringExpression::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSubstringExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSubstringExpression,
        crate::psql::expressions::substring_expression::FormatPsqlSubstringExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::substring_expression::FormatPsqlSubstringExpression::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::PsqlSubstringForClause>
    for crate::psql::clauses::substring_for_clause::FormatPsqlSubstringForClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlSubstringForClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSubstringForClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSubstringForClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSubstringForClause,
        crate::psql::clauses::substring_for_clause::FormatPsqlSubstringForClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::substring_for_clause::FormatPsqlSubstringForClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSubstringForClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSubstringForClause,
        crate::psql::clauses::substring_for_clause::FormatPsqlSubstringForClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::substring_for_clause::FormatPsqlSubstringForClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlSubstringFromClause>
    for crate::psql::clauses::substring_from_clause::FormatPsqlSubstringFromClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlSubstringFromClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSubstringFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSubstringFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSubstringFromClause,
        crate::psql::clauses::substring_from_clause::FormatPsqlSubstringFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::substring_from_clause::FormatPsqlSubstringFromClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSubstringFromClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSubstringFromClause,
        crate::psql::clauses::substring_from_clause::FormatPsqlSubstringFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::substring_from_clause::FormatPsqlSubstringFromClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTildeArrayExpression>
    for crate::psql::expressions::tilde_array_expression::FormatPsqlTildeArrayExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTildeArrayExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTildeArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTildeArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTildeArrayExpression,
        crate::psql::expressions::tilde_array_expression::FormatPsqlTildeArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: tilde_array_expression :: FormatPsqlTildeArrayExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTildeArrayExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTildeArrayExpression,
        crate::psql::expressions::tilde_array_expression::FormatPsqlTildeArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: tilde_array_expression :: FormatPsqlTildeArrayExpression :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlTildeArraySuffix>
    for crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTildeArraySuffix,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTildeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTildeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTildeArraySuffix,
        crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTildeArraySuffix {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTildeArraySuffix,
        crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTriggerEvent>
    for crate::psql::auxiliary::trigger_event::FormatPsqlTriggerEvent
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlTriggerEvent, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerEvent>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerEvent {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerEvent,
        crate::psql::auxiliary::trigger_event::FormatPsqlTriggerEvent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::trigger_event::FormatPsqlTriggerEvent::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerEvent {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerEvent,
        crate::psql::auxiliary::trigger_event::FormatPsqlTriggerEvent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::trigger_event::FormatPsqlTriggerEvent::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTriggerForEachClause>
    for crate::psql::clauses::trigger_for_each_clause::FormatPsqlTriggerForEachClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTriggerForEachClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerForEachClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerForEachClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerForEachClause,
        crate::psql::clauses::trigger_for_each_clause::FormatPsqlTriggerForEachClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::trigger_for_each_clause::FormatPsqlTriggerForEachClause::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerForEachClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerForEachClause,
        crate::psql::clauses::trigger_for_each_clause::FormatPsqlTriggerForEachClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::trigger_for_each_clause::FormatPsqlTriggerForEachClause::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTriggerReferencingClause>
    for crate::psql::clauses::trigger_referencing_clause::FormatPsqlTriggerReferencingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTriggerReferencingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerReferencingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerReferencingClause,
        crate::psql::clauses::trigger_referencing_clause::FormatPsqlTriggerReferencingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: clauses :: trigger_referencing_clause :: FormatPsqlTriggerReferencingClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerReferencingClause,
        crate::psql::clauses::trigger_referencing_clause::FormatPsqlTriggerReferencingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: clauses :: trigger_referencing_clause :: FormatPsqlTriggerReferencingClause :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlTriggerReferencingItem>
    for crate::psql::auxiliary::trigger_referencing_item::FormatPsqlTriggerReferencingItem
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTriggerReferencingItem,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerReferencingItem>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerReferencingItem,
        crate::psql::auxiliary::trigger_referencing_item::FormatPsqlTriggerReferencingItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: trigger_referencing_item :: FormatPsqlTriggerReferencingItem :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingItem {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerReferencingItem,
        crate::psql::auxiliary::trigger_referencing_item::FormatPsqlTriggerReferencingItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: trigger_referencing_item :: FormatPsqlTriggerReferencingItem :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlTriggerUpdateOfClause>
    for crate::psql::clauses::trigger_update_of_clause::FormatPsqlTriggerUpdateOfClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTriggerUpdateOfClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerUpdateOfClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerUpdateOfClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerUpdateOfClause,
        crate::psql::clauses::trigger_update_of_clause::FormatPsqlTriggerUpdateOfClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: clauses :: trigger_update_of_clause :: FormatPsqlTriggerUpdateOfClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerUpdateOfClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerUpdateOfClause,
        crate::psql::clauses::trigger_update_of_clause::FormatPsqlTriggerUpdateOfClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: clauses :: trigger_update_of_clause :: FormatPsqlTriggerUpdateOfClause :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlTriggerWhenClause>
    for crate::psql::clauses::trigger_when_clause::FormatPsqlTriggerWhenClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTriggerWhenClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTriggerWhenClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerWhenClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerWhenClause,
        crate::psql::clauses::trigger_when_clause::FormatPsqlTriggerWhenClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::trigger_when_clause::FormatPsqlTriggerWhenClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerWhenClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerWhenClause,
        crate::psql::clauses::trigger_when_clause::FormatPsqlTriggerWhenClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::trigger_when_clause::FormatPsqlTriggerWhenClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTypeArraySuffix>
    for crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTypeArraySuffix,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTypeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTypeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTypeArraySuffix,
        crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTypeArraySuffix {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTypeArraySuffix,
        crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlViewOption>
    for crate::psql::auxiliary::view_option::FormatPsqlViewOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlViewOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlViewOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlViewOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlViewOption,
        crate::psql::auxiliary::view_option::FormatPsqlViewOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::view_option::FormatPsqlViewOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlViewOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlViewOption,
        crate::psql::auxiliary::view_option::FormatPsqlViewOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::view_option::FormatPsqlViewOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlViewOptions>
    for crate::psql::auxiliary::view_options::FormatPsqlViewOptions
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlViewOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlViewOptions>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlViewOptions {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlViewOptions,
        crate::psql::auxiliary::view_options::FormatPsqlViewOptions,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::view_options::FormatPsqlViewOptions::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlViewOptions {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlViewOptions,
        crate::psql::auxiliary::view_options::FormatPsqlViewOptions,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::view_options::FormatPsqlViewOptions::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlVolatilityOption>
    for crate::psql::auxiliary::volatility_option::FormatPsqlVolatilityOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlVolatilityOption,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlVolatilityOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlVolatilityOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlVolatilityOption,
        crate::psql::auxiliary::volatility_option::FormatPsqlVolatilityOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::volatility_option::FormatPsqlVolatilityOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlVolatilityOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlVolatilityOption,
        crate::psql::auxiliary::volatility_option::FormatPsqlVolatilityOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::volatility_option::FormatPsqlVolatilityOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlias> for crate::sql::auxiliary::alias::FormatSqlAlias {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlAlias, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlias>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlias {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::SqlAlias, crate::sql::auxiliary::alias::FormatSqlAlias>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alias::FormatSqlAlias::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlias {
    type Format =
        FormatOwnedWithRule<sql_syntax::SqlAlias, crate::sql::auxiliary::alias::FormatSqlAlias>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alias::FormatSqlAlias::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAliasColumnDefinition>
    for crate::sql::auxiliary::alias_column_definition::FormatSqlAliasColumnDefinition
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAliasColumnDefinition,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAliasColumnDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAliasColumnDefinition,
        crate::sql::auxiliary::alias_column_definition::FormatSqlAliasColumnDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alias_column_definition::FormatSqlAliasColumnDefinition::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnDefinition {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAliasColumnDefinition,
        crate::sql::auxiliary::alias_column_definition::FormatSqlAliasColumnDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alias_column_definition::FormatSqlAliasColumnDefinition::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlAliasColumnList>
    for crate::sql::auxiliary::alias_column_list::FormatSqlAliasColumnList
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlAliasColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAliasColumnList>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAliasColumnList,
        crate::sql::auxiliary::alias_column_list::FormatSqlAliasColumnList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alias_column_list::FormatSqlAliasColumnList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAliasColumnList,
        crate::sql::auxiliary::alias_column_list::FormatSqlAliasColumnList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alias_column_list::FormatSqlAliasColumnList::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAnyAllExpression>
    for crate::sql::expressions::any_all_expression::FormatSqlAnyAllExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAnyAllExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAnyAllExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAnyAllExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAnyAllExpression,
        crate::sql::expressions::any_all_expression::FormatSqlAnyAllExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::any_all_expression::FormatSqlAnyAllExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAnyAllExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAnyAllExpression,
        crate::sql::expressions::any_all_expression::FormatSqlAnyAllExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::any_all_expression::FormatSqlAnyAllExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBetweenExpression>
    for crate::sql::expressions::between_expression::FormatSqlBetweenExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlBetweenExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlBetweenExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBetweenExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBetweenExpression,
        crate::sql::expressions::between_expression::FormatSqlBetweenExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::between_expression::FormatSqlBetweenExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBetweenExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBetweenExpression,
        crate::sql::expressions::between_expression::FormatSqlBetweenExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::between_expression::FormatSqlBetweenExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBinaryExpression>
    for crate::sql::expressions::binary_expression::FormatSqlBinaryExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlBinaryExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBinaryExpression,
        crate::sql::expressions::binary_expression::FormatSqlBinaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::binary_expression::FormatSqlBinaryExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBinaryExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBinaryExpression,
        crate::sql::expressions::binary_expression::FormatSqlBinaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::binary_expression::FormatSqlBinaryExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBooleanLiteralExpression>
    for crate::sql::expressions::boolean_literal_expression::FormatSqlBooleanLiteralExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlBooleanLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlBooleanLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBooleanLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBooleanLiteralExpression,
        crate::sql::expressions::boolean_literal_expression::FormatSqlBooleanLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: boolean_literal_expression :: FormatSqlBooleanLiteralExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBooleanLiteralExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBooleanLiteralExpression,
        crate::sql::expressions::boolean_literal_expression::FormatSqlBooleanLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: boolean_literal_expression :: FormatSqlBooleanLiteralExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlCallExpression>
    for crate::sql::expressions::call_expression::FormatSqlCallExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCallExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCallExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCallExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCallExpression,
        crate::sql::expressions::call_expression::FormatSqlCallExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::call_expression::FormatSqlCallExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCallExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCallExpression,
        crate::sql::expressions::call_expression::FormatSqlCallExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::call_expression::FormatSqlCallExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCaseElseClause>
    for crate::sql::clauses::case_else_clause::FormatSqlCaseElseClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCaseElseClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCaseElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCaseElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCaseElseClause,
        crate::sql::clauses::case_else_clause::FormatSqlCaseElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::case_else_clause::FormatSqlCaseElseClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCaseElseClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCaseElseClause,
        crate::sql::clauses::case_else_clause::FormatSqlCaseElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::case_else_clause::FormatSqlCaseElseClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCaseExpression>
    for crate::sql::expressions::case_expression::FormatSqlCaseExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCaseExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCaseExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCaseExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCaseExpression,
        crate::sql::expressions::case_expression::FormatSqlCaseExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::case_expression::FormatSqlCaseExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCaseExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCaseExpression,
        crate::sql::expressions::case_expression::FormatSqlCaseExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::case_expression::FormatSqlCaseExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCaseWhenClause>
    for crate::sql::clauses::case_when_clause::FormatSqlCaseWhenClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCaseWhenClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCaseWhenClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCaseWhenClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCaseWhenClause,
        crate::sql::clauses::case_when_clause::FormatSqlCaseWhenClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::case_when_clause::FormatSqlCaseWhenClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCaseWhenClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCaseWhenClause,
        crate::sql::clauses::case_when_clause::FormatSqlCaseWhenClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::case_when_clause::FormatSqlCaseWhenClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCastFunctionExpression>
    for crate::sql::expressions::cast_function_expression::FormatSqlCastFunctionExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCastFunctionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCastFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCastFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCastFunctionExpression,
        crate::sql::expressions::cast_function_expression::FormatSqlCastFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: cast_function_expression :: FormatSqlCastFunctionExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCastFunctionExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCastFunctionExpression,
        crate::sql::expressions::cast_function_expression::FormatSqlCastFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: cast_function_expression :: FormatSqlCastFunctionExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlColReference>
    for crate::sql::auxiliary::col_reference::FormatSqlColReference
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlColReference, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlColReference>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColReference,
        crate::sql::auxiliary::col_reference::FormatSqlColReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::col_reference::FormatSqlColReference::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColReference {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColReference,
        crate::sql::auxiliary::col_reference::FormatSqlColReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::col_reference::FormatSqlColReference::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlColumnDefinition>
    for crate::sql::auxiliary::column_definition::FormatSqlColumnDefinition
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlColumnDefinition,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlColumnDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColumnDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColumnDefinition,
        crate::sql::auxiliary::column_definition::FormatSqlColumnDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::column_definition::FormatSqlColumnDefinition::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColumnDefinition {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColumnDefinition,
        crate::sql::auxiliary::column_definition::FormatSqlColumnDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::column_definition::FormatSqlColumnDefinition::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlColumnList>
    for crate::sql::auxiliary::column_list::FormatSqlColumnList
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlColumnList, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlColumnList>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColumnList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColumnList,
        crate::sql::auxiliary::column_list::FormatSqlColumnList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::column_list::FormatSqlColumnList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColumnList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColumnList,
        crate::sql::auxiliary::column_list::FormatSqlColumnList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::column_list::FormatSqlColumnList::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCreateTableStatement>
    for crate::sql::statements::create_table_statement::FormatSqlCreateTableStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateTableStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateTableStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateTableStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateTableStatement,
        crate::sql::statements::create_table_statement::FormatSqlCreateTableStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::create_table_statement::FormatSqlCreateTableStatement::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateTableStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateTableStatement,
        crate::sql::statements::create_table_statement::FormatSqlCreateTableStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::create_table_statement::FormatSqlCreateTableStatement::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlCreateViewStatement>
    for crate::sql::statements::create_view_statement::FormatSqlCreateViewStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateViewStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateViewStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateViewStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateViewStatement,
        crate::sql::statements::create_view_statement::FormatSqlCreateViewStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::create_view_statement::FormatSqlCreateViewStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateViewStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateViewStatement,
        crate::sql::statements::create_view_statement::FormatSqlCreateViewStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::create_view_statement::FormatSqlCreateViewStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCteDefinition>
    for crate::sql::auxiliary::cte_definition::FormatSqlCteDefinition
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCteDefinition, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCteDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCteDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCteDefinition,
        crate::sql::auxiliary::cte_definition::FormatSqlCteDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::cte_definition::FormatSqlCteDefinition::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCteDefinition {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCteDefinition,
        crate::sql::auxiliary::cte_definition::FormatSqlCteDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::cte_definition::FormatSqlCteDefinition::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDataBaseName>
    for crate::sql::auxiliary::data_base_name::FormatSqlDataBaseName
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlDataBaseName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDataBaseName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDataBaseName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDataBaseName,
        crate::sql::auxiliary::data_base_name::FormatSqlDataBaseName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::data_base_name::FormatSqlDataBaseName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDataBaseName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDataBaseName,
        crate::sql::auxiliary::data_base_name::FormatSqlDataBaseName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::data_base_name::FormatSqlDataBaseName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDeleteStatement>
    for crate::sql::statements::delete_statement::FormatSqlDeleteStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlDeleteStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDeleteStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDeleteStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDeleteStatement,
        crate::sql::statements::delete_statement::FormatSqlDeleteStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::delete_statement::FormatSqlDeleteStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDeleteStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDeleteStatement,
        crate::sql::statements::delete_statement::FormatSqlDeleteStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::delete_statement::FormatSqlDeleteStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropFunctionStatement>
    for crate::sql::statements::drop_function_statement::FormatSqlDropFunctionStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropFunctionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropFunctionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropFunctionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropFunctionStatement,
        crate::sql::statements::drop_function_statement::FormatSqlDropFunctionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: drop_function_statement :: FormatSqlDropFunctionStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropFunctionStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropFunctionStatement,
        crate::sql::statements::drop_function_statement::FormatSqlDropFunctionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: drop_function_statement :: FormatSqlDropFunctionStatement :: default ())
    }
}
impl FormatRule<sql_syntax::SqlDropTableStatement>
    for crate::sql::statements::drop_table_statement::FormatSqlDropTableStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropTableStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropTableStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropTableStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropTableStatement,
        crate::sql::statements::drop_table_statement::FormatSqlDropTableStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::drop_table_statement::FormatSqlDropTableStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropTableStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropTableStatement,
        crate::sql::statements::drop_table_statement::FormatSqlDropTableStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::drop_table_statement::FormatSqlDropTableStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropViewStatement>
    for crate::sql::statements::drop_view_statement::FormatSqlDropViewStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropViewStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropViewStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropViewStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropViewStatement,
        crate::sql::statements::drop_view_statement::FormatSqlDropViewStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::drop_view_statement::FormatSqlDropViewStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropViewStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropViewStatement,
        crate::sql::statements::drop_view_statement::FormatSqlDropViewStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::drop_view_statement::FormatSqlDropViewStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlEmptyStatement>
    for crate::sql::statements::empty_statement::FormatSqlEmptyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlEmptyStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlEmptyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlEmptyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlEmptyStatement,
        crate::sql::statements::empty_statement::FormatSqlEmptyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::empty_statement::FormatSqlEmptyStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlEmptyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlEmptyStatement,
        crate::sql::statements::empty_statement::FormatSqlEmptyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::empty_statement::FormatSqlEmptyStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlExistsExpression>
    for crate::sql::expressions::exists_expression::FormatSqlExistsExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlExistsExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlExistsExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlExistsExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlExistsExpression,
        crate::sql::expressions::exists_expression::FormatSqlExistsExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::exists_expression::FormatSqlExistsExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlExistsExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlExistsExpression,
        crate::sql::expressions::exists_expression::FormatSqlExistsExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::exists_expression::FormatSqlExistsExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFetchClause>
    for crate::sql::clauses::fetch_clause::FormatSqlFetchClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFetchClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFetchClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFetchClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFetchClause,
        crate::sql::clauses::fetch_clause::FormatSqlFetchClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::fetch_clause::FormatSqlFetchClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFetchClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFetchClause,
        crate::sql::clauses::fetch_clause::FormatSqlFetchClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::fetch_clause::FormatSqlFetchClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFetchOnlyTail>
    for crate::sql::auxiliary::fetch_only_tail::FormatSqlFetchOnlyTail
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFetchOnlyTail, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFetchOnlyTail>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFetchOnlyTail {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFetchOnlyTail,
        crate::sql::auxiliary::fetch_only_tail::FormatSqlFetchOnlyTail,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::fetch_only_tail::FormatSqlFetchOnlyTail::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFetchOnlyTail {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFetchOnlyTail,
        crate::sql::auxiliary::fetch_only_tail::FormatSqlFetchOnlyTail,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::fetch_only_tail::FormatSqlFetchOnlyTail::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFetchWithTiesTail>
    for crate::sql::auxiliary::fetch_with_ties_tail::FormatSqlFetchWithTiesTail
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlFetchWithTiesTail,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFetchWithTiesTail>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFetchWithTiesTail {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFetchWithTiesTail,
        crate::sql::auxiliary::fetch_with_ties_tail::FormatSqlFetchWithTiesTail,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::fetch_with_ties_tail::FormatSqlFetchWithTiesTail::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFetchWithTiesTail {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFetchWithTiesTail,
        crate::sql::auxiliary::fetch_with_ties_tail::FormatSqlFetchWithTiesTail,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::fetch_with_ties_tail::FormatSqlFetchWithTiesTail::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFromClause>
    for crate::sql::clauses::from_clause::FormatSqlFromClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFromClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFromClause,
        crate::sql::clauses::from_clause::FormatSqlFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::from_clause::FormatSqlFromClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFromClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFromClause,
        crate::sql::clauses::from_clause::FormatSqlFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::from_clause::FormatSqlFromClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFromItem> for crate::sql::auxiliary::from_item::FormatSqlFromItem {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFromItem, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFromItem>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFromItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFromItem,
        crate::sql::auxiliary::from_item::FormatSqlFromItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::from_item::FormatSqlFromItem::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFromItem {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFromItem,
        crate::sql::auxiliary::from_item::FormatSqlFromItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::from_item::FormatSqlFromItem::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlFunctionBinding>
    for crate::sql::bindings::function_binding::FormatSqlFunctionBinding
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFunctionBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFunctionBinding>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFunctionBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFunctionBinding,
        crate::sql::bindings::function_binding::FormatSqlFunctionBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bindings::function_binding::FormatSqlFunctionBinding::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFunctionBinding {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFunctionBinding,
        crate::sql::bindings::function_binding::FormatSqlFunctionBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bindings::function_binding::FormatSqlFunctionBinding::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlGrantStatement>
    for crate::sql::statements::grant_statement::FormatSqlGrantStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlGrantStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlGrantStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGrantStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGrantStatement,
        crate::sql::statements::grant_statement::FormatSqlGrantStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::grant_statement::FormatSqlGrantStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGrantStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGrantStatement,
        crate::sql::statements::grant_statement::FormatSqlGrantStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::grant_statement::FormatSqlGrantStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlGroupByClause>
    for crate::sql::clauses::group_by_clause::FormatSqlGroupByClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlGroupByClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlGroupByClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGroupByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGroupByClause,
        crate::sql::clauses::group_by_clause::FormatSqlGroupByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::group_by_clause::FormatSqlGroupByClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGroupByClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGroupByClause,
        crate::sql::clauses::group_by_clause::FormatSqlGroupByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::group_by_clause::FormatSqlGroupByClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlHavingClause>
    for crate::sql::clauses::having_clause::FormatSqlHavingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlHavingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlHavingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlHavingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlHavingClause,
        crate::sql::clauses::having_clause::FormatSqlHavingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::having_clause::FormatSqlHavingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlHavingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlHavingClause,
        crate::sql::clauses::having_clause::FormatSqlHavingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::having_clause::FormatSqlHavingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlInExpression>
    for crate::sql::expressions::in_expression::FormatSqlInExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlInExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlInExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlInExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlInExpression,
        crate::sql::expressions::in_expression::FormatSqlInExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::in_expression::FormatSqlInExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlInExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlInExpression,
        crate::sql::expressions::in_expression::FormatSqlInExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::in_expression::FormatSqlInExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlInValueList>
    for crate::sql::auxiliary::in_value_list::FormatSqlInValueList
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlInValueList, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlInValueList>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlInValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlInValueList,
        crate::sql::auxiliary::in_value_list::FormatSqlInValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::in_value_list::FormatSqlInValueList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlInValueList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlInValueList,
        crate::sql::auxiliary::in_value_list::FormatSqlInValueList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::in_value_list::FormatSqlInValueList::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlInsertStatement>
    for crate::sql::statements::insert_statement::FormatSqlInsertStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlInsertStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlInsertStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlInsertStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlInsertStatement,
        crate::sql::statements::insert_statement::FormatSqlInsertStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::insert_statement::FormatSqlInsertStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlInsertStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlInsertStatement,
        crate::sql::statements::insert_statement::FormatSqlInsertStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::insert_statement::FormatSqlInsertStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlIsNullExpression>
    for crate::sql::expressions::is_null_expression::FormatSqlIsNullExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlIsNullExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlIsNullExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIsNullExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIsNullExpression,
        crate::sql::expressions::is_null_expression::FormatSqlIsNullExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::is_null_expression::FormatSqlIsNullExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIsNullExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIsNullExpression,
        crate::sql::expressions::is_null_expression::FormatSqlIsNullExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::is_null_expression::FormatSqlIsNullExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlJoinClause>
    for crate::sql::clauses::join_clause::FormatSqlJoinClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlJoinClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlJoinClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlJoinClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlJoinClause,
        crate::sql::clauses::join_clause::FormatSqlJoinClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::join_clause::FormatSqlJoinClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlJoinClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlJoinClause,
        crate::sql::clauses::join_clause::FormatSqlJoinClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::join_clause::FormatSqlJoinClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlLikeExpression>
    for crate::sql::expressions::like_expression::FormatSqlLikeExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlLikeExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlLikeExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlLikeExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlLikeExpression,
        crate::sql::expressions::like_expression::FormatSqlLikeExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::like_expression::FormatSqlLikeExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlLikeExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlLikeExpression,
        crate::sql::expressions::like_expression::FormatSqlLikeExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::like_expression::FormatSqlLikeExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlLogicalExpression>
    for crate::sql::expressions::logical_expression::FormatSqlLogicalExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlLogicalExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlLogicalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlLogicalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlLogicalExpression,
        crate::sql::expressions::logical_expression::FormatSqlLogicalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::logical_expression::FormatSqlLogicalExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlLogicalExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlLogicalExpression,
        crate::sql::expressions::logical_expression::FormatSqlLogicalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::logical_expression::FormatSqlLogicalExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlName> for crate::sql::auxiliary::name::FormatSqlName {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlName {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::SqlName, crate::sql::auxiliary::name::FormatSqlName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::sql::auxiliary::name::FormatSqlName::default())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlName {
    type Format =
        FormatOwnedWithRule<sql_syntax::SqlName, crate::sql::auxiliary::name::FormatSqlName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::sql::auxiliary::name::FormatSqlName::default())
    }
}
impl FormatRule<sql_syntax::SqlNullLiteralExpression>
    for crate::sql::expressions::null_literal_expression::FormatSqlNullLiteralExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlNullLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlNullLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlNullLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlNullLiteralExpression,
        crate::sql::expressions::null_literal_expression::FormatSqlNullLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: null_literal_expression :: FormatSqlNullLiteralExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlNullLiteralExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlNullLiteralExpression,
        crate::sql::expressions::null_literal_expression::FormatSqlNullLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: null_literal_expression :: FormatSqlNullLiteralExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlNumberLiteralExpression>
    for crate::sql::expressions::number_literal_expression::FormatSqlNumberLiteralExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlNumberLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlNumberLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlNumberLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlNumberLiteralExpression,
        crate::sql::expressions::number_literal_expression::FormatSqlNumberLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: number_literal_expression :: FormatSqlNumberLiteralExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlNumberLiteralExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlNumberLiteralExpression,
        crate::sql::expressions::number_literal_expression::FormatSqlNumberLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: number_literal_expression :: FormatSqlNumberLiteralExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlOffsetClause>
    for crate::sql::clauses::offset_clause::FormatSqlOffsetClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlOffsetClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlOffsetClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOffsetClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOffsetClause,
        crate::sql::clauses::offset_clause::FormatSqlOffsetClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::offset_clause::FormatSqlOffsetClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOffsetClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOffsetClause,
        crate::sql::clauses::offset_clause::FormatSqlOffsetClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::offset_clause::FormatSqlOffsetClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlOrderByClause>
    for crate::sql::clauses::order_by_clause::FormatSqlOrderByClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlOrderByClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlOrderByClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOrderByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOrderByClause,
        crate::sql::clauses::order_by_clause::FormatSqlOrderByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::order_by_clause::FormatSqlOrderByClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOrderByClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOrderByClause,
        crate::sql::clauses::order_by_clause::FormatSqlOrderByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::order_by_clause::FormatSqlOrderByClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlOrderByExpression>
    for crate::sql::expressions::order_by_expression::FormatSqlOrderByExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlOrderByExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlOrderByExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOrderByExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOrderByExpression,
        crate::sql::expressions::order_by_expression::FormatSqlOrderByExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::order_by_expression::FormatSqlOrderByExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOrderByExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOrderByExpression,
        crate::sql::expressions::order_by_expression::FormatSqlOrderByExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::order_by_expression::FormatSqlOrderByExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlParameterExpression>
    for crate::sql::expressions::parameter_expression::FormatSqlParameterExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlParameterExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlParameterExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlParameterExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlParameterExpression,
        crate::sql::expressions::parameter_expression::FormatSqlParameterExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::parameter_expression::FormatSqlParameterExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlParameterExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlParameterExpression,
        crate::sql::expressions::parameter_expression::FormatSqlParameterExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::parameter_expression::FormatSqlParameterExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlParenthesizedExpression>
    for crate::sql::expressions::parenthesized_expression::FormatSqlParenthesizedExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlParenthesizedExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlParenthesizedExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlParenthesizedExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlParenthesizedExpression,
        crate::sql::expressions::parenthesized_expression::FormatSqlParenthesizedExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: parenthesized_expression :: FormatSqlParenthesizedExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlParenthesizedExpression,
        crate::sql::expressions::parenthesized_expression::FormatSqlParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: parenthesized_expression :: FormatSqlParenthesizedExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlParenthesizedJoinBinding>
    for crate::sql::bindings::parenthesized_join_binding::FormatSqlParenthesizedJoinBinding
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlParenthesizedJoinBinding,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlParenthesizedJoinBinding>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlParenthesizedJoinBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlParenthesizedJoinBinding,
        crate::sql::bindings::parenthesized_join_binding::FormatSqlParenthesizedJoinBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: bindings :: parenthesized_join_binding :: FormatSqlParenthesizedJoinBinding :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlParenthesizedJoinBinding {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlParenthesizedJoinBinding,
        crate::sql::bindings::parenthesized_join_binding::FormatSqlParenthesizedJoinBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: bindings :: parenthesized_join_binding :: FormatSqlParenthesizedJoinBinding :: default ())
    }
}
impl FormatRule<sql_syntax::SqlPrecisionModifier>
    for crate::sql::auxiliary::precision_modifier::FormatSqlPrecisionModifier
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPrecisionModifier,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrecisionModifier>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrecisionModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrecisionModifier,
        crate::sql::auxiliary::precision_modifier::FormatSqlPrecisionModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::precision_modifier::FormatSqlPrecisionModifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrecisionModifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrecisionModifier,
        crate::sql::auxiliary::precision_modifier::FormatSqlPrecisionModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::precision_modifier::FormatSqlPrecisionModifier::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlRoot> for crate::sql::auxiliary::root::FormatSqlRoot {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlRoot, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRoot {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::SqlRoot, crate::sql::auxiliary::root::FormatSqlRoot>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::sql::auxiliary::root::FormatSqlRoot::default())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRoot {
    type Format =
        FormatOwnedWithRule<sql_syntax::SqlRoot, crate::sql::auxiliary::root::FormatSqlRoot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::sql::auxiliary::root::FormatSqlRoot::default())
    }
}
impl FormatRule<sql_syntax::SqlSelectAllQuantifier>
    for crate::sql::auxiliary::select_all_quantifier::FormatSqlSelectAllQuantifier
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSelectAllQuantifier,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSelectAllQuantifier>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectAllQuantifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectAllQuantifier,
        crate::sql::auxiliary::select_all_quantifier::FormatSqlSelectAllQuantifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::select_all_quantifier::FormatSqlSelectAllQuantifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectAllQuantifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectAllQuantifier,
        crate::sql::auxiliary::select_all_quantifier::FormatSqlSelectAllQuantifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::select_all_quantifier::FormatSqlSelectAllQuantifier::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSelectClause>
    for crate::sql::clauses::select_clause::FormatSqlSelectClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSelectClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSelectClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectClause,
        crate::sql::clauses::select_clause::FormatSqlSelectClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::select_clause::FormatSqlSelectClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectClause,
        crate::sql::clauses::select_clause::FormatSqlSelectClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::select_clause::FormatSqlSelectClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSelectDistinctQuantifier>
    for crate::sql::auxiliary::select_distinct_quantifier::FormatSqlSelectDistinctQuantifier
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSelectDistinctQuantifier,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSelectDistinctQuantifier>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectDistinctQuantifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectDistinctQuantifier,
        crate::sql::auxiliary::select_distinct_quantifier::FormatSqlSelectDistinctQuantifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: select_distinct_quantifier :: FormatSqlSelectDistinctQuantifier :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectDistinctQuantifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectDistinctQuantifier,
        crate::sql::auxiliary::select_distinct_quantifier::FormatSqlSelectDistinctQuantifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: select_distinct_quantifier :: FormatSqlSelectDistinctQuantifier :: default ())
    }
}
impl FormatRule<sql_syntax::SqlSelectExpression>
    for crate::sql::expressions::select_expression::FormatSqlSelectExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSelectExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSelectExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectExpression,
        crate::sql::expressions::select_expression::FormatSqlSelectExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::select_expression::FormatSqlSelectExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectExpression,
        crate::sql::expressions::select_expression::FormatSqlSelectExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::select_expression::FormatSqlSelectExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSelectStatement>
    for crate::sql::statements::select_statement::FormatSqlSelectStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSelectStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSelectStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectStatement,
        crate::sql::statements::select_statement::FormatSqlSelectStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::select_statement::FormatSqlSelectStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectStatement,
        crate::sql::statements::select_statement::FormatSqlSelectStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::select_statement::FormatSqlSelectStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSetClause> for crate::sql::clauses::set_clause::FormatSqlSetClause {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSetClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetClause,
        crate::sql::clauses::set_clause::FormatSqlSetClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::set_clause::FormatSqlSetClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetClause,
        crate::sql::clauses::set_clause::FormatSqlSetClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::set_clause::FormatSqlSetClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSetItem> for crate::sql::auxiliary::set_item::FormatSqlSetItem {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSetItem, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetItem>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetItem,
        crate::sql::auxiliary::set_item::FormatSqlSetItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::set_item::FormatSqlSetItem::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetItem {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetItem,
        crate::sql::auxiliary::set_item::FormatSqlSetItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::set_item::FormatSqlSetItem::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSetOperation>
    for crate::sql::auxiliary::set_operation::FormatSqlSetOperation
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSetOperation, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetOperation>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetOperation,
        crate::sql::auxiliary::set_operation::FormatSqlSetOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::set_operation::FormatSqlSetOperation::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetOperation {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetOperation,
        crate::sql::auxiliary::set_operation::FormatSqlSetOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::set_operation::FormatSqlSetOperation::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlShemaName>
    for crate::sql::auxiliary::shema_name::FormatSqlShemaName
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlShemaName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlShemaName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlShemaName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlShemaName,
        crate::sql::auxiliary::shema_name::FormatSqlShemaName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::shema_name::FormatSqlShemaName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlShemaName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlShemaName,
        crate::sql::auxiliary::shema_name::FormatSqlShemaName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::shema_name::FormatSqlShemaName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlStar> for crate::sql::auxiliary::star::FormatSqlStar {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlStar, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlStar>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlStar {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::SqlStar, crate::sql::auxiliary::star::FormatSqlStar>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::sql::auxiliary::star::FormatSqlStar::default())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlStar {
    type Format =
        FormatOwnedWithRule<sql_syntax::SqlStar, crate::sql::auxiliary::star::FormatSqlStar>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::sql::auxiliary::star::FormatSqlStar::default())
    }
}
impl FormatRule<sql_syntax::SqlStringLiteralExpression>
    for crate::sql::expressions::string_literal_expression::FormatSqlStringLiteralExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlStringLiteralExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlStringLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlStringLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlStringLiteralExpression,
        crate::sql::expressions::string_literal_expression::FormatSqlStringLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: string_literal_expression :: FormatSqlStringLiteralExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlStringLiteralExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlStringLiteralExpression,
        crate::sql::expressions::string_literal_expression::FormatSqlStringLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: string_literal_expression :: FormatSqlStringLiteralExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlSubqueryBinding>
    for crate::sql::bindings::subquery_binding::FormatSqlSubqueryBinding
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSubqueryBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSubqueryBinding>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSubqueryBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSubqueryBinding,
        crate::sql::bindings::subquery_binding::FormatSqlSubqueryBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bindings::subquery_binding::FormatSqlSubqueryBinding::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSubqueryBinding {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSubqueryBinding,
        crate::sql::bindings::subquery_binding::FormatSqlSubqueryBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bindings::subquery_binding::FormatSqlSubqueryBinding::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSubqueryExpression>
    for crate::sql::expressions::subquery_expression::FormatSqlSubqueryExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSubqueryExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSubqueryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSubqueryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSubqueryExpression,
        crate::sql::expressions::subquery_expression::FormatSqlSubqueryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::subquery_expression::FormatSqlSubqueryExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSubqueryExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSubqueryExpression,
        crate::sql::expressions::subquery_expression::FormatSqlSubqueryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::subquery_expression::FormatSqlSubqueryExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTableBinding>
    for crate::sql::bindings::table_binding::FormatSqlTableBinding
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTableBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTableBinding>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableBinding,
        crate::sql::bindings::table_binding::FormatSqlTableBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bindings::table_binding::FormatSqlTableBinding::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableBinding {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableBinding,
        crate::sql::bindings::table_binding::FormatSqlTableBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bindings::table_binding::FormatSqlTableBinding::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTableColReference>
    for crate::sql::auxiliary::table_col_reference::FormatSqlTableColReference
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTableColReference,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTableColReference>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableColReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableColReference,
        crate::sql::auxiliary::table_col_reference::FormatSqlTableColReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::table_col_reference::FormatSqlTableColReference::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableColReference {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableColReference,
        crate::sql::auxiliary::table_col_reference::FormatSqlTableColReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::table_col_reference::FormatSqlTableColReference::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTableName>
    for crate::sql::auxiliary::table_name::FormatSqlTableName
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTableName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTableName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableName,
        crate::sql::auxiliary::table_name::FormatSqlTableName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::table_name::FormatSqlTableName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableName,
        crate::sql::auxiliary::table_name::FormatSqlTableName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::table_name::FormatSqlTableName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTableStar>
    for crate::sql::auxiliary::table_star::FormatSqlTableStar
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTableStar, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTableStar>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableStar {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableStar,
        crate::sql::auxiliary::table_star::FormatSqlTableStar,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::table_star::FormatSqlTableStar::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableStar {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableStar,
        crate::sql::auxiliary::table_star::FormatSqlTableStar,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::table_star::FormatSqlTableStar::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTildeName>
    for crate::sql::auxiliary::tilde_name::FormatSqlTildeName
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTildeName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTildeName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTildeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTildeName,
        crate::sql::auxiliary::tilde_name::FormatSqlTildeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::tilde_name::FormatSqlTildeName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTildeName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTildeName,
        crate::sql::auxiliary::tilde_name::FormatSqlTildeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::tilde_name::FormatSqlTildeName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTimeZoneModifier>
    for crate::sql::auxiliary::time_zone_modifier::FormatSqlTimeZoneModifier
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTimeZoneModifier,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTimeZoneModifier>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTimeZoneModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTimeZoneModifier,
        crate::sql::auxiliary::time_zone_modifier::FormatSqlTimeZoneModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::time_zone_modifier::FormatSqlTimeZoneModifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTimeZoneModifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTimeZoneModifier,
        crate::sql::auxiliary::time_zone_modifier::FormatSqlTimeZoneModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::time_zone_modifier::FormatSqlTimeZoneModifier::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTypeArguments>
    for crate::sql::auxiliary::type_arguments::FormatSqlTypeArguments
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTypeArguments, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTypeArguments>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTypeArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTypeArguments,
        crate::sql::auxiliary::type_arguments::FormatSqlTypeArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::type_arguments::FormatSqlTypeArguments::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTypeArguments {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTypeArguments,
        crate::sql::auxiliary::type_arguments::FormatSqlTypeArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::type_arguments::FormatSqlTypeArguments::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTypeName> for crate::sql::auxiliary::type_name::FormatSqlTypeName {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTypeName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTypeName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTypeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTypeName,
        crate::sql::auxiliary::type_name::FormatSqlTypeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::type_name::FormatSqlTypeName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTypeName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTypeName,
        crate::sql::auxiliary::type_name::FormatSqlTypeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::type_name::FormatSqlTypeName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlUnaryExpression>
    for crate::sql::expressions::unary_expression::FormatSqlUnaryExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlUnaryExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlUnaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlUnaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlUnaryExpression,
        crate::sql::expressions::unary_expression::FormatSqlUnaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::unary_expression::FormatSqlUnaryExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlUnaryExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlUnaryExpression,
        crate::sql::expressions::unary_expression::FormatSqlUnaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::unary_expression::FormatSqlUnaryExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlUpdateFromClause>
    for crate::sql::clauses::update_from_clause::FormatSqlUpdateFromClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlUpdateFromClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlUpdateFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlUpdateFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlUpdateFromClause,
        crate::sql::clauses::update_from_clause::FormatSqlUpdateFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::update_from_clause::FormatSqlUpdateFromClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlUpdateFromClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlUpdateFromClause,
        crate::sql::clauses::update_from_clause::FormatSqlUpdateFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::update_from_clause::FormatSqlUpdateFromClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlUpdateStatement>
    for crate::sql::statements::update_statement::FormatSqlUpdateStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlUpdateStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlUpdateStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlUpdateStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlUpdateStatement,
        crate::sql::statements::update_statement::FormatSqlUpdateStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::update_statement::FormatSqlUpdateStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlUpdateStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlUpdateStatement,
        crate::sql::statements::update_statement::FormatSqlUpdateStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::update_statement::FormatSqlUpdateStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlValuesClause>
    for crate::sql::clauses::values_clause::FormatSqlValuesClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlValuesClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlValuesClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlValuesClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlValuesClause,
        crate::sql::clauses::values_clause::FormatSqlValuesClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::values_clause::FormatSqlValuesClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlValuesClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlValuesClause,
        crate::sql::clauses::values_clause::FormatSqlValuesClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::values_clause::FormatSqlValuesClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlValuesRow>
    for crate::sql::auxiliary::values_row::FormatSqlValuesRow
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlValuesRow, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlValuesRow>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlValuesRow {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlValuesRow,
        crate::sql::auxiliary::values_row::FormatSqlValuesRow,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::values_row::FormatSqlValuesRow::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlValuesRow {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlValuesRow,
        crate::sql::auxiliary::values_row::FormatSqlValuesRow,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::values_row::FormatSqlValuesRow::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlVaryingModifier>
    for crate::sql::auxiliary::varying_modifier::FormatSqlVaryingModifier
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlVaryingModifier, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlVaryingModifier>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlVaryingModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlVaryingModifier,
        crate::sql::auxiliary::varying_modifier::FormatSqlVaryingModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::varying_modifier::FormatSqlVaryingModifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlVaryingModifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlVaryingModifier,
        crate::sql::auxiliary::varying_modifier::FormatSqlVaryingModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::varying_modifier::FormatSqlVaryingModifier::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlWhereClause>
    for crate::sql::clauses::where_clause::FormatSqlWhereClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlWhereClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWhereClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWhereClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWhereClause,
        crate::sql::clauses::where_clause::FormatSqlWhereClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::where_clause::FormatSqlWhereClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWhereClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWhereClause,
        crate::sql::clauses::where_clause::FormatSqlWhereClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::where_clause::FormatSqlWhereClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlWindowFunctionExpression>
    for crate::sql::expressions::window_function_expression::FormatSqlWindowFunctionExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowFunctionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowFunctionExpression,
        crate::sql::expressions::window_function_expression::FormatSqlWindowFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: window_function_expression :: FormatSqlWindowFunctionExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowFunctionExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowFunctionExpression,
        crate::sql::expressions::window_function_expression::FormatSqlWindowFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: window_function_expression :: FormatSqlWindowFunctionExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlWindowPartitionByClause>
    for crate::sql::clauses::window_partition_by_clause::FormatSqlWindowPartitionByClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowPartitionByClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowPartitionByClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowPartitionByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowPartitionByClause,
        crate::sql::clauses::window_partition_by_clause::FormatSqlWindowPartitionByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: clauses :: window_partition_by_clause :: FormatSqlWindowPartitionByClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowPartitionByClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowPartitionByClause,
        crate::sql::clauses::window_partition_by_clause::FormatSqlWindowPartitionByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: clauses :: window_partition_by_clause :: FormatSqlWindowPartitionByClause :: default ())
    }
}
impl FormatRule<sql_syntax::SqlWindowSpecification>
    for crate::sql::auxiliary::window_specification::FormatSqlWindowSpecification
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowSpecification,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowSpecification>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowSpecification {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowSpecification,
        crate::sql::auxiliary::window_specification::FormatSqlWindowSpecification,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::window_specification::FormatSqlWindowSpecification::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowSpecification {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowSpecification,
        crate::sql::auxiliary::window_specification::FormatSqlWindowSpecification,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::window_specification::FormatSqlWindowSpecification::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlWithClause>
    for crate::sql::clauses::with_clause::FormatSqlWithClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlWithClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWithClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWithClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWithClause,
        crate::sql::clauses::with_clause::FormatSqlWithClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::with_clause::FormatSqlWithClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWithClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWithClause,
        crate::sql::clauses::with_clause::FormatSqlWithClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::with_clause::FormatSqlWithClause::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlFunctionOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlFunctionOptionList,
        crate::psql::lists::function_option_list::FormatPsqlFunctionOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::function_option_list::FormatPsqlFunctionOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlFunctionOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlFunctionOptionList,
        crate::psql::lists::function_option_list::FormatPsqlFunctionOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::function_option_list::FormatPsqlFunctionOptionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlFunctionParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlFunctionParameterList,
        crate::psql::lists::function_parameter_list::FormatPsqlFunctionParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::function_parameter_list::FormatPsqlFunctionParameterList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlFunctionParameterList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlFunctionParameterList,
        crate::psql::lists::function_parameter_list::FormatPsqlFunctionParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::function_parameter_list::FormatPsqlFunctionParameterList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableColumnList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlReturnsTableColumnList,
        crate::psql::lists::returns_table_column_list::FormatPsqlReturnsTableColumnList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: lists :: returns_table_column_list :: FormatPsqlReturnsTableColumnList :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlReturnsTableColumnList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlReturnsTableColumnList,
        crate::psql::lists::returns_table_column_list::FormatPsqlReturnsTableColumnList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: lists :: returns_table_column_list :: FormatPsqlReturnsTableColumnList :: default ())
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerEventList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerEventList,
        crate::psql::lists::trigger_event_list::FormatPsqlTriggerEventList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::trigger_event_list::FormatPsqlTriggerEventList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerEventList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerEventList,
        crate::psql::lists::trigger_event_list::FormatPsqlTriggerEventList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::trigger_event_list::FormatPsqlTriggerEventList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTriggerReferencingItemList,
        crate::psql::lists::trigger_referencing_item_list::FormatPsqlTriggerReferencingItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: lists :: trigger_referencing_item_list :: FormatPsqlTriggerReferencingItemList :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTriggerReferencingItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTriggerReferencingItemList,
        crate::psql::lists::trigger_referencing_item_list::FormatPsqlTriggerReferencingItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: lists :: trigger_referencing_item_list :: FormatPsqlTriggerReferencingItemList :: default ())
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTypeNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTypeNameList,
        crate::psql::lists::type_name_list::FormatPsqlTypeNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::type_name_list::FormatPsqlTypeNameList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTypeNameList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTypeNameList,
        crate::psql::lists::type_name_list::FormatPsqlTypeNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::type_name_list::FormatPsqlTypeNameList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlViewOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlViewOptionList,
        crate::psql::lists::view_option_list::FormatPsqlViewOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::view_option_list::FormatPsqlViewOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlViewOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlViewOptionList,
        crate::psql::lists::view_option_list::FormatPsqlViewOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::view_option_list::FormatPsqlViewOptionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAliasColumnDefinitionList,
        crate::sql::lists::alias_column_definition_list::FormatSqlAliasColumnDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: lists :: alias_column_definition_list :: FormatSqlAliasColumnDefinitionList :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAliasColumnDefinitionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAliasColumnDefinitionList,
        crate::sql::lists::alias_column_definition_list::FormatSqlAliasColumnDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: lists :: alias_column_definition_list :: FormatSqlAliasColumnDefinitionList :: default ())
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCaseWhenClauseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCaseWhenClauseList,
        crate::sql::lists::case_when_clause_list::FormatSqlCaseWhenClauseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::case_when_clause_list::FormatSqlCaseWhenClauseList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCaseWhenClauseList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCaseWhenClauseList,
        crate::sql::lists::case_when_clause_list::FormatSqlCaseWhenClauseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::case_when_clause_list::FormatSqlCaseWhenClauseList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColumnDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColumnDefinitionList,
        crate::sql::lists::column_definition_list::FormatSqlColumnDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::column_definition_list::FormatSqlColumnDefinitionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColumnDefinitionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColumnDefinitionList,
        crate::sql::lists::column_definition_list::FormatSqlColumnDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::column_definition_list::FormatSqlColumnDefinitionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColumnNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColumnNameList,
        crate::sql::lists::column_name_list::FormatSqlColumnNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::column_name_list::FormatSqlColumnNameList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColumnNameList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColumnNameList,
        crate::sql::lists::column_name_list::FormatSqlColumnNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::column_name_list::FormatSqlColumnNameList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCteDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCteDefinitionList,
        crate::sql::lists::cte_definition_list::FormatSqlCteDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::cte_definition_list::FormatSqlCteDefinitionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCteDefinitionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCteDefinitionList,
        crate::sql::lists::cte_definition_list::FormatSqlCteDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::cte_definition_list::FormatSqlCteDefinitionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlExpressionList,
        crate::sql::lists::expression_list::FormatSqlExpressionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::expression_list::FormatSqlExpressionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlExpressionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlExpressionList,
        crate::sql::lists::expression_list::FormatSqlExpressionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::expression_list::FormatSqlExpressionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFromItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFromItemList,
        crate::sql::lists::from_item_list::FormatSqlFromItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::from_item_list::FormatSqlFromItemList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFromItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFromItemList,
        crate::sql::lists::from_item_list::FormatSqlFromItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::from_item_list::FormatSqlFromItemList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGranteeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGranteeList,
        crate::sql::lists::grantee_list::FormatSqlGranteeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::grantee_list::FormatSqlGranteeList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGranteeList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGranteeList,
        crate::sql::lists::grantee_list::FormatSqlGranteeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::grantee_list::FormatSqlGranteeList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGroupByItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGroupByItemList,
        crate::sql::lists::group_by_item_list::FormatSqlGroupByItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::group_by_item_list::FormatSqlGroupByItemList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGroupByItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGroupByItemList,
        crate::sql::lists::group_by_item_list::FormatSqlGroupByItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::group_by_item_list::FormatSqlGroupByItemList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlJoinClauseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlJoinClauseList,
        crate::sql::lists::join_clause_list::FormatSqlJoinClauseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::join_clause_list::FormatSqlJoinClauseList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlJoinClauseList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlJoinClauseList,
        crate::sql::lists::join_clause_list::FormatSqlJoinClauseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::join_clause_list::FormatSqlJoinClauseList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOrderByExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOrderByExpressionList,
        crate::sql::lists::order_by_expression_list::FormatSqlOrderByExpressionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::order_by_expression_list::FormatSqlOrderByExpressionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOrderByExpressionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOrderByExpressionList,
        crate::sql::lists::order_by_expression_list::FormatSqlOrderByExpressionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::order_by_expression_list::FormatSqlOrderByExpressionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSelectItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSelectItemList,
        crate::sql::lists::select_item_list::FormatSqlSelectItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::select_item_list::FormatSqlSelectItemList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSelectItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSelectItemList,
        crate::sql::lists::select_item_list::FormatSqlSelectItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::select_item_list::FormatSqlSelectItemList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetItemList,
        crate::sql::lists::set_item_list::FormatSqlSetItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::set_item_list::FormatSqlSetItemList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetItemList,
        crate::sql::lists::set_item_list::FormatSqlSetItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::set_item_list::FormatSqlSetItemList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetOperationList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetOperationList,
        crate::sql::lists::set_operation_list::FormatSqlSetOperationList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::set_operation_list::FormatSqlSetOperationList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetOperationList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetOperationList,
        crate::sql::lists::set_operation_list::FormatSqlSetOperationList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::set_operation_list::FormatSqlSetOperationList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlStatementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlStatementList,
        crate::sql::lists::statement_list::FormatSqlStatementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::statement_list::FormatSqlStatementList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlStatementList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlStatementList,
        crate::sql::lists::statement_list::FormatSqlStatementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::statement_list::FormatSqlStatementList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableNameList,
        crate::sql::lists::table_name_list::FormatSqlTableNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::table_name_list::FormatSqlTableNameList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableNameList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableNameList,
        crate::sql::lists::table_name_list::FormatSqlTableNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::table_name_list::FormatSqlTableNameList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTypeArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTypeArgumentList,
        crate::sql::lists::type_argument_list::FormatSqlTypeArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::type_argument_list::FormatSqlTypeArgumentList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTypeArgumentList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTypeArgumentList,
        crate::sql::lists::type_argument_list::FormatSqlTypeArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::type_argument_list::FormatSqlTypeArgumentList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlValuesRowList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlValuesRowList,
        crate::sql::lists::values_row_list::FormatSqlValuesRowList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::values_row_list::FormatSqlValuesRowList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlValuesRowList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlValuesRowList,
        crate::sql::lists::values_row_list::FormatSqlValuesRowList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::values_row_list::FormatSqlValuesRowList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowPartitionByItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowPartitionByItemList,
        crate::sql::lists::window_partition_by_item_list::FormatSqlWindowPartitionByItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: lists :: window_partition_by_item_list :: FormatSqlWindowPartitionByItemList :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowPartitionByItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowPartitionByItemList,
        crate::sql::lists::window_partition_by_item_list::FormatSqlWindowPartitionByItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: lists :: window_partition_by_item_list :: FormatSqlWindowPartitionByItemList :: default ())
    }
}
impl FormatRule<sql_syntax::SqlBogus> for crate::sql::bogus::bogus::FormatSqlBogus {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogus, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogus {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::SqlBogus, crate::sql::bogus::bogus::FormatSqlBogus>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::sql::bogus::bogus::FormatSqlBogus::default())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogus {
    type Format =
        FormatOwnedWithRule<sql_syntax::SqlBogus, crate::sql::bogus::bogus::FormatSqlBogus>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::sql::bogus::bogus::FormatSqlBogus::default())
    }
}
impl FormatRule<sql_syntax::SqlBogusAssignment>
    for crate::sql::bogus::bogus_assignment::FormatSqlBogusAssignment
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusAssignment, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusAssignment,
        crate::sql::bogus::bogus_assignment::FormatSqlBogusAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_assignment::FormatSqlBogusAssignment::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusAssignment {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusAssignment,
        crate::sql::bogus::bogus_assignment::FormatSqlBogusAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_assignment::FormatSqlBogusAssignment::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBogusBinding>
    for crate::sql::bogus::bogus_binding::FormatSqlBogusBinding
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusBinding>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusBinding,
        crate::sql::bogus::bogus_binding::FormatSqlBogusBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_binding::FormatSqlBogusBinding::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusBinding {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusBinding,
        crate::sql::bogus::bogus_binding::FormatSqlBogusBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_binding::FormatSqlBogusBinding::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBogusExpression>
    for crate::sql::bogus::bogus_expression::FormatSqlBogusExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusExpression,
        crate::sql::bogus::bogus_expression::FormatSqlBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_expression::FormatSqlBogusExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusExpression,
        crate::sql::bogus::bogus_expression::FormatSqlBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_expression::FormatSqlBogusExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBogusMember>
    for crate::sql::bogus::bogus_member::FormatSqlBogusMember
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusMember, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusMember,
        crate::sql::bogus::bogus_member::FormatSqlBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_member::FormatSqlBogusMember::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusMember {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusMember,
        crate::sql::bogus::bogus_member::FormatSqlBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_member::FormatSqlBogusMember::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBogusParameter>
    for crate::sql::bogus::bogus_parameter::FormatSqlBogusParameter
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusParameter, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusParameter,
        crate::sql::bogus::bogus_parameter::FormatSqlBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_parameter::FormatSqlBogusParameter::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusParameter {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusParameter,
        crate::sql::bogus::bogus_parameter::FormatSqlBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_parameter::FormatSqlBogusParameter::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlBogusStatement>
    for crate::sql::bogus::bogus_statement::FormatSqlBogusStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBogusStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<sql_syntax::SqlBogusStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBogusStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBogusStatement,
        crate::sql::bogus::bogus_statement::FormatSqlBogusStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::bogus::bogus_statement::FormatSqlBogusStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBogusStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBogusStatement,
        crate::sql::bogus::bogus_statement::FormatSqlBogusStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::bogus::bogus_statement::FormatSqlBogusStatement::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlAnyAllSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlAnyAllSource,
        crate::sql::any::any_all_source::FormatAnySqlAnyAllSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::any_all_source::FormatAnySqlAnyAllSource::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlAnyAllSource {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlAnyAllSource,
        crate::sql::any::any_all_source::FormatAnySqlAnyAllSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::any_all_source::FormatAnySqlAnyAllSource::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlConflictAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlConflictAction,
        crate::sql::any::conflict_action::FormatAnySqlConflictAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::conflict_action::FormatAnySqlConflictAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlConflictAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlConflictAction,
        crate::sql::any::conflict_action::FormatAnySqlConflictAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::conflict_action::FormatAnySqlConflictAction::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlConflictTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlConflictTarget,
        crate::sql::any::conflict_target::FormatAnySqlConflictTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::conflict_target::FormatAnySqlConflictTarget::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlConflictTarget {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlConflictTarget,
        crate::sql::any::conflict_target::FormatAnySqlConflictTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::conflict_target::FormatAnySqlConflictTarget::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlExpression,
        crate::sql::any::expression::FormatAnySqlExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::expression::FormatAnySqlExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlExpression,
        crate::sql::any::expression::FormatAnySqlExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::expression::FormatAnySqlExpression::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlFetchTail {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlFetchTail,
        crate::sql::any::fetch_tail::FormatAnySqlFetchTail,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::fetch_tail::FormatAnySqlFetchTail::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlFetchTail {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlFetchTail,
        crate::sql::any::fetch_tail::FormatAnySqlFetchTail,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::fetch_tail::FormatAnySqlFetchTail::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlFromExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlFromExpression,
        crate::sql::any::from_expression::FormatAnySqlFromExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::from_expression::FormatAnySqlFromExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlFromExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlFromExpression,
        crate::sql::any::from_expression::FormatAnySqlFromExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::from_expression::FormatAnySqlFromExpression::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlFunctionOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlFunctionOption,
        crate::sql::any::function_option::FormatAnySqlFunctionOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::function_option::FormatAnySqlFunctionOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlFunctionOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlFunctionOption,
        crate::sql::any::function_option::FormatAnySqlFunctionOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::function_option::FormatAnySqlFunctionOption::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlInSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlInSource,
        crate::sql::any::in_source::FormatAnySqlInSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::in_source::FormatAnySqlInSource::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlInSource {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlInSource,
        crate::sql::any::in_source::FormatAnySqlInSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::in_source::FormatAnySqlInSource::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlInsertSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlInsertSource,
        crate::sql::any::insert_source::FormatAnySqlInsertSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::insert_source::FormatAnySqlInsertSource::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlInsertSource {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlInsertSource,
        crate::sql::any::insert_source::FormatAnySqlInsertSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::insert_source::FormatAnySqlInsertSource::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlLimitValue {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlLimitValue,
        crate::sql::any::limit_value::FormatAnySqlLimitValue,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::limit_value::FormatAnySqlLimitValue::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlLimitValue {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlLimitValue,
        crate::sql::any::limit_value::FormatAnySqlLimitValue,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::limit_value::FormatAnySqlLimitValue::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlLiteralExpression,
        crate::sql::any::literal_expression::FormatAnySqlLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::literal_expression::FormatAnySqlLiteralExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlLiteralExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlLiteralExpression,
        crate::sql::any::literal_expression::FormatAnySqlLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::literal_expression::FormatAnySqlLiteralExpression::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlName {
    type Format<'a> =
        FormatRefWithRule<'a, sql_syntax::AnySqlName, crate::sql::any::name::FormatAnySqlName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::sql::any::name::FormatAnySqlName::default())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlName {
    type Format =
        FormatOwnedWithRule<sql_syntax::AnySqlName, crate::sql::any::name::FormatAnySqlName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::sql::any::name::FormatAnySqlName::default())
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlReturnsType {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlReturnsType,
        crate::sql::any::returns_type::FormatAnySqlReturnsType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::returns_type::FormatAnySqlReturnsType::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlReturnsType {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlReturnsType,
        crate::sql::any::returns_type::FormatAnySqlReturnsType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::returns_type::FormatAnySqlReturnsType::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlSelectItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlSelectItem,
        crate::sql::any::select_item::FormatAnySqlSelectItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::select_item::FormatAnySqlSelectItem::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlSelectItem {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlSelectItem,
        crate::sql::any::select_item::FormatAnySqlSelectItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::select_item::FormatAnySqlSelectItem::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlSelectQuantifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlSelectQuantifier,
        crate::sql::any::select_quantifier::FormatAnySqlSelectQuantifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::select_quantifier::FormatAnySqlSelectQuantifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlSelectQuantifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlSelectQuantifier,
        crate::sql::any::select_quantifier::FormatAnySqlSelectQuantifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::select_quantifier::FormatAnySqlSelectQuantifier::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlStatement,
        crate::sql::any::statement::FormatAnySqlStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::statement::FormatAnySqlStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlStatement,
        crate::sql::any::statement::FormatAnySqlStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::statement::FormatAnySqlStatement::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlSubqueryBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlSubqueryBody,
        crate::sql::any::subquery_body::FormatAnySqlSubqueryBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::subquery_body::FormatAnySqlSubqueryBody::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlSubqueryBody {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlSubqueryBody,
        crate::sql::any::subquery_body::FormatAnySqlSubqueryBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::subquery_body::FormatAnySqlSubqueryBody::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlTypeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlTypeArraySuffix,
        crate::sql::any::type_array_suffix::FormatAnySqlTypeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::type_array_suffix::FormatAnySqlTypeArraySuffix::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlTypeArraySuffix {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlTypeArraySuffix,
        crate::sql::any::type_array_suffix::FormatAnySqlTypeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::type_array_suffix::FormatAnySqlTypeArraySuffix::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlTypeModifier {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlTypeModifier,
        crate::sql::any::type_modifier::FormatAnySqlTypeModifier,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::type_modifier::FormatAnySqlTypeModifier::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlTypeModifier {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlTypeModifier,
        crate::sql::any::type_modifier::FormatAnySqlTypeModifier,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::type_modifier::FormatAnySqlTypeModifier::default(),
        )
    }
}
