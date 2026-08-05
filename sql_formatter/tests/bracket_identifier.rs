#[macro_use]
mod helper;

#[test]
fn format_bracket_identifier_in_select_list() {
    assert_fmt_eq!(
        r#"--
select [Col-Name], [Col2] from t;
"#,
        r#"--
select "Col-Name", "Col2" from t;
"#
    );
}

#[test]
fn format_bracket_identifier_in_where_clause() {
    assert_fmt_eq!(
        r#"--
select a from t where [Col-Name] = 1;
"#,
        r#"--
select a from t where "Col-Name" = 1;
"#
    );
}

#[test]
fn format_dotted_bracket_identifier() {
    assert_fmt_eq!(
        r#"--
select t.[Col Name] from t;
"#,
        r#"--
select t."Col Name" from t;
"#
    );
}

#[test]
fn format_bracket_identifier_as_table_qualifier() {
    assert_fmt_eq!(
        r#"--
select a from [Table Name].t;
"#,
        r#"--
select a from "Table Name".t;
"#
    );
}

#[test]
fn format_bracket_identifier_in_insert_column_list() {
    assert_fmt_eq!(
        r#"--
insert into t ([Col-Name], [Col2]) values (1, 2);
"#,
        r#"--
insert into t ("Col-Name", "Col2")
values (1, 2);
"#
    );
}

#[test]
fn format_bracket_identifier_in_update_set_clause() {
    assert_fmt_eq!(
        r#"--
update t set [Col-Name] = 1 where id = 2;
"#,
        r#"--
update t
set "Col-Name" = 1
where id = 2;
"#
    );
}

#[test]
fn format_already_double_quoted_identifier_unaffected() {
    // Regression guard: an identifier that was already Postgres-style
    // double-quoted (not bracket-quoted) must round-trip unchanged -- only
    // the bracket spelling gets canonicalized.
    assert_fmt!(
        r#"--
select "already-quoted" from t
"#
    );
}

#[test]
fn format_array_subscript_unaffected_by_bracket_identifier_support() {
    assert_fmt!(
        r#"--
select arr[1] from t
"#
    );
}

#[test]
fn format_array_type_suffix_unaffected_by_bracket_identifier_support() {
    assert_fmt!(
        r#"--
create table t (a int[]);
"#
    );
}
