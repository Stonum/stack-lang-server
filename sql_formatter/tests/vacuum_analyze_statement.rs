#[macro_use]
mod helper;

#[test]
fn format_vacuum_alone() {
    assert_fmt!(
        r#"--
vacuum;
"#
    );
}

#[test]
fn format_analyze_alone() {
    assert_fmt!(
        r#"--
analyze
"#
    );
}

#[test]
fn format_vacuum_flags_and_column_lists() {
    assert_fmt!(
        r#"--
vacuum full freeze verbose analyze a (x, y), s.b;
"#
    );
}

#[test]
fn format_vacuum_analyze_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
VACUUM   ANALYZE   ~Таблица~;
"#,
        r#"--
vacuum analyze ~Таблица~;
"#
    );
}

#[test]
fn format_analyze_normalizes_spacing_and_case() {
    assert_fmt_eq!(
        r#"--
ANALYZE  VERBOSE   #tmp,b(x,y);
"#,
        r#"--
analyze verbose #tmp, b (x, y);
"#
    );
}

#[test]
fn format_vacuum_tables_wrap_when_too_long() {
    assert_fmt_eq!(
        r#"--
vacuum full verbose analyze first_long_table_name (column_one, column_two), second_long_table_name (column_three, column_four), third_table
"#,
        r#"--
vacuum full verbose analyze
	first_long_table_name (column_one, column_two), second_long_table_name (column_three, column_four), third_table
"#
    );
}

#[test]
fn format_analyze_many_tables_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
analyze accounts, payments, invoices, orders, customers, products, suppliers, warehouses, shipments, returns, refunds, discounts, taxes, currencies, regions, countries, cities, streets, buildings, rooms
"#,
        r#"--
analyze
	accounts, payments, invoices, orders, customers, products, suppliers, warehouses, shipments, returns,
	refunds, discounts, taxes, currencies, regions, countries, cities, streets, buildings, rooms
"#
    );
}

#[test]
fn format_vacuum_many_columns_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
vacuum analyze accounts (id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags, extra, more, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth)
"#,
        r#"--
vacuum analyze accounts (
	id, name, code, kind, state, owner, created, updated, deleted, comment, parent, sort, flags,
	extra, more, first, second, third, fourth, fifth, sixth, seventh, eighth, ninth, tenth
)
"#
    );
}

#[test]
fn format_vacuum_many_tables_with_columns_fill_several_lines() {
    assert_fmt_eq!(
        r#"--
vacuum analyze accounts (id, name), payments (id, amount), invoices (id, total), orders (id, state), customers (id, name), products (id, price), suppliers (id, name)
"#,
        r#"--
vacuum analyze
	accounts (id, name), payments (id, amount), invoices (id, total), orders (id, state),
	customers (id, name), products (id, price), suppliers (id, name)
"#
    );
}
