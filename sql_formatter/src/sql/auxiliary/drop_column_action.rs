use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlDropColumnAction;
use sql_syntax::SqlDropColumnActionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlDropColumnAction;
impl FormatNodeRule<SqlDropColumnAction> for FormatSqlDropColumnAction {
    fn fmt_fields(&self, node: &SqlDropColumnAction, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlDropColumnActionFields {
            drop_token,
            column_token,
            if_token,
            exists_token,
            name,
            drop_behavior,
        } = node.as_fields();

        write!(f, [drop_token.format()])?;
        for token in [&column_token, &if_token, &exists_token]
            .into_iter()
            .flatten()
        {
            write!(f, [space(), token.format()])?;
        }
        write!(f, [space(), name.format()])?;
        if let Some(drop_behavior) = drop_behavior {
            write!(f, [space(), drop_behavior.format()])?;
        }
        Ok(())
    }
}
