#[macro_use]
mod helper;

#[test]
fn format_real_production_query_shape() {
    assert_fmt!(
        r#"--
with agg as (
	select hist."СвязьДокумент", "Период", max("Расчетный период") as "Период"
	from ~Объекты~ ol
	join ~История~ hist on ol."Код" = 1
		and hist."СвязьДокумент" = ol."Документ"
		and hist."Период" = '2024-01-01'
		and coalesce(hist."ВидРасчета", 0) = 0
	join ~Документ~ doc on doc."row_id" = hist."СвязьДокумент" and doc."ВидДок" = 1
	where ol."Услуга" = 2
	group_by hist."Период", hist."СвязьДокумент"
)
select "итог", "дата"
from (
	select hist."Значение" * hist."Коэффициент" as "итог", hist."Дата"
	from agg
	join ~История~ hist on hist."СвязьДокумент" = agg."СвязьДокумент"
		and hist."Период" = '2024-01-01'
		and hist."Расчетный период" = agg."Расчетный период"
		and hist."Код-Деление" = 1
		and hist."ТипВвода" != 4
		and coalesce(hist."ВидРасчета", 0) = 0
	join ~Иерархия~ tui on tui."Родитель" = 2 and tui."Потомок" = hist."История-Услуга"
		union all
	select ps."Значение" as "итог", ps."дата"
	from ~Показания~ ps
	where ps."Код" = 1
		and ps."Тип" = 1
		and ps."Расчетный период" = '2024-01-01'
		and ps."Услуга" = 2
		and ps."ТипВвода" not in (4, 7)
) as it
"#
    );
}
