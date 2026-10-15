use crate::prelude::*;
use biome_formatter::trivia::format_replaced;
use biome_formatter::write;
use sql_syntax::SqlSyntaxKind;
use sql_syntax::SqlTypeName;
use sql_syntax::SqlTypeNameFields;

/// Built-in Postgres types that lex as plain identifiers; printed in lower
/// case like the keyword type names. Other identifiers keep their case.
const BUILTIN_TYPE_IDENTS: &[&str] = &[
    "bigserial",
    "bool",
    "box",
    "character",
    "cidr",
    "circle",
    "float",
    "float4",
    "float8",
    "inet",
    "int",
    "int2",
    "int4",
    "int8",
    "line",
    "macaddr",
    "money",
    "oid",
    "point",
    "polygon",
    "record",
    "refcursor",
    "regclass",
    "serial",
    "serial2",
    "serial4",
    "serial8",
    "smallint",
    "smallserial",
    "timestamptz",
    "timetz",
    "tsquery",
    "tsvector",
    "varbit",
    "void",
    "xml",
];

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatSqlTypeName;
impl FormatNodeRule<SqlTypeName> for FormatSqlTypeName {
    fn fmt_fields(&self, node: &SqlTypeName, f: &mut SqlFormatter) -> FormatResult<()> {
        let SqlTypeNameFields {
            name,
            args,
            modifier,
            array_suffix,
        } = node.as_fields();

        let name = name?;
        let lowercase = name.text_trimmed().to_ascii_lowercase();
        if name.kind() == SqlSyntaxKind::IDENT
            && name.text_trimmed() != lowercase
            && BUILTIN_TYPE_IDENTS.contains(&lowercase.as_str())
        {
            let start = name.text_trimmed_range().start();
            write!(
                f,
                [format_replaced(&name, &dynamic_text(&lowercase, start))]
            )?;
        } else {
            write!(f, [name.format()])?;
        }
        if let Some(args) = args {
            write!(f, [args.format()])?;
        }
        if let Some(modifier) = modifier {
            write!(f, [space(), modifier.format()])?;
        }
        if let Some(array_suffix) = array_suffix {
            write!(f, [array_suffix.format()])?;
        }
        Ok(())
    }
}
