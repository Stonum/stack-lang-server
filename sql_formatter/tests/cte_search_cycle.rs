#[macro_use]
mod helper;

#[test]
fn format_cte_cycle_and_search() {
    assert_fmt_eq!(
        r#"--
WITH RECURSIVE g AS (SELECT 1 AS id) SEARCH BREADTH FIRST BY id SET ord CYCLE id,parent SET is_cycle TO TRUE DEFAULT FALSE USING path SELECT * FROM g;
"#,
        r#"--
with recursive g as (
	select 1 as id
) search breadth first by id set ord cycle id, parent set is_cycle to true default false using path
select * from g;
"#
    );
}

#[test]
fn format_long_recursive_cte_with_cycle() {
    assert_fmt_eq!(
        r#"--
with recursive tree as (select accounts.row_id, accounts.parent_id from accounts where accounts.parent_id is null union all select a.row_id, a.parent_id from accounts a join tree on a.parent_id = tree.row_id) cycle row_id set is_cycle using path select * from tree;
"#,
        r#"--
with recursive tree as (
	select accounts.row_id, accounts.parent_id
	from accounts
	where accounts.parent_id is null
		union all
	select a.row_id, a.parent_id
	from accounts a
	join tree on a.parent_id = tree.row_id
) cycle row_id set is_cycle using path
select * from tree;
"#
    );
}

#[test]
fn format_short_cte_cycle_stays_on_one_line() {
    assert_fmt_eq!(
        r#"--
WITH RECURSIVE g AS (SELECT 1 AS id) CYCLE id SET is_cycle USING path SELECT * FROM g;
"#,
        r#"--
with recursive g as (select 1 as id) cycle id set is_cycle using path
select * from g;
"#
    );
}
