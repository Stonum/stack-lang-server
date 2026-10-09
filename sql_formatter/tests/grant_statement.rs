#[macro_use]
mod helper;

#[test]
fn format_grant_all_on_table_to_public() {
    assert_fmt!(
        r#"--
grant all on table foo to public
"#
    );
}

#[test]
fn format_grant_without_table_keyword() {
    assert_fmt!(
        r#"--
grant all on foo to public
"#
    );
}

#[test]
fn format_grant_multiple_objects_and_grantees() {
    assert_fmt!(
        r#"--
grant all on table foo, bar to public, other_role;
"#
    );
}

#[test]
fn format_grant_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
GRANT   ALL   ON   TABLE   foo   TO   public;
"#,
        r#"--
grant all on table foo to public;
"#
    );
}

#[test]
fn format_grant_and_revoke_statements() {
    assert_fmt!(
        r#"--
grant select, update (a, b) on table t to r with grant option;
grant usage on schema s to r
grant some_role to r
revoke all privileges on all sequences in schema "s" from "u";
revoke grant option for execute on function f from r cascade;
revoke first_role, second_role from u
"#
    );
}

#[test]
fn format_revoke_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
REVOKE  ALL  PRIVILEGES  ON  ALL  TABLES  IN  SCHEMA  "s"  FROM  "u"  CASCADE;
GRANT   SELECT,INSERT   ON   t   TO   r   WITH   GRANT   OPTION;
"#,
        r#"--
revoke all privileges on all tables in schema "s" from "u" cascade;
grant select, insert on t to r with grant option;
"#
    );
}

#[test]
fn format_grant_long_object_names_wrap() {
    assert_fmt_eq!(
        r#"--
grant select, insert, update, delete on table accounts_summary_history, accounts_summary_archive, accounts_summary_current to reporting_role, auditing_role;
"#,
        r#"--
grant select, insert, update, delete on table
	accounts_summary_history, accounts_summary_archive, accounts_summary_current
to reporting_role, auditing_role;
"#
    );
}

#[test]
fn format_revoke_all_in_schema_long_wraps() {
    assert_fmt_eq!(
        r#"--
revoke all privileges on all tables in schema first_schema, second_schema, third_schema, fourth_schema, fifth_schema, sixth_schema from some_role cascade;
"#,
        r#"--
revoke all privileges on all tables in schema
	first_schema, second_schema, third_schema, fourth_schema, fifth_schema, sixth_schema
from some_role
cascade;
"#
    );
}

#[test]
fn format_grant_many_tables_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
grant select on table t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o, t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d to r_a, r_b, r_c;
"#,
        r#"--
grant select on table
	t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o,
	t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d
to r_a, r_b, r_c;
"#
    );
}

#[test]
fn format_revoke_many_grantees_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
revoke select on t from r_a, r_b, r_c, r_d, r_e, r_f, r_g, r_h, r_i, r_j, r_k, r_l, r_m, r_n, r_o, r_p, r_q, r_r, r_s, r_t, r_u, r_v, r_w, r_x, r_y, r_z, s_a, s_b, s_c, s_d cascade;
"#,
        r#"--
revoke select on t
from
	r_a, r_b, r_c, r_d, r_e, r_f, r_g, r_h, r_i, r_j, r_k, r_l, r_m, r_n, r_o,
	r_p, r_q, r_r, r_s, r_t, r_u, r_v, r_w, r_x, r_y, r_z, s_a, s_b, s_c, s_d
cascade;
"#
    );
}
