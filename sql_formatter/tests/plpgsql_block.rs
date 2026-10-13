#[macro_use]
mod helper;

#[test]
fn format_plpgsql_block() {
    assert_fmt!(
        r#"--
do $$
begin
	drop table if exists t;
	begin
		delete from u;
	end;
end
$$;
"#
    );
}

#[test]
fn format_plpgsql_block_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO LANGUAGE plpgsql $$ BEGIN DROP TABLE IF EXISTS t; TRUNCATE u; END $$;
"#,
        r#"--
do language plpgsql $$
begin
	drop table if exists t;
	truncate u;
end
$$;
"#
    );
}

#[test]
fn format_plpgsql_empty_block() {
    assert_fmt_eq!(
        r#"--
do $body$ begin end $body$ language plpgsql
"#,
        r#"--
do $body$
begin
end
$body$ language plpgsql
"#
    );
}

#[test]
fn format_long_statement_inside_plpgsql_block_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin insert into accounts_history (id, owner, amount, created) select id, owner, amount, created from accounts where accounts.deleted is null and accounts.amount > 0 and accounts.owner is not null; end $$;
"#,
        r#"--
do $$
begin
	insert into accounts_history (id, owner, amount, created)
	select id, owner, amount, created
	from accounts
	where accounts.deleted is null
		and accounts.amount > 0
		and accounts.owner is not null;
end
$$;
"#
    );
}

#[test]
fn format_other_language_body_stays_verbatim() {
    assert_fmt!(
        r#"--
do $py$ print(1) $py$ language plpython3u
"#
    );
}
