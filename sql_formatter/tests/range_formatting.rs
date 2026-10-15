use biome_rowan::{TextRange, TextSize};
use sql_formatter::{SqlFormatOptions, format_node, format_range};
use sql_parser::parse;
use sql_syntax::{SqlDialect, SqlFileSource};

fn postgres() -> SqlFileSource {
    SqlFileSource::script()
        .with_dialect(SqlDialect::Postgres)
        .with_mlang_extension(true)
}

/// Formats the statement around `marker` as range formatting does.
fn format_at(src: &str, marker: &str) -> String {
    let at = src.find(marker).expect("marker") as u32;
    let range = TextRange::new(TextSize::from(at), TextSize::from(at + marker.len() as u32));
    let tree = parse(src, postgres());
    let printed = format_range(SqlFormatOptions::new(postgres()), &tree.syntax(), range).unwrap();
    let range = printed.range().expect("formatted range");

    let mut result = src.to_string();
    result.replace_range(
        usize::from(range.start())..usize::from(range.end()),
        printed.as_code(),
    );
    result
}

fn format_whole(src: &str) -> String {
    let tree = parse(src, postgres());
    format_node(SqlFormatOptions::new(postgres()), &tree.syntax())
        .unwrap()
        .print()
        .unwrap()
        .into_code()
}

#[test]
fn range_formatting_removes_redundant_parentheses_like_whole_file() {
    let src = r#"drop function if exists ~$f~ cascade;
create function ~$f~() returns trigger as $function$
declare
   op int;
begin
   op = ~$getOp~(tg_op);
   if (op = 3) then
      update ~$Таблица~ t set "Поле" = (1) where (t.row_id = 1);
   end if;
   return null;
end;
$function$ language plpgsql;
"#;

    let formatted = format_at(src, "~$f~()");

    assert!(formatted.contains("\tif op = 3 then\n"), "{formatted}");
    assert!(
        formatted.contains("set \"Поле\" = 1 where t.row_id = 1;"),
        "{formatted}"
    );
    assert_eq!(formatted, format_whole(src));
}

#[test]
fn range_formatting_of_a_statement_far_from_the_file_start() {
    let src = format!(
        "{}select (a) + 2 from t where (x = 1) and (y = 2);\n",
        "select 1;\n".repeat(200)
    );

    let formatted = format_at(&src, "from t");

    assert!(
        formatted.ends_with("select a + 2 from t where x = 1 and y = 2;\n"),
        "{formatted}"
    );
}
