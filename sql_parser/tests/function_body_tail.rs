#[macro_use]
mod helper;

use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

fn count(src: &str, kind: &str) -> usize {
    let res = parse(src, postgres());
    assert_parser!(res);
    format!("{:#?}", res.syntax())
        .matches(&format!("{kind}@"))
        .count()
}

#[test]
fn test_similar_to() {
    let src = r#"select a similar to '\d*', b not similar to '%(x|y)%' from t"#;

    assert_eq!(count(src, "SQL_LIKE_EXPRESSION"), 2);
    assert_eq!(count(src, "TO_KW"), 2);
}

#[test]
fn test_bitwise_xor() {
    let src = "select (crc::bit(16) # get_byte(buf, i)::bit(16))::int, #tmp.a from #tmp";

    assert_eq!(count(src, "SQL_BINARY_EXPRESSION"), 1);
}

#[test]
fn test_substring_for_without_from() {
    let src = "select substring(data for 7000), substring(data from 2 for 3) from t";

    assert_eq!(count(src, "PSQL_SUBSTRING_EXPRESSION"), 2);
}

#[test]
fn test_keywords_after_dot() {
    let src = r#"update ~Таблица~ set "Полное" = kr.full, b = kr.order from kr"#;

    assert_eq!(count(src, "SQL_TABLE_COL_REFERENCE"), 2);
}

#[test]
fn test_tilde_name_as_value() {
    let src = r#"do $$
begin
    tmp_table = ~$#tmpParamLS~;
    perform f(~$#tmpLS~);
    select * from ~$#AppConfig~;
end $$"#;

    assert_eq!(count(src, "PSQL_TILDE_NAME_EXPRESSION"), 2);
}

#[test]
fn test_nested_join() {
    let src = r#"select *
from a sd
join b vz
join c ls on ls.row_id = vz.c_id
    on sd.b_id = vz.row_id
left join d on d.id = sd.d_id"#;

    assert_eq!(count(src, "SQL_JOIN_CLAUSE"), 3);
}

#[test]
fn test_absolute_value_operator() {
    let src = "select case when @k_i > 0 then 1 end, @ -5, a @> b from t where c = @d";

    assert_eq!(count(src, "SQL_UNARY_EXPRESSION"), 4);
}
