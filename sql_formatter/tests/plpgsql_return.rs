#[macro_use]
mod helper;

#[test]
fn format_return_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN RETURN  NULL; RETURN ; RETURN NEXT rec; RETURN QUERY SELECT 1; RETURN QUERY EXECUTE q USING a; END $$;
"#,
        r#"--
do $$
begin
	return null;
	return;
	return next rec;
	return query select 1;
	return query execute q using a;
end
$$;
"#
    );
}

#[test]
fn format_long_return_query_moves_under_the_keyword() {
    assert_fmt_eq!(
        r#"--
do $$ begin return query select accounts.owner, sum(accounts.amount) from accounts where accounts.owner = owner_id group by accounts.owner order by sum(accounts.amount) desc; end $$;
"#,
        r#"--
do $$
begin
	return query
		select accounts.owner, sum(accounts.amount)
		from accounts
		where accounts.owner = owner_id
		group by accounts.owner
		order by sum(accounts.amount) desc;
end
$$;
"#
    );
}

#[test]
fn format_long_return_value_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin return (select sum(payments.amount) from payments where payments.owner = 1 and payments.created > now() - interval '30 days'); end $$;
"#,
        r#"--
do $$
begin
	return (
		select sum(payments.amount) from payments where payments.owner = 1 and payments.created > now() - interval '30 days'
	);
end
$$;
"#
    );
}
