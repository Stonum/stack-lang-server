use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPartitionHashValues;
use sql_syntax::PsqlPartitionHashValuesFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPartitionHashValues;
impl FormatNodeRule<PsqlPartitionHashValues> for FormatPsqlPartitionHashValues {
    fn fmt_fields(&self, node: &PsqlPartitionHashValues, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlPartitionHashValuesFields {
            with_token,
            l_paren_token,
            modulus_token,
            modulus,
            comma_token,
            remainder_token,
            remainder,
            r_paren_token,
        } = node.as_fields();

        write!(
            f,
            [
                with_token.format(),
                space(),
                l_paren_token.format(),
                modulus_token.format(),
                space(),
                modulus.format(),
                comma_token.format(),
                space(),
                remainder_token.format(),
                space(),
                remainder.format(),
                r_paren_token.format()
            ]
        )
    }
}
