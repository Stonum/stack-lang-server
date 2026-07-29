use line_index::{LineCol, LineColRange};
use mlang_formatter::{IndentStyle, IndentWidth, LineWidth};
use mlang_syntax::MFileSource;
use psql_syntax::PsqlFileSource;

use tower_lsp::lsp_types::{FormattingOptions, Position};
use tower_lsp::lsp_types::{Range, TextEdit};

use crate::document::{CurrentDocument, DocumentLanguage};

const FORMAT_LINE_WIDTH: u16 = 120;
const FORMAT_PRETTY_LINE_WIDTH: u16 = 90;

pub fn format(
    document: &CurrentDocument,
    options: FormattingOptions,
    range: Range,
) -> Option<Vec<TextEdit>> {
    let indent_style = match options.insert_spaces {
        true => IndentStyle::Space,
        false => IndentStyle::Tab,
    };
    let line_width = LineWidth::try_from(FORMAT_LINE_WIDTH).unwrap();
    let indent_width = IndentWidth::from(options.tab_size as u8);

    let line_index = document.line_index();
    let text_range = {
        let Range { start, end } = range;
        let range = LineColRange {
            start: LineCol {
                col: start.character,
                line: start.line,
            },
            end: LineCol {
                col: end.character,
                line: end.line,
            },
        };
        line_index.range(range)?
    };

    let formatted_text = match document.language() {
        DocumentLanguage::Mlang => {
            let format_options = mlang_formatter::MFormatOptions::new(MFileSource::module())
                .with_indent_style(indent_style)
                .with_line_width(line_width)
                .with_pretty_line_width(LineWidth::try_from(FORMAT_PRETTY_LINE_WIDTH).unwrap())
                .with_indent_width(indent_width)
                .with_bracket_spacing(false.into());

            mlang_formatter::format_range(format_options, &document.mlang_syntax()?, text_range)
                .ok()?
        }
        DocumentLanguage::Psql => {
            let format_options = psql_formatter::PsqlFormatOptions::new(PsqlFileSource::script())
                .with_indent_style(indent_style)
                .with_line_width(line_width)
                .with_indent_width(indent_width);

            psql_formatter::format_range(format_options, &document.psql_syntax()?, text_range)
                .ok()?
        }
    };

    let range = {
        let range = formatted_text.range()?;
        let LineColRange { start, end } = line_index.line_col_range(range)?;
        Range {
            start: Position::new(start.line, start.col),
            end: Position::new(end.line, end.col),
        }
    };

    let new_text = formatted_text.into_code();
    let edits = vec![TextEdit { range, new_text }];

    Some(edits)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tower_lsp::lsp_types::{Position, Url};

    fn formatting_options() -> FormattingOptions {
        FormattingOptions {
            tab_size: 4,
            insert_spaces: true,
            properties: Default::default(),
            trim_trailing_whitespace: None,
            insert_final_newline: None,
            trim_final_newlines: None,
        }
    }

    fn whole_document_range(text: &str) -> Range {
        let lines = text.lines().count() as u32;
        Range {
            start: Position::new(0, 0),
            end: Position::new(lines, 0),
        }
    }

    #[test]
    fn formats_a_psql_document() {
        let uri = Url::parse("file:///test.sql").unwrap();
        let text = "select   a,b   from t\n";
        let document = CurrentDocument::new_psql(uri, text, PsqlFileSource::script());

        let edits = format(&document, formatting_options(), whole_document_range(text))
            .expect("psql document should format");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "select a, b from t");
    }

    #[test]
    fn formats_an_mlang_document() {
        let uri = Url::parse("file:///test.prg").unwrap();
        let text = "var   a=1;\n";
        let document = CurrentDocument::new_mlang(uri, text, MFileSource::module());

        let edits = format(&document, formatting_options(), whole_document_range(text))
            .expect("mlang document should format");

        assert_eq!(edits.len(), 1);
        assert_eq!(edits[0].new_text, "var a = 1;");
    }
}
