#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script().with_dialect(SqlDialect::Postgres)
}

fn mlang() -> SqlFileSource {
    postgres().with_mlang_extension(true)
}

#[test]
fn test_create_table_partition_by() {
    for src in [
        "create table t (a int, d date) partition by range (d)",
        "create table t (a int) partition by list (a);",
        "create table t (a int, b int) partition by hash (a, b)",
        "create table t (a int) partition by range ((a * 2), lower(b))",
        r#"create table s."p" (like ~Таблица~) partition by range ("Дата")"#,
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("PSQL_PARTITION_BY_CLAUSE"), "{src}");
    }
}

#[test]
fn test_create_table_like_element() {
    let res = parse("create table t (like s.source, extra int)", postgres());

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert!(tree.contains("PSQL_TABLE_LIKE_ELEMENT"));
}

#[test]
fn test_create_table_partition_of_forms() {
    for src in [
        "create table p partition of t default",
        r#"CREATE TABLE IF NOT EXISTS s."p_def" PARTITION OF ~Таблица~ DEFAULT;"#,
        "create table p partition of t for values in ('a', 'b')",
        "create table p partition of t for values from ('2024-01-01'::date) to ('2024-02-01'::date);",
        "create table p partition of t for values from (minvalue, 0) to (maxvalue, 10)",
        "create table p partition of t for values with (modulus 4, remainder 3)",
        "create table p partition of t for values from (1) to (10) partition by list (kind)",
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(
            tree.contains("PSQL_CREATE_TABLE_PARTITION_OF_STATEMENT"),
            "{src}"
        );
    }
}

#[test]
fn test_alter_table_attach_and_detach_partition() {
    for src in [
        "alter table t attach partition p default",
        r#"alter table ~Таблица~ attach partition s."p" for values in ('x');"#,
        "alter table t attach partition p for values from (1) to (10)",
        "alter table t detach partition p",
        "alter table t detach partition p concurrently;",
        "alter table t detach partition p finalize",
    ] {
        let res = parse(src, mlang());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(
            tree.contains("PSQL_ATTACH_PARTITION_ACTION")
                || tree.contains("PSQL_DETACH_PARTITION_ACTION"),
            "{src}"
        );
    }
}

#[test]
fn test_partition_batch_without_semicolons() {
    let res = parse(
        r#"alter table s."p" drop column if exists id cascade
alter table ~Таблица~ attach partition s."p" default
alter table s."p" rename to "p_def""#,
        mlang(),
    );

    assert_parser!(res);
    let tree = format!("{:#?}", res.syntax());
    assert_eq!(tree.matches("SQL_ALTER_TABLE_STATEMENT").count(), 3);
}

#[test]
fn test_partition_keywords_are_still_valid_names() {
    let res = parse(
        "select partition, range, list, hash, modulus, remainder, attach, detach, finalize from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_partition_by_still_works() {
    let res = parse(
        "select row_number() over (partition by a order by b) from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_partition_errors() {
    for src in [
        "create table t (a int) partition by (a)",
        "create table p partition of t",
        "create table p partition of t for values (1)",
        "create table p partition of t for values with (modulus 4)",
        "alter table t attach partition p",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}

#[test]
fn test_partitioning_rejected_under_mssql_dialect() {
    for src in [
        "create table t (a int) partition by range (a)",
        "create table p partition of t default",
        "alter table t detach partition p",
    ] {
        let res = parse(src, SqlFileSource::script().with_dialect(SqlDialect::Mssql));

        assert!(res.has_errors(), "{src}");
    }
}
