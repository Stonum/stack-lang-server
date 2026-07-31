use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlSubstringExpression;
use psql_syntax::PsqlSubstringExpressionFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlSubstringExpression;
impl FormatNodeRule<PsqlSubstringExpression> for FormatPsqlSubstringExpression {
    fn fmt_fields(
        &self,
        node: &PsqlSubstringExpression,
        f: &mut PsqlFormatter,
    ) -> FormatResult<()> {
        let PsqlSubstringExpressionFields {
            name_token,
            l_paren_token,
            expression,
            from_clause,
            for_clause,
            r_paren_token,
        } = node.as_fields();

        // `name_token` isn't a real keyword (`substring` isn't reserved in
        // Postgres), so its original casing is preserved verbatim -- same
        // convention already used for `old`/`new` in a trigger's
        // REFERENCING clause.
        write!(f, [name_token.format(), l_paren_token.format()])?;

        write!(
            f,
            [group(&soft_block_indent(&format_with(|f| {
                write!(f, [expression.format()])?;
                if let Some(from_clause) = &from_clause {
                    write!(f, [soft_line_break_or_space(), from_clause.format()])?;
                }
                if let Some(for_clause) = &for_clause {
                    write!(f, [soft_line_break_or_space(), for_clause.format()])?;
                }
                Ok(())
            })))]
        )?;

        write!(f, [r_paren_token.format()])
    }
}
