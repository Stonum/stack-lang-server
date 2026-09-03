//! Parser error-handling tests: malformed input, recovery, and the
//! delimiter-balance diagnostics (missing `{` / `}`). Valid-input parsing
//! lives in `parser.rs`.

use biome_rowan::{SyntaxKind, SyntaxNode, SyntaxSlot};
use mlang_parser::parse;
use mlang_syntax::MFileSource;

macro_rules! assert_parser {
    ($res:expr) => {
        assert!($res.try_tree().is_some());
        assert!(!$res.has_errors());
        assert!(!has_bogus_nodes_or_empty_slots(&$res.syntax()));
    };
}

fn has_bogus_nodes_or_empty_slots<L: biome_rowan::Language>(node: &SyntaxNode<L>) -> bool {
    node.descendants().any(|descendant| {
        let kind = descendant.kind();
        if kind.is_bogus() {
            return true;
        }

        if kind.is_list() {
            return descendant
                .slots()
                .any(|slot| matches!(slot, SyntaxSlot::Empty { .. }));
        }

        false
    })
}

#[test]
fn test_parse_report_with_bogus() {
    let res = parse(
        r#"
CommonReport
ШС             : ТЕКСТ
НазваниеОтчета : ТЕКСТ
.CloseWindow = 1;
{
    var month = WorkMonth();
}
print
Ф12"Courier New"УП^ШС^;^Артикул^
Ф16"Times New Roman"ЖЦ^ШС^;^НазваниеОтчета^ж
Ф12"Times New Roman"
{
    var i = 1;
    while( i < 10 )
       i = add(i);
}
        "#,
        MFileSource::report(),
    );
    assert!(res.has_errors());
}

