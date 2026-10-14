#[macro_use]
mod helper;

#[test]
fn format_short_select_into_stays_on_one_line() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN SELECT a INTO   STRICT x FROM t; END $$;
"#,
        r#"--
do $$
begin
	select a into strict x from t;
end
$$;
"#
    );
}

#[test]
fn format_long_select_into_puts_into_on_its_own_line() {
    assert_fmt_eq!(
        r#"--
do $$ begin select cast(accounts.amount as int), accounts.owner into total_amount, owner_id from accounts where accounts.owner = 1; end $$;
"#,
        r#"--
do $$
begin
	select cast(accounts.amount as int), accounts.owner
	into total_amount, owner_id
	from accounts
	where accounts.owner = 1;
end
$$;
"#
    );
}

#[test]
fn format_many_into_targets_wrap() {
    assert_fmt_eq!(
        r#"--
do $$ begin select a, b, c, d, e, f into first_target_value, second_target_value, third_target_value, fourth_target_value, fifth_target_value, sixth_target_value, seventh_target_value from t; end $$;
"#,
        r#"--
do $$
begin
	select a, b, c, d, e, f
	into
		first_target_value, second_target_value, third_target_value, fourth_target_value,
		fifth_target_value, sixth_target_value, seventh_target_value
	from t;
end
$$;
"#
    );
}

#[test]
fn format_returning_into() {
    assert_fmt_eq!(
        r#"--
do $$ begin insert into t (a) values (1) RETURNING row_id INTO v_id; end $$;
"#,
        r#"--
do $$
begin
	insert into t (a) values (1) returning row_id into v_id;
end
$$;
"#
    );
}

#[test]
fn format_long_returning_into_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin update accounts set amount = amount + 100 where accounts.owner = 1 returning accounts.row_id, accounts.amount into account_id, account_amount; end $$;
"#,
        r#"--
do $$
begin
	update accounts
	set amount = amount + 100
	where accounts.owner = 1
	returning accounts.row_id, accounts.amount
	into account_id, account_amount;
end
$$;
"#
    );
}

#[test]
fn format_nested_fill_list_respects_the_indentation() {
    // Balanced lines are computed for the full width; three levels deep they
    // don't fit, so the fill wraps them earlier: uneven, but never past the
    // line limit.
    assert_fmt_eq!(
        r#"--
do $$ begin begin begin select a into account_target_value_name_01, account_target_value_name_02, account_target_value_name_03, account_target_value_name_04, account_target_value_name_05, account_target_value_name_06, account_target_value_name_07, account_target_value_name_08, account_target_value_name_09, account_target_value_name_10, account_target_value_name_11 from t; end; end; end $$;
"#,
        r#"--
do $$
begin
	begin
		begin
			select a
			into
				account_target_value_name_01, account_target_value_name_02, account_target_value_name_03,
				account_target_value_name_04,
				account_target_value_name_05, account_target_value_name_06, account_target_value_name_07,
				account_target_value_name_08,
				account_target_value_name_09, account_target_value_name_10, account_target_value_name_11
			from t;
		end;
	end;
end
$$;
"#
    );
}
