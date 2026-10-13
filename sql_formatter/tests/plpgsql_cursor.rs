#[macro_use]
mod helper;

#[test]
fn format_cursors_normalize_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ DECLARE c NO SCROLL CURSOR FOR SELECT a FROM t; BEGIN OPEN c; OPEN b(1,2); OPEN r SCROLL FOR SELECT 1; OPEN r FOR EXECUTE q USING a; FETCH NEXT FROM c INTO x , y; FETCH c INTO x; MOVE RELATIVE -2 IN c; CLOSE c; END $$;
"#,
        r#"--
do $$
declare
	c no scroll cursor for select a from t;
begin
	open c;
	open b(1, 2);
	open r scroll for select 1;
	open r for execute q using a;
	fetch next from c into x, y;
	fetch c into x;
	move relative -2 in c;
	close c;
end
$$;
"#
    );
}

#[test]
fn format_long_cursor_queries_move_under_the_header() {
    assert_fmt_eq!(
        r#"--
do $$ declare accounts_cursor cursor for select accounts.owner, sum(accounts.amount) from accounts where accounts.owner = 1 and accounts.kind = 2 group by accounts.owner; begin open payments_cursor for select payments.owner, sum(payments.amount) from payments where payments.owner = 1 and payments.kind = 2 group by payments.owner; end $$;
"#,
        r#"--
do $$
declare
	accounts_cursor cursor for
		select accounts.owner, sum(accounts.amount)
		from accounts
		where accounts.owner = 1 and accounts.kind = 2
		group by accounts.owner;
begin
	open payments_cursor for
		select payments.owner, sum(payments.amount)
		from payments
		where payments.owner = 1 and payments.kind = 2
		group by payments.owner;
end
$$;
"#
    );
}

#[test]
fn format_long_fetch_into_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin fetch next from accounts_cursor into account_owner_id, account_amount_total, account_created_at, account_kind_name, account_parent_id; end $$;
"#,
        r#"--
do $$
begin
	fetch next from accounts_cursor
	into account_owner_id, account_amount_total, account_created_at, account_kind_name, account_parent_id;
end
$$;
"#
    );
}
