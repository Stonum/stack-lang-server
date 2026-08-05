#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn mlang() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_create_trigger_after_insert() {
    let res = parse(
        "create trigger t after insert on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_before_update() {
    let res = parse(
        "create trigger t before update on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_multiple_events() {
    let res = parse(
        "create trigger t after insert or update or delete on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_update_of_single_column() {
    let res = parse(
        "create trigger t after update of a on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_update_of_multiple_columns() {
    let res = parse(
        "create trigger t after update of a, b, c on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_insert_or_update_of_column() {
    let res = parse(
        "create trigger t after insert or update of a on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_update_of_quoted_column_then_or_delete() {
    let res = parse(
        r#"create trigger t after update of "Col A" or delete on foo for each row execute function f()"#,
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_update_no_of_clause_still_works() {
    let res = parse(
        "create trigger t after update on foo for each row execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_update_of_trigger_shape() {
    // Representative of the real shape this was fixed for: `UPDATE OF
    // "column"` with a quoted (Cyrillic) column name, quoted trigger and
    // table names, no trailing `;`, followed by `ON`/`FOR EACH ROW` on
    // their own lines.
    let res = parse(
        r#"create trigger "SomeTrigger" after insert or update of "SomeColumn"
on ~$some_table~ for each row
execute function ~$some_func~();"#,
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_execute_procedure() {
    let res = parse(
        "create trigger t after insert on foo for each row execute procedure f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_function_with_arguments() {
    let res = parse(
        "create trigger t after insert on foo for each row execute function f('a', 'b')",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_for_each_statement() {
    let res = parse(
        "create trigger t after delete on foo for each statement execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_no_for_each_clause() {
    let res = parse(
        "create trigger t after insert on foo execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_when_clause() {
    let res = parse(
        "create trigger t after update on foo for each row when (old.a > 0) execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_when_clause_no_for_each() {
    let res = parse(
        "create trigger t after insert on foo when (new.a > 0) execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_when_clause_after_referencing() {
    let res = parse(
        "create trigger t after update on foo referencing old table as deleted new table as inserted when (new.a > 0) execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_when_clause_referencing_both_old_and_new() {
    let res = parse(
        "create trigger t after update on foo for each row when (old.a <> new.a) execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_old_and_new_are_ordinary_identifiers_outside_referencing() {
    // `OLD`/`NEW` are only ever meaningful (and only checked by text, not
    // by dedicated keyword tokens) inside a `REFERENCING` item -- elsewhere
    // they must parse as any other column/table reference would.
    let res = parse(
        "select old, new, old.a, new.b from old join new on old.id = new.id",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_referencing_case_insensitive() {
    let res = parse(
        "create trigger t after update on foo referencing OLD table as deleted NEW table as inserted for each statement execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_referencing_old_table() {
    let res = parse(
        "create trigger t after delete on foo referencing old table as deleted for each statement execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_referencing_new_table() {
    let res = parse(
        "create trigger t after insert on foo referencing new table as inserted for each statement execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_referencing_both_tables() {
    let res = parse(
        "create trigger t after update on foo referencing new table as inserted old table as deleted for each statement execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_referencing_without_for_each() {
    let res = parse(
        "create trigger t after update on foo referencing old table as deleted new table as inserted execute function f()",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_quoted_name() {
    let res = parse(
        r#"create trigger "MyTrigger" after insert on foo for each row execute function f();"#,
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_followed_by_another_statement() {
    let res = parse(
        "create trigger t after insert on foo execute function f(); select a from foo;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_create_trigger_tilde_names_in_mlang_dialect() {
    let res = parse(
        "create trigger t after insert on ~foo~ for each row execute function ~$f~()",
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_trigger_bare() {
    let res = parse("drop trigger t on foo", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_trigger_if_exists() {
    let res = parse("drop trigger if exists t on foo;", SqlFileSource::script());

    assert_parser!(res);
}

#[test]
fn test_drop_trigger_cascade() {
    let res = parse(
        "drop trigger if exists t on foo cascade;",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_trigger_quoted_name() {
    let res = parse(
        r#"drop trigger if exists "MyTrigger" on foo;"#,
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_drop_trigger_tilde_name_in_mlang_dialect() {
    let res = parse("drop trigger if exists t on ~foo~;", mlang());

    assert_parser!(res);
}

#[test]
fn test_create_and_drop_trigger_do_not_shadow_policy_view() {
    let res = parse(
        "drop policy if exists p on foo; drop trigger if exists t on foo; create policy p on foo for all; create trigger t after insert on foo execute function f();",
        SqlFileSource::script(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_trigger_attachment_shape() {
    // Representative of a real trigger attachment: multiple events,
    // `FOR EACH ROW`, `EXECUTE FUNCTION` with a tilde-qualified function.
    let res = parse(
        r#"drop trigger if exists "t_change_trig" on ~business_process~;
create trigger "t_change_trig" after insert or update or delete
on ~business_process~ for each row
execute function ~$business_process_log~();"#,
        mlang(),
    );

    assert_parser!(res);
}

#[test]
fn test_realistic_conditional_trigger_shape() {
    // Representative of a real conditional row-level trigger: `WHEN (...)`
    // referencing `OLD`/`NEW` columns, placed between `FOR EACH ROW` and
    // `EXECUTE FUNCTION`.
    let res = parse(
        r#"create trigger "some_conditional_trig" after update or delete
on ~some_table~ for each row
when ( old."SomeFlag" > 0 )
execute function ~$some_func~();"#,
        mlang(),
    );

    assert_parser!(res);
}
