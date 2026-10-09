use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlExtensionSchemaClause;
use sql_syntax::PsqlExtensionSchemaClauseFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlExtensionSchemaClause;
impl FormatNodeRule<PsqlExtensionSchemaClause> for FormatPsqlExtensionSchemaClause {
    fn fmt_fields(
        &self,
        node: &PsqlExtensionSchemaClause,
        f: &mut SqlFormatter,
    ) -> FormatResult<()> {
        let PsqlExtensionSchemaClauseFields {
            with_token,
            schema_token,
            schema,
        } = node.as_fields();

        if let Some(with_token) = with_token {
            write!(f, [with_token.format(), space()])?;
        }
        write!(f, [schema_token.format(), space(), schema.format()])
    }
}
