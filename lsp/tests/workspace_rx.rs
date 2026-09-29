//! Navigation between rx resources (`<Select>`, `<APIBrowser>`) and `.hdl` handlers.

mod common;

use common::TempDir;
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::{
    GotoDefinitionResponse, HoverContents, Location, MarkedString, Position, Url, WorkspaceFolder,
};

const RX: &str = r#"<?xml version="1.1"?>
<Resources>
   <Select Имя="Записи" Обработчик="Внешний"/>
   <Select Имя="Пустая"/>
   <APIBrowser Имя="Модуль.Записи" Имя_выборки="Записи" ДопМетоды="ДействиеА"/>
</Resources>
"#;

const MORE_HDL: &str = r#"Функция 'Записи'( Событие )
{
   Вернуть 2;
}
"#;

const ORPHAN_RX: &str = r#"<?xml version="1.1"?>
<Resources>
   <APIBrowser Имя="Модуль.Нет" Имя_выборки="Нет"/>
</Resources>
"#;

const CARD_RX: &str = r#"<?xml version="1.1"?>
<Resources>
   <Select Имя="Другие" Обработчик="Foo"/>
</Resources>
"#;

const CARD_PRG: &str = r#"Функция Foo_new(...args)
{
   Вернуть 1;
}
"#;

const MULTILINE_RX: &str = r#"<?xml version="1.1"?>
<Resources>
   <APIBrowser
      Имя="Модуль.Записи"
      Имя_выборки="Записи"
   />
</Resources>
"#;

const EXPR_RX: &str = r#"<?xml version="1.1"?>
<Resources>
   <APIBrowser Имя="Модуль.Вызов" Выражение="Вычислить(&apos;x&apos;, Другая())"/>
   <APIBrowser Имя="Модуль.Имя" Выражение="Состояние"/>
   <APIBrowser Имя="Модуль.Объект" Выражение="@{ поле: Третья() }"/>
   <Поле Имя="Поле" Выражение="Четвертая()"/>
</Resources>
"#;

const EXPR_PRG: &str = r#"func Вычислить(a) {
}
func Вычислить(a, b) {
}
func Другая() {
}
func Третья() {
}
func Состояние() {
}
func Четвертая() {
}
"#;

const HDL: &str = r#"Функция 'Записи'( Событие )
{
   Вернуть 1;
}

Функция 'Модуль.Записи_АПИ'( Событие )
{
   ВыборПо( Событие )
   {
      Выбор "ДействиеА":
         Вернуть 1;
   }
}

Функция 'Внешний'( выборка, запись, событие )
{
   Вернуть 1;
}
"#;

struct Fixture {
    dir: TempDir,
    workspace: Workspace,
}

impl Fixture {
    /// Programs and resources are found through `PRG=` and `RS=` of `stack.ini`.
    async fn new(tag: &str) -> Self {
        Self::with(tag, &[]).await
    }

    async fn with(tag: &str, extra: &[(&str, &str)]) -> Self {
        let dir = TempDir::new(&format!("rx_{tag}"));
        dir.write("rx/app.rx", RX);
        dir.write("prg/app.hdl", HDL);
        for (file, text) in extra {
            dir.write(file, text);
        }
        let ini = format!(
            "[AppPath]\nPRG={}\nRS={}\n",
            dir.0.join("prg").display(),
            dir.0.join("rx").display()
        );
        dir.write("stack.ini", &ini);

        let workspace = Workspace::new();
        workspace
            .init_with_settings_file(&dir.0.to_string_lossy())
            .await
            .expect("ini init");
        workspace.update_semantic_information().await;

        Fixture { dir, workspace }
    }

    fn uri(&self, file: &str) -> Url {
        Url::from_file_path(self.dir.0.join(file)).unwrap()
    }

