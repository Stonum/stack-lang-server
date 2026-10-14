#[macro_use]
mod helper;

#[test]
fn format_keeps_one_blank_line_between_statements() {
    assert_fmt_eq!(
        r#"--
select 1;
select 2;


select 3;

-- a comment before the next statement
select 4;
"#,
        r#"--
select 1;
select 2;

select 3;

-- a comment before the next statement
select 4;
"#
    );
}

#[test]
fn format_keeps_blank_lines_in_plpgsql_bodies() {
    assert_fmt_eq!(
        r#"--
do $$
declare
	a int;

	b int;
begin

	a := 1;



	b := 2;
	if a = b then
		null;

		return;
	end if;

end
$$;
"#,
        r#"--
do $$
declare
	a int;

	b int;
begin
	a := 1;

	b := 2;
	if a = b then
		null;

		return;
	end if;
end
$$;
"#
    );
}
