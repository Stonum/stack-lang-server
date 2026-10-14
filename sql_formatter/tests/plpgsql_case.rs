#[macro_use]
mod helper;

#[test]
fn format_case_statement_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN CASE x WHEN 1,2 THEN y := 1; WHEN 3 THEN ELSE y := CASE WHEN z THEN 1 ELSE 0 END; END CASE; CASE WHEN a > 1 THEN DELETE FROM t; END CASE; END $$;
"#,
        r#"--
do $$
begin
	case x
		when 1, 2 then
			y := 1;
		when 3 then
		else
			y := case when z then 1 else 0 end;
	end case;
	case
		when a > 1 then
			delete from t;
	end case;
end
$$;
"#
    );
}

#[test]
fn format_long_when_conditions_wrap() {
    assert_fmt_eq!(
        r#"--
do $$ begin case account_kind when 'first_account_kind', 'second_account_kind', 'third_account_kind', 'fourth_account_kind', 'fifth_account_kind', 'sixth_account_kind', 'seventh_account_kind' then y := 1; end case; end $$;
"#,
        r#"--
do $$
begin
	case account_kind
		when
			'first_account_kind', 'second_account_kind', 'third_account_kind', 'fourth_account_kind',
			'fifth_account_kind', 'sixth_account_kind', 'seventh_account_kind'
		then
			y := 1;
	end case;
end
$$;
"#
    );
}
