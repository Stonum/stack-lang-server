#[macro_use]
mod helper;

#[test]
fn format_for_range_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN FOR i IN 1 .. 10 LOOP x := x + i; END LOOP; FOR i IN REVERSE 10..1 BY 2 LOOP END LOOP; END $$;
"#,
        r#"--
do $$
begin
	for i in 1..10 loop
		x := x + i;
	end loop;
	for i in reverse 10..1 by 2 loop
	end loop;
end
$$;
"#
    );
}

#[test]
fn format_long_for_range_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin for account_index in reverse array_length(processed_accounts_array, 1)..array_lower(processed_accounts_array, 1) by batch_size loop end loop; end $$;
"#,
        r#"--
do $$
begin
	for account_index in
		reverse array_length(processed_accounts_array, 1)..array_lower(processed_accounts_array, 1) by batch_size
	loop
	end loop;
end
$$;
"#
    );
}
