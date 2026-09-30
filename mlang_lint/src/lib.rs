mod diagnostic;
mod rules;

pub use diagnostic::{Diagnostic, DiagnosticTag, Severity};

use mlang_core::AnyMCoreDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::MSyntaxNode;

/// Pure-syntax rules. Can run immediately after parsing, before any
/// semantic model exists.
pub fn syntax_diagnostics(root: &MSyntaxNode) -> Vec<Diagnostic> {
    rules::lint(root, None)
}

/// All rules, including the ones that need resolved declarations (built-ins
/// and user functions), in one walk over the tree.
pub fn diagnostics<'a>(
    root: &MSyntaxNode,
    core: &'a [AnyMCoreDefinition],
    definitions: impl Iterator<Item = &'a AnyMDefinition>,
) -> Vec<Diagnostic> {
    let semantic = rules::SemanticIndex::new(core, definitions);
    rules::lint(root, Some(&semantic))
}
