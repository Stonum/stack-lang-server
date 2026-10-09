//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlStatement;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlStatement;
impl FormatRule<AnySqlStatement> for FormatAnySqlStatement {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlStatement, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlStatement::SqlAlterIndexStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlAlterTableStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlBogusStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlCallStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlCreateIndexStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlCreateTableAsStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlCreateTableStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlCreateViewStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlDeleteStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlDropFunctionStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlDropIndexStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlDropTableStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlDropViewStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlEmptyStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlGrantStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlInsertStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlSelectStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlTruncateStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlUpdateStatement(node) => node.format().fmt(f),
            AnySqlStatement::SqlValuesClause(node) => node.format().fmt(f),
            AnySqlStatement::PsqlAnalyzeStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlCreateExtensionStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlCreateFunctionStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlCreateMaterializedViewStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlCreatePolicyStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlCreateTriggerStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlDropExtensionStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlDropMaterializedViewStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlDropPolicyStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlDropTriggerStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlRefreshMaterializedViewStatement(node) => node.format().fmt(f),
            AnySqlStatement::PsqlVacuumStatement(node) => node.format().fmt(f),
        }
    }
}
