#![cfg(test)]
#![allow(unused_mut, unused_variables, unused_assignments)]

use super::{SqlLexContext, SqlLexer};
use sql_syntax::SqlSyntaxKind::{self, EOF};
use sql_syntax::T;
use sql_syntax::{SqlDialect, SqlFileSource};

use biome_parser::lexer::{BufferedLexer, Lexer};
use biome_rowan::TextSize;

// Макрос для проверки лексирования
macro_rules! assert_lex {
    ($src:expr, $($kind:ident:$len:expr $(,)?)*) => {{
        let mut lexer = SqlLexer::from_str($src, SqlFileSource::query());
        let mut idx = 0;
        let mut new_str = String::with_capacity($src.len());
        let mut tokens = vec![];

        while lexer.next_token(SqlLexContext::default()) != EOF {
            tokens.push((lexer.current(), lexer.current_range()));
        }

        $(
            assert_eq!(
                tokens[idx].0,
                sql_syntax::SqlSyntaxKind::$kind,
                "expected token kind {}, but found {:?}",
                stringify!($kind),
                tokens[idx].0,
            );

            assert_eq!(
                tokens[idx].1.len(),
                TextSize::from($len),
                "expected token length of {}, but found {:?} for token {:?}",
                $len,
                tokens[idx].1.len(),
                tokens[idx].0,
            );

            new_str.push_str(&$src[tokens[idx].1]);
            idx += 1;
        )*

        if idx < tokens.len() {
            panic!(
                "expected {} tokens but lexer returned {}, first unexpected token is '{:?}'",
                idx,
                tokens.len(),
                tokens[idx].0
            );
        } else {
            assert_eq!(idx, tokens.len());
        }

        assert_eq!($src, new_str, "Failed to reconstruct input");
    }};
}

#[test]
fn empty() {
    assert_lex! { "", }
}

#[test]
fn identifiers() {
    assert_lex! { "my_table", IDENT:8 }
    assert_lex! { "user_name", IDENT:9 }
    assert_lex! { "id", IDENT:2 }
    assert_lex! { "column1", IDENT:7 }
    assert_lex! { "CamelCase", IDENT:9 }
    assert_lex! { "with_underscore", IDENT:15 }
    assert_lex! { "_starts_with_underscore", IDENT:23 }
    assert_lex! { "\"quoted\"", IDENT:8 }
    assert_lex! { "\"quoted with space\"", IDENT:19 }
}

#[test]
fn hash_identifier_only_in_mlang_dialect() {
    let mut lexer = SqlLexer::from_str(
        "#tmptable",
        SqlFileSource::query()
            .with_dialect(SqlDialect::Postgres)
            .with_mlang_extension(true),
    );
    lexer.next_token(SqlLexContext);
    assert_eq!(lexer.current(), SqlSyntaxKind::IDENT);
    assert_eq!(lexer.current_range().len(), TextSize::from(9));

    let mut lexer = SqlLexer::from_str("#tmptable", SqlFileSource::query());
    lexer.next_token(SqlLexContext);
    assert_eq!(lexer.current(), SqlSyntaxKind::ERROR_TOKEN);
}

#[test]
fn keywords() {
    let keywords = vec![
        "select",
        "from",
        "where",
        "and",
        "or",
        "not",
        "insert",
        "update",
        "delete",
        "create",
        "table",
        "view",
        "index",
        "drop",
        "alter",
        "join",
        "on",
        "as",
        "distinct",
        // `group by`/`order by`/`partition by` can't go through this
        // generic `from_keyword`-based loop -- `from_keyword` doesn't
        // recognize either spelling for these three at all (neither the
        // fused `group_by` identifier form, used only for the
        // SyntaxKind/`T!` symbolic name, nor a string containing a literal
        // space). The real two-word spelling is recognized by the lexer's
        // own `try_fuse_two_word_by_keyword` instead, covered by
        // `two_word_by_keywords_fuse_into_one_token` below.
        "having",
        "union",
        "case",
        "when",
        "then",
        "end",
        "if",
        "else",
        "null",
        "true",
        "false",
        "between",
        "in",
        "like",
        "ilike",
        "is",
        "asc",
        "desc",
        "primary",
        "foreign",
        "key",
        "unique",
        "constraint",
        "check",
        "default",
        "integer",
        "bigint",
        "varchar",
        "text",
        "boolean",
        "date",
        "timestamp",
        "interval",
        "numeric",
        "json",
        "jsonb",
        "uuid",
        "array",
        "bytea",
    ];

    for keyword in keywords {
        let kind = SqlSyntaxKind::from_keyword(keyword).unwrap_or_else(|| {
            panic!("Expected `SqlSyntaxKind::from_keyword` to return a kind for keyword {keyword}.")
        });

        let mut lexer = SqlLexer::from_str(keyword, SqlFileSource::query());
        lexer.next_token(SqlLexContext);

        let lexed_kind = lexer.current();
        assert_eq!(
            lexed_kind, kind,
            "Expected token '{keyword}' to be of kind {kind:?} but is {lexed_kind:?}."
        );

        let lexed_range = lexer.current_range();
        assert_eq!(
            lexed_range.len(),
            TextSize::from(keyword.len() as u32),
            "Expected lexed keyword to be of len {} but has length {:?}",
            keyword.len(),
            lexed_range.len()
        );

        assert_eq!(lexer.next_token(SqlLexContext), EOF);
    }
}

