//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, SqlFormatContext, SqlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
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
impl FormatRule<sql_syntax::SqlArrayExpression>
    for crate::sql::expressions::array_expression::FormatSqlArrayExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlArrayExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlArrayExpression,
        crate::sql::expressions::array_expression::FormatSqlArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::array_expression::FormatSqlArrayExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlArrayExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlArrayExpression,
        crate::sql::expressions::array_expression::FormatSqlArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::array_expression::FormatSqlArrayExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlArraySubscriptExpression>
    for crate::sql::expressions::array_subscript_expression::FormatSqlArraySubscriptExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlArraySubscriptExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlArraySubscriptExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlArraySubscriptExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlArraySubscriptExpression,
        crate::sql::expressions::array_subscript_expression::FormatSqlArraySubscriptExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: array_subscript_expression :: FormatSqlArraySubscriptExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlArraySubscriptExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlArraySubscriptExpression,
        crate::sql::expressions::array_subscript_expression::FormatSqlArraySubscriptExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: array_subscript_expression :: FormatSqlArraySubscriptExpression :: default ())
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
impl FormatRule<sql_syntax::SqlCastExpression>
    for crate::sql::expressions::cast_expression::FormatSqlCastExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCastExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCastExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCastExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCastExpression,
        crate::sql::expressions::cast_expression::FormatSqlCastExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::cast_expression::FormatSqlCastExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCastExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCastExpression,
        crate::sql::expressions::cast_expression::FormatSqlCastExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::cast_expression::FormatSqlCastExpression::default(),
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
impl FormatRule<sql_syntax::SqlCreateFunctionStatement>
    for crate::sql::statements::create_function_statement::FormatSqlCreateFunctionStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateFunctionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateFunctionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateFunctionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateFunctionStatement,
        crate::sql::statements::create_function_statement::FormatSqlCreateFunctionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: create_function_statement :: FormatSqlCreateFunctionStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateFunctionStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateFunctionStatement,
        crate::sql::statements::create_function_statement::FormatSqlCreateFunctionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: create_function_statement :: FormatSqlCreateFunctionStatement :: default ())
    }
}
impl FormatRule<sql_syntax::SqlCreatePolicyStatement>
    for crate::sql::statements::create_policy_statement::FormatSqlCreatePolicyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreatePolicyStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreatePolicyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreatePolicyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreatePolicyStatement,
        crate::sql::statements::create_policy_statement::FormatSqlCreatePolicyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: create_policy_statement :: FormatSqlCreatePolicyStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreatePolicyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreatePolicyStatement,
        crate::sql::statements::create_policy_statement::FormatSqlCreatePolicyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: create_policy_statement :: FormatSqlCreatePolicyStatement :: default ())
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
impl FormatRule<sql_syntax::SqlCreateTriggerStatement>
    for crate::sql::statements::create_trigger_statement::FormatSqlCreateTriggerStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateTriggerStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateTriggerStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateTriggerStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateTriggerStatement,
        crate::sql::statements::create_trigger_statement::FormatSqlCreateTriggerStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: create_trigger_statement :: FormatSqlCreateTriggerStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateTriggerStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateTriggerStatement,
        crate::sql::statements::create_trigger_statement::FormatSqlCreateTriggerStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: create_trigger_statement :: FormatSqlCreateTriggerStatement :: default ())
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
impl FormatRule<sql_syntax::SqlCteMaterializedHint>
    for crate::sql::auxiliary::cte_materialized_hint::FormatSqlCteMaterializedHint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCteMaterializedHint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCteMaterializedHint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCteMaterializedHint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCteMaterializedHint,
        crate::sql::auxiliary::cte_materialized_hint::FormatSqlCteMaterializedHint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::cte_materialized_hint::FormatSqlCteMaterializedHint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCteMaterializedHint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCteMaterializedHint,
        crate::sql::auxiliary::cte_materialized_hint::FormatSqlCteMaterializedHint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::cte_materialized_hint::FormatSqlCteMaterializedHint::default(),
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
impl FormatRule<sql_syntax::SqlDeleteUsingClause>
    for crate::sql::clauses::delete_using_clause::FormatSqlDeleteUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDeleteUsingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDeleteUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDeleteUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDeleteUsingClause,
        crate::sql::clauses::delete_using_clause::FormatSqlDeleteUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::delete_using_clause::FormatSqlDeleteUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDeleteUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDeleteUsingClause,
        crate::sql::clauses::delete_using_clause::FormatSqlDeleteUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::delete_using_clause::FormatSqlDeleteUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDistinctOnClause>
    for crate::sql::clauses::distinct_on_clause::FormatSqlDistinctOnClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDistinctOnClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDistinctOnClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDistinctOnClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDistinctOnClause,
        crate::sql::clauses::distinct_on_clause::FormatSqlDistinctOnClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::distinct_on_clause::FormatSqlDistinctOnClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDistinctOnClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDistinctOnClause,
        crate::sql::clauses::distinct_on_clause::FormatSqlDistinctOnClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::distinct_on_clause::FormatSqlDistinctOnClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDoNothingClause>
    for crate::sql::clauses::do_nothing_clause::FormatSqlDoNothingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlDoNothingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDoNothingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDoNothingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDoNothingClause,
        crate::sql::clauses::do_nothing_clause::FormatSqlDoNothingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::do_nothing_clause::FormatSqlDoNothingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDoNothingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDoNothingClause,
        crate::sql::clauses::do_nothing_clause::FormatSqlDoNothingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::do_nothing_clause::FormatSqlDoNothingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDoUpdateClause>
    for crate::sql::clauses::do_update_clause::FormatSqlDoUpdateClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlDoUpdateClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDoUpdateClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDoUpdateClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDoUpdateClause,
        crate::sql::clauses::do_update_clause::FormatSqlDoUpdateClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::do_update_clause::FormatSqlDoUpdateClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDoUpdateClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDoUpdateClause,
        crate::sql::clauses::do_update_clause::FormatSqlDoUpdateClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::do_update_clause::FormatSqlDoUpdateClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropFunctionParameters>
    for crate::sql::auxiliary::drop_function_parameters::FormatSqlDropFunctionParameters
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropFunctionParameters,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropFunctionParameters>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropFunctionParameters {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropFunctionParameters,
        crate::sql::auxiliary::drop_function_parameters::FormatSqlDropFunctionParameters,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: drop_function_parameters :: FormatSqlDropFunctionParameters :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropFunctionParameters {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropFunctionParameters,
        crate::sql::auxiliary::drop_function_parameters::FormatSqlDropFunctionParameters,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: drop_function_parameters :: FormatSqlDropFunctionParameters :: default ())
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
impl FormatRule<sql_syntax::SqlDropPolicyStatement>
    for crate::sql::statements::drop_policy_statement::FormatSqlDropPolicyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropPolicyStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropPolicyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropPolicyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropPolicyStatement,
        crate::sql::statements::drop_policy_statement::FormatSqlDropPolicyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::drop_policy_statement::FormatSqlDropPolicyStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropPolicyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropPolicyStatement,
        crate::sql::statements::drop_policy_statement::FormatSqlDropPolicyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::drop_policy_statement::FormatSqlDropPolicyStatement::default(),
        )
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
impl FormatRule<sql_syntax::SqlDropTriggerStatement>
    for crate::sql::statements::drop_trigger_statement::FormatSqlDropTriggerStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropTriggerStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropTriggerStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropTriggerStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropTriggerStatement,
        crate::sql::statements::drop_trigger_statement::FormatSqlDropTriggerStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::drop_trigger_statement::FormatSqlDropTriggerStatement::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropTriggerStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropTriggerStatement,
        crate::sql::statements::drop_trigger_statement::FormatSqlDropTriggerStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::drop_trigger_statement::FormatSqlDropTriggerStatement::default(
            ),
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
impl FormatRule<sql_syntax::SqlFilterClause>
    for crate::sql::clauses::filter_clause::FormatSqlFilterClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlFilterClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFilterClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFilterClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFilterClause,
        crate::sql::clauses::filter_clause::FormatSqlFilterClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::filter_clause::FormatSqlFilterClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFilterClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFilterClause,
        crate::sql::clauses::filter_clause::FormatSqlFilterClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::filter_clause::FormatSqlFilterClause::default(),
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
impl FormatRule<sql_syntax::SqlFunctionParameter>
    for crate::sql::auxiliary::function_parameter::FormatSqlFunctionParameter
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlFunctionParameter,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlFunctionParameter>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFunctionParameter {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFunctionParameter,
        crate::sql::auxiliary::function_parameter::FormatSqlFunctionParameter,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::function_parameter::FormatSqlFunctionParameter::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFunctionParameter {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFunctionParameter,
        crate::sql::auxiliary::function_parameter::FormatSqlFunctionParameter,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::function_parameter::FormatSqlFunctionParameter::default(),
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
impl FormatRule<sql_syntax::SqlIntervalExpression>
    for crate::sql::expressions::interval_expression::FormatSqlIntervalExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlIntervalExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlIntervalExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIntervalExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIntervalExpression,
        crate::sql::expressions::interval_expression::FormatSqlIntervalExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::interval_expression::FormatSqlIntervalExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIntervalExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIntervalExpression,
        crate::sql::expressions::interval_expression::FormatSqlIntervalExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::interval_expression::FormatSqlIntervalExpression::default(),
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
impl FormatRule<sql_syntax::SqlJoinUsingClause>
    for crate::sql::clauses::join_using_clause::FormatSqlJoinUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlJoinUsingClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlJoinUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlJoinUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlJoinUsingClause,
        crate::sql::clauses::join_using_clause::FormatSqlJoinUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::join_using_clause::FormatSqlJoinUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlJoinUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlJoinUsingClause,
        crate::sql::clauses::join_using_clause::FormatSqlJoinUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::join_using_clause::FormatSqlJoinUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlLanguageOption>
    for crate::sql::auxiliary::language_option::FormatSqlLanguageOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlLanguageOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlLanguageOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlLanguageOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlLanguageOption,
        crate::sql::auxiliary::language_option::FormatSqlLanguageOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::language_option::FormatSqlLanguageOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlLanguageOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlLanguageOption,
        crate::sql::auxiliary::language_option::FormatSqlLanguageOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::language_option::FormatSqlLanguageOption::default(),
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
impl FormatRule<sql_syntax::SqlLimitClause>
    for crate::sql::clauses::limit_clause::FormatSqlLimitClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlLimitClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlLimitClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlLimitClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlLimitClause,
        crate::sql::clauses::limit_clause::FormatSqlLimitClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::limit_clause::FormatSqlLimitClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlLimitClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlLimitClause,
        crate::sql::clauses::limit_clause::FormatSqlLimitClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::limit_clause::FormatSqlLimitClause::default(),
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
impl FormatRule<sql_syntax::SqlOnConflictClause>
    for crate::sql::clauses::on_conflict_clause::FormatSqlOnConflictClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlOnConflictClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlOnConflictClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOnConflictClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOnConflictClause,
        crate::sql::clauses::on_conflict_clause::FormatSqlOnConflictClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::on_conflict_clause::FormatSqlOnConflictClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOnConflictClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOnConflictClause,
        crate::sql::clauses::on_conflict_clause::FormatSqlOnConflictClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::on_conflict_clause::FormatSqlOnConflictClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlOnConstraintClause>
    for crate::sql::clauses::on_constraint_clause::FormatSqlOnConstraintClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlOnConstraintClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlOnConstraintClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlOnConstraintClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlOnConstraintClause,
        crate::sql::clauses::on_constraint_clause::FormatSqlOnConstraintClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::on_constraint_clause::FormatSqlOnConstraintClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlOnConstraintClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlOnConstraintClause,
        crate::sql::clauses::on_constraint_clause::FormatSqlOnConstraintClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::on_constraint_clause::FormatSqlOnConstraintClause::default(),
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
impl FormatRule<sql_syntax::SqlParameterDefault>
    for crate::sql::auxiliary::parameter_default::FormatSqlParameterDefault
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlParameterDefault,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlParameterDefault>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlParameterDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlParameterDefault,
        crate::sql::auxiliary::parameter_default::FormatSqlParameterDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::parameter_default::FormatSqlParameterDefault::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlParameterDefault {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlParameterDefault,
        crate::sql::auxiliary::parameter_default::FormatSqlParameterDefault,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::parameter_default::FormatSqlParameterDefault::default(),
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
impl FormatRule<sql_syntax::SqlPolicyForClause>
    for crate::sql::clauses::policy_for_clause::FormatSqlPolicyForClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlPolicyForClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPolicyForClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPolicyForClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPolicyForClause,
        crate::sql::clauses::policy_for_clause::FormatSqlPolicyForClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::policy_for_clause::FormatSqlPolicyForClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPolicyForClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPolicyForClause,
        crate::sql::clauses::policy_for_clause::FormatSqlPolicyForClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::policy_for_clause::FormatSqlPolicyForClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlPolicyUsingClause>
    for crate::sql::clauses::policy_using_clause::FormatSqlPolicyUsingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPolicyUsingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPolicyUsingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPolicyUsingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPolicyUsingClause,
        crate::sql::clauses::policy_using_clause::FormatSqlPolicyUsingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::policy_using_clause::FormatSqlPolicyUsingClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPolicyUsingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPolicyUsingClause,
        crate::sql::clauses::policy_using_clause::FormatSqlPolicyUsingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::policy_using_clause::FormatSqlPolicyUsingClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlPolicyWithCheckClause>
    for crate::sql::clauses::policy_with_check_clause::FormatSqlPolicyWithCheckClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPolicyWithCheckClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPolicyWithCheckClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPolicyWithCheckClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPolicyWithCheckClause,
        crate::sql::clauses::policy_with_check_clause::FormatSqlPolicyWithCheckClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::policy_with_check_clause::FormatSqlPolicyWithCheckClause::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPolicyWithCheckClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPolicyWithCheckClause,
        crate::sql::clauses::policy_with_check_clause::FormatSqlPolicyWithCheckClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::policy_with_check_clause::FormatSqlPolicyWithCheckClause::default(
            ),
        )
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
impl FormatRule<sql_syntax::SqlReturningClause>
    for crate::sql::clauses::returning_clause::FormatSqlReturningClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlReturningClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturningClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturningClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturningClause,
        crate::sql::clauses::returning_clause::FormatSqlReturningClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::returning_clause::FormatSqlReturningClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturningClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturningClause,
        crate::sql::clauses::returning_clause::FormatSqlReturningClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::returning_clause::FormatSqlReturningClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsClause>
    for crate::sql::clauses::returns_clause::FormatSqlReturnsClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlReturnsClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsClause,
        crate::sql::clauses::returns_clause::FormatSqlReturnsClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::returns_clause::FormatSqlReturnsClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsClause,
        crate::sql::clauses::returns_clause::FormatSqlReturnsClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::returns_clause::FormatSqlReturnsClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsNullOption>
    for crate::sql::auxiliary::returns_null_option::FormatSqlReturnsNullOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReturnsNullOption,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsNullOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsNullOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsNullOption,
        crate::sql::auxiliary::returns_null_option::FormatSqlReturnsNullOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::returns_null_option::FormatSqlReturnsNullOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsNullOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsNullOption,
        crate::sql::auxiliary::returns_null_option::FormatSqlReturnsNullOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::returns_null_option::FormatSqlReturnsNullOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsSetofClause>
    for crate::sql::clauses::returns_setof_clause::FormatSqlReturnsSetofClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReturnsSetofClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsSetofClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsSetofClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsSetofClause,
        crate::sql::clauses::returns_setof_clause::FormatSqlReturnsSetofClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::returns_setof_clause::FormatSqlReturnsSetofClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsSetofClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsSetofClause,
        crate::sql::clauses::returns_setof_clause::FormatSqlReturnsSetofClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::returns_setof_clause::FormatSqlReturnsSetofClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsTableClause>
    for crate::sql::clauses::returns_table_clause::FormatSqlReturnsTableClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReturnsTableClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsTableClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsTableClause,
        crate::sql::clauses::returns_table_clause::FormatSqlReturnsTableClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::returns_table_clause::FormatSqlReturnsTableClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsTableClause,
        crate::sql::clauses::returns_table_clause::FormatSqlReturnsTableClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::returns_table_clause::FormatSqlReturnsTableClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsTableColumn>
    for crate::sql::auxiliary::returns_table_column::FormatSqlReturnsTableColumn
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReturnsTableColumn,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsTableColumn>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableColumn {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsTableColumn,
        crate::sql::auxiliary::returns_table_column::FormatSqlReturnsTableColumn,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::returns_table_column::FormatSqlReturnsTableColumn::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableColumn {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsTableColumn,
        crate::sql::auxiliary::returns_table_column::FormatSqlReturnsTableColumn,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::returns_table_column::FormatSqlReturnsTableColumn::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReturnsTriggerClause>
    for crate::sql::clauses::returns_trigger_clause::FormatSqlReturnsTriggerClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReturnsTriggerClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReturnsTriggerClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsTriggerClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsTriggerClause,
        crate::sql::clauses::returns_trigger_clause::FormatSqlReturnsTriggerClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::returns_trigger_clause::FormatSqlReturnsTriggerClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsTriggerClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsTriggerClause,
        crate::sql::clauses::returns_trigger_clause::FormatSqlReturnsTriggerClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::returns_trigger_clause::FormatSqlReturnsTriggerClause::default(),
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
impl FormatRule<sql_syntax::SqlSecurityOption>
    for crate::sql::auxiliary::security_option::FormatSqlSecurityOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSecurityOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSecurityOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSecurityOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSecurityOption,
        crate::sql::auxiliary::security_option::FormatSqlSecurityOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::security_option::FormatSqlSecurityOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSecurityOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSecurityOption,
        crate::sql::auxiliary::security_option::FormatSqlSecurityOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::security_option::FormatSqlSecurityOption::default(),
        )
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
impl FormatRule<sql_syntax::SqlStrictOption>
    for crate::sql::auxiliary::strict_option::FormatSqlStrictOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlStrictOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlStrictOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlStrictOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlStrictOption,
        crate::sql::auxiliary::strict_option::FormatSqlStrictOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::strict_option::FormatSqlStrictOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlStrictOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlStrictOption,
        crate::sql::auxiliary::strict_option::FormatSqlStrictOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::strict_option::FormatSqlStrictOption::default(),
        )
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
impl FormatRule<sql_syntax::SqlSubstringExpression>
    for crate::sql::expressions::substring_expression::FormatSqlSubstringExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSubstringExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSubstringExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSubstringExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSubstringExpression,
        crate::sql::expressions::substring_expression::FormatSqlSubstringExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::substring_expression::FormatSqlSubstringExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSubstringExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSubstringExpression,
        crate::sql::expressions::substring_expression::FormatSqlSubstringExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::substring_expression::FormatSqlSubstringExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSubstringForClause>
    for crate::sql::clauses::substring_for_clause::FormatSqlSubstringForClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSubstringForClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSubstringForClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSubstringForClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSubstringForClause,
        crate::sql::clauses::substring_for_clause::FormatSqlSubstringForClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::substring_for_clause::FormatSqlSubstringForClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSubstringForClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSubstringForClause,
        crate::sql::clauses::substring_for_clause::FormatSqlSubstringForClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::substring_for_clause::FormatSqlSubstringForClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSubstringFromClause>
    for crate::sql::clauses::substring_from_clause::FormatSqlSubstringFromClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSubstringFromClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSubstringFromClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSubstringFromClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSubstringFromClause,
        crate::sql::clauses::substring_from_clause::FormatSqlSubstringFromClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::substring_from_clause::FormatSqlSubstringFromClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSubstringFromClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSubstringFromClause,
        crate::sql::clauses::substring_from_clause::FormatSqlSubstringFromClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::substring_from_clause::FormatSqlSubstringFromClause::default(),
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
impl FormatRule<sql_syntax::SqlTildeArrayExpression>
    for crate::sql::expressions::tilde_array_expression::FormatSqlTildeArrayExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTildeArrayExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTildeArrayExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTildeArrayExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTildeArrayExpression,
        crate::sql::expressions::tilde_array_expression::FormatSqlTildeArrayExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::tilde_array_expression::FormatSqlTildeArrayExpression::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTildeArrayExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTildeArrayExpression,
        crate::sql::expressions::tilde_array_expression::FormatSqlTildeArrayExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::tilde_array_expression::FormatSqlTildeArrayExpression::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlTildeArraySuffix>
    for crate::sql::auxiliary::tilde_array_suffix::FormatSqlTildeArraySuffix
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTildeArraySuffix,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTildeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTildeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTildeArraySuffix,
        crate::sql::auxiliary::tilde_array_suffix::FormatSqlTildeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::tilde_array_suffix::FormatSqlTildeArraySuffix::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTildeArraySuffix {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTildeArraySuffix,
        crate::sql::auxiliary::tilde_array_suffix::FormatSqlTildeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::tilde_array_suffix::FormatSqlTildeArraySuffix::default(),
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
impl FormatRule<sql_syntax::SqlTriggerEvent>
    for crate::sql::auxiliary::trigger_event::FormatSqlTriggerEvent
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTriggerEvent, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerEvent>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerEvent {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerEvent,
        crate::sql::auxiliary::trigger_event::FormatSqlTriggerEvent,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::trigger_event::FormatSqlTriggerEvent::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerEvent {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerEvent,
        crate::sql::auxiliary::trigger_event::FormatSqlTriggerEvent,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::trigger_event::FormatSqlTriggerEvent::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTriggerForEachClause>
    for crate::sql::clauses::trigger_for_each_clause::FormatSqlTriggerForEachClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTriggerForEachClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerForEachClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerForEachClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerForEachClause,
        crate::sql::clauses::trigger_for_each_clause::FormatSqlTriggerForEachClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::trigger_for_each_clause::FormatSqlTriggerForEachClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerForEachClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerForEachClause,
        crate::sql::clauses::trigger_for_each_clause::FormatSqlTriggerForEachClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::trigger_for_each_clause::FormatSqlTriggerForEachClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlTriggerReferencingClause>
    for crate::sql::clauses::trigger_referencing_clause::FormatSqlTriggerReferencingClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTriggerReferencingClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerReferencingClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerReferencingClause,
        crate::sql::clauses::trigger_referencing_clause::FormatSqlTriggerReferencingClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: clauses :: trigger_referencing_clause :: FormatSqlTriggerReferencingClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerReferencingClause,
        crate::sql::clauses::trigger_referencing_clause::FormatSqlTriggerReferencingClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: clauses :: trigger_referencing_clause :: FormatSqlTriggerReferencingClause :: default ())
    }
}
impl FormatRule<sql_syntax::SqlTriggerReferencingItem>
    for crate::sql::auxiliary::trigger_referencing_item::FormatSqlTriggerReferencingItem
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTriggerReferencingItem,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerReferencingItem>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingItem {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerReferencingItem,
        crate::sql::auxiliary::trigger_referencing_item::FormatSqlTriggerReferencingItem,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: trigger_referencing_item :: FormatSqlTriggerReferencingItem :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingItem {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerReferencingItem,
        crate::sql::auxiliary::trigger_referencing_item::FormatSqlTriggerReferencingItem,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: trigger_referencing_item :: FormatSqlTriggerReferencingItem :: default ())
    }
}
impl FormatRule<sql_syntax::SqlTriggerUpdateOfClause>
    for crate::sql::clauses::trigger_update_of_clause::FormatSqlTriggerUpdateOfClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTriggerUpdateOfClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerUpdateOfClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerUpdateOfClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerUpdateOfClause,
        crate::sql::clauses::trigger_update_of_clause::FormatSqlTriggerUpdateOfClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::trigger_update_of_clause::FormatSqlTriggerUpdateOfClause::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerUpdateOfClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerUpdateOfClause,
        crate::sql::clauses::trigger_update_of_clause::FormatSqlTriggerUpdateOfClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::trigger_update_of_clause::FormatSqlTriggerUpdateOfClause::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlTriggerWhenClause>
    for crate::sql::clauses::trigger_when_clause::FormatSqlTriggerWhenClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTriggerWhenClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTriggerWhenClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerWhenClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerWhenClause,
        crate::sql::clauses::trigger_when_clause::FormatSqlTriggerWhenClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::trigger_when_clause::FormatSqlTriggerWhenClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerWhenClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerWhenClause,
        crate::sql::clauses::trigger_when_clause::FormatSqlTriggerWhenClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::trigger_when_clause::FormatSqlTriggerWhenClause::default(),
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
impl FormatRule<sql_syntax::SqlTypeArraySuffix>
    for crate::sql::auxiliary::type_array_suffix::FormatSqlTypeArraySuffix
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTypeArraySuffix, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTypeArraySuffix>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTypeArraySuffix {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTypeArraySuffix,
        crate::sql::auxiliary::type_array_suffix::FormatSqlTypeArraySuffix,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::type_array_suffix::FormatSqlTypeArraySuffix::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTypeArraySuffix {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTypeArraySuffix,
        crate::sql::auxiliary::type_array_suffix::FormatSqlTypeArraySuffix,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::type_array_suffix::FormatSqlTypeArraySuffix::default(),
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
impl FormatRule<sql_syntax::SqlViewOption>
    for crate::sql::auxiliary::view_option::FormatSqlViewOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlViewOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlViewOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlViewOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlViewOption,
        crate::sql::auxiliary::view_option::FormatSqlViewOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::view_option::FormatSqlViewOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlViewOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlViewOption,
        crate::sql::auxiliary::view_option::FormatSqlViewOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::view_option::FormatSqlViewOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlViewOptions>
    for crate::sql::auxiliary::view_options::FormatSqlViewOptions
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlViewOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlViewOptions>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlViewOptions {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlViewOptions,
        crate::sql::auxiliary::view_options::FormatSqlViewOptions,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::view_options::FormatSqlViewOptions::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlViewOptions {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlViewOptions,
        crate::sql::auxiliary::view_options::FormatSqlViewOptions,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::view_options::FormatSqlViewOptions::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlVolatilityOption>
    for crate::sql::auxiliary::volatility_option::FormatSqlVolatilityOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlVolatilityOption,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlVolatilityOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlVolatilityOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlVolatilityOption,
        crate::sql::auxiliary::volatility_option::FormatSqlVolatilityOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::volatility_option::FormatSqlVolatilityOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlVolatilityOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlVolatilityOption,
        crate::sql::auxiliary::volatility_option::FormatSqlVolatilityOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::volatility_option::FormatSqlVolatilityOption::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFunctionOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFunctionOptionList,
        crate::sql::lists::function_option_list::FormatSqlFunctionOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::function_option_list::FormatSqlFunctionOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFunctionOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFunctionOptionList,
        crate::sql::lists::function_option_list::FormatSqlFunctionOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::function_option_list::FormatSqlFunctionOptionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlFunctionParameterList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlFunctionParameterList,
        crate::sql::lists::function_parameter_list::FormatSqlFunctionParameterList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::function_parameter_list::FormatSqlFunctionParameterList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlFunctionParameterList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlFunctionParameterList,
        crate::sql::lists::function_parameter_list::FormatSqlFunctionParameterList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::function_parameter_list::FormatSqlFunctionParameterList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableColumnList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReturnsTableColumnList,
        crate::sql::lists::returns_table_column_list::FormatSqlReturnsTableColumnList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::returns_table_column_list::FormatSqlReturnsTableColumnList::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReturnsTableColumnList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReturnsTableColumnList,
        crate::sql::lists::returns_table_column_list::FormatSqlReturnsTableColumnList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::returns_table_column_list::FormatSqlReturnsTableColumnList::default(
            ),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerEventList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerEventList,
        crate::sql::lists::trigger_event_list::FormatSqlTriggerEventList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::trigger_event_list::FormatSqlTriggerEventList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerEventList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerEventList,
        crate::sql::lists::trigger_event_list::FormatSqlTriggerEventList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::trigger_event_list::FormatSqlTriggerEventList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingItemList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTriggerReferencingItemList,
        crate::sql::lists::trigger_referencing_item_list::FormatSqlTriggerReferencingItemList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: lists :: trigger_referencing_item_list :: FormatSqlTriggerReferencingItemList :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTriggerReferencingItemList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTriggerReferencingItemList,
        crate::sql::lists::trigger_referencing_item_list::FormatSqlTriggerReferencingItemList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: lists :: trigger_referencing_item_list :: FormatSqlTriggerReferencingItemList :: default ())
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTypeNameList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTypeNameList,
        crate::sql::lists::type_name_list::FormatSqlTypeNameList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::type_name_list::FormatSqlTypeNameList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTypeNameList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTypeNameList,
        crate::sql::lists::type_name_list::FormatSqlTypeNameList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::type_name_list::FormatSqlTypeNameList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlViewOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlViewOptionList,
        crate::sql::lists::view_option_list::FormatSqlViewOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::view_option_list::FormatSqlViewOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlViewOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlViewOptionList,
        crate::sql::lists::view_option_list::FormatSqlViewOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::view_option_list::FormatSqlViewOptionList::default(),
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
