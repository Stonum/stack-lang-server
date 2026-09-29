//! End-to-end resolution: cursor -> `SemanticInfo` -> `lsp_definition` queries.

use biome_rowan::TextSize;
use lsp_definition::{get_completion, get_declaration, get_hover, get_reference, get_signatures};
use mlang_parser::parse;
use mlang_semantic::{
    SemanticModel, identifier_for_completion, identifier_for_offset, identifier_for_signature_help,
    semantics,
};
use mlang_syntax::MFileSource;
use tower_lsp::lsp_types::{Location, MarkedString, Url};

const SRC: &str = r#"func f(a) {}
func f(a, b) {}
func g() {}
class Base {
    constructor(a) {}
    m(x) {}
    get p() { return 1; }
}
class Child extends Base {
    constructor() { super(1); this.m(1); }
    m(x, y) {}
    run() { return this.p; }
}
func use() {
    f(1);
    f(1, 2);
    g();
    var r = g();
    r;
    var x = new Child();
    x.run();
    new Base(1);
    new Base();
    z.m(1);
    g();
}
"#;

fn model() -> SemanticModel {
    let file_source = MFileSource::module();
    semantics(SRC, parse(SRC, file_source).syntax(), file_source)
}

fn uri() -> Url {
    Url::parse("file:///test.prg").unwrap()
}

/// Offset of `$` inside `pattern`; `pattern` without `$` must occur once in `SRC`.
fn cursor(pattern: &str) -> TextSize {
    let pos = pattern.find('$').expect("pattern needs a `$` cursor");
    let needle = pattern.replace('$', "");
    let start = SRC
        .find(&needle)
        .unwrap_or_else(|| panic!("`{needle}` not found"));
    assert_eq!(SRC.matches(&needle).count(), 1, "`{needle}` is ambiguous");
    TextSize::from((start + pos) as u32)
}

/// `line:text` of each location, to keep assertions readable.
fn texts(locations: &[Location]) -> Vec<String> {
    let lines: Vec<&str> = SRC.lines().collect();
    locations
        .iter()
        .map(|l| {
            let (start, end) = (l.range.start, l.range.end);
            assert_eq!(start.line, end.line);
            let line = lines[start.line as usize];
            let text = &line[start.character as usize..end.character as usize];
            format!("{}:{text}", start.line)
        })
        .collect()
}

fn goto(pattern: &str) -> Vec<String> {
    let model = model();
    let info = identifier_for_offset(parse(SRC, MFileSource::module()).syntax(), cursor(pattern))
        .unwrap_or_else(|| panic!("no info at `{pattern}`"));
    let locations = get_declaration(&info, model.definitions().map(|d| (uri(), d)));
    texts(&locations)
}

fn hover(pattern: &str) -> Vec<String> {
    let model = model();
    let info = identifier_for_offset(parse(SRC, MFileSource::module()).syntax(), cursor(pattern))
        .unwrap_or_else(|| panic!("no info at `{pattern}`"));
    get_hover(&info, model.definitions().map(|d| (uri(), d)))
        .into_iter()
        .map(|m| match m {
            MarkedString::String(s) => s,
            MarkedString::LanguageString(s) => s.value,
        })
        .collect()
}

fn references(pattern: &str) -> Vec<String> {
    let model = model();
    let info = identifier_for_offset(parse(SRC, MFileSource::module()).syntax(), cursor(pattern))
        .unwrap_or_else(|| panic!("no info at `{pattern}`"));
    // references are grouped by a hash map key, so their order is unspecified
    let mut locations = get_reference(&info, &uri(), model.references());
    locations.sort_by_key(|l| (l.range.start.line, l.range.start.character));
    texts(&locations)
}

fn signatures(pattern: &str) -> Vec<String> {
    let model = model();
    let (info, _) =
        identifier_for_signature_help(parse(SRC, MFileSource::module()).syntax(), cursor(pattern))
            .unwrap_or_else(|| panic!("no info at `{pattern}`"));
    get_signatures(&info, model.definitions().map(|d| (uri(), d)), 0)
        .into_iter()
        .map(|s| s.label)
        .collect()
}

