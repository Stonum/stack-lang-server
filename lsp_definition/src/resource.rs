//! Name-based links between rx resources and `.hdl` handlers.

use tower_lsp::lsp_types::Url;

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
    if same_name(function, attribute) {
        return true;
    }
    let Some(split) = function.len().checked_sub(EXTRA_HANDLER_SUFFIX.len()) else {
        return false;
    };
    function
        .split_at_checked(split)
        .is_some_and(|(name, suffix)| {
            unicase::eq(suffix, EXTRA_HANDLER_SUFFIX) && same_name(name, attribute)
        })
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
pub(crate) fn resolve_linked<'a, I, D>(symbol: &Symbol, definitions: I) -> Vec<(Url, &'a D)>
where
    I: IntoIterator<Item = (Url, &'a D)>,
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

    fn resolve(symbol: Symbol) -> Vec<(DefinitionKind, &'static str)> {
        let uri = Url::parse("file:///a.rx").unwrap();
        let defs = workspace();
        resolve_linked(&symbol, defs.iter().map(|d| (uri.clone(), d)))
            .into_iter()
            .map(|(_, d)| (d.kind, d.id))
            .collect()
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
