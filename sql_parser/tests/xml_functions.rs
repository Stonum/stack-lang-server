#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, postgres());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_xmlexists() {
    let src = r#"select xmlexists('/Root' PASSING BY REF doc),
    XMLEXISTS(('/Root/' || what) passing _xml),
    xmlexists('(Action[Result = "Ошибка"])' passing by value d by ref),
    xmlexists('/a' passing (select x from t))
from t"#;

    assert_eq!(count(src, "PSQL_XML_EXISTS_EXPRESSION"), 4);
    assert_eq!(count(src, "PSQL_XML_BY_MECHANISM"), 3);
}

#[test]
fn test_xmltable() {
    let src = r#"SELECT xmltable.*
FROM XMLTABLE(XMLNAMESPACES ('http://example.com/format' AS n), '/n:FORMAT/n:RECORD/n:FIELD'
    PASSING (xmldata)
    COLUMNS ID int PATH '@ID', TERMINATOR text PATH '@TERMINATOR', ord for ordinality,
        "Поле" varchar(10) default 'x' not null) x
join t on t.id = x.ID"#;

    assert_eq!(count(src, "PSQL_XML_TABLE_BINDING"), 1);
    assert_eq!(count(src, "PSQL_XML_NAMESPACE"), 1);
    assert_eq!(count(src, "PSQL_XML_TABLE_VALUE_COLUMN"), 3);
    assert_eq!(count(src, "PSQL_XML_TABLE_ORDINALITY_COLUMN"), 1);
}

#[test]
fn test_xml_names_stay_ordinary_names() {
    let res = parse(
        "select xmltable.*, xmlexists(x), passing, columns, path, ref from xmltable",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_broken_xml_functions_report_errors() {
    for src in [
        "select xmlexists('/a' passing) from t",
        "select xmlexists('/a' passing by doc) from t",
        "select * from xmltable('/a' passing d)",
        "select * from xmltable('/a' passing d columns)",
        "select * from xmltable('/a' columns a int)",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
