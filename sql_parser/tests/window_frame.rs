#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

#[test]
fn test_window_frames() {
    for src in [
        "select sum(a) over (order by b rows between unbounded preceding and current row) from t",
        "select sum(a) over (partition by c order by b rows between unbounded preceding and 1 preceding) from t",
        "select avg(a) over (order by b range 3 preceding) from t",
        "select count(*) over (rows between current row and unbounded following) from t",
        "select sum(a) OVER (ORDER BY b GROUPS BETWEEN 1 PRECEDING AND 2 FOLLOWING) from t",
    ] {
        let res = parse(src, postgres());

        assert_parser!(res);
        let tree = format!("{:#?}", res.syntax());
        assert!(tree.contains("SQL_WINDOW_FRAME_CLAUSE"), "{src}");
    }
}

#[test]
fn test_window_frame_keywords_are_still_valid_names() {
    let res = parse(
        "select groups, unbounded, preceding, following, current from t",
        postgres(),
    );

    assert_parser!(res);
}

#[test]
fn test_window_frame_errors() {
    for src in [
        "select sum(a) over (rows between unbounded preceding) from t",
        "select sum(a) over (rows unbounded) from t",
        "select sum(a) over (rows 1) from t",
        "select sum(a) over (rows current) from t",
    ] {
        let res = parse(src, postgres());

        assert!(res.has_errors(), "{src}");
    }
}
