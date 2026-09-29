use biome_rowan::{
    AstNode, AstSeparatedList, NodeOrToken, SyntaxNode, SyntaxToken, TextRange, TextSize,
};
use lsp_definition::{Class, SemanticInfo, Symbol, Usage};

use mlang_syntax::{
    AnyMAssignment, AnyMBinding, AnyMExpression, MAssignmentExpression, MCallExpression,
    MCaseClause, MClassDeclaration, MExpressionStatement, MFileSource, MFunctionDeclaration,
    MLanguage, MNewExpression, MSequenceExpression, MStaticMemberAssignment,
    MStaticMemberExpression, MSwitchStatement, MSyntaxKind, MVariableStatement,
};

pub fn identifier_for_offset(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
    source_type: MFileSource,
) -> Option<SemanticInfo> {
    // checking the boundaries if cursor is at the start or end token
    let offsets = [
        offset,
        offset.checked_add(1.into()).unwrap_or_default(),
        offset.checked_sub(1.into()).unwrap_or_default(),
    ];

    for offset in offsets {
        let range = TextRange::new(offset, offset);
        if !root.text_range().contains_range(range) {
            continue;
        }
        let node = root.covering_element(range);
        let token = node.as_token();
        token?;
        let token = token.unwrap();
        let handler_info = source_type
            .is_handler()
            .then(|| handler_declaration(token))
            .flatten();
        if let Some(info) = handler_info.or_else(|| identifier_for_token(token)) {
            return Some(info);
        }
    }
    None
}

pub fn identifier_for_completion(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<SemanticInfo> {
    let range = TextRange::new(offset, offset);
    if !root.text_range().contains_range(range) {
        return None;
    }
    let node = root.covering_element(range);
    if let Some(token) = node.as_token() {
        return identifier_for_token(token);
    }
    None
}

pub fn identifier_for_signature_help(
    root: SyntaxNode<MLanguage>,
    offset: TextSize,
) -> Option<(SemanticInfo, u32)> {
    let range = TextRange::new(offset, offset);
    if !root.text_range().contains_range(range) {
        return None;
    }
    let node = root.covering_element(range);
    find_identifier_for_signature_body(node, offset)
}

fn identifier_for_token(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    rparen_handler(token)
        .or_else(|| declaration_handler(token))
        .or_else(|| iterator_handler(token))
        .or_else(|| new_expression_handler(token))
        .or_else(|| super_expression_handler(token))
        .or_else(|| this_expression_handler(token))
        .or_else(|| call_handler(token))
        .or_else(|| property_handler(token))
        .or_else(|| reference_handler(token))
}

// HANDLERS

/// In a `.hdl` file: the handler function name or a `Выбор "event":` label of its top-level switch.
fn handler_declaration(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    let function = token.ancestors().find_map(MFunctionDeclaration::cast)?;
    let id = function.id().ok()?;
    let handler = id.text();

    if id.range().contains_range(token.text_trimmed_range()) {
        return Some(SemanticInfo::new(
            Symbol::Handler(handler),
            Usage::Declaration,
        ));
    }

    let literal = token.parent()?;
    let case = MCaseClause::cast(literal.parent()?)?;
    let is_label = case.test().ok()?.syntax() == &literal;
    let in_body = case
        .syntax()
        .ancestors()
        .find_map(MSwitchStatement::cast)?
        .syntax()
        .grand_parent()
        .is_some_and(|body| body.kind() == MSyntaxKind::M_FUNCTION_BODY);
    if !is_label || !in_body {
        return None;
    }

    Some(SemanticInfo::new(
        Symbol::HandlerEvent {
            handler,
            event: token.text_trimmed().to_string(),
        },
        Usage::Declaration,
    ))
}

fn rparen_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::R_PAREN) {
        return None;
    }

    let node = token.ancestors().nth(1)?;
    let kind = node.kind();

    match kind {
        MSyntaxKind::M_NEW_EXPRESSION | MSyntaxKind::M_CALL_EXPRESSION => {
            let info_token = find_info_token(node)?;
            match kind {
                MSyntaxKind::M_NEW_EXPRESSION => new_expression_handler(&info_token),
                MSyntaxKind::M_CALL_EXPRESSION => call_handler(&info_token),
                _ => unreachable!(),
            }
        }
        _ => None,
    }
}

