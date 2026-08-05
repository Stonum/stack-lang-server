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
fn format_create_trigger_multiple_events_stay_flat_when_execute_call_wraps() {
    // Regression test: a long `execute function(...)` argument list must
    // not leak into the (short, fixed-vocabulary) event list's own wrap
    // decision -- `after insert or update or delete` always stays on one
    // line, and the rest of the header wraps clause-by-clause instead of
    // collapsing everything after the last event onto one glued, unindented
    // line.
    assert_fmt!(
        r#"--
create trigger "T_LOG_ALLTRIG" after insert or update or delete
on foo
for each row
execute function audit_general_trigger_js(
	'row_id, name, folder_add, number, code, category, other_field, another_field', 'row_id, name, folder_add, number, code'
)
"#
    );
}

#[test]
fn format_create_trigger_update_of_single_column() {
    assert_fmt!(
        r#"--
create trigger t after update of a on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_update_of_multiple_columns() {
    assert_fmt!(
        r#"--
create trigger t after update of a, b, c on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_insert_or_update_of_column() {
    assert_fmt!(
        r#"--
create trigger t after insert or update of a on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_update_of_normalizes_case() {
    assert_fmt_eq!(
        r#"--
create trigger t after UPDATE OF a,b on foo for each row execute function f()
"#,
        r#"--
create trigger t after update of a, b on foo for each row execute function f()
"#
    );
}

#[test]
fn format_create_trigger_when_clause() {
    assert_fmt!(
        r#"--
create trigger t after update on foo for each row when (old.a > 0) execute function f()
"#
    );
}

#[test]
fn format_create_trigger_when_clause_no_for_each() {
    assert_fmt!(
        r#"--
create trigger t after insert on foo when (new.a > 0) execute function f()
"#
    );
}

#[test]
fn format_create_trigger_when_clause_normalizes_case_and_spacing() {
    assert_fmt_eq!(
        r#"--
create trigger t after update on foo for each row WHEN(old.a > 0) execute function f()
"#,
        r#"--
create trigger t after update on foo for each row when (old.a > 0) execute function f()
"#
    );
}

#[test]
fn format_create_trigger_referencing_old_new_case_is_preserved() {
    // `OLD`/`NEW` in a `REFERENCING` item are now an ordinary identifier
    // token (not a dedicated keyword), so -- unlike real keywords -- their
    // casing is preserved verbatim rather than forced to lowercase, same
    // as any other identifier this formatter never rewrites.
    assert_fmt!(
        r#"--
create trigger t after update on foo referencing OLD table as deleted for each row execute function f()
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
create trigger t after update
on foo
referencing new table as inserted old table as deleted
for each statement
execute function f()
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
