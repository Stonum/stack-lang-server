use super::stmt::parse_statements;
use sql_syntax::SqlSyntaxKind::*;

use biome_parser::prelude::*;

use crate::SqlParser;

pub(crate) fn parse(p: &mut SqlParser) -> CompletedMarker {
    let m = p.start();
    p.eat(UNICODE_BOM);

    let statement_list = p.start();

    parse_statements(p, statement_list);
    m.complete(p, SQL_ROOT)
}