fn declaration_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT) {
        return None;
    }

    let ident = token.text_trimmed().trim().to_string();

    // take nearest parents
    for n in token.ancestors().take(3) {
        match n.kind() {
            MSyntaxKind::M_FUNCTION_DECLARATION => {
                return Some(SemanticInfo::new(
                    Symbol::Function(ident),
                    Usage::Declaration,
                ));
            }
            MSyntaxKind::M_CLASS_DECLARATION => {
                return Some(SemanticInfo::new(Symbol::Class(ident), Usage::Declaration));
            }
            MSyntaxKind::M_METHOD_CLASS_MEMBER => {
                let class_member_list_node = n.parent()?;
                let class_node = class_member_list_node.parent()?;

                let class = MClassDeclaration::cast(class_node)?;
                let class_id = class.id().ok()?.text();

                return Some(SemanticInfo::new(
                    Symbol::Member {
                        name: ident,
                        class: Some(class_id),
                    },
                    Usage::Declaration,
                ));
            }
            MSyntaxKind::M_EXTENDS_CLAUSE => {
                return Some(SemanticInfo::new(Symbol::Class(ident), Usage::Extends));
            }
            _ => continue,
        }
    }

    None
}

fn iterator_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT) {
        return None;
    }

    let node = token.ancestors().nth(2)?;
    if node.kind() == MSyntaxKind::M_FOR_ITERATOR_FACTORY {
        return Some(SemanticInfo::new(
            Symbol::Function(token.text_trimmed().trim().to_string()),
            Usage::Call(2),
        ));
    }

    None
}

fn new_expression_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT | MSyntaxKind::NEW_KW) {
        return None;
    }

    if token.kind() == MSyntaxKind::NEW_KW {
        // zero args for new expression without class name
        return Some(SemanticInfo::new(Symbol::AnyClass, Usage::New(0)));
    }

    let node = token.ancestors().nth(2)?;
    let new = MNewExpression::cast(node)?;
    let args_count = {
        let args = new.arguments()?.args();
        Some(args.len())
    }
    .unwrap_or_default();

    Some(SemanticInfo::new(
        Symbol::Class(token.text_trimmed().trim().to_string()),
        Usage::New(args_count),
    ))
}

fn super_expression_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::SUPER_KW) {
        return None;
    }

    let class_id = {
        let class = get_nearest_class_declaration(token)?;
        let id = class.extends_clause()?.super_class().ok()?.text();
        Some(id)
    }?; // return None if class is not founded

    let node = token.ancestors().nth(1)?;
    if MCallExpression::can_cast(node.kind()) {
        let call = MCallExpression::unwrap_cast(node);
        let args_count = {
            let args = call.arguments().ok()?.args();
            Some(args.len())
        }
        .unwrap_or_default();

        return Some(SemanticInfo::new(
            Symbol::Class(class_id),
            Usage::Super(args_count),
        ));
    }

    Some(SemanticInfo::new(Symbol::Class(class_id), Usage::Instance))
}

fn this_expression_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::THIS_KW) {
        return None;
    }

    let class_id = {
        let class = get_nearest_class_declaration(token)?;
        let id = class.id().ok()?.text();
        Some(id)
    }?; // return None if class is not founded

    Some(SemanticInfo::new(Symbol::Class(class_id), Usage::Instance))
}

