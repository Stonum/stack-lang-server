#[macro_use]
mod helper;

#[test]
fn format_perform_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN PERFORM  pg_advisory_xact_lock( hashtext(name)::bigint ); PERFORM a FROM t WHERE a > 1; END $$;
"#,
        r#"--
do $$
begin
	perform pg_advisory_xact_lock(hashtext(name)::bigint);
	perform a from t where a > 1;
end
$$;
"#
    );
}

#[test]
fn format_long_perform_query_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin perform accounts.owner, sum(accounts.amount) from accounts where accounts.owner = 1 and accounts.created > now() - interval '30 days'; end $$;
"#,
        r#"--
do $$
begin
	perform accounts.owner, sum(accounts.amount)
	from accounts
	where accounts.owner = 1 and accounts.created > now() - interval '30 days';
end
$$;
"#
    );
}

#[test]
fn format_long_perform_call_wraps_arguments() {
    assert_fmt_eq!(
        r#"--
do $$ begin perform process_table(schema_name, accounts_record.table_name, accounts_record.column_name, related_table_name, related_column_name, true); end $$;
"#,
        r#"--
do $$
begin
	perform process_table(
		schema_name, accounts_record.table_name, accounts_record.column_name, related_table_name, related_column_name, true
	);
end
$$;
"#
    );
}
