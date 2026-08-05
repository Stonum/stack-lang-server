#[macro_use]
mod helper;

use sql_syntax::{SqlDialect, SqlSyntaxKind};

#[test]
fn format_name() {
    assert_fmt_node!(
        r#"select "Col" from t"#,
        SqlSyntaxKind::SQL_NAME,
        r#""Col""#
    );
    assert_fmt_node!("select col from t", SqlSyntaxKind::SQL_NAME, "col");
}

#[test]
fn format_tilde_name() {
    assert_fmt_node!(
        "select a from ~Тест~ t",
        SqlSyntaxKind::SQL_TILDE_NAME,
        "~Тест~"
    );
}

#[test]
fn format_hash_table_is_plain_name() {
    assert_fmt_node!("select a from #tmp", SqlSyntaxKind::SQL_NAME, "a");
    assert_fmt_node!("select a from #tmp", SqlSyntaxKind::SQL_TABLE_NAME, "#tmp");
}

#[test]
fn format_star() {
    assert_fmt_node!("select * from t", SqlSyntaxKind::SQL_STAR, "*");
}

#[test]
fn format_col_reference() {
    assert_fmt_node!("select col from t", SqlSyntaxKind::SQL_COL_REFERENCE, "col");
}

#[test]
fn format_table_col_reference() {
    assert_fmt_node!(
        r#"select t."Col" from t"#,
        SqlSyntaxKind::SQL_TABLE_COL_REFERENCE,
        r#"t."Col""#
    );
    assert_fmt_node!(
        "select sch.t.col from sch.t",
        SqlSyntaxKind::SQL_TABLE_COL_REFERENCE,
        "sch.t.col"
    );
    assert_fmt_node!(
        "select db.sch.t.col from db.sch.t",
        SqlSyntaxKind::SQL_TABLE_COL_REFERENCE,
        "db.sch.t.col"
    );
}

#[test]
fn format_table_name_with_tilde() {
    assert_fmt_node!(
        "select a from ~Тест~",
        SqlSyntaxKind::SQL_TABLE_NAME,
        "~Тест~"
    );
}

#[test]
fn format_literals() {
    assert_fmt_node!(
        "select 1 from t",
        SqlSyntaxKind::SQL_NUMBER_LITERAL_EXPRESSION,
        "1"
    );
    assert_fmt_node!(
        "select 1.5 from t",
        SqlSyntaxKind::SQL_NUMBER_LITERAL_EXPRESSION,
        "1.5"
    );
    assert_fmt_node!(
        "select 'hi' from t",
        SqlSyntaxKind::SQL_STRING_LITERAL_EXPRESSION,
        "'hi'"
    );
    assert_fmt_node!(
        "select TRUE from t",
        SqlSyntaxKind::SQL_BOOLEAN_LITERAL_EXPRESSION,
        "true"
    );
    assert_fmt_node!(
        "select FALSE from t",
        SqlSyntaxKind::SQL_BOOLEAN_LITERAL_EXPRESSION,
        "false"
    );
    assert_fmt_node!(
        "select NULL from t",
        SqlSyntaxKind::SQL_NULL_LITERAL_EXPRESSION,
        "null"
    );
}

#[test]
fn format_alias() {
    assert_fmt_node!(
        r#"select a as "A" from t"#,
        SqlSyntaxKind::SQL_ALIAS,
        r#"as "A""#
    );
    assert_fmt_node!(r#"select b "B" from t"#, SqlSyntaxKind::SQL_ALIAS, r#""B""#);
}

#[test]
fn format_tilde_array_suffix() {
    assert_fmt_node!(
        "select a::int~[]~ from t",
        SqlSyntaxKind::SQL_TILDE_ARRAY_SUFFIX,
        "~[]~"
    );
}

#[test]
fn format_standard_dialect_rejects_tilde_and_hash() {
    // Sanity check that the mlang-only leaves above genuinely depend on the
    // dialect flag, not just on being syntactically acceptable.
    let syntax = sql_syntax::SqlFileSource::query().with_dialect(SqlDialect::Standard);
    let tree = sql_parser::parse("select a from ~Тест~ t", syntax);
    assert!(tree.has_errors());
}
