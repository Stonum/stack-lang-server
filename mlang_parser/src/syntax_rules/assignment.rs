use super::MParserCheckpoint;
use super::rewrite_parser::{RewriteMarker, RewriteParser, RewriteToken};

use super::expr::{
    ExpressionContext, is_at_identifier, parse_reference_identifier, parse_unary_expr,
};
use super::m_parse_error;
use super::m_parse_error::invalid_assignment_error;
use super::object::parse_object_member_name;

use super::MParser;
use super::rewrite::{RewriteParseEvents, rewrite_events};
use super::{ParseRecoveryTokenSet, RecoveryResult};
use mlang_syntax::{MSyntaxKind::*, *};

use super::ParsedSyntax::{Absent, Present};
use biome_parser::parse_lists::ParseSeparatedList;
use biome_parser::prelude::*;

// test assignment_target
// a.foo -= bar;
// (foo = bar);
// (((foo))) = bar;
// a["test"] = bar;
// a.call().chain().member = x;
// ++count == 3
// a['b'] = c[d] = "test"

// test_err invalid_assignment_target
// ++a = b;
// (++a) = b;
// (a = b;
// (a +) = b;

// test array_assignment_target
// @[a, b] = arr;
// @[a] = arr;
// @[a, _, b] = arr;
// @[a, ...rest] = arr;
// @[a,,c] = arr;
// @[a,] = arr;
// @[a.b, c[0]] = arr;

// test_err array_assignment_target_err
// @[a, ...rest, other] = arr;
// @[...] = arr;
// @[a, ...rest,] = arr;

// test object_assignment_target
// @{} = obj;
// @{a, b} = obj;
// @{a: x, b: y} = obj;
// @{a, ...rest} = obj;
// @{y: thisY, A, ...s} = obj;

// test_err object_assignment_target_err
// @{...} = obj;
// @{...rest, other} = obj;
// @{...rest,} = obj;

/// Converts the passed in lhs expression to an assignment pattern.
/// Array and object expressions are rewound and re-parsed as destructuring
/// assignment patterns since their element grammar differs from plain
/// expressions (rest elements, holes, shorthand/renamed properties).
/// The passed checkpoint allows to restore the parser to the state before it started parsing the expression.
pub fn expression_to_assignment_pattern(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> ParsedSyntax {
    match target.kind(p) {
        M_ARRAY_EXPRESSION => {
            p.rewind(checkpoint);
            parse_array_assignment_pattern(p)
        }
        M_OBJECT_EXPRESSION => {
            p.rewind(checkpoint);
            parse_object_assignment_pattern(p)
        }
        _ => Present(expression_to_assignment(p, target, checkpoint)),
    }
}

/// Parses an assignment target, allowing array/object destructuring patterns.
pub fn parse_assignment_pattern(p: &mut MParser) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = parse_unary_expr(p, ExpressionContext::default());

    assignment_expression.and_then(|expr| expression_to_assignment_pattern(p, expr, checkpoint))
}

