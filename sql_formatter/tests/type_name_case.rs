#[macro_use]
mod helper;

#[test]
fn format_lowercases_builtin_type_aliases() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE t (A INT, B SmallInt NOT NULL, C FLOAT8[], D BOOL DEFAULT TRUE, E INTEGER, F MyType, G "INT");
SELECT X::INT4, CAST(Y AS TIMESTAMPTZ) FROM t;
"#,
        r#"--
create table t (A int, B smallint not null, C float8[], D bool default true, E integer, F MyType, G "INT");
select X::int4, cast(Y as timestamptz) from t;
"#
    );
}

#[test]
fn format_lowercases_builtin_types_in_plpgsql() {
    assert_fmt_eq!(
        r#"--
CREATE FUNCTION f(A INT, B INT8) RETURNS VOID LANGUAGE plpgsql AS $$
DECLARE
	C INT := A;
	D SERIAL;
BEGIN
	C := C::BIGINT;
END
$$;
"#,
        r#"--
create function f(A int, B int8) returns void language plpgsql as $$
declare
	C int := A;
	D serial;
begin
	C := C::bigint;
end
$$;
"#
    );
}

#[test]
fn format_long_table_with_builtin_type_aliases() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE accounts (first_column_name INT NOT NULL, second_column_name SMALLINT, third_column_name FLOAT8, fourth_column_name BOOL, fifth_column_name TIMESTAMPTZ);
"#,
        r#"--
create table accounts (
	first_column_name int not null,
	second_column_name smallint,
	third_column_name float8,
	fourth_column_name bool,
	fifth_column_name timestamptz
);
"#
    );
}
