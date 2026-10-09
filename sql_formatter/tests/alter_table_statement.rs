#[macro_use]
mod helper;

#[test]
fn format_alter_table_disable_trigger_user() {
    assert_fmt!(
        r#"--
alter table ~Таблица~ disable trigger user;
"#
    );
}

#[test]
fn format_alter_table_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER   TABLE  IF EXISTS t   ENABLE  TRIGGER  ALL ,DISABLE TRIGGER "Trg";
"#,
        r#"--
alter table if exists t enable trigger all, disable trigger "Trg";
"#
    );
}

#[test]
fn format_alter_table_add_and_drop_columns() {
    assert_fmt!(
        r#"--
alter table t add column if not exists a int not null default 0, drop column if exists b cascade;
"#
    );
}

#[test]
fn format_alter_table_add_column_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER TABLE t  ADD   COLUMN  a  varchar(10)  NULL ,DROP  b
"#,
        r#"--
alter table t add column a varchar(10) null, drop b
"#
    );
}

#[test]
fn format_alter_column_operations() {
    assert_fmt!(
        r#"--
alter table t alter column a type int using a::int, alter b set default 0, alter c drop not null;
"#
    );
}

#[test]
fn format_alter_column_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER TABLE t ALTER  COLUMN a  SET  DATA  TYPE  text ,ALTER b  DROP   DEFAULT
"#,
        r#"--
alter table t alter column a set data type text, alter b drop default
"#
    );
}

#[test]
fn format_alter_table_actions_one_per_line_when_too_long() {
    assert_fmt_eq!(
        r#"--
alter table some_long_table_name add column if not exists first_new_column varchar(256) not null default 'x', drop column if exists old_column cascade, alter column third_column type numeric(15, 2) using third_column::numeric(15, 2), disable trigger user
"#,
        r#"--
alter table some_long_table_name
	add column if not exists first_new_column varchar(256) not null default 'x',
	drop column if exists old_column cascade,
	alter column third_column type numeric(15, 2) using third_column::numeric(15, 2),
	disable trigger user
"#
    );
}

#[test]
fn format_alter_column_long_using_expression_wraps() {
    assert_fmt_eq!(
        r#"--
alter table t alter column a type numeric using case when some_function(first_argument, second_argument) > 0 then another_function(third_argument) else 0 end
"#,
        r#"--
alter table t
	alter column a type numeric using case
		when some_function(first_argument, second_argument) > 0 then another_function(third_argument)
		else 0
	end
"#
    );
}

#[test]
fn format_alter_table_rename_and_set_schema() {
    assert_fmt!(
        r#"--
alter table t rename column a to b;
alter table t rename to u;
alter table t rename constraint c to d;
alter table t set schema s;
"#
    );
}

#[test]
fn format_alter_table_rename_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ALTER TABLE ~Таблица~  RENAME  COLUMN  "Поле"  TO  "Новое поле"
"#,
        r#"--
alter table ~Таблица~ rename column "Поле" to "Новое поле"
"#
    );
}

#[test]
fn format_alter_table_single_long_action_moves_under_the_table_name() {
    assert_fmt_eq!(
        r#"--
alter table some_long_schema_name.some_long_table_name rename column some_very_long_old_column_name to some_very_long_new_column_name;
"#,
        r#"--
alter table some_long_schema_name.some_long_table_name
	rename column some_very_long_old_column_name to some_very_long_new_column_name;
"#
    );
}

#[test]
fn format_alter_table_rename_two_columns() {
    assert_fmt!(
        r#"--
alter table accounts rename column a to b, rename column c to d
"#
    );
}

#[test]
fn format_alter_table_many_renames_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts rename column first_name to given_name, rename column last_name to family_name, rename column phone to phone_number
"#,
        r#"--
alter table accounts
	rename column first_name to given_name,
	rename column last_name to family_name,
	rename column phone to phone_number
"#
    );
}

#[test]
fn format_alter_table_many_drop_columns_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts drop column a, drop column b, drop column c, drop column d, drop column e, drop column f, drop column g, drop column h
"#,
        r#"--
alter table accounts
	drop column a,
	drop column b,
	drop column c,
	drop column d,
	drop column e,
	drop column f,
	drop column g,
	drop column h
"#
    );
}

#[test]
fn format_alter_table_many_add_columns_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts add column a int, add column b int, add column c int, add column d int, add column e int, add column f int, add g int
"#,
        r#"--
alter table accounts
	add column a int,
	add column b int,
	add column c int,
	add column d int,
	add column e int,
	add column f int,
	add g int
"#
    );
}

#[test]
fn format_alter_table_many_trigger_actions_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts disable trigger a, disable trigger b, disable trigger c, disable trigger d, enable trigger e, enable trigger f
"#,
        r#"--
alter table accounts
	disable trigger a,
	disable trigger b,
	disable trigger c,
	disable trigger d,
	enable trigger e,
	enable trigger f
"#
    );
}

#[test]
fn format_alter_table_many_alter_column_actions_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts alter column a drop default, alter column b drop not null, alter column c set not null, alter d set default 0
"#,
        r#"--
alter table accounts
	alter column a drop default,
	alter column b drop not null,
	alter column c set not null,
	alter d set default 0
"#
    );
}

#[test]
fn format_alter_table_many_rename_constraints_wrap() {
    assert_fmt_eq!(
        r#"--
alter table accounts rename constraint a to b, rename constraint c to d, rename constraint e to f, rename constraint g to h, rename constraint i to j
"#,
        r#"--
alter table accounts
	rename constraint a to b,
	rename constraint c to d,
	rename constraint e to f,
	rename constraint g to h,
	rename constraint i to j
"#
    );
}