#[test]
fn test_err_computed_class_member_name() {
    let res = parse(
        r#"
            class foo {
                "bar"() {}
            }
        "#,
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let res = parse(
        r#"
            class foo {
                set "bar"(x) {}
            }
        "#,
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let res = parse(
        r#"
            class foo {
                get 1() {}
            }
        "#,
        MFileSource::module(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_err_array_destructuring_assignment_rest_not_last() {
    let res = parse(
        r#"
            @[a, ...rest, other] = arr;
        "#,
        MFileSource::script(),
    );

    assert!(res.has_errors());

    let res = parse(
        r#"
            @[a, ...rest,] = arr;
        "#,
        MFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_err_object_destructuring_assignment_rest_not_last() {
    let res = parse(
        r#"
            @{...rest, other} = obj;
        "#,
        MFileSource::script(),
    );

    assert!(res.has_errors());

    let res = parse(
        r#"
            @{...rest,} = obj;
        "#,
        MFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_err_object_destructuring_assignment_nested_pattern_rest() {
    let res = parse(
        r#"
            @{...@{a, b}} = obj;
        "#,
        MFileSource::script(),
    );

    assert!(res.has_errors());
}

#[test]
fn test_err_missing_inner_brace_is_pinpointed() {
    // The inner `if` block loses its `}`; the parser only notices at EOF. The
    // indentation heuristic should point at the inner `{` and suppress the
    // coarse end-of-file message.
    let res = parse(
        "func main() {\n    if (x) {\n        work();\n\n    after();\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert!(
        messages.iter().any(|m| m == "This `{` is never closed"),
        "{messages:?}"
    );
    assert!(
        !messages.iter().any(|m| m == "Missing closing `}`"),
        "{messages:?}"
    );
}

#[test]
fn test_balanced_braces_are_not_flagged_by_the_heuristic() {
    let res = parse(
        "func main() {\n    if (x) {\n        work();\n    }\n}\n",
        MFileSource::module(),
    );

    assert_parser!(res);
}

#[test]
fn test_err_missing_brace_in_class_method_blames_the_method() {
    // The method's own `}` is missing; the diagnostic should land on the method
    // signature, not the class `{` or a lone brace.
    let res = parse(
        "class C {\n    doStuff(a, b)\n    {\n        work();\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());
    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert!(
        messages.iter().any(|m| m == "This `{` is never closed"),
        "{messages:?}"
    );
    assert!(
        !messages.iter().any(|m| m == "Missing closing `}`"),
        "{messages:?}"
    );
}

#[test]
fn test_err_class_method_missing_opening_brace_recovers_cleanly() {
    // The last method lost its body `{` (K&R style, so its `}` is now orphaned).
    // The parser must not choke trying to parse the orphaned statements as class
    // members, and the heuristic should blame the method signature.
    let res = parse(
        "class C {\n    m1() {\n        ok();\n    }\n    m2()\n    \n        work();\n        more();\n    }\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());
    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert!(
        messages
            .iter()
            .any(|m| m == "This block is missing its opening `{`"),
        "{messages:?}"
    );
    // No storm: recovery collapses the orphaned body into one bogus node.
    assert!(messages.len() <= 3, "{messages:?}");
}

#[test]
fn test_err_missing_brace_deep_in_a_large_body_does_not_flood() {
    // One `}` removed from a deeply nested block in a body that also contains
    // well-formed object literals and one-liners: exactly one re-anchored
    // diagnostic, no storm of false "never closed" on the healthy braces.
    let res = parse(
        "func main() {\n\
         \x20   x = @{ a: 1, b: 2 };\n\
         \x20   if (a) { one(); }\n\
         \x20   while (b) {\n\
         \x20       if (c) {\n\
         \x20           work();\n\
         \x20   }\n\
         \x20   y = @{ k: 1 };\n\
         }\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let unclosed = res
        .diagnostics()
        .iter()
        .filter(|d| d.message.to_string() == "This `{` is never closed")
        .count();

    assert_eq!(unclosed, 1, "{:?}", res.diagnostics());
}

#[test]
fn test_err_unclosed_own_brace_blames_the_flagged_block_not_an_inner_one() {
    // The block's own final `}` is missing and every inner block is fine: the
    // diagnostic lands on this block's header line, no invented inner culprit,
    // and the parser's terse "Missing closing `}`" is replaced.
    let res = parse(
        "func main() {\n    if (a) {\n        work();\n    }\n    tail();\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert_eq!(
        messages
            .iter()
            .filter(|m| *m == "This `{` is never closed")
            .count(),
        1,
        "{messages:?}"
    );
    assert!(
        !messages.iter().any(|m| m == "Missing closing `}`"),
        "{messages:?}"
    );
}

#[test]
fn test_err_missing_opening_brace_points_at_the_header() {
    // `while (a)` lost its `{`; the parser trips on a stray `}` further down.
    // The heuristic should blame the `while` header instead.
    let res = parse(
        "func main() {\n    while (a)\n        step1();\n        step2();\n    }\n    tail();\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert!(
        messages
            .iter()
            .any(|m| m == "This block is missing its opening `{`"),
        "{messages:?}"
    );
    assert!(
        !messages.iter().any(|m| m.contains("instead found '}'")),
        "{messages:?}"
    );
}

#[test]
fn test_err_missing_opening_brace_with_blank_lines_in_body() {
    // Blank lines inside the block body must not cut short the walk back to the
    // `while` header.
    let res = parse(
        "func f() {\n    while (a)\n        one();\n\n        two();\n\n        three();\n    }\n    tail();\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());

    let messages: Vec<String> = res
        .diagnostics()
        .iter()
        .map(|d| d.message.to_string())
        .collect();

    assert!(
        messages
            .iter()
            .any(|m| m == "This block is missing its opening `{`"),
        "{messages:?}"
    );
}

#[test]
fn test_err_extra_closing_brace_is_left_to_the_parser() {
    // A lone extra `}` with no indented body above it: don't invent a missing
    // `{` story, keep the parser's own diagnostic.
    let res = parse(
        "func main() {\n    work();\n    }\n}\n",
        MFileSource::module(),
    );

    assert!(res.has_errors());
    assert!(
        !res.diagnostics()
            .iter()
            .any(|d| d.message.to_string().contains("missing its opening")),
        "{:?}",
        res.diagnostics()
    );
}
