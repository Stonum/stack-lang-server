use crate::prelude::*;
use biome_formatter::write;
use sql_syntax::PsqlCopyFile;
use sql_syntax::PsqlCopyFileFields;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlCopyFile;
impl FormatNodeRule<PsqlCopyFile> for FormatPsqlCopyFile {
    fn fmt_fields(&self, node: &PsqlCopyFile, f: &mut SqlFormatter) -> FormatResult<()> {
        let PsqlCopyFileFields {
            program_token,
            file,
        } = node.as_fields();

        if let Some(program_token) = program_token {
            write!(f, [program_token.format(), space()])?;
        }
        write!(f, [file.format()])
    }
}
