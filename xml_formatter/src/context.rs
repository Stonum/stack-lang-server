use crate::comments::{FormatXmlLeadingComment, XmlCommentStyle, XmlComments};
use biome_formatter::printer::PrinterOptions;
use biome_formatter::{
    AttributePosition, CstFormatContext, FormatContext, FormatOptions, IndentStyle, IndentWidth,
    LineEnding, LineWidth, TransformSourceMap,
};
use std::fmt;
use std::rc::Rc;
use xml_syntax::{XmlFileSource, XmlLanguage, XmlVariant};

#[derive(Debug, Clone)]
pub struct XmlFormatContext {
    options: XmlFormatOptions,

    /// The comments of the nodes and tokens in the document.
    comments: Rc<XmlComments>,

    source_map: Option<TransformSourceMap>,
}

impl XmlFormatContext {
    pub fn new(options: XmlFormatOptions, comments: XmlComments) -> Self {
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

impl FormatContext for XmlFormatContext {
    type Options = XmlFormatOptions;

    fn options(&self) -> &Self::Options {
        &self.options
    }

    fn source_map(&self) -> Option<&TransformSourceMap> {
        self.source_map.as_ref()
    }
}

impl CstFormatContext for XmlFormatContext {
    type Language = XmlLanguage;
    type Style = XmlCommentStyle;
    type CommentRule = FormatXmlLeadingComment;

    fn comments(&self) -> &XmlComments {
        &self.comments
    }
}

/// How to print a self-closing tag: `<a/>` or `<a />`.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Default)]
pub enum SelfClosingSpacing {
    /// `<a/>` — no space before the slash. This is the overwhelmingly
    /// dominant spelling in real Stack resources/dictionaries.
    #[default]
    Tight,
    /// `<a />` — a single space before the slash.
    Loose,
}

impl SelfClosingSpacing {
    pub const fn is_tight(&self) -> bool {
        matches!(self, Self::Tight)
    }
    pub const fn is_loose(&self) -> bool {
        matches!(self, Self::Loose)
    }
}

fn default_indent_width(variant: XmlVariant) -> IndentWidth {
    let width: u8 = match variant {
        XmlVariant::Dictionary => 4,
        XmlVariant::Resource => 3,
        XmlVariant::Plain => 2,
    };
    IndentWidth::from(width)
}

#[derive(Debug, Clone)]
pub struct XmlFormatOptions {
    indent_style: IndentStyle,
    indent_width: IndentWidth,
    line_ending: LineEnding,
    line_width: LineWidth,
    self_closing_spacing: SelfClosingSpacing,
    source_type: XmlFileSource,
}

impl XmlFormatOptions {
    pub fn new(source_type: XmlFileSource) -> Self {
        Self {
            source_type,
            // Stack files are space-indented, and each flavour has its own
            // house style: dictionaries use 4 spaces, resources use 3.
            indent_style: IndentStyle::Space,
            indent_width: default_indent_width(source_type.variant()),
            line_ending: LineEnding::default(),
            line_width: LineWidth::default(),
            self_closing_spacing: SelfClosingSpacing::default(),
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

    pub fn with_self_closing_spacing(mut self, spacing: SelfClosingSpacing) -> Self {
        self.self_closing_spacing = spacing;
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

    pub fn set_self_closing_spacing(&mut self, spacing: SelfClosingSpacing) {
        self.self_closing_spacing = spacing;
    }

    pub fn source_type(&self) -> XmlFileSource {
        self.source_type
    }

    pub fn self_closing_spacing(&self) -> SelfClosingSpacing {
        self.self_closing_spacing
    }
}

impl FormatOptions for XmlFormatOptions {
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

    fn attribute_position(&self) -> AttributePosition {
        AttributePosition::default()
    }

    fn as_print_options(&self) -> PrinterOptions {
        PrinterOptions::from(self)
    }
}

impl fmt::Display for XmlFormatOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Indent style: {}", self.indent_style)?;
        writeln!(f, "Indent width: {}", self.indent_width.value())?;
        writeln!(f, "Line ending: {}", self.line_ending)?;
        writeln!(f, "Line width: {}", self.line_width.get())
    }
}
