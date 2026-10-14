#[macro_use]
mod helper;

#[test]
fn format_locking_clauses() {
    assert_fmt_eq!(
        r#"--
SELECT * FROM t FOR UPDATE;
SELECT * FROM t FOR NO KEY UPDATE OF t,t2 NOWAIT;
SELECT * FROM t FOR SHARE SKIP LOCKED;
"#,
        r#"--
select * from t for update;
select * from t for no key update of t, t2 nowait;
select * from t for share skip locked;
"#
    );
}

#[test]
fn format_long_query_with_locking_clause_wraps() {
    assert_fmt_eq!(
        r#"--
select accounts.row_id, accounts.amount from accounts where accounts.owner = 1 and accounts.kind = 2 order by accounts.row_id for update skip locked;
"#,
        r#"--
select accounts.row_id, accounts.amount
from accounts
where accounts.owner = 1 and accounts.kind = 2
order by accounts.row_id
for update skip locked;
"#
    );
}
