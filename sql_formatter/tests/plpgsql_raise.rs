#[macro_use]
mod helper;

#[test]
fn format_raise_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN RAISE NOTICE '% rows: %',now(),cnt; RAISE EXCEPTION 'locked' USING ERRCODE='27101'; RAISE ; RAISE SQLSTATE '22012'; END $$;
"#,
        r#"--
do $$
begin
	raise notice '% rows: %', now(), cnt;
	raise exception 'locked' using ERRCODE = '27101';
	raise;
	raise sqlstate '22012';
end
$$;
"#
    );
}

#[test]
fn format_long_raise_puts_using_on_its_own_line() {
    assert_fmt_eq!(
        r#"--
do $$ begin raise exception 'The account % is locked by another session, try again later', account_id using errcode = '27101', hint = 'Wait for the lock'; end $$;
"#,
        r#"--
do $$
begin
	raise exception 'The account % is locked by another session, try again later', account_id
	using errcode = '27101', hint = 'Wait for the lock';
end
$$;
"#
    );
}

#[test]
fn format_many_raise_arguments_wrap() {
    assert_fmt_eq!(
        r#"--
do $$ begin raise notice '% % % % % % %', first_value_name, second_value_name, third_value_name, fourth_value_name, fifth_value_name, sixth_value_name; end $$;
"#,
        r#"--
do $$
begin
	raise notice
		'% % % % % % %', first_value_name, second_value_name, third_value_name,
		fourth_value_name, fifth_value_name, sixth_value_name;
end
$$;
"#
    );
}
