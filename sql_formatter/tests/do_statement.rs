#[macro_use]
mod helper;

#[test]
fn format_do_statements() {
    assert_fmt!(
        r#"--
do $$ begin perform 1; end $$;
do $body$ begin null; end $body$ language plpgsql
"#
    );
}

#[test]
fn format_do_normalizes_keywords_and_keeps_the_body_verbatim() {
    assert_fmt_eq!(
        r#"--
DO   LANGUAGE   plpgsql   $$
BEGIN
    DROP TABLE IF EXISTS t;
        RAISE NOTICE 'done';
END $$;
"#,
        r#"--
do language plpgsql $$
BEGIN
    DROP TABLE IF EXISTS t;
        RAISE NOTICE 'done';
END $$;
"#
    );
}
