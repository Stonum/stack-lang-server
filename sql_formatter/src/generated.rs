//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

#![allow(clippy::use_self)]
#![expect(clippy::default_constructed_unit_structs)]
use crate::{
    AsFormat, FormatBogusNodeRule, FormatNodeRule, IntoFormat, SqlFormatContext, SqlFormatter,
};
use biome_formatter::{FormatOwnedWithRule, FormatRefWithRule, FormatResult, FormatRule};
impl FormatRule<sql_syntax::PsqlAlterColumnUsing>
    for crate::psql::auxiliary::alter_column_using::FormatPsqlAlterColumnUsing
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlAlterColumnUsing,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlAlterColumnUsing>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlAlterColumnUsing {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlAlterColumnUsing,
        crate::psql::auxiliary::alter_column_using::FormatPsqlAlterColumnUsing,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::alter_column_using::FormatPsqlAlterColumnUsing::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlAlterColumnUsing {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlAlterColumnUsing,
        crate::psql::auxiliary::alter_column_using::FormatPsqlAlterColumnUsing,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::alter_column_using::FormatPsqlAlterColumnUsing::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlAnalyzeStatement>
    for crate::psql::statements::analyze_statement::FormatPsqlAnalyzeStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlAnalyzeStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlAnalyzeStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlAnalyzeStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlAnalyzeStatement,
        crate::psql::statements::analyze_statement::FormatPsqlAnalyzeStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::analyze_statement::FormatPsqlAnalyzeStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlAnalyzeStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlAnalyzeStatement,
        crate::psql::statements::analyze_statement::FormatPsqlAnalyzeStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::analyze_statement::FormatPsqlAnalyzeStatement::default(),
        )
    }
}
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
impl FormatRule<sql_syntax::PsqlAttachPartitionAction>
    for crate::psql::auxiliary::attach_partition_action::FormatPsqlAttachPartitionAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlAttachPartitionAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlAttachPartitionAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlAttachPartitionAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlAttachPartitionAction,
        crate::psql::auxiliary::attach_partition_action::FormatPsqlAttachPartitionAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: attach_partition_action :: FormatPsqlAttachPartitionAction :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlAttachPartitionAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlAttachPartitionAction,
        crate::psql::auxiliary::attach_partition_action::FormatPsqlAttachPartitionAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: attach_partition_action :: FormatPsqlAttachPartitionAction :: default ())
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
impl FormatRule<sql_syntax::PsqlCopyFile>
    for crate::psql::auxiliary::copy_file::FormatPsqlCopyFile
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCopyFile, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyFile>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyFile {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyFile,
        crate::psql::auxiliary::copy_file::FormatPsqlCopyFile,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::copy_file::FormatPsqlCopyFile::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyFile {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyFile,
        crate::psql::auxiliary::copy_file::FormatPsqlCopyFile,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::copy_file::FormatPsqlCopyFile::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCopyOption>
    for crate::psql::auxiliary::copy_option::FormatPsqlCopyOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCopyOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyOption,
        crate::psql::auxiliary::copy_option::FormatPsqlCopyOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::copy_option::FormatPsqlCopyOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyOption,
        crate::psql::auxiliary::copy_option::FormatPsqlCopyOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::copy_option::FormatPsqlCopyOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCopyOptions>
    for crate::psql::auxiliary::copy_options::FormatPsqlCopyOptions
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCopyOptions, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyOptions>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyOptions {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyOptions,
        crate::psql::auxiliary::copy_options::FormatPsqlCopyOptions,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::copy_options::FormatPsqlCopyOptions::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyOptions {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyOptions,
        crate::psql::auxiliary::copy_options::FormatPsqlCopyOptions,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::copy_options::FormatPsqlCopyOptions::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCopyStatement>
    for crate::psql::statements::copy_statement::FormatPsqlCopyStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCopyStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyStatement,
        crate::psql::statements::copy_statement::FormatPsqlCopyStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::copy_statement::FormatPsqlCopyStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyStatement,
        crate::psql::statements::copy_statement::FormatPsqlCopyStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::copy_statement::FormatPsqlCopyStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCopyStdio>
    for crate::psql::auxiliary::copy_stdio::FormatPsqlCopyStdio
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlCopyStdio, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyStdio>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyStdio {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyStdio,
        crate::psql::auxiliary::copy_stdio::FormatPsqlCopyStdio,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::copy_stdio::FormatPsqlCopyStdio::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyStdio {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyStdio,
        crate::psql::auxiliary::copy_stdio::FormatPsqlCopyStdio,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::copy_stdio::FormatPsqlCopyStdio::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCopyTableSource>
    for crate::psql::auxiliary::copy_table_source::FormatPsqlCopyTableSource
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCopyTableSource,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCopyTableSource>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyTableSource {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyTableSource,
        crate::psql::auxiliary::copy_table_source::FormatPsqlCopyTableSource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::copy_table_source::FormatPsqlCopyTableSource::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyTableSource {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyTableSource,
        crate::psql::auxiliary::copy_table_source::FormatPsqlCopyTableSource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::copy_table_source::FormatPsqlCopyTableSource::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlCreateExtensionStatement>
    for crate::psql::statements::create_extension_statement::FormatPsqlCreateExtensionStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlCreateExtensionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlCreateExtensionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreateExtensionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCreateExtensionStatement,
        crate::psql::statements::create_extension_statement::FormatPsqlCreateExtensionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_extension_statement :: FormatPsqlCreateExtensionStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreateExtensionStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCreateExtensionStatement,
        crate::psql::statements::create_extension_statement::FormatPsqlCreateExtensionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_extension_statement :: FormatPsqlCreateExtensionStatement :: default ())
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
impl FormatRule < sql_syntax :: PsqlCreateMaterializedViewStatement > for crate :: psql :: statements :: create_materialized_view_statement :: FormatPsqlCreateMaterializedViewStatement { type Context = SqlFormatContext ; # [inline (always)] fn fmt (& self , node : & sql_syntax :: PsqlCreateMaterializedViewStatement , f : & mut SqlFormatter) -> FormatResult < () > { FormatNodeRule :: < sql_syntax :: PsqlCreateMaterializedViewStatement > :: fmt (self , node , f) } }
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreateMaterializedViewStatement {
    type Format < 'a > = FormatRefWithRule < 'a , sql_syntax :: PsqlCreateMaterializedViewStatement , crate :: psql :: statements :: create_materialized_view_statement :: FormatPsqlCreateMaterializedViewStatement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_materialized_view_statement :: FormatPsqlCreateMaterializedViewStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreateMaterializedViewStatement {
    type Format = FormatOwnedWithRule < sql_syntax :: PsqlCreateMaterializedViewStatement , crate :: psql :: statements :: create_materialized_view_statement :: FormatPsqlCreateMaterializedViewStatement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_materialized_view_statement :: FormatPsqlCreateMaterializedViewStatement :: default ())
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
impl FormatRule < sql_syntax :: PsqlCreateTablePartitionOfStatement > for crate :: psql :: statements :: create_table_partition_of_statement :: FormatPsqlCreateTablePartitionOfStatement { type Context = SqlFormatContext ; # [inline (always)] fn fmt (& self , node : & sql_syntax :: PsqlCreateTablePartitionOfStatement , f : & mut SqlFormatter) -> FormatResult < () > { FormatNodeRule :: < sql_syntax :: PsqlCreateTablePartitionOfStatement > :: fmt (self , node , f) } }
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCreateTablePartitionOfStatement {
    type Format < 'a > = FormatRefWithRule < 'a , sql_syntax :: PsqlCreateTablePartitionOfStatement , crate :: psql :: statements :: create_table_partition_of_statement :: FormatPsqlCreateTablePartitionOfStatement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: create_table_partition_of_statement :: FormatPsqlCreateTablePartitionOfStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCreateTablePartitionOfStatement {
    type Format = FormatOwnedWithRule < sql_syntax :: PsqlCreateTablePartitionOfStatement , crate :: psql :: statements :: create_table_partition_of_statement :: FormatPsqlCreateTablePartitionOfStatement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: create_table_partition_of_statement :: FormatPsqlCreateTablePartitionOfStatement :: default ())
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
impl FormatRule<sql_syntax::PsqlDetachPartitionAction>
    for crate::psql::auxiliary::detach_partition_action::FormatPsqlDetachPartitionAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDetachPartitionAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDetachPartitionAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDetachPartitionAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDetachPartitionAction,
        crate::psql::auxiliary::detach_partition_action::FormatPsqlDetachPartitionAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: detach_partition_action :: FormatPsqlDetachPartitionAction :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDetachPartitionAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDetachPartitionAction,
        crate::psql::auxiliary::detach_partition_action::FormatPsqlDetachPartitionAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: detach_partition_action :: FormatPsqlDetachPartitionAction :: default ())
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
impl FormatRule<sql_syntax::PsqlDoStatement>
    for crate::psql::statements::do_statement::FormatPsqlDoStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlDoStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDoStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDoStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDoStatement,
        crate::psql::statements::do_statement::FormatPsqlDoStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::do_statement::FormatPsqlDoStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDoStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDoStatement,
        crate::psql::statements::do_statement::FormatPsqlDoStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::do_statement::FormatPsqlDoStatement::default(),
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
impl FormatRule<sql_syntax::PsqlDropExtensionStatement>
    for crate::psql::statements::drop_extension_statement::FormatPsqlDropExtensionStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlDropExtensionStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlDropExtensionStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDropExtensionStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlDropExtensionStatement,
        crate::psql::statements::drop_extension_statement::FormatPsqlDropExtensionStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: drop_extension_statement :: FormatPsqlDropExtensionStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDropExtensionStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlDropExtensionStatement,
        crate::psql::statements::drop_extension_statement::FormatPsqlDropExtensionStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: drop_extension_statement :: FormatPsqlDropExtensionStatement :: default ())
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
impl FormatRule < sql_syntax :: PsqlDropMaterializedViewStatement > for crate :: psql :: statements :: drop_materialized_view_statement :: FormatPsqlDropMaterializedViewStatement { type Context = SqlFormatContext ; # [inline (always)] fn fmt (& self , node : & sql_syntax :: PsqlDropMaterializedViewStatement , f : & mut SqlFormatter) -> FormatResult < () > { FormatNodeRule :: < sql_syntax :: PsqlDropMaterializedViewStatement > :: fmt (self , node , f) } }
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlDropMaterializedViewStatement {
    type Format < 'a > = FormatRefWithRule < 'a , sql_syntax :: PsqlDropMaterializedViewStatement , crate :: psql :: statements :: drop_materialized_view_statement :: FormatPsqlDropMaterializedViewStatement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: drop_materialized_view_statement :: FormatPsqlDropMaterializedViewStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlDropMaterializedViewStatement {
    type Format = FormatOwnedWithRule < sql_syntax :: PsqlDropMaterializedViewStatement , crate :: psql :: statements :: drop_materialized_view_statement :: FormatPsqlDropMaterializedViewStatement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: drop_materialized_view_statement :: FormatPsqlDropMaterializedViewStatement :: default ())
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
impl FormatRule<sql_syntax::PsqlExtensionSchemaClause>
    for crate::psql::clauses::extension_schema_clause::FormatPsqlExtensionSchemaClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlExtensionSchemaClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlExtensionSchemaClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlExtensionSchemaClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlExtensionSchemaClause,
        crate::psql::clauses::extension_schema_clause::FormatPsqlExtensionSchemaClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::extension_schema_clause::FormatPsqlExtensionSchemaClause::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlExtensionSchemaClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlExtensionSchemaClause,
        crate::psql::clauses::extension_schema_clause::FormatPsqlExtensionSchemaClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::extension_schema_clause::FormatPsqlExtensionSchemaClause::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::PsqlExtractExpression>
    for crate::psql::expressions::extract_expression::FormatPsqlExtractExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlExtractExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlExtractExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlExtractExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlExtractExpression,
        crate::psql::expressions::extract_expression::FormatPsqlExtractExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::extract_expression::FormatPsqlExtractExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlExtractExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlExtractExpression,
        crate::psql::expressions::extract_expression::FormatPsqlExtractExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::extract_expression::FormatPsqlExtractExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlFieldSelectionExpression>
    for crate::psql::expressions::field_selection_expression::FormatPsqlFieldSelectionExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlFieldSelectionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlFieldSelectionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlFieldSelectionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlFieldSelectionExpression,
        crate::psql::expressions::field_selection_expression::FormatPsqlFieldSelectionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: expressions :: field_selection_expression :: FormatPsqlFieldSelectionExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlFieldSelectionExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlFieldSelectionExpression,
        crate::psql::expressions::field_selection_expression::FormatPsqlFieldSelectionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: expressions :: field_selection_expression :: FormatPsqlFieldSelectionExpression :: default ())
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
impl FormatRule<sql_syntax::PsqlGeneratedColumnConstraint>
    for crate::psql::auxiliary::generated_column_constraint::FormatPsqlGeneratedColumnConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlGeneratedColumnConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlGeneratedColumnConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlGeneratedColumnConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlGeneratedColumnConstraint,
        crate::psql::auxiliary::generated_column_constraint::FormatPsqlGeneratedColumnConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: generated_column_constraint :: FormatPsqlGeneratedColumnConstraint :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlGeneratedColumnConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlGeneratedColumnConstraint,
        crate::psql::auxiliary::generated_column_constraint::FormatPsqlGeneratedColumnConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: generated_column_constraint :: FormatPsqlGeneratedColumnConstraint :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlIdentityConstraint>
    for crate::psql::auxiliary::identity_constraint::FormatPsqlIdentityConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlIdentityConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlIdentityConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlIdentityConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlIdentityConstraint,
        crate::psql::auxiliary::identity_constraint::FormatPsqlIdentityConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::identity_constraint::FormatPsqlIdentityConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlIdentityConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlIdentityConstraint,
        crate::psql::auxiliary::identity_constraint::FormatPsqlIdentityConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::identity_constraint::FormatPsqlIdentityConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlIndexMethodClause>
    for crate::psql::clauses::index_method_clause::FormatPsqlIndexMethodClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlIndexMethodClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlIndexMethodClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlIndexMethodClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlIndexMethodClause,
        crate::psql::clauses::index_method_clause::FormatPsqlIndexMethodClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::index_method_clause::FormatPsqlIndexMethodClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlIndexMethodClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlIndexMethodClause,
        crate::psql::clauses::index_method_clause::FormatPsqlIndexMethodClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::index_method_clause::FormatPsqlIndexMethodClause::default(),
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
impl FormatRule<sql_syntax::PsqlLockMode>
    for crate::psql::auxiliary::lock_mode::FormatPsqlLockMode
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlLockMode, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlLockMode>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlLockMode {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlLockMode,
        crate::psql::auxiliary::lock_mode::FormatPsqlLockMode,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::lock_mode::FormatPsqlLockMode::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlLockMode {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlLockMode,
        crate::psql::auxiliary::lock_mode::FormatPsqlLockMode,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::lock_mode::FormatPsqlLockMode::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlLockStatement>
    for crate::psql::statements::lock_statement::FormatPsqlLockStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlLockStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlLockStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlLockStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlLockStatement,
        crate::psql::statements::lock_statement::FormatPsqlLockStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::lock_statement::FormatPsqlLockStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlLockStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlLockStatement,
        crate::psql::statements::lock_statement::FormatPsqlLockStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::lock_statement::FormatPsqlLockStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlNamedArgument>
    for crate::psql::auxiliary::named_argument::FormatPsqlNamedArgument
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlNamedArgument, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlNamedArgument>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlNamedArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlNamedArgument,
        crate::psql::auxiliary::named_argument::FormatPsqlNamedArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::named_argument::FormatPsqlNamedArgument::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlNamedArgument {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlNamedArgument,
        crate::psql::auxiliary::named_argument::FormatPsqlNamedArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::named_argument::FormatPsqlNamedArgument::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlOnCommitClause>
    for crate::psql::clauses::on_commit_clause::FormatPsqlOnCommitClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlOnCommitClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlOnCommitClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlOnCommitClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlOnCommitClause,
        crate::psql::clauses::on_commit_clause::FormatPsqlOnCommitClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::on_commit_clause::FormatPsqlOnCommitClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlOnCommitClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlOnCommitClause,
        crate::psql::clauses::on_commit_clause::FormatPsqlOnCommitClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::on_commit_clause::FormatPsqlOnCommitClause::default(),
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
impl FormatRule<sql_syntax::PsqlPartitionByClause>
    for crate::psql::clauses::partition_by_clause::FormatPsqlPartitionByClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionByClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionByClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionByClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionByClause,
        crate::psql::clauses::partition_by_clause::FormatPsqlPartitionByClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::partition_by_clause::FormatPsqlPartitionByClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionByClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionByClause,
        crate::psql::clauses::partition_by_clause::FormatPsqlPartitionByClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::partition_by_clause::FormatPsqlPartitionByClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPartitionDefaultBound>
    for crate::psql::auxiliary::partition_default_bound::FormatPsqlPartitionDefaultBound
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionDefaultBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionDefaultBound>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionDefaultBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionDefaultBound,
        crate::psql::auxiliary::partition_default_bound::FormatPsqlPartitionDefaultBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: partition_default_bound :: FormatPsqlPartitionDefaultBound :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionDefaultBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionDefaultBound,
        crate::psql::auxiliary::partition_default_bound::FormatPsqlPartitionDefaultBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: partition_default_bound :: FormatPsqlPartitionDefaultBound :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlPartitionForValuesBound>
    for crate::psql::auxiliary::partition_for_values_bound::FormatPsqlPartitionForValuesBound
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionForValuesBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionForValuesBound>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionForValuesBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionForValuesBound,
        crate::psql::auxiliary::partition_for_values_bound::FormatPsqlPartitionForValuesBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: partition_for_values_bound :: FormatPsqlPartitionForValuesBound :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionForValuesBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionForValuesBound,
        crate::psql::auxiliary::partition_for_values_bound::FormatPsqlPartitionForValuesBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: partition_for_values_bound :: FormatPsqlPartitionForValuesBound :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlPartitionHashValues>
    for crate::psql::auxiliary::partition_hash_values::FormatPsqlPartitionHashValues
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionHashValues,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionHashValues>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionHashValues {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionHashValues,
        crate::psql::auxiliary::partition_hash_values::FormatPsqlPartitionHashValues,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::partition_hash_values::FormatPsqlPartitionHashValues::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionHashValues {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionHashValues,
        crate::psql::auxiliary::partition_hash_values::FormatPsqlPartitionHashValues,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::partition_hash_values::FormatPsqlPartitionHashValues::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPartitionInValues>
    for crate::psql::auxiliary::partition_in_values::FormatPsqlPartitionInValues
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionInValues,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionInValues>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionInValues {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionInValues,
        crate::psql::auxiliary::partition_in_values::FormatPsqlPartitionInValues,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::partition_in_values::FormatPsqlPartitionInValues::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionInValues {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionInValues,
        crate::psql::auxiliary::partition_in_values::FormatPsqlPartitionInValues,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::partition_in_values::FormatPsqlPartitionInValues::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPartitionRangeValues>
    for crate::psql::auxiliary::partition_range_values::FormatPsqlPartitionRangeValues
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionRangeValues,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionRangeValues>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionRangeValues {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionRangeValues,
        crate::psql::auxiliary::partition_range_values::FormatPsqlPartitionRangeValues,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::partition_range_values::FormatPsqlPartitionRangeValues::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionRangeValues {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionRangeValues,
        crate::psql::auxiliary::partition_range_values::FormatPsqlPartitionRangeValues,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::partition_range_values::FormatPsqlPartitionRangeValues::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPartitionValueList>
    for crate::psql::auxiliary::partition_value_list::FormatPsqlPartitionValueList
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPartitionValueList,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPartitionValueList>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPartitionValueList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPartitionValueList,
        crate::psql::auxiliary::partition_value_list::FormatPsqlPartitionValueList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::partition_value_list::FormatPsqlPartitionValueList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPartitionValueList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPartitionValueList,
        crate::psql::auxiliary::partition_value_list::FormatPsqlPartitionValueList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::partition_value_list::FormatPsqlPartitionValueList::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPlBlock> for crate::psql::auxiliary::pl_block::FormatPsqlPlBlock {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlPlBlock, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPlBlock>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlBlock {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlBlock,
        crate::psql::auxiliary::pl_block::FormatPsqlPlBlock,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::pl_block::FormatPsqlPlBlock::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlBlock {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlBlock,
        crate::psql::auxiliary::pl_block::FormatPsqlPlBlock,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::pl_block::FormatPsqlPlBlock::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPlBody> for crate::psql::auxiliary::pl_body::FormatPsqlPlBody {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlPlBody, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPlBody>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlBody,
        crate::psql::auxiliary::pl_body::FormatPsqlPlBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::pl_body::FormatPsqlPlBody::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlBody {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlBody,
        crate::psql::auxiliary::pl_body::FormatPsqlPlBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::pl_body::FormatPsqlPlBody::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPlDeclareSection>
    for crate::psql::auxiliary::pl_declare_section::FormatPsqlPlDeclareSection
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPlDeclareSection,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPlDeclareSection>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlDeclareSection {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlDeclareSection,
        crate::psql::auxiliary::pl_declare_section::FormatPsqlPlDeclareSection,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::pl_declare_section::FormatPsqlPlDeclareSection::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlDeclareSection {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlDeclareSection,
        crate::psql::auxiliary::pl_declare_section::FormatPsqlPlDeclareSection,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::pl_declare_section::FormatPsqlPlDeclareSection::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlPlVariableDeclaration>
    for crate::psql::auxiliary::pl_variable_declaration::FormatPsqlPlVariableDeclaration
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPlVariableDeclaration,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPlVariableDeclaration>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlVariableDeclaration {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlVariableDeclaration,
        crate::psql::auxiliary::pl_variable_declaration::FormatPsqlPlVariableDeclaration,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: auxiliary :: pl_variable_declaration :: FormatPsqlPlVariableDeclaration :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlVariableDeclaration {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlVariableDeclaration,
        crate::psql::auxiliary::pl_variable_declaration::FormatPsqlPlVariableDeclaration,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: auxiliary :: pl_variable_declaration :: FormatPsqlPlVariableDeclaration :: default ())
    }
}
impl FormatRule<sql_syntax::PsqlPlVariableDefault>
    for crate::psql::auxiliary::pl_variable_default::FormatPsqlPlVariableDefault
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPlVariableDefault,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPlVariableDefault>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlVariableDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlVariableDefault,
        crate::psql::auxiliary::pl_variable_default::FormatPsqlPlVariableDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::pl_variable_default::FormatPsqlPlVariableDefault::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlVariableDefault {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlVariableDefault,
        crate::psql::auxiliary::pl_variable_default::FormatPsqlPlVariableDefault,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::pl_variable_default::FormatPsqlPlVariableDefault::default(),
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
impl FormatRule<sql_syntax::PsqlPositionExpression>
    for crate::psql::expressions::position_expression::FormatPsqlPositionExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlPositionExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlPositionExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPositionExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPositionExpression,
        crate::psql::expressions::position_expression::FormatPsqlPositionExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::position_expression::FormatPsqlPositionExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPositionExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPositionExpression,
        crate::psql::expressions::position_expression::FormatPsqlPositionExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::position_expression::FormatPsqlPositionExpression::default(),
        )
    }
}
impl FormatRule < sql_syntax :: PsqlRefreshMaterializedViewStatement > for crate :: psql :: statements :: refresh_materialized_view_statement :: FormatPsqlRefreshMaterializedViewStatement { type Context = SqlFormatContext ; # [inline (always)] fn fmt (& self , node : & sql_syntax :: PsqlRefreshMaterializedViewStatement , f : & mut SqlFormatter) -> FormatResult < () > { FormatNodeRule :: < sql_syntax :: PsqlRefreshMaterializedViewStatement > :: fmt (self , node , f) } }
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlRefreshMaterializedViewStatement {
    type Format < 'a > = FormatRefWithRule < 'a , sql_syntax :: PsqlRefreshMaterializedViewStatement , crate :: psql :: statements :: refresh_materialized_view_statement :: FormatPsqlRefreshMaterializedViewStatement > ;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: statements :: refresh_materialized_view_statement :: FormatPsqlRefreshMaterializedViewStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlRefreshMaterializedViewStatement {
    type Format = FormatOwnedWithRule < sql_syntax :: PsqlRefreshMaterializedViewStatement , crate :: psql :: statements :: refresh_materialized_view_statement :: FormatPsqlRefreshMaterializedViewStatement > ;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: statements :: refresh_materialized_view_statement :: FormatPsqlRefreshMaterializedViewStatement :: default ())
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
impl FormatRule<sql_syntax::PsqlSequenceOption>
    for crate::psql::auxiliary::sequence_option::FormatPsqlSequenceOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlSequenceOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSequenceOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSequenceOption,
        crate::psql::auxiliary::sequence_option::FormatPsqlSequenceOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::sequence_option::FormatPsqlSequenceOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSequenceOption,
        crate::psql::auxiliary::sequence_option::FormatPsqlSequenceOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::sequence_option::FormatPsqlSequenceOption::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlSequenceOptions>
    for crate::psql::auxiliary::sequence_options::FormatPsqlSequenceOptions
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlSequenceOptions,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlSequenceOptions>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOptions {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSequenceOptions,
        crate::psql::auxiliary::sequence_options::FormatPsqlSequenceOptions,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::sequence_options::FormatPsqlSequenceOptions::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOptions {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSequenceOptions,
        crate::psql::auxiliary::sequence_options::FormatPsqlSequenceOptions,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::sequence_options::FormatPsqlSequenceOptions::default(),
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
impl FormatRule<sql_syntax::PsqlTableAndColumns>
    for crate::psql::auxiliary::table_and_columns::FormatPsqlTableAndColumns
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTableAndColumns,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTableAndColumns>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTableAndColumns {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTableAndColumns,
        crate::psql::auxiliary::table_and_columns::FormatPsqlTableAndColumns,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::table_and_columns::FormatPsqlTableAndColumns::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTableAndColumns {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTableAndColumns,
        crate::psql::auxiliary::table_and_columns::FormatPsqlTableAndColumns,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::table_and_columns::FormatPsqlTableAndColumns::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTableLikeElement>
    for crate::psql::auxiliary::table_like_element::FormatPsqlTableLikeElement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTableLikeElement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTableLikeElement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTableLikeElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTableLikeElement,
        crate::psql::auxiliary::table_like_element::FormatPsqlTableLikeElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::table_like_element::FormatPsqlTableLikeElement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTableLikeElement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTableLikeElement,
        crate::psql::auxiliary::table_like_element::FormatPsqlTableLikeElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::table_like_element::FormatPsqlTableLikeElement::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTableQuery>
    for crate::psql::auxiliary::table_query::FormatPsqlTableQuery
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlTableQuery, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTableQuery>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTableQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTableQuery,
        crate::psql::auxiliary::table_query::FormatPsqlTableQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::auxiliary::table_query::FormatPsqlTableQuery::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTableQuery {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTableQuery,
        crate::psql::auxiliary::table_query::FormatPsqlTableQuery,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::auxiliary::table_query::FormatPsqlTableQuery::default(),
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
impl FormatRule<sql_syntax::PsqlTrimExpression>
    for crate::psql::expressions::trim_expression::FormatPsqlTrimExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlTrimExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTrimExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTrimExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTrimExpression,
        crate::psql::expressions::trim_expression::FormatPsqlTrimExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::expressions::trim_expression::FormatPsqlTrimExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTrimExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTrimExpression,
        crate::psql::expressions::trim_expression::FormatPsqlTrimExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::expressions::trim_expression::FormatPsqlTrimExpression::default(),
        )
    }
}
impl FormatRule<sql_syntax::PsqlTruncateIdentityClause>
    for crate::psql::clauses::truncate_identity_clause::FormatPsqlTruncateIdentityClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlTruncateIdentityClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlTruncateIdentityClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTruncateIdentityClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTruncateIdentityClause,
        crate::psql::clauses::truncate_identity_clause::FormatPsqlTruncateIdentityClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: psql :: clauses :: truncate_identity_clause :: FormatPsqlTruncateIdentityClause :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTruncateIdentityClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTruncateIdentityClause,
        crate::psql::clauses::truncate_identity_clause::FormatPsqlTruncateIdentityClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: psql :: clauses :: truncate_identity_clause :: FormatPsqlTruncateIdentityClause :: default ())
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
impl FormatRule<sql_syntax::PsqlVacuumStatement>
    for crate::psql::statements::vacuum_statement::FormatPsqlVacuumStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::PsqlVacuumStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlVacuumStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlVacuumStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlVacuumStatement,
        crate::psql::statements::vacuum_statement::FormatPsqlVacuumStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::statements::vacuum_statement::FormatPsqlVacuumStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlVacuumStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlVacuumStatement,
        crate::psql::statements::vacuum_statement::FormatPsqlVacuumStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::statements::vacuum_statement::FormatPsqlVacuumStatement::default(),
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
impl FormatRule<sql_syntax::PsqlWithDataClause>
    for crate::psql::clauses::with_data_clause::FormatPsqlWithDataClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::PsqlWithDataClause, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::PsqlWithDataClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlWithDataClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlWithDataClause,
        crate::psql::clauses::with_data_clause::FormatPsqlWithDataClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::clauses::with_data_clause::FormatPsqlWithDataClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlWithDataClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlWithDataClause,
        crate::psql::clauses::with_data_clause::FormatPsqlWithDataClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::clauses::with_data_clause::FormatPsqlWithDataClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAddColumnAction>
    for crate::sql::auxiliary::add_column_action::FormatSqlAddColumnAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlAddColumnAction, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAddColumnAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAddColumnAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAddColumnAction,
        crate::sql::auxiliary::add_column_action::FormatSqlAddColumnAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::add_column_action::FormatSqlAddColumnAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAddColumnAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAddColumnAction,
        crate::sql::auxiliary::add_column_action::FormatSqlAddColumnAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::add_column_action::FormatSqlAddColumnAction::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAddConstraintAction>
    for crate::sql::auxiliary::add_constraint_action::FormatSqlAddConstraintAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAddConstraintAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAddConstraintAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAddConstraintAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAddConstraintAction,
        crate::sql::auxiliary::add_constraint_action::FormatSqlAddConstraintAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::add_constraint_action::FormatSqlAddConstraintAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAddConstraintAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAddConstraintAction,
        crate::sql::auxiliary::add_constraint_action::FormatSqlAddConstraintAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::add_constraint_action::FormatSqlAddConstraintAction::default(),
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
impl FormatRule<sql_syntax::SqlAllTriggersTarget>
    for crate::sql::auxiliary::all_triggers_target::FormatSqlAllTriggersTarget
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAllTriggersTarget,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAllTriggersTarget>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAllTriggersTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAllTriggersTarget,
        crate::sql::auxiliary::all_triggers_target::FormatSqlAllTriggersTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::all_triggers_target::FormatSqlAllTriggersTarget::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAllTriggersTarget {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAllTriggersTarget,
        crate::sql::auxiliary::all_triggers_target::FormatSqlAllTriggersTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::all_triggers_target::FormatSqlAllTriggersTarget::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlterColumnAction>
    for crate::sql::auxiliary::alter_column_action::FormatSqlAlterColumnAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAlterColumnAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlterColumnAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterColumnAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterColumnAction,
        crate::sql::auxiliary::alter_column_action::FormatSqlAlterColumnAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alter_column_action::FormatSqlAlterColumnAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterColumnAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterColumnAction,
        crate::sql::auxiliary::alter_column_action::FormatSqlAlterColumnAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alter_column_action::FormatSqlAlterColumnAction::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlterColumnType>
    for crate::sql::auxiliary::alter_column_type::FormatSqlAlterColumnType
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlAlterColumnType, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlterColumnType>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterColumnType {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterColumnType,
        crate::sql::auxiliary::alter_column_type::FormatSqlAlterColumnType,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alter_column_type::FormatSqlAlterColumnType::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterColumnType {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterColumnType,
        crate::sql::auxiliary::alter_column_type::FormatSqlAlterColumnType,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alter_column_type::FormatSqlAlterColumnType::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlterIndexStatement>
    for crate::sql::statements::alter_index_statement::FormatSqlAlterIndexStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAlterIndexStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlterIndexStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterIndexStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterIndexStatement,
        crate::sql::statements::alter_index_statement::FormatSqlAlterIndexStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::alter_index_statement::FormatSqlAlterIndexStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterIndexStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterIndexStatement,
        crate::sql::statements::alter_index_statement::FormatSqlAlterIndexStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::alter_index_statement::FormatSqlAlterIndexStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlterTableStatement>
    for crate::sql::statements::alter_table_statement::FormatSqlAlterTableStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAlterTableStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlterTableStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterTableStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterTableStatement,
        crate::sql::statements::alter_table_statement::FormatSqlAlterTableStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::alter_table_statement::FormatSqlAlterTableStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterTableStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterTableStatement,
        crate::sql::statements::alter_table_statement::FormatSqlAlterTableStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::alter_table_statement::FormatSqlAlterTableStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlAlterTriggerAction>
    for crate::sql::auxiliary::alter_trigger_action::FormatSqlAlterTriggerAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlAlterTriggerAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlAlterTriggerAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterTriggerAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterTriggerAction,
        crate::sql::auxiliary::alter_trigger_action::FormatSqlAlterTriggerAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::alter_trigger_action::FormatSqlAlterTriggerAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterTriggerAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterTriggerAction,
        crate::sql::auxiliary::alter_trigger_action::FormatSqlAlterTriggerAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::alter_trigger_action::FormatSqlAlterTriggerAction::default(),
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
impl FormatRule<sql_syntax::SqlBeginStatement>
    for crate::sql::statements::begin_statement::FormatSqlBeginStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlBeginStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlBeginStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlBeginStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlBeginStatement,
        crate::sql::statements::begin_statement::FormatSqlBeginStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::begin_statement::FormatSqlBeginStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlBeginStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlBeginStatement,
        crate::sql::statements::begin_statement::FormatSqlBeginStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::begin_statement::FormatSqlBeginStatement::default(),
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
impl FormatRule<sql_syntax::SqlCallStatement>
    for crate::sql::statements::call_statement::FormatSqlCallStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCallStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCallStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCallStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCallStatement,
        crate::sql::statements::call_statement::FormatSqlCallStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::call_statement::FormatSqlCallStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCallStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCallStatement,
        crate::sql::statements::call_statement::FormatSqlCallStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::call_statement::FormatSqlCallStatement::default(),
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
impl FormatRule<sql_syntax::SqlCheckConstraint>
    for crate::sql::auxiliary::check_constraint::FormatSqlCheckConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCheckConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCheckConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCheckConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCheckConstraint,
        crate::sql::auxiliary::check_constraint::FormatSqlCheckConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::check_constraint::FormatSqlCheckConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCheckConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCheckConstraint,
        crate::sql::auxiliary::check_constraint::FormatSqlCheckConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::check_constraint::FormatSqlCheckConstraint::default(),
        )
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
impl FormatRule<sql_syntax::SqlCollateConstraint>
    for crate::sql::auxiliary::collate_constraint::FormatSqlCollateConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCollateConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCollateConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCollateConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCollateConstraint,
        crate::sql::auxiliary::collate_constraint::FormatSqlCollateConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::collate_constraint::FormatSqlCollateConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCollateConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCollateConstraint,
        crate::sql::auxiliary::collate_constraint::FormatSqlCollateConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::collate_constraint::FormatSqlCollateConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCollateExpression>
    for crate::sql::expressions::collate_expression::FormatSqlCollateExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCollateExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCollateExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCollateExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCollateExpression,
        crate::sql::expressions::collate_expression::FormatSqlCollateExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::collate_expression::FormatSqlCollateExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCollateExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCollateExpression,
        crate::sql::expressions::collate_expression::FormatSqlCollateExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::collate_expression::FormatSqlCollateExpression::default(),
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
impl FormatRule<sql_syntax::SqlCommitStatement>
    for crate::sql::statements::commit_statement::FormatSqlCommitStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlCommitStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCommitStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCommitStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCommitStatement,
        crate::sql::statements::commit_statement::FormatSqlCommitStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::commit_statement::FormatSqlCommitStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCommitStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCommitStatement,
        crate::sql::statements::commit_statement::FormatSqlCommitStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::commit_statement::FormatSqlCommitStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCompoundSelectStatement>
    for crate::sql::statements::compound_select_statement::FormatSqlCompoundSelectStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCompoundSelectStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCompoundSelectStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCompoundSelectStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCompoundSelectStatement,
        crate::sql::statements::compound_select_statement::FormatSqlCompoundSelectStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: compound_select_statement :: FormatSqlCompoundSelectStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCompoundSelectStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCompoundSelectStatement,
        crate::sql::statements::compound_select_statement::FormatSqlCompoundSelectStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: compound_select_statement :: FormatSqlCompoundSelectStatement :: default ())
    }
}
impl FormatRule<sql_syntax::SqlConstraintName>
    for crate::sql::auxiliary::constraint_name::FormatSqlConstraintName
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlConstraintName, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlConstraintName>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlConstraintName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlConstraintName,
        crate::sql::auxiliary::constraint_name::FormatSqlConstraintName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::constraint_name::FormatSqlConstraintName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlConstraintName {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlConstraintName,
        crate::sql::auxiliary::constraint_name::FormatSqlConstraintName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::constraint_name::FormatSqlConstraintName::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlCreateIndexStatement>
    for crate::sql::statements::create_index_statement::FormatSqlCreateIndexStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateIndexStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateIndexStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateIndexStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateIndexStatement,
        crate::sql::statements::create_index_statement::FormatSqlCreateIndexStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::create_index_statement::FormatSqlCreateIndexStatement::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateIndexStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateIndexStatement,
        crate::sql::statements::create_index_statement::FormatSqlCreateIndexStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::create_index_statement::FormatSqlCreateIndexStatement::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlCreateTableAsStatement>
    for crate::sql::statements::create_table_as_statement::FormatSqlCreateTableAsStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlCreateTableAsStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlCreateTableAsStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCreateTableAsStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCreateTableAsStatement,
        crate::sql::statements::create_table_as_statement::FormatSqlCreateTableAsStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: statements :: create_table_as_statement :: FormatSqlCreateTableAsStatement :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCreateTableAsStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCreateTableAsStatement,
        crate::sql::statements::create_table_as_statement::FormatSqlCreateTableAsStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: statements :: create_table_as_statement :: FormatSqlCreateTableAsStatement :: default ())
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
impl FormatRule<sql_syntax::SqlDefaultConstraint>
    for crate::sql::auxiliary::default_constraint::FormatSqlDefaultConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDefaultConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDefaultConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDefaultConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDefaultConstraint,
        crate::sql::auxiliary::default_constraint::FormatSqlDefaultConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::default_constraint::FormatSqlDefaultConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDefaultConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDefaultConstraint,
        crate::sql::auxiliary::default_constraint::FormatSqlDefaultConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::default_constraint::FormatSqlDefaultConstraint::default(),
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
impl FormatRule<sql_syntax::SqlDropColumnAction>
    for crate::sql::auxiliary::drop_column_action::FormatSqlDropColumnAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropColumnAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropColumnAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropColumnAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropColumnAction,
        crate::sql::auxiliary::drop_column_action::FormatSqlDropColumnAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_action::FormatSqlDropColumnAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropColumnAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropColumnAction,
        crate::sql::auxiliary::drop_column_action::FormatSqlDropColumnAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_action::FormatSqlDropColumnAction::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropColumnDefault>
    for crate::sql::auxiliary::drop_column_default::FormatSqlDropColumnDefault
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropColumnDefault,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropColumnDefault>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropColumnDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropColumnDefault,
        crate::sql::auxiliary::drop_column_default::FormatSqlDropColumnDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_default::FormatSqlDropColumnDefault::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropColumnDefault {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropColumnDefault,
        crate::sql::auxiliary::drop_column_default::FormatSqlDropColumnDefault,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_default::FormatSqlDropColumnDefault::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropColumnNotNull>
    for crate::sql::auxiliary::drop_column_not_null::FormatSqlDropColumnNotNull
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropColumnNotNull,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropColumnNotNull>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropColumnNotNull {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropColumnNotNull,
        crate::sql::auxiliary::drop_column_not_null::FormatSqlDropColumnNotNull,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_not_null::FormatSqlDropColumnNotNull::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropColumnNotNull {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropColumnNotNull,
        crate::sql::auxiliary::drop_column_not_null::FormatSqlDropColumnNotNull,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::drop_column_not_null::FormatSqlDropColumnNotNull::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlDropConstraintAction>
    for crate::sql::auxiliary::drop_constraint_action::FormatSqlDropConstraintAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropConstraintAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropConstraintAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropConstraintAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropConstraintAction,
        crate::sql::auxiliary::drop_constraint_action::FormatSqlDropConstraintAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::drop_constraint_action::FormatSqlDropConstraintAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropConstraintAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropConstraintAction,
        crate::sql::auxiliary::drop_constraint_action::FormatSqlDropConstraintAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::drop_constraint_action::FormatSqlDropConstraintAction::default(),
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
impl FormatRule<sql_syntax::SqlDropIndexStatement>
    for crate::sql::statements::drop_index_statement::FormatSqlDropIndexStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlDropIndexStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlDropIndexStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlDropIndexStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlDropIndexStatement,
        crate::sql::statements::drop_index_statement::FormatSqlDropIndexStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::drop_index_statement::FormatSqlDropIndexStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlDropIndexStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlDropIndexStatement,
        crate::sql::statements::drop_index_statement::FormatSqlDropIndexStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::drop_index_statement::FormatSqlDropIndexStatement::default(),
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
impl FormatRule<sql_syntax::SqlForeignKeyTableConstraint>
    for crate::sql::auxiliary::foreign_key_table_constraint::FormatSqlForeignKeyTableConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlForeignKeyTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlForeignKeyTableConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlForeignKeyTableConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlForeignKeyTableConstraint,
        crate::sql::auxiliary::foreign_key_table_constraint::FormatSqlForeignKeyTableConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: foreign_key_table_constraint :: FormatSqlForeignKeyTableConstraint :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlForeignKeyTableConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlForeignKeyTableConstraint,
        crate::sql::auxiliary::foreign_key_table_constraint::FormatSqlForeignKeyTableConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: foreign_key_table_constraint :: FormatSqlForeignKeyTableConstraint :: default ())
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
impl FormatRule<sql_syntax::SqlGoStatement>
    for crate::sql::statements::go_statement::FormatSqlGoStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlGoStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlGoStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGoStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGoStatement,
        crate::sql::statements::go_statement::FormatSqlGoStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::go_statement::FormatSqlGoStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGoStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGoStatement,
        crate::sql::statements::go_statement::FormatSqlGoStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::go_statement::FormatSqlGoStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlGrantOptionFor>
    for crate::sql::auxiliary::grant_option_for::FormatSqlGrantOptionFor
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlGrantOptionFor, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlGrantOptionFor>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlGrantOptionFor {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlGrantOptionFor,
        crate::sql::auxiliary::grant_option_for::FormatSqlGrantOptionFor,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::grant_option_for::FormatSqlGrantOptionFor::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlGrantOptionFor {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlGrantOptionFor,
        crate::sql::auxiliary::grant_option_for::FormatSqlGrantOptionFor,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::grant_option_for::FormatSqlGrantOptionFor::default(),
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
impl FormatRule<sql_syntax::SqlIndexIncludeClause>
    for crate::sql::clauses::index_include_clause::FormatSqlIndexIncludeClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlIndexIncludeClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlIndexIncludeClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIndexIncludeClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIndexIncludeClause,
        crate::sql::clauses::index_include_clause::FormatSqlIndexIncludeClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::index_include_clause::FormatSqlIndexIncludeClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIndexIncludeClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIndexIncludeClause,
        crate::sql::clauses::index_include_clause::FormatSqlIndexIncludeClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::index_include_clause::FormatSqlIndexIncludeClause::default(),
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
impl FormatRule<sql_syntax::SqlIsDistinctFromExpression>
    for crate::sql::expressions::is_distinct_from_expression::FormatSqlIsDistinctFromExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlIsDistinctFromExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlIsDistinctFromExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIsDistinctFromExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIsDistinctFromExpression,
        crate::sql::expressions::is_distinct_from_expression::FormatSqlIsDistinctFromExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: is_distinct_from_expression :: FormatSqlIsDistinctFromExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIsDistinctFromExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIsDistinctFromExpression,
        crate::sql::expressions::is_distinct_from_expression::FormatSqlIsDistinctFromExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: is_distinct_from_expression :: FormatSqlIsDistinctFromExpression :: default ())
    }
}
impl FormatRule<sql_syntax::SqlIsExpression>
    for crate::sql::expressions::is_expression::FormatSqlIsExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlIsExpression, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlIsExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIsExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIsExpression,
        crate::sql::expressions::is_expression::FormatSqlIsExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::expressions::is_expression::FormatSqlIsExpression::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIsExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIsExpression,
        crate::sql::expressions::is_expression::FormatSqlIsExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::expressions::is_expression::FormatSqlIsExpression::default(),
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
impl FormatRule<sql_syntax::SqlNotNullConstraint>
    for crate::sql::auxiliary::not_null_constraint::FormatSqlNotNullConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlNotNullConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlNotNullConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlNotNullConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlNotNullConstraint,
        crate::sql::auxiliary::not_null_constraint::FormatSqlNotNullConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::not_null_constraint::FormatSqlNotNullConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlNotNullConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlNotNullConstraint,
        crate::sql::auxiliary::not_null_constraint::FormatSqlNotNullConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::not_null_constraint::FormatSqlNotNullConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlNullConstraint>
    for crate::sql::auxiliary::null_constraint::FormatSqlNullConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlNullConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlNullConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlNullConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlNullConstraint,
        crate::sql::auxiliary::null_constraint::FormatSqlNullConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::null_constraint::FormatSqlNullConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlNullConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlNullConstraint,
        crate::sql::auxiliary::null_constraint::FormatSqlNullConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::null_constraint::FormatSqlNullConstraint::default(),
        )
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
impl FormatRule<sql_syntax::SqlNullsOrder>
    for crate::sql::auxiliary::nulls_order::FormatSqlNullsOrder
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlNullsOrder, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlNullsOrder>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlNullsOrder {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlNullsOrder,
        crate::sql::auxiliary::nulls_order::FormatSqlNullsOrder,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::nulls_order::FormatSqlNullsOrder::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlNullsOrder {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlNullsOrder,
        crate::sql::auxiliary::nulls_order::FormatSqlNullsOrder,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::nulls_order::FormatSqlNullsOrder::default(),
        )
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
impl FormatRule<sql_syntax::SqlPostfixNullExpression>
    for crate::sql::expressions::postfix_null_expression::FormatSqlPostfixNullExpression
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPostfixNullExpression,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPostfixNullExpression>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPostfixNullExpression {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPostfixNullExpression,
        crate::sql::expressions::postfix_null_expression::FormatSqlPostfixNullExpression,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: expressions :: postfix_null_expression :: FormatSqlPostfixNullExpression :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPostfixNullExpression {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPostfixNullExpression,
        crate::sql::expressions::postfix_null_expression::FormatSqlPostfixNullExpression,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: expressions :: postfix_null_expression :: FormatSqlPostfixNullExpression :: default ())
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
impl FormatRule<sql_syntax::SqlPrimaryKeyConstraint>
    for crate::sql::auxiliary::primary_key_constraint::FormatSqlPrimaryKeyConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPrimaryKeyConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrimaryKeyConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrimaryKeyConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrimaryKeyConstraint,
        crate::sql::auxiliary::primary_key_constraint::FormatSqlPrimaryKeyConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::primary_key_constraint::FormatSqlPrimaryKeyConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrimaryKeyConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrimaryKeyConstraint,
        crate::sql::auxiliary::primary_key_constraint::FormatSqlPrimaryKeyConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::primary_key_constraint::FormatSqlPrimaryKeyConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlPrimaryKeyTableConstraint>
    for crate::sql::auxiliary::primary_key_table_constraint::FormatSqlPrimaryKeyTableConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPrimaryKeyTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrimaryKeyTableConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrimaryKeyTableConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrimaryKeyTableConstraint,
        crate::sql::auxiliary::primary_key_table_constraint::FormatSqlPrimaryKeyTableConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: primary_key_table_constraint :: FormatSqlPrimaryKeyTableConstraint :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrimaryKeyTableConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrimaryKeyTableConstraint,
        crate::sql::auxiliary::primary_key_table_constraint::FormatSqlPrimaryKeyTableConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: primary_key_table_constraint :: FormatSqlPrimaryKeyTableConstraint :: default ())
    }
}
impl FormatRule<sql_syntax::SqlPrivilege> for crate::sql::auxiliary::privilege::FormatSqlPrivilege {
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlPrivilege, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrivilege>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrivilege {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrivilege,
        crate::sql::auxiliary::privilege::FormatSqlPrivilege,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::privilege::FormatSqlPrivilege::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrivilege {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrivilege,
        crate::sql::auxiliary::privilege::FormatSqlPrivilege,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::privilege::FormatSqlPrivilege::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlPrivilegeAllInSchema>
    for crate::sql::auxiliary::privilege_all_in_schema::FormatSqlPrivilegeAllInSchema
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPrivilegeAllInSchema,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrivilegeAllInSchema>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeAllInSchema {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrivilegeAllInSchema,
        crate::sql::auxiliary::privilege_all_in_schema::FormatSqlPrivilegeAllInSchema,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::privilege_all_in_schema::FormatSqlPrivilegeAllInSchema::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeAllInSchema {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrivilegeAllInSchema,
        crate::sql::auxiliary::privilege_all_in_schema::FormatSqlPrivilegeAllInSchema,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::privilege_all_in_schema::FormatSqlPrivilegeAllInSchema::default(
            ),
        )
    }
}
impl FormatRule<sql_syntax::SqlPrivilegeObjects>
    for crate::sql::auxiliary::privilege_objects::FormatSqlPrivilegeObjects
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlPrivilegeObjects,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlPrivilegeObjects>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeObjects {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrivilegeObjects,
        crate::sql::auxiliary::privilege_objects::FormatSqlPrivilegeObjects,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::privilege_objects::FormatSqlPrivilegeObjects::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeObjects {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrivilegeObjects,
        crate::sql::auxiliary::privilege_objects::FormatSqlPrivilegeObjects,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::privilege_objects::FormatSqlPrivilegeObjects::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlReferencesConstraint>
    for crate::sql::auxiliary::references_constraint::FormatSqlReferencesConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlReferencesConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlReferencesConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlReferencesConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlReferencesConstraint,
        crate::sql::auxiliary::references_constraint::FormatSqlReferencesConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::references_constraint::FormatSqlReferencesConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlReferencesConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlReferencesConstraint,
        crate::sql::auxiliary::references_constraint::FormatSqlReferencesConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::references_constraint::FormatSqlReferencesConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlRenameColumnAction>
    for crate::sql::auxiliary::rename_column_action::FormatSqlRenameColumnAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlRenameColumnAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRenameColumnAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRenameColumnAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlRenameColumnAction,
        crate::sql::auxiliary::rename_column_action::FormatSqlRenameColumnAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::rename_column_action::FormatSqlRenameColumnAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRenameColumnAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlRenameColumnAction,
        crate::sql::auxiliary::rename_column_action::FormatSqlRenameColumnAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::rename_column_action::FormatSqlRenameColumnAction::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlRenameConstraintAction>
    for crate::sql::auxiliary::rename_constraint_action::FormatSqlRenameConstraintAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlRenameConstraintAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRenameConstraintAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRenameConstraintAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlRenameConstraintAction,
        crate::sql::auxiliary::rename_constraint_action::FormatSqlRenameConstraintAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: rename_constraint_action :: FormatSqlRenameConstraintAction :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRenameConstraintAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlRenameConstraintAction,
        crate::sql::auxiliary::rename_constraint_action::FormatSqlRenameConstraintAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: rename_constraint_action :: FormatSqlRenameConstraintAction :: default ())
    }
}
impl FormatRule<sql_syntax::SqlRenameTableAction>
    for crate::sql::auxiliary::rename_table_action::FormatSqlRenameTableAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlRenameTableAction,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRenameTableAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRenameTableAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlRenameTableAction,
        crate::sql::auxiliary::rename_table_action::FormatSqlRenameTableAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::rename_table_action::FormatSqlRenameTableAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRenameTableAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlRenameTableAction,
        crate::sql::auxiliary::rename_table_action::FormatSqlRenameTableAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::rename_table_action::FormatSqlRenameTableAction::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlRevokeStatement>
    for crate::sql::statements::revoke_statement::FormatSqlRevokeStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlRevokeStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRevokeStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRevokeStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlRevokeStatement,
        crate::sql::statements::revoke_statement::FormatSqlRevokeStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::revoke_statement::FormatSqlRevokeStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRevokeStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlRevokeStatement,
        crate::sql::statements::revoke_statement::FormatSqlRevokeStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::revoke_statement::FormatSqlRevokeStatement::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlRollbackStatement>
    for crate::sql::statements::rollback_statement::FormatSqlRollbackStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlRollbackStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlRollbackStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlRollbackStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlRollbackStatement,
        crate::sql::statements::rollback_statement::FormatSqlRollbackStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::rollback_statement::FormatSqlRollbackStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlRollbackStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlRollbackStatement,
        crate::sql::statements::rollback_statement::FormatSqlRollbackStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::rollback_statement::FormatSqlRollbackStatement::default(),
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
impl FormatRule<sql_syntax::SqlSetColumnDefault>
    for crate::sql::auxiliary::set_column_default::FormatSqlSetColumnDefault
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSetColumnDefault,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetColumnDefault>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetColumnDefault {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetColumnDefault,
        crate::sql::auxiliary::set_column_default::FormatSqlSetColumnDefault,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::set_column_default::FormatSqlSetColumnDefault::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetColumnDefault {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetColumnDefault,
        crate::sql::auxiliary::set_column_default::FormatSqlSetColumnDefault,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::set_column_default::FormatSqlSetColumnDefault::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlSetColumnNotNull>
    for crate::sql::auxiliary::set_column_not_null::FormatSqlSetColumnNotNull
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlSetColumnNotNull,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetColumnNotNull>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetColumnNotNull {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetColumnNotNull,
        crate::sql::auxiliary::set_column_not_null::FormatSqlSetColumnNotNull,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::set_column_not_null::FormatSqlSetColumnNotNull::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetColumnNotNull {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetColumnNotNull,
        crate::sql::auxiliary::set_column_not_null::FormatSqlSetColumnNotNull,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::set_column_not_null::FormatSqlSetColumnNotNull::default(),
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
impl FormatRule<sql_syntax::SqlSetSchemaAction>
    for crate::sql::auxiliary::set_schema_action::FormatSqlSetSchemaAction
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlSetSchemaAction, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlSetSchemaAction>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlSetSchemaAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlSetSchemaAction,
        crate::sql::auxiliary::set_schema_action::FormatSqlSetSchemaAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::set_schema_action::FormatSqlSetSchemaAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlSetSchemaAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlSetSchemaAction,
        crate::sql::auxiliary::set_schema_action::FormatSqlSetSchemaAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::set_schema_action::FormatSqlSetSchemaAction::default(),
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
impl FormatRule<sql_syntax::SqlTableConstraint>
    for crate::sql::auxiliary::table_constraint::FormatSqlTableConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlTableConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTableConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableConstraint,
        crate::sql::auxiliary::table_constraint::FormatSqlTableConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::table_constraint::FormatSqlTableConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableConstraint,
        crate::sql::auxiliary::table_constraint::FormatSqlTableConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::table_constraint::FormatSqlTableConstraint::default(),
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
impl FormatRule<sql_syntax::SqlTruncateStatement>
    for crate::sql::statements::truncate_statement::FormatSqlTruncateStatement
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlTruncateStatement,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlTruncateStatement>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTruncateStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTruncateStatement,
        crate::sql::statements::truncate_statement::FormatSqlTruncateStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::statements::truncate_statement::FormatSqlTruncateStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTruncateStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTruncateStatement,
        crate::sql::statements::truncate_statement::FormatSqlTruncateStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::statements::truncate_statement::FormatSqlTruncateStatement::default(),
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
impl FormatRule<sql_syntax::SqlUniqueConstraint>
    for crate::sql::auxiliary::unique_constraint::FormatSqlUniqueConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlUniqueConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlUniqueConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlUniqueConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlUniqueConstraint,
        crate::sql::auxiliary::unique_constraint::FormatSqlUniqueConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::unique_constraint::FormatSqlUniqueConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlUniqueConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlUniqueConstraint,
        crate::sql::auxiliary::unique_constraint::FormatSqlUniqueConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::unique_constraint::FormatSqlUniqueConstraint::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlUniqueTableConstraint>
    for crate::sql::auxiliary::unique_table_constraint::FormatSqlUniqueTableConstraint
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlUniqueTableConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlUniqueTableConstraint>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlUniqueTableConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlUniqueTableConstraint,
        crate::sql::auxiliary::unique_table_constraint::FormatSqlUniqueTableConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::unique_table_constraint::FormatSqlUniqueTableConstraint::default(
            ),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlUniqueTableConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlUniqueTableConstraint,
        crate::sql::auxiliary::unique_table_constraint::FormatSqlUniqueTableConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::unique_table_constraint::FormatSqlUniqueTableConstraint::default(
            ),
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
impl FormatRule<sql_syntax::SqlWindowFrameClause>
    for crate::sql::clauses::window_frame_clause::FormatSqlWindowFrameClause
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowFrameClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowFrameClause>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameClause {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowFrameClause,
        crate::sql::clauses::window_frame_clause::FormatSqlWindowFrameClause,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::clauses::window_frame_clause::FormatSqlWindowFrameClause::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameClause {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowFrameClause,
        crate::sql::clauses::window_frame_clause::FormatSqlWindowFrameClause,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::clauses::window_frame_clause::FormatSqlWindowFrameClause::default(),
        )
    }
}
impl FormatRule<sql_syntax::SqlWindowFrameCurrentRowBound>
    for crate::sql::auxiliary::window_frame_current_row_bound::FormatSqlWindowFrameCurrentRowBound
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowFrameCurrentRowBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowFrameCurrentRowBound>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameCurrentRowBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowFrameCurrentRowBound,
        crate::sql::auxiliary::window_frame_current_row_bound::FormatSqlWindowFrameCurrentRowBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_current_row_bound :: FormatSqlWindowFrameCurrentRowBound :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameCurrentRowBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowFrameCurrentRowBound,
        crate::sql::auxiliary::window_frame_current_row_bound::FormatSqlWindowFrameCurrentRowBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_current_row_bound :: FormatSqlWindowFrameCurrentRowBound :: default ())
    }
}
impl FormatRule<sql_syntax::SqlWindowFrameOffsetBound>
    for crate::sql::auxiliary::window_frame_offset_bound::FormatSqlWindowFrameOffsetBound
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowFrameOffsetBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowFrameOffsetBound>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameOffsetBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowFrameOffsetBound,
        crate::sql::auxiliary::window_frame_offset_bound::FormatSqlWindowFrameOffsetBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_offset_bound :: FormatSqlWindowFrameOffsetBound :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameOffsetBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowFrameOffsetBound,
        crate::sql::auxiliary::window_frame_offset_bound::FormatSqlWindowFrameOffsetBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_offset_bound :: FormatSqlWindowFrameOffsetBound :: default ())
    }
}
impl FormatRule<sql_syntax::SqlWindowFrameUnboundedBound>
    for crate::sql::auxiliary::window_frame_unbounded_bound::FormatSqlWindowFrameUnboundedBound
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(
        &self,
        node: &sql_syntax::SqlWindowFrameUnboundedBound,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWindowFrameUnboundedBound>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameUnboundedBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWindowFrameUnboundedBound,
        crate::sql::auxiliary::window_frame_unbounded_bound::FormatSqlWindowFrameUnboundedBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_unbounded_bound :: FormatSqlWindowFrameUnboundedBound :: default ())
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWindowFrameUnboundedBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWindowFrameUnboundedBound,
        crate::sql::auxiliary::window_frame_unbounded_bound::FormatSqlWindowFrameUnboundedBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule :: new (self , crate :: sql :: auxiliary :: window_frame_unbounded_bound :: FormatSqlWindowFrameUnboundedBound :: default ())
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
impl FormatRule<sql_syntax::SqlWithGrantOption>
    for crate::sql::auxiliary::with_grant_option::FormatSqlWithGrantOption
{
    type Context = SqlFormatContext;
    #[inline(always)]
    fn fmt(&self, node: &sql_syntax::SqlWithGrantOption, f: &mut SqlFormatter) -> FormatResult<()> {
        FormatNodeRule::<sql_syntax::SqlWithGrantOption>::fmt(self, node, f)
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlWithGrantOption {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlWithGrantOption,
        crate::sql::auxiliary::with_grant_option::FormatSqlWithGrantOption,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::auxiliary::with_grant_option::FormatSqlWithGrantOption::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlWithGrantOption {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlWithGrantOption,
        crate::sql::auxiliary::with_grant_option::FormatSqlWithGrantOption,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::auxiliary::with_grant_option::FormatSqlWithGrantOption::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlCopyOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlCopyOptionList,
        crate::psql::lists::copy_option_list::FormatPsqlCopyOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::copy_option_list::FormatPsqlCopyOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlCopyOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlCopyOptionList,
        crate::psql::lists::copy_option_list::FormatPsqlCopyOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::copy_option_list::FormatPsqlCopyOptionList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlDeclarationList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlDeclarationList,
        crate::psql::lists::pl_declaration_list::FormatPsqlPlDeclarationList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::pl_declaration_list::FormatPsqlPlDeclarationList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlDeclarationList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlDeclarationList,
        crate::psql::lists::pl_declaration_list::FormatPsqlPlDeclarationList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::pl_declaration_list::FormatPsqlPlDeclarationList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlPlStatementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlPlStatementList,
        crate::psql::lists::pl_statement_list::FormatPsqlPlStatementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::pl_statement_list::FormatPsqlPlStatementList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlPlStatementList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlPlStatementList,
        crate::psql::lists::pl_statement_list::FormatPsqlPlStatementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::pl_statement_list::FormatPsqlPlStatementList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOptionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlSequenceOptionList,
        crate::psql::lists::sequence_option_list::FormatPsqlSequenceOptionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::sequence_option_list::FormatPsqlSequenceOptionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlSequenceOptionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlSequenceOptionList,
        crate::psql::lists::sequence_option_list::FormatPsqlSequenceOptionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::sequence_option_list::FormatPsqlSequenceOptionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::PsqlTableAndColumnsList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::PsqlTableAndColumnsList,
        crate::psql::lists::table_and_columns_list::FormatPsqlTableAndColumnsList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::lists::table_and_columns_list::FormatPsqlTableAndColumnsList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::PsqlTableAndColumnsList {
    type Format = FormatOwnedWithRule<
        sql_syntax::PsqlTableAndColumnsList,
        crate::psql::lists::table_and_columns_list::FormatPsqlTableAndColumnsList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::lists::table_and_columns_list::FormatPsqlTableAndColumnsList::default(),
        )
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlAlterTableActionList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlAlterTableActionList,
        crate::sql::lists::alter_table_action_list::FormatSqlAlterTableActionList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::alter_table_action_list::FormatSqlAlterTableActionList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlAlterTableActionList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlAlterTableActionList,
        crate::sql::lists::alter_table_action_list::FormatSqlAlterTableActionList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::alter_table_action_list::FormatSqlAlterTableActionList::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::SqlCallArgumentList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlCallArgumentList,
        crate::sql::lists::call_argument_list::FormatSqlCallArgumentList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::call_argument_list::FormatSqlCallArgumentList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlCallArgumentList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlCallArgumentList,
        crate::sql::lists::call_argument_list::FormatSqlCallArgumentList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::call_argument_list::FormatSqlCallArgumentList::default(),
        )
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlColumnConstraintList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlColumnConstraintList,
        crate::sql::lists::column_constraint_list::FormatSqlColumnConstraintList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::column_constraint_list::FormatSqlColumnConstraintList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlColumnConstraintList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlColumnConstraintList,
        crate::sql::lists::column_constraint_list::FormatSqlColumnConstraintList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::column_constraint_list::FormatSqlColumnConstraintList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlIndexElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlIndexElementList,
        crate::sql::lists::index_element_list::FormatSqlIndexElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::index_element_list::FormatSqlIndexElementList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlIndexElementList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlIndexElementList,
        crate::sql::lists::index_element_list::FormatSqlIndexElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::index_element_list::FormatSqlIndexElementList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlPrivilegeList,
        crate::sql::lists::privilege_list::FormatSqlPrivilegeList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::privilege_list::FormatSqlPrivilegeList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlPrivilegeList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlPrivilegeList,
        crate::sql::lists::privilege_list::FormatSqlPrivilegeList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::privilege_list::FormatSqlPrivilegeList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::SqlTableElementList {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::SqlTableElementList,
        crate::sql::lists::table_element_list::FormatSqlTableElementList,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::lists::table_element_list::FormatSqlTableElementList::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::SqlTableElementList {
    type Format = FormatOwnedWithRule<
        sql_syntax::SqlTableElementList,
        crate::sql::lists::table_element_list::FormatSqlTableElementList,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::lists::table_element_list::FormatSqlTableElementList::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlCopySource {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlCopySource,
        crate::psql::any::copy_source::FormatAnyPsqlCopySource,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::copy_source::FormatAnyPsqlCopySource::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlCopySource {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlCopySource,
        crate::psql::any::copy_source::FormatAnyPsqlCopySource,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::copy_source::FormatAnyPsqlCopySource::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlCopyTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlCopyTarget,
        crate::psql::any::copy_target::FormatAnyPsqlCopyTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::copy_target::FormatAnyPsqlCopyTarget::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlCopyTarget {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlCopyTarget,
        crate::psql::any::copy_target::FormatAnyPsqlCopyTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::copy_target::FormatAnyPsqlCopyTarget::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlLanguageName {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlLanguageName,
        crate::psql::any::language_name::FormatAnyPsqlLanguageName,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::language_name::FormatAnyPsqlLanguageName::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlLanguageName {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlLanguageName,
        crate::psql::any::language_name::FormatAnyPsqlLanguageName,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::language_name::FormatAnyPsqlLanguageName::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlPartitionBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlPartitionBound,
        crate::psql::any::partition_bound::FormatAnyPsqlPartitionBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::partition_bound::FormatAnyPsqlPartitionBound::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlPartitionBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlPartitionBound,
        crate::psql::any::partition_bound::FormatAnyPsqlPartitionBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::partition_bound::FormatAnyPsqlPartitionBound::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlPartitionValues {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlPartitionValues,
        crate::psql::any::partition_values::FormatAnyPsqlPartitionValues,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::partition_values::FormatAnyPsqlPartitionValues::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlPartitionValues {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlPartitionValues,
        crate::psql::any::partition_values::FormatAnyPsqlPartitionValues,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::partition_values::FormatAnyPsqlPartitionValues::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlPlStatement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlPlStatement,
        crate::psql::any::pl_statement::FormatAnyPsqlPlStatement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::pl_statement::FormatAnyPsqlPlStatement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlPlStatement {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlPlStatement,
        crate::psql::any::pl_statement::FormatAnyPsqlPlStatement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::pl_statement::FormatAnyPsqlPlStatement::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnyPsqlRoutineBody {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnyPsqlRoutineBody,
        crate::psql::any::routine_body::FormatAnyPsqlRoutineBody,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::psql::any::routine_body::FormatAnyPsqlRoutineBody::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnyPsqlRoutineBody {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnyPsqlRoutineBody,
        crate::psql::any::routine_body::FormatAnyPsqlRoutineBody,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::psql::any::routine_body::FormatAnyPsqlRoutineBody::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlAlterColumnOperation {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlAlterColumnOperation,
        crate::sql::any::alter_column_operation::FormatAnySqlAlterColumnOperation,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::alter_column_operation::FormatAnySqlAlterColumnOperation::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlAlterColumnOperation {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlAlterColumnOperation,
        crate::sql::any::alter_column_operation::FormatAnySqlAlterColumnOperation,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::alter_column_operation::FormatAnySqlAlterColumnOperation::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlAlterTableAction {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlAlterTableAction,
        crate::sql::any::alter_table_action::FormatAnySqlAlterTableAction,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::alter_table_action::FormatAnySqlAlterTableAction::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlAlterTableAction {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlAlterTableAction,
        crate::sql::any::alter_table_action::FormatAnySqlAlterTableAction,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::alter_table_action::FormatAnySqlAlterTableAction::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlCallArgument {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlCallArgument,
        crate::sql::any::call_argument::FormatAnySqlCallArgument,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::call_argument::FormatAnySqlCallArgument::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlCallArgument {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlCallArgument,
        crate::sql::any::call_argument::FormatAnySqlCallArgument,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::call_argument::FormatAnySqlCallArgument::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlColumnConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlColumnConstraint,
        crate::sql::any::column_constraint::FormatAnySqlColumnConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::column_constraint::FormatAnySqlColumnConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlColumnConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlColumnConstraint,
        crate::sql::any::column_constraint::FormatAnySqlColumnConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::column_constraint::FormatAnySqlColumnConstraint::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlCreateTableAsQuery {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlCreateTableAsQuery,
        crate::sql::any::create_table_as_query::FormatAnySqlCreateTableAsQuery,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::create_table_as_query::FormatAnySqlCreateTableAsQuery::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlCreateTableAsQuery {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlCreateTableAsQuery,
        crate::sql::any::create_table_as_query::FormatAnySqlCreateTableAsQuery,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::create_table_as_query::FormatAnySqlCreateTableAsQuery::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlPrivilegeTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlPrivilegeTarget,
        crate::sql::any::privilege_target::FormatAnySqlPrivilegeTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::privilege_target::FormatAnySqlPrivilegeTarget::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlPrivilegeTarget {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlPrivilegeTarget,
        crate::sql::any::privilege_target::FormatAnySqlPrivilegeTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::privilege_target::FormatAnySqlPrivilegeTarget::default(),
        )
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlTableConstraint {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlTableConstraint,
        crate::sql::any::table_constraint::FormatAnySqlTableConstraint,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::table_constraint::FormatAnySqlTableConstraint::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlTableConstraint {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlTableConstraint,
        crate::sql::any::table_constraint::FormatAnySqlTableConstraint,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::table_constraint::FormatAnySqlTableConstraint::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlTableElement {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlTableElement,
        crate::sql::any::table_element::FormatAnySqlTableElement,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::table_element::FormatAnySqlTableElement::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlTableElement {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlTableElement,
        crate::sql::any::table_element::FormatAnySqlTableElement,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::table_element::FormatAnySqlTableElement::default(),
        )
    }
}
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlTriggerTarget {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlTriggerTarget,
        crate::sql::any::trigger_target::FormatAnySqlTriggerTarget,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::trigger_target::FormatAnySqlTriggerTarget::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlTriggerTarget {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlTriggerTarget,
        crate::sql::any::trigger_target::FormatAnySqlTriggerTarget,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::trigger_target::FormatAnySqlTriggerTarget::default(),
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
impl AsFormat<SqlFormatContext> for sql_syntax::AnySqlWindowFrameBound {
    type Format<'a> = FormatRefWithRule<
        'a,
        sql_syntax::AnySqlWindowFrameBound,
        crate::sql::any::window_frame_bound::FormatAnySqlWindowFrameBound,
    >;
    fn format(&self) -> Self::Format<'_> {
        FormatRefWithRule::new(
            self,
            crate::sql::any::window_frame_bound::FormatAnySqlWindowFrameBound::default(),
        )
    }
}
impl IntoFormat<SqlFormatContext> for sql_syntax::AnySqlWindowFrameBound {
    type Format = FormatOwnedWithRule<
        sql_syntax::AnySqlWindowFrameBound,
        crate::sql::any::window_frame_bound::FormatAnySqlWindowFrameBound,
    >;
    fn into_format(self) -> Self::Format {
        FormatOwnedWithRule::new(
            self,
            crate::sql::any::window_frame_bound::FormatAnySqlWindowFrameBound::default(),
        )
    }
}
