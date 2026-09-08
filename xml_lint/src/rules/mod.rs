pub mod duplicate_attribute;

use xml_syntax::XmlSyntaxNode;

use crate::Diagnostic;

type SyntaxRule = fn(&XmlSyntaxNode) -> Vec<Diagnostic>;

pub(crate) const SYNTAX_RULES: &[SyntaxRule] = &[duplicate_attribute::check];