fn call_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT) {
        return None;
    }

    let node = token.ancestors().nth(2)?;
    let call = MCallExpression::cast(node)?;
    let args_count = {
        let args = call.arguments().ok()?.args();
        Some(args.len())
    }
    .unwrap_or_default();

    let identifier = token.text_trimmed().trim().to_string();

    if let Ok(callee) = call.callee() {
        // get class id for his object
        if let AnyMExpression::MStaticMemberExpression(expr) = callee {
            let object = expr.object().ok()?;
            let class = instance_class(get_nearest_variable_declaration(&object));
            return Some(method_call(identifier, args_count, class));
        }

        // get super class id for super expressions
        if let AnyMExpression::MSuperExpression(expr) = callee {
            let super_token = expr.super_token().ok()?;
            let class_id = {
                let class = get_nearest_class_declaration(&super_token)?;
                let id = class.extends_clause()?.super_class().ok()?.text();
                Some(id)
            };
            return Some(method_call(identifier, args_count, class_id));
        };
    }

    Some(SemanticInfo::new(
        Symbol::Function(identifier),
        Usage::Call(args_count),
    ))
}

fn property_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT) {
        return None;
    }

    let node = token.ancestors().nth(1)?;
    let object = match node {
        _ if MStaticMemberAssignment::can_cast(node.kind()) => {
            MStaticMemberAssignment::unwrap_cast(node).object().ok()?
        }
        _ if MStaticMemberExpression::can_cast(node.kind()) => {
            MStaticMemberExpression::unwrap_cast(node).object().ok()?
        }
        _ => return None,
    };

    let class = instance_class(get_nearest_variable_declaration(&object))?;
    let identifier = token.text_trimmed().trim().to_string();
    Some(SemanticInfo::new(
        Symbol::Member {
            name: identifier,
            class: Some(class),
        },
        Usage::Access,
    ))
}

fn reference_handler(token: &SyntaxToken<MLanguage>) -> Option<SemanticInfo> {
    if !matches!(token.kind(), MSyntaxKind::IDENT) {
        return None;
    }

    let node = token.ancestors().nth(1)?;
    let expression = AnyMExpression::cast(node)?;
    get_nearest_variable_declaration(&expression)
}

// UTILITY FUNCTIONS

fn method_call(name: String, args_count: usize, class: Option<Class>) -> SemanticInfo {
    SemanticInfo::new(Symbol::Member { name, class }, Usage::Call(args_count))
}

fn instance_class(info: Option<SemanticInfo>) -> Option<Class> {
    match info? {
        SemanticInfo {
            symbol: Symbol::Class(class),
            usage: Usage::Instance,
        } => Some(class),
        _ => None,
    }
}

fn get_nearest_class_declaration(token: &SyntaxToken<MLanguage>) -> Option<MClassDeclaration> {
    token
        .ancestors()
        .find(|parent| parent.kind() == MSyntaxKind::M_CLASS_DECLARATION)
        .and_then(MClassDeclaration::cast)
}

fn get_nearest_variable_declaration(variable: &AnyMExpression) -> Option<SemanticInfo> {
    let name = match variable {
        AnyMExpression::MIdentifierExpression(ident) => ident.name().ok()?.text().to_lowercase(),
        AnyMExpression::MThisExpression(this) if this.this_token().is_ok() => {
            return this_expression_handler(&this.this_token().unwrap());
        }
        AnyMExpression::MSuperExpression(s) if s.super_token().is_ok() => {
            return super_expression_handler(&s.super_token().unwrap());
        }
        _ => return None,
    };

    variable
        .syntax()
        .ancestors()
        .find_map(|parent| get_first_assignment_or_declaration(&parent, &name))
        .and_then(find_identifier_from_right_side)
}

fn get_first_assignment_or_declaration(
    parent: &SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    parent
        .siblings(biome_rowan::Direction::Prev)
        .find_map(|n| match n.kind() {
            MSyntaxKind::M_EXPRESSION_STATEMENT => find_expression_statement(n, ident),
            MSyntaxKind::M_ASSIGNMENT_EXPRESSION => find_assignment_expression(n, ident),
            MSyntaxKind::M_VARIABLE_STATEMENT => find_variable_statement(n, ident),
            _ => None,
        })
}

