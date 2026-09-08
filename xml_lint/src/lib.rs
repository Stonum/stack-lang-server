mod diagnostic;
mod rules;

pub use diagnostic::{Diagnostic, Severity};

use xml_syntax::XmlSyntaxNode;

/// Pure-syntax lint rules — run straight after parsing, no semantic model.
pub fn syntax_diagnostics(root: &XmlSyntaxNode) -> Vec<Diagnostic> {
    rules::SYNTAX_RULES
        .iter()
        .flat_map(|rule| rule(root))
        .collect()
}