/// Re-parses an expression as an assignment.
pub fn expression_to_assignment(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> CompletedMarker {
    try_expression_to_assignment(p, target, checkpoint).unwrap_or_else(
        // test_err M_regex_assignment
        // /=0*_:m/=/*_:|
        |mut invalid_assignment_target| {
            // Doesn't seem to be a valid assignment target. Recover and create an error.
            invalid_assignment_target.change_kind(p, M_BOGUS_ASSIGNMENT);

            p.error(invalid_assignment_error(
                p,
                invalid_assignment_target.range(p),
            ));

            invalid_assignment_target
        },
    )
}

pub fn parse_assignment(p: &mut MParser, context: ExpressionContext) -> ParsedSyntax {
    let checkpoint = p.checkpoint();
    let assignment_expression = parse_unary_expr(p, context);

    assignment_expression.map(|expr| expression_to_assignment(p, expr, checkpoint))
}

fn try_expression_to_assignment(
    p: &mut MParser,
    target: CompletedMarker,
    checkpoint: MParserCheckpoint,
) -> Result<CompletedMarker, CompletedMarker> {
    if !matches!(
        target.kind(p),
        M_PARENTHESIZED_EXPRESSION
            | M_STATIC_MEMBER_EXPRESSION
            | M_COMPUTED_MEMBER_EXPRESSION
            | M_IDENTIFIER_EXPRESSION
    ) {
        return Err(target);
    }

    // At this point it's guaranteed that the root node can be mapped to an assignment,
    // but it's not yet guaranteed if it is valid or not (for example, a static member expression
    // is valid, except if it uses optional chaining).
    let mut reparse_assignment = ReparseAssignment::new();
    rewrite_events(&mut reparse_assignment, checkpoint, p);

    Ok(reparse_assignment.result.unwrap())
}

struct ReparseAssignment {
    // Stores the unfinished parents
    // Index 0: Re-mapped kind of the node
    // Index 1: Started marker. A `None` marker means that this node should be dropped
    //          from the re-written tree
    parents: Vec<(MSyntaxKind, Option<RewriteMarker>)>,
    // Stores the completed assignment node (valid or invalid).
    result: Option<CompletedMarker>,
    // Tracks if the visitor is still inside an assignment
    inside_assignment: bool,
}

impl ReparseAssignment {
    pub fn new() -> Self {
        Self {
            parents: Vec::default(),
            result: None,
            inside_assignment: true,
        }
    }
}

/// Rewrites expressions to assignments
/// * Converts parenthesized expression to parenthesized assignment
/// * Converts computed/static member expressions to computed/static member assignment.
/// * Converts identifier expressions to identifier assignment, drops the inner reference identifier
impl RewriteParseEvents for ReparseAssignment {
    fn start_node(&mut self, kind: MSyntaxKind, p: &mut RewriteParser) {
        if !self.inside_assignment {
            self.parents.push((kind, Some(p.start())));
            return;
        }

        // Make sure to also add the kind to the match in `try_expression_to_assignment`
        let mapped_kind = match kind {
            M_PARENTHESIZED_EXPRESSION => M_PARENTHESIZED_ASSIGNMENT,
            M_STATIC_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                M_STATIC_MEMBER_ASSIGNMENT
            }
            M_COMPUTED_MEMBER_EXPRESSION => {
                self.inside_assignment = false;
                M_COMPUTED_MEMBER_ASSIGNMENT
            }
            M_IDENTIFIER_EXPRESSION => M_IDENTIFIER_ASSIGNMENT,
            M_REFERENCE_IDENTIFIER => {
                self.parents.push((kind, None)); // Omit reference identifiers
                return;
            }
            _ => {
                self.inside_assignment = false;
                M_BOGUS_ASSIGNMENT
            }
        };

        self.parents.push((mapped_kind, Some(p.start())));
    }

    fn finish_node(&mut self, p: &mut RewriteParser) {
        let (kind, m) = self.parents.pop().unwrap();

        if let Some(m) = m {
            let completed = m.complete(p, kind);

            if kind == M_BOGUS_ASSIGNMENT {
                let range = completed.range(p);
                p.error(
                    p.err_builder(
                        format!("Invalid assignment to `{}`", completed.text(p)),
                        range,
                    )
                    .with_hint("This expression cannot be assigned to"),
                );
            }

            self.result = Some(completed.into());
        }
    }

    fn token(&mut self, token: RewriteToken, p: &mut RewriteParser) {
        p.bump(token)
    }
}

///////////////
// ARRAY ASSIGNMENT PATTERN
///////////////

struct ArrayAssignmentPatternElementList;

impl ParseSeparatedList for ArrayAssignmentPatternElementList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_ARRAY_ASSIGNMENT_PATTERN_ELEMENT_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        match p.cur() {
            T![,] => Present(p.start().complete(p, M_ARRAY_HOLE)),
            T![...] => parse_array_assignment_pattern_rest_element(p),
            _ => parse_assignment_pattern(p),
        }
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T![']'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_ASSIGNMENT, token_set![T![,], T![']']])
                .enable_recovery_on_line_break(),
            m_parse_error::expected_array_assignment_target_element,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An array destructuring assignment target such as `@[a, b, ...rest]`.
fn parse_array_assignment_pattern(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![@]) || !p.nth_at(1, T!['[']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);
    p.bump(T!['[']);

    ArrayAssignmentPatternElementList.parse_list(p);

    p.expect(T![']']);
    Present(m.complete(p, M_ARRAY_ASSIGNMENT_PATTERN))
}

/// A rest element inside an array assignment pattern such as `...rest`.
fn parse_array_assignment_pattern_rest_element(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![...]);
    parse_assignment_pattern(p).or_add_diagnostic(p, m_parse_error::expected_assignment_target);

    let rest = m.complete(p, M_ARRAY_ASSIGNMENT_PATTERN_REST_ELEMENT);
    Present(validate_rest_pattern(p, rest, T![']'], "]"))
}

///////////////
// OBJECT ASSIGNMENT PATTERN
///////////////

struct ObjectAssignmentPatternPropertyList;

impl ParseSeparatedList for ObjectAssignmentPatternPropertyList {
    type Kind = MSyntaxKind;
    type Parser<'source> = MParser<'source>;

