pub mod call_arity_mismatch;
pub mod else_stray_condition;
pub mod if_stray_semicolon;

use mlang_syntax::MSyntaxNode;

use crate::Diagnostic;

type SyntaxRule = fn(&MSyntaxNode) -> Vec<Diagnostic>;

pub(crate) const SYNTAX_RULES: &[SyntaxRule] =
    &[if_stray_semicolon::check, else_stray_condition::check];
