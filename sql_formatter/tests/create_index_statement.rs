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
