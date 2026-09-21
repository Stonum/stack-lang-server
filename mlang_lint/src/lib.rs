mod diagnostic;
mod rules;

pub use diagnostic::{Diagnostic, DiagnosticTag, Severity};

use mlang_core::AnyMCoreDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::MSyntaxNode;

/// Pure-syntax rules. Can run immediately after parsing, before any
/// semantic model exists.
pub fn syntax_diagnostics(root: &MSyntaxNode) -> Vec<Diagnostic> {
    rules::SYNTAX_RULES
        .iter()
        .flat_map(|rule| rule(root))
        .collect()
}

/// Rules that need resolved declarations (built-ins and user functions).
pub fn semantic_diagnostics<'a>(
    root: &MSyntaxNode,
    core: &'a [AnyMCoreDefinition],
    definitions: impl Iterator<Item = &'a AnyMDefinition>,
) -> Vec<Diagnostic> {
    let definitions = definitions.collect::<Vec<_>>();

    let mut diagnostics =
        rules::call_arity_mismatch::check(root, core, definitions.iter().copied());

    diagnostics.extend(rules::deprecated_usage::check(
        root,
        definitions.into_iter(),
    ));
    diagnostics
}
