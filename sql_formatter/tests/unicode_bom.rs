#[macro_use]
mod helper;

#[test]
fn format_leading_bom_is_preserved() {
    // Preserved verbatim, not stripped -- some Windows-side tooling that
    // produced the original "UTF-8 with BOM" file may still rely on it
    // being there.
    assert_fmt!("\u{FEFF}select a from t\n");
}

#[test]
fn format_leading_bom_does_not_block_normalization() {
    assert_fmt_eq!(
        "\u{FEFF}CREATE   TABLE   foo (a int)\n",
        "\u{FEFF}create table foo (a int)\n"
    );
}
