#[macro_use]
mod helper;

#[test]
fn format_loop_normalizes_layout_and_case() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN LOOP i := i + 1; CONTINUE WHEN i < 3; EXIT  WHEN i > 10; END LOOP; LOOP END LOOP; END $$;
"#,
        r#"--
do $$
begin
	loop
		i := i + 1;
		continue when i < 3;
		exit when i > 10;
	end loop;
	loop
	end loop;
end
$$;
"#
    );
}

#[test]
fn format_labels() {
    assert_fmt_eq!(
        r#"--
do $$ <<main>> begin <<outer_loop>> loop <<inner_loop>> loop exit outer_loop; end loop inner_loop; end loop outer_loop; <<nested>> begin exit nested; end nested; end main $$;
"#,
        r#"--
do $$
<<main>>
begin
	<<outer_loop>>
	loop
		<<inner_loop>>
		loop
			exit outer_loop;
		end loop inner_loop;
	end loop outer_loop;
	<<nested>>
	begin
		exit nested;
	end nested;
end main
$$;
"#
    );
}

#[test]
fn format_long_exit_condition_wraps() {
    assert_fmt_eq!(
        r#"--
do $$ begin loop exit when processed_accounts_count >= total_accounts_count or now() - started_at > interval '30 minutes' or cancel_requested; end loop; end $$;
"#,
        r#"--
do $$
begin
	loop
		exit when processed_accounts_count >= total_accounts_count
			or now() - started_at > interval '30 minutes'
			or cancel_requested;
	end loop;
end
$$;
"#
    );
}
