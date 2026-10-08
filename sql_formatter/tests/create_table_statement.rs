#[macro_use]
mod helper;

#[test]
fn format_create_table_single_column() {
    assert_fmt!(
        r#"--
create table foo (a int)
"#
    );
}

#[test]
fn format_create_table_column_named_full() {
    assert_fmt!(
        r#"--
create table foo (full text)
"#
    );
}

#[test]
fn format_create_table_multiple_columns() {
    assert_fmt!(
        r#"--
create table foo (a int, b text, c boolean)
"#
    );
}

#[test]
fn format_create_table_if_not_exists() {
    assert_fmt!(
        r#"--
create table if not exists foo (a int);
"#
    );
}

#[test]
fn format_create_table_qualified_name() {
    assert_fmt!(
        r#"--
create table myschema.foo (a int)
"#
    );
}

#[test]
fn format_create_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   TABLE   IF NOT EXISTS   foo (a   int,b text);
"#,
        r#"--
create table if not exists foo (a int, b text);
"#
    );
}

#[test]
fn format_create_table_columns_wrap_when_too_long() {
    assert_fmt!(
        r#"--
create table foo (
	really_long_column_name_number_one int, really_long_column_name_number_two text,
	really_long_column_name_number_three boolean, really_long_column_name_number_four numeric
)
"#
    );
}

#[test]
fn format_create_table_column_constraints() {
    assert_fmt!(
        r#"--
create table foo (a int not null default 0, b text null, c timestamp default now())
"#
    );
}

#[test]
fn format_create_table_column_constraints_normalize_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE #t ( a int   NOT   NULL,b VARCHAR(256)   NULL , c int DEFAULT   1+2 )
"#,
        r#"--
create table #t (a int not null, b varchar(256) null, c int default 1 + 2)
"#
    );
}

#[test]
fn format_create_table_key_constraints() {
    assert_fmt!(
        r#"--
create table foo (id int primary key, code text unique, parent int references bar (id))
"#
    );
}

#[test]
fn format_create_table_key_constraints_normalize_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE foo ( a int  CONSTRAINT  a_positive  CHECK(a>0)   NOT NULL,b int REFERENCES  bar(id) )
"#,
        r#"--
create table foo (a int constraint a_positive check (a > 0) not null, b int references bar (id))
"#
    );
}

#[test]
fn format_create_table_table_level_constraints() {
    assert_fmt!(
        r#"--
create table foo (a int, b int, primary key (a, b), unique (b), foreign key (b) references bar (id))
"#
    );
}

#[test]
fn format_create_table_table_level_constraints_normalize_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE TABLE foo ( a int, b int,CONSTRAINT  pk_foo  PRIMARY KEY(a,b) ,CHECK(a<b) )
"#,
        r#"--
create table foo (a int, b int, constraint pk_foo primary key (a, b), check (a < b))
"#
    );
}

#[test]
fn format_create_temp_table_on_commit_drop() {
    assert_fmt!(
        r#"--
create temp table foo (a int) on commit drop;
"#
    );
}

#[test]
fn format_create_temp_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   TEMPORARY  TABLE foo (a int)  ON   COMMIT  PRESERVE   ROWS;
"#,
        r#"--
create temporary table foo (a int) on commit preserve rows;
"#
    );
}

#[test]
fn format_create_unlogged_table() {
    assert_fmt_eq!(
        r#"--
CREATE UNLOGGED TABLE tmp.foo (a int)
"#,
        r#"--
create unlogged table tmp.foo (a int)
"#
    );
}
