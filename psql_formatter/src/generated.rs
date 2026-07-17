//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, PsqlFormatContext, PsqlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<psql_syntax::PsqlAlias> for crate::psql::auxiliary::alias::FormatPsqlAlias {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlAlias, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlAlias>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlAlias {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlAlias,
        crate::psql::auxiliary::alias::FormatPsqlAlias,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::alias::FormatPsqlAlias::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlAlias {
    type Format =
        FormatOwnedWithRule<psql_syntax::PsqlAlias, crate::psql::auxiliary::alias::FormatPsqlAlias>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::alias::FormatPsqlAlias::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlArrayExpression>
    for crate::psql::expressions::array_expression::FormatPsqlArrayExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlArrayExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlArrayExpression,
        crate::psql::expressions::array_expression::FormatPsqlArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::array_expression::FormatPsqlArrayExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlArrayExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlArrayExpression,
        crate::psql::expressions::array_expression::FormatPsqlArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::array_expression::FormatPsqlArrayExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlArraySubscriptExpression>
    for crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlArraySubscriptExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlArraySubscriptExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlArraySubscriptExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlArraySubscriptExpression,
        crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: array_subscript_expression :: FormatPsqlArraySubscriptExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlArraySubscriptExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlArraySubscriptExpression,
        crate::psql::expressions::array_subscript_expression::FormatPsqlArraySubscriptExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: array_subscript_expression :: FormatPsqlArraySubscriptExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlBetweenExpression>
    for crate::psql::expressions::between_expression::FormatPsqlBetweenExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBetweenExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlBetweenExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBetweenExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBetweenExpression,
        crate::psql::expressions::between_expression::FormatPsqlBetweenExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::between_expression::FormatPsqlBetweenExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBetweenExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBetweenExpression,
        crate::psql::expressions::between_expression::FormatPsqlBetweenExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::between_expression::FormatPsqlBetweenExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBinaryExpression>
    for crate::psql::expressions::binary_expression::FormatPsqlBinaryExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBinaryExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlBinaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBinaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBinaryExpression,
        crate::psql::expressions::binary_expression::FormatPsqlBinaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::binary_expression::FormatPsqlBinaryExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBinaryExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBinaryExpression,
        crate::psql::expressions::binary_expression::FormatPsqlBinaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::binary_expression::FormatPsqlBinaryExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBooleanLiteralExpression>
    for crate::psql::expressions::boolean_literal_expression::FormatPsqlBooleanLiteralExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBooleanLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlBooleanLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBooleanLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBooleanLiteralExpression,
        crate::psql::expressions::boolean_literal_expression::FormatPsqlBooleanLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: boolean_literal_expression :: FormatPsqlBooleanLiteralExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBooleanLiteralExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBooleanLiteralExpression,
        crate::psql::expressions::boolean_literal_expression::FormatPsqlBooleanLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: boolean_literal_expression :: FormatPsqlBooleanLiteralExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlCallExpression>
    for crate::psql::expressions::call_expression::FormatPsqlCallExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCallExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCallExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCallExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCallExpression,
        crate::psql::expressions::call_expression::FormatPsqlCallExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::call_expression::FormatPsqlCallExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCallExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCallExpression,
        crate::psql::expressions::call_expression::FormatPsqlCallExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::call_expression::FormatPsqlCallExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCaseElseClause>
    for crate::psql::clauses::case_else_clause::FormatPsqlCaseElseClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCaseElseClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCaseElseClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCaseElseClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCaseElseClause,
        crate::psql::clauses::case_else_clause::FormatPsqlCaseElseClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::case_else_clause::FormatPsqlCaseElseClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCaseElseClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCaseElseClause,
        crate::psql::clauses::case_else_clause::FormatPsqlCaseElseClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::case_else_clause::FormatPsqlCaseElseClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCaseExpression>
    for crate::psql::expressions::case_expression::FormatPsqlCaseExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCaseExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCaseExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCaseExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCaseExpression,
        crate::psql::expressions::case_expression::FormatPsqlCaseExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::case_expression::FormatPsqlCaseExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCaseExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCaseExpression,
        crate::psql::expressions::case_expression::FormatPsqlCaseExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::case_expression::FormatPsqlCaseExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCaseWhenClause>
    for crate::psql::clauses::case_when_clause::FormatPsqlCaseWhenClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCaseWhenClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCaseWhenClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCaseWhenClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCaseWhenClause,
        crate::psql::clauses::case_when_clause::FormatPsqlCaseWhenClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::case_when_clause::FormatPsqlCaseWhenClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCaseWhenClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCaseWhenClause,
        crate::psql::clauses::case_when_clause::FormatPsqlCaseWhenClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::case_when_clause::FormatPsqlCaseWhenClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCastExpression>
    for crate::psql::expressions::cast_expression::FormatPsqlCastExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCastExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCastExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCastExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCastExpression,
        crate::psql::expressions::cast_expression::FormatPsqlCastExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::cast_expression::FormatPsqlCastExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCastExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCastExpression,
        crate::psql::expressions::cast_expression::FormatPsqlCastExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::cast_expression::FormatPsqlCastExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCastFunctionExpression>
    for crate::psql::expressions::cast_function_expression::FormatPsqlCastFunctionExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCastFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCastFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCastFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCastFunctionExpression,
        crate::psql::expressions::cast_function_expression::FormatPsqlCastFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: cast_function_expression :: FormatPsqlCastFunctionExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCastFunctionExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCastFunctionExpression,
        crate::psql::expressions::cast_function_expression::FormatPsqlCastFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: cast_function_expression :: FormatPsqlCastFunctionExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlColReference>
    for crate::psql::auxiliary::col_reference::FormatPsqlColReference
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlColReference, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlColReference>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlColReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlColReference,
        crate::psql::auxiliary::col_reference::FormatPsqlColReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::col_reference::FormatPsqlColReference::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlColReference {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlColReference,
        crate::psql::auxiliary::col_reference::FormatPsqlColReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::col_reference::FormatPsqlColReference::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlColumnList>
    for crate::psql::auxiliary::column_list::FormatPsqlColumnList
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlColumnList, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlColumnList>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlColumnList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlColumnList,
        crate::psql::auxiliary::column_list::FormatPsqlColumnList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::column_list::FormatPsqlColumnList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlColumnList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlColumnList,
        crate::psql::auxiliary::column_list::FormatPsqlColumnList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::column_list::FormatPsqlColumnList::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlCteDefinition>
    for crate::psql::auxiliary::cte_definition::FormatPsqlCteDefinition
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlCteDefinition,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlCteDefinition>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCteDefinition {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCteDefinition,
        crate::psql::auxiliary::cte_definition::FormatPsqlCteDefinition,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::cte_definition::FormatPsqlCteDefinition::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCteDefinition {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCteDefinition,
        crate::psql::auxiliary::cte_definition::FormatPsqlCteDefinition,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::cte_definition::FormatPsqlCteDefinition::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlDataBaseName>
    for crate::psql::auxiliary::data_base_name::FormatPsqlDataBaseName
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlDataBaseName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlDataBaseName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlDataBaseName {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlDataBaseName,
        crate::psql::auxiliary::data_base_name::FormatPsqlDataBaseName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::data_base_name::FormatPsqlDataBaseName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlDataBaseName {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlDataBaseName,
        crate::psql::auxiliary::data_base_name::FormatPsqlDataBaseName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::data_base_name::FormatPsqlDataBaseName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlDeleteStatement>
    for crate::psql::statements::delete_statement::FormatPsqlDeleteStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlDeleteStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlDeleteStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlDeleteStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlDeleteStatement,
        crate::psql::statements::delete_statement::FormatPsqlDeleteStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::delete_statement::FormatPsqlDeleteStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlDeleteStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlDeleteStatement,
        crate::psql::statements::delete_statement::FormatPsqlDeleteStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::delete_statement::FormatPsqlDeleteStatement::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlDeleteUsingClause>
    for crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlDeleteUsingClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlDeleteUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlDeleteUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlDeleteUsingClause,
        crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlDeleteUsingClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlDeleteUsingClause,
        crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::delete_using_clause::FormatPsqlDeleteUsingClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlDoNothingClause>
    for crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlDoNothingClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlDoNothingClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlDoNothingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlDoNothingClause,
        crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlDoNothingClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlDoNothingClause,
        crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::do_nothing_clause::FormatPsqlDoNothingClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlDoUpdateClause>
    for crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlDoUpdateClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlDoUpdateClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlDoUpdateClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlDoUpdateClause,
        crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlDoUpdateClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlDoUpdateClause,
        crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::do_update_clause::FormatPsqlDoUpdateClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlEmptyStatement>
    for crate::psql::statements::empty_statement::FormatPsqlEmptyStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlEmptyStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlEmptyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlEmptyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlEmptyStatement,
        crate::psql::statements::empty_statement::FormatPsqlEmptyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::empty_statement::FormatPsqlEmptyStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlEmptyStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlEmptyStatement,
        crate::psql::statements::empty_statement::FormatPsqlEmptyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::empty_statement::FormatPsqlEmptyStatement::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlFromClause>
    for crate::psql::clauses::from_clause::FormatPsqlFromClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlFromClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlFromClause,
        crate::psql::clauses::from_clause::FormatPsqlFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::from_clause::FormatPsqlFromClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlFromClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlFromClause,
        crate::psql::clauses::from_clause::FormatPsqlFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::from_clause::FormatPsqlFromClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlFromItem>
    for crate::psql::auxiliary::from_item::FormatPsqlFromItem
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlFromItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlFromItem>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlFromItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlFromItem,
        crate::psql::auxiliary::from_item::FormatPsqlFromItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::from_item::FormatPsqlFromItem::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlFromItem {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlFromItem,
        crate::psql::auxiliary::from_item::FormatPsqlFromItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::from_item::FormatPsqlFromItem::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlFunctionBinding>
    for crate::psql::bindings::function_binding::FormatPsqlFunctionBinding
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlFunctionBinding,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlFunctionBinding>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlFunctionBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlFunctionBinding,
        crate::psql::bindings::function_binding::FormatPsqlFunctionBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bindings::function_binding::FormatPsqlFunctionBinding::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlFunctionBinding {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlFunctionBinding,
        crate::psql::bindings::function_binding::FormatPsqlFunctionBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bindings::function_binding::FormatPsqlFunctionBinding::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlGroupByClause>
    for crate::psql::clauses::group_by_clause::FormatPsqlGroupByClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlGroupByClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlGroupByClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlGroupByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlGroupByClause,
        crate::psql::clauses::group_by_clause::FormatPsqlGroupByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::group_by_clause::FormatPsqlGroupByClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlGroupByClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlGroupByClause,
        crate::psql::clauses::group_by_clause::FormatPsqlGroupByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::group_by_clause::FormatPsqlGroupByClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlHavingClause>
    for crate::psql::clauses::having_clause::FormatPsqlHavingClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlHavingClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlHavingClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlHavingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlHavingClause,
        crate::psql::clauses::having_clause::FormatPsqlHavingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::having_clause::FormatPsqlHavingClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlHavingClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlHavingClause,
        crate::psql::clauses::having_clause::FormatPsqlHavingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::having_clause::FormatPsqlHavingClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlInExpression>
    for crate::psql::expressions::in_expression::FormatPsqlInExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlInExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlInExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlInExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlInExpression,
        crate::psql::expressions::in_expression::FormatPsqlInExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::in_expression::FormatPsqlInExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlInExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlInExpression,
        crate::psql::expressions::in_expression::FormatPsqlInExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::in_expression::FormatPsqlInExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlInValueList>
    for crate::psql::auxiliary::in_value_list::FormatPsqlInValueList
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlInValueList, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlInValueList>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlInValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlInValueList,
        crate::psql::auxiliary::in_value_list::FormatPsqlInValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::in_value_list::FormatPsqlInValueList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlInValueList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlInValueList,
        crate::psql::auxiliary::in_value_list::FormatPsqlInValueList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::in_value_list::FormatPsqlInValueList::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlInsertStatement>
    for crate::psql::statements::insert_statement::FormatPsqlInsertStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlInsertStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlInsertStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlInsertStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlInsertStatement,
        crate::psql::statements::insert_statement::FormatPsqlInsertStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::insert_statement::FormatPsqlInsertStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlInsertStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlInsertStatement,
        crate::psql::statements::insert_statement::FormatPsqlInsertStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::insert_statement::FormatPsqlInsertStatement::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlInsertValues>
    for crate::psql::auxiliary::insert_values::FormatPsqlInsertValues
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlInsertValues, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlInsertValues>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlInsertValues {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlInsertValues,
        crate::psql::auxiliary::insert_values::FormatPsqlInsertValues,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::insert_values::FormatPsqlInsertValues::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlInsertValues {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlInsertValues,
        crate::psql::auxiliary::insert_values::FormatPsqlInsertValues,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::insert_values::FormatPsqlInsertValues::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlIsNullExpression>
    for crate::psql::expressions::is_null_expression::FormatPsqlIsNullExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlIsNullExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlIsNullExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlIsNullExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlIsNullExpression,
        crate::psql::expressions::is_null_expression::FormatPsqlIsNullExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::is_null_expression::FormatPsqlIsNullExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlIsNullExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlIsNullExpression,
        crate::psql::expressions::is_null_expression::FormatPsqlIsNullExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::is_null_expression::FormatPsqlIsNullExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlJoinClause>
    for crate::psql::clauses::join_clause::FormatPsqlJoinClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlJoinClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlJoinClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlJoinClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlJoinClause,
        crate::psql::clauses::join_clause::FormatPsqlJoinClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::join_clause::FormatPsqlJoinClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlJoinClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlJoinClause,
        crate::psql::clauses::join_clause::FormatPsqlJoinClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::join_clause::FormatPsqlJoinClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlLikeExpression>
    for crate::psql::expressions::like_expression::FormatPsqlLikeExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlLikeExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlLikeExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlLikeExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlLikeExpression,
        crate::psql::expressions::like_expression::FormatPsqlLikeExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::like_expression::FormatPsqlLikeExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlLikeExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlLikeExpression,
        crate::psql::expressions::like_expression::FormatPsqlLikeExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::like_expression::FormatPsqlLikeExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlLimitClause>
    for crate::psql::clauses::limit_clause::FormatPsqlLimitClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlLimitClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlLimitClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlLimitClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlLimitClause,
        crate::psql::clauses::limit_clause::FormatPsqlLimitClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::limit_clause::FormatPsqlLimitClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlLimitClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlLimitClause,
        crate::psql::clauses::limit_clause::FormatPsqlLimitClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::limit_clause::FormatPsqlLimitClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlLogicalExpression>
    for crate::psql::expressions::logical_expression::FormatPsqlLogicalExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlLogicalExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlLogicalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlLogicalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlLogicalExpression,
        crate::psql::expressions::logical_expression::FormatPsqlLogicalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::logical_expression::FormatPsqlLogicalExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlLogicalExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlLogicalExpression,
        crate::psql::expressions::logical_expression::FormatPsqlLogicalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::logical_expression::FormatPsqlLogicalExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlName> for crate::psql::auxiliary::name::FormatPsqlName {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlName {
    type Format<'a> =
        FormatRefWithRule<'a, psql_syntax::PsqlName, crate::psql::auxiliary::name::FormatPsqlName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::name::FormatPsqlName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlName {
    type Format =
        FormatOwnedWithRule<psql_syntax::PsqlName, crate::psql::auxiliary::name::FormatPsqlName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::name::FormatPsqlName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlNullLiteralExpression>
    for crate::psql::expressions::null_literal_expression::FormatPsqlNullLiteralExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlNullLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlNullLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlNullLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlNullLiteralExpression,
        crate::psql::expressions::null_literal_expression::FormatPsqlNullLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: null_literal_expression :: FormatPsqlNullLiteralExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlNullLiteralExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlNullLiteralExpression,
        crate::psql::expressions::null_literal_expression::FormatPsqlNullLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: null_literal_expression :: FormatPsqlNullLiteralExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlNumberLiteralExpression>
    for crate::psql::expressions::number_literal_expression::FormatPsqlNumberLiteralExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlNumberLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlNumberLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlNumberLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlNumberLiteralExpression,
        crate::psql::expressions::number_literal_expression::FormatPsqlNumberLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: number_literal_expression :: FormatPsqlNumberLiteralExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlNumberLiteralExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlNumberLiteralExpression,
        crate::psql::expressions::number_literal_expression::FormatPsqlNumberLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: number_literal_expression :: FormatPsqlNumberLiteralExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlOffsetClause>
    for crate::psql::clauses::offset_clause::FormatPsqlOffsetClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlOffsetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlOffsetClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOffsetClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOffsetClause,
        crate::psql::clauses::offset_clause::FormatPsqlOffsetClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::offset_clause::FormatPsqlOffsetClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOffsetClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOffsetClause,
        crate::psql::clauses::offset_clause::FormatPsqlOffsetClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::offset_clause::FormatPsqlOffsetClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlOnConflictClause>
    for crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlOnConflictClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlOnConflictClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOnConflictClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOnConflictClause,
        crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOnConflictClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOnConflictClause,
        crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::on_conflict_clause::FormatPsqlOnConflictClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlOnConstraintClause>
    for crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlOnConstraintClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlOnConstraintClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOnConstraintClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOnConstraintClause,
        crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOnConstraintClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOnConstraintClause,
        crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::on_constraint_clause::FormatPsqlOnConstraintClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlOrderByClause>
    for crate::psql::clauses::order_by_clause::FormatPsqlOrderByClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlOrderByClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlOrderByClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOrderByClause,
        crate::psql::clauses::order_by_clause::FormatPsqlOrderByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::order_by_clause::FormatPsqlOrderByClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOrderByClause,
        crate::psql::clauses::order_by_clause::FormatPsqlOrderByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::order_by_clause::FormatPsqlOrderByClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlOrderByExpression>
    for crate::psql::expressions::order_by_expression::FormatPsqlOrderByExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlOrderByExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlOrderByExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOrderByExpression,
        crate::psql::expressions::order_by_expression::FormatPsqlOrderByExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::order_by_expression::FormatPsqlOrderByExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOrderByExpression,
        crate::psql::expressions::order_by_expression::FormatPsqlOrderByExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::order_by_expression::FormatPsqlOrderByExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlParenthesizedExpression>
    for crate::psql::expressions::parenthesized_expression::FormatPsqlParenthesizedExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlParenthesizedExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlParenthesizedExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlParenthesizedExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlParenthesizedExpression,
        crate::psql::expressions::parenthesized_expression::FormatPsqlParenthesizedExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: parenthesized_expression :: FormatPsqlParenthesizedExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlParenthesizedExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlParenthesizedExpression,
        crate::psql::expressions::parenthesized_expression::FormatPsqlParenthesizedExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: parenthesized_expression :: FormatPsqlParenthesizedExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlReturningClause>
    for crate::psql::clauses::returning_clause::FormatPsqlReturningClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlReturningClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlReturningClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlReturningClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlReturningClause,
        crate::psql::clauses::returning_clause::FormatPsqlReturningClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::returning_clause::FormatPsqlReturningClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlReturningClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlReturningClause,
        crate::psql::clauses::returning_clause::FormatPsqlReturningClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::returning_clause::FormatPsqlReturningClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlRoot> for crate::psql::auxiliary::root::FormatPsqlRoot {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlRoot, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlRoot>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlRoot {
    type Format<'a> =
        FormatRefWithRule<'a, psql_syntax::PsqlRoot, crate::psql::auxiliary::root::FormatPsqlRoot>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::root::FormatPsqlRoot::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlRoot {
    type Format =
        FormatOwnedWithRule<psql_syntax::PsqlRoot, crate::psql::auxiliary::root::FormatPsqlRoot>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::root::FormatPsqlRoot::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSelectClause>
    for crate::psql::clauses::select_clause::FormatPsqlSelectClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlSelectClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSelectClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSelectClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSelectClause,
        crate::psql::clauses::select_clause::FormatPsqlSelectClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::select_clause::FormatPsqlSelectClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSelectClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSelectClause,
        crate::psql::clauses::select_clause::FormatPsqlSelectClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::select_clause::FormatPsqlSelectClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSelectExpression>
    for crate::psql::expressions::select_expression::FormatPsqlSelectExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlSelectExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSelectExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSelectExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSelectExpression,
        crate::psql::expressions::select_expression::FormatPsqlSelectExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::select_expression::FormatPsqlSelectExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSelectExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSelectExpression,
        crate::psql::expressions::select_expression::FormatPsqlSelectExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::select_expression::FormatPsqlSelectExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSelectStatement>
    for crate::psql::statements::select_statement::FormatPsqlSelectStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlSelectStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSelectStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSelectStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSelectStatement,
        crate::psql::statements::select_statement::FormatPsqlSelectStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::select_statement::FormatPsqlSelectStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSelectStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSelectStatement,
        crate::psql::statements::select_statement::FormatPsqlSelectStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::select_statement::FormatPsqlSelectStatement::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSetClause>
    for crate::psql::clauses::set_clause::FormatPsqlSetClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlSetClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSetClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSetClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSetClause,
        crate::psql::clauses::set_clause::FormatPsqlSetClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::set_clause::FormatPsqlSetClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSetClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSetClause,
        crate::psql::clauses::set_clause::FormatPsqlSetClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::set_clause::FormatPsqlSetClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSetItem> for crate::psql::auxiliary::set_item::FormatPsqlSetItem {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlSetItem, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSetItem>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSetItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSetItem,
        crate::psql::auxiliary::set_item::FormatPsqlSetItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::set_item::FormatPsqlSetItem::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSetItem {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSetItem,
        crate::psql::auxiliary::set_item::FormatPsqlSetItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::set_item::FormatPsqlSetItem::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSetOperation>
    for crate::psql::auxiliary::set_operation::FormatPsqlSetOperation
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlSetOperation, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSetOperation>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSetOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSetOperation,
        crate::psql::auxiliary::set_operation::FormatPsqlSetOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::set_operation::FormatPsqlSetOperation::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSetOperation {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSetOperation,
        crate::psql::auxiliary::set_operation::FormatPsqlSetOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::set_operation::FormatPsqlSetOperation::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlShemaName>
    for crate::psql::auxiliary::shema_name::FormatPsqlShemaName
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlShemaName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlShemaName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlShemaName {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlShemaName,
        crate::psql::auxiliary::shema_name::FormatPsqlShemaName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::shema_name::FormatPsqlShemaName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlShemaName {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlShemaName,
        crate::psql::auxiliary::shema_name::FormatPsqlShemaName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::shema_name::FormatPsqlShemaName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlStar> for crate::psql::auxiliary::star::FormatPsqlStar {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlStar, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlStar>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlStar {
    type Format<'a> =
        FormatRefWithRule<'a, psql_syntax::PsqlStar, crate::psql::auxiliary::star::FormatPsqlStar>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::star::FormatPsqlStar::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlStar {
    type Format =
        FormatOwnedWithRule<psql_syntax::PsqlStar, crate::psql::auxiliary::star::FormatPsqlStar>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::star::FormatPsqlStar::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlStringLiteralExpression>
    for crate::psql::expressions::string_literal_expression::FormatPsqlStringLiteralExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlStringLiteralExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlStringLiteralExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlStringLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlStringLiteralExpression,
        crate::psql::expressions::string_literal_expression::FormatPsqlStringLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: string_literal_expression :: FormatPsqlStringLiteralExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlStringLiteralExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlStringLiteralExpression,
        crate::psql::expressions::string_literal_expression::FormatPsqlStringLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: string_literal_expression :: FormatPsqlStringLiteralExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlSubqueryBinding>
    for crate::psql::bindings::subquery_binding::FormatPsqlSubqueryBinding
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlSubqueryBinding,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSubqueryBinding>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSubqueryBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSubqueryBinding,
        crate::psql::bindings::subquery_binding::FormatPsqlSubqueryBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bindings::subquery_binding::FormatPsqlSubqueryBinding::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSubqueryBinding {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSubqueryBinding,
        crate::psql::bindings::subquery_binding::FormatPsqlSubqueryBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bindings::subquery_binding::FormatPsqlSubqueryBinding::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlSubqueryExpression>
    for crate::psql::expressions::subquery_expression::FormatPsqlSubqueryExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlSubqueryExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlSubqueryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSubqueryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSubqueryExpression,
        crate::psql::expressions::subquery_expression::FormatPsqlSubqueryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::subquery_expression::FormatPsqlSubqueryExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSubqueryExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSubqueryExpression,
        crate::psql::expressions::subquery_expression::FormatPsqlSubqueryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::subquery_expression::FormatPsqlSubqueryExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTableBinding>
    for crate::psql::bindings::table_binding::FormatPsqlTableBinding
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlTableBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTableBinding>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTableBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTableBinding,
        crate::psql::bindings::table_binding::FormatPsqlTableBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bindings::table_binding::FormatPsqlTableBinding::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTableBinding {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTableBinding,
        crate::psql::bindings::table_binding::FormatPsqlTableBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bindings::table_binding::FormatPsqlTableBinding::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTableColReference>
    for crate::psql::auxiliary::table_col_reference::FormatPsqlTableColReference
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlTableColReference,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTableColReference>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTableColReference {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTableColReference,
        crate::psql::auxiliary::table_col_reference::FormatPsqlTableColReference,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::table_col_reference::FormatPsqlTableColReference::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTableColReference {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTableColReference,
        crate::psql::auxiliary::table_col_reference::FormatPsqlTableColReference,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::table_col_reference::FormatPsqlTableColReference::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTableName>
    for crate::psql::auxiliary::table_name::FormatPsqlTableName
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlTableName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTableName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTableName {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTableName,
        crate::psql::auxiliary::table_name::FormatPsqlTableName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::table_name::FormatPsqlTableName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTableName {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTableName,
        crate::psql::auxiliary::table_name::FormatPsqlTableName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::table_name::FormatPsqlTableName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTildeArraySuffix>
    for crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlTildeArraySuffix,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTildeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTildeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTildeArraySuffix,
        crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTildeArraySuffix {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTildeArraySuffix,
        crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::tilde_array_suffix::FormatPsqlTildeArraySuffix::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTildeName>
    for crate::psql::auxiliary::tilde_name::FormatPsqlTildeName
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlTildeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTildeName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTildeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTildeName,
        crate::psql::auxiliary::tilde_name::FormatPsqlTildeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::tilde_name::FormatPsqlTildeName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTildeName {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTildeName,
        crate::psql::auxiliary::tilde_name::FormatPsqlTildeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::tilde_name::FormatPsqlTildeName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTypeArguments>
    for crate::psql::auxiliary::type_arguments::FormatPsqlTypeArguments
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlTypeArguments,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTypeArguments>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArguments {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTypeArguments,
        crate::psql::auxiliary::type_arguments::FormatPsqlTypeArguments,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::type_arguments::FormatPsqlTypeArguments::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArguments {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTypeArguments,
        crate::psql::auxiliary::type_arguments::FormatPsqlTypeArguments,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::type_arguments::FormatPsqlTypeArguments::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTypeArraySuffix>
    for crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlTypeArraySuffix,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTypeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTypeArraySuffix,
        crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArraySuffix {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTypeArraySuffix,
        crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::type_array_suffix::FormatPsqlTypeArraySuffix::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlTypeName>
    for crate::psql::auxiliary::type_name::FormatPsqlTypeName
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlTypeName, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlTypeName>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTypeName {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTypeName,
        crate::psql::auxiliary::type_name::FormatPsqlTypeName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::type_name::FormatPsqlTypeName::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTypeName {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTypeName,
        crate::psql::auxiliary::type_name::FormatPsqlTypeName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::type_name::FormatPsqlTypeName::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlUnaryExpression>
    for crate::psql::expressions::unary_expression::FormatPsqlUnaryExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlUnaryExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlUnaryExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlUnaryExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlUnaryExpression,
        crate::psql::expressions::unary_expression::FormatPsqlUnaryExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::unary_expression::FormatPsqlUnaryExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlUnaryExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlUnaryExpression,
        crate::psql::expressions::unary_expression::FormatPsqlUnaryExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::unary_expression::FormatPsqlUnaryExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlUpdateStatement>
    for crate::psql::statements::update_statement::FormatPsqlUpdateStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlUpdateStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlUpdateStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlUpdateStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlUpdateStatement,
        crate::psql::statements::update_statement::FormatPsqlUpdateStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::update_statement::FormatPsqlUpdateStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlUpdateStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlUpdateStatement,
        crate::psql::statements::update_statement::FormatPsqlUpdateStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::update_statement::FormatPsqlUpdateStatement::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlWhereClause>
    for crate::psql::clauses::where_clause::FormatPsqlWhereClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlWhereClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlWhereClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWhereClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWhereClause,
        crate::psql::clauses::where_clause::FormatPsqlWhereClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::where_clause::FormatPsqlWhereClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWhereClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWhereClause,
        crate::psql::clauses::where_clause::FormatPsqlWhereClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::where_clause::FormatPsqlWhereClause::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlWindowFunctionExpression>
    for crate::psql::expressions::window_function_expression::FormatPsqlWindowFunctionExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlWindowFunctionExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlWindowFunctionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWindowFunctionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWindowFunctionExpression,
        crate::psql::expressions::window_function_expression::FormatPsqlWindowFunctionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: window_function_expression :: FormatPsqlWindowFunctionExpression :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWindowFunctionExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWindowFunctionExpression,
        crate::psql::expressions::window_function_expression::FormatPsqlWindowFunctionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: window_function_expression :: FormatPsqlWindowFunctionExpression :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlWindowPartitionByClause>
    for crate::psql::clauses::window_partition_by_clause::FormatPsqlWindowPartitionByClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlWindowPartitionByClause,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlWindowPartitionByClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWindowPartitionByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWindowPartitionByClause,
        crate::psql::clauses::window_partition_by_clause::FormatPsqlWindowPartitionByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: clauses :: window_partition_by_clause :: FormatPsqlWindowPartitionByClause :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWindowPartitionByClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWindowPartitionByClause,
        crate::psql::clauses::window_partition_by_clause::FormatPsqlWindowPartitionByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: clauses :: window_partition_by_clause :: FormatPsqlWindowPartitionByClause :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlWindowSpecification>
    for crate::psql::auxiliary::window_specification::FormatPsqlWindowSpecification
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlWindowSpecification,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlWindowSpecification>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWindowSpecification {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWindowSpecification,
        crate::psql::auxiliary::window_specification::FormatPsqlWindowSpecification,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::window_specification::FormatPsqlWindowSpecification::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWindowSpecification {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWindowSpecification,
        crate::psql::auxiliary::window_specification::FormatPsqlWindowSpecification,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::window_specification::FormatPsqlWindowSpecification::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlWithClause>
    for crate::psql::clauses::with_clause::FormatPsqlWithClause
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlWithClause, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<psql_syntax::PsqlWithClause>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWithClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWithClause,
        crate::psql::clauses::with_clause::FormatPsqlWithClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::with_clause::FormatPsqlWithClause::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWithClause {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWithClause,
        crate::psql::clauses::with_clause::FormatPsqlWithClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::with_clause::FormatPsqlWithClause::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCaseWhenClauseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCaseWhenClauseList,
        crate::psql::lists::case_when_clause_list::FormatPsqlCaseWhenClauseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::case_when_clause_list::FormatPsqlCaseWhenClauseList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCaseWhenClauseList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCaseWhenClauseList,
        crate::psql::lists::case_when_clause_list::FormatPsqlCaseWhenClauseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::case_when_clause_list::FormatPsqlCaseWhenClauseList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlColumnNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlColumnNameList,
        crate::psql::lists::column_name_list::FormatPsqlColumnNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::column_name_list::FormatPsqlColumnNameList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlColumnNameList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlColumnNameList,
        crate::psql::lists::column_name_list::FormatPsqlColumnNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::column_name_list::FormatPsqlColumnNameList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlCteDefinitionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlCteDefinitionList,
        crate::psql::lists::cte_definition_list::FormatPsqlCteDefinitionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::cte_definition_list::FormatPsqlCteDefinitionList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlCteDefinitionList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlCteDefinitionList,
        crate::psql::lists::cte_definition_list::FormatPsqlCteDefinitionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::cte_definition_list::FormatPsqlCteDefinitionList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlExpressionList,
        crate::psql::lists::expression_list::FormatPsqlExpressionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::expression_list::FormatPsqlExpressionList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlExpressionList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlExpressionList,
        crate::psql::lists::expression_list::FormatPsqlExpressionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::expression_list::FormatPsqlExpressionList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlFromItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlFromItemList,
        crate::psql::lists::from_item_list::FormatPsqlFromItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::from_item_list::FormatPsqlFromItemList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlFromItemList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlFromItemList,
        crate::psql::lists::from_item_list::FormatPsqlFromItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::from_item_list::FormatPsqlFromItemList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlGroupByItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlGroupByItemList,
        crate::psql::lists::group_by_item_list::FormatPsqlGroupByItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::group_by_item_list::FormatPsqlGroupByItemList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlGroupByItemList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlGroupByItemList,
        crate::psql::lists::group_by_item_list::FormatPsqlGroupByItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::group_by_item_list::FormatPsqlGroupByItemList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlJoinClauseList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlJoinClauseList,
        crate::psql::lists::join_clause_list::FormatPsqlJoinClauseList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::join_clause_list::FormatPsqlJoinClauseList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlJoinClauseList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlJoinClauseList,
        crate::psql::lists::join_clause_list::FormatPsqlJoinClauseList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::join_clause_list::FormatPsqlJoinClauseList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByExpressionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlOrderByExpressionList,
        crate::psql::lists::order_by_expression_list::FormatPsqlOrderByExpressionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::order_by_expression_list::FormatPsqlOrderByExpressionList::default(
            ),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlOrderByExpressionList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlOrderByExpressionList,
        crate::psql::lists::order_by_expression_list::FormatPsqlOrderByExpressionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::order_by_expression_list::FormatPsqlOrderByExpressionList::default(
            ),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSelectItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSelectItemList,
        crate::psql::lists::select_item_list::FormatPsqlSelectItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::select_item_list::FormatPsqlSelectItemList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSelectItemList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSelectItemList,
        crate::psql::lists::select_item_list::FormatPsqlSelectItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::select_item_list::FormatPsqlSelectItemList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSetItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSetItemList,
        crate::psql::lists::set_item_list::FormatPsqlSetItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::set_item_list::FormatPsqlSetItemList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSetItemList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSetItemList,
        crate::psql::lists::set_item_list::FormatPsqlSetItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::set_item_list::FormatPsqlSetItemList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlSetOperationList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlSetOperationList,
        crate::psql::lists::set_operation_list::FormatPsqlSetOperationList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::set_operation_list::FormatPsqlSetOperationList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlSetOperationList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlSetOperationList,
        crate::psql::lists::set_operation_list::FormatPsqlSetOperationList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::set_operation_list::FormatPsqlSetOperationList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlStatementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlStatementList,
        crate::psql::lists::statement_list::FormatPsqlStatementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::statement_list::FormatPsqlStatementList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlStatementList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlStatementList,
        crate::psql::lists::statement_list::FormatPsqlStatementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::statement_list::FormatPsqlStatementList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlTypeArgumentList,
        crate::psql::lists::type_argument_list::FormatPsqlTypeArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::type_argument_list::FormatPsqlTypeArgumentList::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlTypeArgumentList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlTypeArgumentList,
        crate::psql::lists::type_argument_list::FormatPsqlTypeArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::type_argument_list::FormatPsqlTypeArgumentList::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlWindowPartitionByItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlWindowPartitionByItemList,
        crate::psql::lists::window_partition_by_item_list::FormatPsqlWindowPartitionByItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: lists :: window_partition_by_item_list :: FormatPsqlWindowPartitionByItemList :: default ())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlWindowPartitionByItemList {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlWindowPartitionByItemList,
        crate::psql::lists::window_partition_by_item_list::FormatPsqlWindowPartitionByItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: lists :: window_partition_by_item_list :: FormatPsqlWindowPartitionByItemList :: default ())
    }
}
impl FormatRule<psql_syntax::PsqlBogus> for crate::psql::bogus::bogus::FormatPsqlBogus {
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlBogus, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogus>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogus {
    type Format<'a> =
        FormatRefWithRule<'a, psql_syntax::PsqlBogus, crate::psql::bogus::bogus::FormatPsqlBogus>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::psql::bogus::bogus::FormatPsqlBogus::default())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogus {
    type Format =
        FormatOwnedWithRule<psql_syntax::PsqlBogus, crate::psql::bogus::bogus::FormatPsqlBogus>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::psql::bogus::bogus::FormatPsqlBogus::default())
    }
}
impl FormatRule<psql_syntax::PsqlBogusAssignment>
    for crate::psql::bogus::bogus_assignment::FormatPsqlBogusAssignment
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBogusAssignment,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusAssignment>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusAssignment {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusAssignment,
        crate::psql::bogus::bogus_assignment::FormatPsqlBogusAssignment,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_assignment::FormatPsqlBogusAssignment::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusAssignment {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusAssignment,
        crate::psql::bogus::bogus_assignment::FormatPsqlBogusAssignment,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_assignment::FormatPsqlBogusAssignment::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBogusBinding>
    for crate::psql::bogus::bogus_binding::FormatPsqlBogusBinding
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlBogusBinding, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusBinding>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusBinding {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusBinding,
        crate::psql::bogus::bogus_binding::FormatPsqlBogusBinding,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_binding::FormatPsqlBogusBinding::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusBinding {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusBinding,
        crate::psql::bogus::bogus_binding::FormatPsqlBogusBinding,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_binding::FormatPsqlBogusBinding::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBogusExpression>
    for crate::psql::bogus::bogus_expression::FormatPsqlBogusExpression
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBogusExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusExpression>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusExpression,
        crate::psql::bogus::bogus_expression::FormatPsqlBogusExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_expression::FormatPsqlBogusExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusExpression,
        crate::psql::bogus::bogus_expression::FormatPsqlBogusExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_expression::FormatPsqlBogusExpression::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBogusMember>
    for crate::psql::bogus::bogus_member::FormatPsqlBogusMember
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &psql_syntax::PsqlBogusMember, f: &mut PsqlFormatter) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusMember>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusMember {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusMember,
        crate::psql::bogus::bogus_member::FormatPsqlBogusMember,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_member::FormatPsqlBogusMember::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusMember {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusMember,
        crate::psql::bogus::bogus_member::FormatPsqlBogusMember,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_member::FormatPsqlBogusMember::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBogusParameter>
    for crate::psql::bogus::bogus_parameter::FormatPsqlBogusParameter
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBogusParameter,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusParameter>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusParameter,
        crate::psql::bogus::bogus_parameter::FormatPsqlBogusParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_parameter::FormatPsqlBogusParameter::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusParameter {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusParameter,
        crate::psql::bogus::bogus_parameter::FormatPsqlBogusParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_parameter::FormatPsqlBogusParameter::default(),
        )
    }
}
impl FormatRule<psql_syntax::PsqlBogusStatement>
    for crate::psql::bogus::bogus_statement::FormatPsqlBogusStatement
{
    type Context = PsqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &psql_syntax::PsqlBogusStatement,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        FormatBogusNodeRule::<psql_syntax::PsqlBogusStatement>::fmt(self, node, f)
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::PsqlBogusStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::PsqlBogusStatement,
        crate::psql::bogus::bogus_statement::FormatPsqlBogusStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::bogus::bogus_statement::FormatPsqlBogusStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::PsqlBogusStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::PsqlBogusStatement,
        crate::psql::bogus::bogus_statement::FormatPsqlBogusStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::bogus::bogus_statement::FormatPsqlBogusStatement::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlConflictAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlConflictAction,
        crate::psql::any::conflict_action::FormatAnyPsqlConflictAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::conflict_action::FormatAnyPsqlConflictAction::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlConflictAction {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlConflictAction,
        crate::psql::any::conflict_action::FormatAnyPsqlConflictAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::conflict_action::FormatAnyPsqlConflictAction::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlConflictTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlConflictTarget,
        crate::psql::any::conflict_target::FormatAnyPsqlConflictTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::conflict_target::FormatAnyPsqlConflictTarget::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlConflictTarget {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlConflictTarget,
        crate::psql::any::conflict_target::FormatAnyPsqlConflictTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::conflict_target::FormatAnyPsqlConflictTarget::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlExpression,
        crate::psql::any::expression::FormatAnyPsqlExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::expression::FormatAnyPsqlExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlExpression,
        crate::psql::any::expression::FormatAnyPsqlExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::expression::FormatAnyPsqlExpression::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlFromExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlFromExpression,
        crate::psql::any::from_expression::FormatAnyPsqlFromExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::from_expression::FormatAnyPsqlFromExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlFromExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlFromExpression,
        crate::psql::any::from_expression::FormatAnyPsqlFromExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::from_expression::FormatAnyPsqlFromExpression::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlInSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlInSource,
        crate::psql::any::in_source::FormatAnyPsqlInSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::in_source::FormatAnyPsqlInSource::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlInSource {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlInSource,
        crate::psql::any::in_source::FormatAnyPsqlInSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::in_source::FormatAnyPsqlInSource::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlInsertSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlInsertSource,
        crate::psql::any::insert_source::FormatAnyPsqlInsertSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::insert_source::FormatAnyPsqlInsertSource::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlInsertSource {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlInsertSource,
        crate::psql::any::insert_source::FormatAnyPsqlInsertSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::insert_source::FormatAnyPsqlInsertSource::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlLiteralExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlLiteralExpression,
        crate::psql::any::literal_expression::FormatAnyPsqlLiteralExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::literal_expression::FormatAnyPsqlLiteralExpression::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlLiteralExpression {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlLiteralExpression,
        crate::psql::any::literal_expression::FormatAnyPsqlLiteralExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::literal_expression::FormatAnyPsqlLiteralExpression::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlName {
    type Format<'a> =
        FormatRefWithRule<'a, psql_syntax::AnyPsqlName, crate::psql::any::name::FormatAnyPsqlName>;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(self, crate::psql::any::name::FormatAnyPsqlName::default())
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlName {
    type Format =
        FormatOwnedWithRule<psql_syntax::AnyPsqlName, crate::psql::any::name::FormatAnyPsqlName>;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(self, crate::psql::any::name::FormatAnyPsqlName::default())
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlSelectItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlSelectItem,
        crate::psql::any::select_item::FormatAnyPsqlSelectItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::select_item::FormatAnyPsqlSelectItem::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlSelectItem {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlSelectItem,
        crate::psql::any::select_item::FormatAnyPsqlSelectItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::select_item::FormatAnyPsqlSelectItem::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlStatement,
        crate::psql::any::statement::FormatAnyPsqlStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::statement::FormatAnyPsqlStatement::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlStatement {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlStatement,
        crate::psql::any::statement::FormatAnyPsqlStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::statement::FormatAnyPsqlStatement::default(),
        )
    }
}
impl AsFormat<PsqlFormatContext> for psql_syntax::AnyPsqlTypeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        psql_syntax::AnyPsqlTypeArraySuffix,
        crate::psql::any::type_array_suffix::FormatAnyPsqlTypeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::type_array_suffix::FormatAnyPsqlTypeArraySuffix::default(),
        )
    }
}
impl IntoFormat<PsqlFormatContext> for psql_syntax::AnyPsqlTypeArraySuffix {
    type Format = FormatOwnedWithRule<
        psql_syntax::AnyPsqlTypeArraySuffix,
        crate::psql::any::type_array_suffix::FormatAnyPsqlTypeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::type_array_suffix::FormatAnyPsqlTypeArraySuffix::default(),
        )
    }
}
