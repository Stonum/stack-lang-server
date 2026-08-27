#[macro_use]
mod helper;

#[test]
fn format_function_variable() {
    assert_fmt!(
        r#"#
перем проверитьВложение = Функция(_вложение) {
   Вернуть _вложение.служебный == "Нет"
      и Извлечь(_вложение.удален, "Нет") != "Да"
      и Извлечь(_вложение.удаленКонтрагентом, "Нет") != "Да";
};
"#
    );
}

#[test]
fn format_long_ternary_operator() {
    assert_fmt!(
        r#"#
перем коэффициентДней = _мДанныеИПУ.днейРаботает
   ? _мДанныеИнтервала.днейИнтервала / _мДанныеИПУ.днейРаботает
   : 99999999;
"#
    );
}

#[test]
fn format_init_object() {
    assert_fmt!(
        r#"#
перем мПараметры = @{
   параметрыПолива,
   параметрыКупания,
   круглогодичныеПараметры
};
"#
    );
}

#[test]
fn format_if_with_long_condition() {
    assert_fmt!(
        r#"#
Если(!_мДанныеИнтервала.сезонность
   и !_мДанныеИнтервала.расходОКПУ_ХН
   и !_мДанныеИнтервала.начислениеПоСчетчикуХН
   и !_мДанныеИнтервала.начислениеПоСреднемуХН
   и !_мДанныеИнтервала.параметрыЛС["ОТКЛ" + ресурсУслугиСчетчика]
)
{
}
"#
    );
}

#[test]
fn format_if_with_short_condition() {
    assert_fmt!(
        r#"#
Если(!_мДанныеИнтервала.сезонность и !_мДанныеИнтервала.расход)
{
}
"#
    );
}

#[test]
fn format_binary_expression() {
    assert_fmt!(
        r#"#
мРезультат[тариф] = Окр(
   (суммарныйОбьем / итогоДней)
      * постояннаяДней1
      * постояннаяДней2
      * постояннаяДней3
      * постояннаяДней4
      * (учитыватьДельты ? среднесуточнаяДельта : 1),
   _точностьОкругления
);
"#
    );
}

#[test]
fn format_function_declaration() {
    assert_fmt!(
        r#"#
Функция Тестовая(_первый, _второй)
{
   var xxx = @[1, 2, 3, 4, 5, 6, 7, 8, 9];
}
"#
    );
}

#[test]
fn format_function_declaration_with_many_parameters() {
    assert_fmt!(
        r#"#
Функция ВнестиЗаписьВТаблицуКонтроляПоказаний(
   НПП, Дата, КонтрольДокумент, ПоказанияСчет, ПоказанияУслуга, Показания, Показание,
   РасчетныйМесяц, Тарифность, Тариф, ТипПоказания, ТипВвода, ДополнительныйРасход,
   Расход, Платеж, Групповой, ...
)
{
   var xxx = @[1, 2, 3, 4, 5, 6, 7, 8, 9];
}
"#
    );
}

#[test]
fn format_object_literal() {
    assert_fmt!(
        r#"#
var xxx = @{x: 1, y: 2};
"#
    );
}

#[test]
fn format_object_literal_compact_fill() {
    // At 8+ members, simple members are packed together instead of one per line.
    assert_fmt!(
        r#"#
var m = @{
   a, b, c, d, e, f, g, h
};
"#
    );
}

#[test]
fn format_object_literal_compact_fill_breaks_complex_member() {
    // A computed member name isn't "simple", so it forces its own line, while
    // runs of simple neighbors on either side still pack together.
    assert_fmt!(
        r#"#
var m = @{
   a, b,
   [x]: 1,
   d, e, f, g, h
};
"#
    );
}

#[test]
fn format_object_literal_compact_fill_reformats_one_member_per_line_input() {
    // Sanity check the round-trip tests above aren't trivially passing: feed
    // mis-formatted (one member per line) input and confirm it actually gets
    // packed, rather than just accepting already-correct input unchanged.
    assert_fmt_eq!(
        r#"#
var m = @{
   a,
   b,
   c,
   d,
   e,
   f,
   g,
   h
};
"#,
        "#\nvar m = @{\n   a, b, c, d, e, f, g, h\n};"
    );
}

#[test]
fn format_hash_map_literal_compact_fill() {
    assert_fmt!(
        r#"#
var m = @(
   a: 1, b: 2, c: 3, d: 4, e: 5, f: 6, g: 7, h: 8
);
"#
    );
}

#[test]
fn format_array_literal_compact_fill() {
    // Arrays (unlike objects) don't expand just because the source has a
    // leading newline, so a short list collapses onto one line either way;
    // `format_array_literal_compact_fill_wraps_long_content` below is what
    // actually exercises the packed layout.
    assert_fmt!(
        r#"#
var m = @[a, b, c, d, e, f, g, h];
"#
    );
}

#[test]
fn format_array_literal_compact_fill_wraps_long_content() {
    assert_fmt!(
        r#"#
var m = @[
   значениеПервоеДлинное, значениеВтороеДлинное, значениеТретьеДлинное,
   значениеЧетвертоеДлинное, значениеПятоеДлинное, значениеШестоеДлинное,
   значениеСедьмоеДлинное, значениеВосьмоеДлинное
];
"#
    );
}

#[test]
fn format_array_literal_with_hole_keeps_one_per_line() {
    // Array holes need a forced trailing comma that the compact fill layout
    // doesn't special-case, so arrays containing a hole must keep falling
    // back to one-per-line even past the compact-fill element threshold.
    assert_fmt!(
        r#"#
var m = @[
   значениеПервоеДлинное,
   ,
   значениеТретьеДлинное,
   значениеЧетвертоеДлинное,
   значениеПятоеДлинное,
   значениеШестоеДлинное,
   значениеСедьмоеДлинное,
   значениеВосьмоеДлинное
];
"#
    );
}
