#[macro_use]
mod helper;

#[test]
fn format_foreach_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN FOREACH name IN ARRAY string_to_array(s,',') LOOP x := x || name; END LOOP; FOREACH r SLICE 1 IN ARRAY m LOOP END LOOP; END $$;
"#,
        r#"--
do $$
begin
	foreach name in array string_to_array(s, ',') loop
		x := x || name;
	end loop;
	foreach r slice 1 in array m loop
	end loop;
end
$$;
"#
    );
}

#[test]
fn format_long_foreach_array_moves_under_the_header() {
    assert_fmt_eq!(
        r#"--
do $$ begin foreach table_name in array array(select tbl.relname from pg_catalog.pg_trigger tr join pg_catalog.pg_class tbl on tbl.oid = tr.tgrelid) loop end loop; end $$;
"#,
        r#"--
do $$
begin
	foreach table_name in array
		array(
			select tbl.relname
			from pg_catalog.pg_trigger tr
			join pg_catalog.pg_class tbl on tbl.oid = tr.tgrelid
		)
	loop
	end loop;
end
$$;
"#
    );
}

#[test]
fn format_array_subquery_expression() {
    assert_fmt_eq!(
        r#"--
SELECT ARRAY( SELECT a FROM t ), array[1, 2];
"#,
        r#"--
select array(select a from t), array[1, 2];
"#
    );
}
