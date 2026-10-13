#[macro_use]
mod helper;

#[test]
fn format_declare_section() {
    assert_fmt!(
        r#"--
do $$
declare
	_xml xml;
	b_month timestamp := '2024-01-01';
	cnt int = 0;
	name text not null default 'x';
	k constant int := 1;
	c text collate "C";
begin
	delete from t;
end
$$;
"#
    );
}

#[test]
fn format_declare_section_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ DECLARE x INTEGER:=0; y TEXT NOT NULL DEFAULT 'a'; BEGIN TRUNCATE t; END $$;
"#,
        r#"--
do $$
declare
	x integer := 0;
	y text not null default 'a';
begin
	truncate t;
end
$$;
"#
    );
}

#[test]
fn format_long_declaration_default_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ declare total_amount numeric := (select sum(payments.amount) from payments where payments.owner = 1 and payments.created > now() - interval '30 days'); begin end $$;
"#,
        r#"--
do $$
declare
	total_amount numeric := (
		select sum(payments.amount) from payments where payments.owner = 1 and payments.created > now() - interval '30 days'
	);
begin
end
$$;
"#
    );
}
