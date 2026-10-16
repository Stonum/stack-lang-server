#[macro_use]
mod helper;

#[test]
fn format_execute_string_query() {
    assert_fmt_eq!(
        r#"--
do $$ begin EXECUTE 'UPDATE ~Таблица~ SET "Поле" = 1 WHERE row_id IN (SELECT row_id FROM deleted) AND "Поле" <> ''x'' AND "Поле" IS NOT NULL'; execute 'ANALYSE ' || tbl; end $$;
"#,
        r#"--
do $$
begin
	execute '
		update ~Таблица~
		set "Поле" = 1
		where row_id in (select row_id from deleted)
			and "Поле" <> ''x''
			and "Поле" is not null
	';
	execute 'analyse ' || tbl;
end
$$;
"#
    );
}

#[test]
fn format_execute_chain_with_name_holes() {
    assert_fmt_eq!(
        r#"--
do $$ begin
execute 'insert into ' || target_table || '
         ( account_owner, account_kind, account_amount, account_created_at, account_parent_id, account_name )
         select a.owner, ' || kind || ' as kind, a.amount, a.created_at, a.parent_id, a.name
         from ' || source_table || ' a where a.created_at >= ''' || date_from || '''';
end $$;
"#,
        r#"--
do $$
begin
	execute '
		insert into ' || target_table || ' (
			account_owner, account_kind, account_amount, account_created_at, account_parent_id, account_name
		)
		select a.owner, ' || kind || ' as kind, a.amount, a.created_at, a.parent_id, a.name
		from ' || source_table || ' a
		where a.created_at >= ''' || date_from || '''
	';
end
$$;
"#
    );
}

#[test]
fn format_execute_chain_with_fragment_hole() {
    assert_fmt_eq!(
        r#"--
do $$ begin
execute 'select accounts.owner, accounts.amount, accounts.created_at, accounts.parent_id from accounts join payments p on p.owner = accounts.owner ' || extra_condition || ' and p.amount > 0 where accounts.kind = 1';
end $$;
"#,
        r#"--
do $$
begin
	execute '
		select accounts.owner, accounts.amount, accounts.created_at, accounts.parent_id
		from accounts
		join payments p on p.owner = accounts.owner ' || extra_condition || ' and p.amount > 0
		where accounts.kind = 1
	';
end
$$;
"#
    );
}

#[test]
fn format_for_and_return_query_execute() {
    assert_fmt_eq!(
        r#"--
do $$ begin
FOR r IN EXECUTE 'SELECT * FROM ' || tbl LOOP NULL; END LOOP;
RETURN QUERY EXECUTE 'SELECT a FROM ' || tbl || ' WHERE b = $1' USING x;
end $$;
"#,
        r#"--
do $$
begin
	for r in execute 'select * from ' || tbl loop
		null;
	end loop;
	return query execute 'select a from ' || tbl || ' where b = $1' using x;
end
$$;
"#
    );
}

#[test]
fn format_execute_keeps_unformattable_commands() {
    assert_fmt_eq!(
        r#"--
do $$ begin
execute 'not a query ' || x;
execute 'select ' || a || b;
execute E'select \'a\'';
execute 'select ' || /* c */ a;
execute 'select (' || e || ') * 2';
execute $q$select 1$q$;
execute cmd;
end $$;
"#,
        r#"--
do $$
begin
	execute 'not a query ' || x;
	execute 'select ' || a || b;
	execute E'select \'a\'';
	execute 'select ' || /* c */ a;
	execute 'select (' || e || ') * 2';
	execute $q$select 1$q$;
	execute cmd;
end
$$;
"#
    );
}

#[test]
fn format_execute_keeps_parentheses_around_holes() {
    assert_fmt_eq!(
        r#"--
do $$ begin EXECUTE 'SELECT (' || e || ') * 2, ((a)) FROM t WHERE (x = ' || v || ')'; end $$;
"#,
        r#"--
do $$
begin
	execute 'select (' || e || ') * 2, a from t where (x = ' || v || ')';
end
$$;
"#
    );
}
