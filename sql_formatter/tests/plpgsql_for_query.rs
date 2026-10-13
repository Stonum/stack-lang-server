#[macro_use]
mod helper;

#[test]
fn format_short_for_stays_on_one_line() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN FOR rec IN SELECT * FROM t LOOP DELETE FROM t2 WHERE a = rec.a; END LOOP; FOR rec , val IN EXECUTE q USING a LOOP END LOOP; END $$;
"#,
        r#"--
do $$
begin
	for rec in select * from t loop
		delete from t2 where a = rec.a;
	end loop;
	for rec, val in execute q using a loop
	end loop;
end
$$;
"#
    );
}

#[test]
fn format_long_for_query_moves_under_the_header() {
    assert_fmt_eq!(
        r#"--
do $$ begin <<rows>> for rec in select accounts.owner, sum(accounts.amount) as total from accounts where accounts.owner = owner_id group by accounts.owner loop exit rows; end loop rows; end $$;
"#,
        r#"--
do $$
begin
	<<rows>>
	for rec in
		select accounts.owner, sum(accounts.amount) as total
		from accounts
		where accounts.owner = owner_id
		group by accounts.owner
	loop
		exit rows;
	end loop rows;
end
$$;
"#
    );
}

#[test]
fn format_long_for_execute_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin for rec in execute format('select * from %I where owner = $1 and kind = $2', accounts_table_name) using owner_id, kind_id loop end loop; end $$;
"#,
        r#"--
do $$
begin
	for rec in
		execute format('select * from %I where owner = $1 and kind = $2', accounts_table_name)
		using owner_id, kind_id
	loop
	end loop;
end
$$;
"#
    );
}
