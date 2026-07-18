use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlWindowSpecification;
use psql_syntax::PsqlWindowSpecificationFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlWindowSpecification;
impl FormatNodeRule<PsqlWindowSpecification> for FormatPsqlWindowSpecification {
    fn fmt_fields(
        &self,
        node: &PsqlWindowSpecification,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlWindowSpecificationFields {
            l_paren_token,
            partition_by_clause,
            order_by_clause,
            r_paren_token,
        } = node.as_fields();

        write!(f, [l_paren_token.format()])?;
        if let Some(partition_by_clause) = &partition_by_clause {
            write!(f, [partition_by_clause.format()])?;
        }
        if let Some(order_by_clause) = order_by_clause {
            if partition_by_clause.is_some() {
                write!(f, [space()])?;
            }
            write!(f, [order_by_clause.format()])?;
        }
        write!(f, [r_paren_token.format()])
    }
}
