#[macro_use]
mod helper;

#[test]
fn format_into_after_from() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN SELECT count(*) FROM t INTO cnt; SELECT * FROM f(x) t INTO a, b; END $$;
"#,
        r#"--
do $$
begin
	select count(*) from t into cnt;
	select * from f(x) t into a, b;
end
$$;
"#
    );
}

#[test]
fn format_select_into_before_items() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN SELECT INTO a, b x, y FROM t; END $$;
"#,
        r#"--
do $$
begin
	select into a, b x, y from t;
end
$$;
"#
    );
}

#[test]
fn format_long_select_into_before_items_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin select into type_block, block_month dog.block_type, coalesce(dog.block_month, 0) from accounts dog where dog.row_id = 1 and dog.kind = 2; end $$;
"#,
        r#"--
do $$
begin
	select into type_block, block_month dog.block_type, coalesce(dog.block_month, 0)
	from accounts dog
	where dog.row_id = 1 and dog.kind = 2;
end
$$;
"#
    );
}
