#[macro_use]
mod helper;

#[test]
fn format_assignments() {
    assert_fmt!(
        r#"--
do $$
begin
	x := 1;
	x = x + 1;
	r.f := 'a';
	a[1] := (select count(*) from t);
end
$$;
"#
    );
}

#[test]
fn format_assignment_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
do $$ begin x:=1; y  =  x+1; end $$;
"#,
        r#"--
do $$
begin
	x := 1;
	y = x + 1;
end
$$;
"#
    );
}

#[test]
fn format_long_assignment_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin total := (select sum(payments.amount) from payments where payments.owner = 1 and payments.created > now() - interval '30 days' and payments.kind = 2); end $$;
"#,
        r#"--
do $$
begin
	total := (
		select sum(payments.amount)
		from payments
		where payments.owner = 1
			and payments.created > now() - interval '30 days'
			and payments.kind = 2
	);
end
$$;
"#
    );
}
