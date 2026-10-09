#[macro_use]
mod helper;

#[test]
fn format_create_view_simple() {
    assert_fmt!(
        r#"--
create view foo as select a from t
"#
    );
}

#[test]
fn format_create_or_replace_view() {
    assert_fmt!(
        r#"--
create or replace view foo as select a from t
"#
    );
}

#[test]
fn format_create_view_qualified_name() {
    assert_fmt!(
        r#"--
create view myschema.foo as select a from t
"#
    );
}

#[test]
fn format_create_view_with_options() {
    assert_fmt!(
        r#"--
create view foo with (security_invoker = true) as select a from t;
"#
    );
}

#[test]
fn format_create_view_with_multiple_options() {
    assert_fmt!(
        r#"--
create view foo with (security_invoker = true, check_option = cascaded) as select a from t
"#
    );
}

#[test]
fn format_create_view_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   OR REPLACE   VIEW foo   WITH(security_invoker=true)   AS select a from t;
"#,
        r#"--
create or replace view foo with (security_invoker = true) as select a from t;
"#
    );
}

#[test]
fn format_create_view_with_cte_query() {
    assert_fmt!(
        r#"--
create view v as with x as (select 1 as a)
select a from x;
create view w as (select a from t);
create view u as table t;
"#
    );
}

#[test]
fn format_create_view_with_cte_normalizes_case() {
    assert_fmt_eq!(
        r#"--
CREATE OR REPLACE VIEW v (a, b) AS WITH x AS (SELECT 1 AS a, 2 AS b) SELECT a, b FROM x;
"#,
        r#"--
create or replace view v (a, b) as with x as (select 1 as a, 2 as b)
select a, b from x;
"#
    );
}

#[test]
fn format_create_view_long_cte_query_wraps() {
    assert_fmt_eq!(
        r#"--
create view accounts_summary as with recent as (select owner, sum(amount) as total from payments where created > now() - interval '30 days' group by owner) select owners.name, recent.total from owners join recent on recent.owner = owners.id where recent.total > 0
"#,
        r#"--
create view accounts_summary as with recent as (
	select owner, sum(amount) as total from payments where created > now() - interval '30 days' group by owner
)
select owners.name, recent.total
from owners
join recent on recent.owner = owners.id
where recent.total > 0
"#
    );
}

#[test]
fn format_create_view_many_columns_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
create view v (id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags, extra, more, first, second, third) as select * from s
"#,
        r#"--
create view v (
	id, name, code, kind, state, owner, created, updated, deleted,
	comment, parent, sort, flags, extra, more, first, second, third
) as select * from s
"#
    );
}
