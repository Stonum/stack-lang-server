#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlExtensions, SqlFileSource};

#[test]
fn test_default_dialect_is_standard() {
    assert_eq!(SqlFileSource::script().dialect(), SqlDialect::Standard);
}

#[test]
fn test_mlang_extension_is_off_by_default() {
    assert!(!SqlFileSource::script().has_mlang_extension());
}

#[test]
fn test_mlang_extension_flag_round_trips() {
    let source = SqlFileSource::script().with_mlang_extension(true);

    assert!(source.has_mlang_extension());
    assert_eq!(
        source.extensions(),
        SqlExtensions::default().with_mlang(true)
    );
    // The extension is independent of the dialect -- enabling it doesn't
    // implicitly change which dialect is selected.
    assert_eq!(source.dialect(), SqlDialect::Standard);
}

#[test]
fn test_mlang_extension_works_under_any_dialect() {
    // Real-world confirmed: legacy mlang queries that are otherwise
    // ordinary, valid Postgres still use `#temp` tables -- the extension
    // isn't tied to a particular dialect selection.
    let standard = parse(
        "select a from #tmptable",
        SqlFileSource::script().with_mlang_extension(true),
    );
    let postgres = parse(
        "select a from #tmptable",
        SqlFileSource::script()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true),
    );

    assert_parser!(standard);
    assert_parser!(postgres);
}

#[test]
fn test_plain_sql_parses_identically_regardless_of_dialect_or_extension() {
    // Neither the dialect flag nor the mlang extension change anything yet
    // for ordinary Postgres syntax -- they only extend what's
    // *additionally* accepted.
    let standard = parse("select a from t where a = 1", SqlFileSource::script());
    let mlang = parse(
        "select a from t where a = 1",
        SqlFileSource::script()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true),
    );

    assert_parser!(standard);
    assert_parser!(mlang);
}

#[test]
fn test_hash_temp_table_name_with_mlang_extension() {
    let res = parse(
        "select a from #tmptable",
        SqlFileSource::script()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true),
    );

    assert_parser!(res);
}

#[test]
fn test_hash_temp_table_name_rejected_without_mlang_extension() {
    let res = parse("select a from #tmptable", SqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_hash_temp_table_name_in_join_and_column_qualifier() {
    let res = parse(
        "select a from #tmp t join #other o on t.row_id = o.row_id where #tmp.a = 1",
        SqlFileSource::script()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true),
    );

    assert_parser!(res);
}

#[test]
fn test_hash_temp_table_name_in_insert_update_delete() {
    let dialect = || {
        SqlFileSource::script()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    };

    let insert = parse("insert into #tmp (a) values (1)", dialect());
    assert_parser!(insert);

    let update = parse("update #tmp set a = 1", dialect());
    assert_parser!(update);

    let delete = parse("delete from #tmp where a = 1", dialect());
    assert_parser!(delete);
}