    /// Position of `$` inside `pattern`, which must occur once in `file`.
    fn position(file: &str, pattern: &str) -> Position {
        let text = source(file);
        let cursor = pattern.find('$').expect("pattern needs a `$` cursor");
        let needle = pattern.replace('$', "");
        assert_eq!(
            text.matches(&needle).count(),
            1,
            "`{needle}` must occur once"
        );
        let offset = text.find(&needle).unwrap() + cursor;
        let line = text[..offset].matches('\n').count();
        let line_start = text[..offset].rfind('\n').map_or(0, |i| i + 1);
        let col = text[line_start..offset].encode_utf16().count();
        Position::new(line as u32, col as u32)
    }

    async fn goto(&self, file: &str, pattern: &str) -> Vec<String> {
        let response = self
            .workspace
            .goto_definition(&self.uri(file), Self::position(file, pattern))
            .await
            .expect("goto")
            .expect("Some");
        let GotoDefinitionResponse::Array(locations) = response else {
            panic!("expected an array response");
        };
        self.texts(&locations)
    }

    async fn references(&self, file: &str, pattern: &str) -> Vec<String> {
        let locations = self
            .workspace
            .references(&self.uri(file), Self::position(file, pattern))
            .await
            .expect("references")
            .expect("Some");
        self.texts(&locations)
    }

    async fn hover(&self, file: &str, pattern: &str) -> Vec<String> {
        let hover = self
            .workspace
            .hover(&self.uri(file), Self::position(file, pattern))
            .await
            .expect("hover")
            .expect("Some");
        let HoverContents::Array(markups) = hover.contents else {
            panic!("expected an array of markups");
        };
        markups
            .into_iter()
            .map(|m| match m {
                MarkedString::String(s) => s,
                MarkedString::LanguageString(s) => s.value,
            })
            .collect()
    }

    /// `(line, title, targets)` of each lens; targets as `file:text`.
    async fn lenses(&self, file: &str) -> Vec<(u32, String, Vec<String>)> {
        let lenses = self
            .workspace
            .code_lens(&self.uri(file))
            .await
            .expect("code lens")
            .expect("Some");
        lenses
            .into_iter()
            // class member lenses of mlang documents are not about resources
            .filter(|lens| {
                lens.command
                    .as_ref()
                    .is_some_and(|c| c.command != "stack.movetoLine")
            })
            .map(|lens| {
                let command = lens.command.expect("a command");
                let Some(arguments) = command.arguments else {
                    // nothing found: a plain-text lens
                    assert_eq!(command.command, "");
                    return (lens.range.start.line, command.title, vec![]);
                };
                assert_eq!(command.command, "stack.goToLocations");
                let uri: Url = serde_json::from_value(arguments[0].clone()).unwrap();
                let position: Position = serde_json::from_value(arguments[1].clone()).unwrap();
                let locations: Vec<Location> =
                    serde_json::from_value(arguments[2].clone()).unwrap();
                assert_eq!((uri, position), (self.uri(file), lens.range.start));
                (lens.range.start.line, command.title, self.texts(&locations))
            })
            .collect()
    }

    /// `file:text` of each location.
    fn texts(&self, locations: &[Location]) -> Vec<String> {
        locations
            .iter()
            .map(|location| {
                let path = location.uri.to_file_path().unwrap();
                let file = path.strip_prefix(&self.dir.0).unwrap().to_string_lossy();
                let file = file.replace('\\', "/");
                let range = location.range;
                assert_eq!(range.start.line, range.end.line);
                let line = source(&file)
                    .lines()
                    .nth(range.start.line as usize)
                    .unwrap();
                let units: Vec<u16> = line.encode_utf16().collect();
                let text = &units[range.start.character as usize..range.end.character as usize];
                format!("{file}:{}", String::from_utf16(text).unwrap())
            })
            .collect()
    }
}

fn source(file: &str) -> &'static str {
    match file {
        "rx/app.rx" => RX,
        "rx/orphan.rx" => ORPHAN_RX,
        "prg/app.hdl" => HDL,
        "prg/more.hdl" => MORE_HDL,
        "rx/card.rx" => CARD_RX,
        "rx/multiline.rx" => MULTILINE_RX,
        "rx/expr.rx" => EXPR_RX,
        "prg/expr.prg" => EXPR_PRG,
        "prg/card.prg" => CARD_PRG,
        _ => panic!("unknown file {file}"),
    }
}

