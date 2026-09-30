pub mod call_arity_mismatch;
pub mod deprecated_usage;
pub mod else_stray_condition;
pub mod if_stray_semicolon;

use lsp_definition::CodeSymbolDefinition;
use mlang_semantic::AnyMDefinition;
use mlang_syntax::MSyntaxKind::{M_CALL_EXPRESSION, M_IF_STATEMENT, M_NEW_EXPRESSION};
use mlang_syntax::{AstNode, MCallExpression, MIfStatement, MNewExpression, MSyntaxNode};

use crate::Diagnostic;
use crate::index::{CoreIndex, Definitions, Entry, Project};

/// Declarations the semantic rules resolve against: the built-in API, the project
/// and the linted document, whose definitions replace its indexed version.
pub(crate) struct SemanticIndex<'a> {
    pub(crate) core: &'a CoreIndex,
    project: Project<'a>,
    document: &'a [AnyMDefinition],
    own: Definitions<u32>,
}

impl<'a> SemanticIndex<'a> {
    pub(crate) fn new(
        core: &'a CoreIndex,
        project: Project<'a>,
        document: &'a [AnyMDefinition],
    ) -> Self {
        let mut own = Definitions::default();
        for (i, d) in document.iter().enumerate() {
            own.insert(d, i as u32);
        }
        Self {
            core,
            project,
            document,
            own,
        }
    }

    fn candidates<'b>(
        &'b self,
        project: &'a [Entry],
        own: &'b [u32],
    ) -> impl Iterator<Item = &'a AnyMDefinition> + 'b {
        let document = self.document;
        self.project
            .resolve(project)
            .chain(own.iter().map(move |&i| &document[i as usize]))
    }

    pub(crate) fn functions<'b>(
        &'b self,
        name: &'b str,
    ) -> impl Iterator<Item = &'a AnyMDefinition> + 'b {
        let project = self.project.definitions.functions(name);
        self.candidates(project, self.own.functions(name))
            .filter(move |d| unicase::eq(d.id(), name))
    }

    pub(crate) fn classes<'b>(
        &'b self,
        name: &'b str,
    ) -> impl Iterator<Item = &'a AnyMDefinition> + 'b {
        let project = self.project.definitions.classes(name);
        self.candidates(project, self.own.classes(name))
            .filter(move |d| unicase::eq(d.id(), name))
    }

    pub(crate) fn constructors<'b>(
        &'b self,
        class: &'b str,
    ) -> impl Iterator<Item = &'a AnyMDefinition> + 'b {
        let project = self.project.definitions.constructors(class);
        self.candidates(project, self.own.constructors(class))
            .filter(move |d| in_class(d, class))
    }

    pub(crate) fn methods<'b>(
        &'b self,
        class: &'b str,
        name: &'b str,
    ) -> impl Iterator<Item = &'a AnyMDefinition> + 'b {
        let project = self.project.definitions.methods(class, name);
        self.candidates(project, self.own.methods(class, name))
            .filter(move |d| unicase::eq(d.id(), name) && in_class(d, class))
    }
}

fn in_class(d: &AnyMDefinition, class: &str) -> bool {
    d.container().is_some_and(|c| unicase::eq(c.id(), class))
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
                diagnostics.extend(call_arity_mismatch::check(&call, semantic));
                diagnostics.extend(deprecated_usage::check_call(&call, semantic));
            }
            M_NEW_EXPRESSION => {
                let Some(semantic) = semantic else { continue };
                let new = MNewExpression::unwrap_cast(node);
                diagnostics.extend(deprecated_usage::check_new(&new, semantic));
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
            codes(crate::tests::lint(&root, &model)),
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
