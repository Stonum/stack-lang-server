#[macro_use]
mod helper;

#[test]
fn format_execute_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN EXECUTE   _body; EXECUTE format('truncate table %s',tmp) ; EXECUTE q INTO STRICT x USING a,b; END $$;
"#,
        r#"--
do $$
begin
	execute _body;
	execute format('truncate table %s', tmp);
	execute q into strict x using a, b;
end
$$;
"#
    );
}

#[test]
fn format_long_execute_puts_clauses_on_their_own_lines() {
    assert_fmt_eq!(
        r#"--
do $$ begin execute format('select count(*) from %I where owner = $1 and kind = $2', accounts_table_name) into accounts_count using owner_id, kind_id; end $$;
"#,
        r#"--
do $$
begin
	execute format('select count(*) from %I where owner = $1 and kind = $2', accounts_table_name)
	into accounts_count
	using owner_id, kind_id;
end
$$;
"#
    );
}

#[test]
fn format_many_using_arguments_wrap() {
    assert_fmt_eq!(
        r#"--
do $$ begin execute q using first_argument_value, second_argument_value, third_argument_value, fourth_argument_value, fifth_argument_value, sixth_argument_value; end $$;
"#,
        r#"--
do $$
begin
	execute q
	using
		first_argument_value, second_argument_value, third_argument_value,
		fourth_argument_value, fifth_argument_value, sixth_argument_value;
end
$$;
"#
    );
}
