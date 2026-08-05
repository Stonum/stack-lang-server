#[macro_use]
mod helper;

use sql_syntax::{SqlDialect, SqlFileSource, SqlSyntaxKind};

#[test]
fn format_named_parameter() {
    assert_fmt_node!(
        "select a from t where a = :ls",
        SqlSyntaxKind::SQL_PARAMETER_EXPRESSION,
        ":ls",
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}

#[test]
fn format_positional_parameter() {
    assert_fmt_node!(
        "select a from t where a = :1",
        SqlSyntaxKind::SQL_PARAMETER_EXPRESSION,
        ":1",
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}

#[test]
fn format_query_with_multiple_parameters() {
    assert_fmt!(
        r#"--
select a from t where a = :ls and b = :usl
"#,
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}

#[test]
fn format_limit_offset_with_parameters() {
    assert_fmt!(
        r#"--
select a from t limit :n offset :o
"#,
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}

#[test]
fn format_parameter_in_standard_dialect() {
    // Colon parameters aren't an mlang-specific extension -- they format
    // the same way regardless of dialect.
    assert_fmt!(
        r#"--
select a from t where a = :ls
"#,
        SqlFileSource::query()
    );
}

#[test]
fn format_parameter_name_colliding_with_a_reserved_keyword() {
    assert_fmt_node!(
        "select a from t where id = :check",
        SqlSyntaxKind::SQL_PARAMETER_EXPRESSION,
        ":check",
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}

#[test]
fn format_query_with_keyword_named_parameter() {
    assert_fmt!(
        r#"--
select a from t where id = :to
"#,
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true)
    );
}