#[test]
fn two_word_by_keywords_fuse_into_one_token() {
    assert_lex! { "order by", ORDER_BY_KW:8 }
    assert_lex! { "group by", GROUP_BY_KW:8 }
    assert_lex! { "partition by", PARTITION_BY_KW:12 }
    assert_lex! { "ORDER   BY", ORDER_BY_KW:10 }
    assert_lex! { "order\nby", ORDER_BY_KW:8 }
}

#[test]
fn bare_order_group_partition_without_by_are_plain_identifiers() {
    assert_lex! { "order", IDENT:5 }
    assert_lex! { "group", IDENT:5 }
    assert_lex! { "partition", IDENT:9 }
    assert_lex! { "order from", IDENT:5, WHITESPACE:1, FROM_KW:4 }
}

#[test]
fn fused_underscore_spelling_is_rejected_as_a_plain_identifier() {
    // `order_by`/`group_by`/`partition_by` (one word, underscore-joined)
    // isn't real Postgres syntax -- only the genuine two-word spelling
    // (tested above) is a keyword. A query actually containing this
    // spelling must fail to parse normally, not be silently accepted.
    assert_lex! { "order_by", IDENT:8 }
    assert_lex! { "group_by", IDENT:8 }
    assert_lex! { "partition_by", IDENT:12 }
}

#[test]
fn punctuators() {
    assert_lex! {
        "(),.:;+-*/<!>~=&|^%",
        L_PAREN:1,
        R_PAREN:1,
        COMMA:1,
        DOT:1,
        COLON:1,
        SEMICOLON:1,
        PLUS:1,
        MINUS:1,
        STAR:1,
        SLASH:1,
        L_ANGLE:1,
        BANG:1,
        R_ANGLE:1,
        TILDE:1,
        EQ:1,
        AMP:1,
        PIPE:1,
        CARET:1,
        PERCENT:1,
    }
}

#[test]
fn comparison_operators() {
    assert_lex! { ">", R_ANGLE:1 }
    assert_lex! { "<", L_ANGLE:1 }
    assert_lex! { "=", EQ:1 }
    assert_lex! { "!=", NEQ:2 }
    assert_lex! { "<>", LTGT:2 }
    assert_lex! { ">=", GTEQ:2 }
    assert_lex! { "<=", LTEQ:2 }
    assert_lex! { "~", TILDE:1 }
    assert_lex! { "~*", RGX:2 }
    assert_lex! { "!~", NEG_TILDE:2 }
    assert_lex! { "!~*", NEG_RGX:3 }
    assert_lex! { "|", PIPE:1 }
    assert_lex! { "||", PIPEPIPE:2 }
    assert_lex! { "| |", PIPE:1, WHITESPACE:1, PIPE:1 }
    assert_lex! { "->", ARROW:2 }
    assert_lex! { "->>", ARROW_ARROW:3 }
    assert_lex! { "-", MINUS:1 }
    assert_lex! { "-1", MINUS:1, SQL_NUMBER_LITERAL:1 }
}

