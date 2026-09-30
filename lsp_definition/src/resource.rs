//! Name-based links between rx resources and `.hdl` handlers.

use std::collections::HashMap;
use std::hash::{BuildHasher, RandomState};

use tower_lsp::lsp_types::Url;
use unicase::UniCase;

use crate::{CodeSymbolDefinition, DefinitionKind, Symbol};

const API_HANDLER_SUFFIX: &str = "_АПИ";
const EXTRA_HANDLER_SUFFIX: &str = "_new";

/// `<APIBrowser Имя="X">` is handled by `'X_АПИ'`.
pub fn api_handler_name(browser: &str) -> String {
    format!("{}{API_HANDLER_SUFFIX}", unquote(browser))
}

fn api_browser_name(handler: &str) -> Option<&str> {
    let handler = unquote(handler);
    let split = handler.len().checked_sub(API_HANDLER_SUFFIX.len())?;
    let (browser, suffix) = handler.split_at_checked(split)?;
    unicase::eq(suffix, API_HANDLER_SUFFIX).then_some(browser)
}

/// Handler ids keep their quotes: `'Имя'`, event ids: `"Событие"`.
fn unquote(name: &str) -> &str {
    for quote in ['\'', '"'] {
        if let Some(inner) = name.strip_prefix(quote).and_then(|n| n.strip_suffix(quote)) {
            return inner;
        }
    }
    name
}

pub(crate) fn same_name(a: &str, b: &str) -> bool {
    unicase::eq(unquote(a), unquote(b))
}

/// `Обработчик="X"` names the function `X` itself or `X_new`.
pub(crate) fn is_extra_handler(function: &str, attribute: &str) -> bool {
    let function = unquote(function);
    same_name(function, attribute)
        || without_extra_suffix(function).is_some_and(|name| same_name(name, attribute))
}

fn without_extra_suffix(function: &str) -> Option<&str> {
    let split = function.len().checked_sub(EXTRA_HANDLER_SUFFIX.len())?;
    let (name, suffix) = function.split_at_checked(split)?;
    unicase::eq(suffix, EXTRA_HANDLER_SUFFIX).then_some(name)
}

fn name_key(name: &str) -> UniCase<&str> {
    UniCase::new(unquote(name))
}

/// Hashes of names as compared by [same_name]; collisions are told apart on lookup.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Key {
    Named(DefinitionKind, u64),
    /// Handlers and functions by the `X` of `Обработчик="X"` they answer to.
    Extra(u64),
}

/// Definitions indexed by name, to resolve many linked symbols without scanning them all
/// for each one; finds the same as [resolve_linked].
pub struct LinkIndex<'a, U, D> {
    state: RandomState,
    entries: HashMap<Key, Vec<(U, &'a D)>>,
}

