//! This is a generated file. Don't modify it by hand! Run 'cargo codegen formatter' to re-generate the file.

use crate::prelude::*;
use psql_syntax::AnyPsqlFunctionOption;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatAnyPsqlFunctionOption;
impl FormatRule<AnyPsqlFunctionOption> for FormatAnyPsqlFunctionOption {
    type Context = PsqlFormatContext;
    fn fmt(&self, node: &AnyPsqlFunctionOption, f: &mut PsqlFormatter) -> FormatResult<()> {
        match node {
            AnyPsqlFunctionOption::PsqlLanguageOption(node) => node.format().fmt(f),
            AnyPsqlFunctionOption::PsqlReturnsNullOption(node) => node.format().fmt(f),
            AnyPsqlFunctionOption::PsqlSecurityOption(node) => node.format().fmt(f),
            AnyPsqlFunctionOption::PsqlStrictOption(node) => node.format().fmt(f),
            AnyPsqlFunctionOption::PsqlVolatilityOption(node) => node.format().fmt(f),
        }
    }
}
