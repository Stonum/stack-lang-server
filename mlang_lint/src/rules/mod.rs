pub mod call_arity_mismatch;
pub mod deprecated_usage;
pub mod else_stray_condition;
pub mod if_stray_semicolon;

use mlang_core::AnyMCoreDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::MSyntaxKind::{M_CALL_EXPRESSION, M_IF_STATEMENT, M_NEW_EXPRESSION};
use mlang_syntax::{AstNode, MCallExpression, MIfStatement, MNewExpression, MSyntaxNode};

use crate::Diagnostic;

/// Declarations the semantic rules resolve against.
pub(crate) struct SemanticIndex<'a> {
    arity: call_arity_mismatch::Index<'a>,
    /// `None` when nothing is deprecated.
    deprecated: Option<deprecated_usage::Index<'a>>,
}

impl<'a> SemanticIndex<'a> {
    pub(crate) fn new(
        core: &'a [AnyMCoreDefinition],
        definitions: impl Iterator<Item = &'a AnyMDefinition>,
    ) -> Self {
        let definitions = definitions.collect::<Vec<_>>();
        Self {
            arity: call_arity_mismatch::Index::new(core, definitions.iter().copied()),
            deprecated: deprecated_usage::Index::new(definitions.into_iter()),
        }
    }
}

/// Runs every rule in a single walk over the tree, dispatching on the node kind.
pub(crate) fn lint(root: &MSyntaxNode, semantic: Option<&SemanticIndex>) -> Vec<Diagnostic> {
    let mut diagnostics = vec![];

    for node in root.descendants() {
        match node.kind() {
            M_IF_STATEMENT => {
                let statement = MIfStatement::unwrap_cast(node);
                diagnostics.extend(if_stray_semicolon::check(&statement));
                diagnostics.extend(else_stray_condition::check(&statement));
            }
            M_CALL_EXPRESSION => {
                let Some(semantic) = semantic else { continue };
                let call = MCallExpression::unwrap_cast(node);
                diagnostics.extend(call_arity_mismatch::check(&call, &semantic.arity));
                if let Some(index) = &semantic.deprecated {
                    diagnostics.extend(deprecated_usage::check_call(&call, index));
                }
            }
            M_NEW_EXPRESSION => {
                let Some(index) = semantic.and_then(|s| s.deprecated.as_ref()) else {
                    continue;
                };
                let new = MNewExpression::unwrap_cast(node);
                diagnostics.extend(deprecated_usage::check_new(&new, index));
            }
            _ => {}
        }
    }

    diagnostics
}

#[cfg(test)]
mod tests {
    use line_index::LineIndex;
    use mlang_parser::parse;
    use mlang_semantic::semantics;
    use mlang_syntax::MFileSource;

    use super::*;

    const SRC: &str = r#"
# @deprecated
func f(a) {}
if (x);
f();
f(1);
var o = new Object();
if (y) g() else (z) g()
"#;

    fn codes(diagnostics: Vec<Diagnostic>) -> Vec<&'static str> {
        diagnostics.into_iter().map(|d| d.code).collect()
    }

    #[test]
    fn one_walk_runs_every_rule_in_tree_order() {
        let root = parse(SRC, MFileSource::module()).syntax();
        let model = semantics(&LineIndex::new(SRC), root.clone(), MFileSource::module());
        assert_eq!(
            codes(crate::diagnostics(&root, &[], model.definitions())),
            [
                if_stray_semicolon::CODE,
                call_arity_mismatch::CODE,
                deprecated_usage::CODE,
                else_stray_condition::CODE,
            ]
        );
    }

    #[test]
    fn syntax_diagnostics_skip_semantic_rules() {
        let root = parse(SRC, MFileSource::module()).syntax();
        assert_eq!(
            codes(crate::syntax_diagnostics(&root)),
            [if_stray_semicolon::CODE, else_stray_condition::CODE]
        );
    }
}
