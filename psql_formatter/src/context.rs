use crate::comments::{FormatPsqlLeadingComment, PsqlCommentStyle, PsqlComments};
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    AttributePosition, CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth,
    LineEnding, LineWidth, TransformSourceMap,
};
use psql_syntax::PsqlFileSource;
use psql_syntax::PsqlLanguage;
use std::fmt;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct PsqlFormatContext {
    options: PsqlFormatOptions,

    /// The comments of the nodes and tokens in the program.
    comments: Rc<PsqlComments>,

    source_map: Option<TransformSourceMap>,
}

impl PsqlFormatContext {
    pub fn new(options: PsqlFormatOptions, comments: PsqlComments) -> Self {
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

impl FormatContext for PsqlFormatContext {
    type Options = PsqlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for PsqlFormatContext {
    type Language = PsqlLanguage;
    type Style = PsqlCommentStyle;
    type CommentRule = FormatPsqlLeadingComment;

    fn comments(&self) -> &PsqlComments {
        &self.comments
    }
}

/// SQL doesn't have JS-style stylistic choices like quote style (string
/// literals are always single-quoted) or trailing commas (a comma after the
/// last item in a list is a syntax error, not a style option) -- this is
/// deliberately smaller than `MFormatOptions`, not a placeholder to fill in
/// later.
#[derive(Debug, Clone)]
pub struct PsqlFormatOptions {
    /// The indent style.
    indent_style: IndentStyle,

    /// The indent width.
    indent_width: IndentWidth,

    /// The type of line ending.
    line_ending: LineEnding,

    /// What's the max width of a line. Defaults to 80.
    line_width: LineWidth,

    /// Information related to the current file
    source_type: PsqlFileSource,
}

impl PsqlFormatOptions {
    pub fn new(source_type: PsqlFileSource) -> Self {
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

    pub fn source_type(&self) -> PsqlFileSource {
        self.source_type
    }
}

impl FormatOptions for PsqlFormatOptions {
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

impl fmt::Display for PsqlFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.get())
    }
}
