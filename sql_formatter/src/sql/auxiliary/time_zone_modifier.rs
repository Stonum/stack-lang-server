use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::SqlTimeZoneModifier;
use sql_syntax::SqlTimeZoneModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTimeZoneModifier;
impl FormatNodeRule<SqlTimeZoneModifier> for FormatSqlTimeZoneModifier {
    fn fmt_fields(&self, node: &SqlTimeZoneModifier, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTimeZoneModifierFields {
            with_or_without,
            time_token,
            zone_token,
        } = node.as_fields();

        write!(
            f,
            [
                with_or_without.format(),
                space(),
                time_token.format(),
                space(),
                zone_token.format()
            ]
        )
    }
}
