#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, mlang());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_tilde_subscripts_in_expressions() {
    let src = r#"select coalesce(captions~[i]~, ''), (xpath('/a/@b', doc))~[1]~::text,
    f(args~[1]~, args~[2]~), m~[1]~
from t
where tg_argv~[0]~ is not null and a~[i]~ is distinct from b~[i]~ and args~[3]~ = 'true'"#;

    assert_eq!(count(src, "PSQL_TILDE_ARRAY_SUBSCRIPT_EXPRESSION"), 9);
}

#[test]
fn test_tilde_subscript_assignment_targets() {
    let src = r#"do $$
begin
    infoFields~[0]~ = 'row_guid';
    "Интервал"~[i]~ := i;
    headers~[rec.id]~ = rec.name;
    file_path = match~[1]~;
end $$"#;

    assert_eq!(count(src, "PSQL_PL_ASSIGNMENT_STATEMENT"), 4);
    assert_eq!(count(src, "PSQL_TILDE_ARRAY_SUBSCRIPT_EXPRESSION"), 4);
}

#[test]
fn test_tilde_is_still_the_regex_operator() {
    let src = "select a ~ 'x', a ~* b from t where a ~ b";

    assert_eq!(count(src, "PSQL_TILDE_ARRAY_SUBSCRIPT_EXPRESSION"), 0);
}

#[test]
fn test_broken_tilde_subscripts_report_errors() {
    for src in ["select a~[]~ from t", "select a~[(1]~ from t"] {
        let res = parse(src, mlang());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_regex_against_bracket_name_is_not_a_subscript() {
    let src = "select (regexp_match(q, 'x'))~[~1]::jsonb from t";

    assert_eq!(count(src, "PSQL_TILDE_ARRAY_SUBSCRIPT_EXPRESSION"), 0);
}
