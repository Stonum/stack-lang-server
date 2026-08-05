use crate::comments::{FormatSqlLeadingComment, SqlCommentStyle, SqlComments};
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    AttributePosition, CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth,
    LineEnding, LineWidth, TransformSourceMap,
};
use sql_syntax::SqlFileSource;
use sql_syntax::SqlLanguage;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct SqlFormatContext {
    options: SqlFormatOptions,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<SqlComments>,

    source_map: Option<TransformSourceMap>,
}

impl SqlFormatContext {
    pub fn new(options: SqlFormatOptions, comments: SqlComments) -> Self {
        Self {
            options,
            comments: Rc::new(comments),
            source_map: None,
        }
    }

    pub fn with_source_map(mut self, source_map: Option<TransformSourceMap>) -> Self {
        self.source_map = source_map;
        self
    }
}

impl FormatContext for SqlFormatContext {
    type Options = SqlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for SqlFormatContext {
    type Language = SqlLanguage;
    type Style = SqlCommentStyle;
    type CommentRule = FormatSqlLeadingComment;

    fn comments(&self) -> &SqlComments {
        &self.comments
    }
}

/// SQL doesn't have JS-style stylistic choices like quote style (string
/// literals are always single-quoted) or trailing commas (a comma after the
/// last item in a list is a syntax error, not a style option) -- this is
/// deliberately smaller than `MFormatOptions`, not a placeholder to fill in
/// later.
#[derive(Debug, Clone)]
pub struct SqlFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// Information related to the current file
    source_type: SqlFileSource,
}

impl SqlFormatOptions {
    pub fn new(source_type: SqlFileSource) -> Self {
        Self {
            source_type,
            indent_style: IndentStyle::default(),
            indent_width: IndentWidth::default(),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
        }
    }

    pub fn with_indent_style(mut self, indent_style: IndentStyle) -> Self {
        self.indent_style = indent_style;
        self
    }

    pub fn with_indent_width(mut self, indent_width: IndentWidth) -> Self {
        self.indent_width = indent_width;
        self
    }

    pub fn with_line_ending(mut self, line_ending: LineEnding) -> Self {
        self.line_ending = line_ending;
        self
    }

    pub fn with_line_width(mut self, line_width: LineWidth) -> Self {
        self.line_width = line_width;
        self
    }

    pub fn set_indent_style(&mut self, indent_style: IndentStyle) {
        self.indent_style = indent_style;
    }

    pub fn set_indent_width(&mut self, indent_width: IndentWidth) {
        self.indent_width = indent_width;
    }

    pub fn set_line_ending(&mut self, line_ending: LineEnding) {
        self.line_ending = line_ending;
    }

    pub fn set_line_width(&mut self, line_width: LineWidth) {
        self.line_width = line_width;
    }

    pub fn source_type(&self) -> SqlFileSource {
        self.source_type
    }
}

impl FormatOptions for SqlFormatOptions {
    fn indent_style(&self) -> IndentStyle {
        self.indent_style
    }

    fn indent_width(&self) -> IndentWidth {
        self.indent_width
    }

    fn line_width(&self) -> LineWidth {
        self.line_width
    }

    fn line_ending(&self) -> LineEnding {
        self.line_ending
    }

    /// SQL has no JSX/attribute concept -- this is a required `FormatOptions`
    /// method, not a real style choice, so it's hardcoded rather than exposed
    /// as a configurable field.
    fn attribute_position(&self) -> AttributePosition {
        AttributePosition::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for SqlFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.get())
    }
}
