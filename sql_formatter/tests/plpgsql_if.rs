#[macro_use]
mod helper;

#[test]
fn format_if_statement() {
    assert_fmt!(
        r#"--
do $$
begin
	if x = 1 then
		y := 1;
	elsif x = 2 then
		y := 2;
	else
		delete from t;
	end if;
end
$$;
"#
    );
}

#[test]
fn format_if_statement_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN IF NOT EXISTS (SELECT 1 FROM t) THEN INSERT INTO t (a) VALUES (1); ELSEIF x THEN ELSE TRUNCATE t; END IF; END $$;
"#,
        r#"--
do $$
begin
	if not exists (select 1 from t) then
		insert into t (a) values (1);
	elseif x then
	else
		truncate t;
	end if;
end
$$;
"#
    );
}

#[test]
fn format_nested_if() {
    assert_fmt_eq!(
        r#"--
do $$ begin if a then if b then delete from t; end if; end if; end $$;
"#,
        r#"--
do $$
begin
	if a then
		if b then
			delete from t;
		end if;
	end if;
end
$$;
"#
    );
}

#[test]
fn format_long_if_condition_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin if exists (select 1 from accounts where accounts.owner = 1) and accounts_total > 100 and not accounts_locked then delete from t; end if; end $$;
"#,
        r#"--
do $$
begin
	if exists (select 1 from accounts where accounts.owner = 1)
		and accounts_total > 100
		and not accounts_locked then
		delete from t;
	end if;
end
$$;
"#
    );
}
