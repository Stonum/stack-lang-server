#[macro_use]
mod helper;

#[test]
fn format_materialized_view_statements() {
    assert_fmt!(
        r#"--
create materialized view if not exists mv (a, b) as select a, b from t with no data;
drop materialized view if exists mv1, s.mv2 cascade;
refresh materialized view concurrently mv with data;
"#
    );
}

#[test]
fn format_materialized_view_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE  MATERIALIZED  VIEW  mv  AS  TABLE  t  WITH  NO  DATA
"#,
        r#"--
create materialized view mv as table t with no data
"#
    );
}

#[test]
fn format_materialized_view_parenthesized_query_wraps() {
    assert_fmt_eq!(
        r#"--
create materialized view mv as (with x as (select 1 as a) select a from x)
"#,
        r#"--
create materialized view mv as (
	with x as (select 1 as a)
	select a from x
)
"#
    );
}

#[test]
fn format_materialized_view_long_query_wraps() {
    assert_fmt_eq!(
        r#"--
create materialized view accounts_summary as select owner, count(*) as total, sum(amount) as amount_total from accounts join owners on owners.id = accounts.owner where accounts.deleted is null group by owner
"#,
        r#"--
create materialized view accounts_summary as select owner, count(*) as total, sum(amount) as amount_total
from accounts
join owners on owners.id = accounts.owner
where accounts.deleted is null
group by owner
"#
    );
}

#[test]
fn format_materialized_view_many_columns_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
create materialized view accounts_summary (id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags, extra, more, first, second, third) as select * from s
"#,
        r#"--
create materialized view accounts_summary (
	id, name, code, kind, state, owner, created, updated, deleted,
	comment, parent, sort, flags, extra, more, first, second, third
) as select * from s
"#
    );
}

#[test]
fn format_drop_materialized_view_many_views_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
drop materialized view if exists mv_a, mv_b, mv_c, mv_d, mv_e, mv_f, mv_g, mv_h, mv_i, mv_j, mv_k, mv_l, mv_m, mv_n, mv_o, mv_p, mv_q, mv_r, mv_s, mv_t, mv_u, mv_v, mv_w cascade
"#,
        r#"--
drop materialized view if exists
	mv_a, mv_b, mv_c, mv_d, mv_e, mv_f, mv_g, mv_h, mv_i, mv_j, mv_k, mv_l,
	mv_m, mv_n, mv_o, mv_p, mv_q, mv_r, mv_s, mv_t, mv_u, mv_v, mv_w
cascade
"#
    );
}
