#[macro_use]
mod helper;

#[test]
fn pretty_format_query_call_1() {
    assert_fmt!(
        r#"#
перем результат = запрос.установитьПараметры(
   индекс, дата1, дата2,
   извлечь(массив[индекс1, индекс2, индекс3, "дата"], 00.00.00)
);
"#
    );
}

#[test]
fn pretty_format_query_call_2() {
    assert_fmt!(
        r#"#
Если(true)
{
   з_кор.УстановитьПараметры(
      НачалоМесяца(месяцРасчета), ПоследнееЧисло(месяцРасчета), НачалоМесяца(месяцРасчета),
      ПоследнееЧисло(месяцРасчета), месяцРасчета, Число(пДог)
   );
}
"#
    );
}

#[test]
fn pretty_format_query_call_with_object() {
    assert_fmt!(
        r#"#
з_кор.УстановитьПараметры(@{
   abonent: _идАбонента,
   contact: контакт,
   type: _оКонтакт.тип == 1 ? 0 : 1,
   flags: _оКонтакт.флаги,
   act: _актуален ? 1 : _оКонтакт.актуальность,
   id_base: _идбазы,
   ...зЗаявитель.params
}, true);
"#
    );
}

#[test]
fn pretty_format_query_call_with_object_and_comments() {
    assert_fmt!(
        r#"#
з_кор.УстановитьПараметры(@{
   abonent: _идАбонента,
   contact: контакт,
   # В КЦ 0 - моб. телефон, 1 - email
   type: _оКонтакт.тип == 1 ? 0 : 1,
   flags: _оКонтакт.флаги,
   act: _актуален ? 1 : _оКонтакт.актуальность,
   id_base: _идбазы,
   ...зЗаявитель.params
}, true);
"#
    );
}

#[test]
fn pretty_format_object_property_short_call() {
    // Short property value call: must stay on one line, not expand to (
    //    value
    // )
    assert_fmt!(
        r#"#
перем запрос = @{
   данные: список.количество(),
   элемент: список.получить(0)
};
"#
    );
}

#[test]
fn pretty_format_object_declaration() {
    assert_fmt!(
        r#"#
перем запрос = @{
   метод: "GET",
   ресурс: "/v6/GetMessage",
   параметры: "boxId={}&messageId={}&entityId={}&injectEntityContent={}".format(
      _мДанДок.boxId,
      _мДанДок.messageId,
      _мДанДок.entityId,
      true
   ),
   тело: "boxId={}&messageId={}&entityId={}&injectEntityContent={}"
      .replace("{}", "null")
      .split("=")
      .iterator()
      .join("===========")
};
"#
    );
}

#[test]
fn pretty_format_method_chain() {
    assert_fmt!(
        r#"#
перем документыДляПечати = new ДокументыДляПечати(_стр_дог, _датнач, _даткнц, true)
   .дляАналитики(_аналитика)
   .дляВидасчета(_флаги)
   .дляРеестра(_типРеестра)
   .дляТиповДокументов(_типыдокументов)
   .дляТранспорта(_рассылка ? "рассылка" : "печать");
"#
    );

    assert_fmt!(
        r#"#
"1 2 3 4".split(" ").iterator().join(",");
"#
    );
}

#[test]
fn pretty_format_long_method_call() {
    assert_fmt!(
        r#"#
перем документыДляПечати = new ДокументыДляПечати(_стр_дог, _датнач, _даткнц, true).дляТранспорта(
   _рассылка ? "рассылка" : "печать"
);
"#
    );
}

#[test]
fn format_object_key_unquoting() {
    // latin key: quotes removed
    assert_fmt_eq!(r#"var a = @{"x": 1};"#, r#"var a = @{x: 1};"#);
    // cyrillic key: quotes removed
    assert_fmt_eq!(r#"var a = @{"ключ": 1};"#, r#"var a = @{ключ: 1};"#);
    // key with underscore and digits: quotes removed
    assert_fmt_eq!(r#"var a = @{"my_key_1": 1};"#, r#"var a = @{my_key_1: 1};"#);
    // reserved keyword: quotes preserved
    assert_fmt_eq!(r#"var a = @{"if": 1};"#, r#"var a = @{"if": 1};"#);
    // key with spaces: quotes preserved
    assert_fmt_eq!(r#"var a = @{"my key": 1};"#, r#"var a = @{"my key": 1};"#);
    // already unquoted latin key: unchanged
    assert_fmt_eq!(r#"var a = @{x: 1};"#, r#"var a = @{x: 1};"#);
    // already unquoted cyrillic key: unchanged
    assert_fmt_eq!(r#"var a = @{ключ: 1};"#, r#"var a = @{ключ: 1};"#);
}
