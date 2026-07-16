#[macro_use]
mod helper;

use psql_parser::parse;
use psql_syntax::{PsqlDialect, PsqlFileSource};

#[test]
fn test_default_dialect_is_standard() {
    assert_eq!(PsqlFileSource::script().dialect(), PsqlDialect::Standard);
}

#[test]
fn test_mlang_dialect_flag_round_trips() {
    let source = PsqlFileSource::script().with_dialect(PsqlDialect::Mlang);

    assert_eq!(source.dialect(), PsqlDialect::Mlang);
    assert!(source.is_mlang_dialect());
}

#[test]
fn test_plain_sql_parses_identically_regardless_of_dialect() {
    // The dialect flag doesn't change anything yet for ordinary Postgres
    // syntax -- it only extends what's *additionally* accepted.
    let standard = parse("select a from t where a = 1", PsqlFileSource::script());
    let mlang = parse(
        "select a from t where a = 1",
        PsqlFileSource::script().with_dialect(PsqlDialect::Mlang),
    );

    assert_parser!(standard);
    assert_parser!(mlang);
}

#[test]
fn test_hash_temp_table_name_in_mlang_dialect() {
    let res = parse(
        "select a from #tmptable",
        PsqlFileSource::script().with_dialect(PsqlDialect::Mlang),
    );

    assert_parser!(res);
}

#[test]
fn test_hash_temp_table_name_rejected_in_standard_dialect() {
    let res = parse("select a from #tmptable", PsqlFileSource::script());

    assert!(res.has_errors());
}

#[test]
fn test_hash_temp_table_name_in_join_and_column_qualifier() {
    let res = parse(
        "select a from #tmp t join #other o on t.row_id = o.row_id where #tmp.a = 1",
        PsqlFileSource::script().with_dialect(PsqlDialect::Mlang),
    );

    assert_parser!(res);
}

#[test]
fn test_hash_temp_table_name_in_insert_update_delete() {
    let dialect = || PsqlFileSource::script().with_dialect(PsqlDialect::Mlang);

    let insert = parse("insert into #tmp (a) values (1)", dialect());
    assert_parser!(insert);

    let update = parse("update #tmp set a = 1", dialect());
    assert_parser!(update);

    let delete = parse("delete from #tmp where a = 1", dialect());
    assert_parser!(delete);
}