#[test]
fn strings() {
    assert_lex! { r#"'simple'"#, SQL_STRING_LITERAL:8 }
    assert_lex! { r#"''"#, SQL_STRING_LITERAL:2 }
    assert_lex! { r#"'with spaces and words'"#, SQL_STRING_LITERAL:23 }
    assert_lex! { r#"'with '' quotes'"#, SQL_STRING_LITERAL:16 } // doubled quotes
    assert_lex! { r#"'with \n escape'"#, SQL_STRING_LITERAL:16 }
}

#[test]
fn dollar_quoted_strings() {
    assert_lex! { "$$$$", SQL_STRING_LITERAL:4 }
    assert_lex! { "$$dollar quoted$$", SQL_STRING_LITERAL:17 }
    assert_lex! { "$tag$dollar quoted$tag$", SQL_STRING_LITERAL:23 }
    // body may contain quotes/semicolons/newlines untouched -- exactly why
    // this is the real-world delimiter for PL/pgSQL function bodies
    assert_lex! { "$func$begin return 'it''s'; end;$func$", SQL_STRING_LITERAL:38 }
    assert_lex! { "$$line one\nline two$$", SQL_STRING_LITERAL:21 }
    // a `$` inside the body that doesn't match the closing delimiter is
    // just body content, not a premature close
    assert_lex! { "$$a$b$$", SQL_STRING_LITERAL:7 }
    assert_lex! { "$tag$a$$b$tag$", SQL_STRING_LITERAL:14 }
}

#[test]
fn unterminated_dollar_quoted_string() {
    assert_lex! { "$$unterminated", ERROR_TOKEN:14 }
    assert_lex! { "$tag$unterminated", ERROR_TOKEN:17 }
    assert_lex! { "$tag$almost$tagg$", ERROR_TOKEN:17 }
}

#[test]
fn dollar_not_followed_by_a_valid_tag_backs_off_to_a_single_error_token() {
    // `$1` (a Postgres positional parameter) isn't a dollar-quote opener --
    // not supported yet, but must not swallow the `1` as part of a failed
    // dollar-quote attempt.
    assert_lex! { "$1", ERROR_TOKEN:1, SQL_NUMBER_LITERAL:1 }
    assert_lex! { "$ 1", ERROR_TOKEN:1, WHITESPACE:1, SQL_NUMBER_LITERAL:1 }
}

#[test]
fn unterminated_string() {
    assert_lex! { r#"'unterminated"#, ERROR_TOKEN:13 }
    assert_lex! { r#"'abc\"#, ERROR_TOKEN:5 }
    assert_lex! { r#"'abc''"#, ERROR_TOKEN:6 }
}

#[test]
fn numbers() {
    assert_lex! { "123", SQL_NUMBER_LITERAL:3 }
    assert_lex! { "0", SQL_NUMBER_LITERAL:1 }
    assert_lex! { "3.14", SQL_NUMBER_LITERAL:4 }
    assert_lex! { "2.5e10", SQL_NUMBER_LITERAL:6 }
    assert_lex! { ".5", DOT:1, SQL_NUMBER_LITERAL:1 }
    assert_lex! { "1.", SQL_NUMBER_LITERAL:2 }
    assert_lex! { "1e+5", SQL_NUMBER_LITERAL:4 }
    assert_lex! { "1e-5", SQL_NUMBER_LITERAL:4 }
    assert_lex! { "1E5", SQL_NUMBER_LITERAL:3 }
}

#[test]
fn numbers_do_not_swallow_adjacent_operators() {
    // A number must not greedily consume a following `+`/`-`/`.` that isn't
    // part of a well-formed exponent -- those are separate operator tokens
    // handled by the expression parser, not part of the numeric literal.
    assert_lex! { "1-2", SQL_NUMBER_LITERAL:1, MINUS:1, SQL_NUMBER_LITERAL:1 }
    assert_lex! { "1+2", SQL_NUMBER_LITERAL:1, PLUS:1, SQL_NUMBER_LITERAL:1 }
    assert_lex! { "1.2.3", SQL_NUMBER_LITERAL:3, DOT:1, SQL_NUMBER_LITERAL:1 }
    assert_lex! { "2.5e10-1", SQL_NUMBER_LITERAL:6, MINUS:1, SQL_NUMBER_LITERAL:1 }
}

#[test]
fn numbers_with_incomplete_exponent_backtrack() {
    // An `e`/`E` not followed by a valid exponent (optional sign + at
    // least one digit) is not part of the number -- it re-lexes as the
    // start of an identifier instead of being swallowed into a malformed
    // number token.
    assert_lex! { "1e", SQL_NUMBER_LITERAL:1, IDENT:1 }
    assert_lex! { "1e+", SQL_NUMBER_LITERAL:1, IDENT:1, PLUS:1 }
    assert_lex! { "1e+a", SQL_NUMBER_LITERAL:1, IDENT:1, PLUS:1, IDENT:1 }
}

#[test]
fn whitespace_and_newlines() {
    assert_lex! { " ", WHITESPACE:1 }
    assert_lex! { "\t", WHITESPACE:1 }
    assert_lex! { "\n", NEWLINE:1 }
    assert_lex! { "\r\n", NEWLINE:2 }
    assert_lex! { " \t\n\r\n  ", WHITESPACE:2, NEWLINE:1, NEWLINE:2, WHITESPACE:2 }
}

#[test]
fn comments() {
    assert_lex! { "-- single line comment", COMMENT:22 }
    assert_lex! { "--", COMMENT:2 }
    assert_lex! { "-- a\nSELECT", COMMENT:4, NEWLINE:1, SELECT_KW:6 }
    assert_lex! { "x -- comment", IDENT:1, WHITESPACE:1, COMMENT:10 }
    assert_lex! { "/* comment */", COMMENT:13 }
    assert_lex! { "/* comment */SELECT", COMMENT:13, SELECT_KW:6 }
    assert_lex! { "/* comment /n another */", COMMENT:24 }
}

#[test]
fn simple_select() {
    assert_lex! {
        "SELECT * FROM users WHERE age > 18;",
        SELECT_KW:6,
        WHITESPACE:1,
        STAR:1,
        WHITESPACE:1,
        FROM_KW:4,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:3,
        WHITESPACE:1,
        R_ANGLE:1,
        WHITESPACE:1,
        SQL_NUMBER_LITERAL:2,
        SEMICOLON:1,
    }
}

#[test]
fn insert_statement() {
    assert_lex! {
        "INSERT INTO users (name, age) VALUES ('Alice', 30);",
        INSERT_KW:6,
        WHITESPACE:1,
        INTO_KW:4,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        L_PAREN:1,
        IDENT:4,
        COMMA:1,
        WHITESPACE:1,
        IDENT:3,
        R_PAREN:1,
        WHITESPACE:1,
        VALUES_KW:6,
        WHITESPACE:1,
        L_PAREN:1,
        SQL_STRING_LITERAL:7,
        COMMA:1,
        WHITESPACE:1,
        SQL_NUMBER_LITERAL:2,
        R_PAREN:1,
        SEMICOLON:1,
    }
}

#[test]
fn update_statement() {
    assert_lex! {
        "UPDATE users SET name = 'Bob' WHERE id = 1;",
        UPDATE_KW:6,
        WHITESPACE:1,
        IDENT:5,
        WHITESPACE:1,
        SET_KW:3,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        SQL_STRING_LITERAL:5,
        WHITESPACE:1,
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:2,
        WHITESPACE:1,
        EQ:1,
        WHITESPACE:1,
        SQL_NUMBER_LITERAL:1,
        SEMICOLON:1,
    }
}

#[test]
fn complex_where() {
    assert_lex! {
        "WHERE name LIKE 'A%' AND active IS TRUE;",
        WHERE_KW:5,
        WHITESPACE:1,
        IDENT:4,
        WHITESPACE:1,
        LIKE_KW:4,
        WHITESPACE:1,
        SQL_STRING_LITERAL:4,
        WHITESPACE:1,
        AND_KW:3,
        WHITESPACE:1,
        IDENT:6,
        WHITESPACE:1,
        IS_KW:2,
        WHITESPACE:1,
        TRUE_KW:4,
        SEMICOLON:1,
    }
}

#[test]
fn case_expression() {
    assert_lex! {
        "CASE WHEN x > 0 THEN 'positive' ELSE 'zero' END",
        CASE_KW:4,
        WHITESPACE:1,
        WHEN_KW:4,
        WHITESPACE:1,
        IDENT:1,
        WHITESPACE:1,
        R_ANGLE:1,
        WHITESPACE:1,
        SQL_NUMBER_LITERAL:1,
        WHITESPACE:1,
        THEN_KW:4,
        WHITESPACE:1,
        SQL_STRING_LITERAL:10,
        WHITESPACE:1,
        ELSE_KW:4,
        WHITESPACE:1,
        SQL_STRING_LITERAL:6,
        WHITESPACE:1,
        END_KW:3,
    }
}

#[test]
fn lookahead_buffer() {
    use sql_syntax::SqlSyntaxKind::{FROM_KW, IDENT, STAR, WHITESPACE};

    let lexer = SqlLexer::from_str("SELECT * FROM t", SqlFileSource::query());
    let mut buffered = BufferedLexer::new(lexer);

    buffered.next_token(SqlLexContext);
    assert_eq!(buffered.current(), T![select]);
    assert!(!buffered.has_preceding_line_break());

    {
        let lookahead: Vec<_> = buffered.lookahead_iter().map(|l| l.kind()).collect();
        assert_eq!(
            lookahead,
            vec![
                WHITESPACE, STAR, WHITESPACE, FROM_KW, WHITESPACE, IDENT, EOF
            ]
        );
    }

    buffered.next_token(SqlLexContext); // WHITESPACE
    buffered.next_token(SqlLexContext); // STAR
    buffered.next_token(SqlLexContext); // WHITESPACE
    buffered.next_token(SqlLexContext); // FROM_KW
    assert!(!buffered.has_preceding_line_break());

    buffered.next_token(SqlLexContext); // WHITESPACE
    buffered.next_token(SqlLexContext); // IDENT
    buffered.next_token(SqlLexContext); // EOF
    assert_eq!(buffered.current(), EOF);
}