fn completion(pattern: &str) -> Vec<String> {
    let model = model();
    let Some(info) =
        identifier_for_completion(parse(SRC, MFileSource::module()).syntax(), cursor(pattern))
    else {
        return vec![];
    };
    let mut labels: Vec<String> = get_completion(&info, model.definitions().map(|d| (uri(), d)))
        .into_iter()
        .map(|c| c.label)
        .collect();
    labels.sort();
    labels
}

#[test]
fn function_call_goes_to_overload_matching_arity() {
    assert_eq!(goto("    $f(1);"), ["0:f"]);
    assert_eq!(goto("    $f(1, 2);"), ["1:f"]);
}

#[test]
fn function_declaration_goes_to_all_overloads() {
    assert_eq!(goto("func $f(a) {}"), ["0:f", "1:f"]);
}

#[test]
fn function_call_hover_prefers_matching_overload() {
    let markups = hover("    $f(1, 2);");
    assert_eq!(markups.len(), 2);
    assert!(markups[0].contains("f(a, b)"), "{markups:?}");
}

#[test]
fn variable_with_call_result_goes_to_function() {
    assert_eq!(goto("    $r;"), ["2:g"]);
}

#[test]
fn new_expression_goes_to_matching_constructor_or_class() {
    assert_eq!(goto("new C$hild()"), ["9:constructor"]);
    assert_eq!(goto("new B$ase(1)"), ["4:constructor"]);
    assert_eq!(goto("new B$ase()"), ["3:Base"]);
}

#[test]
fn class_declaration_goes_to_constructor() {
    assert_eq!(goto("class $Child"), ["9:constructor"]);
}

#[test]
fn class_hover_shows_class_and_constructors() {
    let markups = hover("new B$ase(1)");
    assert_eq!(markups.len(), 2);
    assert!(markups[0].contains("class Base"), "{markups:?}");
}

#[test]
fn extends_goes_to_class() {
    assert_eq!(goto("extends $Base"), ["3:Base"]);
}

#[test]
fn super_call_goes_to_parent_constructor() {
    assert_eq!(goto("s$uper(1)"), ["4:constructor"]);
}

#[test]
fn this_method_call_resolves_through_hierarchy() {
    assert_eq!(goto("this.$m(1)"), ["5:m"]);
}

#[test]
fn this_property_goes_to_getter() {
    assert_eq!(goto("this.$p"), ["6:p"]);
}

#[test]
fn method_declaration_goes_to_class_members() {
    assert_eq!(goto("    $m(x, y) {}"), ["10:m", "5:m"]);
}

#[test]
fn method_call_on_typed_variable_goes_to_its_class() {
    assert_eq!(goto("x.$run()"), ["11:run"]);
    assert_eq!(goto("$x.run()"), ["8:Child"]);
}

#[test]
fn method_call_on_unknown_object_matches_any_class() {
    assert_eq!(goto("z.$m(1)"), ["5:m"]);
}

#[test]
fn references_of_function_declaration() {
    assert_eq!(references("func $g()"), ["16:g", "17:g", "24:g"]);
}

#[test]
fn references_of_class_declaration() {
    assert_eq!(references("class $Base"), ["21:Base", "22:Base"]);
}

#[test]
fn references_of_method_declaration() {
    // `this.m` inside `Child` is keyed by `Child`, not by the declaring `Base`.
    assert_eq!(references("    $m(x) {}"), ["23:m"]);
}

#[test]
fn references_from_call_site_are_empty() {
    assert!(references("    $f(1);").is_empty());
}

#[test]
fn signature_help_lists_overloads_best_first() {
    assert_eq!(signatures("f(1, $2)"), ["func f(a, b)", "func f(a)"]);
}

#[test]
fn signature_help_for_constructor() {
    // TODO: constructor labels are empty (`SignatureParameters::text` skips them).
    assert_eq!(signatures("new Base($1)").len(), 1);
}

#[test]
fn completion_after_this_lists_class_members() {
    assert_eq!(completion("return this$.p"), ["m", "p", "run"]);
}
