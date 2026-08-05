#[macro_use]
mod helper;

#[test]
fn format_create_policy_bare() {
    assert_fmt!(
        r#"--
create policy p on t
"#
    );
}

#[test]
fn format_create_policy_for_all() {
    assert_fmt!(
        r#"--
create policy p on t for all
"#
    );
}

#[test]
fn format_create_policy_using_clause() {
    assert_fmt!(
        r#"--
create policy p on t using (a = 1)
"#
    );
}

#[test]
fn format_create_policy_for_and_using() {
    assert_fmt!(
        r#"--
create policy p on t for all using (a = 1);
"#
    );
}

#[test]
fn format_create_policy_with_check_clause() {
    assert_fmt!(
        r#"--
create policy p on t with check (a = 1)
"#
    );
}

#[test]
fn format_create_policy_using_and_with_check() {
    assert_fmt!(
        r#"--
create policy p on t for all using (a = 1) with check (b = 2);
"#
    );
}

#[test]
fn format_create_policy_with_check_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   POLICY p   ON t   WITH   CHECK(a = 1);
"#,
        r#"--
create policy p on t with check (a = 1);
"#
    );
}

#[test]
fn format_create_policy_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE   POLICY p   ON t   FOR ALL   USING(a = 1);
"#,
        r#"--
create policy p on t for all using (a = 1);
"#
    );
}

#[test]
fn format_drop_policy_bare() {
    assert_fmt!(
        r#"--
drop policy p on t
"#
    );
}

#[test]
fn format_drop_policy_if_exists() {
    assert_fmt!(
        r#"--
drop policy if exists p on t;
"#
    );
}

#[test]
fn format_drop_policy_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
DROP   POLICY   IF EXISTS   p   ON   t;
"#,
        r#"--
drop policy if exists p on t;
"#
    );
}
