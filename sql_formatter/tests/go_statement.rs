#[macro_use]
mod helper;

#[test]
fn format_go_separated_batches() {
    assert_fmt!(
        r#"--
drop view if exists ~Представление~
go
create view ~Представление~ as select a from t
go
grant all on table ~Представление~ to public;
go
"#
    );
}

#[test]
fn format_go_normalizes_case() {
    assert_fmt_eq!(
        r#"--
select a from t
GO
select b from u
Go
"#,
        r#"--
select a from t
go
select b from u
go
"#
    );
}

#[test]
fn format_go_between_wrapped_statements() {
    assert_fmt_eq!(
        r#"--
truncate table t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o, t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d
GO
update accounts set owner_name = owners.full_name, owner_code = owners.code, owner_kind = owners.kind from owners where owners.id = accounts.owner
GO
"#,
        r#"--
truncate table
	t_a, t_b, t_c, t_d, t_e, t_f, t_g, t_h, t_i, t_j, t_k, t_l, t_m, t_n, t_o,
	t_p, t_q, t_r, t_s, t_t, t_u, t_v, t_w, t_x, t_y, t_z, u_a, u_b, u_c, u_d
go
update accounts
set owner_name = owners.full_name, owner_code = owners.code, owner_kind = owners.kind
from owners
where owners.id = accounts.owner
go
"#
    );
}
