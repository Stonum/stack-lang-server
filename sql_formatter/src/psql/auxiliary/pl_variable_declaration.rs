use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlPlVariableDeclaration;
use sql_syntax::PsqlPlVariableDeclarationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlPlVariableDeclaration;
impl FormatNodeRule<PsqlPlVariableDeclaration> for FormatPsqlPlVariableDeclaration {
    fn fmt_fields(
        &self,
        node: &PsqlPlVariableDeclaration,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlPlVariableDeclarationFields {
            name,
            constant_token,
            ty,
            collation,
            not_null,
            default,
            semicolon_token,
        } = node.as_fields();

        write!(f, [name.format()])?;
        if let Some(constant_token) = constant_token {
            write!(f, [space(), constant_token.format()])?;
        }
        write!(f, [space(), ty.format()])?;
        if let Some(collation) = collation {
            write!(f, [space(), collation.format()])?;
        }
        if let Some(not_null) = not_null {
            write!(f, [space(), not_null.format()])?;
        }
        if let Some(default) = default {
            write!(f, [space(), default.format()])?;
        }
        write!(f, [semicolon_token.format()])
    }
}
