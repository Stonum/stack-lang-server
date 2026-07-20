#[macro_use]
mod helper;

/// Proves the formatter actually normalizes messy input, not just
/// round-trips already-correct text: extra/irregular whitespace around
/// commas and keywords, mixed-case keywords (normalized to lowercase),
/// inconsistent original indentation, and an artificially
/// line-broken 2-condition `and` chain that collapses back onto one line
#[test]
fn format_normalizes_messy_input() {
    assert_fmt_eq!(
        r#"SELECT    a,   b,c
FROM    t1
   JOIN t2 ON t1.id = t2.id
WhErE a > 1
   AND b < 2
"#,
        r#"select a, b, c
from t1
join t2 on t1.id = t2.id
where a > 1 and b < 2
"#
    );
}
