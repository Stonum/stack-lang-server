#[macro_use]
mod helper;

#[test]
fn format_trailing_into() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN SELECT a FROM t WHERE b = 1 INTO x; SELECT string_agg(indexrelid::regclass::text, ', ') FROM pg_index i LEFT JOIN pg_depend d ON d.objid = i.indexrelid WHERE d.objid IS NULL INTO _sql; END $$;
"#,
        r#"--
do $$
begin
	select a from t where b = 1 into x;
	select string_agg(indexrelid::regclass::text, ', ')
	from pg_index i
	left join pg_depend d on d.objid = i.indexrelid
	where d.objid is null
	into _sql;
end
$$;
"#
    );
}

#[test]
fn format_insert_overriding() {
    assert_fmt_eq!(
        r#"--
INSERT INTO t (a, b) OVERRIDING SYSTEM VALUE SELECT 1, 2;
INSERT INTO accounts (row_id, owner_id, amount, created_at) OVERRIDING SYSTEM VALUE SELECT src.row_id, src.owner_id, src.amount, src.created_at FROM source_accounts src;
"#,
        r#"--
insert into t (a, b) overriding system value select 1, 2;
insert into accounts (row_id, owner_id, amount, created_at)
overriding system value
select src.row_id, src.owner_id, src.amount, src.created_at from source_accounts src;
"#
    );
}

#[test]
fn format_multi_column_set_item() {
    assert_fmt_eq!(
        r#"--
UPDATE t SET (a,b) = row(1, 2);
UPDATE accounts pp SET (opened_at, closed_at) = (SELECT s.opened_at, s.closed_at FROM schedule s WHERE s.account_id = pp.row_id AND s.kind = 1);
"#,
        r#"--
update t set (a, b) = row(1, 2);
update accounts pp
set (opened_at, closed_at) = (
	select s.opened_at, s.closed_at from schedule s where s.account_id = pp.row_id and s.kind = 1
);
"#
    );
}

#[test]
fn format_repeated_declare() {
    assert_fmt_eq!(
        r#"--
DO $$ DECLARE a int = 1; DECLARE b text = 'x'; BEGIN END $$;
"#,
        r#"--
do $$
declare
	a int = 1;
	declare
	b text = 'x';
begin
end
$$;
"#
    );
}
