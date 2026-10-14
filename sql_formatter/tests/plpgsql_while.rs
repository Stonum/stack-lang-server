#[macro_use]
mod helper;

#[test]
fn format_while_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN WHILE i<10 LOOP i := i + 1; END LOOP; <<batches>> WHILE true LOOP EXIT batches; END LOOP batches; END $$;
"#,
        r#"--
do $$
begin
	while i < 10 loop
		i := i + 1;
	end loop;
	<<batches>>
	while true loop
		exit batches;
	end loop batches;
end
$$;
"#
    );
}

#[test]
fn format_long_while_condition_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin while processed_accounts_count < total_accounts_count and now() - started_at < interval '30 minutes' and not cancel_requested loop processed_accounts_count := processed_accounts_count + 1; end loop; end $$;
"#,
        r#"--
do $$
begin
	while processed_accounts_count < total_accounts_count
		and now() - started_at < interval '30 minutes'
		and not cancel_requested
	loop
		processed_accounts_count := processed_accounts_count + 1;
	end loop;
end
$$;
"#
    );
}
