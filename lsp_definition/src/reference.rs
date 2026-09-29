use tower_lsp::lsp_types::{Location, Url};

use crate::{LocationDefinition, SemanticInfo, Symbol, Usage};

/// Usages of the symbol declared under the cursor; empty for any other usage.
pub fn get_reference<'a, I, R>(info: &SemanticInfo, uri: &Url, references: I) -> Vec<Location>
where
    I: IntoIterator<Item = (&'a SemanticInfo, &'a Vec<R>)>,
    R: LocationDefinition + 'a,
{
    if info.usage != Usage::Declaration {
        return vec![];
    }

    references
        .into_iter()
        .filter(|(reference, _)| refers_to(&info.symbol, reference))
        .flat_map(|(_, locations)| locations.iter().map(|r| r.location(uri.clone())))
        .collect()
}

fn refers_to(declared: &Symbol, reference: &SemanticInfo) -> bool {
    match (declared, &reference.symbol, reference.usage) {
        (Symbol::Function(a), Symbol::Function(b), Usage::Call(_)) => unicase::eq(a, b),
        (Symbol::Class(a), Symbol::Class(b), Usage::New(_)) => unicase::eq(a, b),
        (
            Symbol::Member {
                name: a,
                class: a_class,
            },
            Symbol::Member {
                name: b,
                class: b_class,
            },
            Usage::Call(_),
        ) => {
            let same_class = match (a_class, b_class) {
                (Some(a), Some(b)) => unicase::eq(a, b),
                _ => true,
            };
            unicase::eq(a, b) && same_class
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn member(name: &str, class: Option<&str>) -> Symbol {
        Symbol::Member {
            name: name.into(),
            class: class.map(Into::into),
        }
    }

    #[test]
    fn function_is_referenced_by_calls_ignoring_case() {
        let declared = Symbol::Function("Foo".into());
        let call = SemanticInfo::new(Symbol::Function("foo".into()), Usage::Call(2));
        let result = SemanticInfo::new(Symbol::Function("foo".into()), Usage::CallResult(2));

        assert!(refers_to(&declared, &call));
        assert!(!refers_to(&declared, &result));
    }

    #[test]
    fn method_is_referenced_by_calls_on_same_or_unknown_class() {
        let declared = member("m", Some("A"));
        let call = |class| SemanticInfo::new(member("m", class), Usage::Call(0));

        assert!(refers_to(&declared, &call(Some("a"))));
        assert!(refers_to(&declared, &call(None)));
        assert!(!refers_to(&declared, &call(Some("B"))));
    }

    #[test]
    fn class_is_referenced_by_new_only() {
        let declared = Symbol::Class("A".into());

        assert!(refers_to(
            &declared,
            &SemanticInfo::new(Symbol::Class("A".into()), Usage::New(0))
        ));
        assert!(!refers_to(
            &declared,
            &SemanticInfo::new(Symbol::Class("A".into()), Usage::Instance)
        ));
    }
}
