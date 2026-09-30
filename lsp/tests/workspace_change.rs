//! `change_document` keeps the cross-file index in step with the edited text,
//! whether the document is open in the editor or only changed on disk.

mod common;

use common::{TempDir, published, text_document};
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
    assert!(diagnostics.is_none());
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
    assert!(diagnostics.is_none());
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
    let changed = published(changed);

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

    let reopened = published(reopened);
    assert_eq!(changed, reopened);
}

fn versioned(uri: &Url, version: i32, text: &str) -> tower_lsp::lsp_types::TextDocumentItem {
    let mut document = text_document(uri.clone(), "mlang", text);
    document.version = version;
    document
}

/// Clean for every version but the last, which calls `Helper` with too few arguments.
fn typed(version: i32, last: i32) -> String {
    let args = if version == last { "1" } else { "1, 2" };
    format!("func Main{version}() {{\n  Helper({args});\n}}\n")
}

#[tokio::test]
async fn fast_typing_publishes_only_the_last_version() {
    let dir = TempDir::new("change_fast_typing");
    dir.write("lib.prg", "func Helper(a, b) {\n}\n");
    let workspace = warmed_workspace(&dir).await;
    let uri = Url::from_file_path(dir.0.join("main.prg")).unwrap();
    workspace
        .open_document(versioned(&uri, 1, &typed(1, 0)))
        .await
        .unwrap();

    // every change arrives before the first one is linted
    let last = 20;
    let changes =
        (2..=last).map(|v| workspace.change_document(versioned(&uri, v, &typed(v, last))));
    let results = futures::future::join_all(changes).await;

    let published: Vec<_> = results.into_iter().filter_map(Result::unwrap).collect();
    let versions: Vec<_> = published.iter().map(|d| d.version).collect();
    assert_eq!(versions, [last]);
    let diagnostics = &published[0].diagnostics;
    assert!(
        diagnostics.iter().any(|d| d.message.contains("1 argument")),
        "{diagnostics:?}"
    );
    drop(published);

    // the index holds the last text too
    assert_eq!(symbol_names(&workspace).await, ["Helper", "Main20"]);
    let symbols = workspace.document_symbol_response(&uri).await.unwrap();
    assert!(format!("{symbols:?}").contains(&format!("Main{last}")));
}

#[tokio::test]
async fn a_superseded_change_is_applied_but_not_linted() {
    let dir = TempDir::new("change_superseded");
    dir.write("lib.prg", "func Helper(a, b) {\n}\n");
    let workspace = warmed_workspace(&dir).await;
    let uri = Url::from_file_path(dir.0.join("main.prg")).unwrap();
    workspace
        .open_document(versioned(&uri, 1, "func Main() {\n}\n"))
        .await
        .unwrap();

    let first = versioned(&uri, 2, "func First() {\n  Helper(1);\n}\n");
    let second = versioned(&uri, 3, "func Second() {\n}\n");
    let (first, second) = tokio::join!(
        workspace.change_document(first),
        workspace.change_document(second)
    );
    assert!(first.unwrap().is_none());
    let second = second.unwrap().expect("the last version is published");
    assert_eq!(second.version, 3);
    assert!(second.diagnostics.is_empty(), "{:?}", second.diagnostics);
}

#[tokio::test]
async fn an_older_version_does_not_replace_a_newer_one() {
    let uri = common::temp_uri("change_older_version.prg");
    let workspace = Workspace::new();
    workspace
        .open_document(versioned(&uri, 1, "func Main() {\n}\n"))
        .await
        .unwrap();

    let newer = workspace
        .change_document(versioned(&uri, 3, "func Newer() {\n}\n"))
        .await
        .unwrap();
    assert_eq!(newer.map(|d| d.version), Some(3));
    let older = workspace
        .change_document(versioned(&uri, 2, "func Older() {\n  x = ;\n}\n"))
        .await
        .unwrap();
    assert!(older.is_none());

    let symbols = workspace.document_symbol_response(&uri).await.unwrap();
    let symbols = format!("{symbols:?}");
    assert!(
        symbols.contains("Newer") && !symbols.contains("Older"),
        "{symbols}"
    );
}

#[tokio::test]
async fn a_reload_from_disk_keeps_the_version() {
    let uri = common::temp_uri("change_reload_from_disk.prg");
    let workspace = Workspace::new();
    workspace
        .open_document(versioned(&uri, 4, "func Main() {\n}\n"))
        .await
        .unwrap();

    let reloaded = workspace
        .reload_document(uri.clone(), "func Main() {\n  x = ;\n}\n".to_string())
        .await
        .unwrap()
        .expect("an opened document is linted");
    assert_eq!(reloaded.version, 4);
    assert!(!reloaded.diagnostics.is_empty());
}

#[tokio::test]
async fn nothing_is_published_for_a_document_closed_while_changing() {
    let uri = common::temp_uri("change_closed_while_changing.prg");
    let workspace = Workspace::new();
    workspace
        .open_document(versioned(&uri, 1, "func Main() {\n}\n"))
        .await
        .unwrap();

    let (changed, ()) = tokio::join!(
        workspace.change_document(versioned(&uri, 2, "func Main() {\n  x = ;\n}\n")),
        workspace.close_document(&uri)
    );
    assert!(changed.unwrap().is_none());
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn racing_changes_end_with_the_last_version() {
    let dir = TempDir::new("change_racing");
    dir.write("lib.prg", "func Helper(a, b) {\n}\n");
    let workspace = std::sync::Arc::new(warmed_workspace(&dir).await);
    let uri = Url::from_file_path(dir.0.join("main.prg")).unwrap();
    workspace
        .open_document(versioned(&uri, 1, &typed(1, 0)))
        .await
        .unwrap();

    let last = 40;
    let tasks: Vec<_> = (2..=last)
        .map(|v| {
            let workspace = std::sync::Arc::clone(&workspace);
            let document = versioned(&uri, v, &typed(v, last));
            // a publish holds its document's order until dropped
            tokio::spawn(async move {
                let published = workspace.change_document(document).await.unwrap();
                published.map(|d| (d.version, d.diagnostics))
            })
        })
        .collect();
    let mut published = vec![];
    for task in tasks {
        published.extend(task.await.unwrap());
    }

    let (version, diagnostics) = published.iter().max_by_key(|(v, _)| *v).unwrap();
    assert_eq!(*version, last);
    assert!(
        diagnostics.iter().any(|d| d.message.contains("1 argument")),
        "{diagnostics:?}"
    );
    assert_eq!(symbol_names(&workspace).await, ["Helper", "Main40"]);
}
