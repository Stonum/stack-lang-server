//! `change_document` keeps the cross-file index in step with the edited text,
//! whether the document is open in the editor or only changed on disk.

mod common;

use common::{TempDir, text_document};
use stack_lang_server::workspace::Workspace;
use tower_lsp::lsp_types::{Position, Url, WorkspaceFolder};

async fn warmed_workspace(dir: &TempDir) -> Workspace {
    let workspace = Workspace::new();
    let folder = WorkspaceFolder {
        uri: Url::from_file_path(&dir.0).unwrap(),
        name: "w".to_string(),
    };
    workspace
        .init_with_workspace_folders(Some(vec![folder]))
        .await
        .unwrap();
    workspace.update_semantic_information().await;
    workspace
}

async fn symbol_names(workspace: &Workspace) -> Vec<String> {
    let mut names: Vec<_> = workspace
        .symbol_information("")
        .await
        .expect("Some")
        .into_iter()
        .map(|s| s.name)
        .collect();
    names.sort();
    names
}

#[tokio::test]
async fn changing_an_open_module_updates_the_index() {
    let dir = TempDir::new("change_open_module");
    dir.write("lib.prg", "func Old() {\n}\n");
    let workspace = warmed_workspace(&dir).await;
    let uri = Url::from_file_path(dir.0.join("lib.prg")).unwrap();

    workspace
        .open_document(text_document(uri.clone(), "mlang", "func Old() {\n}\n"))
        .await
        .unwrap();
    assert_eq!(symbol_names(&workspace).await, ["Old"]);

    workspace
        .change_document(text_document(uri, "mlang", "func Fresh() {\n}\n"))
        .await
        .unwrap();
    assert_eq!(symbol_names(&workspace).await, ["Fresh"]);
}

#[tokio::test]
async fn changing_an_unopened_module_updates_the_index_silently() {
    let dir = TempDir::new("change_unopened_module");
    dir.write("lib.hdl", "func Old() {\n}\n");
    let workspace = warmed_workspace(&dir).await;
    let uri = Url::from_file_path(dir.0.join("lib.hdl")).unwrap();

    // diagnostics are only published for open documents
    let diagnostics = workspace
        .change_document(text_document(uri, "mlang", "func Fresh() {\n  x = ;\n}\n"))
        .await
        .unwrap();
    assert!(diagnostics.is_empty(), "{diagnostics:?}");
    assert_eq!(symbol_names(&workspace).await, ["Fresh"]);
}

#[tokio::test]
async fn reports_and_sql_stay_out_of_the_index() {
    let dir = TempDir::new("change_not_indexed");
    dir.write("lib.prg", "func Lib() {\n}\n");
    let workspace = warmed_workspace(&dir).await;

    for (name, text) in [("a.rpt", "func Report() {\n}\n"), ("a.sql", "select 1")] {
        let uri = Url::from_file_path(dir.0.join(name)).unwrap();
        workspace
            .change_document(text_document(uri, "", text))
            .await
            .unwrap();
    }
    assert_eq!(symbol_names(&workspace).await, ["Lib"]);
}

const HDL: &str = "Функция 'Внешний'( выборка, запись, событие )\n{\n   Вернуть 1;\n}\n";

fn rx(select: &str) -> String {
    format!("<?xml version=\"1.1\"?>\n<Resources>\n   {select}\n</Resources>\n")
}

#[tokio::test]
async fn changing_an_unopened_resource_updates_the_index() {
    let dir = TempDir::new("change_unopened_resource");
    dir.write("app.hdl", HDL);
    dir.write(
        "app.rx",
        &rx(r#"<Select Имя="Записи" Обработчик="Внешний"/>"#),
    );
    let workspace = warmed_workspace(&dir).await;

    let hdl = Url::from_file_path(dir.0.join("app.hdl")).unwrap();
    let handler_name = Position::new(0, 10);
    let references = async || {
        workspace
            .references(&hdl, handler_name)
            .await
            .unwrap()
            .expect("Some")
            .len()
    };
    assert_eq!(references().await, 1);

    let rx_uri = Url::from_file_path(dir.0.join("app.rx")).unwrap();
    let diagnostics = workspace
        .change_document(text_document(
            rx_uri,
            "xml",
            &rx(r#"<Select Имя="Записи"/>"#),
        ))
        .await
        .unwrap();
    assert!(diagnostics.is_empty());
    assert_eq!(references().await, 0);
}

#[tokio::test]
async fn diagnostics_after_change_match_a_fresh_open() {
    let dir = TempDir::new("change_diagnostics");
    dir.write("lib.prg", "func Helper(a, b) {\n}\n");
    let workspace = warmed_workspace(&dir).await;
    let uri = Url::from_file_path(dir.0.join("main.prg")).unwrap();

    workspace
        .open_document(text_document(uri.clone(), "mlang", "func Main() {\n}\n"))
        .await
        .unwrap();

    let text = "func Main() {\n  Helper(1);\n  var a = ;\n}\n";
    let changed = workspace
        .change_document(text_document(uri.clone(), "mlang", text))
        .await
        .unwrap();

    let sources: Vec<_> = changed.iter().filter_map(|d| d.source.clone()).collect();
    assert!(sources.contains(&"mlang-parser".to_string()), "{changed:?}");
    assert!(
        changed
            .iter()
            .any(|d| d.message.contains("Helper") && d.message.contains("1 argument")),
        "{changed:?}"
    );

    let reopened = workspace
        .open_document(text_document(uri, "mlang", text))
        .await
        .unwrap();
    assert_eq!(changed, reopened);
}
