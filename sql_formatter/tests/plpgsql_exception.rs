#[macro_use]
mod helper;

#[test]
fn format_exception_section_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN INSERT INTO t (a) VALUES (1); EXCEPTION WHEN duplicate_table THEN WHEN check_violation OR unique_violation THEN RAISE NOTICE 'skipped'; WHEN SQLSTATE '22P02' THEN NULL; WHEN OTHERS THEN RETURN; END $$;
"#,
        r#"--
do $$
begin
	insert into t (a) values (1);
exception
	when duplicate_table then
	when check_violation or unique_violation then
		raise notice 'skipped';
	when sqlstate '22P02' then
		null;
	when OTHERS then
		return;
end
$$;
"#
    );
}

#[test]
fn format_nested_block_with_exception() {
    assert_fmt_eq!(
        r#"--
do $$ begin loop begin update accounts set amount = amount + 100, updated_at = now() where accounts.owner = 1 and accounts.kind = 2 and accounts.created_at < now(); exception when others then exit; end; end loop; end $$;
"#,
        r#"--
do $$
begin
	loop
		begin
			update accounts
			set amount = amount + 100, updated_at = now()
			where accounts.owner = 1
				and accounts.kind = 2
				and accounts.created_at < now();
		exception
			when others then
				exit;
		end;
	end loop;
end
$$;
"#
    );
}