#[tokio::test]
async fn select_name_of_api_browser_goes_to_select() {
    let f = Fixture::new("browser_select").await;
    assert_eq!(
        f.goto("rx/app.rx", r#"Имя_выборки="З$аписи""#).await,
        [r#"rx/app.rx:Записи"#]
    );
}

#[tokio::test]
async fn declarations_go_to_themselves() {
    let f = Fixture::new("declarations").await;
    assert_eq!(
        f.goto("rx/app.rx", r#"Имя="М$одуль.Записи""#).await,
        ["rx/app.rx:Модуль.Записи"]
    );
    assert_eq!(
        f.goto("rx/app.rx", r#"Имя="З$аписи" Обработчик"#).await,
        ["rx/app.rx:Записи"]
    );
    assert_eq!(
        f.goto("rx/app.rx", r#"Имя="П$устая""#).await,
        ["rx/app.rx:Пустая"]
    );
    assert_eq!(
        f.goto("prg/app.hdl", "'М$одуль.Записи_АПИ'").await,
        ["prg/app.hdl:'Модуль.Записи_АПИ'"]
    );
}

#[tokio::test]
async fn handler_attribute_goes_to_handler() {
    let f = Fixture::new("handler_attribute").await;
    assert_eq!(
        f.goto("rx/app.rx", r#"Обработчик="В$нешний""#).await,
        ["prg/app.hdl:'Внешний'"]
    );
}

#[tokio::test]
async fn extra_method_goes_to_api_handler_event() {
    let f = Fixture::new("extra_method").await;
    assert_eq!(
        f.goto("rx/app.rx", r#"ДопМетоды="Д$ействиеА""#).await,
        [r#"prg/app.hdl:"ДействиеА""#]
    );
}

#[tokio::test]
async fn references_from_handlers_and_selects_find_resource_attributes() {
    let f = Fixture::new("references").await;
    assert_eq!(
        f.references("prg/app.hdl", "'В$нешний'").await,
        ["rx/app.rx:Внешний"]
    );
    assert_eq!(
        f.references("prg/app.hdl", r#"Выбор "Д$ействиеА""#).await,
        ["rx/app.rx:ДействиеА"]
    );
    assert_eq!(
        f.references("rx/app.rx", r#"Имя="З$аписи" Обработчик"#)
            .await,
        ["rx/app.rx:Записи"]
    );
}

/// `file:line` of the declaration link in each markup.
fn declared_at(markups: &[String]) -> Vec<String> {
    markups
        .iter()
        .map(|m| {
            let end = m.rfind("](file").expect("a declaration link");
            let start = m[..end].rfind('[').unwrap();
            m[start + 1..end].to_string()
        })
        .collect()
}

#[tokio::test]
async fn handler_hover_shows_only_same_named_handlers() {
    let f = Fixture::with("hover_handler", &[("prg/more.hdl", MORE_HDL)]).await;
    let hdl = f.uri("prg/app.hdl");

    let markups = f.hover("prg/app.hdl", "'М$одуль.Записи_АПИ'").await;
    assert_eq!(declared_at(&markups), ["prg/app.hdl:6"]);
    assert!(markups[0].contains(&format!("[prg/app.hdl:6]({hdl}#L6)")));

    let mut declared = declared_at(&f.hover("prg/app.hdl", "'З$аписи'(").await);
    declared.sort();
    assert_eq!(declared, ["prg/app.hdl:1", "prg/more.hdl:1"]);
}

#[tokio::test]
async fn handler_lenses_lead_to_their_resources() {
    let f = Fixture::new("handler_lens").await;
    let target = |t: &str| vec![t.to_string()];
    assert_eq!(
        f.lenses("prg/app.hdl").await,
        [
            (0, "Select: Записи".into(), target("rx/app.rx:Записи")),
            (
                5,
                "API browser: Модуль.Записи".into(),
                target("rx/app.rx:Модуль.Записи")
            ),
            (5, "Select: Записи".into(), target("rx/app.rx:Записи")),
            (14, "Select: Внешний (not found)".into(), vec![]),
        ]
    );
}

#[tokio::test]
async fn resource_hover_shows_only_the_resource() {
    let f = Fixture::new("hover_resource").await;
    // handlers are left to lenses
    assert_eq!(
        declared_at(&f.hover("rx/app.rx", r#"Имя="М$одуль.Записи""#).await),
        ["rx/app.rx:5"]
    );
    assert_eq!(
        declared_at(&f.hover("rx/app.rx", r#"Имя="З$аписи" Обработчик"#).await),
        ["rx/app.rx:3"]
    );
    assert_eq!(
        declared_at(&f.hover("rx/app.rx", r#"Имя_выборки="З$аписи""#).await),
        ["rx/app.rx:3"]
    );
}

#[tokio::test]
async fn lenses_lead_from_resources_to_their_handlers() {
    let f = Fixture::new("lens").await;
    let target = |t: &str| vec![t.to_string()];
    assert_eq!(
        f.lenses("rx/app.rx").await,
        [
            (2, "Handler: Записи".into(), target("prg/app.hdl:'Записи'")),
            (
                2,
                "Handler: Внешний".into(),
                target("prg/app.hdl:'Внешний'")
            ),
            (3, "Handler: Пустая (not found)".into(), vec![]),
            (
                4,
                "Handler: Модуль.Записи_АПИ".into(),
                target("prg/app.hdl:'Модуль.Записи_АПИ'")
            ),
            (
                4,
                "Select handler: Записи".into(),
                target("prg/app.hdl:'Записи'")
            ),
            (
                4,
                "Select handler: Внешний".into(),
                target("prg/app.hdl:'Внешний'")
            ),
        ]
    );
}

#[tokio::test]
async fn lens_counts_same_named_targets() {
    let f = Fixture::with("lens_count", &[("prg/more.hdl", MORE_HDL)]).await;
    let (_, title, mut targets) = f.lenses("rx/app.rx").await.swap_remove(0);
    targets.sort();
    assert_eq!(title, "Handler: Записи (2)");
    assert_eq!(targets, ["prg/app.hdl:'Записи'", "prg/more.hdl:'Записи'"]);
}

#[tokio::test]
async fn lenses_tell_what_is_not_found() {
    let f = Fixture::with("lens_missing", &[("rx/orphan.rx", ORPHAN_RX)]).await;
    let titles: Vec<_> = f
        .lenses("rx/orphan.rx")
        .await
        .into_iter()
        .map(|(_, title, targets)| (title, targets.len()))
        .collect();
    assert_eq!(
        titles,
        [
            ("Handler: Модуль.Нет_АПИ (not found)".to_string(), 0),
            ("Select handler: Нет (not found)".to_string(), 0),
        ]
    );
}

#[tokio::test]
async fn handler_attribute_may_name_a_function_with_new_suffix() {
    let f = Fixture::with(
        "new_suffix",
        &[("rx/card.rx", CARD_RX), ("prg/card.prg", CARD_PRG)],
    )
    .await;
    assert_eq!(
        f.goto("rx/card.rx", r#"Обработчик="F$oo""#).await,
        ["prg/card.prg:Foo_new"]
    );
    assert_eq!(
        f.references("prg/card.prg", "F$oo_new").await,
        ["rx/card.rx:Foo"]
    );
    let lenses = f.lenses("rx/card.rx").await;
    assert_eq!(
        lenses[1],
        (
            2,
            "Handler: Foo".into(),
            vec!["prg/card.prg:Foo_new".into()]
        )
    );
}

#[tokio::test]
async fn lenses_stay_above_the_tag_when_attributes_are_wrapped() {
    let f = Fixture::with("lens_multiline", &[("rx/multiline.rx", MULTILINE_RX)]).await;
    let lines: Vec<u32> = f
        .lenses("rx/multiline.rx")
        .await
        .into_iter()
        .map(|(line, _, _)| line)
        .collect();
    assert!(!lines.is_empty());
    assert!(lines.iter().all(|&line| line == 2), "{lines:?}");
}

#[tokio::test]
async fn hover_paths_are_relative_to_the_editor_workspace_folder() {
    // `stack.ini` also points outside the project, so the common ancestor is too high
    let outside = TempDir::new("rx_roots_outside");
    outside.write("lib.prg", "func Lib() {\n}\n");
    let dir = TempDir::new("rx_roots");
    dir.write("rx/app.rx", RX);
    dir.write("prg/app.hdl", HDL);
    let ini = format!(
        "[AppPath]\nPRG={}\nPRG={}\nRS={}\n",
        dir.0.join("prg").display(),
        outside.0.display(),
        dir.0.join("rx").display()
    );
    dir.write("stack.ini", &ini);

    let workspace = Workspace::new();
    let folder = WorkspaceFolder {
        uri: Url::from_file_path(&dir.0).unwrap(),
        name: "project".into(),
    };
    workspace.set_workspace_folders(Some(&[folder]));
    workspace
        .init_with_settings_file(&dir.0.to_string_lossy())
        .await
        .expect("ini init");
    workspace.update_semantic_information().await;

    let f = Fixture { dir, workspace };
    assert_eq!(
        declared_at(&f.hover("rx/app.rx", r#"Имя="М$одуль.Записи""#).await),
        ["rx/app.rx:5"]
    );
    drop(outside);
}

#[tokio::test]
async fn browser_expression_goes_to_mlang_functions() {
    let f = Fixture::with(
        "expression",
        &[("rx/expr.rx", EXPR_RX), ("prg/expr.prg", EXPR_PRG)],
    )
    .await;
    // the overload taking two arguments
    let calls = f.goto("rx/expr.rx", "Выч$ислить(").await;
    assert_eq!(calls, ["prg/expr.prg:Вычислить"]);
    // after the decoded `&apos;` entities
    assert_eq!(
        f.goto("rx/expr.rx", "Др$угая").await,
        ["prg/expr.prg:Другая"]
    );
    // a bare name is a function
    assert_eq!(
        f.goto("rx/expr.rx", "Сост$ояние").await,
        ["prg/expr.prg:Состояние"]
    );
    // inside an object literal
    assert_eq!(
        f.goto("rx/expr.rx", "Тре$тья").await,
        ["prg/expr.prg:Третья"]
    );
}

#[tokio::test]
async fn field_expression_is_sql_and_is_not_resolved() {
    let f = Fixture::with(
        "field_expression",
        &[("rx/expr.rx", EXPR_RX), ("prg/expr.prg", EXPR_PRG)],
    )
    .await;
    let position = Fixture::position("rx/expr.rx", "Четв$ертая");
    let response = f
        .workspace
        .goto_definition(&f.uri("rx/expr.rx"), position)
        .await
        .expect("goto");
    assert!(response.is_none(), "{response:?}");
}

#[tokio::test]
async fn expression_hover_is_the_same_as_in_programs() {
    const CALLER: &str = "func Вызывающая() {\n   Вычислить(1, 2);\n}\n";
    let f = Fixture::with(
        "expression_hover",
        &[
            ("rx/expr.rx", EXPR_RX),
            ("prg/expr.prg", EXPR_PRG),
            ("prg/caller.prg", CALLER),
        ],
    )
    .await;
    let in_program = f
        .workspace
        .hover(&f.uri("prg/caller.prg"), Position::new(1, 5))
        .await
        .expect("hover")
        .expect("Some");
    let in_resource = f.hover("rx/expr.rx", "Выч$ислить(").await;

    let HoverContents::Array(in_program) = in_program.contents else {
        panic!("expected an array of markups");
    };
    let in_program: Vec<_> = in_program
        .into_iter()
        .map(|m| match m {
            MarkedString::String(s) => s,
            MarkedString::LanguageString(s) => s.value,
        })
        .collect();
    assert_eq!(in_resource, in_program);
    // highlighted as mlang in any document
    assert!(in_resource[0].starts_with("```stack\n"), "{in_resource:?}");
}
