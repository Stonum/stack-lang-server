#[macro_use]
mod helper;

#[test]
fn format_tilde_subscripts() {
    assert_fmt_eq!(
        r#"--
SELECT coalesce(captions~[ i ]~,''), (xpath('/a/@b', doc))~[1]~::TEXT, (a[ i ]) [1], ((x)) .f FROM t WHERE tg_argv~[0]~ IS NOT NULL;
"#,
        r#"--
select coalesce(captions~[i]~, ''), (xpath('/a/@b', doc))~[1]~::text, (a[i])[1], (x).f
from t
where tg_argv~[0]~ is not null;
"#
    );
}

#[test]
fn format_tilde_subscript_assignment() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN infoFields~[0]~='row_guid'; x := m~[ (i) ]~; END $$;
"#,
        r#"--
do $$
begin
	infoFields~[0]~ = 'row_guid';
	x := m~[i]~;
end
$$;
"#
    );
}

#[test]
fn format_long_expression_with_tilde_subscripts_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin if (tg_argv~[0]~ is not null) and (tg_argv~[0]~ != '') and (tg_argv~[1]~ is not null) and (tg_argv~[1]~ != '') then null; end if; end $$;
"#,
        r#"--
do $$
begin
	if tg_argv~[0]~ is not null
		and tg_argv~[0]~ != ''
		and tg_argv~[1]~ is not null
		and tg_argv~[1]~ != ''
	then
		null;
	end if;
end
$$;
"#
    );
}
