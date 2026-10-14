#[macro_use]
mod helper;

#[test]
fn format_strings_with_backslashes() {
    // A backslash only escapes inside `E'...'`; in `'...'` it is an ordinary
    // character, so `'\'` is a complete one-character string.
    assert_fmt_eq!(
        r#"--
SELECT REPLACE(path, '\', '\\'), right(path, 1) != '\', E'it\'s' FROM t;
"#,
        r#"--
select REPLACE(path, '\', '\\'), right(path, 1) != '\', E'it\'s' from t;
"#
    );
}

#[test]
fn format_function_body_with_backslash_strings() {
    assert_fmt_eq!(
        r#"--
create function f(path text) returns text as $$ begin if right(path, 1) != '/' and right(path, 1) != '\' then path = path || '/'; end if; return path; end $$ language plpgsql;
"#,
        r#"--
create function f(path text) returns text as $$
begin
	if right(path, 1) != '/' and right(path, 1) != '\' then
		path = path || '/';
	end if;
	return path;
end
$$ language plpgsql;
"#
    );
}
