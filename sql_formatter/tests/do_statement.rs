#[macro_use]
mod helper;

#[test]
fn format_do_statements() {
    assert_fmt!(
        r#"--
do 'begin null; end';
do $py$ print(1) $py$ language plpython3u
"#
    );
}

#[test]
fn format_do_normalizes_keywords_and_keeps_another_language_body_verbatim() {
    assert_fmt_eq!(
        r#"--
DO   LANGUAGE   plpython3u   $$
import json
    print(json.dumps({}))
$$;
"#,
        r#"--
do language plpython3u $$
import json
    print(json.dumps({}))
$$;
"#
    );
}
