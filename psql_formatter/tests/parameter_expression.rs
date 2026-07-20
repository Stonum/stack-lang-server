#[macro_use]
mod helper;

use psql_syntax::{PsqlDialect, PsqlSyntaxKind};

#[test]
fn format_named_parameter() {
    assert_fmt_node!(
        "select a from t where a = :ls",
        PsqlSyntaxKind::PSQL_PARAMETER_EXPRESSION,
        ":ls",
        PsqlDialect::Mlang
    );
}

#[test]
fn format_positional_parameter() {
    assert_fmt_node!(
        "select a from t where a = :1",
        PsqlSyntaxKind::PSQL_PARAMETER_EXPRESSION,
        ":1",
        PsqlDialect::Mlang
    );
}

#[test]
fn format_query_with_multiple_parameters() {
    assert_fmt!(
        r#"--
select a from t where a = :ls and b = :usl
"#,
        PsqlDialect::Mlang
    );
}

#[test]
fn format_limit_offset_with_parameters() {
    assert_fmt!(
        r#"--
select a from t limit :n offset :o
"#,
        PsqlDialect::Mlang
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
        PsqlDialect::Standard
    );
}
