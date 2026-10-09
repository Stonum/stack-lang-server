#[macro_use]
mod helper;

#[test]
fn format_create_index_with_every_clause() {
    assert_fmt!(
        r#"--
create unique index if not exists ix_t on t using btree (a, b desc) include (c) where d > 0;
"#
    );
}

#[test]
fn format_create_index_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE  INDEX  ON  #tmp(A,B  DESC)  INCLUDE(C)
"#,
        r#"--
create index on #tmp (A, B desc) include (C)
"#
    );
}

#[test]
fn format_create_index_keeps_required_parentheses_around_expressions() {
    assert_fmt_eq!(
        r#"--
create index ix_t on t (lower(name),(amount*2) desc)
"#,
        r#"--
create index ix_t on t (lower(name), (amount * 2) desc)
"#
    );
}

#[test]
fn format_create_index_long_include_and_where_move_to_their_own_lines() {
    assert_fmt_eq!(
        r#"--
create index ix_accounts_state on accounts using btree (state, owner, created) include (name, code) where deleted is null and state > 0 and owner is not null
"#,
        r#"--
create index ix_accounts_state on accounts using btree (state, owner, created)
include (name, code)
where deleted is null
	and state > 0
	and owner is not null
"#
    );
}

#[test]
fn format_create_index_many_columns_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
create index ix_accounts on accounts (id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags, extra, more, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth)
"#,
        r#"--
create index ix_accounts on accounts (
	id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags,
	extra, more, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth
)
"#
    );
}

#[test]
fn format_create_index_expression_elements_wrap() {
    assert_fmt_eq!(
        r#"--
create index ix_accounts on accounts (lower(name), coalesce(code, ''), (amount * 2) desc, date_trunc('month', created), state)
"#,
        r#"--
create index ix_accounts on accounts (
	lower(name), coalesce(code, ''),
	(amount * 2) desc,
	date_trunc('month', created), state
)
"#
    );
}

#[test]
fn format_drop_and_alter_index() {
    assert_fmt!(
        r#"--
drop index concurrently if exists ix_a, s.ix_b cascade;
alter index if exists ix_a rename to ix_b;
"#
    );
}

#[test]
fn format_drop_index_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP  INDEX  IF  EXISTS  ix_a ,ix_b  CASCADE
"#,
        r#"--
drop index if exists ix_a, ix_b cascade
"#
    );
}

#[test]
fn format_drop_index_long_names_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
drop index if exists ix_accounts_a, ix_accounts_b, ix_accounts_c, ix_accounts_d, ix_accounts_e, ix_accounts_f, ix_accounts_g, ix_accounts_h, ix_accounts_i, ix_accounts_j, ix_accounts_k, ix_accounts_l cascade
"#,
        r#"--
drop index if exists
	ix_accounts_a, ix_accounts_b, ix_accounts_c, ix_accounts_d, ix_accounts_e, ix_accounts_f,
	ix_accounts_g, ix_accounts_h, ix_accounts_i, ix_accounts_j, ix_accounts_k, ix_accounts_l
cascade
"#
    );
}

#[test]
fn format_drop_index_many_short_names_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
drop index i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15, i16, i17, i18, i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31, i32, i33, i34, i35
"#,
        r#"--
drop index
	i1, i2, i3, i4, i5, i6, i7, i8, i9, i10, i11, i12, i13, i14, i15, i16, i17, i18,
	i19, i20, i21, i22, i23, i24, i25, i26, i27, i28, i29, i30, i31, i32, i33, i34, i35
"#
    );
}

#[test]
fn format_alter_index_long_rename_moves_under_the_index_name() {
    assert_fmt_eq!(
        r#"--
alter index some_long_schema_name.some_very_long_index_name_for_accounts rename to some_very_long_new_index_name_for_accounts
"#,
        r#"--
alter index some_long_schema_name.some_very_long_index_name_for_accounts
	rename to some_very_long_new_index_name_for_accounts
"#
    );
}
