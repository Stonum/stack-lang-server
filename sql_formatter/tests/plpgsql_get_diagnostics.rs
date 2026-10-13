#[macro_use]
mod helper;

#[test]
fn format_get_diagnostics_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN GET DIAGNOSTICS cnt=ROW_COUNT; GET STACKED DIAGNOSTICS msg := MESSAGE_TEXT , st = RETURNED_SQLSTATE; END $$;
"#,
        r#"--
do $$
begin
	get diagnostics cnt = ROW_COUNT;
	get stacked diagnostics msg := MESSAGE_TEXT, st = RETURNED_SQLSTATE;
end
$$;
"#
    );
}

#[test]
fn format_many_diagnostics_items_wrap() {
    assert_fmt_eq!(
        r#"--
do $$ begin get stacked diagnostics error_message = message_text, error_detail = pg_exception_detail, error_hint = pg_exception_hint, error_context = pg_exception_context; end $$;
"#,
        r#"--
do $$
begin
	get stacked diagnostics
		error_message = message_text, error_detail = pg_exception_detail,
		error_hint = pg_exception_hint, error_context = pg_exception_context;
end
$$;
"#
    );
}
