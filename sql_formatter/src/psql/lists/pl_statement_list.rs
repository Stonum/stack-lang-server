use crate::prelude::*;
use sql_syntax::PsqlPlStatementList;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlStatementList;
impl FormatRule<PsqlPlStatementList> for FormatPsqlPlStatementList {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &PsqlPlStatementList, f: &mut SqlFormatter) -> FormatResult<()> {
        f.join_with(hard_line_break())
            .entries(node.iter().formatted())
            .finish()
    }
}

/// The statements of a block or `IF` branch: indented on their own lines, or
/// just a line break when there are none.
pub(crate) struct FormatPlBranchStatements<'a> {
    statements: &'a PsqlPlStatementList,
}

impl<'a> FormatPlBranchStatements<'a> {
    pub(crate) fn new(statements: &'a PsqlPlStatementList) -> Self {
        Self { statements }
    }
}

impl Format<SqlFormatContext> for FormatPlBranchStatements<'_> {
    fn fmt(&self, f: &mut SqlFormatter) -> FormatResult<()> {
        if self.statements.is_empty() {
            hard_line_break().fmt(f)
        } else {
            block_indent(&self.statements.format()).fmt(f)
        }
    }
}