fn find_expression_statement(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let expr = MExpressionStatement::cast(expression)?;
    match expr.expression().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    }
}

// find assignments from sequence expression
// example x = 1, y = 2;
fn find_sequence_expression(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let seq = MSequenceExpression::cast(expression)?;
    let left = match seq.left().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    };

    if left.is_some() {
        return left;
    }

    match seq.right().ok()? {
        AnyMExpression::MAssignmentExpression(expr) => {
            find_assignment_expression(expr.into_syntax(), ident)
        }
        AnyMExpression::MSequenceExpression(expr) => {
            find_sequence_expression(expr.into_syntax(), ident)
        }
        _ => None,
    }
}

// return the right assignment expression
// example x = 1;
// where x == ident
fn find_assignment_expression(
    expression: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let assignment = MAssignmentExpression::cast(expression)?;

    let left_ident = assignment.left().ok()?;
    let AnyMAssignment::MIdentifierAssignment(left_ident) = left_ident else {
        return None;
    };

    let right = assignment.right().ok()?;
    let left_name = left_ident.name_token().ok()?;

    if left_name.text_trimmed().to_string().to_lowercase() != ident {
        // find our ident in the right side of the assignment
        if matches!(right, AnyMExpression::MAssignmentExpression(_)) {
            return find_assignment_expression(right.into_syntax(), ident);
        }
        return None;
    }

    Some(right.into_syntax())
}

// return the right declaration expression
// example var x = 1;
// where x == ident
fn find_variable_statement(
    statement: SyntaxNode<MLanguage>,
    ident: &str,
) -> Option<SyntaxNode<MLanguage>> {
    let var_statement = MVariableStatement::cast(statement)?;
    let var_declaration = var_statement.declaration().ok()?;

    var_declaration
        .declarators()
        .into_iter()
        .find_map(|declarator| {
            let declarator = declarator.ok()?;
            let binding = declarator.id().ok()?;
            let AnyMBinding::MIdentifierBinding(binding) = binding else {
                return None;
            };

            let binding_name = binding.name_token().ok()?;
            let expression = declarator.initializer()?.expression().ok()?;

            if binding_name.text_trimmed().to_string().to_lowercase() != ident {
                // find our ident in the right side of the assignment
                if matches!(expression, AnyMExpression::MAssignmentExpression(_)) {
                    return find_assignment_expression(expression.into_syntax(), ident);
                }
                return None;
            }

            Some(expression.into_syntax())
        })
}

fn find_identifier_from_right_side(node: SyntaxNode<MLanguage>) -> Option<SemanticInfo> {
    if let Some(assignment) = MAssignmentExpression::cast(node.clone()) {
        let mut right = assignment.right().ok()?;
        while let AnyMExpression::MAssignmentExpression(assignment) = right {
            right = assignment.right().ok()?;
        }
        return find_identifier_from_right_side(right.into_syntax());
    }

    let info_token = find_info_token(node);

    let info_token = info_token?;
    let info = identifier_for_token(&info_token)?;

    let usage = match (&info.symbol, info.usage) {
        (_, Usage::Call(n)) => Usage::CallResult(n),
        (Symbol::Class(_), Usage::New(_)) => Usage::Instance,
        _ => return None,
    };
    Some(SemanticInfo::new(info.symbol, usage))
}

fn find_info_token(node: SyntaxNode<MLanguage>) -> Option<SyntaxToken<MLanguage>> {
    let mut info_token = None;
    if let Some(call_expression) = MCallExpression::cast(node.clone()) {
        let callee = call_expression.callee().ok()?;
        info_token = match callee {
            AnyMExpression::MIdentifierExpression(ident) => {
                let ident = ident.name().ok()?;
                Some(ident.value_token().ok()?)
            }
            AnyMExpression::MStaticMemberExpression(expr) => {
                let ident = expr.member().ok()?;
                Some(ident.value_token().ok()?)
            }
            AnyMExpression::MSuperExpression(expr) => {
                let ident = expr.super_token().ok()?;
                Some(ident)
            }
            _ => None,
        }
    }

    if let Some(new_expression) = MNewExpression::cast(node) {
        let class = new_expression.callee().ok()?;
        if let AnyMExpression::MIdentifierExpression(ident) = class {
            let ident = ident.name().ok()?;
            info_token = Some(ident.value_token().ok()?);
        }
    }
    info_token
}

