mod diagnostic;
mod index;
mod rules;

pub use diagnostic::{Diagnostic, DiagnosticTag, Severity};
pub use index::{CoreIndex, ProjectIndex};

use std::borrow::Borrow;
use std::hash::Hash;

use mlang_semantic::AnyMDefinition;
use mlang_syntax::MSyntaxNode;

/// Pure-syntax rules. Can run immediately after parsing, before any
/// semantic model exists.
pub fn syntax_diagnostics(root: &MSyntaxNode) -> Vec<Diagnostic> {
    rules::lint(root, None)
}

/// All rules, including the ones that need resolved declarations (built-ins
/// and user functions), in one walk over the tree. The `definitions` of the
/// linted `file` replace its version in `project`.
pub fn diagnostics<'a, F, Q>(
    root: &MSyntaxNode,
    core: &'a CoreIndex,
    project: &'a ProjectIndex<F>,
    file: &Q,
    definitions: &'a [AnyMDefinition],
) -> Vec<Diagnostic>
where
    F: Hash + Eq + Borrow<Q>,
    Q: Hash + Eq + ?Sized,
{
    let semantic = rules::SemanticIndex::new(core, project.without(file), definitions);
    rules::lint(root, Some(&semantic))
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, LazyLock};

    use line_index::LineIndex;
    use mlang_parser::parse;
    use mlang_semantic::{SemanticModel, semantics};
    use mlang_syntax::MFileSource;

    use super::*;

    static CORE: LazyLock<CoreIndex> = LazyLock::new(|| CoreIndex::new(Arc::new([])));

    /// Lints a single file without the built-in API.
    pub(crate) fn lint(root: &MSyntaxNode, model: &SemanticModel) -> Vec<Diagnostic> {
        let project = ProjectIndex::<()>::default();
        diagnostics(root, &CORE, &project, &(), model.definitions().as_slice())
    }

    fn model(text: &str) -> (MSyntaxNode, Arc<SemanticModel>) {
        let root = parse(text, MFileSource::module()).syntax();
        let model = semantics(&LineIndex::new(text), root.clone(), MFileSource::module());
        (root, Arc::new(model))
    }

    fn codes(project: &ProjectIndex<&str>, file: &str, text: &str) -> Vec<&'static str> {
        let (root, model) = model(text);
        let definitions = model.definitions().as_slice();
        diagnostics(&root, &CORE, project, file, definitions)
            .into_iter()
            .map(|d| d.code)
            .collect()
    }

    #[test]
    fn sees_definition_changes_of_other_files_once_indexed() {
        let call = r#"
var x = f(1);
"#;
        let mut project = ProjectIndex::default();
        project.insert("a", model("func f(a, b) {}").1);
        assert_eq!(codes(&project, "b", call), ["call-arity-mismatch"]);

        let deprecated = r#"
# @deprecated
func f(a) {}
"#;
        project.insert("a", model(deprecated).1);
        assert_eq!(codes(&project, "b", call), ["deprecated"]);

        project.remove("a");
        assert!(codes(&project, "b", call).is_empty());
    }

    #[test]
    fn document_definitions_replace_its_indexed_version() {
        let indexed = r#"
# @deprecated
func f(a, b) {}
"#;
        let mut project = ProjectIndex::default();
        project.insert("a", model(indexed).1);

        let edited = r#"
func f(a) {}
var x = f(1);
"#;
        assert!(codes(&project, "a", edited).is_empty());

        let other = r#"
var x = f(1, 2);
var y = f(1);
"#;
        assert_eq!(
            codes(&project, "b", other),
            ["deprecated", "call-arity-mismatch"]
        );
    }
}
