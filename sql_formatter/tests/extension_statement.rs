#[macro_use]
mod helper;

#[test]
fn format_extension_statements() {
    assert_fmt!(
        r#"--
create extension if not exists "uuid-ossp" with schema pg_catalog;
create extension hstore schema s cascade
drop extension if exists a, b restrict;
"#
    );
}

#[test]
fn format_extension_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
CREATE  EXTENSION IF NOT EXISTS "pgcrypto"  WITH  SCHEMA  pg_catalog;
DROP   EXTENSION   IF EXISTS   "pgcrypto"   CASCADE;
"#,
        r#"--
create extension if not exists "pgcrypto" with schema pg_catalog;
drop extension if exists "pgcrypto" cascade;
"#
    );
}

#[test]
fn format_create_extension_long_wraps() {
    assert_fmt_eq!(
        r#"--
create extension if not exists "some_really_long_extension_name_for_testing_wrapping_behaviour" with schema some_really_long_schema_name_too cascade;
"#,
        r#"--
create extension if not exists "some_really_long_extension_name_for_testing_wrapping_behaviour"
with schema some_really_long_schema_name_too
cascade;
"#
    );
}

#[test]
fn format_drop_extension_many_extensions_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
drop extension if exists ext_a, ext_b, ext_c, ext_d, ext_e, ext_f, ext_g, ext_h, ext_i, ext_j, ext_k, ext_l, ext_m, ext_n, ext_o, ext_p, ext_q, ext_r, ext_s cascade;
"#,
        r#"--
drop extension if exists
	ext_a, ext_b, ext_c, ext_d, ext_e, ext_f, ext_g, ext_h, ext_i, ext_j,
	ext_k, ext_l, ext_m, ext_n, ext_o, ext_p, ext_q, ext_r, ext_s
cascade;
"#
    );
}
