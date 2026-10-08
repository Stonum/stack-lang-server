#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

/// `boolean` is a Postgres-only type keyword.
fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

#[test]
fn test_create_table_single_column() {
    let res = parse("create table foo (a int)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_multiple_columns() {
    let res = parse("create table foo (a int, b text, c boolean)", postgres());

    assert_parser!(res);
}

#[test]
fn test_create_table_column_named_full() {
    // `FULL` isn't fully reserved in real Postgres -- it can be used as a
    // plain column name (only its meaning in `FROM ... FULL JOIN`/bare-alias
    // position is special).
    let res = parse("create table foo (full text)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_column_named_after_type_keyword() {
    // Same class of collision as `full` above, for a type-name keyword
    // instead (`date timestamp, text_col text` -- both `date` and `text`
    // are used as ordinary column names, followed by their actual type).
    let res = parse(
        "create table foo (date timestamp, text_col text)",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_if_not_exists() {
    let res = parse(
        "create table if not exists foo (a int);",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_qualified_name() {
    let res = parse("create table myschema.foo (a int)", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_create_table_typed_column_with_arguments() {
    let res = parse(
        "create table foo (a numeric(10, 2))",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_followed_by_another_statement() {
    let res = parse(
        "create table foo (a int); select a from foo;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_tilde_name_in_mlang_dialect() {
    let res = parse("create table ~Договор~ (a int)", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_table_do_not_shadow_each_other() {
    let res = parse(
        "drop table if exists foo; create table foo (a int);",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_null_and_not_null_columns() {
    let res = parse(
        "create table #t (a int not null, b varchar(256) null, c int)",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_NOT_NULL_CONSTRAINT"));
    assert!(tree.contains("SQL_NULL_CONSTRAINT"));
}

#[test]
fn test_create_table_default_values() {
    let res = parse(
        "create table t (a int default 0 not null, b timestamp default now(), c text not null default 'x' || 'y')",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_DEFAULT_CONSTRAINT").count(), 3);
}

#[test]
fn test_create_table_default_without_value_reports_error() {
    let res = parse("create table t (a int default)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_dangling_not_reports_error() {
    let res = parse("create table t (a int not)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_key_constraints() {
    let res = parse(
        "create table t (id int primary key, code text unique not null, parent int references p (id))",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_PRIMARY_KEY_CONSTRAINT"));
    assert!(tree.contains("SQL_UNIQUE_CONSTRAINT"));
    assert!(tree.contains("SQL_REFERENCES_CONSTRAINT"));
}

#[test]
fn test_create_table_named_check_constraint() {
    let res = parse(
        "create table t (a int constraint a_positive check (a > 0), b int references ~Таблица~)",
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CONSTRAINT_NAME"));
    assert!(tree.contains("SQL_CHECK_CONSTRAINT"));
}

#[test]
fn test_create_table_primary_without_key_reports_error() {
    let res = parse("create table t (a int primary)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_check_without_parentheses_reports_error() {
    let res = parse("create table t (a int check a > 0)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_table_level_constraints() {
    let res = parse(
        "create table t (a int, b int, c int, primary key (a, b), unique (c), check (a < b), foreign key (c) references p (id))",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_TABLE_CONSTRAINT@").count(), 4);
    assert!(tree.contains("SQL_PRIMARY_KEY_TABLE_CONSTRAINT"));
    assert!(tree.contains("SQL_UNIQUE_TABLE_CONSTRAINT"));
    assert!(tree.contains("SQL_FOREIGN_KEY_TABLE_CONSTRAINT"));
}

#[test]
fn test_create_table_named_table_constraint() {
    let res = parse(
        r#"create table #t ("Ключ" varchar(256), "id" int, constraint pk_t primary key ("Ключ", "id"))"#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CONSTRAINT_NAME"));
}

#[test]
fn test_create_table_constraint_name_without_constraint_reports_error() {
    let res = parse("create table t (a int, constraint c)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_primary_key_without_columns_reports_error() {
    let res = parse("create table t (a int, primary key)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_foreign_key_without_references_reports_error() {
    let res = parse("create table t (a int, foreign key (a))", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_temp_temporary_and_unlogged_tables() {
    for src in [
        "create temp table t (a int)",
        "create temporary table if not exists t (a int)",
        "create unlogged table tmp.t (a int)",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
    }
}

#[test]
fn test_create_table_on_commit_actions() {
    for src in [
        "create temp table t (a int) on commit drop;",
        "create temp table t (a int) on commit delete rows",
        "create temp table t (a int) on commit preserve rows",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_ON_COMMIT_CLAUSE"), "{src}");
    }
}

#[test]
fn test_create_table_on_commit_without_action_reports_error() {
    let res = parse("create temp table t (a int) on commit", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_temp_table_rejected_under_mssql_dialect() {
    let res = parse(
        "create temp table t (a int)",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_table_on_commit_rejected_under_standard_dialect() {
    let res = parse(
        "create table t (a int) on commit drop",
        SqlFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_table_as_select() {
    let res = parse("create table #tmp2 as select distinct a from #t;", mlang());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CREATE_TABLE_AS_STATEMENT"));
}

#[test]
fn test_create_temp_table_on_commit_drop_as_with_query() {
    let res = parse(
        "create temporary table r on commit drop as with x as (select 1 as a) select a from x",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_table_as_parenthesized_query_with_columns() {
    let res = parse("create table t (a, b) as (select 1, 2)", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_SUBQUERY_EXPRESSION"));
    assert!(!tree.contains("SQL_TABLE_ELEMENT_LIST"));
}

#[test]
fn test_create_table_as_table_with_no_data() {
    let res = parse("create table t as table #tmp with no data;", mlang());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_TABLE_QUERY"));
    assert!(tree.contains("PSQL_WITH_DATA_CLAUSE"));
}

#[test]
fn test_create_table_with_on_commit_but_no_as_is_still_a_regular_table() {
    let res = parse("create temp table t (a int) on commit drop", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("SQL_CREATE_TABLE_STATEMENT"));
}

#[test]
fn test_create_table_as_without_query_reports_error() {
    let res = parse("create table t as;", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_as_table_rejected_under_mssql_dialect() {
    let res = parse(
        "create table t as table s",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}

#[test]
fn test_create_table_as_select_from_with_no_data() {
    let res = parse("create table #a as select * from #b with no data", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_table_identity_columns() {
    for src in [
        "create table t (id int generated always as identity primary key)",
        "create table t (id int generated by default as identity (start with 1 increment by 1) not null)",
        "create table t (id bigint generated always as identity (increment 1 start 1 minvalue 1 maxvalue 9223372036854775807 cache 1))",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_IDENTITY_CONSTRAINT"), "{src}");
    }
}

#[test]
fn test_create_table_generated_stored_column() {
    let res = parse(
        "create table t (a int, b int generated always as (a * 2) stored)",
        postgres(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_GENERATED_COLUMN_CONSTRAINT"));
}

#[test]
fn test_create_table_generated_without_kind_reports_error() {
    let res = parse("create table t (id int generated as identity)", postgres());

    assert!(res.has_errors());
}

#[test]
fn test_create_table_identity_rejected_under_mssql_dialect() {
    let res = parse(
        "create table t (id int generated always as identity)",
        SqlFileSource::script().with_dialect(SqlDialect::Mssql),
    );

    assert!(res.has_errors());
}

#[test]
fn test_by_is_still_a_valid_column_name() {
    let res = parse("select by, always, generated from t", postgres());

    assert_parser!(res);
}
