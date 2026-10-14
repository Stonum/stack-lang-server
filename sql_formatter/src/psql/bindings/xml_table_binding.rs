use crate::prelude::*;
use crate::utils::write_wrapping_fill_clause;
use biome_formatter::write;
use sql_syntax::AnyPsqlXmlTableColumn;
use sql_syntax::PsqlXmlTableBinding;
use sql_syntax::PsqlXmlTableBindingFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlXmlTableBinding;
impl FormatNodeRule<PsqlXmlTableBinding> for FormatPsqlXmlTableBinding {
    fn fmt_fields(&self, node: &PsqlXmlTableBinding, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlXmlTableBindingFields {
            name_token,
            l_paren_token,
            namespaces,
            row,
            passing,
            columns_token,
            columns,
            r_paren_token,
            alias,
        } = node.as_fields();

        write!(
            f,
            [
                name_token.format(),
                l_paren_token.format(),
                group(&soft_block_indent(&format_with(|f| {
                    if let Some(namespaces) = &namespaces {
                        write!(f, [namespaces.format(), soft_line_break_or_space()])?;
                    }
                    write!(
                        f,
                        [
                            row.format(),
                            soft_line_break_or_space(),
                            passing.format(),
                            soft_line_break_or_space()
                        ]
                    )?;
                    // A column with a `PATH`/`DEFAULT` takes its own line once
                    // the list wraps.
                    write_wrapping_fill_clause(
                        columns_token.format(),
                        &columns,
                        |column: &AnyPsqlXmlTableColumn| match column {
                            AnyPsqlXmlTableColumn::PsqlXmlTableValueColumn(column) => {
                                column.path().is_some() || column.default().is_some()
                            }
                            AnyPsqlXmlTableColumn::PsqlXmlTableOrdinalityColumn(_) => false,
                        },
                        f,
                    )
                }))),
                r_paren_token.format()
            ]
        )?;
        if let Some(alias) = alias {
            write!(f, [space(), alias.format()])?;
        }
        Ok(())
    }
}
