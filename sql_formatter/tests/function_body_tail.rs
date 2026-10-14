#[macro_use]
mod helper;

#[test]
fn format_similar_to_and_xor() {
    assert_fmt_eq!(
        r#"--
SELECT a SIMILAR  TO 'x%', b NOT SIMILAR TO 'y', c  #  d, substring(s FOR 3), kr.full FROM t;
"#,
        r#"--
select a similar to 'x%', b not similar to 'y', c # d, substring(s for 3), kr.full from t;
"#
    );
}

#[test]
fn format_tilde_name_value() {
    assert_fmt_eq!(
        r#"--
DO $$ BEGIN tmp_table = ~$#tmpParamLS~; END $$;
"#,
        r#"--
do $$
begin
	tmp_table = ~$#tmpParamLS~;
end
$$;
"#
    );
}

#[test]
fn format_nested_join() {
    assert_fmt_eq!(
        r#"--
select * from a sd join b vz join c ls on ls.row_id = vz.c_id on sd.b_id = vz.row_id left join d on d.id = sd.d_id;
"#,
        r#"--
select *
from a sd
join b vz
	join c ls on ls.row_id = vz.c_id
	on sd.b_id = vz.row_id
left join d on d.id = sd.d_id;
"#
    );
}

#[test]
fn format_absolute_value_operator() {
    assert_fmt_eq!(
        r#"--
SELECT @ k_i, @-5, a@>b FROM t WHERE c = @ d;
"#,
        r#"--
select @k_i, @-5, a @> b from t where c = @d;
"#
    );
}
