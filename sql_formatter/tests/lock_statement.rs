#[macro_use]
mod helper;

#[test]
fn format_lock_statements() {
    assert_fmt!(
        r#"--
lock t
lock table only a, b in access share mode nowait;
lock t in share update exclusive mode
"#
    );
}

#[test]
fn format_lock_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
LOCK   TABLE   ~Таблица~   IN   SHARE   ROW   EXCLUSIVE   MODE;
"#,
        r#"--
lock table ~Таблица~ in share row exclusive mode;
"#
    );
}

#[test]
fn format_lock_long_header_wraps() {
    assert_fmt_eq!(
        r#"--
lock table accounts_summary_history, accounts_summary_archive, accounts_summary_current in share row exclusive mode nowait;
"#,
        r#"--
lock table accounts_summary_history, accounts_summary_archive, accounts_summary_current
in share row exclusive mode
nowait;
"#
    );
}

#[test]
fn format_lock_many_tables_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
lock table t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o, t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d in access exclusive mode;
"#,
        r#"--
lock table
	t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o,
	t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d
in access exclusive mode;
"#
    );
}
