#[macro_use]
mod helper;

#[test]
fn format_named_arguments() {
    assert_fmt!(
        r#"--
select make_interval(hours => 1, mins => :2)
"#
    );
}

#[test]
fn format_named_arguments_normalizes_spacing() {
    assert_fmt_eq!(
        r#"--
SELECT make_interval( hours=>1,mins   =>   x*2 )
"#,
        r#"--
select make_interval(hours => 1, mins => x * 2)
"#
    );
}

#[test]
fn format_named_arguments_wrap_when_too_long() {
    assert_fmt_eq!(
        r#"--
select make_interval(years => :years_value, months => :months_value, days => :days_value, hours => :hours_value, mins => :minutes)
"#,
        r#"--
select make_interval(
	years => :years_value, months => :months_value, days => :days_value, hours => :hours_value, mins => :minutes
)
"#
    );
}

#[test]
fn format_many_named_arguments_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
select make_interval(years => 1, months => 2, weeks => 3, days => 4, hours => 5, mins => 6, secs => 7, a => 8, b => 9, c => 10, d => 11, e => 12, f => 13, g => 14, h => 15, i => 16, j => 17, k => 18, l => 19, m => 20)
"#,
        r#"--
select make_interval(
	years => 1, months => 2, weeks => 3, days => 4, hours => 5, mins => 6, secs => 7, a => 8, b => 9,
	c => 10, d => 11, e => 12, f => 13, g => 14, h => 15, i => 16, j => 17, k => 18, l => 19, m => 20
)
"#
    );
}
