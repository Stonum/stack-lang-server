//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    MLanguage as Language, MSyntaxElement as SyntaxElement,
    MSyntaxElementChildren as SyntaxElementChildren,
    MSyntaxKind::{self as SyntaxKind, *},
    MSyntaxList as SyntaxList, MSyntaxNode as SyntaxNode, MSyntaxToken as SyntaxToken,
    macros::map_syntax_node,
};
use biome_rowan::{
    AstNode, AstNodeList, AstNodeListIterator, AstNodeSlotMap, AstSeparatedList,
    AstSeparatedListNodesIterator, RawSyntaxKind, SyntaxKindSet, SyntaxResult, support,
};
use serde::ser::SerializeSeq;
use serde::{Serialize, Serializer};
use std::fmt::{Debug, Formatter};
#[doc = r" Sentinel value indicating a missing element in a dynamic node, where"]
#[doc = r" the slots are not statically known."]
pub(crate) const SLOT_MAP_EMPTY_VALUE: u8 = u8::MAX;
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MAnnotationAttribute {
    pub(crate) syntax: SyntaxNode,
}
impl MAnnotationAttribute {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MAnnotationAttributeFields {
        MAnnotationAttributeFields {
            name: self.name(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyMLiteralExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MAnnotationAttribute {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MAnnotationAttributeFields {
    pub name: SyntaxResult<AnyMBinding>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyMLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MAnnotationBinding {
    pub(crate) syntax: SyntaxNode,
}
impl MAnnotationBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MAnnotationBindingFields {
        MAnnotationBindingFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MAnnotationBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MAnnotationBindingFields {
    pub name: SyntaxResult<AnyMBinding>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MAnnotationElement {
    pub(crate) syntax: SyntaxNode,
}
impl MAnnotationElement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MAnnotationElementFields {
        MAnnotationElementFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            attributes: self.attributes(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn attributes(&self) -> MAnnotationAttributeList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MAnnotationElement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MAnnotationElementFields {
    pub name: SyntaxResult<AnyMBinding>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub attributes: MAnnotationAttributeList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MAnnotationGroup {
    pub(crate) syntax: SyntaxNode,
}
impl MAnnotationGroup {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MAnnotationGroupFields {
        MAnnotationGroupFields {
            colon_token: self.colon_token(),
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn elements(&self) -> MAnnotationList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MAnnotationGroup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MAnnotationGroupFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: MAnnotationList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MArrayExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MArrayExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MArrayExpressionFields {
        MArrayExpressionFields {
            at_token: self.at_token(),
            l_brack_token: self.l_brack_token(),
            elements: self.elements(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn elements(&self) -> MArrayElementList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MArrayExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MArrayExpressionFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub elements: MArrayElementList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MArrayHole {
    pub(crate) syntax: SyntaxNode,
}
impl MArrayHole {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MArrayHoleFields {
        MArrayHoleFields {}
    }
}
impl Serialize for MArrayHole {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MArrayHoleFields {}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MAssignmentExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MAssignmentExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MAssignmentExpressionFields {
        MAssignmentExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyMAssignment> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MAssignmentExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MAssignmentExpressionFields {
    pub left: SyntaxResult<AnyMAssignment>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MBinaryExpressionFields {
        MBinaryExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MBinaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MBinaryExpressionFields {
    pub left: SyntaxResult<AnyMExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MBlockStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MBlockStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MBlockStatementFields {
        MBlockStatementFields {
            l_curly_token: self.l_curly_token(),
            statements: self.statements(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn statements(&self) -> MStatementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MBlockStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MBlockStatementFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub statements: MStatementList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MBooleanLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MBooleanLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MBooleanLiteralExpressionFields {
        MBooleanLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MBooleanLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MBooleanLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MBreakStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MBreakStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MBreakStatementFields {
        MBreakStatementFields {
            break_token: self.break_token(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn break_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MBreakStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MBreakStatementFields {
    pub break_token: SyntaxResult<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MCallArguments {
    pub(crate) syntax: SyntaxNode,
}
impl MCallArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MCallArgumentsFields {
        MCallArgumentsFields {
            l_paren_token: self.l_paren_token(),
            args: self.args(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn args(&self) -> MCallArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MCallArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MCallArgumentsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub args: MCallArgumentList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MCallExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MCallExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MCallExpressionFields {
        MCallExpressionFields {
            callee: self.callee(),
            arguments: self.arguments(),
        }
    }
    pub fn callee(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn arguments(&self) -> SyntaxResult<MCallArguments> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MCallExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MCallExpressionFields {
    pub callee: SyntaxResult<AnyMExpression>,
    pub arguments: SyntaxResult<MCallArguments>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MCaseClause {
    pub(crate) syntax: SyntaxNode,
}
impl MCaseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MCaseClauseFields {
        MCaseClauseFields {
            case_token: self.case_token(),
            test: self.test(),
            colon_token: self.colon_token(),
            consequent: self.consequent(),
        }
    }
    pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn test(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn consequent(&self) -> MStatementList {
        support::list(&self.syntax, 3usize)
    }
}
impl Serialize for MCaseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MCaseClauseFields {
    pub case_token: SyntaxResult<SyntaxToken>,
    pub test: SyntaxResult<AnyMExpression>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub consequent: MStatementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MCatchClause {
    pub(crate) syntax: SyntaxNode,
}
impl MCatchClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MCatchClauseFields {
        MCatchClauseFields {
            catch_token: self.catch_token(),
            declaration: self.declaration(),
            body: self.body(),
        }
    }
    pub fn catch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declaration(&self) -> Option<MCatchDeclaration> {
        support::node(&self.syntax, 1usize)
    }
    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MCatchClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MCatchClauseFields {
    pub catch_token: SyntaxResult<SyntaxToken>,
    pub declaration: Option<MCatchDeclaration>,
    pub body: SyntaxResult<MBlockStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MCatchDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl MCatchDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MCatchDeclarationFields {
        MCatchDeclarationFields {
            l_paren_token: self.l_paren_token(),
            binding: self.binding(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MCatchDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MCatchDeclarationFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub binding: SyntaxResult<AnyMBinding>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MClassDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl MClassDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MClassDeclarationFields {
        MClassDeclarationFields {
            annotation: self.annotation(),
            class_token: self.class_token(),
            id: self.id(),
            extends_clause: self.extends_clause(),
            doc_string: self.doc_string(),
            l_curly_token: self.l_curly_token(),
            members: self.members(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn id(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn extends_clause(&self) -> Option<MExtendsClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 4usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn members(&self) -> MClassMemberList {
        support::list(&self.syntax, 6usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
}
impl Serialize for MClassDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MClassDeclarationFields {
    pub annotation: MAnnotationGroupList,
    pub class_token: SyntaxResult<SyntaxToken>,
    pub id: SyntaxResult<AnyMBinding>,
    pub extends_clause: Option<MExtendsClause>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub members: MClassMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MClassMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl MClassMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MClassMemberNameFields {
        MClassMemberNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MClassMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MClassMemberNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MComputedMemberAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl MComputedMemberAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MComputedMemberAssignmentFields {
        MComputedMemberAssignmentFields {
            object: self.object(),
            l_brack_token: self.l_brack_token(),
            member: self.member(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn object(&self) -> Option<AnyMExpression> {
        support::node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MComputedMemberAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MComputedMemberAssignmentFields {
    pub object: Option<AnyMExpression>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<AnyMExpression>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MComputedMemberExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MComputedMemberExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MComputedMemberExpressionFields {
        MComputedMemberExpressionFields {
            object: self.object(),
            l_brack_token: self.l_brack_token(),
            member: self.member(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn object(&self) -> Option<AnyMExpression> {
        support::node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MComputedMemberExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MComputedMemberExpressionFields {
    pub object: Option<AnyMExpression>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<AnyMExpression>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MComputedMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl MComputedMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MComputedMemberNameFields {
        MComputedMemberNameFields {
            l_brack_token: self.l_brack_token(),
            expression: self.expression(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MComputedMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MComputedMemberNameFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyMExpression>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MConditionalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MConditionalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MConditionalExpressionFields {
        MConditionalExpressionFields {
            test: self.test(),
            question_mark_token: self.question_mark_token(),
            consequent: self.consequent(),
            colon_token: self.colon_token(),
            alternate: self.alternate(),
        }
    }
    pub fn test(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn question_mark_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn consequent(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn alternate(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for MConditionalExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MConditionalExpressionFields {
    pub test: SyntaxResult<AnyMExpression>,
    pub question_mark_token: SyntaxResult<SyntaxToken>,
    pub consequent: SyntaxResult<AnyMExpression>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub alternate: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MConstantExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MConstantExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MConstantExpressionFields {
        MConstantExpressionFields {
            token: self.token(),
            constant: self.constant(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn constant(&self) -> SyntaxResult<AnyMStringLiteralExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MConstantExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MConstantExpressionFields {
    pub token: SyntaxResult<SyntaxToken>,
    pub constant: SyntaxResult<AnyMStringLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MConstructorClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl MConstructorClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MConstructorClassMemberFields {
        MConstructorClassMemberFields {
            annotation: self.annotation(),
            name: self.name(),
            parameters: self.parameters(),
            doc_string: self.doc_string(),
            body: self.body(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<MClassMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> SyntaxResult<MConstructorParameters> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for MConstructorClassMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MConstructorClassMemberFields {
    pub annotation: MAnnotationGroupList,
    pub name: SyntaxResult<MClassMemberName>,
    pub parameters: SyntaxResult<MConstructorParameters>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MConstructorParameters {
    pub(crate) syntax: SyntaxNode,
}
impl MConstructorParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MConstructorParametersFields {
        MConstructorParametersFields {
            l_paren_token: self.l_paren_token(),
            parameters: self.parameters(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> MConstructorParameterList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MConstructorParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MConstructorParametersFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub parameters: MConstructorParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MContinueStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MContinueStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MContinueStatementFields {
        MContinueStatementFields {
            continue_token: self.continue_token(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn continue_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MContinueStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MContinueStatementFields {
    pub continue_token: SyntaxResult<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MDateLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MDateLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MDateLiteralExpressionFields {
        MDateLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MDateLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MDateLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MDebugStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MDebugStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MDebugStatementFields {
        MDebugStatementFields {
            debug_token: self.debug_token(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn debug_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MDebugStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MDebugStatementFields {
    pub debug_token: SyntaxResult<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MDefaultClause {
    pub(crate) syntax: SyntaxNode,
}
impl MDefaultClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MDefaultClauseFields {
        MDefaultClauseFields {
            else_token: self.else_token(),
            consequent: self.consequent(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn consequent(&self) -> MStatementList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for MDefaultClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MDefaultClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub consequent: MStatementList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MDirective {
    pub(crate) syntax: SyntaxNode,
}
impl MDirective {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MDirectiveFields {
        MDirectiveFields {
            version_token: self.version_token(),
            value_token: self.value_token(),
        }
    }
    pub fn version_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MDirective {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MDirectiveFields {
    pub version_token: SyntaxResult<SyntaxToken>,
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl MElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MElseClauseFields {
        MElseClauseFields {
            else_token: self.else_token(),
            alternate: self.alternate(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn alternate(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub alternate: SyntaxResult<AnyMStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MEmptyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MEmptyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MEmptyStatementFields {
        MEmptyStatementFields {
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MEmptyStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MEmptyStatementFields {
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MExpressionSnipped {
    pub(crate) syntax: SyntaxNode,
}
impl MExpressionSnipped {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MExpressionSnippedFields {
        MExpressionSnippedFields {
            expression: self.expression(),
            eof_token: self.eof_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MExpressionSnipped {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MExpressionSnippedFields {
    pub expression: SyntaxResult<AnyMExpression>,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MExpressionStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MExpressionStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MExpressionStatementFields {
        MExpressionStatementFields {
            expression: self.expression(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MExpressionStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MExpressionStatementFields {
    pub expression: SyntaxResult<AnyMExpression>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MExtendedBinding {
    pub(crate) syntax: SyntaxNode,
}
impl MExtendedBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MExtendedBindingFields {
        MExtendedBindingFields {
            object: self.object(),
            operator_token: self.operator_token(),
            member: self.member(),
        }
    }
    pub fn object(&self) -> SyntaxResult<AnyMFunctionBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<MName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MExtendedBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MExtendedBindingFields {
    pub object: SyntaxResult<AnyMFunctionBinding>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<MName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MExtendsClause {
    pub(crate) syntax: SyntaxNode,
}
impl MExtendsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MExtendsClauseFields {
        MExtendsClauseFields {
            extends_token: self.extends_token(),
            super_class: self.super_class(),
        }
    }
    pub fn extends_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn super_class(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MExtendsClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MExtendsClauseFields {
    pub extends_token: SyntaxResult<SyntaxToken>,
    pub super_class: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MFinallyClause {
    pub(crate) syntax: SyntaxNode,
}
impl MFinallyClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MFinallyClauseFields {
        MFinallyClauseFields {
            finally_token: self.finally_token(),
            body: self.body(),
        }
    }
    pub fn finally_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MFinallyClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MFinallyClauseFields {
    pub finally_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<MBlockStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MForAllInStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MForAllInStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MForAllInStatementFields {
        MForAllInStatementFields {
            forall_token: self.forall_token(),
            l_paren_token: self.l_paren_token(),
            initializer: self.initializer(),
            in_token: self.in_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn forall_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> SyntaxResult<AnyMForInInitializer> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 6usize)
    }
}
impl Serialize for MForAllInStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MForAllInStatementFields {
    pub forall_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub initializer: SyntaxResult<AnyMForInInitializer>,
    pub in_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyMStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MForAllStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MForAllStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MForAllStatementFields {
        MForAllStatementFields {
            forall_token: self.forall_token(),
            l_paren_token: self.l_paren_token(),
            iter: self.iter(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn forall_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn iter(&self) -> SyntaxResult<MForIteratorFactory> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for MForAllStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MForAllStatementFields {
    pub forall_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub iter: SyntaxResult<MForIteratorFactory>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyMStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MForIteratorFactory {
    pub(crate) syntax: SyntaxNode,
}
impl MForIteratorFactory {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MForIteratorFactoryFields {
        MForIteratorFactoryFields {
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            comma_token: self.comma_token(),
            initializer: self.initializer(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<MIdentifierExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn initializer(&self) -> SyntaxResult<MVariableDeclarator> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for MForIteratorFactory {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MForIteratorFactoryFields {
    pub name: SyntaxResult<MIdentifierExpression>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyMExpression>,
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub initializer: SyntaxResult<MVariableDeclarator>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MForStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MForStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MForStatementFields {
        MForStatementFields {
            for_token: self.for_token(),
            l_paren_token: self.l_paren_token(),
            initializer: self.initializer(),
            first_semi_token: self.first_semi_token(),
            test: self.test(),
            second_semi_token: self.second_semi_token(),
            update: self.update(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn initializer(&self) -> Option<AnyMForInitializer> {
        support::node(&self.syntax, 2usize)
    }
    pub fn first_semi_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn test(&self) -> Option<AnyMExpression> {
        support::node(&self.syntax, 4usize)
    }
    pub fn second_semi_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn update(&self) -> Option<AnyMExpression> {
        support::node(&self.syntax, 6usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 8usize)
    }
}
impl Serialize for MForStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MForStatementFields {
    pub for_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub initializer: Option<AnyMForInitializer>,
    pub first_semi_token: SyntaxResult<SyntaxToken>,
    pub test: Option<AnyMExpression>,
    pub second_semi_token: SyntaxResult<SyntaxToken>,
    pub update: Option<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyMStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MForVariableDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl MForVariableDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MForVariableDeclarationFields {
        MForVariableDeclarationFields {
            var_token: self.var_token(),
            declarator: self.declarator(),
        }
    }
    pub fn var_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn declarator(&self) -> SyntaxResult<MVariableDeclarator> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MForVariableDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MForVariableDeclarationFields {
    pub var_token: Option<SyntaxToken>,
    pub declarator: SyntaxResult<MVariableDeclarator>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MFormalParameter {
    pub(crate) syntax: SyntaxNode,
}
impl MFormalParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MFormalParameterFields {
        MFormalParameterFields {
            binding: self.binding(),
            initializer: self.initializer(),
        }
    }
    pub fn binding(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<MInitializerClause> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for MFormalParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MFormalParameterFields {
    pub binding: SyntaxResult<AnyMBinding>,
    pub initializer: Option<MInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MFunctionBody {
    pub(crate) syntax: SyntaxNode,
}
impl MFunctionBody {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MFunctionBodyFields {
        MFunctionBodyFields {
            l_curly_token: self.l_curly_token(),
            directives: self.directives(),
            statements: self.statements(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn directives(&self) -> MDirectiveList {
        support::list(&self.syntax, 1usize)
    }
    pub fn statements(&self) -> MStatementList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MFunctionBody {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MFunctionBodyFields {
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub directives: MDirectiveList,
    pub statements: MStatementList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MFunctionDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl MFunctionDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MFunctionDeclarationFields {
        MFunctionDeclarationFields {
            annotation: self.annotation(),
            inline_token: self.inline_token(),
            function_token: self.function_token(),
            id: self.id(),
            parameters: self.parameters(),
            doc_string: self.doc_string(),
            body: self.body(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn inline_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn id(&self) -> SyntaxResult<AnyMFunctionBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn parameters(&self) -> SyntaxResult<MParameters> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 6usize)
    }
}
impl Serialize for MFunctionDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MFunctionDeclarationFields {
    pub annotation: MAnnotationGroupList,
    pub inline_token: Option<SyntaxToken>,
    pub function_token: SyntaxResult<SyntaxToken>,
    pub id: SyntaxResult<AnyMFunctionBinding>,
    pub parameters: SyntaxResult<MParameters>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MFunctionExpressionFields {
        MFunctionExpressionFields {
            function_token: self.function_token(),
            parameters: self.parameters(),
            body: self.body(),
        }
    }
    pub fn function_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn parameters(&self) -> SyntaxResult<MParameters> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MFunctionExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MFunctionExpressionFields {
    pub function_token: SyntaxResult<SyntaxToken>,
    pub parameters: SyntaxResult<MParameters>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MGetterClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl MGetterClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MGetterClassMemberFields {
        MGetterClassMemberFields {
            annotation: self.annotation(),
            get_token: self.get_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            r_paren_token: self.r_paren_token(),
            doc_string: self.doc_string(),
            body: self.body(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn get_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<MClassMemberName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 5usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 6usize)
    }
}
impl Serialize for MGetterClassMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MGetterClassMemberFields {
    pub annotation: MAnnotationGroupList,
    pub get_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<MClassMemberName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MHashMapExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MHashMapExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MHashMapExpressionFields {
        MHashMapExpressionFields {
            at_token: self.at_token(),
            l_paren_token: self.l_paren_token(),
            members: self.members(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn members(&self) -> MHashMapMemberList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MHashMapExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MHashMapExpressionFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub members: MHashMapMemberList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MHashSetExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MHashSetExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MHashSetExpressionFields {
        MHashSetExpressionFields {
            set_token: self.set_token(),
            l_paren_token: self.l_paren_token(),
            members: self.members(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn members(&self) -> MHashSetElementList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MHashSetExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MHashSetExpressionFields {
    pub set_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub members: MHashSetElementList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIdentifierAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl MIdentifierAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MIdentifierAssignmentFields {
        MIdentifierAssignmentFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MIdentifierAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MIdentifierAssignmentFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIdentifierBinding {
    pub(crate) syntax: SyntaxNode,
}
impl MIdentifierBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MIdentifierBindingFields {
        MIdentifierBindingFields {
            name_token: self.name_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MIdentifierBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MIdentifierBindingFields {
    pub name_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIdentifierExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MIdentifierExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MIdentifierExpressionFields {
        MIdentifierExpressionFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<MReferenceIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MIdentifierExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MIdentifierExpressionFields {
    pub name: SyntaxResult<MReferenceIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MIfStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MIfStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MIfStatementFields {
        MIfStatementFields {
            if_token: self.if_token(),
            l_paren_token: self.l_paren_token(),
            test: self.test(),
            r_paren_token: self.r_paren_token(),
            consequent: self.consequent(),
            else_clause: self.else_clause(),
        }
    }
    pub fn if_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn test(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn consequent(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn else_clause(&self) -> Option<MElseClause> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for MIfStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MIfStatementFields {
    pub if_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub test: SyntaxResult<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub consequent: SyntaxResult<AnyMStatement>,
    pub else_clause: Option<MElseClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MInExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MInExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MInExpressionFields {
        MInExpressionFields {
            property: self.property(),
            in_token: self.in_token(),
            object: self.object(),
        }
    }
    pub fn property(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn object(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MInExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MInExpressionFields {
    pub property: SyntaxResult<AnyMExpression>,
    pub in_token: SyntaxResult<SyntaxToken>,
    pub object: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MInitializerClause {
    pub(crate) syntax: SyntaxNode,
}
impl MInitializerClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MInitializerClauseFields {
        MInitializerClauseFields {
            eq_token: self.eq_token(),
            expression: self.expression(),
        }
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MInitializerClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MInitializerClauseFields {
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MInstanceofExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MInstanceofExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MInstanceofExpressionFields {
        MInstanceofExpressionFields {
            left: self.left(),
            instanceof_token: self.instanceof_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn instanceof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MInstanceofExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MInstanceofExpressionFields {
    pub left: SyntaxResult<AnyMExpression>,
    pub instanceof_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MLiteralMemberName {
    pub(crate) syntax: SyntaxNode,
}
impl MLiteralMemberName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MLiteralMemberNameFields {
        MLiteralMemberNameFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MLiteralMemberName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MLiteralMemberNameFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MLogicalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MLogicalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MLogicalExpressionFields {
        MLogicalExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MLogicalExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MLogicalExpressionFields {
    pub left: SyntaxResult<AnyMExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MLongStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MLongStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MLongStringLiteralExpressionFields {
        MLongStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MLongStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MLongStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MMethodClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl MMethodClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MMethodClassMemberFields {
        MMethodClassMemberFields {
            annotation: self.annotation(),
            name: self.name(),
            parameters: self.parameters(),
            doc_string: self.doc_string(),
            body: self.body(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<MClassMemberName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn parameters(&self) -> SyntaxResult<MParameters> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for MMethodClassMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MMethodClassMemberFields {
    pub annotation: MAnnotationGroupList,
    pub name: SyntaxResult<MClassMemberName>,
    pub parameters: SyntaxResult<MParameters>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MModule {
    pub(crate) syntax: SyntaxNode,
}
impl MModule {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MModuleFields {
        MModuleFields {
            directives: self.directives(),
            items: self.items(),
            eof_token: self.eof_token(),
        }
    }
    pub fn directives(&self) -> MDirectiveList {
        support::list(&self.syntax, 0usize)
    }
    pub fn items(&self) -> MModuleItemList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MModule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MModuleFields {
    pub directives: MDirectiveList,
    pub items: MModuleItemList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MName {
    pub(crate) syntax: SyntaxNode,
}
impl MName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MNameFields {
        MNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MNewExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MNewExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MNewExpressionFields {
        MNewExpressionFields {
            new_token: self.new_token(),
            callee: self.callee(),
            arguments: self.arguments(),
        }
    }
    pub fn new_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn callee(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn arguments(&self) -> Option<MCallArguments> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for MNewExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MNewExpressionFields {
    pub new_token: SyntaxResult<SyntaxToken>,
    pub callee: SyntaxResult<AnyMExpression>,
    pub arguments: Option<MCallArguments>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MNullLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MNullLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MNullLiteralExpressionFields {
        MNullLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MNullLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MNullLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MNumberLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MNumberLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MNumberLiteralExpressionFields {
        MNumberLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MNumberLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MNumberLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MObjectExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MObjectExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MObjectExpressionFields {
        MObjectExpressionFields {
            at_token: self.at_token(),
            l_curly_token: self.l_curly_token(),
            members: self.members(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn at_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn members(&self) -> MObjectMemberList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for MObjectExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MObjectExpressionFields {
    pub at_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub members: MObjectMemberList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MParameters {
    pub(crate) syntax: SyntaxNode,
}
impl MParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MParametersFields {
        MParametersFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> MParameterList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MParametersFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: MParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MParenthesizedAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl MParenthesizedAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MParenthesizedAssignmentFields {
        MParenthesizedAssignmentFields {
            l_paren_token: self.l_paren_token(),
            assignment: self.assignment(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn assignment(&self) -> SyntaxResult<AnyMAssignment> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MParenthesizedAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MParenthesizedAssignmentFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub assignment: SyntaxResult<AnyMAssignment>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MParenthesizedExpressionFields {
        MParenthesizedExpressionFields {
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for MParenthesizedExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MParenthesizedExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MPostUpdateExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MPostUpdateExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MPostUpdateExpressionFields {
        MPostUpdateExpressionFields {
            operand: self.operand(),
            operator_token: self.operator_token(),
        }
    }
    pub fn operand(&self) -> SyntaxResult<AnyMAssignment> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MPostUpdateExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MPostUpdateExpressionFields {
    pub operand: SyntaxResult<AnyMAssignment>,
    pub operator_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MPreUpdateExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MPreUpdateExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MPreUpdateExpressionFields {
        MPreUpdateExpressionFields {
            operator_token: self.operator_token(),
            operand: self.operand(),
        }
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn operand(&self) -> SyntaxResult<AnyMAssignment> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MPreUpdateExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MPreUpdateExpressionFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub operand: SyntaxResult<AnyMAssignment>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MPropertyObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl MPropertyObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MPropertyObjectMemberFields {
        MPropertyObjectMemberFields {
            name: self.name(),
            colon_token: self.colon_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<AnyMObjectMemberName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MPropertyObjectMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MPropertyObjectMemberFields {
    pub name: SyntaxResult<AnyMObjectMemberName>,
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReferenceIdentifier {
    pub(crate) syntax: SyntaxNode,
}
impl MReferenceIdentifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReferenceIdentifierFields {
        MReferenceIdentifierFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MReferenceIdentifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReferenceIdentifierFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReport {
    pub(crate) syntax: SyntaxNode,
}
impl MReport {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReportFields {
        MReportFields {
            name: self.name(),
            init: self.init(),
            default: self.default(),
            sections: self.sections(),
        }
    }
    pub fn name(&self) -> SyntaxResult<MReportName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn init(&self) -> MReportInitList {
        support::list(&self.syntax, 1usize)
    }
    pub fn default(&self) -> Option<MBlockStatement> {
        support::node(&self.syntax, 2usize)
    }
    pub fn sections(&self) -> MReportSectionList {
        support::list(&self.syntax, 3usize)
    }
}
impl Serialize for MReport {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReportFields {
    pub name: SyntaxResult<MReportName>,
    pub init: MReportInitList,
    pub default: Option<MBlockStatement>,
    pub sections: MReportSectionList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReportFile {
    pub(crate) syntax: SyntaxNode,
}
impl MReportFile {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReportFileFields {
        MReportFileFields {
            reports: self.reports(),
            eof_token: self.eof_token(),
        }
    }
    pub fn reports(&self) -> MReportList {
        support::list(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MReportFile {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReportFileFields {
    pub reports: MReportList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReportName {
    pub(crate) syntax: SyntaxNode,
}
impl MReportName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReportNameFields {
        MReportNameFields {
            ff2_token: self.ff2_token(),
            m_name: self.m_name(),
        }
    }
    pub fn ff2_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn m_name(&self) -> SyntaxResult<MName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MReportName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReportNameFields {
    pub ff2_token: SyntaxResult<SyntaxToken>,
    pub m_name: SyntaxResult<MName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReportSection {
    pub(crate) syntax: SyntaxNode,
}
impl MReportSection {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReportSectionFields {
        MReportSectionFields {
            name: self.name(),
            body: self.body(),
        }
    }
    pub fn name(&self) -> SyntaxResult<MReportSectionName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MReportSection {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReportSectionFields {
    pub name: SyntaxResult<MReportSectionName>,
    pub body: SyntaxResult<MBlockStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReportSectionName {
    pub(crate) syntax: SyntaxNode,
}
impl MReportSectionName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReportSectionNameFields {
        MReportSectionNameFields {
            ff_token: self.ff_token(),
            m_name: self.m_name(),
        }
    }
    pub fn ff_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn m_name(&self) -> SyntaxResult<MName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MReportSectionName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReportSectionNameFields {
    pub ff_token: SyntaxResult<SyntaxToken>,
    pub m_name: SyntaxResult<MName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MRestParameter {
    pub(crate) syntax: SyntaxNode,
}
impl MRestParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MRestParameterFields {
        MRestParameterFields {
            dotdotdot_token: self.dotdotdot_token(),
            binding: self.binding(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn binding(&self) -> Option<AnyMBinding> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for MRestParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MRestParameterFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub binding: Option<AnyMBinding>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MReturnStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MReturnStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MReturnStatementFields {
        MReturnStatementFields {
            return_token: self.return_token(),
            argument: self.argument(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn return_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> Option<AnyMExpression> {
        support::node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for MReturnStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MReturnStatementFields {
    pub return_token: SyntaxResult<SyntaxToken>,
    pub argument: Option<AnyMExpression>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MScript {
    pub(crate) syntax: SyntaxNode,
}
impl MScript {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MScriptFields {
        MScriptFields {
            statements: self.statements(),
            eof_token: self.eof_token(),
        }
    }
    pub fn statements(&self) -> MStatementList {
        support::list(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for MScript {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MScriptFields {
    pub statements: MStatementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSequenceExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MSequenceExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSequenceExpressionFields {
        MSequenceExpressionFields {
            left: self.left(),
            comma_token: self.comma_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn comma_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MSequenceExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSequenceExpressionFields {
    pub left: SyntaxResult<AnyMExpression>,
    pub comma_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSetterClassMember {
    pub(crate) syntax: SyntaxNode,
}
impl MSetterClassMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSetterClassMemberFields {
        MSetterClassMemberFields {
            annotation: self.annotation(),
            set_token: self.set_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            parameter: self.parameter(),
            comma_token: self.comma_token(),
            r_paren_token: self.r_paren_token(),
            doc_string: self.doc_string(),
            body: self.body(),
        }
    }
    pub fn annotation(&self) -> MAnnotationGroupList {
        support::list(&self.syntax, 0usize)
    }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<MClassMemberName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn parameter(&self) -> SyntaxResult<AnyMFormalParameter> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn comma_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn doc_string(&self) -> Option<AnyMStringLiteralExpression> {
        support::node(&self.syntax, 7usize)
    }
    pub fn body(&self) -> SyntaxResult<MFunctionBody> {
        support::required_node(&self.syntax, 8usize)
    }
}
impl Serialize for MSetterClassMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSetterClassMemberFields {
    pub annotation: MAnnotationGroupList,
    pub set_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<MClassMemberName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub parameter: SyntaxResult<AnyMFormalParameter>,
    pub comma_token: Option<SyntaxToken>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub doc_string: Option<AnyMStringLiteralExpression>,
    pub body: SyntaxResult<MFunctionBody>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MShorthandPropertyObjectMember {
    pub(crate) syntax: SyntaxNode,
}
impl MShorthandPropertyObjectMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MShorthandPropertyObjectMemberFields {
        MShorthandPropertyObjectMemberFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<MReferenceIdentifier> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MShorthandPropertyObjectMember {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MShorthandPropertyObjectMemberFields {
    pub name: SyntaxResult<MReferenceIdentifier>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSpread {
    pub(crate) syntax: SyntaxNode,
}
impl MSpread {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSpreadFields {
        MSpreadFields {
            dotdotdot_token: self.dotdotdot_token(),
            argument: self.argument(),
        }
    }
    pub fn dotdotdot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MSpread {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSpreadFields {
    pub dotdotdot_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSqlConcatenationExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MSqlConcatenationExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSqlConcatenationExpressionFields {
        MSqlConcatenationExpressionFields {
            expression: self.expression(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<MBinaryExpression> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for MSqlConcatenationExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSqlConcatenationExpressionFields {
    pub expression: SyntaxResult<MBinaryExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSqlLongStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MSqlLongStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSqlLongStringLiteralExpressionFields {
        MSqlLongStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MSqlLongStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSqlLongStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSqlStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MSqlStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSqlStringLiteralExpressionFields {
        MSqlStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MSqlStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSqlStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MStaticMemberAssignment {
    pub(crate) syntax: SyntaxNode,
}
impl MStaticMemberAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MStaticMemberAssignmentFields {
        MStaticMemberAssignmentFields {
            object: self.object(),
            dot_token: self.dot_token(),
            member: self.member(),
        }
    }
    pub fn object(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<MName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MStaticMemberAssignment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MStaticMemberAssignmentFields {
    pub object: SyntaxResult<AnyMExpression>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<MName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MStaticMemberExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MStaticMemberExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MStaticMemberExpressionFields {
        MStaticMemberExpressionFields {
            object: self.object(),
            operator_token: self.operator_token(),
            member: self.member(),
        }
    }
    pub fn object(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn member(&self) -> SyntaxResult<MName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MStaticMemberExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MStaticMemberExpressionFields {
    pub object: SyntaxResult<AnyMExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub member: SyntaxResult<MName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MStringLiteralExpressionFields {
        MStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSuperExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MSuperExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSuperExpressionFields {
        MSuperExpressionFields {
            super_token: self.super_token(),
        }
    }
    pub fn super_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MSuperExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSuperExpressionFields {
    pub super_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MSwitchStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MSwitchStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MSwitchStatementFields {
        MSwitchStatementFields {
            switch_token: self.switch_token(),
            l_paren_token: self.l_paren_token(),
            discriminant: self.discriminant(),
            r_paren_token: self.r_paren_token(),
            l_curly_token: self.l_curly_token(),
            cases: self.cases(),
            r_curly_token: self.r_curly_token(),
        }
    }
    pub fn switch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn discriminant(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn cases(&self) -> MSwitchCaseList {
        support::list(&self.syntax, 5usize)
    }
    pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl Serialize for MSwitchStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MSwitchStatementFields {
    pub switch_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub discriminant: SyntaxResult<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub l_curly_token: SyntaxResult<SyntaxToken>,
    pub cases: MSwitchCaseList,
    pub r_curly_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MTemplateExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MTemplateExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MTemplateExpressionFields {
        MTemplateExpressionFields {
            token: self.token(),
            template: self.template(),
        }
    }
    pub fn token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn template(&self) -> SyntaxResult<AnyMStringLiteralExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MTemplateExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MTemplateExpressionFields {
    pub token: SyntaxResult<SyntaxToken>,
    pub template: SyntaxResult<AnyMStringLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MThisExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MThisExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MThisExpressionFields {
        MThisExpressionFields {
            this_token: self.this_token(),
        }
    }
    pub fn this_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MThisExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MThisExpressionFields {
    pub this_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MThrowStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MThrowStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MThrowStatementFields {
        MThrowStatementFields {
            throw_token: self.throw_token(),
            argument: self.argument(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn throw_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
}
impl Serialize for MThrowStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MThrowStatementFields {
    pub throw_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<AnyMExpression>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MTimeLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MTimeLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MTimeLiteralExpressionFields {
        MTimeLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for MTimeLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MTimeLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MTryFinallyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MTryFinallyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MTryFinallyStatementFields {
        MTryFinallyStatementFields {
            try_token: self.try_token(),
            body: self.body(),
            catch_clause: self.catch_clause(),
            finally_clause: self.finally_clause(),
        }
    }
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn catch_clause(&self) -> Option<MCatchClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn finally_clause(&self) -> SyntaxResult<MFinallyClause> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for MTryFinallyStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MTryFinallyStatementFields {
    pub try_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<MBlockStatement>,
    pub catch_clause: Option<MCatchClause>,
    pub finally_clause: SyntaxResult<MFinallyClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MTryStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MTryStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MTryStatementFields {
        MTryStatementFields {
            try_token: self.try_token(),
            body: self.body(),
            catch_clause: self.catch_clause(),
        }
    }
    pub fn try_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn body(&self) -> SyntaxResult<MBlockStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn catch_clause(&self) -> SyntaxResult<MCatchClause> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for MTryStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MTryStatementFields {
    pub try_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<MBlockStatement>,
    pub catch_clause: SyntaxResult<MCatchClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl MUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MUnaryExpressionFields {
        MUnaryExpressionFields {
            operator_token: self.operator_token(),
            argument: self.argument(),
        }
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn argument(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for MUnaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MUnaryExpressionFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub argument: SyntaxResult<AnyMExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MVariableDeclaration {
    pub(crate) syntax: SyntaxNode,
}
impl MVariableDeclaration {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MVariableDeclarationFields {
        MVariableDeclarationFields {
            kind_token: self.kind_token(),
            declarators: self.declarators(),
        }
    }
    pub fn kind_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn declarators(&self) -> MVariableDeclaratorList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for MVariableDeclaration {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MVariableDeclarationFields {
    pub kind_token: SyntaxResult<SyntaxToken>,
    pub declarators: MVariableDeclaratorList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MVariableDeclarationClause {
    pub(crate) syntax: SyntaxNode,
}
impl MVariableDeclarationClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MVariableDeclarationClauseFields {
        MVariableDeclarationClauseFields {
            declaration: self.declaration(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn declaration(&self) -> SyntaxResult<MVariableDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MVariableDeclarationClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MVariableDeclarationClauseFields {
    pub declaration: SyntaxResult<MVariableDeclaration>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MVariableDeclarator {
    pub(crate) syntax: SyntaxNode,
}
impl MVariableDeclarator {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MVariableDeclaratorFields {
        MVariableDeclaratorFields {
            id: self.id(),
            initializer: self.initializer(),
        }
    }
    pub fn id(&self) -> SyntaxResult<AnyMBinding> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn initializer(&self) -> Option<MInitializerClause> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for MVariableDeclarator {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MVariableDeclaratorFields {
    pub id: SyntaxResult<AnyMBinding>,
    pub initializer: Option<MInitializerClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MVariableStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MVariableStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MVariableStatementFields {
        MVariableStatementFields {
            declaration: self.declaration(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn declaration(&self) -> SyntaxResult<MVariableDeclaration> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for MVariableStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MVariableStatementFields {
    pub declaration: SyntaxResult<MVariableDeclaration>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct MWhileStatement {
    pub(crate) syntax: SyntaxNode,
}
impl MWhileStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> MWhileStatementFields {
        MWhileStatementFields {
            while_token: self.while_token(),
            l_paren_token: self.l_paren_token(),
            test: self.test(),
            r_paren_token: self.r_paren_token(),
            body: self.body(),
        }
    }
    pub fn while_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn test(&self) -> SyntaxResult<AnyMExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn body(&self) -> SyntaxResult<AnyMStatement> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for MWhileStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct MWhileStatementFields {
    pub while_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub test: SyntaxResult<AnyMExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<AnyMStatement>,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMAnnotationElement {
    MAnnotationBinding(MAnnotationBinding),
    MAnnotationElement(MAnnotationElement),
}
impl AnyMAnnotationElement {
    pub fn as_m_annotation_binding(&self) -> Option<&MAnnotationBinding> {
        match &self {
            Self::MAnnotationBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_annotation_element(&self) -> Option<&MAnnotationElement> {
        match &self {
            Self::MAnnotationElement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMArrayElement {
    AnyMExpression(AnyMExpression),
    MArrayHole(MArrayHole),
    MSpread(MSpread),
}
impl AnyMArrayElement {
    pub fn as_any_m_expression(&self) -> Option<&AnyMExpression> {
        match &self {
            Self::AnyMExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_array_hole(&self) -> Option<&MArrayHole> {
        match &self {
            Self::MArrayHole(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_spread(&self) -> Option<&MSpread> {
        match &self {
            Self::MSpread(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMAssignment {
    MBogusAssignment(MBogusAssignment),
    MComputedMemberAssignment(MComputedMemberAssignment),
    MIdentifierAssignment(MIdentifierAssignment),
    MParenthesizedAssignment(MParenthesizedAssignment),
    MStaticMemberAssignment(MStaticMemberAssignment),
}
impl AnyMAssignment {
    pub fn as_m_bogus_assignment(&self) -> Option<&MBogusAssignment> {
        match &self {
            Self::MBogusAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_computed_member_assignment(&self) -> Option<&MComputedMemberAssignment> {
        match &self {
            Self::MComputedMemberAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_identifier_assignment(&self) -> Option<&MIdentifierAssignment> {
        match &self {
            Self::MIdentifierAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_parenthesized_assignment(&self) -> Option<&MParenthesizedAssignment> {
        match &self {
            Self::MParenthesizedAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_static_member_assignment(&self) -> Option<&MStaticMemberAssignment> {
        match &self {
            Self::MStaticMemberAssignment(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMBinding {
    MBogusBinding(MBogusBinding),
    MIdentifierBinding(MIdentifierBinding),
}
impl AnyMBinding {
    pub fn as_m_bogus_binding(&self) -> Option<&MBogusBinding> {
        match &self {
            Self::MBogusBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_identifier_binding(&self) -> Option<&MIdentifierBinding> {
        match &self {
            Self::MIdentifierBinding(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMCallArgument {
    AnyMExpression(AnyMExpression),
    MSpread(MSpread),
}
impl AnyMCallArgument {
    pub fn as_any_m_expression(&self) -> Option<&AnyMExpression> {
        match &self {
            Self::AnyMExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_spread(&self) -> Option<&MSpread> {
        match &self {
            Self::MSpread(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMClassMember {
    MBogusMember(MBogusMember),
    MConstructorClassMember(MConstructorClassMember),
    MGetterClassMember(MGetterClassMember),
    MMethodClassMember(MMethodClassMember),
    MSetterClassMember(MSetterClassMember),
}
impl AnyMClassMember {
    pub fn as_m_bogus_member(&self) -> Option<&MBogusMember> {
        match &self {
            Self::MBogusMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_constructor_class_member(&self) -> Option<&MConstructorClassMember> {
        match &self {
            Self::MConstructorClassMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_getter_class_member(&self) -> Option<&MGetterClassMember> {
        match &self {
            Self::MGetterClassMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_method_class_member(&self) -> Option<&MMethodClassMember> {
        match &self {
            Self::MMethodClassMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_setter_class_member(&self) -> Option<&MSetterClassMember> {
        match &self {
            Self::MSetterClassMember(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMConstructorParameter {
    AnyMFormalParameter(AnyMFormalParameter),
    MRestParameter(MRestParameter),
}
impl AnyMConstructorParameter {
    pub fn as_any_m_formal_parameter(&self) -> Option<&AnyMFormalParameter> {
        match &self {
            Self::AnyMFormalParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_rest_parameter(&self) -> Option<&MRestParameter> {
        match &self {
            Self::MRestParameter(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMDeclaration {
    MClassDeclaration(MClassDeclaration),
    MFunctionDeclaration(MFunctionDeclaration),
    MVariableDeclaration(MVariableDeclaration),
}
impl AnyMDeclaration {
    pub fn as_m_class_declaration(&self) -> Option<&MClassDeclaration> {
        match &self {
            Self::MClassDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_declaration(&self) -> Option<&MFunctionDeclaration> {
        match &self {
            Self::MFunctionDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_variable_declaration(&self) -> Option<&MVariableDeclaration> {
        match &self {
            Self::MVariableDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMDeclarationClause {
    MClassDeclaration(MClassDeclaration),
    MFunctionDeclaration(MFunctionDeclaration),
    MVariableDeclarationClause(MVariableDeclarationClause),
}
impl AnyMDeclarationClause {
    pub fn as_m_class_declaration(&self) -> Option<&MClassDeclaration> {
        match &self {
            Self::MClassDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_declaration(&self) -> Option<&MFunctionDeclaration> {
        match &self {
            Self::MFunctionDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_variable_declaration_clause(&self) -> Option<&MVariableDeclarationClause> {
        match &self {
            Self::MVariableDeclarationClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMExpression {
    AnyMLiteralExpression(AnyMLiteralExpression),
    MArrayExpression(MArrayExpression),
    MAssignmentExpression(MAssignmentExpression),
    MBinaryExpression(MBinaryExpression),
    MBogusExpression(MBogusExpression),
    MCallExpression(MCallExpression),
    MComputedMemberExpression(MComputedMemberExpression),
    MConditionalExpression(MConditionalExpression),
    MConstantExpression(MConstantExpression),
    MFunctionExpression(MFunctionExpression),
    MHashMapExpression(MHashMapExpression),
    MHashSetExpression(MHashSetExpression),
    MIdentifierExpression(MIdentifierExpression),
    MInExpression(MInExpression),
    MInstanceofExpression(MInstanceofExpression),
    MLogicalExpression(MLogicalExpression),
    MNewExpression(MNewExpression),
    MObjectExpression(MObjectExpression),
    MParenthesizedExpression(MParenthesizedExpression),
    MPostUpdateExpression(MPostUpdateExpression),
    MPreUpdateExpression(MPreUpdateExpression),
    MSequenceExpression(MSequenceExpression),
    MSqlConcatenationExpression(MSqlConcatenationExpression),
    MStaticMemberExpression(MStaticMemberExpression),
    MSuperExpression(MSuperExpression),
    MTemplateExpression(MTemplateExpression),
    MThisExpression(MThisExpression),
    MUnaryExpression(MUnaryExpression),
}
impl AnyMExpression {
    pub fn as_any_m_literal_expression(&self) -> Option<&AnyMLiteralExpression> {
        match &self {
            Self::AnyMLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_array_expression(&self) -> Option<&MArrayExpression> {
        match &self {
            Self::MArrayExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_assignment_expression(&self) -> Option<&MAssignmentExpression> {
        match &self {
            Self::MAssignmentExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_binary_expression(&self) -> Option<&MBinaryExpression> {
        match &self {
            Self::MBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_bogus_expression(&self) -> Option<&MBogusExpression> {
        match &self {
            Self::MBogusExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_call_expression(&self) -> Option<&MCallExpression> {
        match &self {
            Self::MCallExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_computed_member_expression(&self) -> Option<&MComputedMemberExpression> {
        match &self {
            Self::MComputedMemberExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_conditional_expression(&self) -> Option<&MConditionalExpression> {
        match &self {
            Self::MConditionalExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_constant_expression(&self) -> Option<&MConstantExpression> {
        match &self {
            Self::MConstantExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_expression(&self) -> Option<&MFunctionExpression> {
        match &self {
            Self::MFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_hash_map_expression(&self) -> Option<&MHashMapExpression> {
        match &self {
            Self::MHashMapExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_hash_set_expression(&self) -> Option<&MHashSetExpression> {
        match &self {
            Self::MHashSetExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_identifier_expression(&self) -> Option<&MIdentifierExpression> {
        match &self {
            Self::MIdentifierExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_in_expression(&self) -> Option<&MInExpression> {
        match &self {
            Self::MInExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_instanceof_expression(&self) -> Option<&MInstanceofExpression> {
        match &self {
            Self::MInstanceofExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_logical_expression(&self) -> Option<&MLogicalExpression> {
        match &self {
            Self::MLogicalExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_new_expression(&self) -> Option<&MNewExpression> {
        match &self {
            Self::MNewExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_object_expression(&self) -> Option<&MObjectExpression> {
        match &self {
            Self::MObjectExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_parenthesized_expression(&self) -> Option<&MParenthesizedExpression> {
        match &self {
            Self::MParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_post_update_expression(&self) -> Option<&MPostUpdateExpression> {
        match &self {
            Self::MPostUpdateExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_pre_update_expression(&self) -> Option<&MPreUpdateExpression> {
        match &self {
            Self::MPreUpdateExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sequence_expression(&self) -> Option<&MSequenceExpression> {
        match &self {
            Self::MSequenceExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sql_concatenation_expression(&self) -> Option<&MSqlConcatenationExpression> {
        match &self {
            Self::MSqlConcatenationExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_static_member_expression(&self) -> Option<&MStaticMemberExpression> {
        match &self {
            Self::MStaticMemberExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_super_expression(&self) -> Option<&MSuperExpression> {
        match &self {
            Self::MSuperExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_template_expression(&self) -> Option<&MTemplateExpression> {
        match &self {
            Self::MTemplateExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_this_expression(&self) -> Option<&MThisExpression> {
        match &self {
            Self::MThisExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_unary_expression(&self) -> Option<&MUnaryExpression> {
        match &self {
            Self::MUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMForInInitializer {
    AnyMAssignment(AnyMAssignment),
    MForVariableDeclaration(MForVariableDeclaration),
}
impl AnyMForInInitializer {
    pub fn as_any_m_assignment(&self) -> Option<&AnyMAssignment> {
        match &self {
            Self::AnyMAssignment(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_for_variable_declaration(&self) -> Option<&MForVariableDeclaration> {
        match &self {
            Self::MForVariableDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMForInitializer {
    AnyMExpression(AnyMExpression),
    MVariableDeclaration(MVariableDeclaration),
}
impl AnyMForInitializer {
    pub fn as_any_m_expression(&self) -> Option<&AnyMExpression> {
        match &self {
            Self::AnyMExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_variable_declaration(&self) -> Option<&MVariableDeclaration> {
        match &self {
            Self::MVariableDeclaration(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMFormalParameter {
    MBogusParameter(MBogusParameter),
    MFormalParameter(MFormalParameter),
}
impl AnyMFormalParameter {
    pub fn as_m_bogus_parameter(&self) -> Option<&MBogusParameter> {
        match &self {
            Self::MBogusParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_formal_parameter(&self) -> Option<&MFormalParameter> {
        match &self {
            Self::MFormalParameter(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMFunction {
    MFunctionDeclaration(MFunctionDeclaration),
    MFunctionExpression(MFunctionExpression),
}
impl AnyMFunction {
    pub fn as_m_function_declaration(&self) -> Option<&MFunctionDeclaration> {
        match &self {
            Self::MFunctionDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_expression(&self) -> Option<&MFunctionExpression> {
        match &self {
            Self::MFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMFunctionBinding {
    MBogusBinding(MBogusBinding),
    MExtendedBinding(MExtendedBinding),
    MIdentifierBinding(MIdentifierBinding),
}
impl AnyMFunctionBinding {
    pub fn as_m_bogus_binding(&self) -> Option<&MBogusBinding> {
        match &self {
            Self::MBogusBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_extended_binding(&self) -> Option<&MExtendedBinding> {
        match &self {
            Self::MExtendedBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_identifier_binding(&self) -> Option<&MIdentifierBinding> {
        match &self {
            Self::MIdentifierBinding(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMFunctionBody {
    AnyMExpression(AnyMExpression),
    MFunctionBody(MFunctionBody),
}
impl AnyMFunctionBody {
    pub fn as_any_m_expression(&self) -> Option<&AnyMExpression> {
        match &self {
            Self::AnyMExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_body(&self) -> Option<&MFunctionBody> {
        match &self {
            Self::MFunctionBody(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMLiteralExpression {
    MBooleanLiteralExpression(MBooleanLiteralExpression),
    MDateLiteralExpression(MDateLiteralExpression),
    MLongStringLiteralExpression(MLongStringLiteralExpression),
    MNullLiteralExpression(MNullLiteralExpression),
    MNumberLiteralExpression(MNumberLiteralExpression),
    MSqlLongStringLiteralExpression(MSqlLongStringLiteralExpression),
    MSqlStringLiteralExpression(MSqlStringLiteralExpression),
    MStringLiteralExpression(MStringLiteralExpression),
    MTimeLiteralExpression(MTimeLiteralExpression),
}
impl AnyMLiteralExpression {
    pub fn as_m_boolean_literal_expression(&self) -> Option<&MBooleanLiteralExpression> {
        match &self {
            Self::MBooleanLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_date_literal_expression(&self) -> Option<&MDateLiteralExpression> {
        match &self {
            Self::MDateLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_long_string_literal_expression(&self) -> Option<&MLongStringLiteralExpression> {
        match &self {
            Self::MLongStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_null_literal_expression(&self) -> Option<&MNullLiteralExpression> {
        match &self {
            Self::MNullLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_number_literal_expression(&self) -> Option<&MNumberLiteralExpression> {
        match &self {
            Self::MNumberLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sql_long_string_literal_expression(
        &self,
    ) -> Option<&MSqlLongStringLiteralExpression> {
        match &self {
            Self::MSqlLongStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sql_string_literal_expression(&self) -> Option<&MSqlStringLiteralExpression> {
        match &self {
            Self::MSqlStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_string_literal_expression(&self) -> Option<&MStringLiteralExpression> {
        match &self {
            Self::MStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_time_literal_expression(&self) -> Option<&MTimeLiteralExpression> {
        match &self {
            Self::MTimeLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMObjectMember {
    MBogusMember(MBogusMember),
    MPropertyObjectMember(MPropertyObjectMember),
    MShorthandPropertyObjectMember(MShorthandPropertyObjectMember),
    MSpread(MSpread),
}
impl AnyMObjectMember {
    pub fn as_m_bogus_member(&self) -> Option<&MBogusMember> {
        match &self {
            Self::MBogusMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_property_object_member(&self) -> Option<&MPropertyObjectMember> {
        match &self {
            Self::MPropertyObjectMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_shorthand_property_object_member(&self) -> Option<&MShorthandPropertyObjectMember> {
        match &self {
            Self::MShorthandPropertyObjectMember(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_spread(&self) -> Option<&MSpread> {
        match &self {
            Self::MSpread(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMObjectMemberName {
    MComputedMemberName(MComputedMemberName),
    MLiteralMemberName(MLiteralMemberName),
}
impl AnyMObjectMemberName {
    pub fn as_m_computed_member_name(&self) -> Option<&MComputedMemberName> {
        match &self {
            Self::MComputedMemberName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_literal_member_name(&self) -> Option<&MLiteralMemberName> {
        match &self {
            Self::MLiteralMemberName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMParameter {
    AnyMFormalParameter(AnyMFormalParameter),
    MRestParameter(MRestParameter),
}
impl AnyMParameter {
    pub fn as_any_m_formal_parameter(&self) -> Option<&AnyMFormalParameter> {
        match &self {
            Self::AnyMFormalParameter(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_rest_parameter(&self) -> Option<&MRestParameter> {
        match &self {
            Self::MRestParameter(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMRoot {
    MExpressionSnipped(MExpressionSnipped),
    MModule(MModule),
    MReportFile(MReportFile),
    MScript(MScript),
}
impl AnyMRoot {
    pub fn as_m_expression_snipped(&self) -> Option<&MExpressionSnipped> {
        match &self {
            Self::MExpressionSnipped(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_module(&self) -> Option<&MModule> {
        match &self {
            Self::MModule(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_report_file(&self) -> Option<&MReportFile> {
        match &self {
            Self::MReportFile(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_script(&self) -> Option<&MScript> {
        match &self {
            Self::MScript(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMStatement {
    MBlockStatement(MBlockStatement),
    MBogusStatement(MBogusStatement),
    MBreakStatement(MBreakStatement),
    MClassDeclaration(MClassDeclaration),
    MContinueStatement(MContinueStatement),
    MDebugStatement(MDebugStatement),
    MEmptyStatement(MEmptyStatement),
    MExpressionStatement(MExpressionStatement),
    MForAllInStatement(MForAllInStatement),
    MForAllStatement(MForAllStatement),
    MForStatement(MForStatement),
    MFunctionDeclaration(MFunctionDeclaration),
    MIfStatement(MIfStatement),
    MReturnStatement(MReturnStatement),
    MSwitchStatement(MSwitchStatement),
    MThrowStatement(MThrowStatement),
    MTryFinallyStatement(MTryFinallyStatement),
    MTryStatement(MTryStatement),
    MVariableStatement(MVariableStatement),
    MWhileStatement(MWhileStatement),
}
impl AnyMStatement {
    pub fn as_m_block_statement(&self) -> Option<&MBlockStatement> {
        match &self {
            Self::MBlockStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_bogus_statement(&self) -> Option<&MBogusStatement> {
        match &self {
            Self::MBogusStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_break_statement(&self) -> Option<&MBreakStatement> {
        match &self {
            Self::MBreakStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_class_declaration(&self) -> Option<&MClassDeclaration> {
        match &self {
            Self::MClassDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_continue_statement(&self) -> Option<&MContinueStatement> {
        match &self {
            Self::MContinueStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_debug_statement(&self) -> Option<&MDebugStatement> {
        match &self {
            Self::MDebugStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_empty_statement(&self) -> Option<&MEmptyStatement> {
        match &self {
            Self::MEmptyStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_expression_statement(&self) -> Option<&MExpressionStatement> {
        match &self {
            Self::MExpressionStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_for_all_in_statement(&self) -> Option<&MForAllInStatement> {
        match &self {
            Self::MForAllInStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_for_all_statement(&self) -> Option<&MForAllStatement> {
        match &self {
            Self::MForAllStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_for_statement(&self) -> Option<&MForStatement> {
        match &self {
            Self::MForStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_function_declaration(&self) -> Option<&MFunctionDeclaration> {
        match &self {
            Self::MFunctionDeclaration(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_if_statement(&self) -> Option<&MIfStatement> {
        match &self {
            Self::MIfStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_return_statement(&self) -> Option<&MReturnStatement> {
        match &self {
            Self::MReturnStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_switch_statement(&self) -> Option<&MSwitchStatement> {
        match &self {
            Self::MSwitchStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_throw_statement(&self) -> Option<&MThrowStatement> {
        match &self {
            Self::MThrowStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_try_finally_statement(&self) -> Option<&MTryFinallyStatement> {
        match &self {
            Self::MTryFinallyStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_try_statement(&self) -> Option<&MTryStatement> {
        match &self {
            Self::MTryStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_variable_statement(&self) -> Option<&MVariableStatement> {
        match &self {
            Self::MVariableStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_while_statement(&self) -> Option<&MWhileStatement> {
        match &self {
            Self::MWhileStatement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMStringLiteralExpression {
    MLongStringLiteralExpression(MLongStringLiteralExpression),
    MSqlLongStringLiteralExpression(MSqlLongStringLiteralExpression),
    MSqlStringLiteralExpression(MSqlStringLiteralExpression),
    MStringLiteralExpression(MStringLiteralExpression),
}
impl AnyMStringLiteralExpression {
    pub fn as_m_long_string_literal_expression(&self) -> Option<&MLongStringLiteralExpression> {
        match &self {
            Self::MLongStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sql_long_string_literal_expression(
        &self,
    ) -> Option<&MSqlLongStringLiteralExpression> {
        match &self {
            Self::MSqlLongStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_sql_string_literal_expression(&self) -> Option<&MSqlStringLiteralExpression> {
        match &self {
            Self::MSqlStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_string_literal_expression(&self) -> Option<&MStringLiteralExpression> {
        match &self {
            Self::MStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyMSwitchClause {
    MCaseClause(MCaseClause),
    MDefaultClause(MDefaultClause),
}
impl AnyMSwitchClause {
    pub fn as_m_case_clause(&self) -> Option<&MCaseClause> {
        match &self {
            Self::MCaseClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_m_default_clause(&self) -> Option<&MDefaultClause> {
        match &self {
            Self::MDefaultClause(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for MAnnotationAttribute {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_ATTRIBUTE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_ATTRIBUTE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MAnnotationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MAnnotationAttribute")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("MAnnotationAttribute").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MAnnotationAttribute> for SyntaxNode {
    fn from(n: MAnnotationAttribute) -> Self {
        n.syntax
    }
}
impl From<MAnnotationAttribute> for SyntaxElement {
    fn from(n: MAnnotationAttribute) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MAnnotationBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MAnnotationBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MAnnotationBinding")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("MAnnotationBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MAnnotationBinding> for SyntaxNode {
    fn from(n: MAnnotationBinding) -> Self {
        n.syntax
    }
}
impl From<MAnnotationBinding> for SyntaxElement {
    fn from(n: MAnnotationBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MAnnotationElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_ELEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_ELEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MAnnotationElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MAnnotationElement")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("attributes", &self.attributes())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MAnnotationElement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MAnnotationElement> for SyntaxNode {
    fn from(n: MAnnotationElement) -> Self {
        n.syntax
    }
}
impl From<MAnnotationElement> for SyntaxElement {
    fn from(n: MAnnotationElement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MAnnotationGroup {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_GROUP as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_GROUP
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MAnnotationGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MAnnotationGroup")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("elements", &self.elements())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MAnnotationGroup").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MAnnotationGroup> for SyntaxNode {
    fn from(n: MAnnotationGroup) -> Self {
        n.syntax
    }
}
impl From<MAnnotationGroup> for SyntaxElement {
    fn from(n: MAnnotationGroup) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MArrayExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ARRAY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ARRAY_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MArrayExpression")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("elements", &self.elements())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MArrayExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MArrayExpression> for SyntaxNode {
    fn from(n: MArrayExpression) -> Self {
        n.syntax
    }
}
impl From<MArrayExpression> for SyntaxElement {
    fn from(n: MArrayExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MArrayHole {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ARRAY_HOLE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ARRAY_HOLE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MArrayHole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MArrayHole").finish()
    }
}
impl From<MArrayHole> for SyntaxNode {
    fn from(n: MArrayHole) -> Self {
        n.syntax
    }
}
impl From<MArrayHole> for SyntaxElement {
    fn from(n: MArrayHole) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MAssignmentExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ASSIGNMENT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ASSIGNMENT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MAssignmentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MAssignmentExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("MAssignmentExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MAssignmentExpression> for SyntaxNode {
    fn from(n: MAssignmentExpression) -> Self {
        n.syntax
    }
}
impl From<MAssignmentExpression> for SyntaxElement {
    fn from(n: MAssignmentExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MBinaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BINARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BINARY_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MBinaryExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("MBinaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MBinaryExpression> for SyntaxNode {
    fn from(n: MBinaryExpression) -> Self {
        n.syntax
    }
}
impl From<MBinaryExpression> for SyntaxElement {
    fn from(n: MBinaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MBlockStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BLOCK_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BLOCK_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MBlockStatement")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("statements", &self.statements())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("MBlockStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MBlockStatement> for SyntaxNode {
    fn from(n: MBlockStatement) -> Self {
        n.syntax
    }
}
impl From<MBlockStatement> for SyntaxElement {
    fn from(n: MBlockStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MBooleanLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOOLEAN_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOOLEAN_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MBooleanLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MBooleanLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MBooleanLiteralExpression> for SyntaxNode {
    fn from(n: MBooleanLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MBooleanLiteralExpression> for SyntaxElement {
    fn from(n: MBooleanLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MBreakStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BREAK_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BREAK_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBreakStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MBreakStatement")
                .field(
                    "break_token",
                    &support::DebugSyntaxResult(self.break_token()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MBreakStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MBreakStatement> for SyntaxNode {
    fn from(n: MBreakStatement) -> Self {
        n.syntax
    }
}
impl From<MBreakStatement> for SyntaxElement {
    fn from(n: MBreakStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MCallArguments {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CALL_ARGUMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CALL_ARGUMENTS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MCallArguments")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("args", &self.args())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MCallArguments").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MCallArguments> for SyntaxNode {
    fn from(n: MCallArguments) -> Self {
        n.syntax
    }
}
impl From<MCallArguments> for SyntaxElement {
    fn from(n: MCallArguments) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MCallExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CALL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CALL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MCallExpression")
                .field("callee", &support::DebugSyntaxResult(self.callee()))
                .field("arguments", &support::DebugSyntaxResult(self.arguments()))
                .finish()
        } else {
            f.debug_struct("MCallExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MCallExpression> for SyntaxNode {
    fn from(n: MCallExpression) -> Self {
        n.syntax
    }
}
impl From<MCallExpression> for SyntaxElement {
    fn from(n: MCallExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MCaseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CASE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CASE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MCaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MCaseClause")
                .field("case_token", &support::DebugSyntaxResult(self.case_token()))
                .field("test", &support::DebugSyntaxResult(self.test()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("consequent", &self.consequent())
                .finish()
        } else {
            f.debug_struct("MCaseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MCaseClause> for SyntaxNode {
    fn from(n: MCaseClause) -> Self {
        n.syntax
    }
}
impl From<MCaseClause> for SyntaxElement {
    fn from(n: MCaseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MCatchClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CATCH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CATCH_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MCatchClause")
                .field(
                    "catch_token",
                    &support::DebugSyntaxResult(self.catch_token()),
                )
                .field(
                    "declaration",
                    &support::DebugOptionalElement(self.declaration()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MCatchClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MCatchClause> for SyntaxNode {
    fn from(n: MCatchClause) -> Self {
        n.syntax
    }
}
impl From<MCatchClause> for SyntaxElement {
    fn from(n: MCatchClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MCatchDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CATCH_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CATCH_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MCatchDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MCatchDeclaration")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("binding", &support::DebugSyntaxResult(self.binding()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MCatchDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MCatchDeclaration> for SyntaxNode {
    fn from(n: MCatchDeclaration) -> Self {
        n.syntax
    }
}
impl From<MCatchDeclaration> for SyntaxElement {
    fn from(n: MCatchDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MClassDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CLASS_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CLASS_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MClassDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MClassDeclaration")
                .field("annotation", &self.annotation())
                .field(
                    "class_token",
                    &support::DebugSyntaxResult(self.class_token()),
                )
                .field("id", &support::DebugSyntaxResult(self.id()))
                .field(
                    "extends_clause",
                    &support::DebugOptionalElement(self.extends_clause()),
                )
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("MClassDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MClassDeclaration> for SyntaxNode {
    fn from(n: MClassDeclaration) -> Self {
        n.syntax
    }
}
impl From<MClassDeclaration> for SyntaxElement {
    fn from(n: MClassDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MClassMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CLASS_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CLASS_MEMBER_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MClassMemberName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MClassMemberName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MClassMemberName> for SyntaxNode {
    fn from(n: MClassMemberName) -> Self {
        n.syntax
    }
}
impl From<MClassMemberName> for SyntaxElement {
    fn from(n: MClassMemberName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MComputedMemberAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_COMPUTED_MEMBER_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_COMPUTED_MEMBER_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MComputedMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MComputedMemberAssignment")
                .field("object", &support::DebugOptionalElement(self.object()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("member", &support::DebugSyntaxResult(self.member()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MComputedMemberAssignment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MComputedMemberAssignment> for SyntaxNode {
    fn from(n: MComputedMemberAssignment) -> Self {
        n.syntax
    }
}
impl From<MComputedMemberAssignment> for SyntaxElement {
    fn from(n: MComputedMemberAssignment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MComputedMemberExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_COMPUTED_MEMBER_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_COMPUTED_MEMBER_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MComputedMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MComputedMemberExpression")
                .field("object", &support::DebugOptionalElement(self.object()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("member", &support::DebugSyntaxResult(self.member()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MComputedMemberExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MComputedMemberExpression> for SyntaxNode {
    fn from(n: MComputedMemberExpression) -> Self {
        n.syntax
    }
}
impl From<MComputedMemberExpression> for SyntaxElement {
    fn from(n: MComputedMemberExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MComputedMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_COMPUTED_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_COMPUTED_MEMBER_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MComputedMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MComputedMemberName")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("MComputedMemberName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MComputedMemberName> for SyntaxNode {
    fn from(n: MComputedMemberName) -> Self {
        n.syntax
    }
}
impl From<MComputedMemberName> for SyntaxElement {
    fn from(n: MComputedMemberName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MConditionalExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONDITIONAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONDITIONAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MConditionalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MConditionalExpression")
                .field("test", &support::DebugSyntaxResult(self.test()))
                .field(
                    "question_mark_token",
                    &support::DebugSyntaxResult(self.question_mark_token()),
                )
                .field("consequent", &support::DebugSyntaxResult(self.consequent()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("alternate", &support::DebugSyntaxResult(self.alternate()))
                .finish()
        } else {
            f.debug_struct("MConditionalExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MConditionalExpression> for SyntaxNode {
    fn from(n: MConditionalExpression) -> Self {
        n.syntax
    }
}
impl From<MConditionalExpression> for SyntaxElement {
    fn from(n: MConditionalExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MConstantExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONSTANT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONSTANT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MConstantExpression")
                .field("token", &support::DebugSyntaxResult(self.token()))
                .field("constant", &support::DebugSyntaxResult(self.constant()))
                .finish()
        } else {
            f.debug_struct("MConstantExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MConstantExpression> for SyntaxNode {
    fn from(n: MConstantExpression) -> Self {
        n.syntax
    }
}
impl From<MConstantExpression> for SyntaxElement {
    fn from(n: MConstantExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MConstructorClassMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONSTRUCTOR_CLASS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONSTRUCTOR_CLASS_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MConstructorClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MConstructorClassMember")
                .field("annotation", &self.annotation())
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("parameters", &support::DebugSyntaxResult(self.parameters()))
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MConstructorClassMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MConstructorClassMember> for SyntaxNode {
    fn from(n: MConstructorClassMember) -> Self {
        n.syntax
    }
}
impl From<MConstructorClassMember> for SyntaxElement {
    fn from(n: MConstructorClassMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MConstructorParameters {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONSTRUCTOR_PARAMETERS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONSTRUCTOR_PARAMETERS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MConstructorParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MConstructorParameters")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("parameters", &self.parameters())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MConstructorParameters").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MConstructorParameters> for SyntaxNode {
    fn from(n: MConstructorParameters) -> Self {
        n.syntax
    }
}
impl From<MConstructorParameters> for SyntaxElement {
    fn from(n: MConstructorParameters) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MContinueStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONTINUE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONTINUE_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MContinueStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MContinueStatement")
                .field(
                    "continue_token",
                    &support::DebugSyntaxResult(self.continue_token()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MContinueStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MContinueStatement> for SyntaxNode {
    fn from(n: MContinueStatement) -> Self {
        n.syntax
    }
}
impl From<MContinueStatement> for SyntaxElement {
    fn from(n: MContinueStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MDateLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_DATE_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_DATE_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MDateLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MDateLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MDateLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MDateLiteralExpression> for SyntaxNode {
    fn from(n: MDateLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MDateLiteralExpression> for SyntaxElement {
    fn from(n: MDateLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MDebugStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_DEBUG_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_DEBUG_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MDebugStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MDebugStatement")
                .field(
                    "debug_token",
                    &support::DebugSyntaxResult(self.debug_token()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MDebugStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MDebugStatement> for SyntaxNode {
    fn from(n: MDebugStatement) -> Self {
        n.syntax
    }
}
impl From<MDebugStatement> for SyntaxElement {
    fn from(n: MDebugStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MDefaultClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_DEFAULT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_DEFAULT_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MDefaultClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("consequent", &self.consequent())
                .finish()
        } else {
            f.debug_struct("MDefaultClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MDefaultClause> for SyntaxNode {
    fn from(n: MDefaultClause) -> Self {
        n.syntax
    }
}
impl From<MDefaultClause> for SyntaxElement {
    fn from(n: MDefaultClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MDirective {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_DIRECTIVE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_DIRECTIVE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MDirective")
                .field(
                    "version_token",
                    &support::DebugSyntaxResult(self.version_token()),
                )
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MDirective").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MDirective> for SyntaxNode {
    fn from(n: MDirective) -> Self {
        n.syntax
    }
}
impl From<MDirective> for SyntaxElement {
    fn from(n: MDirective) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ELSE_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MElseClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("alternate", &support::DebugSyntaxResult(self.alternate()))
                .finish()
        } else {
            f.debug_struct("MElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MElseClause> for SyntaxNode {
    fn from(n: MElseClause) -> Self {
        n.syntax
    }
}
impl From<MElseClause> for SyntaxElement {
    fn from(n: MElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MEmptyStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_EMPTY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_EMPTY_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MEmptyStatement")
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MEmptyStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MEmptyStatement> for SyntaxNode {
    fn from(n: MEmptyStatement) -> Self {
        n.syntax
    }
}
impl From<MEmptyStatement> for SyntaxElement {
    fn from(n: MEmptyStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MExpressionSnipped {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_EXPRESSION_SNIPPED as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_EXPRESSION_SNIPPED
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MExpressionSnipped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MExpressionSnipped")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("MExpressionSnipped").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MExpressionSnipped> for SyntaxNode {
    fn from(n: MExpressionSnipped) -> Self {
        n.syntax
    }
}
impl From<MExpressionSnipped> for SyntaxElement {
    fn from(n: MExpressionSnipped) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MExpressionStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_EXPRESSION_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_EXPRESSION_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MExpressionStatement")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MExpressionStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MExpressionStatement> for SyntaxNode {
    fn from(n: MExpressionStatement) -> Self {
        n.syntax
    }
}
impl From<MExpressionStatement> for SyntaxElement {
    fn from(n: MExpressionStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MExtendedBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_EXTENDED_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_EXTENDED_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MExtendedBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MExtendedBinding")
                .field("object", &support::DebugSyntaxResult(self.object()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("member", &support::DebugSyntaxResult(self.member()))
                .finish()
        } else {
            f.debug_struct("MExtendedBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MExtendedBinding> for SyntaxNode {
    fn from(n: MExtendedBinding) -> Self {
        n.syntax
    }
}
impl From<MExtendedBinding> for SyntaxElement {
    fn from(n: MExtendedBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MExtendsClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_EXTENDS_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_EXTENDS_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MExtendsClause")
                .field(
                    "extends_token",
                    &support::DebugSyntaxResult(self.extends_token()),
                )
                .field(
                    "super_class",
                    &support::DebugSyntaxResult(self.super_class()),
                )
                .finish()
        } else {
            f.debug_struct("MExtendsClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MExtendsClause> for SyntaxNode {
    fn from(n: MExtendsClause) -> Self {
        n.syntax
    }
}
impl From<MExtendsClause> for SyntaxElement {
    fn from(n: MExtendsClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MFinallyClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FINALLY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FINALLY_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MFinallyClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MFinallyClause")
                .field(
                    "finally_token",
                    &support::DebugSyntaxResult(self.finally_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MFinallyClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MFinallyClause> for SyntaxNode {
    fn from(n: MFinallyClause) -> Self {
        n.syntax
    }
}
impl From<MFinallyClause> for SyntaxElement {
    fn from(n: MFinallyClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MForAllInStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FOR_ALL_IN_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FOR_ALL_IN_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MForAllInStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MForAllInStatement")
                .field(
                    "forall_token",
                    &support::DebugSyntaxResult(self.forall_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "initializer",
                    &support::DebugSyntaxResult(self.initializer()),
                )
                .field("in_token", &support::DebugSyntaxResult(self.in_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MForAllInStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MForAllInStatement> for SyntaxNode {
    fn from(n: MForAllInStatement) -> Self {
        n.syntax
    }
}
impl From<MForAllInStatement> for SyntaxElement {
    fn from(n: MForAllInStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MForAllStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FOR_ALL_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FOR_ALL_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MForAllStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MForAllStatement")
                .field(
                    "forall_token",
                    &support::DebugSyntaxResult(self.forall_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("iter", &support::DebugSyntaxResult(self.iter()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MForAllStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MForAllStatement> for SyntaxNode {
    fn from(n: MForAllStatement) -> Self {
        n.syntax
    }
}
impl From<MForAllStatement> for SyntaxElement {
    fn from(n: MForAllStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MForIteratorFactory {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FOR_ITERATOR_FACTORY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FOR_ITERATOR_FACTORY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MForIteratorFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MForIteratorFactory")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "comma_token",
                    &support::DebugSyntaxResult(self.comma_token()),
                )
                .field(
                    "initializer",
                    &support::DebugSyntaxResult(self.initializer()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MForIteratorFactory").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MForIteratorFactory> for SyntaxNode {
    fn from(n: MForIteratorFactory) -> Self {
        n.syntax
    }
}
impl From<MForIteratorFactory> for SyntaxElement {
    fn from(n: MForIteratorFactory) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MForStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FOR_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FOR_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MForStatement")
                .field("for_token", &support::DebugSyntaxResult(self.for_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .field(
                    "first_semi_token",
                    &support::DebugSyntaxResult(self.first_semi_token()),
                )
                .field("test", &support::DebugOptionalElement(self.test()))
                .field(
                    "second_semi_token",
                    &support::DebugSyntaxResult(self.second_semi_token()),
                )
                .field("update", &support::DebugOptionalElement(self.update()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MForStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MForStatement> for SyntaxNode {
    fn from(n: MForStatement) -> Self {
        n.syntax
    }
}
impl From<MForStatement> for SyntaxElement {
    fn from(n: MForStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MForVariableDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FOR_VARIABLE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FOR_VARIABLE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MForVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MForVariableDeclaration")
                .field(
                    "var_token",
                    &support::DebugOptionalElement(self.var_token()),
                )
                .field("declarator", &support::DebugSyntaxResult(self.declarator()))
                .finish()
        } else {
            f.debug_struct("MForVariableDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MForVariableDeclaration> for SyntaxNode {
    fn from(n: MForVariableDeclaration) -> Self {
        n.syntax
    }
}
impl From<MForVariableDeclaration> for SyntaxElement {
    fn from(n: MForVariableDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MFormalParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FORMAL_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FORMAL_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MFormalParameter")
                .field("binding", &support::DebugSyntaxResult(self.binding()))
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("MFormalParameter").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MFormalParameter> for SyntaxNode {
    fn from(n: MFormalParameter) -> Self {
        n.syntax
    }
}
impl From<MFormalParameter> for SyntaxElement {
    fn from(n: MFormalParameter) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MFunctionBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FUNCTION_BODY as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FUNCTION_BODY
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MFunctionBody")
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("directives", &self.directives())
                .field("statements", &self.statements())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("MFunctionBody").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MFunctionBody> for SyntaxNode {
    fn from(n: MFunctionBody) -> Self {
        n.syntax
    }
}
impl From<MFunctionBody> for SyntaxElement {
    fn from(n: MFunctionBody) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MFunctionDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FUNCTION_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FUNCTION_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MFunctionDeclaration")
                .field("annotation", &self.annotation())
                .field(
                    "inline_token",
                    &support::DebugOptionalElement(self.inline_token()),
                )
                .field(
                    "function_token",
                    &support::DebugSyntaxResult(self.function_token()),
                )
                .field("id", &support::DebugSyntaxResult(self.id()))
                .field("parameters", &support::DebugSyntaxResult(self.parameters()))
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MFunctionDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MFunctionDeclaration> for SyntaxNode {
    fn from(n: MFunctionDeclaration) -> Self {
        n.syntax
    }
}
impl From<MFunctionDeclaration> for SyntaxElement {
    fn from(n: MFunctionDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MFunctionExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_FUNCTION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_FUNCTION_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MFunctionExpression")
                .field(
                    "function_token",
                    &support::DebugSyntaxResult(self.function_token()),
                )
                .field("parameters", &support::DebugSyntaxResult(self.parameters()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MFunctionExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MFunctionExpression> for SyntaxNode {
    fn from(n: MFunctionExpression) -> Self {
        n.syntax
    }
}
impl From<MFunctionExpression> for SyntaxElement {
    fn from(n: MFunctionExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MGetterClassMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_GETTER_CLASS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_GETTER_CLASS_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MGetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MGetterClassMember")
                .field("annotation", &self.annotation())
                .field("get_token", &support::DebugSyntaxResult(self.get_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MGetterClassMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MGetterClassMember> for SyntaxNode {
    fn from(n: MGetterClassMember) -> Self {
        n.syntax
    }
}
impl From<MGetterClassMember> for SyntaxElement {
    fn from(n: MGetterClassMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MHashMapExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_HASH_MAP_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_HASH_MAP_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MHashMapExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MHashMapExpression")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MHashMapExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MHashMapExpression> for SyntaxNode {
    fn from(n: MHashMapExpression) -> Self {
        n.syntax
    }
}
impl From<MHashMapExpression> for SyntaxElement {
    fn from(n: MHashMapExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MHashSetExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_HASH_SET_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_HASH_SET_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MHashSetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MHashSetExpression")
                .field("set_token", &support::DebugSyntaxResult(self.set_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MHashSetExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MHashSetExpression> for SyntaxNode {
    fn from(n: MHashSetExpression) -> Self {
        n.syntax
    }
}
impl From<MHashSetExpression> for SyntaxElement {
    fn from(n: MHashSetExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MIdentifierAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_IDENTIFIER_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_IDENTIFIER_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MIdentifierAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MIdentifierAssignment")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("MIdentifierAssignment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MIdentifierAssignment> for SyntaxNode {
    fn from(n: MIdentifierAssignment) -> Self {
        n.syntax
    }
}
impl From<MIdentifierAssignment> for SyntaxElement {
    fn from(n: MIdentifierAssignment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MIdentifierBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_IDENTIFIER_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_IDENTIFIER_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MIdentifierBinding")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .finish()
        } else {
            f.debug_struct("MIdentifierBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MIdentifierBinding> for SyntaxNode {
    fn from(n: MIdentifierBinding) -> Self {
        n.syntax
    }
}
impl From<MIdentifierBinding> for SyntaxElement {
    fn from(n: MIdentifierBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MIdentifierExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_IDENTIFIER_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_IDENTIFIER_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MIdentifierExpression")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("MIdentifierExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MIdentifierExpression> for SyntaxNode {
    fn from(n: MIdentifierExpression) -> Self {
        n.syntax
    }
}
impl From<MIdentifierExpression> for SyntaxElement {
    fn from(n: MIdentifierExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MIfStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_IF_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_IF_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MIfStatement")
                .field("if_token", &support::DebugSyntaxResult(self.if_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("test", &support::DebugSyntaxResult(self.test()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("consequent", &support::DebugSyntaxResult(self.consequent()))
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .finish()
        } else {
            f.debug_struct("MIfStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MIfStatement> for SyntaxNode {
    fn from(n: MIfStatement) -> Self {
        n.syntax
    }
}
impl From<MIfStatement> for SyntaxElement {
    fn from(n: MIfStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MInExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_IN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_IN_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MInExpression")
                .field("property", &support::DebugSyntaxResult(self.property()))
                .field("in_token", &support::DebugSyntaxResult(self.in_token()))
                .field("object", &support::DebugSyntaxResult(self.object()))
                .finish()
        } else {
            f.debug_struct("MInExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MInExpression> for SyntaxNode {
    fn from(n: MInExpression) -> Self {
        n.syntax
    }
}
impl From<MInExpression> for SyntaxElement {
    fn from(n: MInExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MInitializerClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_INITIALIZER_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_INITIALIZER_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MInitializerClause")
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .finish()
        } else {
            f.debug_struct("MInitializerClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MInitializerClause> for SyntaxNode {
    fn from(n: MInitializerClause) -> Self {
        n.syntax
    }
}
impl From<MInitializerClause> for SyntaxElement {
    fn from(n: MInitializerClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MInstanceofExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_INSTANCEOF_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_INSTANCEOF_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MInstanceofExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MInstanceofExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "instanceof_token",
                    &support::DebugSyntaxResult(self.instanceof_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("MInstanceofExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MInstanceofExpression> for SyntaxNode {
    fn from(n: MInstanceofExpression) -> Self {
        n.syntax
    }
}
impl From<MInstanceofExpression> for SyntaxElement {
    fn from(n: MInstanceofExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MLiteralMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_LITERAL_MEMBER_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_LITERAL_MEMBER_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MLiteralMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MLiteralMemberName")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("MLiteralMemberName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MLiteralMemberName> for SyntaxNode {
    fn from(n: MLiteralMemberName) -> Self {
        n.syntax
    }
}
impl From<MLiteralMemberName> for SyntaxElement {
    fn from(n: MLiteralMemberName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MLogicalExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_LOGICAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_LOGICAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MLogicalExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("MLogicalExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MLogicalExpression> for SyntaxNode {
    fn from(n: MLogicalExpression) -> Self {
        n.syntax
    }
}
impl From<MLogicalExpression> for SyntaxElement {
    fn from(n: MLogicalExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MLongStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_LONG_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_LONG_STRING_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MLongStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MLongStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MLongStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MLongStringLiteralExpression> for SyntaxNode {
    fn from(n: MLongStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MLongStringLiteralExpression> for SyntaxElement {
    fn from(n: MLongStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MMethodClassMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_METHOD_CLASS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_METHOD_CLASS_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MMethodClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MMethodClassMember")
                .field("annotation", &self.annotation())
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("parameters", &support::DebugSyntaxResult(self.parameters()))
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MMethodClassMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MMethodClassMember> for SyntaxNode {
    fn from(n: MMethodClassMember) -> Self {
        n.syntax
    }
}
impl From<MMethodClassMember> for SyntaxElement {
    fn from(n: MMethodClassMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MModule {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_MODULE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_MODULE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MModule")
                .field("directives", &self.directives())
                .field("items", &self.items())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("MModule").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MModule> for SyntaxNode {
    fn from(n: MModule) -> Self {
        n.syntax
    }
}
impl From<MModule> for SyntaxElement {
    fn from(n: MModule) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SyntaxKindSet::from_raw(RawSyntaxKind(M_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MName> for SyntaxNode {
    fn from(n: MName) -> Self {
        n.syntax
    }
}
impl From<MName> for SyntaxElement {
    fn from(n: MName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MNewExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_NEW_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_NEW_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MNewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MNewExpression")
                .field("new_token", &support::DebugSyntaxResult(self.new_token()))
                .field("callee", &support::DebugSyntaxResult(self.callee()))
                .field(
                    "arguments",
                    &support::DebugOptionalElement(self.arguments()),
                )
                .finish()
        } else {
            f.debug_struct("MNewExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MNewExpression> for SyntaxNode {
    fn from(n: MNewExpression) -> Self {
        n.syntax
    }
}
impl From<MNewExpression> for SyntaxElement {
    fn from(n: MNewExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MNullLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_NULL_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_NULL_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MNullLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MNullLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MNullLiteralExpression> for SyntaxNode {
    fn from(n: MNullLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MNullLiteralExpression> for SyntaxElement {
    fn from(n: MNullLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MNumberLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_NUMBER_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_NUMBER_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MNumberLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MNumberLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MNumberLiteralExpression> for SyntaxNode {
    fn from(n: MNumberLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MNumberLiteralExpression> for SyntaxElement {
    fn from(n: MNumberLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MObjectExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_OBJECT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_OBJECT_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MObjectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MObjectExpression")
                .field("at_token", &support::DebugSyntaxResult(self.at_token()))
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("members", &self.members())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("MObjectExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MObjectExpression> for SyntaxNode {
    fn from(n: MObjectExpression) -> Self {
        n.syntax
    }
}
impl From<MObjectExpression> for SyntaxElement {
    fn from(n: MObjectExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MParameters {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PARAMETERS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PARAMETERS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MParameters")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("items", &self.items())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MParameters").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MParameters> for SyntaxNode {
    fn from(n: MParameters) -> Self {
        n.syntax
    }
}
impl From<MParameters> for SyntaxElement {
    fn from(n: MParameters) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MParenthesizedAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PARENTHESIZED_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PARENTHESIZED_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MParenthesizedAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MParenthesizedAssignment")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("assignment", &support::DebugSyntaxResult(self.assignment()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MParenthesizedAssignment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MParenthesizedAssignment> for SyntaxNode {
    fn from(n: MParenthesizedAssignment) -> Self {
        n.syntax
    }
}
impl From<MParenthesizedAssignment> for SyntaxElement {
    fn from(n: MParenthesizedAssignment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MParenthesizedExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PARENTHESIZED_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PARENTHESIZED_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MParenthesizedExpression")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("MParenthesizedExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MParenthesizedExpression> for SyntaxNode {
    fn from(n: MParenthesizedExpression) -> Self {
        n.syntax
    }
}
impl From<MParenthesizedExpression> for SyntaxElement {
    fn from(n: MParenthesizedExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MPostUpdateExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_POST_UPDATE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_POST_UPDATE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MPostUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MPostUpdateExpression")
                .field("operand", &support::DebugSyntaxResult(self.operand()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .finish()
        } else {
            f.debug_struct("MPostUpdateExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MPostUpdateExpression> for SyntaxNode {
    fn from(n: MPostUpdateExpression) -> Self {
        n.syntax
    }
}
impl From<MPostUpdateExpression> for SyntaxElement {
    fn from(n: MPostUpdateExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MPreUpdateExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PRE_UPDATE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PRE_UPDATE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MPreUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MPreUpdateExpression")
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("operand", &support::DebugSyntaxResult(self.operand()))
                .finish()
        } else {
            f.debug_struct("MPreUpdateExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MPreUpdateExpression> for SyntaxNode {
    fn from(n: MPreUpdateExpression) -> Self {
        n.syntax
    }
}
impl From<MPreUpdateExpression> for SyntaxElement {
    fn from(n: MPreUpdateExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MPropertyObjectMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PROPERTY_OBJECT_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PROPERTY_OBJECT_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MPropertyObjectMember")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("MPropertyObjectMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MPropertyObjectMember> for SyntaxNode {
    fn from(n: MPropertyObjectMember) -> Self {
        n.syntax
    }
}
impl From<MPropertyObjectMember> for SyntaxElement {
    fn from(n: MPropertyObjectMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReferenceIdentifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REFERENCE_IDENTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REFERENCE_IDENTIFIER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReferenceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReferenceIdentifier")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MReferenceIdentifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReferenceIdentifier> for SyntaxNode {
    fn from(n: MReferenceIdentifier) -> Self {
        n.syntax
    }
}
impl From<MReferenceIdentifier> for SyntaxElement {
    fn from(n: MReferenceIdentifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReport {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReport")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("init", &self.init())
                .field("default", &support::DebugOptionalElement(self.default()))
                .field("sections", &self.sections())
                .finish()
        } else {
            f.debug_struct("MReport").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReport> for SyntaxNode {
    fn from(n: MReport) -> Self {
        n.syntax
    }
}
impl From<MReport> for SyntaxElement {
    fn from(n: MReport) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReportFile {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_FILE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_FILE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReportFile")
                .field("reports", &self.reports())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("MReportFile").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReportFile> for SyntaxNode {
    fn from(n: MReportFile) -> Self {
        n.syntax
    }
}
impl From<MReportFile> for SyntaxElement {
    fn from(n: MReportFile) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReportName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReportName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReportName")
                .field("ff2_token", &support::DebugSyntaxResult(self.ff2_token()))
                .field("m_name", &support::DebugSyntaxResult(self.m_name()))
                .finish()
        } else {
            f.debug_struct("MReportName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReportName> for SyntaxNode {
    fn from(n: MReportName) -> Self {
        n.syntax
    }
}
impl From<MReportName> for SyntaxElement {
    fn from(n: MReportName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReportSection {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_SECTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_SECTION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReportSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReportSection")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MReportSection").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReportSection> for SyntaxNode {
    fn from(n: MReportSection) -> Self {
        n.syntax
    }
}
impl From<MReportSection> for SyntaxElement {
    fn from(n: MReportSection) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReportSectionName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_SECTION_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_SECTION_NAME
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReportSectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReportSectionName")
                .field("ff_token", &support::DebugSyntaxResult(self.ff_token()))
                .field("m_name", &support::DebugSyntaxResult(self.m_name()))
                .finish()
        } else {
            f.debug_struct("MReportSectionName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReportSectionName> for SyntaxNode {
    fn from(n: MReportSectionName) -> Self {
        n.syntax
    }
}
impl From<MReportSectionName> for SyntaxElement {
    fn from(n: MReportSectionName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MRestParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REST_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REST_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MRestParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MRestParameter")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("binding", &support::DebugOptionalElement(self.binding()))
                .finish()
        } else {
            f.debug_struct("MRestParameter").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MRestParameter> for SyntaxNode {
    fn from(n: MRestParameter) -> Self {
        n.syntax
    }
}
impl From<MRestParameter> for SyntaxElement {
    fn from(n: MRestParameter) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MReturnStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_RETURN_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_RETURN_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MReturnStatement")
                .field(
                    "return_token",
                    &support::DebugSyntaxResult(self.return_token()),
                )
                .field("argument", &support::DebugOptionalElement(self.argument()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MReturnStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MReturnStatement> for SyntaxNode {
    fn from(n: MReturnStatement) -> Self {
        n.syntax
    }
}
impl From<MReturnStatement> for SyntaxElement {
    fn from(n: MReturnStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MScript {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SCRIPT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SCRIPT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MScript")
                .field("statements", &self.statements())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("MScript").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MScript> for SyntaxNode {
    fn from(n: MScript) -> Self {
        n.syntax
    }
}
impl From<MScript> for SyntaxElement {
    fn from(n: MScript) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSequenceExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SEQUENCE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SEQUENCE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSequenceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSequenceExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "comma_token",
                    &support::DebugSyntaxResult(self.comma_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("MSequenceExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSequenceExpression> for SyntaxNode {
    fn from(n: MSequenceExpression) -> Self {
        n.syntax
    }
}
impl From<MSequenceExpression> for SyntaxElement {
    fn from(n: MSequenceExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSetterClassMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SETTER_CLASS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SETTER_CLASS_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSetterClassMember")
                .field("annotation", &self.annotation())
                .field("set_token", &support::DebugSyntaxResult(self.set_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("parameter", &support::DebugSyntaxResult(self.parameter()))
                .field(
                    "comma_token",
                    &support::DebugOptionalElement(self.comma_token()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "doc_string",
                    &support::DebugOptionalElement(self.doc_string()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MSetterClassMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSetterClassMember> for SyntaxNode {
    fn from(n: MSetterClassMember) -> Self {
        n.syntax
    }
}
impl From<MSetterClassMember> for SyntaxElement {
    fn from(n: MSetterClassMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MShorthandPropertyObjectMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SHORTHAND_PROPERTY_OBJECT_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SHORTHAND_PROPERTY_OBJECT_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MShorthandPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MShorthandPropertyObjectMember")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("MShorthandPropertyObjectMember").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MShorthandPropertyObjectMember> for SyntaxNode {
    fn from(n: MShorthandPropertyObjectMember) -> Self {
        n.syntax
    }
}
impl From<MShorthandPropertyObjectMember> for SyntaxElement {
    fn from(n: MShorthandPropertyObjectMember) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSpread {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SPREAD as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SPREAD
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSpread")
                .field(
                    "dotdotdot_token",
                    &support::DebugSyntaxResult(self.dotdotdot_token()),
                )
                .field("argument", &support::DebugSyntaxResult(self.argument()))
                .finish()
        } else {
            f.debug_struct("MSpread").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSpread> for SyntaxNode {
    fn from(n: MSpread) -> Self {
        n.syntax
    }
}
impl From<MSpread> for SyntaxElement {
    fn from(n: MSpread) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSqlConcatenationExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SQL_CONCATENATION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SQL_CONCATENATION_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSqlConcatenationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSqlConcatenationExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .finish()
        } else {
            f.debug_struct("MSqlConcatenationExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSqlConcatenationExpression> for SyntaxNode {
    fn from(n: MSqlConcatenationExpression) -> Self {
        n.syntax
    }
}
impl From<MSqlConcatenationExpression> for SyntaxElement {
    fn from(n: MSqlConcatenationExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSqlLongStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SQL_LONG_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SQL_LONG_STRING_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSqlLongStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSqlLongStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MSqlLongStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSqlLongStringLiteralExpression> for SyntaxNode {
    fn from(n: MSqlLongStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MSqlLongStringLiteralExpression> for SyntaxElement {
    fn from(n: MSqlLongStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSqlStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SQL_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SQL_STRING_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSqlStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MSqlStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSqlStringLiteralExpression> for SyntaxNode {
    fn from(n: MSqlStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MSqlStringLiteralExpression> for SyntaxElement {
    fn from(n: MSqlStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MStaticMemberAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_STATIC_MEMBER_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_STATIC_MEMBER_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MStaticMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MStaticMemberAssignment")
                .field("object", &support::DebugSyntaxResult(self.object()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("member", &support::DebugSyntaxResult(self.member()))
                .finish()
        } else {
            f.debug_struct("MStaticMemberAssignment").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MStaticMemberAssignment> for SyntaxNode {
    fn from(n: MStaticMemberAssignment) -> Self {
        n.syntax
    }
}
impl From<MStaticMemberAssignment> for SyntaxElement {
    fn from(n: MStaticMemberAssignment) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MStaticMemberExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_STATIC_MEMBER_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_STATIC_MEMBER_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MStaticMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MStaticMemberExpression")
                .field("object", &support::DebugSyntaxResult(self.object()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("member", &support::DebugSyntaxResult(self.member()))
                .finish()
        } else {
            f.debug_struct("MStaticMemberExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MStaticMemberExpression> for SyntaxNode {
    fn from(n: MStaticMemberExpression) -> Self {
        n.syntax
    }
}
impl From<MStaticMemberExpression> for SyntaxElement {
    fn from(n: MStaticMemberExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_STRING_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MStringLiteralExpression> for SyntaxNode {
    fn from(n: MStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MStringLiteralExpression> for SyntaxElement {
    fn from(n: MStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSuperExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SUPER_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SUPER_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSuperExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSuperExpression")
                .field(
                    "super_token",
                    &support::DebugSyntaxResult(self.super_token()),
                )
                .finish()
        } else {
            f.debug_struct("MSuperExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSuperExpression> for SyntaxNode {
    fn from(n: MSuperExpression) -> Self {
        n.syntax
    }
}
impl From<MSuperExpression> for SyntaxElement {
    fn from(n: MSuperExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MSwitchStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SWITCH_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SWITCH_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MSwitchStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MSwitchStatement")
                .field(
                    "switch_token",
                    &support::DebugSyntaxResult(self.switch_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "discriminant",
                    &support::DebugSyntaxResult(self.discriminant()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "l_curly_token",
                    &support::DebugSyntaxResult(self.l_curly_token()),
                )
                .field("cases", &self.cases())
                .field(
                    "r_curly_token",
                    &support::DebugSyntaxResult(self.r_curly_token()),
                )
                .finish()
        } else {
            f.debug_struct("MSwitchStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MSwitchStatement> for SyntaxNode {
    fn from(n: MSwitchStatement) -> Self {
        n.syntax
    }
}
impl From<MSwitchStatement> for SyntaxElement {
    fn from(n: MSwitchStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MTemplateExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_TEMPLATE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_TEMPLATE_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MTemplateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MTemplateExpression")
                .field("token", &support::DebugSyntaxResult(self.token()))
                .field("template", &support::DebugSyntaxResult(self.template()))
                .finish()
        } else {
            f.debug_struct("MTemplateExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MTemplateExpression> for SyntaxNode {
    fn from(n: MTemplateExpression) -> Self {
        n.syntax
    }
}
impl From<MTemplateExpression> for SyntaxElement {
    fn from(n: MTemplateExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MThisExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_THIS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_THIS_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MThisExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MThisExpression")
                .field("this_token", &support::DebugSyntaxResult(self.this_token()))
                .finish()
        } else {
            f.debug_struct("MThisExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MThisExpression> for SyntaxNode {
    fn from(n: MThisExpression) -> Self {
        n.syntax
    }
}
impl From<MThisExpression> for SyntaxElement {
    fn from(n: MThisExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MThrowStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_THROW_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_THROW_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MThrowStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MThrowStatement")
                .field(
                    "throw_token",
                    &support::DebugSyntaxResult(self.throw_token()),
                )
                .field("argument", &support::DebugSyntaxResult(self.argument()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MThrowStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MThrowStatement> for SyntaxNode {
    fn from(n: MThrowStatement) -> Self {
        n.syntax
    }
}
impl From<MThrowStatement> for SyntaxElement {
    fn from(n: MThrowStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MTimeLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_TIME_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_TIME_LITERAL_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MTimeLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MTimeLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("MTimeLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MTimeLiteralExpression> for SyntaxNode {
    fn from(n: MTimeLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<MTimeLiteralExpression> for SyntaxElement {
    fn from(n: MTimeLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MTryFinallyStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_TRY_FINALLY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_TRY_FINALLY_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MTryFinallyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MTryFinallyStatement")
                .field("try_token", &support::DebugSyntaxResult(self.try_token()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .field(
                    "catch_clause",
                    &support::DebugOptionalElement(self.catch_clause()),
                )
                .field(
                    "finally_clause",
                    &support::DebugSyntaxResult(self.finally_clause()),
                )
                .finish()
        } else {
            f.debug_struct("MTryFinallyStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MTryFinallyStatement> for SyntaxNode {
    fn from(n: MTryFinallyStatement) -> Self {
        n.syntax
    }
}
impl From<MTryFinallyStatement> for SyntaxElement {
    fn from(n: MTryFinallyStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MTryStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_TRY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_TRY_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MTryStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MTryStatement")
                .field("try_token", &support::DebugSyntaxResult(self.try_token()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .field(
                    "catch_clause",
                    &support::DebugSyntaxResult(self.catch_clause()),
                )
                .finish()
        } else {
            f.debug_struct("MTryStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MTryStatement> for SyntaxNode {
    fn from(n: MTryStatement) -> Self {
        n.syntax
    }
}
impl From<MTryStatement> for SyntaxElement {
    fn from(n: MTryStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MUnaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_UNARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_UNARY_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MUnaryExpression")
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("argument", &support::DebugSyntaxResult(self.argument()))
                .finish()
        } else {
            f.debug_struct("MUnaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MUnaryExpression> for SyntaxNode {
    fn from(n: MUnaryExpression) -> Self {
        n.syntax
    }
}
impl From<MUnaryExpression> for SyntaxElement {
    fn from(n: MUnaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MVariableDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_VARIABLE_DECLARATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_VARIABLE_DECLARATION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MVariableDeclaration")
                .field("kind_token", &support::DebugSyntaxResult(self.kind_token()))
                .field("declarators", &self.declarators())
                .finish()
        } else {
            f.debug_struct("MVariableDeclaration").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MVariableDeclaration> for SyntaxNode {
    fn from(n: MVariableDeclaration) -> Self {
        n.syntax
    }
}
impl From<MVariableDeclaration> for SyntaxElement {
    fn from(n: MVariableDeclaration) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MVariableDeclarationClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_VARIABLE_DECLARATION_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_VARIABLE_DECLARATION_CLAUSE
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MVariableDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MVariableDeclarationClause")
                .field(
                    "declaration",
                    &support::DebugSyntaxResult(self.declaration()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MVariableDeclarationClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MVariableDeclarationClause> for SyntaxNode {
    fn from(n: MVariableDeclarationClause) -> Self {
        n.syntax
    }
}
impl From<MVariableDeclarationClause> for SyntaxElement {
    fn from(n: MVariableDeclarationClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MVariableDeclarator {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_VARIABLE_DECLARATOR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_VARIABLE_DECLARATOR
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MVariableDeclarator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MVariableDeclarator")
                .field("id", &support::DebugSyntaxResult(self.id()))
                .field(
                    "initializer",
                    &support::DebugOptionalElement(self.initializer()),
                )
                .finish()
        } else {
            f.debug_struct("MVariableDeclarator").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MVariableDeclarator> for SyntaxNode {
    fn from(n: MVariableDeclarator) -> Self {
        n.syntax
    }
}
impl From<MVariableDeclarator> for SyntaxElement {
    fn from(n: MVariableDeclarator) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MVariableStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_VARIABLE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_VARIABLE_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MVariableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MVariableStatement")
                .field(
                    "declaration",
                    &support::DebugSyntaxResult(self.declaration()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("MVariableStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MVariableStatement> for SyntaxNode {
    fn from(n: MVariableStatement) -> Self {
        n.syntax
    }
}
impl From<MVariableStatement> for SyntaxElement {
    fn from(n: MVariableStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for MWhileStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_WHILE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_WHILE_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("MWhileStatement")
                .field(
                    "while_token",
                    &support::DebugSyntaxResult(self.while_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("test", &support::DebugSyntaxResult(self.test()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("body", &support::DebugSyntaxResult(self.body()))
                .finish()
        } else {
            f.debug_struct("MWhileStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<MWhileStatement> for SyntaxNode {
    fn from(n: MWhileStatement) -> Self {
        n.syntax
    }
}
impl From<MWhileStatement> for SyntaxElement {
    fn from(n: MWhileStatement) -> Self {
        n.syntax.into()
    }
}
impl From<MAnnotationBinding> for AnyMAnnotationElement {
    fn from(node: MAnnotationBinding) -> Self {
        Self::MAnnotationBinding(node)
    }
}
impl From<MAnnotationElement> for AnyMAnnotationElement {
    fn from(node: MAnnotationElement) -> Self {
        Self::MAnnotationElement(node)
    }
}
impl AstNode for AnyMAnnotationElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MAnnotationBinding::KIND_SET.union(MAnnotationElement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_ANNOTATION_BINDING | M_ANNOTATION_ELEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_ANNOTATION_BINDING => Self::MAnnotationBinding(MAnnotationBinding { syntax }),
            M_ANNOTATION_ELEMENT => Self::MAnnotationElement(MAnnotationElement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MAnnotationBinding(it) => &it.syntax,
            Self::MAnnotationElement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MAnnotationBinding(it) => it.syntax,
            Self::MAnnotationElement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMAnnotationElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MAnnotationBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::MAnnotationElement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMAnnotationElement> for SyntaxNode {
    fn from(n: AnyMAnnotationElement) -> Self {
        match n {
            AnyMAnnotationElement::MAnnotationBinding(it) => it.into(),
            AnyMAnnotationElement::MAnnotationElement(it) => it.into(),
        }
    }
}
impl From<AnyMAnnotationElement> for SyntaxElement {
    fn from(n: AnyMAnnotationElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MArrayHole> for AnyMArrayElement {
    fn from(node: MArrayHole) -> Self {
        Self::MArrayHole(node)
    }
}
impl From<MSpread> for AnyMArrayElement {
    fn from(node: MSpread) -> Self {
        Self::MSpread(node)
    }
}
impl AstNode for AnyMArrayElement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyMExpression::KIND_SET
        .union(MArrayHole::KIND_SET)
        .union(MSpread::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_ARRAY_HOLE | M_SPREAD => true,
            k if AnyMExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_ARRAY_HOLE => Self::MArrayHole(MArrayHole { syntax }),
            M_SPREAD => Self::MSpread(MSpread { syntax }),
            _ => {
                if let Some(any_m_expression) = AnyMExpression::cast(syntax) {
                    return Some(Self::AnyMExpression(any_m_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MArrayHole(it) => &it.syntax,
            Self::MSpread(it) => &it.syntax,
            Self::AnyMExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MArrayHole(it) => it.syntax,
            Self::MSpread(it) => it.syntax,
            Self::AnyMExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMArrayElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MArrayHole(it) => std::fmt::Debug::fmt(it, f),
            Self::MSpread(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMArrayElement> for SyntaxNode {
    fn from(n: AnyMArrayElement) -> Self {
        match n {
            AnyMArrayElement::AnyMExpression(it) => it.into(),
            AnyMArrayElement::MArrayHole(it) => it.into(),
            AnyMArrayElement::MSpread(it) => it.into(),
        }
    }
}
impl From<AnyMArrayElement> for SyntaxElement {
    fn from(n: AnyMArrayElement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusAssignment> for AnyMAssignment {
    fn from(node: MBogusAssignment) -> Self {
        Self::MBogusAssignment(node)
    }
}
impl From<MComputedMemberAssignment> for AnyMAssignment {
    fn from(node: MComputedMemberAssignment) -> Self {
        Self::MComputedMemberAssignment(node)
    }
}
impl From<MIdentifierAssignment> for AnyMAssignment {
    fn from(node: MIdentifierAssignment) -> Self {
        Self::MIdentifierAssignment(node)
    }
}
impl From<MParenthesizedAssignment> for AnyMAssignment {
    fn from(node: MParenthesizedAssignment) -> Self {
        Self::MParenthesizedAssignment(node)
    }
}
impl From<MStaticMemberAssignment> for AnyMAssignment {
    fn from(node: MStaticMemberAssignment) -> Self {
        Self::MStaticMemberAssignment(node)
    }
}
impl AstNode for AnyMAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBogusAssignment::KIND_SET
        .union(MComputedMemberAssignment::KIND_SET)
        .union(MIdentifierAssignment::KIND_SET)
        .union(MParenthesizedAssignment::KIND_SET)
        .union(MStaticMemberAssignment::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BOGUS_ASSIGNMENT
                | M_COMPUTED_MEMBER_ASSIGNMENT
                | M_IDENTIFIER_ASSIGNMENT
                | M_PARENTHESIZED_ASSIGNMENT
                | M_STATIC_MEMBER_ASSIGNMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_ASSIGNMENT => Self::MBogusAssignment(MBogusAssignment { syntax }),
            M_COMPUTED_MEMBER_ASSIGNMENT => {
                Self::MComputedMemberAssignment(MComputedMemberAssignment { syntax })
            }
            M_IDENTIFIER_ASSIGNMENT => {
                Self::MIdentifierAssignment(MIdentifierAssignment { syntax })
            }
            M_PARENTHESIZED_ASSIGNMENT => {
                Self::MParenthesizedAssignment(MParenthesizedAssignment { syntax })
            }
            M_STATIC_MEMBER_ASSIGNMENT => {
                Self::MStaticMemberAssignment(MStaticMemberAssignment { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusAssignment(it) => &it.syntax,
            Self::MComputedMemberAssignment(it) => &it.syntax,
            Self::MIdentifierAssignment(it) => &it.syntax,
            Self::MParenthesizedAssignment(it) => &it.syntax,
            Self::MStaticMemberAssignment(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusAssignment(it) => it.syntax,
            Self::MComputedMemberAssignment(it) => it.syntax,
            Self::MIdentifierAssignment(it) => it.syntax,
            Self::MParenthesizedAssignment(it) => it.syntax,
            Self::MStaticMemberAssignment(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::MComputedMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::MIdentifierAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::MParenthesizedAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::MStaticMemberAssignment(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMAssignment> for SyntaxNode {
    fn from(n: AnyMAssignment) -> Self {
        match n {
            AnyMAssignment::MBogusAssignment(it) => it.into(),
            AnyMAssignment::MComputedMemberAssignment(it) => it.into(),
            AnyMAssignment::MIdentifierAssignment(it) => it.into(),
            AnyMAssignment::MParenthesizedAssignment(it) => it.into(),
            AnyMAssignment::MStaticMemberAssignment(it) => it.into(),
        }
    }
}
impl From<AnyMAssignment> for SyntaxElement {
    fn from(n: AnyMAssignment) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusBinding> for AnyMBinding {
    fn from(node: MBogusBinding) -> Self {
        Self::MBogusBinding(node)
    }
}
impl From<MIdentifierBinding> for AnyMBinding {
    fn from(node: MIdentifierBinding) -> Self {
        Self::MIdentifierBinding(node)
    }
}
impl AstNode for AnyMBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MBogusBinding::KIND_SET.union(MIdentifierBinding::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_BOGUS_BINDING | M_IDENTIFIER_BINDING)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_BINDING => Self::MBogusBinding(MBogusBinding { syntax }),
            M_IDENTIFIER_BINDING => Self::MIdentifierBinding(MIdentifierBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusBinding(it) => &it.syntax,
            Self::MIdentifierBinding(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusBinding(it) => it.syntax,
            Self::MIdentifierBinding(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::MIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMBinding> for SyntaxNode {
    fn from(n: AnyMBinding) -> Self {
        match n {
            AnyMBinding::MBogusBinding(it) => it.into(),
            AnyMBinding::MIdentifierBinding(it) => it.into(),
        }
    }
}
impl From<AnyMBinding> for SyntaxElement {
    fn from(n: AnyMBinding) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MSpread> for AnyMCallArgument {
    fn from(node: MSpread) -> Self {
        Self::MSpread(node)
    }
}
impl AstNode for AnyMCallArgument {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyMExpression::KIND_SET.union(MSpread::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_SPREAD => true,
            k if AnyMExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_SPREAD => Self::MSpread(MSpread { syntax }),
            _ => {
                if let Some(any_m_expression) = AnyMExpression::cast(syntax) {
                    return Some(Self::AnyMExpression(any_m_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MSpread(it) => &it.syntax,
            Self::AnyMExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MSpread(it) => it.syntax,
            Self::AnyMExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMCallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSpread(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMCallArgument> for SyntaxNode {
    fn from(n: AnyMCallArgument) -> Self {
        match n {
            AnyMCallArgument::AnyMExpression(it) => it.into(),
            AnyMCallArgument::MSpread(it) => it.into(),
        }
    }
}
impl From<AnyMCallArgument> for SyntaxElement {
    fn from(n: AnyMCallArgument) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusMember> for AnyMClassMember {
    fn from(node: MBogusMember) -> Self {
        Self::MBogusMember(node)
    }
}
impl From<MConstructorClassMember> for AnyMClassMember {
    fn from(node: MConstructorClassMember) -> Self {
        Self::MConstructorClassMember(node)
    }
}
impl From<MGetterClassMember> for AnyMClassMember {
    fn from(node: MGetterClassMember) -> Self {
        Self::MGetterClassMember(node)
    }
}
impl From<MMethodClassMember> for AnyMClassMember {
    fn from(node: MMethodClassMember) -> Self {
        Self::MMethodClassMember(node)
    }
}
impl From<MSetterClassMember> for AnyMClassMember {
    fn from(node: MSetterClassMember) -> Self {
        Self::MSetterClassMember(node)
    }
}
impl AstNode for AnyMClassMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBogusMember::KIND_SET
        .union(MConstructorClassMember::KIND_SET)
        .union(MGetterClassMember::KIND_SET)
        .union(MMethodClassMember::KIND_SET)
        .union(MSetterClassMember::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BOGUS_MEMBER
                | M_CONSTRUCTOR_CLASS_MEMBER
                | M_GETTER_CLASS_MEMBER
                | M_METHOD_CLASS_MEMBER
                | M_SETTER_CLASS_MEMBER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_MEMBER => Self::MBogusMember(MBogusMember { syntax }),
            M_CONSTRUCTOR_CLASS_MEMBER => {
                Self::MConstructorClassMember(MConstructorClassMember { syntax })
            }
            M_GETTER_CLASS_MEMBER => Self::MGetterClassMember(MGetterClassMember { syntax }),
            M_METHOD_CLASS_MEMBER => Self::MMethodClassMember(MMethodClassMember { syntax }),
            M_SETTER_CLASS_MEMBER => Self::MSetterClassMember(MSetterClassMember { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusMember(it) => &it.syntax,
            Self::MConstructorClassMember(it) => &it.syntax,
            Self::MGetterClassMember(it) => &it.syntax,
            Self::MMethodClassMember(it) => &it.syntax,
            Self::MSetterClassMember(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusMember(it) => it.syntax,
            Self::MConstructorClassMember(it) => it.syntax,
            Self::MGetterClassMember(it) => it.syntax,
            Self::MMethodClassMember(it) => it.syntax,
            Self::MSetterClassMember(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MConstructorClassMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MGetterClassMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MMethodClassMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MSetterClassMember(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMClassMember> for SyntaxNode {
    fn from(n: AnyMClassMember) -> Self {
        match n {
            AnyMClassMember::MBogusMember(it) => it.into(),
            AnyMClassMember::MConstructorClassMember(it) => it.into(),
            AnyMClassMember::MGetterClassMember(it) => it.into(),
            AnyMClassMember::MMethodClassMember(it) => it.into(),
            AnyMClassMember::MSetterClassMember(it) => it.into(),
        }
    }
}
impl From<AnyMClassMember> for SyntaxElement {
    fn from(n: AnyMClassMember) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MRestParameter> for AnyMConstructorParameter {
    fn from(node: MRestParameter) -> Self {
        Self::MRestParameter(node)
    }
}
impl AstNode for AnyMConstructorParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyMFormalParameter::KIND_SET.union(MRestParameter::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_REST_PARAMETER => true,
            k if AnyMFormalParameter::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_REST_PARAMETER => Self::MRestParameter(MRestParameter { syntax }),
            _ => {
                if let Some(any_m_formal_parameter) = AnyMFormalParameter::cast(syntax) {
                    return Some(Self::AnyMFormalParameter(any_m_formal_parameter));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MRestParameter(it) => &it.syntax,
            Self::AnyMFormalParameter(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MRestParameter(it) => it.syntax,
            Self::AnyMFormalParameter(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMConstructorParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMFormalParameter(it) => std::fmt::Debug::fmt(it, f),
            Self::MRestParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMConstructorParameter> for SyntaxNode {
    fn from(n: AnyMConstructorParameter) -> Self {
        match n {
            AnyMConstructorParameter::AnyMFormalParameter(it) => it.into(),
            AnyMConstructorParameter::MRestParameter(it) => it.into(),
        }
    }
}
impl From<AnyMConstructorParameter> for SyntaxElement {
    fn from(n: AnyMConstructorParameter) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MClassDeclaration> for AnyMDeclaration {
    fn from(node: MClassDeclaration) -> Self {
        Self::MClassDeclaration(node)
    }
}
impl From<MFunctionDeclaration> for AnyMDeclaration {
    fn from(node: MFunctionDeclaration) -> Self {
        Self::MFunctionDeclaration(node)
    }
}
impl From<MVariableDeclaration> for AnyMDeclaration {
    fn from(node: MVariableDeclaration) -> Self {
        Self::MVariableDeclaration(node)
    }
}
impl AstNode for AnyMDeclaration {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MClassDeclaration::KIND_SET
        .union(MFunctionDeclaration::KIND_SET)
        .union(MVariableDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_CLASS_DECLARATION | M_FUNCTION_DECLARATION | M_VARIABLE_DECLARATION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_CLASS_DECLARATION => Self::MClassDeclaration(MClassDeclaration { syntax }),
            M_FUNCTION_DECLARATION => Self::MFunctionDeclaration(MFunctionDeclaration { syntax }),
            M_VARIABLE_DECLARATION => Self::MVariableDeclaration(MVariableDeclaration { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MClassDeclaration(it) => &it.syntax,
            Self::MFunctionDeclaration(it) => &it.syntax,
            Self::MVariableDeclaration(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MClassDeclaration(it) => it.syntax,
            Self::MFunctionDeclaration(it) => it.syntax,
            Self::MVariableDeclaration(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMDeclaration> for SyntaxNode {
    fn from(n: AnyMDeclaration) -> Self {
        match n {
            AnyMDeclaration::MClassDeclaration(it) => it.into(),
            AnyMDeclaration::MFunctionDeclaration(it) => it.into(),
            AnyMDeclaration::MVariableDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyMDeclaration> for SyntaxElement {
    fn from(n: AnyMDeclaration) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MClassDeclaration> for AnyMDeclarationClause {
    fn from(node: MClassDeclaration) -> Self {
        Self::MClassDeclaration(node)
    }
}
impl From<MFunctionDeclaration> for AnyMDeclarationClause {
    fn from(node: MFunctionDeclaration) -> Self {
        Self::MFunctionDeclaration(node)
    }
}
impl From<MVariableDeclarationClause> for AnyMDeclarationClause {
    fn from(node: MVariableDeclarationClause) -> Self {
        Self::MVariableDeclarationClause(node)
    }
}
impl AstNode for AnyMDeclarationClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MClassDeclaration::KIND_SET
        .union(MFunctionDeclaration::KIND_SET)
        .union(MVariableDeclarationClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_CLASS_DECLARATION | M_FUNCTION_DECLARATION | M_VARIABLE_DECLARATION_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_CLASS_DECLARATION => Self::MClassDeclaration(MClassDeclaration { syntax }),
            M_FUNCTION_DECLARATION => Self::MFunctionDeclaration(MFunctionDeclaration { syntax }),
            M_VARIABLE_DECLARATION_CLAUSE => {
                Self::MVariableDeclarationClause(MVariableDeclarationClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MClassDeclaration(it) => &it.syntax,
            Self::MFunctionDeclaration(it) => &it.syntax,
            Self::MVariableDeclarationClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MClassDeclaration(it) => it.syntax,
            Self::MFunctionDeclaration(it) => it.syntax,
            Self::MVariableDeclarationClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MVariableDeclarationClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMDeclarationClause> for SyntaxNode {
    fn from(n: AnyMDeclarationClause) -> Self {
        match n {
            AnyMDeclarationClause::MClassDeclaration(it) => it.into(),
            AnyMDeclarationClause::MFunctionDeclaration(it) => it.into(),
            AnyMDeclarationClause::MVariableDeclarationClause(it) => it.into(),
        }
    }
}
impl From<AnyMDeclarationClause> for SyntaxElement {
    fn from(n: AnyMDeclarationClause) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MArrayExpression> for AnyMExpression {
    fn from(node: MArrayExpression) -> Self {
        Self::MArrayExpression(node)
    }
}
impl From<MAssignmentExpression> for AnyMExpression {
    fn from(node: MAssignmentExpression) -> Self {
        Self::MAssignmentExpression(node)
    }
}
impl From<MBinaryExpression> for AnyMExpression {
    fn from(node: MBinaryExpression) -> Self {
        Self::MBinaryExpression(node)
    }
}
impl From<MBogusExpression> for AnyMExpression {
    fn from(node: MBogusExpression) -> Self {
        Self::MBogusExpression(node)
    }
}
impl From<MCallExpression> for AnyMExpression {
    fn from(node: MCallExpression) -> Self {
        Self::MCallExpression(node)
    }
}
impl From<MComputedMemberExpression> for AnyMExpression {
    fn from(node: MComputedMemberExpression) -> Self {
        Self::MComputedMemberExpression(node)
    }
}
impl From<MConditionalExpression> for AnyMExpression {
    fn from(node: MConditionalExpression) -> Self {
        Self::MConditionalExpression(node)
    }
}
impl From<MConstantExpression> for AnyMExpression {
    fn from(node: MConstantExpression) -> Self {
        Self::MConstantExpression(node)
    }
}
impl From<MFunctionExpression> for AnyMExpression {
    fn from(node: MFunctionExpression) -> Self {
        Self::MFunctionExpression(node)
    }
}
impl From<MHashMapExpression> for AnyMExpression {
    fn from(node: MHashMapExpression) -> Self {
        Self::MHashMapExpression(node)
    }
}
impl From<MHashSetExpression> for AnyMExpression {
    fn from(node: MHashSetExpression) -> Self {
        Self::MHashSetExpression(node)
    }
}
impl From<MIdentifierExpression> for AnyMExpression {
    fn from(node: MIdentifierExpression) -> Self {
        Self::MIdentifierExpression(node)
    }
}
impl From<MInExpression> for AnyMExpression {
    fn from(node: MInExpression) -> Self {
        Self::MInExpression(node)
    }
}
impl From<MInstanceofExpression> for AnyMExpression {
    fn from(node: MInstanceofExpression) -> Self {
        Self::MInstanceofExpression(node)
    }
}
impl From<MLogicalExpression> for AnyMExpression {
    fn from(node: MLogicalExpression) -> Self {
        Self::MLogicalExpression(node)
    }
}
impl From<MNewExpression> for AnyMExpression {
    fn from(node: MNewExpression) -> Self {
        Self::MNewExpression(node)
    }
}
impl From<MObjectExpression> for AnyMExpression {
    fn from(node: MObjectExpression) -> Self {
        Self::MObjectExpression(node)
    }
}
impl From<MParenthesizedExpression> for AnyMExpression {
    fn from(node: MParenthesizedExpression) -> Self {
        Self::MParenthesizedExpression(node)
    }
}
impl From<MPostUpdateExpression> for AnyMExpression {
    fn from(node: MPostUpdateExpression) -> Self {
        Self::MPostUpdateExpression(node)
    }
}
impl From<MPreUpdateExpression> for AnyMExpression {
    fn from(node: MPreUpdateExpression) -> Self {
        Self::MPreUpdateExpression(node)
    }
}
impl From<MSequenceExpression> for AnyMExpression {
    fn from(node: MSequenceExpression) -> Self {
        Self::MSequenceExpression(node)
    }
}
impl From<MSqlConcatenationExpression> for AnyMExpression {
    fn from(node: MSqlConcatenationExpression) -> Self {
        Self::MSqlConcatenationExpression(node)
    }
}
impl From<MStaticMemberExpression> for AnyMExpression {
    fn from(node: MStaticMemberExpression) -> Self {
        Self::MStaticMemberExpression(node)
    }
}
impl From<MSuperExpression> for AnyMExpression {
    fn from(node: MSuperExpression) -> Self {
        Self::MSuperExpression(node)
    }
}
impl From<MTemplateExpression> for AnyMExpression {
    fn from(node: MTemplateExpression) -> Self {
        Self::MTemplateExpression(node)
    }
}
impl From<MThisExpression> for AnyMExpression {
    fn from(node: MThisExpression) -> Self {
        Self::MThisExpression(node)
    }
}
impl From<MUnaryExpression> for AnyMExpression {
    fn from(node: MUnaryExpression) -> Self {
        Self::MUnaryExpression(node)
    }
}
impl AstNode for AnyMExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyMLiteralExpression::KIND_SET
        .union(MArrayExpression::KIND_SET)
        .union(MAssignmentExpression::KIND_SET)
        .union(MBinaryExpression::KIND_SET)
        .union(MBogusExpression::KIND_SET)
        .union(MCallExpression::KIND_SET)
        .union(MComputedMemberExpression::KIND_SET)
        .union(MConditionalExpression::KIND_SET)
        .union(MConstantExpression::KIND_SET)
        .union(MFunctionExpression::KIND_SET)
        .union(MHashMapExpression::KIND_SET)
        .union(MHashSetExpression::KIND_SET)
        .union(MIdentifierExpression::KIND_SET)
        .union(MInExpression::KIND_SET)
        .union(MInstanceofExpression::KIND_SET)
        .union(MLogicalExpression::KIND_SET)
        .union(MNewExpression::KIND_SET)
        .union(MObjectExpression::KIND_SET)
        .union(MParenthesizedExpression::KIND_SET)
        .union(MPostUpdateExpression::KIND_SET)
        .union(MPreUpdateExpression::KIND_SET)
        .union(MSequenceExpression::KIND_SET)
        .union(MSqlConcatenationExpression::KIND_SET)
        .union(MStaticMemberExpression::KIND_SET)
        .union(MSuperExpression::KIND_SET)
        .union(MTemplateExpression::KIND_SET)
        .union(MThisExpression::KIND_SET)
        .union(MUnaryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_ARRAY_EXPRESSION
            | M_ASSIGNMENT_EXPRESSION
            | M_BINARY_EXPRESSION
            | M_BOGUS_EXPRESSION
            | M_CALL_EXPRESSION
            | M_COMPUTED_MEMBER_EXPRESSION
            | M_CONDITIONAL_EXPRESSION
            | M_CONSTANT_EXPRESSION
            | M_FUNCTION_EXPRESSION
            | M_HASH_MAP_EXPRESSION
            | M_HASH_SET_EXPRESSION
            | M_IDENTIFIER_EXPRESSION
            | M_IN_EXPRESSION
            | M_INSTANCEOF_EXPRESSION
            | M_LOGICAL_EXPRESSION
            | M_NEW_EXPRESSION
            | M_OBJECT_EXPRESSION
            | M_PARENTHESIZED_EXPRESSION
            | M_POST_UPDATE_EXPRESSION
            | M_PRE_UPDATE_EXPRESSION
            | M_SEQUENCE_EXPRESSION
            | M_SQL_CONCATENATION_EXPRESSION
            | M_STATIC_MEMBER_EXPRESSION
            | M_SUPER_EXPRESSION
            | M_TEMPLATE_EXPRESSION
            | M_THIS_EXPRESSION
            | M_UNARY_EXPRESSION => true,
            k if AnyMLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_ARRAY_EXPRESSION => Self::MArrayExpression(MArrayExpression { syntax }),
            M_ASSIGNMENT_EXPRESSION => {
                Self::MAssignmentExpression(MAssignmentExpression { syntax })
            }
            M_BINARY_EXPRESSION => Self::MBinaryExpression(MBinaryExpression { syntax }),
            M_BOGUS_EXPRESSION => Self::MBogusExpression(MBogusExpression { syntax }),
            M_CALL_EXPRESSION => Self::MCallExpression(MCallExpression { syntax }),
            M_COMPUTED_MEMBER_EXPRESSION => {
                Self::MComputedMemberExpression(MComputedMemberExpression { syntax })
            }
            M_CONDITIONAL_EXPRESSION => {
                Self::MConditionalExpression(MConditionalExpression { syntax })
            }
            M_CONSTANT_EXPRESSION => Self::MConstantExpression(MConstantExpression { syntax }),
            M_FUNCTION_EXPRESSION => Self::MFunctionExpression(MFunctionExpression { syntax }),
            M_HASH_MAP_EXPRESSION => Self::MHashMapExpression(MHashMapExpression { syntax }),
            M_HASH_SET_EXPRESSION => Self::MHashSetExpression(MHashSetExpression { syntax }),
            M_IDENTIFIER_EXPRESSION => {
                Self::MIdentifierExpression(MIdentifierExpression { syntax })
            }
            M_IN_EXPRESSION => Self::MInExpression(MInExpression { syntax }),
            M_INSTANCEOF_EXPRESSION => {
                Self::MInstanceofExpression(MInstanceofExpression { syntax })
            }
            M_LOGICAL_EXPRESSION => Self::MLogicalExpression(MLogicalExpression { syntax }),
            M_NEW_EXPRESSION => Self::MNewExpression(MNewExpression { syntax }),
            M_OBJECT_EXPRESSION => Self::MObjectExpression(MObjectExpression { syntax }),
            M_PARENTHESIZED_EXPRESSION => {
                Self::MParenthesizedExpression(MParenthesizedExpression { syntax })
            }
            M_POST_UPDATE_EXPRESSION => {
                Self::MPostUpdateExpression(MPostUpdateExpression { syntax })
            }
            M_PRE_UPDATE_EXPRESSION => Self::MPreUpdateExpression(MPreUpdateExpression { syntax }),
            M_SEQUENCE_EXPRESSION => Self::MSequenceExpression(MSequenceExpression { syntax }),
            M_SQL_CONCATENATION_EXPRESSION => {
                Self::MSqlConcatenationExpression(MSqlConcatenationExpression { syntax })
            }
            M_STATIC_MEMBER_EXPRESSION => {
                Self::MStaticMemberExpression(MStaticMemberExpression { syntax })
            }
            M_SUPER_EXPRESSION => Self::MSuperExpression(MSuperExpression { syntax }),
            M_TEMPLATE_EXPRESSION => Self::MTemplateExpression(MTemplateExpression { syntax }),
            M_THIS_EXPRESSION => Self::MThisExpression(MThisExpression { syntax }),
            M_UNARY_EXPRESSION => Self::MUnaryExpression(MUnaryExpression { syntax }),
            _ => {
                if let Some(any_m_literal_expression) = AnyMLiteralExpression::cast(syntax) {
                    return Some(Self::AnyMLiteralExpression(any_m_literal_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MArrayExpression(it) => &it.syntax,
            Self::MAssignmentExpression(it) => &it.syntax,
            Self::MBinaryExpression(it) => &it.syntax,
            Self::MBogusExpression(it) => &it.syntax,
            Self::MCallExpression(it) => &it.syntax,
            Self::MComputedMemberExpression(it) => &it.syntax,
            Self::MConditionalExpression(it) => &it.syntax,
            Self::MConstantExpression(it) => &it.syntax,
            Self::MFunctionExpression(it) => &it.syntax,
            Self::MHashMapExpression(it) => &it.syntax,
            Self::MHashSetExpression(it) => &it.syntax,
            Self::MIdentifierExpression(it) => &it.syntax,
            Self::MInExpression(it) => &it.syntax,
            Self::MInstanceofExpression(it) => &it.syntax,
            Self::MLogicalExpression(it) => &it.syntax,
            Self::MNewExpression(it) => &it.syntax,
            Self::MObjectExpression(it) => &it.syntax,
            Self::MParenthesizedExpression(it) => &it.syntax,
            Self::MPostUpdateExpression(it) => &it.syntax,
            Self::MPreUpdateExpression(it) => &it.syntax,
            Self::MSequenceExpression(it) => &it.syntax,
            Self::MSqlConcatenationExpression(it) => &it.syntax,
            Self::MStaticMemberExpression(it) => &it.syntax,
            Self::MSuperExpression(it) => &it.syntax,
            Self::MTemplateExpression(it) => &it.syntax,
            Self::MThisExpression(it) => &it.syntax,
            Self::MUnaryExpression(it) => &it.syntax,
            Self::AnyMLiteralExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MArrayExpression(it) => it.syntax,
            Self::MAssignmentExpression(it) => it.syntax,
            Self::MBinaryExpression(it) => it.syntax,
            Self::MBogusExpression(it) => it.syntax,
            Self::MCallExpression(it) => it.syntax,
            Self::MComputedMemberExpression(it) => it.syntax,
            Self::MConditionalExpression(it) => it.syntax,
            Self::MConstantExpression(it) => it.syntax,
            Self::MFunctionExpression(it) => it.syntax,
            Self::MHashMapExpression(it) => it.syntax,
            Self::MHashSetExpression(it) => it.syntax,
            Self::MIdentifierExpression(it) => it.syntax,
            Self::MInExpression(it) => it.syntax,
            Self::MInstanceofExpression(it) => it.syntax,
            Self::MLogicalExpression(it) => it.syntax,
            Self::MNewExpression(it) => it.syntax,
            Self::MObjectExpression(it) => it.syntax,
            Self::MParenthesizedExpression(it) => it.syntax,
            Self::MPostUpdateExpression(it) => it.syntax,
            Self::MPreUpdateExpression(it) => it.syntax,
            Self::MSequenceExpression(it) => it.syntax,
            Self::MSqlConcatenationExpression(it) => it.syntax,
            Self::MStaticMemberExpression(it) => it.syntax,
            Self::MSuperExpression(it) => it.syntax,
            Self::MTemplateExpression(it) => it.syntax,
            Self::MThisExpression(it) => it.syntax,
            Self::MUnaryExpression(it) => it.syntax,
            Self::AnyMLiteralExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MArrayExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MAssignmentExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MBogusExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MCallExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MComputedMemberExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MConditionalExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MConstantExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MHashMapExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MHashSetExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MIdentifierExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MInExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MInstanceofExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MNewExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MObjectExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MPostUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MPreUpdateExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSequenceExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSqlConcatenationExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MStaticMemberExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSuperExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MTemplateExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MThisExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMExpression> for SyntaxNode {
    fn from(n: AnyMExpression) -> Self {
        match n {
            AnyMExpression::AnyMLiteralExpression(it) => it.into(),
            AnyMExpression::MArrayExpression(it) => it.into(),
            AnyMExpression::MAssignmentExpression(it) => it.into(),
            AnyMExpression::MBinaryExpression(it) => it.into(),
            AnyMExpression::MBogusExpression(it) => it.into(),
            AnyMExpression::MCallExpression(it) => it.into(),
            AnyMExpression::MComputedMemberExpression(it) => it.into(),
            AnyMExpression::MConditionalExpression(it) => it.into(),
            AnyMExpression::MConstantExpression(it) => it.into(),
            AnyMExpression::MFunctionExpression(it) => it.into(),
            AnyMExpression::MHashMapExpression(it) => it.into(),
            AnyMExpression::MHashSetExpression(it) => it.into(),
            AnyMExpression::MIdentifierExpression(it) => it.into(),
            AnyMExpression::MInExpression(it) => it.into(),
            AnyMExpression::MInstanceofExpression(it) => it.into(),
            AnyMExpression::MLogicalExpression(it) => it.into(),
            AnyMExpression::MNewExpression(it) => it.into(),
            AnyMExpression::MObjectExpression(it) => it.into(),
            AnyMExpression::MParenthesizedExpression(it) => it.into(),
            AnyMExpression::MPostUpdateExpression(it) => it.into(),
            AnyMExpression::MPreUpdateExpression(it) => it.into(),
            AnyMExpression::MSequenceExpression(it) => it.into(),
            AnyMExpression::MSqlConcatenationExpression(it) => it.into(),
            AnyMExpression::MStaticMemberExpression(it) => it.into(),
            AnyMExpression::MSuperExpression(it) => it.into(),
            AnyMExpression::MTemplateExpression(it) => it.into(),
            AnyMExpression::MThisExpression(it) => it.into(),
            AnyMExpression::MUnaryExpression(it) => it.into(),
        }
    }
}
impl From<AnyMExpression> for SyntaxElement {
    fn from(n: AnyMExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MForVariableDeclaration> for AnyMForInInitializer {
    fn from(node: MForVariableDeclaration) -> Self {
        Self::MForVariableDeclaration(node)
    }
}
impl AstNode for AnyMForInInitializer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyMAssignment::KIND_SET.union(MForVariableDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_FOR_VARIABLE_DECLARATION => true,
            k if AnyMAssignment::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_FOR_VARIABLE_DECLARATION => {
                Self::MForVariableDeclaration(MForVariableDeclaration { syntax })
            }
            _ => {
                if let Some(any_m_assignment) = AnyMAssignment::cast(syntax) {
                    return Some(Self::AnyMAssignment(any_m_assignment));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MForVariableDeclaration(it) => &it.syntax,
            Self::AnyMAssignment(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MForVariableDeclaration(it) => it.syntax,
            Self::AnyMAssignment(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMForInInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMAssignment(it) => std::fmt::Debug::fmt(it, f),
            Self::MForVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMForInInitializer> for SyntaxNode {
    fn from(n: AnyMForInInitializer) -> Self {
        match n {
            AnyMForInInitializer::AnyMAssignment(it) => it.into(),
            AnyMForInInitializer::MForVariableDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyMForInInitializer> for SyntaxElement {
    fn from(n: AnyMForInInitializer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MVariableDeclaration> for AnyMForInitializer {
    fn from(node: MVariableDeclaration) -> Self {
        Self::MVariableDeclaration(node)
    }
}
impl AstNode for AnyMForInitializer {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyMExpression::KIND_SET.union(MVariableDeclaration::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_VARIABLE_DECLARATION => true,
            k if AnyMExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_VARIABLE_DECLARATION => Self::MVariableDeclaration(MVariableDeclaration { syntax }),
            _ => {
                if let Some(any_m_expression) = AnyMExpression::cast(syntax) {
                    return Some(Self::AnyMExpression(any_m_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MVariableDeclaration(it) => &it.syntax,
            Self::AnyMExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MVariableDeclaration(it) => it.syntax,
            Self::AnyMExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMForInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MVariableDeclaration(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMForInitializer> for SyntaxNode {
    fn from(n: AnyMForInitializer) -> Self {
        match n {
            AnyMForInitializer::AnyMExpression(it) => it.into(),
            AnyMForInitializer::MVariableDeclaration(it) => it.into(),
        }
    }
}
impl From<AnyMForInitializer> for SyntaxElement {
    fn from(n: AnyMForInitializer) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusParameter> for AnyMFormalParameter {
    fn from(node: MBogusParameter) -> Self {
        Self::MBogusParameter(node)
    }
}
impl From<MFormalParameter> for AnyMFormalParameter {
    fn from(node: MFormalParameter) -> Self {
        Self::MFormalParameter(node)
    }
}
impl AstNode for AnyMFormalParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MBogusParameter::KIND_SET.union(MFormalParameter::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_BOGUS_PARAMETER | M_FORMAL_PARAMETER)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_PARAMETER => Self::MBogusParameter(MBogusParameter { syntax }),
            M_FORMAL_PARAMETER => Self::MFormalParameter(MFormalParameter { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusParameter(it) => &it.syntax,
            Self::MFormalParameter(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusParameter(it) => it.syntax,
            Self::MFormalParameter(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusParameter(it) => std::fmt::Debug::fmt(it, f),
            Self::MFormalParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMFormalParameter> for SyntaxNode {
    fn from(n: AnyMFormalParameter) -> Self {
        match n {
            AnyMFormalParameter::MBogusParameter(it) => it.into(),
            AnyMFormalParameter::MFormalParameter(it) => it.into(),
        }
    }
}
impl From<AnyMFormalParameter> for SyntaxElement {
    fn from(n: AnyMFormalParameter) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MFunctionDeclaration> for AnyMFunction {
    fn from(node: MFunctionDeclaration) -> Self {
        Self::MFunctionDeclaration(node)
    }
}
impl From<MFunctionExpression> for AnyMFunction {
    fn from(node: MFunctionExpression) -> Self {
        Self::MFunctionExpression(node)
    }
}
impl AstNode for AnyMFunction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MFunctionDeclaration::KIND_SET.union(MFunctionExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_FUNCTION_DECLARATION | M_FUNCTION_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_FUNCTION_DECLARATION => Self::MFunctionDeclaration(MFunctionDeclaration { syntax }),
            M_FUNCTION_EXPRESSION => Self::MFunctionExpression(MFunctionExpression { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MFunctionDeclaration(it) => &it.syntax,
            Self::MFunctionExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MFunctionDeclaration(it) => it.syntax,
            Self::MFunctionExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMFunction> for SyntaxNode {
    fn from(n: AnyMFunction) -> Self {
        match n {
            AnyMFunction::MFunctionDeclaration(it) => it.into(),
            AnyMFunction::MFunctionExpression(it) => it.into(),
        }
    }
}
impl From<AnyMFunction> for SyntaxElement {
    fn from(n: AnyMFunction) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusBinding> for AnyMFunctionBinding {
    fn from(node: MBogusBinding) -> Self {
        Self::MBogusBinding(node)
    }
}
impl From<MExtendedBinding> for AnyMFunctionBinding {
    fn from(node: MExtendedBinding) -> Self {
        Self::MExtendedBinding(node)
    }
}
impl From<MIdentifierBinding> for AnyMFunctionBinding {
    fn from(node: MIdentifierBinding) -> Self {
        Self::MIdentifierBinding(node)
    }
}
impl AstNode for AnyMFunctionBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBogusBinding::KIND_SET
        .union(MExtendedBinding::KIND_SET)
        .union(MIdentifierBinding::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BOGUS_BINDING | M_EXTENDED_BINDING | M_IDENTIFIER_BINDING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_BINDING => Self::MBogusBinding(MBogusBinding { syntax }),
            M_EXTENDED_BINDING => Self::MExtendedBinding(MExtendedBinding { syntax }),
            M_IDENTIFIER_BINDING => Self::MIdentifierBinding(MIdentifierBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusBinding(it) => &it.syntax,
            Self::MExtendedBinding(it) => &it.syntax,
            Self::MIdentifierBinding(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusBinding(it) => it.syntax,
            Self::MExtendedBinding(it) => it.syntax,
            Self::MIdentifierBinding(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::MExtendedBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::MIdentifierBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMFunctionBinding> for SyntaxNode {
    fn from(n: AnyMFunctionBinding) -> Self {
        match n {
            AnyMFunctionBinding::MBogusBinding(it) => it.into(),
            AnyMFunctionBinding::MExtendedBinding(it) => it.into(),
            AnyMFunctionBinding::MIdentifierBinding(it) => it.into(),
        }
    }
}
impl From<AnyMFunctionBinding> for SyntaxElement {
    fn from(n: AnyMFunctionBinding) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MFunctionBody> for AnyMFunctionBody {
    fn from(node: MFunctionBody) -> Self {
        Self::MFunctionBody(node)
    }
}
impl AstNode for AnyMFunctionBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyMExpression::KIND_SET.union(MFunctionBody::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_FUNCTION_BODY => true,
            k if AnyMExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_FUNCTION_BODY => Self::MFunctionBody(MFunctionBody { syntax }),
            _ => {
                if let Some(any_m_expression) = AnyMExpression::cast(syntax) {
                    return Some(Self::AnyMExpression(any_m_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MFunctionBody(it) => &it.syntax,
            Self::AnyMExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MFunctionBody(it) => it.syntax,
            Self::AnyMExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionBody(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMFunctionBody> for SyntaxNode {
    fn from(n: AnyMFunctionBody) -> Self {
        match n {
            AnyMFunctionBody::AnyMExpression(it) => it.into(),
            AnyMFunctionBody::MFunctionBody(it) => it.into(),
        }
    }
}
impl From<AnyMFunctionBody> for SyntaxElement {
    fn from(n: AnyMFunctionBody) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBooleanLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MBooleanLiteralExpression) -> Self {
        Self::MBooleanLiteralExpression(node)
    }
}
impl From<MDateLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MDateLiteralExpression) -> Self {
        Self::MDateLiteralExpression(node)
    }
}
impl From<MLongStringLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MLongStringLiteralExpression) -> Self {
        Self::MLongStringLiteralExpression(node)
    }
}
impl From<MNullLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MNullLiteralExpression) -> Self {
        Self::MNullLiteralExpression(node)
    }
}
impl From<MNumberLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MNumberLiteralExpression) -> Self {
        Self::MNumberLiteralExpression(node)
    }
}
impl From<MSqlLongStringLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MSqlLongStringLiteralExpression) -> Self {
        Self::MSqlLongStringLiteralExpression(node)
    }
}
impl From<MSqlStringLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MSqlStringLiteralExpression) -> Self {
        Self::MSqlStringLiteralExpression(node)
    }
}
impl From<MStringLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MStringLiteralExpression) -> Self {
        Self::MStringLiteralExpression(node)
    }
}
impl From<MTimeLiteralExpression> for AnyMLiteralExpression {
    fn from(node: MTimeLiteralExpression) -> Self {
        Self::MTimeLiteralExpression(node)
    }
}
impl AstNode for AnyMLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBooleanLiteralExpression::KIND_SET
        .union(MDateLiteralExpression::KIND_SET)
        .union(MLongStringLiteralExpression::KIND_SET)
        .union(MNullLiteralExpression::KIND_SET)
        .union(MNumberLiteralExpression::KIND_SET)
        .union(MSqlLongStringLiteralExpression::KIND_SET)
        .union(MSqlStringLiteralExpression::KIND_SET)
        .union(MStringLiteralExpression::KIND_SET)
        .union(MTimeLiteralExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BOOLEAN_LITERAL_EXPRESSION
                | M_DATE_LITERAL_EXPRESSION
                | M_LONG_STRING_LITERAL_EXPRESSION
                | M_NULL_LITERAL_EXPRESSION
                | M_NUMBER_LITERAL_EXPRESSION
                | M_SQL_LONG_STRING_LITERAL_EXPRESSION
                | M_SQL_STRING_LITERAL_EXPRESSION
                | M_STRING_LITERAL_EXPRESSION
                | M_TIME_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOOLEAN_LITERAL_EXPRESSION => {
                Self::MBooleanLiteralExpression(MBooleanLiteralExpression { syntax })
            }
            M_DATE_LITERAL_EXPRESSION => {
                Self::MDateLiteralExpression(MDateLiteralExpression { syntax })
            }
            M_LONG_STRING_LITERAL_EXPRESSION => {
                Self::MLongStringLiteralExpression(MLongStringLiteralExpression { syntax })
            }
            M_NULL_LITERAL_EXPRESSION => {
                Self::MNullLiteralExpression(MNullLiteralExpression { syntax })
            }
            M_NUMBER_LITERAL_EXPRESSION => {
                Self::MNumberLiteralExpression(MNumberLiteralExpression { syntax })
            }
            M_SQL_LONG_STRING_LITERAL_EXPRESSION => {
                Self::MSqlLongStringLiteralExpression(MSqlLongStringLiteralExpression { syntax })
            }
            M_SQL_STRING_LITERAL_EXPRESSION => {
                Self::MSqlStringLiteralExpression(MSqlStringLiteralExpression { syntax })
            }
            M_STRING_LITERAL_EXPRESSION => {
                Self::MStringLiteralExpression(MStringLiteralExpression { syntax })
            }
            M_TIME_LITERAL_EXPRESSION => {
                Self::MTimeLiteralExpression(MTimeLiteralExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBooleanLiteralExpression(it) => &it.syntax,
            Self::MDateLiteralExpression(it) => &it.syntax,
            Self::MLongStringLiteralExpression(it) => &it.syntax,
            Self::MNullLiteralExpression(it) => &it.syntax,
            Self::MNumberLiteralExpression(it) => &it.syntax,
            Self::MSqlLongStringLiteralExpression(it) => &it.syntax,
            Self::MSqlStringLiteralExpression(it) => &it.syntax,
            Self::MStringLiteralExpression(it) => &it.syntax,
            Self::MTimeLiteralExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBooleanLiteralExpression(it) => it.syntax,
            Self::MDateLiteralExpression(it) => it.syntax,
            Self::MLongStringLiteralExpression(it) => it.syntax,
            Self::MNullLiteralExpression(it) => it.syntax,
            Self::MNumberLiteralExpression(it) => it.syntax,
            Self::MSqlLongStringLiteralExpression(it) => it.syntax,
            Self::MSqlStringLiteralExpression(it) => it.syntax,
            Self::MStringLiteralExpression(it) => it.syntax,
            Self::MTimeLiteralExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MDateLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MLongStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSqlLongStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSqlStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MTimeLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMLiteralExpression> for SyntaxNode {
    fn from(n: AnyMLiteralExpression) -> Self {
        match n {
            AnyMLiteralExpression::MBooleanLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MDateLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MLongStringLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MNullLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MNumberLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MSqlLongStringLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MSqlStringLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MStringLiteralExpression(it) => it.into(),
            AnyMLiteralExpression::MTimeLiteralExpression(it) => it.into(),
        }
    }
}
impl From<AnyMLiteralExpression> for SyntaxElement {
    fn from(n: AnyMLiteralExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBogusMember> for AnyMObjectMember {
    fn from(node: MBogusMember) -> Self {
        Self::MBogusMember(node)
    }
}
impl From<MPropertyObjectMember> for AnyMObjectMember {
    fn from(node: MPropertyObjectMember) -> Self {
        Self::MPropertyObjectMember(node)
    }
}
impl From<MShorthandPropertyObjectMember> for AnyMObjectMember {
    fn from(node: MShorthandPropertyObjectMember) -> Self {
        Self::MShorthandPropertyObjectMember(node)
    }
}
impl From<MSpread> for AnyMObjectMember {
    fn from(node: MSpread) -> Self {
        Self::MSpread(node)
    }
}
impl AstNode for AnyMObjectMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBogusMember::KIND_SET
        .union(MPropertyObjectMember::KIND_SET)
        .union(MShorthandPropertyObjectMember::KIND_SET)
        .union(MSpread::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BOGUS_MEMBER
                | M_PROPERTY_OBJECT_MEMBER
                | M_SHORTHAND_PROPERTY_OBJECT_MEMBER
                | M_SPREAD
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BOGUS_MEMBER => Self::MBogusMember(MBogusMember { syntax }),
            M_PROPERTY_OBJECT_MEMBER => {
                Self::MPropertyObjectMember(MPropertyObjectMember { syntax })
            }
            M_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
                Self::MShorthandPropertyObjectMember(MShorthandPropertyObjectMember { syntax })
            }
            M_SPREAD => Self::MSpread(MSpread { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBogusMember(it) => &it.syntax,
            Self::MPropertyObjectMember(it) => &it.syntax,
            Self::MShorthandPropertyObjectMember(it) => &it.syntax,
            Self::MSpread(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBogusMember(it) => it.syntax,
            Self::MPropertyObjectMember(it) => it.syntax,
            Self::MShorthandPropertyObjectMember(it) => it.syntax,
            Self::MSpread(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBogusMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MShorthandPropertyObjectMember(it) => std::fmt::Debug::fmt(it, f),
            Self::MSpread(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMObjectMember> for SyntaxNode {
    fn from(n: AnyMObjectMember) -> Self {
        match n {
            AnyMObjectMember::MBogusMember(it) => it.into(),
            AnyMObjectMember::MPropertyObjectMember(it) => it.into(),
            AnyMObjectMember::MShorthandPropertyObjectMember(it) => it.into(),
            AnyMObjectMember::MSpread(it) => it.into(),
        }
    }
}
impl From<AnyMObjectMember> for SyntaxElement {
    fn from(n: AnyMObjectMember) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MComputedMemberName> for AnyMObjectMemberName {
    fn from(node: MComputedMemberName) -> Self {
        Self::MComputedMemberName(node)
    }
}
impl From<MLiteralMemberName> for AnyMObjectMemberName {
    fn from(node: MLiteralMemberName) -> Self {
        Self::MLiteralMemberName(node)
    }
}
impl AstNode for AnyMObjectMemberName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        MComputedMemberName::KIND_SET.union(MLiteralMemberName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_COMPUTED_MEMBER_NAME | M_LITERAL_MEMBER_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_COMPUTED_MEMBER_NAME => Self::MComputedMemberName(MComputedMemberName { syntax }),
            M_LITERAL_MEMBER_NAME => Self::MLiteralMemberName(MLiteralMemberName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MComputedMemberName(it) => &it.syntax,
            Self::MLiteralMemberName(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MComputedMemberName(it) => it.syntax,
            Self::MLiteralMemberName(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMObjectMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MComputedMemberName(it) => std::fmt::Debug::fmt(it, f),
            Self::MLiteralMemberName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMObjectMemberName> for SyntaxNode {
    fn from(n: AnyMObjectMemberName) -> Self {
        match n {
            AnyMObjectMemberName::MComputedMemberName(it) => it.into(),
            AnyMObjectMemberName::MLiteralMemberName(it) => it.into(),
        }
    }
}
impl From<AnyMObjectMemberName> for SyntaxElement {
    fn from(n: AnyMObjectMemberName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MRestParameter> for AnyMParameter {
    fn from(node: MRestParameter) -> Self {
        Self::MRestParameter(node)
    }
}
impl AstNode for AnyMParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        AnyMFormalParameter::KIND_SET.union(MRestParameter::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            M_REST_PARAMETER => true,
            k if AnyMFormalParameter::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_REST_PARAMETER => Self::MRestParameter(MRestParameter { syntax }),
            _ => {
                if let Some(any_m_formal_parameter) = AnyMFormalParameter::cast(syntax) {
                    return Some(Self::AnyMFormalParameter(any_m_formal_parameter));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MRestParameter(it) => &it.syntax,
            Self::AnyMFormalParameter(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MRestParameter(it) => it.syntax,
            Self::AnyMFormalParameter(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyMParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyMFormalParameter(it) => std::fmt::Debug::fmt(it, f),
            Self::MRestParameter(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMParameter> for SyntaxNode {
    fn from(n: AnyMParameter) -> Self {
        match n {
            AnyMParameter::AnyMFormalParameter(it) => it.into(),
            AnyMParameter::MRestParameter(it) => it.into(),
        }
    }
}
impl From<AnyMParameter> for SyntaxElement {
    fn from(n: AnyMParameter) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MExpressionSnipped> for AnyMRoot {
    fn from(node: MExpressionSnipped) -> Self {
        Self::MExpressionSnipped(node)
    }
}
impl From<MModule> for AnyMRoot {
    fn from(node: MModule) -> Self {
        Self::MModule(node)
    }
}
impl From<MReportFile> for AnyMRoot {
    fn from(node: MReportFile) -> Self {
        Self::MReportFile(node)
    }
}
impl From<MScript> for AnyMRoot {
    fn from(node: MScript) -> Self {
        Self::MScript(node)
    }
}
impl AstNode for AnyMRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MExpressionSnipped::KIND_SET
        .union(MModule::KIND_SET)
        .union(MReportFile::KIND_SET)
        .union(MScript::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_EXPRESSION_SNIPPED | M_MODULE | M_REPORT_FILE | M_SCRIPT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_EXPRESSION_SNIPPED => Self::MExpressionSnipped(MExpressionSnipped { syntax }),
            M_MODULE => Self::MModule(MModule { syntax }),
            M_REPORT_FILE => Self::MReportFile(MReportFile { syntax }),
            M_SCRIPT => Self::MScript(MScript { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MExpressionSnipped(it) => &it.syntax,
            Self::MModule(it) => &it.syntax,
            Self::MReportFile(it) => &it.syntax,
            Self::MScript(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MExpressionSnipped(it) => it.syntax,
            Self::MModule(it) => it.syntax,
            Self::MReportFile(it) => it.syntax,
            Self::MScript(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MExpressionSnipped(it) => std::fmt::Debug::fmt(it, f),
            Self::MModule(it) => std::fmt::Debug::fmt(it, f),
            Self::MReportFile(it) => std::fmt::Debug::fmt(it, f),
            Self::MScript(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMRoot> for SyntaxNode {
    fn from(n: AnyMRoot) -> Self {
        match n {
            AnyMRoot::MExpressionSnipped(it) => it.into(),
            AnyMRoot::MModule(it) => it.into(),
            AnyMRoot::MReportFile(it) => it.into(),
            AnyMRoot::MScript(it) => it.into(),
        }
    }
}
impl From<AnyMRoot> for SyntaxElement {
    fn from(n: AnyMRoot) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MBlockStatement> for AnyMStatement {
    fn from(node: MBlockStatement) -> Self {
        Self::MBlockStatement(node)
    }
}
impl From<MBogusStatement> for AnyMStatement {
    fn from(node: MBogusStatement) -> Self {
        Self::MBogusStatement(node)
    }
}
impl From<MBreakStatement> for AnyMStatement {
    fn from(node: MBreakStatement) -> Self {
        Self::MBreakStatement(node)
    }
}
impl From<MClassDeclaration> for AnyMStatement {
    fn from(node: MClassDeclaration) -> Self {
        Self::MClassDeclaration(node)
    }
}
impl From<MContinueStatement> for AnyMStatement {
    fn from(node: MContinueStatement) -> Self {
        Self::MContinueStatement(node)
    }
}
impl From<MDebugStatement> for AnyMStatement {
    fn from(node: MDebugStatement) -> Self {
        Self::MDebugStatement(node)
    }
}
impl From<MEmptyStatement> for AnyMStatement {
    fn from(node: MEmptyStatement) -> Self {
        Self::MEmptyStatement(node)
    }
}
impl From<MExpressionStatement> for AnyMStatement {
    fn from(node: MExpressionStatement) -> Self {
        Self::MExpressionStatement(node)
    }
}
impl From<MForAllInStatement> for AnyMStatement {
    fn from(node: MForAllInStatement) -> Self {
        Self::MForAllInStatement(node)
    }
}
impl From<MForAllStatement> for AnyMStatement {
    fn from(node: MForAllStatement) -> Self {
        Self::MForAllStatement(node)
    }
}
impl From<MForStatement> for AnyMStatement {
    fn from(node: MForStatement) -> Self {
        Self::MForStatement(node)
    }
}
impl From<MFunctionDeclaration> for AnyMStatement {
    fn from(node: MFunctionDeclaration) -> Self {
        Self::MFunctionDeclaration(node)
    }
}
impl From<MIfStatement> for AnyMStatement {
    fn from(node: MIfStatement) -> Self {
        Self::MIfStatement(node)
    }
}
impl From<MReturnStatement> for AnyMStatement {
    fn from(node: MReturnStatement) -> Self {
        Self::MReturnStatement(node)
    }
}
impl From<MSwitchStatement> for AnyMStatement {
    fn from(node: MSwitchStatement) -> Self {
        Self::MSwitchStatement(node)
    }
}
impl From<MThrowStatement> for AnyMStatement {
    fn from(node: MThrowStatement) -> Self {
        Self::MThrowStatement(node)
    }
}
impl From<MTryFinallyStatement> for AnyMStatement {
    fn from(node: MTryFinallyStatement) -> Self {
        Self::MTryFinallyStatement(node)
    }
}
impl From<MTryStatement> for AnyMStatement {
    fn from(node: MTryStatement) -> Self {
        Self::MTryStatement(node)
    }
}
impl From<MVariableStatement> for AnyMStatement {
    fn from(node: MVariableStatement) -> Self {
        Self::MVariableStatement(node)
    }
}
impl From<MWhileStatement> for AnyMStatement {
    fn from(node: MWhileStatement) -> Self {
        Self::MWhileStatement(node)
    }
}
impl AstNode for AnyMStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MBlockStatement::KIND_SET
        .union(MBogusStatement::KIND_SET)
        .union(MBreakStatement::KIND_SET)
        .union(MClassDeclaration::KIND_SET)
        .union(MContinueStatement::KIND_SET)
        .union(MDebugStatement::KIND_SET)
        .union(MEmptyStatement::KIND_SET)
        .union(MExpressionStatement::KIND_SET)
        .union(MForAllInStatement::KIND_SET)
        .union(MForAllStatement::KIND_SET)
        .union(MForStatement::KIND_SET)
        .union(MFunctionDeclaration::KIND_SET)
        .union(MIfStatement::KIND_SET)
        .union(MReturnStatement::KIND_SET)
        .union(MSwitchStatement::KIND_SET)
        .union(MThrowStatement::KIND_SET)
        .union(MTryFinallyStatement::KIND_SET)
        .union(MTryStatement::KIND_SET)
        .union(MVariableStatement::KIND_SET)
        .union(MWhileStatement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_BLOCK_STATEMENT
                | M_BOGUS_STATEMENT
                | M_BREAK_STATEMENT
                | M_CLASS_DECLARATION
                | M_CONTINUE_STATEMENT
                | M_DEBUG_STATEMENT
                | M_EMPTY_STATEMENT
                | M_EXPRESSION_STATEMENT
                | M_FOR_ALL_IN_STATEMENT
                | M_FOR_ALL_STATEMENT
                | M_FOR_STATEMENT
                | M_FUNCTION_DECLARATION
                | M_IF_STATEMENT
                | M_RETURN_STATEMENT
                | M_SWITCH_STATEMENT
                | M_THROW_STATEMENT
                | M_TRY_FINALLY_STATEMENT
                | M_TRY_STATEMENT
                | M_VARIABLE_STATEMENT
                | M_WHILE_STATEMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_BLOCK_STATEMENT => Self::MBlockStatement(MBlockStatement { syntax }),
            M_BOGUS_STATEMENT => Self::MBogusStatement(MBogusStatement { syntax }),
            M_BREAK_STATEMENT => Self::MBreakStatement(MBreakStatement { syntax }),
            M_CLASS_DECLARATION => Self::MClassDeclaration(MClassDeclaration { syntax }),
            M_CONTINUE_STATEMENT => Self::MContinueStatement(MContinueStatement { syntax }),
            M_DEBUG_STATEMENT => Self::MDebugStatement(MDebugStatement { syntax }),
            M_EMPTY_STATEMENT => Self::MEmptyStatement(MEmptyStatement { syntax }),
            M_EXPRESSION_STATEMENT => Self::MExpressionStatement(MExpressionStatement { syntax }),
            M_FOR_ALL_IN_STATEMENT => Self::MForAllInStatement(MForAllInStatement { syntax }),
            M_FOR_ALL_STATEMENT => Self::MForAllStatement(MForAllStatement { syntax }),
            M_FOR_STATEMENT => Self::MForStatement(MForStatement { syntax }),
            M_FUNCTION_DECLARATION => Self::MFunctionDeclaration(MFunctionDeclaration { syntax }),
            M_IF_STATEMENT => Self::MIfStatement(MIfStatement { syntax }),
            M_RETURN_STATEMENT => Self::MReturnStatement(MReturnStatement { syntax }),
            M_SWITCH_STATEMENT => Self::MSwitchStatement(MSwitchStatement { syntax }),
            M_THROW_STATEMENT => Self::MThrowStatement(MThrowStatement { syntax }),
            M_TRY_FINALLY_STATEMENT => Self::MTryFinallyStatement(MTryFinallyStatement { syntax }),
            M_TRY_STATEMENT => Self::MTryStatement(MTryStatement { syntax }),
            M_VARIABLE_STATEMENT => Self::MVariableStatement(MVariableStatement { syntax }),
            M_WHILE_STATEMENT => Self::MWhileStatement(MWhileStatement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MBlockStatement(it) => &it.syntax,
            Self::MBogusStatement(it) => &it.syntax,
            Self::MBreakStatement(it) => &it.syntax,
            Self::MClassDeclaration(it) => &it.syntax,
            Self::MContinueStatement(it) => &it.syntax,
            Self::MDebugStatement(it) => &it.syntax,
            Self::MEmptyStatement(it) => &it.syntax,
            Self::MExpressionStatement(it) => &it.syntax,
            Self::MForAllInStatement(it) => &it.syntax,
            Self::MForAllStatement(it) => &it.syntax,
            Self::MForStatement(it) => &it.syntax,
            Self::MFunctionDeclaration(it) => &it.syntax,
            Self::MIfStatement(it) => &it.syntax,
            Self::MReturnStatement(it) => &it.syntax,
            Self::MSwitchStatement(it) => &it.syntax,
            Self::MThrowStatement(it) => &it.syntax,
            Self::MTryFinallyStatement(it) => &it.syntax,
            Self::MTryStatement(it) => &it.syntax,
            Self::MVariableStatement(it) => &it.syntax,
            Self::MWhileStatement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MBlockStatement(it) => it.syntax,
            Self::MBogusStatement(it) => it.syntax,
            Self::MBreakStatement(it) => it.syntax,
            Self::MClassDeclaration(it) => it.syntax,
            Self::MContinueStatement(it) => it.syntax,
            Self::MDebugStatement(it) => it.syntax,
            Self::MEmptyStatement(it) => it.syntax,
            Self::MExpressionStatement(it) => it.syntax,
            Self::MForAllInStatement(it) => it.syntax,
            Self::MForAllStatement(it) => it.syntax,
            Self::MForStatement(it) => it.syntax,
            Self::MFunctionDeclaration(it) => it.syntax,
            Self::MIfStatement(it) => it.syntax,
            Self::MReturnStatement(it) => it.syntax,
            Self::MSwitchStatement(it) => it.syntax,
            Self::MThrowStatement(it) => it.syntax,
            Self::MTryFinallyStatement(it) => it.syntax,
            Self::MTryStatement(it) => it.syntax,
            Self::MVariableStatement(it) => it.syntax,
            Self::MWhileStatement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MBlockStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MBogusStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MBreakStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MClassDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MContinueStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MDebugStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MEmptyStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MExpressionStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MForAllInStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MForAllStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MForStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MFunctionDeclaration(it) => std::fmt::Debug::fmt(it, f),
            Self::MIfStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MReturnStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MSwitchStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MThrowStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MTryFinallyStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MTryStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MVariableStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::MWhileStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMStatement> for SyntaxNode {
    fn from(n: AnyMStatement) -> Self {
        match n {
            AnyMStatement::MBlockStatement(it) => it.into(),
            AnyMStatement::MBogusStatement(it) => it.into(),
            AnyMStatement::MBreakStatement(it) => it.into(),
            AnyMStatement::MClassDeclaration(it) => it.into(),
            AnyMStatement::MContinueStatement(it) => it.into(),
            AnyMStatement::MDebugStatement(it) => it.into(),
            AnyMStatement::MEmptyStatement(it) => it.into(),
            AnyMStatement::MExpressionStatement(it) => it.into(),
            AnyMStatement::MForAllInStatement(it) => it.into(),
            AnyMStatement::MForAllStatement(it) => it.into(),
            AnyMStatement::MForStatement(it) => it.into(),
            AnyMStatement::MFunctionDeclaration(it) => it.into(),
            AnyMStatement::MIfStatement(it) => it.into(),
            AnyMStatement::MReturnStatement(it) => it.into(),
            AnyMStatement::MSwitchStatement(it) => it.into(),
            AnyMStatement::MThrowStatement(it) => it.into(),
            AnyMStatement::MTryFinallyStatement(it) => it.into(),
            AnyMStatement::MTryStatement(it) => it.into(),
            AnyMStatement::MVariableStatement(it) => it.into(),
            AnyMStatement::MWhileStatement(it) => it.into(),
        }
    }
}
impl From<AnyMStatement> for SyntaxElement {
    fn from(n: AnyMStatement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MLongStringLiteralExpression> for AnyMStringLiteralExpression {
    fn from(node: MLongStringLiteralExpression) -> Self {
        Self::MLongStringLiteralExpression(node)
    }
}
impl From<MSqlLongStringLiteralExpression> for AnyMStringLiteralExpression {
    fn from(node: MSqlLongStringLiteralExpression) -> Self {
        Self::MSqlLongStringLiteralExpression(node)
    }
}
impl From<MSqlStringLiteralExpression> for AnyMStringLiteralExpression {
    fn from(node: MSqlStringLiteralExpression) -> Self {
        Self::MSqlStringLiteralExpression(node)
    }
}
impl From<MStringLiteralExpression> for AnyMStringLiteralExpression {
    fn from(node: MStringLiteralExpression) -> Self {
        Self::MStringLiteralExpression(node)
    }
}
impl AstNode for AnyMStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MLongStringLiteralExpression::KIND_SET
        .union(MSqlLongStringLiteralExpression::KIND_SET)
        .union(MSqlStringLiteralExpression::KIND_SET)
        .union(MStringLiteralExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            M_LONG_STRING_LITERAL_EXPRESSION
                | M_SQL_LONG_STRING_LITERAL_EXPRESSION
                | M_SQL_STRING_LITERAL_EXPRESSION
                | M_STRING_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_LONG_STRING_LITERAL_EXPRESSION => {
                Self::MLongStringLiteralExpression(MLongStringLiteralExpression { syntax })
            }
            M_SQL_LONG_STRING_LITERAL_EXPRESSION => {
                Self::MSqlLongStringLiteralExpression(MSqlLongStringLiteralExpression { syntax })
            }
            M_SQL_STRING_LITERAL_EXPRESSION => {
                Self::MSqlStringLiteralExpression(MSqlStringLiteralExpression { syntax })
            }
            M_STRING_LITERAL_EXPRESSION => {
                Self::MStringLiteralExpression(MStringLiteralExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MLongStringLiteralExpression(it) => &it.syntax,
            Self::MSqlLongStringLiteralExpression(it) => &it.syntax,
            Self::MSqlStringLiteralExpression(it) => &it.syntax,
            Self::MStringLiteralExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MLongStringLiteralExpression(it) => it.syntax,
            Self::MSqlLongStringLiteralExpression(it) => it.syntax,
            Self::MSqlStringLiteralExpression(it) => it.syntax,
            Self::MStringLiteralExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MLongStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSqlLongStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MSqlStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::MStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMStringLiteralExpression> for SyntaxNode {
    fn from(n: AnyMStringLiteralExpression) -> Self {
        match n {
            AnyMStringLiteralExpression::MLongStringLiteralExpression(it) => it.into(),
            AnyMStringLiteralExpression::MSqlLongStringLiteralExpression(it) => it.into(),
            AnyMStringLiteralExpression::MSqlStringLiteralExpression(it) => it.into(),
            AnyMStringLiteralExpression::MStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<AnyMStringLiteralExpression> for SyntaxElement {
    fn from(n: AnyMStringLiteralExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<MCaseClause> for AnyMSwitchClause {
    fn from(node: MCaseClause) -> Self {
        Self::MCaseClause(node)
    }
}
impl From<MDefaultClause> for AnyMSwitchClause {
    fn from(node: MDefaultClause) -> Self {
        Self::MDefaultClause(node)
    }
}
impl AstNode for AnyMSwitchClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = MCaseClause::KIND_SET.union(MDefaultClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, M_CASE_CLAUSE | M_DEFAULT_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            M_CASE_CLAUSE => Self::MCaseClause(MCaseClause { syntax }),
            M_DEFAULT_CLAUSE => Self::MDefaultClause(MDefaultClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::MCaseClause(it) => &it.syntax,
            Self::MDefaultClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::MCaseClause(it) => it.syntax,
            Self::MDefaultClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyMSwitchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MCaseClause(it) => std::fmt::Debug::fmt(it, f),
            Self::MDefaultClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyMSwitchClause> for SyntaxNode {
    fn from(n: AnyMSwitchClause) -> Self {
        match n {
            AnyMSwitchClause::MCaseClause(it) => it.into(),
            AnyMSwitchClause::MDefaultClause(it) => it.into(),
        }
    }
}
impl From<AnyMSwitchClause> for SyntaxElement {
    fn from(n: AnyMSwitchClause) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyMAnnotationElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMArrayElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMCallArgument {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMConstructorParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMForInInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMForInitializer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMObjectMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyMSwitchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MAnnotationAttribute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MAnnotationBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MAnnotationElement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MAnnotationGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MArrayHole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MAssignmentExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MBlockStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MBreakStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MCallArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MCaseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MCatchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MCatchDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MClassDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MClassMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MComputedMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MComputedMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MComputedMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MConditionalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MConstantExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MConstructorClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MConstructorParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MContinueStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MDateLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MDebugStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MDefaultClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MDirective {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MExpressionSnipped {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MExpressionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MExtendedBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MExtendsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MFinallyClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MForAllInStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MForAllStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MForIteratorFactory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MForStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MForVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MFormalParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MFunctionBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MFunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MGetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MHashMapExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MHashSetExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MIdentifierAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MIdentifierBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MIdentifierExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MIfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MInitializerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MInstanceofExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MLiteralMemberName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MLongStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MMethodClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MNewExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MObjectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MParenthesizedAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MPostUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MPreUpdateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReferenceIdentifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReportFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReportName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReportSection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReportSectionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MRestParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MScript {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSequenceExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSetterClassMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MShorthandPropertyObjectMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSpread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSqlConcatenationExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSqlLongStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MStaticMemberAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MStaticMemberExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSuperExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MSwitchStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MTemplateExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MThisExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MThrowStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MTimeLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MTryFinallyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MTryStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MVariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MVariableDeclarationClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MVariableDeclarator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MVariableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for MWhileStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogus {
    syntax: SyntaxNode,
}
impl MBogus {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogus> for SyntaxNode {
    fn from(n: MBogus) -> Self {
        n.syntax
    }
}
impl From<MBogus> for SyntaxElement {
    fn from(n: MBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusAssignment {
    syntax: SyntaxNode,
}
impl MBogusAssignment {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_ASSIGNMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusAssignment")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusAssignment> for SyntaxNode {
    fn from(n: MBogusAssignment) -> Self {
        n.syntax
    }
}
impl From<MBogusAssignment> for SyntaxElement {
    fn from(n: MBogusAssignment) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusBinding {
    syntax: SyntaxNode,
}
impl MBogusBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_BINDING
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusBinding")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusBinding> for SyntaxNode {
    fn from(n: MBogusBinding) -> Self {
        n.syntax
    }
}
impl From<MBogusBinding> for SyntaxElement {
    fn from(n: MBogusBinding) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusExpression {
    syntax: SyntaxNode,
}
impl MBogusExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_EXPRESSION
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusExpression> for SyntaxNode {
    fn from(n: MBogusExpression) -> Self {
        n.syntax
    }
}
impl From<MBogusExpression> for SyntaxElement {
    fn from(n: MBogusExpression) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusMember {
    syntax: SyntaxNode,
}
impl MBogusMember {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_MEMBER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusMember")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusMember> for SyntaxNode {
    fn from(n: MBogusMember) -> Self {
        n.syntax
    }
}
impl From<MBogusMember> for SyntaxElement {
    fn from(n: MBogusMember) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusParameter {
    syntax: SyntaxNode,
}
impl MBogusParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_PARAMETER
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusParameter> for SyntaxNode {
    fn from(n: MBogusParameter) -> Self {
        n.syntax
    }
}
impl From<MBogusParameter> for SyntaxElement {
    fn from(n: MBogusParameter) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct MBogusStatement {
    syntax: SyntaxNode,
}
impl MBogusStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn items(&self) -> SyntaxElementChildren {
        support::elements(&self.syntax)
    }
}
impl AstNode for MBogusStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_BOGUS_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_BOGUS_STATEMENT
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        &self.syntax
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax
    }
}
impl std::fmt::Debug for MBogusStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MBogusStatement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<MBogusStatement> for SyntaxNode {
    fn from(n: MBogusStatement) -> Self {
        n.syntax
    }
}
impl From<MBogusStatement> for SyntaxElement {
    fn from(n: MBogusStatement) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyMBogusNode = MBogus | MBogusAssignment | MBogusBinding | MBogusExpression | MBogusMember | MBogusParameter | MBogusStatement }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MAnnotationAttributeList {
    syntax_list: SyntaxList,
}
impl MAnnotationAttributeList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MAnnotationAttributeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_ATTRIBUTE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_ATTRIBUTE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MAnnotationAttributeList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MAnnotationAttributeList {
    type Language = Language;
    type Node = MAnnotationAttribute;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MAnnotationAttributeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MAnnotationAttributeList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MAnnotationAttributeList {
    type Item = SyntaxResult<MAnnotationAttribute>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MAnnotationAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MAnnotationAttributeList {
    type Item = SyntaxResult<MAnnotationAttribute>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MAnnotationAttribute>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MAnnotationGroupList {
    syntax_list: SyntaxList,
}
impl MAnnotationGroupList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MAnnotationGroupList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_GROUP_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_GROUP_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MAnnotationGroupList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MAnnotationGroupList {
    type Language = Language;
    type Node = MAnnotationGroup;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MAnnotationGroupList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MAnnotationGroupList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MAnnotationGroupList {
    type Item = MAnnotationGroup;
    type IntoIter = AstNodeListIterator<Language, MAnnotationGroup>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MAnnotationGroupList {
    type Item = MAnnotationGroup;
    type IntoIter = AstNodeListIterator<Language, MAnnotationGroup>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MAnnotationList {
    syntax_list: SyntaxList,
}
impl MAnnotationList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MAnnotationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ANNOTATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ANNOTATION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MAnnotationList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MAnnotationList {
    type Language = Language;
    type Node = AnyMAnnotationElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MAnnotationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MAnnotationList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MAnnotationList {
    type Item = SyntaxResult<AnyMAnnotationElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMAnnotationElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MAnnotationList {
    type Item = SyntaxResult<AnyMAnnotationElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMAnnotationElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MArrayElementList {
    syntax_list: SyntaxList,
}
impl MArrayElementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MArrayElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_ARRAY_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_ARRAY_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MArrayElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MArrayElementList {
    type Language = Language;
    type Node = AnyMArrayElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MArrayElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MArrayElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MArrayElementList {
    type Item = SyntaxResult<AnyMArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMArrayElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MArrayElementList {
    type Item = SyntaxResult<AnyMArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMArrayElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MCallArgumentList {
    syntax_list: SyntaxList,
}
impl MCallArgumentList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MCallArgumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CALL_ARGUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CALL_ARGUMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MCallArgumentList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MCallArgumentList {
    type Language = Language;
    type Node = AnyMCallArgument;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MCallArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MCallArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MCallArgumentList {
    type Item = SyntaxResult<AnyMCallArgument>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMCallArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MCallArgumentList {
    type Item = SyntaxResult<AnyMCallArgument>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMCallArgument>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MClassMemberList {
    syntax_list: SyntaxList,
}
impl MClassMemberList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MClassMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CLASS_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CLASS_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MClassMemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MClassMemberList {
    type Language = Language;
    type Node = AnyMClassMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MClassMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MClassMemberList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MClassMemberList {
    type Item = AnyMClassMember;
    type IntoIter = AstNodeListIterator<Language, AnyMClassMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MClassMemberList {
    type Item = AnyMClassMember;
    type IntoIter = AstNodeListIterator<Language, AnyMClassMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MConstructorParameterList {
    syntax_list: SyntaxList,
}
impl MConstructorParameterList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MConstructorParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_CONSTRUCTOR_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_CONSTRUCTOR_PARAMETER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MConstructorParameterList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MConstructorParameterList {
    type Language = Language;
    type Node = AnyMConstructorParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MConstructorParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MConstructorParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MConstructorParameterList {
    type Item = SyntaxResult<AnyMConstructorParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMConstructorParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MConstructorParameterList {
    type Item = SyntaxResult<AnyMConstructorParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMConstructorParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MDirectiveList {
    syntax_list: SyntaxList,
}
impl MDirectiveList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MDirectiveList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_DIRECTIVE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_DIRECTIVE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MDirectiveList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MDirectiveList {
    type Language = Language;
    type Node = MDirective;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MDirectiveList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MDirectiveList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MDirectiveList {
    type Item = MDirective;
    type IntoIter = AstNodeListIterator<Language, MDirective>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MDirectiveList {
    type Item = MDirective;
    type IntoIter = AstNodeListIterator<Language, MDirective>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MHashMapMemberList {
    syntax_list: SyntaxList,
}
impl MHashMapMemberList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MHashMapMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_HASH_MAP_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_HASH_MAP_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MHashMapMemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MHashMapMemberList {
    type Language = Language;
    type Node = AnyMObjectMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MHashMapMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MHashMapMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MHashMapMemberList {
    type Item = SyntaxResult<AnyMObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MHashMapMemberList {
    type Item = SyntaxResult<AnyMObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MHashSetElementList {
    syntax_list: SyntaxList,
}
impl MHashSetElementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MHashSetElementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_HASH_SET_ELEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_HASH_SET_ELEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MHashSetElementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MHashSetElementList {
    type Language = Language;
    type Node = AnyMArrayElement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MHashSetElementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MHashSetElementList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MHashSetElementList {
    type Item = SyntaxResult<AnyMArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMArrayElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MHashSetElementList {
    type Item = SyntaxResult<AnyMArrayElement>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMArrayElement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MModuleItemList {
    syntax_list: SyntaxList,
}
impl MModuleItemList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MModuleItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_MODULE_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_MODULE_ITEM_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MModuleItemList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MModuleItemList {
    type Language = Language;
    type Node = AnyMStatement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MModuleItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MModuleItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MModuleItemList {
    type Item = AnyMStatement;
    type IntoIter = AstNodeListIterator<Language, AnyMStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MModuleItemList {
    type Item = AnyMStatement;
    type IntoIter = AstNodeListIterator<Language, AnyMStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MObjectMemberList {
    syntax_list: SyntaxList,
}
impl MObjectMemberList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MObjectMemberList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_OBJECT_MEMBER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_OBJECT_MEMBER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MObjectMemberList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MObjectMemberList {
    type Language = Language;
    type Node = AnyMObjectMember;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MObjectMemberList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MObjectMemberList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MObjectMemberList {
    type Item = SyntaxResult<AnyMObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MObjectMemberList {
    type Item = SyntaxResult<AnyMObjectMember>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMObjectMember>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MParameterList {
    syntax_list: SyntaxList,
}
impl MParameterList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_PARAMETER_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MParameterList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MParameterList {
    type Language = Language;
    type Node = AnyMParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MParameterList {
    type Item = SyntaxResult<AnyMParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MParameterList {
    type Item = SyntaxResult<AnyMParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyMParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MReportInitList {
    syntax_list: SyntaxList,
}
impl MReportInitList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MReportInitList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_INIT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_INIT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MReportInitList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MReportInitList {
    type Language = Language;
    type Node = MExpressionStatement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MReportInitList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MReportInitList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MReportInitList {
    type Item = MExpressionStatement;
    type IntoIter = AstNodeListIterator<Language, MExpressionStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MReportInitList {
    type Item = MExpressionStatement;
    type IntoIter = AstNodeListIterator<Language, MExpressionStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MReportList {
    syntax_list: SyntaxList,
}
impl MReportList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MReportList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MReportList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MReportList {
    type Language = Language;
    type Node = MReport;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MReportList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MReportList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MReportList {
    type Item = MReport;
    type IntoIter = AstNodeListIterator<Language, MReport>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MReportList {
    type Item = MReport;
    type IntoIter = AstNodeListIterator<Language, MReport>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MReportSectionList {
    syntax_list: SyntaxList,
}
impl MReportSectionList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MReportSectionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_REPORT_SECTION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_REPORT_SECTION_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MReportSectionList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MReportSectionList {
    type Language = Language;
    type Node = MReportSection;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MReportSectionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MReportSectionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MReportSectionList {
    type Item = MReportSection;
    type IntoIter = AstNodeListIterator<Language, MReportSection>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MReportSectionList {
    type Item = MReportSection;
    type IntoIter = AstNodeListIterator<Language, MReportSection>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MStatementList {
    syntax_list: SyntaxList,
}
impl MStatementList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MStatementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_STATEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_STATEMENT_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MStatementList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MStatementList {
    type Language = Language;
    type Node = AnyMStatement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MStatementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MStatementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MStatementList {
    type Item = AnyMStatement;
    type IntoIter = AstNodeListIterator<Language, AnyMStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MStatementList {
    type Item = AnyMStatement;
    type IntoIter = AstNodeListIterator<Language, AnyMStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MSwitchCaseList {
    syntax_list: SyntaxList,
}
impl MSwitchCaseList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MSwitchCaseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_SWITCH_CASE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_SWITCH_CASE_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MSwitchCaseList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstNodeList for MSwitchCaseList {
    type Language = Language;
    type Node = AnyMSwitchClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MSwitchCaseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MSwitchCaseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &MSwitchCaseList {
    type Item = AnyMSwitchClause;
    type IntoIter = AstNodeListIterator<Language, AnyMSwitchClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for MSwitchCaseList {
    type Item = AnyMSwitchClause;
    type IntoIter = AstNodeListIterator<Language, AnyMSwitchClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct MVariableDeclaratorList {
    syntax_list: SyntaxList,
}
impl MVariableDeclaratorList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self {
            syntax_list: syntax.into_list(),
        }
    }
}
impl AstNode for MVariableDeclaratorList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(M_VARIABLE_DECLARATOR_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == M_VARIABLE_DECLARATOR_LIST
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self {
                syntax_list: syntax.into_list(),
            })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxNode {
        self.syntax_list.node()
    }
    fn into_syntax(self) -> SyntaxNode {
        self.syntax_list.into_node()
    }
}
impl Serialize for MVariableDeclaratorList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(Some(self.len()))?;
        for e in self.iter() {
            seq.serialize_element(&e)?;
        }
        seq.end()
    }
}
impl AstSeparatedList for MVariableDeclaratorList {
    type Language = Language;
    type Node = MVariableDeclarator;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for MVariableDeclaratorList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("MVariableDeclaratorList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for MVariableDeclaratorList {
    type Item = SyntaxResult<MVariableDeclarator>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MVariableDeclarator>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &MVariableDeclaratorList {
    type Item = SyntaxResult<MVariableDeclarator>;
    type IntoIter = AstSeparatedListNodesIterator<Language, MVariableDeclarator>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone)]
pub struct DebugSyntaxElementChildren(pub SyntaxElementChildren);
impl Debug for DebugSyntaxElementChildren {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(self.clone().0.map(DebugSyntaxElement))
            .finish()
    }
}
struct DebugSyntaxElement(SyntaxElement);
impl Debug for DebugSyntaxElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            SyntaxElement::Node(node) => {
                map_syntax_node ! (node . clone () , node => std :: fmt :: Debug :: fmt (& node , f))
            }
            SyntaxElement::Token(token) => Debug::fmt(token, f),
        }
    }
}
