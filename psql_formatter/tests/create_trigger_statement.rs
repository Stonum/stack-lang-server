#[macro_use]
mod helper;

#[test]
fn format_create_trigger_after_insert() {
    assert_fmt!(
        r#"--
create trigger t after insert on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_before_update() {
    assert_fmt!(
        r#"--
create trigger t before update on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_multiple_events() {
    assert_fmt!(
        r#"--
create trigger t after insert or update or delete on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_execute_procedure() {
    assert_fmt!(
        r#"--
create trigger t after insert on foo for each row execute procedure f()
"#
    );
}

#[test]
fn format_create_trigger_no_for_each_clause() {
    assert_fmt!(
        r#"--
create trigger t after insert on foo execute function f()
"#
    );
}

#[test]
fn format_create_trigger_referencing_both_tables() {
    assert_fmt!(
        r#"--
create trigger t after update on foo referencing new table as ins old table as del execute function f()
"#
    );
}

#[test]
fn format_create_trigger_referencing_wraps_when_too_long() {
    assert_fmt!(
        r#"--
create trigger t after update on foo referencing new table as inserted
old table as deleted for each statement execute function f()
"#
    );
}

#[test]
fn format_create_trigger_quoted_name() {
    assert_fmt!(
        r#"--
create trigger "MyTrigger" after insert on foo for each row execute function f();
"#
    );
}

#[test]
fn format_create_trigger_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   TRIGGER t   AFTER INSERT OR UPDATE   ON foo   FOR EACH ROW   EXECUTE FUNCTION f();
"#,
        r#"--
create trigger t after insert or update on foo for each row execute function f();
"#
    );
}

#[test]
fn format_drop_trigger_bare() {
    assert_fmt!(
        r#"--
drop trigger t on foo
"#
    );
}

#[test]
fn format_drop_trigger_if_exists() {
    assert_fmt!(
        r#"--
drop trigger if exists t on foo;
"#
    );
}

#[test]
fn format_drop_trigger_cascade() {
    assert_fmt!(
        r#"--
drop trigger if exists t on foo cascade;
"#
    );
}

#[test]
fn format_drop_trigger_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP   TRIGGER   IF EXISTS   t   ON   foo   CASCADE;
"#,
        r#"--
drop trigger if exists t on foo cascade;
"#
    );
}
