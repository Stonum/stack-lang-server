use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTableConstraint;
use sql_syntax::SqlTableConstraintFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTableConstraint;
impl FormatNodeRule<SqlTableConstraint> for FormatSqlTableConstraint {
    fn fmt_fields(&self, node: &SqlTableConstraint, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTableConstraintFields { name, constraint } = node.as_fields();

        if let Some(name) = name {
            write!(f, [name.format(), space()])?;
        }
        write!(f, [constraint.format()])
    }
}
