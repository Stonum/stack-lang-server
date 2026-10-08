use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlIdentityConstraint;
use sql_syntax::PsqlIdentityConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlIdentityConstraint;
impl FormatNodeRule<PsqlIdentityConstraint> for FormatPsqlIdentityConstraint {
    fn fmt_fields(&self, node: &PsqlIdentityConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlIdentityConstraintFields {
            generated_token,
            always_token,
            by_token,
            default_token,
            as_token,
            identity_token,
            options,
        } = node.as_fields();

        write!(f, [generated_token.format()])?;
        for token in [&always_token, &by_token, &default_token]
            .into_iter()
            .flatten()
        {
            write!(f, [space(), token.format()])?;
        }
        write!(
            f,
            [space(), as_token.format(), space(), identity_token.format()]
        )?;
        if let Some(options) = options {
            write!(f, [space(), options.format()])?;
        }
        Ok(())
    }
}
