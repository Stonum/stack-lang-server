#[macro_use]
mod helper;

#[test]
fn format_transaction_statements() {
    assert_fmt!(
        r#"--
begin;
begin transaction;
start transaction;
commit work;
rollback
"#
    );
}

#[test]
fn format_transaction_statements_normalize_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
BEGIN   TRANSACTION;  COMMIT  TRANSACTION;
START  TRANSACTION
ROLLBACK   WORK;
"#,
        r#"--
begin transaction;
commit transaction;
start transaction
rollback work;
"#
    );
}

#[test]
fn format_transaction_around_long_update_wraps() {
    assert_fmt_eq!(
        r#"/*'multiplecommand'*/
BEGIN TRANSACTION;
ALTER TABLE accounts DISABLE TRIGGER ALL;
update accounts set owner_name = owners.full_name, owner_code = owners.code, owner_kind = owners.kind from owners where owners.id = accounts.owner and accounts.owner_name is null;
ALTER TABLE accounts ENABLE TRIGGER ALL;
COMMIT;
"#,
        r#"/*'multiplecommand'*/
begin transaction;
alter table accounts disable trigger all;
update accounts
set owner_name = owners.full_name, owner_code = owners.code, owner_kind = owners.kind
from owners
where owners.id = accounts.owner and accounts.owner_name is null;
alter table accounts enable trigger all;
commit;
"#
    );
}
