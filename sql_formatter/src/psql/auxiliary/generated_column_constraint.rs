use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlGeneratedColumnConstraint;
use sql_syntax::PsqlGeneratedColumnConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlGeneratedColumnConstraint;
impl FormatNodeRule<PsqlGeneratedColumnConstraint> for FormatPsqlGeneratedColumnConstraint {
    fn fmt_fields(
        &self,
        node: &PsqlGeneratedColumnConstraint,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlGeneratedColumnConstraintFields {
            generated_token,
            always_token,
            as_token,
            l_paren_token,
            expression,
            r_paren_token,
            stored_token,
        } = node.as_fields();

        write!(
            f,
            [
                generated_token.format(),
                space(),
                always_token.format(),
                space(),
                as_token.format(),
                space(),
                l_paren_token.format(),
                expression.format(),
                r_paren_token.format(),
                space(),
                stored_token.format()
            ]
        )
    }
}
