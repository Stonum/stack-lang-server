use crate::prelude::*;
use biome_formatter::write;
use psql_syntax::PsqlTimeZoneModifier;
use psql_syntax::PsqlTimeZoneModifierFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlTimeZoneModifier;
impl FormatNodeRule<PsqlTimeZoneModifier> for FormatPsqlTimeZoneModifier {
    fn fmt_fields(&self, node: &PsqlTimeZoneModifier, f: &mut PsqlFormatter) -> FormatResult<()> {
        let PsqlTimeZoneModifierFields {
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
