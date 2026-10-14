#[macro_use]
mod helper;

#[test]
fn format_xmlexists() {
    assert_fmt_eq!(
        r#"--
SELECT xmlexists( '/Root'  PASSING  BY REF doc ), xmlexists(('/Root/' || what) PASSING _xml BY VALUE) FROM t;
"#,
        r#"--
select xmlexists('/Root' passing by ref doc), xmlexists(('/Root/' || what) passing _xml by value) from t;
"#
    );
}

#[test]
fn format_xmltable() {
    assert_fmt_eq!(
        r#"--
SELECT x.* FROM XMLTABLE('/a/b' PASSING doc COLUMNS id int, name text) x;
"#,
        r#"--
select x.* from XMLTABLE('/a/b' passing doc columns id int, name text) x;
"#
    );
}

#[test]
fn format_long_xmltable_wraps() {
    assert_fmt_eq!(
        r#"--
select xmltable.* from XMLTABLE(XMLNAMESPACES ('http://example.com/schemas/bulkload/format' AS n), '/n:FORMAT/n:RECORD/n:FIELD' PASSING (xmldata) COLUMNS ID int PATH '@ID', TERMINATOR text PATH '@TERMINATOR', COLUMN_NAME text PATH '@NAME', MAX_LENGTH int PATH '@MAX_LENGTH', position for ordinality);
"#,
        r#"--
select xmltable.*
from XMLTABLE(
	XMLNAMESPACES('http://example.com/schemas/bulkload/format' as n),
	'/n:FORMAT/n:RECORD/n:FIELD'
	passing xmldata
	columns
		ID int path '@ID',
		TERMINATOR text path '@TERMINATOR',
		COLUMN_NAME text path '@NAME',
		MAX_LENGTH int path '@MAX_LENGTH',
		position for ordinality
);
"#
    );
}
