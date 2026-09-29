use line_index::LineColRange;
use tower_lsp::lsp_types::{Location, Position, Range, SymbolKind, Url};

pub struct StringLowerCase(String);
impl StringLowerCase {
    pub fn new(s: &str) -> Self {
        StringLowerCase(s.to_lowercase())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DefinitionKind {
    Function,
    Handler,
    HandlerEvent,
    Class,
    Constructor,
    Method,
    Getter,
    Setter,
    Property,
    Report,
    ReportSection,
    Select,
    ApiBrowser,
}

impl DefinitionKind {
    /// Class members reachable without a call: `obj.name`.
    pub fn is_accessor(self) -> bool {
        matches!(self, Self::Getter | Self::Setter | Self::Property)
    }

    pub fn is_member(self) -> bool {
        self == Self::Method || self.is_accessor()
    }
}

pub trait CodeSymbolDefinition: Sized + PartialEq {
    fn kind(&self) -> DefinitionKind;
    fn id(&self) -> &str;
    fn parameters(&self) -> Option<&str>;
    fn container(&self) -> Option<Self>;
    fn parent(&self) -> Option<&str>;

    fn can_be_overridden(&self, another: &Self) -> bool {
        if self.compare_id_with(another.id()) {
            return false;
        }

        self.parameters() == another.parameters()
    }

    fn can_be_called(&self, count: usize) -> bool;

    fn call_priority(&self, another: &Self, count: usize) -> core::cmp::Ordering;

    fn compare_id_with(&self, another: &str) -> bool {
        unicase::eq(self.id(), another)
    }
    fn partial_compare_id_with(&self, another: &StringLowerCase) -> bool {
        self.id().to_lowercase().contains(&another.0)
    }
}

pub trait CodeSymbolInformation: CodeSymbolDefinition {
    fn symbol_kind(&self) -> SymbolKind;
    fn symbol_name(&self) -> String {
        let mut name = self.id();

        let double_quoted = name.starts_with('\"') && name.ends_with('\"');
        let single_quoted = name.starts_with('\'') && name.ends_with('\'');

        if double_quoted || single_quoted {
            name = &name[1..name.len() - 1];
        }

        // symbol name must not be falsy
        if name.is_empty() {
            return String::from(" ");
        }

        name.to_string()
    }
}

pub trait LocationDefinition {
    fn range(&self) -> LineColRange;
    fn lsp_range(&self) -> Range {
        to_lsp_range(self.range())
    }
    fn location(&self, uri: Url) -> Location {
        Location {
            uri,
            range: self.lsp_range(),
        }
    }
    fn id_range(&self) -> LineColRange {
        self.range()
    }
    fn id_lsp_range(&self) -> Range {
        to_lsp_range(self.id_range())
    }
    fn id_location(&self, uri: Url) -> Location {
        Location {
            uri,
            range: self.id_lsp_range(),
        }
    }
}

fn to_lsp_range(LineColRange { start, end }: LineColRange) -> Range {
    Range::new(
        Position::new(start.line, start.col),
        Position::new(end.line, end.col),
    )
}

const SPECIAL_CHARS: [char; 2] = ['\\', '#'];

pub trait MarkupDefinition {
    fn markdown(&self) -> String;
    fn documentation(&self) -> Option<String>;

    fn full_markdown(&self) -> String {
        if let Some(documentation) = self.documentation() {
            return format!("{}  \n{}", self.markdown(), documentation);
        }
        self.markdown()
    }

    fn escape_markdown_with_newlines(&self, s: &str) -> String {
        let mut result = String::with_capacity(s.len() * 2);

        for c in s.chars() {
            match c {
                '\r' => {}
                '\n' => {
                    result.push_str("  \n");
                }
                _ if SPECIAL_CHARS.contains(&c) => {
                    result.push('\\');
                    result.push(c);
                }
                _ => result.push(c),
            }
        }
        result
    }
}

pub trait SignatureParameters {
    fn text(&self) -> String;
    fn offsets(&self) -> Vec<[u32; 2]>;
    fn has_rest(&self) -> bool;
}

/// Describes how many arguments a callable (function/method/constructor) accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Arity {
    /// Total declared parameters, including a trailing rest parameter, if any.
    pub total_count: usize,
    pub optional_count: usize,
    pub has_rest: bool,
}

impl Arity {
    pub fn can_be_called(&self, count: usize) -> bool {
        let strict_count = self.total_count - self.optional_count - self.has_rest as usize;

        // (a, b, c) && count < 3
        if count < strict_count {
            return false;
        }

        // (a, ...) && count >= 1
        if self.has_rest {
            return true;
        }

        // (a, b, c, d = 1) && count == 3 || count == 4
        count - strict_count <= self.optional_count
    }
}