fn find_identifier_for_signature_body(
    node: NodeOrToken<SyntaxNode<MLanguage>, SyntaxToken<MLanguage>>,
    offset: TextSize,
) -> Option<(SemanticInfo, u32)> {
    for n in node.ancestors().take(5) {
        match n.kind() {
            MSyntaxKind::M_NEW_EXPRESSION | MSyntaxKind::M_CALL_EXPRESSION => {
                let mut current_arg_number = 0_u32;
                let args = match n.kind() {
                    MSyntaxKind::M_CALL_EXPRESSION => {
                        MCallExpression::unwrap_cast(n.clone()).arguments().ok()
                    }
                    MSyntaxKind::M_NEW_EXPRESSION => {
                        MNewExpression::unwrap_cast(n.clone()).arguments()
                    }
                    _ => None,
                };
                if let Some(args) = args {
                    let offset_sub = offset.checked_sub(1.into()).unwrap_or_default();
                    // check for token otside argumenrs
                    if let Ok(ft) = args.l_paren_token()
                        && ft.text_range().start().gt(&offset_sub)
                    {
                        return None;
                    }

                    current_arg_number = args
                        .args()
                        .elements()
                        .filter(|e| {
                            e.clone().into_trailing_separator().is_ok_and(|n| {
                                n.is_some_and(|n| {
                                    offset.checked_add(1.into()).is_some_and(|sub| {
                                        n.text_range()
                                            .start()
                                            .lt(&sub.checked_sub(1.into()).unwrap_or_default())
                                    })
                                })
                            })
                        })
                        .count() as u32;
                }
                let info_token = find_info_token(n)?;
                if let Some(ident) = identifier_for_token(&info_token) {
                    return Some((ident, current_arg_number));
                }
            }
            _ => {}
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use mlang_parser::parse;
    use mlang_syntax::MFileSource;

    use super::*;

    fn func_decl(name: &str) -> SemanticInfo {
        SemanticInfo::new(Symbol::Function(name.into()), Usage::Declaration)
    }
    fn class_decl(name: &str) -> SemanticInfo {
        SemanticInfo::new(Symbol::Class(name.into()), Usage::Declaration)
    }
    fn extends(name: &str) -> SemanticInfo {
        SemanticInfo::new(Symbol::Class(name.into()), Usage::Extends)
    }
    fn instance(class: &str) -> SemanticInfo {
        SemanticInfo::new(Symbol::Class(class.into()), Usage::Instance)
    }
    fn new(class: &str, args: usize) -> SemanticInfo {
        SemanticInfo::new(Symbol::Class(class.into()), Usage::New(args))
    }
    fn new_any() -> SemanticInfo {
        SemanticInfo::new(Symbol::AnyClass, Usage::New(0))
    }
    fn super_call(args: usize, class: &str) -> SemanticInfo {
        SemanticInfo::new(Symbol::Class(class.into()), Usage::Super(args))
    }
    fn call(name: &str, args: usize) -> SemanticInfo {
        SemanticInfo::new(Symbol::Function(name.into()), Usage::Call(args))
    }
    fn call_result(name: &str, args: usize) -> SemanticInfo {
        SemanticInfo::new(Symbol::Function(name.into()), Usage::CallResult(args))
    }
    fn member(name: &str, class: Option<&str>) -> Symbol {
        Symbol::Member {
            name: name.into(),
            class: class.map(Into::into),
        }
    }
    fn method_decl(name: &str, class: &str) -> SemanticInfo {
        SemanticInfo::new(member(name, Some(class)), Usage::Declaration)
    }
    fn method(name: &str, args: usize, class: Option<&str>) -> SemanticInfo {
        SemanticInfo::new(member(name, class), Usage::Call(args))
    }
    fn method_result(name: &str, args: usize, class: Option<&str>) -> SemanticInfo {
        SemanticInfo::new(member(name, class), Usage::CallResult(args))
    }
    fn prop(name: &str, class: &str) -> SemanticInfo {
        SemanticInfo::new(member(name, Some(class)), Usage::Access)
    }

    #[test]
    fn test_identifier_for_offset() {
        #[rustfmt::skip]
        let inputs = [
            ("func x() {}", 6, func_decl("x")),
            ("class A {}", 7, class_decl("A")),
            ("class B extends A {}", 17, extends("A")),
            ("class A { x() {} }", 11, method_decl("x", "A")),

            ("forall( iterator(arr, ind)) {}", 15, call("iterator", 2)),

            ("new ", 2, new_any()),
            ("var x = new TodoClass()",15, new("TodoClass", 0)),
            ("var x = new TodoClass(1, 2, 3)",15, new("TodoClass", 3)),

            ("var x = callFunction()", 15, call("callFunction", 0)),
            ("var x = callFunction(1, 2, 3, 4)", 15, call("callFunction", 4)),

            ("class B extends A { constructor() { super() } }", 40, super_call(0, "A")),

            ("var x = z.callMethod(1, 2)", 15, method("callMethod", 2, None)),

            ("var z = new TodoClass(); z.callMethod();",30, method("callMethod", 0, Some("TodoClass"))),

            ("var x = callFunction( z.callMethod() )", 30, method("callMethod", 0, None)),
            ("var x = z.callMethod( callFunction() )", 30, call("callFunction", 0)),
            ("var x = z.callMethod( new TodoClass() )",30, new("TodoClass", 0)),

            ("#comment line
              callaFterComment()",30, call("callaFterComment", 0)),

            ("var xyz = xyz()", 12, call("xyz", 0))
        ];

        for (input, offset, info) in inputs {
            let parsed = parse(input, MFileSource::script());
            let semantic_info = identifier_for_offset(
                parsed.syntax(),
                TextSize::from(offset),
                MFileSource::script(),
            )
            .unwrap_or_else(|| panic!("failed for `{input}`"));
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_for_offset_in_class_declaration() {
        let input = r#"
            class Test {
                constructor() { this.m2(); }
                m1() {}
                m2() { this.m1(); }
                get xxx() { return 1; }
                m3()
                {
                    this.yyy = 1;
                    return this.xxx;
                }
            }
        "#;
        let parsed = parse(input, MFileSource::script());

        #[rustfmt::skip]
        let offsets = [
            (65, method("m2", 0, Some("Test"))),
            (125, method("m1", 0, Some("Test"))),
            (62, instance("Test")),
            (240, prop("yyy", "Test")),
            (280, prop("xxx", "Test")),
        ];

        for (offset, info) in offsets {
            let semantic_info = identifier_for_offset(
                parsed.syntax(),
                TextSize::from(offset),
                MFileSource::script(),
            )
            .unwrap_or_else(|| panic!("failed for offset: {offset}"));
            assert_eq!(info, semantic_info, "offset: {offset}");
        }
    }

    #[test]
    fn test_identifier_for_offset_in_handler() {
        let input = r#"Функция 'Модуль.Записи_АПИ'( Событие )
{
   ВыборПо( Событие )
   {
      Выбор "ДействиеА":
         ВыборПо( Режим ) { Выбор "Вложенный": Вернуть 1; }
         Вернуть Вычислить();
   }
}"#;
        let at = |needle: &str, source: MFileSource| {
            let offset = input.find(needle).unwrap() + 1;
            let parsed = parse(input, source);
            identifier_for_offset(parsed.syntax(), TextSize::from(offset as u32), source)
        };
        let handler = "'Модуль.Записи_АПИ'".to_string();

        assert_eq!(
            at("Модуль", MFileSource::handler()),
            Some(SemanticInfo::new(
                Symbol::Handler(handler.clone()),
                Usage::Declaration
            ))
        );
        assert_eq!(
            at("ДействиеА", MFileSource::handler()),
            Some(SemanticInfo::new(
                Symbol::HandlerEvent {
                    handler,
                    event: "\"ДействиеА\"".into()
                },
                Usage::Declaration
            ))
        );
        assert_eq!(at("Вложенный", MFileSource::handler()), None);
        assert_eq!(
            at("Вычислить", MFileSource::handler()),
            Some(call("Вычислить", 0))
        );
        assert_ne!(
            at("Модуль", MFileSource::module()).map(|info| info.symbol),
            Some(Symbol::Handler("'Модуль.Записи_АПИ'".into()))
        );
    }

    #[test]
    fn test_identifier_by_reference() {
        #[rustfmt::skip]
        let inputs = [
            ("var x = callFunction(); X ", 25, call_result("callFunction", 0)),
            ("var x = z.callMethod(); x ", 25, method_result("callMethod", 0, None)),
            ("var x = z.callMethod(1,2,3); x ", 30, method_result("callMethod", 3, None)),
            ("var x = callFunction(); y = x + 3 ", 29, call_result("callFunction", 0)),
            ("var x = callFunction(1,2); y = x + 3 ", 32, call_result("callFunction", 2)),
            ("var x = new Tst(); x.callMethod() ", 20, instance("Tst")),
            ("var x = new Tst(); if (true) x.callMethod() ", 30, instance("Tst")),
            ("var a = 3, x = new Tst(); x ", 27, instance("Tst")),
            ("var x = 3; x = new Tst(); x ", 27, instance("Tst")),
            ("var x = new Tst(); x.a = 3; x ", 29, instance("Tst")),
            ("a = 3, x = new Tst(); x ", 23, instance("Tst")),
            ("x = new Tst(), a = 3; x ", 23, instance("Tst")),
            ("x = callFunction(); x ", 21, call_result("callFunction", 0)),
            ("var y = z = x = new Tst(); x ", 28, instance("Tst"))
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let semantic_info =
                identifier_for_token(&token).unwrap_or_else(|| panic!("failed for `{input}`"));
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_from_r_paren() {
        #[rustfmt::skip]
        let inputs = [
            ("new Tst() ", 9, new("Tst", 0)),
            ("functionName() ", 14, call("functionName", 0)),
            ("cl.m1() ", 7, method("m1", 0, None)),
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let semantic_info =
                identifier_for_token(&token).unwrap_or_else(|| panic!("failed for `{input}`"));
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    #[test]
    fn test_identifier_from_signature_help() {
        #[rustfmt::skip]
        let inputs = [
            ("funcName(a, b) ", 11, Some((call("funcName", 2), 1))),
            ("new Test(a, b) ", 11, Some((new("Test", 2), 1))),
            ("x.m1(a, b) ", 8, Some((method("m1", 2, None), 1))),
            ("class Tst extends Par{ constructor(a, b) { super(a, b); } }", 51, Some((super_call(2, "Par"), 1))),
            ("funcName(a, b) ", 1, None),
        ];

        for (input, offset, info) in inputs {
            let token = get_token_from_offset(input, offset);
            let node = NodeOrToken::Token(token);
            let semantic_info = find_identifier_for_signature_body(node, TextSize::from(offset));
            assert_eq!(info, semantic_info, "{input}");
        }
    }

    fn get_token_from_offset(input: &str, offset: u32) -> SyntaxToken<MLanguage> {
        let parsed = parse(input, MFileSource::script());
        let syntax = parsed.syntax();
        let text_size_offset = TextSize::from(offset);
        let range = TextRange::new(text_size_offset, text_size_offset);
        let element = syntax.covering_element(range);
        let token = element.as_token().unwrap();
        token.clone()
    }
}