    const LIST_KIND: Self::Kind = M_OBJECT_ASSIGNMENT_PATTERN_PROPERTY_LIST;

    fn parse_element(&mut self, p: &mut MParser) -> ParsedSyntax {
        if p.at(T![...]) {
            parse_object_assignment_pattern_rest(p)
        } else {
            parse_object_assignment_pattern_property(p)
        }
    }

    fn is_at_list_end(&self, p: &mut MParser) -> bool {
        p.at(T!['}'])
    }

    fn recover(&mut self, p: &mut MParser, parsed_element: ParsedSyntax) -> RecoveryResult {
        parsed_element.or_recover_with_token_set(
            p,
            &ParseRecoveryTokenSet::new(M_BOGUS_ASSIGNMENT, token_set![T![,], T!['}']])
                .enable_recovery_on_line_break(),
            m_parse_error::expected_object_assignment_target_property,
        )
    }

    fn separating_element_kind(&mut self) -> MSyntaxKind {
        T![,]
    }

    fn allow_trailing_separating_element(&self) -> bool {
        true
    }
}

/// An object destructuring assignment target such as `@{a, b: y, ...rest}`.
fn parse_object_assignment_pattern(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![@]) || !p.nth_at(1, T!['{']) {
        return Absent;
    }
    let m = p.start();
    p.bump(T![@]);
    p.bump(T!['{']);

    ObjectAssignmentPatternPropertyList.parse_list(p);

    p.expect(T!['}']);
    Present(m.complete(p, M_OBJECT_ASSIGNMENT_PATTERN))
}

/// A shorthand `@{a}` or renamed `@{a: b}` property inside an object assignment pattern.
fn parse_object_assignment_pattern_property(p: &mut MParser) -> ParsedSyntax {
    let m = p.start();

    if is_at_identifier(p) && !p.nth_at(1, T![:]) {
        parse_reference_identifier(p).or_add_diagnostic(p, m_parse_error::expected_identifier);
        return Present(m.complete(p, M_OBJECT_ASSIGNMENT_PATTERN_SHORTHAND_PROPERTY));
    }

    parse_object_member_name(p).or_add_diagnostic(p, m_parse_error::expected_object_member);
    p.expect(T![:]);
    parse_assignment_pattern(p).or_add_diagnostic(p, m_parse_error::expected_assignment_target);
    Present(m.complete(p, M_OBJECT_ASSIGNMENT_PATTERN_PROPERTY))
}

/// A rest property inside an object assignment pattern such as `...rest`.
fn parse_object_assignment_pattern_rest(p: &mut MParser) -> ParsedSyntax {
    if !p.at(T![...]) {
        return Absent;
    }

    let m = p.start();
    p.bump(T![...]);

    let target =
        parse_assignment_pattern(p).or_add_diagnostic(p, m_parse_error::expected_assignment_target);

    if let Some(mut target) = target
        && matches!(
            target.kind(p),
            M_ARRAY_ASSIGNMENT_PATTERN | M_OBJECT_ASSIGNMENT_PATTERN
        )
    {
        target.change_kind(p, M_BOGUS_ASSIGNMENT);
        p.error(p.err_builder(
            "object and array assignment targets are not allowed in rest patterns",
            target.range(p),
        ));
    }

    let rest = m.complete(p, M_OBJECT_ASSIGNMENT_PATTERN_REST);
    Present(validate_rest_pattern(p, rest, T!['}'], "}"))
}

/// Validates that a parsed rest element/property is the last element of its
/// pattern and isn't followed by a trailing comma, converting it to a bogus
/// assignment target otherwise.
fn validate_rest_pattern(
    p: &mut MParser,
    mut rest: CompletedMarker,
    end_token: MSyntaxKind,
    end_token_text: &str,
) -> CompletedMarker {
    if p.at(end_token) {
        return rest;
    }

    if p.at(T![,]) && p.nth_at(1, end_token) {
        p.error(
            p.err_builder("rest element may not have a trailing comma", p.cur_range())
                .with_detail(p.cur_range(), "Remove the trailing comma here")
                .with_detail(rest.range(p), "Rest element"),
        );
        rest.change_kind(p, M_BOGUS_ASSIGNMENT);
        rest
    } else {
        p.error(
            p.err_builder("rest element must be the last element", rest.range(p))
                .with_hint(format!(
                    "Move the rest element to the end of the pattern, right before the closing '{end_token_text}'",
                )),
        );
        rest.change_kind(p, M_BOGUS_ASSIGNMENT);
        rest
    }
}
