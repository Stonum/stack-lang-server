#[macro_use]
mod helper;

#[test]
fn format_nested_blocks_and_null() {
    assert_fmt_eq!(
        r#"--
DO $$ DECLARE x int := 1; BEGIN NULL; BEGIN NULL; END; DECLARE y int := x; BEGIN <<inner_block>> BEGIN y := y + 1; END inner_block; END; END $$;
"#,
        r#"--
do $$
declare
	x int := 1;
begin
	null;
	begin
		null;
	end;
	declare
		y int := x;
	begin
		<<inner_block>>
		begin
			y := y + 1;
		end inner_block;
	end;
end
$$;
"#
    );
}

#[test]
fn format_deeply_nested_long_statement_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin begin begin update accounts set amount = amount + 100, updated_at = now() where accounts.owner = 1 and accounts.kind = 2 and accounts.created_at < now(); end; end; end $$;
"#,
        r#"--
do $$
begin
	begin
		begin
			update accounts
			set amount = amount + 100, updated_at = now()
			where accounts.owner = 1
				and accounts.kind = 2
				and accounts.created_at < now();
		end;
	end;
end
$$;
"#
    );
}
