//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use sql_syntax::AnySqlFunctionOption;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnySqlFunctionOption;
impl FormatRule<AnySqlFunctionOption> for FormatAnySqlFunctionOption {
    type Context = SqlFormatContext;
    fn fmt(&self, node: &AnySqlFunctionOption, f: &mut SqlFormatter) -> FormatResult<()> {
        match node {
            AnySqlFunctionOption::PsqlLanguageOption(node) => node.format().fmt(f),
            AnySqlFunctionOption::PsqlReturnsNullOption(node) => node.format().fmt(f),
            AnySqlFunctionOption::PsqlSecurityOption(node) => node.format().fmt(f),
            AnySqlFunctionOption::PsqlStrictOption(node) => node.format().fmt(f),
            AnySqlFunctionOption::PsqlVolatilityOption(node) => node.format().fmt(f),
        }
    }
}