impl<'a, U: Clone, D: CodeSymbolDefinition> LinkIndex<'a, U, D> {
    pub fn new(definitions: impl IntoIterator<Item = (U, &'a D)>) -> Self {
        let state = RandomState::new();
        let hash = |name: &str| state.hash_one(name_key(name));
        let mut entries: HashMap<Key, Vec<(U, &'a D)>> = HashMap::new();
        for (uri, d) in definitions {
            let kind = d.kind();
            let id = d.id();
            if matches!(
                kind,
                DefinitionKind::Select
                    | DefinitionKind::ApiBrowser
                    | DefinitionKind::Handler
                    | DefinitionKind::HandlerEvent
            ) {
                let key = Key::Named(kind, hash(id));
                entries.entry(key).or_default().push((uri.clone(), d));
            }
            if matches!(kind, DefinitionKind::Handler | DefinitionKind::Function) {
                let id = unquote(id);
                let exact = hash(id);
                entries
                    .entry(Key::Extra(exact))
                    .or_default()
                    .push((uri.clone(), d));
                let stripped = without_extra_suffix(id).map(hash);
                if let Some(stripped) = stripped.filter(|&h| h != exact) {
                    entries
                        .entry(Key::Extra(stripped))
                        .or_default()
                        .push((uri, d));
                }
            }
        }
        Self { state, entries }
    }

    fn get(&self, key: Key) -> impl Iterator<Item = &(U, &'a D)> {
        self.entries.get(&key).into_iter().flatten()
    }

    fn hash(&self, name: &str) -> u64 {
        self.state.hash_one(name_key(name))
    }

    /// Definitions of `kind` named `name`.
    pub fn named(&self, kind: DefinitionKind, name: &str) -> impl Iterator<Item = &'a D> {
        self.get(Key::Named(kind, self.hash(name)))
            .filter(move |(_, d)| same_name(d.id(), name))
            .map(|(_, d)| *d)
    }

    /// `None` for symbols that are not linked by name.
    pub fn resolve(&self, symbol: &Symbol) -> Option<Vec<(U, &'a D)>> {
        let named = |kind, name: &str| -> Vec<_> {
            self.get(Key::Named(kind, self.hash(name)))
                .filter(|(_, d)| same_name(d.id(), name))
                .cloned()
                .collect()
        };
        let found = match symbol {
            Symbol::Select(name) => named(DefinitionKind::Select, name),
            Symbol::ApiBrowser(name) => named(DefinitionKind::ApiBrowser, name),
            Symbol::Handler(name) => named(DefinitionKind::Handler, name),
            Symbol::ExtraHandler(name) => self
                .get(Key::Extra(self.hash(name)))
                .filter(|(_, d)| is_extra_handler(d.id(), name))
                .cloned()
                .collect(),
            Symbol::HandlerEvent { handler, event } => {
                let mut events = named(DefinitionKind::HandlerEvent, event);
                events.retain(|(_, d)| d.container().is_some_and(|h| same_name(h.id(), handler)));
                events
            }
            _ => return None,
        };
        Some(found)
    }
}

/// The resource a handler serves: the API browser `X` for `'X_АПИ'`, otherwise the select
/// of the same name.
pub fn handler_resource(handler: &str) -> Symbol {
    match api_browser_name(handler) {
        Some(browser) => Symbol::ApiBrowser(browser.to_string()),
        None => Symbol::Select(unquote(handler).to_string()),
    }
}

/// Definitions of a resource, handler or handler event with the given name.
/// Links between them are given by [handler_resource] and by the resources themselves.
pub(crate) fn resolve_linked<'a, I, D>(symbol: &Symbol, definitions: I) -> Vec<(&'a Url, &'a D)>
where
    I: IntoIterator<Item = (&'a Url, &'a D)>,
    D: CodeSymbolDefinition + 'a,
{
    let named =
        |kind: DefinitionKind, name: &str, d: &D| d.kind() == kind && same_name(d.id(), name);
    definitions
        .into_iter()
        .filter(|(_, d)| match symbol {
            Symbol::Select(name) => named(DefinitionKind::Select, name, d),
            Symbol::ApiBrowser(name) => named(DefinitionKind::ApiBrowser, name, d),
            Symbol::Handler(name) => named(DefinitionKind::Handler, name, d),
            Symbol::ExtraHandler(name) => {
                matches!(d.kind(), DefinitionKind::Handler | DefinitionKind::Function)
                    && is_extra_handler(d.id(), name)
            }
            Symbol::HandlerEvent { handler, event } => {
                named(DefinitionKind::HandlerEvent, event, d)
                    && d.container().is_some_and(|h| same_name(h.id(), handler))
            }
            _ => false,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Def {
        kind: DefinitionKind,
        id: &'static str,
        handler: Option<&'static str>,
    }

    impl CodeSymbolDefinition for Def {
        fn kind(&self) -> DefinitionKind {
            self.kind
        }
        fn id(&self) -> &str {
            self.id
        }
        fn parameters(&self) -> Option<&str> {
            None
        }
        fn container(&self) -> Option<Self> {
            self.handler.map(|id| def(DefinitionKind::Handler, id))
        }
        fn parent(&self) -> Option<&str> {
            None
        }
        fn can_be_called(&self, _count: usize) -> bool {
            true
        }
        fn call_priority(&self, _another: &Self, _count: usize) -> core::cmp::Ordering {
            core::cmp::Ordering::Equal
        }
    }

    fn def(kind: DefinitionKind, id: &'static str) -> Def {
        Def {
            kind,
            id,
            handler: None,
        }
    }

    fn event(handler: &'static str, id: &'static str) -> Def {
        Def {
            kind: DefinitionKind::HandlerEvent,
            id,
            handler: Some(handler),
        }
    }

    fn workspace() -> Vec<Def> {
        use DefinitionKind::*;
        vec![
            def(Select, "Записи"),
            def(Select, "Пустая"),
            def(Handler, "'Записи'"),
            def(ApiBrowser, "Модуль.Записи"),
            def(Handler, "'Модуль.Записи_АПИ'"),
            event("'Модуль.Записи_АПИ'", "\"ДействиеА\""),
            event("'Записи'", "\"ДействиеА\""),
            def(Handler, "'Одиночный'"),
            def(Function, "Foo_new"),
            def(Function, "FooBar"),
        ]
    }

    /// Resolved both by scanning and by [LinkIndex], which must agree.
    fn resolve(symbol: Symbol) -> Vec<(DefinitionKind, &'static str)> {
        let uri = Url::parse("file:///a.rx").unwrap();
        let defs = workspace();
        let found = |found: Vec<(&Url, &Def)>| -> Vec<_> {
            found.into_iter().map(|(_, d)| (d.kind, d.id)).collect()
        };
        let scanned = found(resolve_linked(&symbol, defs.iter().map(|d| (&uri, d))));
        let index = LinkIndex::new(defs.iter().map(|d| (&uri, d)));
        assert_eq!(
            found(index.resolve(&symbol).unwrap()),
            scanned,
            "{symbol:?}"
        );
        scanned
    }

    #[test]
    fn link_index_keeps_definitions_order_and_skips_other_symbols() {
        use DefinitionKind::*;
        let a = Url::parse("file:///a.hdl").unwrap();
        let b = Url::parse("file:///b.hdl").unwrap();
        let defs = [
            def(Handler, "'Записи'"),
            def(Function, "Записи_NEW"),
            def(Handler, "'ЗАПИСИ'"),
        ];
        let index = LinkIndex::new([(&a, &defs[0]), (&a, &defs[1]), (&b, &defs[2])]);
        let found = |symbol| -> Vec<_> {
            index
                .resolve(&symbol)
                .unwrap()
                .into_iter()
                .map(|(uri, d)| (uri.path(), d.id))
                .collect()
        };
        assert_eq!(
            found(Symbol::Handler("записи".into())),
            [("/a.hdl", "'Записи'"), ("/b.hdl", "'ЗАПИСИ'")]
        );
        assert_eq!(
            found(Symbol::ExtraHandler("'записи'".into())),
            [
                ("/a.hdl", "'Записи'"),
                ("/a.hdl", "Записи_NEW"),
                ("/b.hdl", "'ЗАПИСИ'")
            ]
        );
        assert!(index.resolve(&Symbol::Function("Записи".into())).is_none());
    }

    #[test]
    fn resources_and_handlers_resolve_to_themselves_by_name() {
        use DefinitionKind::*;
        assert_eq!(
            resolve(Symbol::Select("записи".into())),
            [(Select, "Записи")]
        );
        assert_eq!(
            resolve(Symbol::ApiBrowser("Модуль.Записи".into())),
            [(ApiBrowser, "Модуль.Записи")]
        );
        assert_eq!(
            resolve(Symbol::Handler("Одиночный".into())),
            [(Handler, "'Одиночный'")]
        );
        assert!(resolve(Symbol::Select("Нет".into())).is_empty());
    }

    #[test]
    fn extra_handler_is_a_function_or_handler_optionally_with_new_suffix() {
        use DefinitionKind::*;
        assert_eq!(
            resolve(Symbol::ExtraHandler("Foo".into())),
            [(Function, "Foo_new")]
        );
        assert_eq!(
            resolve(Symbol::ExtraHandler("одиночный".into())),
            [(Handler, "'Одиночный'")]
        );
        assert!(resolve(Symbol::ExtraHandler("Card".into())).is_empty());
    }

    #[test]
    fn handler_event_is_looked_up_within_its_handler() {
        let symbol = Symbol::HandlerEvent {
            handler: api_handler_name("Модуль.Записи"),
            event: "ДействиеА".into(),
        };
        assert_eq!(
            resolve(symbol),
            [(DefinitionKind::HandlerEvent, "\"ДействиеА\"")]
        );
    }

    #[test]
    fn handler_serves_api_browser_or_select_of_same_name() {
        assert_eq!(
            handler_resource("'Модуль.Записи_АПИ'"),
            Symbol::ApiBrowser("Модуль.Записи".into())
        );
        assert_eq!(
            handler_resource("'Записи'"),
            Symbol::Select("Записи".into())
        );
    }

    #[test]
    fn api_handler_name_appends_suffix() {
        assert_eq!(api_handler_name("Модуль.Записи"), "Модуль.Записи_АПИ");
    }

    #[test]
    fn api_browser_name_strips_suffix_ignoring_case_and_quotes() {
        assert_eq!(
            api_browser_name("'Модуль.Записи_АПИ'"),
            Some("Модуль.Записи")
        );
        assert_eq!(
            api_browser_name("'Модуль.Записи_апи'"),
            Some("Модуль.Записи")
        );
        assert_eq!(api_browser_name("'Модуль - Записи'"), None);
        assert_eq!(api_browser_name("АПИ"), None);
    }

    #[test]
    fn same_name_ignores_quotes_and_case() {
        assert!(same_name("'Данные'", "данные"));
        assert!(same_name("\"Фильтр\"", "Фильтр"));
        assert!(!same_name("'Данные'", "Данные2"));
    }
}
