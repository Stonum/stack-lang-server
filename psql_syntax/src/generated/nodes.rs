//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    PsqlLanguage as Language, PsqlSyntaxElement as SyntaxElement,
    PsqlSyntaxElementChildren as SyntaxElementChildren,
    PsqlSyntaxKind::{self as SyntaxKind, *},
    PsqlSyntaxList as SyntaxList, PsqlSyntaxNode as SyntaxNode, PsqlSyntaxToken as SyntaxToken,
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
pub struct PsqlAlias {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlAlias {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlAliasFields {
        PsqlAliasFields {
            as_token: self.as_token(),
            value: self.value(),
        }
    }
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlAliasFields {
    pub as_token: Option<SyntaxToken>,
    pub value: SyntaxResult<PsqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlArrayExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlArrayExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlArrayExpressionFields {
        PsqlArrayExpressionFields {
            array_token: self.array_token(),
            l_brack_token: self.l_brack_token(),
            items: self.items(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn array_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> PsqlExpressionList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlArrayExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlArrayExpressionFields {
    pub array_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlExpressionList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlArraySubscriptExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlArraySubscriptExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlArraySubscriptExpressionFields {
        PsqlArraySubscriptExpressionFields {
            expression: self.expression(),
            l_brack_token: self.l_brack_token(),
            index: self.index(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlArraySubscriptExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlArraySubscriptExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub index: SyntaxResult<AnyPsqlExpression>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlBetweenExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlBetweenExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlBetweenExpressionFields {
        PsqlBetweenExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            between_token: self.between_token(),
            low: self.low(),
            and_token: self.and_token(),
            high: self.high(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn between_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn low(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn high(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl Serialize for PsqlBetweenExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlBetweenExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub between_token: SyntaxResult<SyntaxToken>,
    pub low: SyntaxResult<AnyPsqlExpression>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub high: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlBinaryExpressionFields {
        PsqlBinaryExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlBinaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlBinaryExpressionFields {
    pub left: SyntaxResult<AnyPsqlExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlBooleanLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlBooleanLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlBooleanLiteralExpressionFields {
        PsqlBooleanLiteralExpressionFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlBooleanLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlBooleanLiteralExpressionFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCallExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCallExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCallExpressionFields {
        PsqlCallExpressionFields {
            schema: self.schema(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn schema(&self) -> Option<PsqlShemaName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn arguments(&self) -> PsqlExpressionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for PsqlCallExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCallExpressionFields {
    pub schema: Option<PsqlShemaName>,
    pub name: SyntaxResult<PsqlName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: PsqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCaseElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCaseElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCaseElseClauseFields {
        PsqlCaseElseClauseFields {
            else_token: self.else_token(),
            result: self.result(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn result(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlCaseElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCaseElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub result: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCaseExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCaseExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCaseExpressionFields {
        PsqlCaseExpressionFields {
            case_token: self.case_token(),
            expression: self.expression(),
            when_clauses: self.when_clauses(),
            else_clause: self.else_clause(),
            end_token: self.end_token(),
        }
    }
    pub fn case_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> Option<AnyPsqlExpression> {
        support::node(&self.syntax, 1usize)
    }
    pub fn when_clauses(&self) -> PsqlCaseWhenClauseList {
        support::list(&self.syntax, 2usize)
    }
    pub fn else_clause(&self) -> Option<PsqlCaseElseClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for PsqlCaseExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCaseExpressionFields {
    pub case_token: SyntaxResult<SyntaxToken>,
    pub expression: Option<AnyPsqlExpression>,
    pub when_clauses: PsqlCaseWhenClauseList,
    pub else_clause: Option<PsqlCaseElseClause>,
    pub end_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCaseWhenClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCaseWhenClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCaseWhenClauseFields {
        PsqlCaseWhenClauseFields {
            when_token: self.when_token(),
            condition: self.condition(),
            then_token: self.then_token(),
            result: self.result(),
        }
    }
    pub fn when_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn then_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn result(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlCaseWhenClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCaseWhenClauseFields {
    pub when_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyPsqlExpression>,
    pub then_token: SyntaxResult<SyntaxToken>,
    pub result: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCastExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCastExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCastExpressionFields {
        PsqlCastExpressionFields {
            expression: self.expression(),
            double_colon_token: self.double_colon_token(),
            ty: self.ty(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn double_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<PsqlTypeName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlCastExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCastExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub double_colon_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<PsqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCastFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCastFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCastFunctionExpressionFields {
        PsqlCastFunctionExpressionFields {
            cast_token: self.cast_token(),
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            as_token: self.as_token(),
            ty: self.ty(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn cast_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn ty(&self) -> SyntaxResult<PsqlTypeName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for PsqlCastFunctionExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCastFunctionExpressionFields {
    pub cast_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<PsqlTypeName>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlColReference {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlColReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlColReferenceFields {
        PsqlColReferenceFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlColReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlColReferenceFields {
    pub name: SyntaxResult<PsqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlColumnList {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlColumnList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlColumnListFields {
        PsqlColumnListFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlColumnNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlColumnList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlColumnListFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlColumnNameList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlCteDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlCteDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlCteDefinitionFields {
        PsqlCteDefinitionFields {
            name: self.name(),
            columns: self.columns(),
            as_token: self.as_token(),
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn columns(&self) -> Option<PsqlColumnList> {
        support::node(&self.syntax, 1usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn query(&self) -> SyntaxResult<AnyPsqlStatement> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for PsqlCteDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlCteDefinitionFields {
    pub name: SyntaxResult<PsqlName>,
    pub columns: Option<PsqlColumnList>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnyPsqlStatement>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlDataBaseName {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlDataBaseName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlDataBaseNameFields {
        PsqlDataBaseNameFields {
            name: self.name(),
            dot_token: self.dot_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlDataBaseName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlDataBaseNameFields {
    pub name: SyntaxResult<PsqlName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlDeleteStatement {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlDeleteStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlDeleteStatementFields {
        PsqlDeleteStatementFields {
            with_clause: self.with_clause(),
            delete_token: self.delete_token(),
            from_token: self.from_token(),
            table: self.table(),
            using: self.using(),
            where_clause: self.where_clause(),
            returning_clause: self.returning_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<PsqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn delete_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn table(&self) -> SyntaxResult<PsqlTableBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn using(&self) -> Option<PsqlDeleteUsingClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn where_clause(&self) -> Option<PsqlWhereClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn returning_clause(&self) -> Option<PsqlReturningClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for PsqlDeleteStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlDeleteStatementFields {
    pub with_clause: Option<PsqlWithClause>,
    pub delete_token: SyntaxResult<SyntaxToken>,
    pub from_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<PsqlTableBinding>,
    pub using: Option<PsqlDeleteUsingClause>,
    pub where_clause: Option<PsqlWhereClause>,
    pub returning_clause: Option<PsqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlDeleteUsingClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlDeleteUsingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlDeleteUsingClauseFields {
        PsqlDeleteUsingClauseFields {
            using_token: self.using_token(),
            items: self.items(),
        }
    }
    pub fn using_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlFromItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlDeleteUsingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlDeleteUsingClauseFields {
    pub using_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlFromItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlDoNothingClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlDoNothingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlDoNothingClauseFields {
        PsqlDoNothingClauseFields {
            do_token: self.do_token(),
            nothing_token: self.nothing_token(),
        }
    }
    pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn nothing_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlDoNothingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlDoNothingClauseFields {
    pub do_token: SyntaxResult<SyntaxToken>,
    pub nothing_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlDoUpdateClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlDoUpdateClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlDoUpdateClauseFields {
        PsqlDoUpdateClauseFields {
            do_token: self.do_token(),
            update_token: self.update_token(),
            set_clause: self.set_clause(),
            where_clause: self.where_clause(),
        }
    }
    pub fn do_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn update_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn set_clause(&self) -> SyntaxResult<PsqlSetClause> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn where_clause(&self) -> Option<PsqlWhereClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlDoUpdateClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlDoUpdateClauseFields {
    pub do_token: SyntaxResult<SyntaxToken>,
    pub update_token: SyntaxResult<SyntaxToken>,
    pub set_clause: SyntaxResult<PsqlSetClause>,
    pub where_clause: Option<PsqlWhereClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlFromClauseFields {
        PsqlFromClauseFields {
            from_token: self.from_token(),
            items: self.items(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlFromItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlFromClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlFromClauseFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlFromItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlFromItem {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlFromItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlFromItemFields {
        PsqlFromItemFields {
            source: self.source(),
            joins: self.joins(),
        }
    }
    pub fn source(&self) -> SyntaxResult<AnyPsqlFromExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn joins(&self) -> PsqlJoinClauseList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlFromItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlFromItemFields {
    pub source: SyntaxResult<AnyPsqlFromExpression>,
    pub joins: PsqlJoinClauseList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlFunctionBinding {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlFunctionBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlFunctionBindingFields {
        PsqlFunctionBindingFields {
            schema: self.schema(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
            alias: self.alias(),
        }
    }
    pub fn schema(&self) -> Option<PsqlShemaName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn arguments(&self) -> PsqlExpressionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn alias(&self) -> Option<PsqlAlias> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for PsqlFunctionBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlFunctionBindingFields {
    pub schema: Option<PsqlShemaName>,
    pub name: SyntaxResult<PsqlName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: PsqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub alias: Option<PsqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlGroupByClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlGroupByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlGroupByClauseFields {
        PsqlGroupByClauseFields {
            group_by_token: self.group_by_token(),
            items: self.items(),
        }
    }
    pub fn group_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlGroupByItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlGroupByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlGroupByClauseFields {
    pub group_by_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlGroupByItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlHavingClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlHavingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlHavingClauseFields {
        PsqlHavingClauseFields {
            having_token: self.having_token(),
            condition: self.condition(),
        }
    }
    pub fn having_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlHavingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlHavingClauseFields {
    pub having_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlInExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlInExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlInExpressionFields {
        PsqlInExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            in_token: self.in_token(),
            source: self.source(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<AnyPsqlInSource> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlInExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlInExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub in_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnyPsqlInSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlInValueList {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlInValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlInValueListFields {
        PsqlInValueListFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlInValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlInValueListFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlInsertStatement {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlInsertStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlInsertStatementFields {
        PsqlInsertStatementFields {
            with_clause: self.with_clause(),
            insert_token: self.insert_token(),
            into_token: self.into_token(),
            table: self.table(),
            columns: self.columns(),
            source: self.source(),
            on_conflict_clause: self.on_conflict_clause(),
            returning_clause: self.returning_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<PsqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn insert_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn into_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn table(&self) -> SyntaxResult<PsqlTableBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn columns(&self) -> Option<PsqlColumnList> {
        support::node(&self.syntax, 4usize)
    }
    pub fn source(&self) -> SyntaxResult<AnyPsqlInsertSource> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn on_conflict_clause(&self) -> Option<PsqlOnConflictClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn returning_clause(&self) -> Option<PsqlReturningClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 8usize)
    }
}
impl Serialize for PsqlInsertStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlInsertStatementFields {
    pub with_clause: Option<PsqlWithClause>,
    pub insert_token: SyntaxResult<SyntaxToken>,
    pub into_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<PsqlTableBinding>,
    pub columns: Option<PsqlColumnList>,
    pub source: SyntaxResult<AnyPsqlInsertSource>,
    pub on_conflict_clause: Option<PsqlOnConflictClause>,
    pub returning_clause: Option<PsqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlInsertValues {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlInsertValues {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlInsertValuesFields {
        PsqlInsertValuesFields {
            values_token: self.values_token(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn values_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> PsqlExpressionList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlInsertValues {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlInsertValuesFields {
    pub values_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlIsNullExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlIsNullExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlIsNullExpressionFields {
        PsqlIsNullExpressionFields {
            expression: self.expression(),
            is_token: self.is_token(),
            not_token: self.not_token(),
            null_token: self.null_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn is_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlIsNullExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlIsNullExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub is_token: SyntaxResult<SyntaxToken>,
    pub not_token: Option<SyntaxToken>,
    pub null_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlJoinClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlJoinClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlJoinClauseFields {
        PsqlJoinClauseFields {
            join_type: self.join_type(),
            outer_token: self.outer_token(),
            join_token: self.join_token(),
            source: self.source(),
            on_token: self.on_token(),
            condition: self.condition(),
        }
    }
    pub fn join_type(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn outer_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn join_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<AnyPsqlFromExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
    }
    pub fn condition(&self) -> Option<AnyPsqlExpression> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for PsqlJoinClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlJoinClauseFields {
    pub join_type: Option<SyntaxToken>,
    pub outer_token: Option<SyntaxToken>,
    pub join_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnyPsqlFromExpression>,
    pub on_token: Option<SyntaxToken>,
    pub condition: Option<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlLikeExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlLikeExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlLikeExpressionFields {
        PsqlLikeExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            operator_token: self.operator_token(),
            pattern: self.pattern(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlLikeExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlLikeExpressionFields {
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlLimitClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlLimitClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlLimitClauseFields {
        PsqlLimitClauseFields {
            limit_token: self.limit_token(),
            limit_count: self.limit_count(),
        }
    }
    pub fn limit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn limit_count(&self) -> SyntaxResult<PsqlNumberLiteralExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlLimitClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlLimitClauseFields {
    pub limit_token: SyntaxResult<SyntaxToken>,
    pub limit_count: SyntaxResult<PsqlNumberLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlLogicalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlLogicalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlLogicalExpressionFields {
        PsqlLogicalExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlLogicalExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlLogicalExpressionFields {
    pub left: SyntaxResult<AnyPsqlExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlName {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlNameFields {
        PsqlNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlNullLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlNullLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlNullLiteralExpressionFields {
        PsqlNullLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlNullLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlNullLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlNumberLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlNumberLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlNumberLiteralExpressionFields {
        PsqlNumberLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlNumberLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlNumberLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlOffsetClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlOffsetClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlOffsetClauseFields {
        PsqlOffsetClauseFields {
            offset_token: self.offset_token(),
            start: self.start(),
        }
    }
    pub fn offset_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn start(&self) -> SyntaxResult<PsqlNumberLiteralExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlOffsetClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlOffsetClauseFields {
    pub offset_token: SyntaxResult<SyntaxToken>,
    pub start: SyntaxResult<PsqlNumberLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlOnConflictClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlOnConflictClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlOnConflictClauseFields {
        PsqlOnConflictClauseFields {
            on_token: self.on_token(),
            conflict_token: self.conflict_token(),
            target: self.target(),
            action: self.action(),
        }
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn conflict_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn target(&self) -> Option<AnyPsqlConflictTarget> {
        support::node(&self.syntax, 2usize)
    }
    pub fn action(&self) -> SyntaxResult<AnyPsqlConflictAction> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlOnConflictClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlOnConflictClauseFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub conflict_token: SyntaxResult<SyntaxToken>,
    pub target: Option<AnyPsqlConflictTarget>,
    pub action: SyntaxResult<AnyPsqlConflictAction>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlOnConstraintClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlOnConstraintClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlOnConstraintClauseFields {
        PsqlOnConstraintClauseFields {
            on_token: self.on_token(),
            constraint_token: self.constraint_token(),
            name: self.name(),
        }
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn constraint_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlOnConstraintClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlOnConstraintClauseFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub constraint_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<PsqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlOrderByClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlOrderByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlOrderByClauseFields {
        PsqlOrderByClauseFields {
            order_by_token: self.order_by_token(),
            items: self.items(),
        }
    }
    pub fn order_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlOrderByExpressionList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlOrderByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlOrderByClauseFields {
    pub order_by_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlOrderByExpressionList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlOrderByExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlOrderByExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlOrderByExpressionFields {
        PsqlOrderByExpressionFields {
            item: self.item(),
            order: self.order(),
        }
    }
    pub fn item(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn order(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlOrderByExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlOrderByExpressionFields {
    pub item: SyntaxResult<AnyPsqlExpression>,
    pub order: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlParenthesizedExpressionFields {
        PsqlParenthesizedExpressionFields {
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlParenthesizedExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlParenthesizedExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyPsqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlReturningClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlReturningClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlReturningClauseFields {
        PsqlReturningClauseFields {
            returning_token: self.returning_token(),
            items: self.items(),
        }
    }
    pub fn returning_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlSelectItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlReturningClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlReturningClauseFields {
    pub returning_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlSelectItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlRootFields {
        PsqlRootFields {
            stmt: self.stmt(),
            eof_token: self.eof_token(),
        }
    }
    pub fn stmt(&self) -> PsqlStatementList {
        support::list(&self.syntax, 0usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlRootFields {
    pub stmt: PsqlStatementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSelectClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSelectClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSelectClauseFields {
        PsqlSelectClauseFields {
            select_token: self.select_token(),
            list: self.list(),
        }
    }
    pub fn select_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn list(&self) -> PsqlSelectItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlSelectClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSelectClauseFields {
    pub select_token: SyntaxResult<SyntaxToken>,
    pub list: PsqlSelectItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSelectExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSelectExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSelectExpressionFields {
        PsqlSelectExpressionFields {
            expr: self.expr(),
            alias: self.alias(),
        }
    }
    pub fn expr(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn alias(&self) -> Option<PsqlAlias> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlSelectExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSelectExpressionFields {
    pub expr: SyntaxResult<AnyPsqlExpression>,
    pub alias: Option<PsqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSelectStatement {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSelectStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSelectStatementFields {
        PsqlSelectStatementFields {
            with_clause: self.with_clause(),
            select_clause: self.select_clause(),
            from_clause: self.from_clause(),
            where_clause: self.where_clause(),
            group_by_clause: self.group_by_clause(),
            having_clause: self.having_clause(),
            set_operations: self.set_operations(),
            order_by_clause: self.order_by_clause(),
            limit_clause: self.limit_clause(),
            offset_clause: self.offset_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<PsqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn select_clause(&self) -> SyntaxResult<PsqlSelectClause> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn from_clause(&self) -> Option<PsqlFromClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn where_clause(&self) -> Option<PsqlWhereClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn group_by_clause(&self) -> Option<PsqlGroupByClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn having_clause(&self) -> Option<PsqlHavingClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn set_operations(&self) -> PsqlSetOperationList {
        support::list(&self.syntax, 6usize)
    }
    pub fn order_by_clause(&self) -> Option<PsqlOrderByClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn limit_clause(&self) -> Option<PsqlLimitClause> {
        support::node(&self.syntax, 8usize)
    }
    pub fn offset_clause(&self) -> Option<PsqlOffsetClause> {
        support::node(&self.syntax, 9usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 10usize)
    }
}
impl Serialize for PsqlSelectStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSelectStatementFields {
    pub with_clause: Option<PsqlWithClause>,
    pub select_clause: SyntaxResult<PsqlSelectClause>,
    pub from_clause: Option<PsqlFromClause>,
    pub where_clause: Option<PsqlWhereClause>,
    pub group_by_clause: Option<PsqlGroupByClause>,
    pub having_clause: Option<PsqlHavingClause>,
    pub set_operations: PsqlSetOperationList,
    pub order_by_clause: Option<PsqlOrderByClause>,
    pub limit_clause: Option<PsqlLimitClause>,
    pub offset_clause: Option<PsqlOffsetClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSetClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSetClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSetClauseFields {
        PsqlSetClauseFields {
            set_token: self.set_token(),
            items: self.items(),
        }
    }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlSetItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlSetClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSetClauseFields {
    pub set_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlSetItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSetItem {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSetItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSetItemFields {
        PsqlSetItemFields {
            column: self.column(),
            eq_token: self.eq_token(),
            expr: self.expr(),
        }
    }
    pub fn column(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expr(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlSetItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSetItemFields {
    pub column: SyntaxResult<PsqlName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub expr: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSetOperation {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSetOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSetOperationFields {
        PsqlSetOperationFields {
            operator_token: self.operator_token(),
            quantifier: self.quantifier(),
            select_clause: self.select_clause(),
            from_clause: self.from_clause(),
            where_clause: self.where_clause(),
            group_by_clause: self.group_by_clause(),
            having_clause: self.having_clause(),
        }
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn quantifier(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn select_clause(&self) -> SyntaxResult<PsqlSelectClause> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn from_clause(&self) -> Option<PsqlFromClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn where_clause(&self) -> Option<PsqlWhereClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn group_by_clause(&self) -> Option<PsqlGroupByClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn having_clause(&self) -> Option<PsqlHavingClause> {
        support::node(&self.syntax, 6usize)
    }
}
impl Serialize for PsqlSetOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSetOperationFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub quantifier: Option<SyntaxToken>,
    pub select_clause: SyntaxResult<PsqlSelectClause>,
    pub from_clause: Option<PsqlFromClause>,
    pub where_clause: Option<PsqlWhereClause>,
    pub group_by_clause: Option<PsqlGroupByClause>,
    pub having_clause: Option<PsqlHavingClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlShemaName {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlShemaName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlShemaNameFields {
        PsqlShemaNameFields {
            base: self.base(),
            name: self.name(),
            dot_token: self.dot_token(),
        }
    }
    pub fn base(&self) -> Option<PsqlDataBaseName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlShemaName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlShemaNameFields {
    pub base: Option<PsqlDataBaseName>,
    pub name: SyntaxResult<PsqlName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlStar {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlStar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlStarFields {
        PsqlStarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlStar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlStarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlStringLiteralExpressionFields {
        PsqlStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for PsqlStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSubqueryBinding {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSubqueryBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSubqueryBindingFields {
        PsqlSubqueryBindingFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
            alias: self.alias(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<PsqlSelectStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn alias(&self) -> Option<PsqlAlias> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlSubqueryBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSubqueryBindingFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<PsqlSelectStatement>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub alias: Option<PsqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlSubqueryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlSubqueryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlSubqueryExpressionFields {
        PsqlSubqueryExpressionFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<PsqlSelectStatement> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlSubqueryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlSubqueryExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<PsqlSelectStatement>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTableBinding {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTableBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTableBindingFields {
        PsqlTableBindingFields {
            table: self.table(),
            alias: self.alias(),
        }
    }
    pub fn table(&self) -> SyntaxResult<PsqlTableName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn alias(&self) -> Option<PsqlAlias> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlTableBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTableBindingFields {
    pub table: SyntaxResult<PsqlTableName>,
    pub alias: Option<PsqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTableColReference {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTableColReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTableColReferenceFields {
        PsqlTableColReferenceFields {
            table: self.table(),
            dot_token: self.dot_token(),
            name: self.name(),
        }
    }
    pub fn table(&self) -> SyntaxResult<PsqlTableName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlTableColReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTableColReferenceFields {
    pub table: SyntaxResult<PsqlTableName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<PsqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTableName {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTableName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTableNameFields {
        PsqlTableNameFields {
            schema: self.schema(),
            name: self.name(),
        }
    }
    pub fn schema(&self) -> Option<PsqlShemaName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<PsqlName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlTableName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTableNameFields {
    pub schema: Option<PsqlShemaName>,
    pub name: SyntaxResult<PsqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTypeArguments {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTypeArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTypeArgumentsFields {
        PsqlTypeArgumentsFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlTypeArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlTypeArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTypeArgumentsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlTypeArgumentList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTypeArraySuffix {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTypeArraySuffix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTypeArraySuffixFields {
        PsqlTypeArraySuffixFields {
            l_brack_token: self.l_brack_token(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlTypeArraySuffix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTypeArraySuffixFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlTypeName {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlTypeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlTypeNameFields {
        PsqlTypeNameFields {
            name: self.name(),
            args: self.args(),
            array_suffix: self.array_suffix(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn args(&self) -> Option<PsqlTypeArguments> {
        support::node(&self.syntax, 1usize)
    }
    pub fn array_suffix(&self) -> Option<PsqlTypeArraySuffix> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlTypeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlTypeNameFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub args: Option<PsqlTypeArguments>,
    pub array_suffix: Option<PsqlTypeArraySuffix>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlUnaryExpressionFields {
        PsqlUnaryExpressionFields {
            operator_token: self.operator_token(),
            expression: self.expression(),
        }
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlUnaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlUnaryExpressionFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlUpdateStatement {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlUpdateStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlUpdateStatementFields {
        PsqlUpdateStatementFields {
            with_clause: self.with_clause(),
            update_token: self.update_token(),
            table: self.table(),
            set_clause: self.set_clause(),
            where_clause: self.where_clause(),
            returning_clause: self.returning_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<PsqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn update_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn table(&self) -> SyntaxResult<PsqlTableBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn set_clause(&self) -> SyntaxResult<PsqlSetClause> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn where_clause(&self) -> Option<PsqlWhereClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn returning_clause(&self) -> Option<PsqlReturningClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 6usize)
    }
}
impl Serialize for PsqlUpdateStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlUpdateStatementFields {
    pub with_clause: Option<PsqlWithClause>,
    pub update_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<PsqlTableBinding>,
    pub set_clause: SyntaxResult<PsqlSetClause>,
    pub where_clause: Option<PsqlWhereClause>,
    pub returning_clause: Option<PsqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlWhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlWhereClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlWhereClauseFields {
        PsqlWhereClauseFields {
            where_token: self.where_token(),
            condition: self.condition(),
        }
    }
    pub fn where_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnyPsqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlWhereClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlWhereClauseFields {
    pub where_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnyPsqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlWindowFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlWindowFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlWindowFunctionExpressionFields {
        PsqlWindowFunctionExpressionFields {
            call: self.call(),
            over_token: self.over_token(),
            window: self.window(),
        }
    }
    pub fn call(&self) -> SyntaxResult<PsqlCallExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn over_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn window(&self) -> SyntaxResult<PsqlWindowSpecification> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlWindowFunctionExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlWindowFunctionExpressionFields {
    pub call: SyntaxResult<PsqlCallExpression>,
    pub over_token: SyntaxResult<SyntaxToken>,
    pub window: SyntaxResult<PsqlWindowSpecification>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlWindowPartitionByClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlWindowPartitionByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlWindowPartitionByClauseFields {
        PsqlWindowPartitionByClauseFields {
            partition_by_token: self.partition_by_token(),
            items: self.items(),
        }
    }
    pub fn partition_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> PsqlWindowPartitionByItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for PsqlWindowPartitionByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlWindowPartitionByClauseFields {
    pub partition_by_token: SyntaxResult<SyntaxToken>,
    pub items: PsqlWindowPartitionByItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlWindowSpecification {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlWindowSpecification {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlWindowSpecificationFields {
        PsqlWindowSpecificationFields {
            l_paren_token: self.l_paren_token(),
            partition_by_clause: self.partition_by_clause(),
            order_by_clause: self.order_by_clause(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn partition_by_clause(&self) -> Option<PsqlWindowPartitionByClause> {
        support::node(&self.syntax, 1usize)
    }
    pub fn order_by_clause(&self) -> Option<PsqlOrderByClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for PsqlWindowSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlWindowSpecificationFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub partition_by_clause: Option<PsqlWindowPartitionByClause>,
    pub order_by_clause: Option<PsqlOrderByClause>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct PsqlWithClause {
    pub(crate) syntax: SyntaxNode,
}
impl PsqlWithClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> PsqlWithClauseFields {
        PsqlWithClauseFields {
            with_token: self.with_token(),
            recursive_token: self.recursive_token(),
            ctes: self.ctes(),
        }
    }
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn recursive_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn ctes(&self) -> PsqlCteDefinitionList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for PsqlWithClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct PsqlWithClauseFields {
    pub with_token: SyntaxResult<SyntaxToken>,
    pub recursive_token: Option<SyntaxToken>,
    pub ctes: PsqlCteDefinitionList,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlConflictAction {
    PsqlDoNothingClause(PsqlDoNothingClause),
    PsqlDoUpdateClause(PsqlDoUpdateClause),
}
impl AnyPsqlConflictAction {
    pub fn as_psql_do_nothing_clause(&self) -> Option<&PsqlDoNothingClause> {
        match &self {
            Self::PsqlDoNothingClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_do_update_clause(&self) -> Option<&PsqlDoUpdateClause> {
        match &self {
            Self::PsqlDoUpdateClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlConflictTarget {
    PsqlColumnList(PsqlColumnList),
    PsqlOnConstraintClause(PsqlOnConstraintClause),
}
impl AnyPsqlConflictTarget {
    pub fn as_psql_column_list(&self) -> Option<&PsqlColumnList> {
        match &self {
            Self::PsqlColumnList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_on_constraint_clause(&self) -> Option<&PsqlOnConstraintClause> {
        match &self {
            Self::PsqlOnConstraintClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlExpression {
    AnyPsqlLiteralExpression(AnyPsqlLiteralExpression),
    PsqlArrayExpression(PsqlArrayExpression),
    PsqlArraySubscriptExpression(PsqlArraySubscriptExpression),
    PsqlBetweenExpression(PsqlBetweenExpression),
    PsqlBinaryExpression(PsqlBinaryExpression),
    PsqlCallExpression(PsqlCallExpression),
    PsqlCaseExpression(PsqlCaseExpression),
    PsqlCastExpression(PsqlCastExpression),
    PsqlCastFunctionExpression(PsqlCastFunctionExpression),
    PsqlColReference(PsqlColReference),
    PsqlInExpression(PsqlInExpression),
    PsqlIsNullExpression(PsqlIsNullExpression),
    PsqlLikeExpression(PsqlLikeExpression),
    PsqlLogicalExpression(PsqlLogicalExpression),
    PsqlName(PsqlName),
    PsqlParenthesizedExpression(PsqlParenthesizedExpression),
    PsqlStar(PsqlStar),
    PsqlSubqueryExpression(PsqlSubqueryExpression),
    PsqlTableColReference(PsqlTableColReference),
    PsqlUnaryExpression(PsqlUnaryExpression),
    PsqlWindowFunctionExpression(PsqlWindowFunctionExpression),
}
impl AnyPsqlExpression {
    pub fn as_any_psql_literal_expression(&self) -> Option<&AnyPsqlLiteralExpression> {
        match &self {
            Self::AnyPsqlLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_array_expression(&self) -> Option<&PsqlArrayExpression> {
        match &self {
            Self::PsqlArrayExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_array_subscript_expression(&self) -> Option<&PsqlArraySubscriptExpression> {
        match &self {
            Self::PsqlArraySubscriptExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_between_expression(&self) -> Option<&PsqlBetweenExpression> {
        match &self {
            Self::PsqlBetweenExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_binary_expression(&self) -> Option<&PsqlBinaryExpression> {
        match &self {
            Self::PsqlBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_call_expression(&self) -> Option<&PsqlCallExpression> {
        match &self {
            Self::PsqlCallExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_case_expression(&self) -> Option<&PsqlCaseExpression> {
        match &self {
            Self::PsqlCaseExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_cast_expression(&self) -> Option<&PsqlCastExpression> {
        match &self {
            Self::PsqlCastExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_cast_function_expression(&self) -> Option<&PsqlCastFunctionExpression> {
        match &self {
            Self::PsqlCastFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_col_reference(&self) -> Option<&PsqlColReference> {
        match &self {
            Self::PsqlColReference(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_in_expression(&self) -> Option<&PsqlInExpression> {
        match &self {
            Self::PsqlInExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_is_null_expression(&self) -> Option<&PsqlIsNullExpression> {
        match &self {
            Self::PsqlIsNullExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_like_expression(&self) -> Option<&PsqlLikeExpression> {
        match &self {
            Self::PsqlLikeExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_logical_expression(&self) -> Option<&PsqlLogicalExpression> {
        match &self {
            Self::PsqlLogicalExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_name(&self) -> Option<&PsqlName> {
        match &self {
            Self::PsqlName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_parenthesized_expression(&self) -> Option<&PsqlParenthesizedExpression> {
        match &self {
            Self::PsqlParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_star(&self) -> Option<&PsqlStar> {
        match &self {
            Self::PsqlStar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_subquery_expression(&self) -> Option<&PsqlSubqueryExpression> {
        match &self {
            Self::PsqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_table_col_reference(&self) -> Option<&PsqlTableColReference> {
        match &self {
            Self::PsqlTableColReference(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_unary_expression(&self) -> Option<&PsqlUnaryExpression> {
        match &self {
            Self::PsqlUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_window_function_expression(&self) -> Option<&PsqlWindowFunctionExpression> {
        match &self {
            Self::PsqlWindowFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlFromExpression {
    PsqlFunctionBinding(PsqlFunctionBinding),
    PsqlSubqueryBinding(PsqlSubqueryBinding),
    PsqlTableBinding(PsqlTableBinding),
}
impl AnyPsqlFromExpression {
    pub fn as_psql_function_binding(&self) -> Option<&PsqlFunctionBinding> {
        match &self {
            Self::PsqlFunctionBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_subquery_binding(&self) -> Option<&PsqlSubqueryBinding> {
        match &self {
            Self::PsqlSubqueryBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_table_binding(&self) -> Option<&PsqlTableBinding> {
        match &self {
            Self::PsqlTableBinding(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlInSource {
    PsqlInValueList(PsqlInValueList),
    PsqlSubqueryExpression(PsqlSubqueryExpression),
}
impl AnyPsqlInSource {
    pub fn as_psql_in_value_list(&self) -> Option<&PsqlInValueList> {
        match &self {
            Self::PsqlInValueList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_subquery_expression(&self) -> Option<&PsqlSubqueryExpression> {
        match &self {
            Self::PsqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlInsertSource {
    PsqlInsertValues(PsqlInsertValues),
    PsqlSelectStatement(PsqlSelectStatement),
}
impl AnyPsqlInsertSource {
    pub fn as_psql_insert_values(&self) -> Option<&PsqlInsertValues> {
        match &self {
            Self::PsqlInsertValues(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_select_statement(&self) -> Option<&PsqlSelectStatement> {
        match &self {
            Self::PsqlSelectStatement(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlLiteralExpression {
    PsqlBooleanLiteralExpression(PsqlBooleanLiteralExpression),
    PsqlNullLiteralExpression(PsqlNullLiteralExpression),
    PsqlNumberLiteralExpression(PsqlNumberLiteralExpression),
    PsqlStringLiteralExpression(PsqlStringLiteralExpression),
}
impl AnyPsqlLiteralExpression {
    pub fn as_psql_boolean_literal_expression(&self) -> Option<&PsqlBooleanLiteralExpression> {
        match &self {
            Self::PsqlBooleanLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_null_literal_expression(&self) -> Option<&PsqlNullLiteralExpression> {
        match &self {
            Self::PsqlNullLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_number_literal_expression(&self) -> Option<&PsqlNumberLiteralExpression> {
        match &self {
            Self::PsqlNumberLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_string_literal_expression(&self) -> Option<&PsqlStringLiteralExpression> {
        match &self {
            Self::PsqlStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlSelectItem {
    PsqlSelectExpression(PsqlSelectExpression),
    PsqlStar(PsqlStar),
}
impl AnyPsqlSelectItem {
    pub fn as_psql_select_expression(&self) -> Option<&PsqlSelectExpression> {
        match &self {
            Self::PsqlSelectExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_star(&self) -> Option<&PsqlStar> {
        match &self {
            Self::PsqlStar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnyPsqlStatement {
    PsqlDeleteStatement(PsqlDeleteStatement),
    PsqlInsertStatement(PsqlInsertStatement),
    PsqlSelectStatement(PsqlSelectStatement),
    PsqlUpdateStatement(PsqlUpdateStatement),
}
impl AnyPsqlStatement {
    pub fn as_psql_delete_statement(&self) -> Option<&PsqlDeleteStatement> {
        match &self {
            Self::PsqlDeleteStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_insert_statement(&self) -> Option<&PsqlInsertStatement> {
        match &self {
            Self::PsqlInsertStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_select_statement(&self) -> Option<&PsqlSelectStatement> {
        match &self {
            Self::PsqlSelectStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_psql_update_statement(&self) -> Option<&PsqlUpdateStatement> {
        match &self {
            Self::PsqlUpdateStatement(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for PsqlAlias {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ALIAS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ALIAS
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
impl std::fmt::Debug for PsqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlAlias")
                .field("as_token", &support::DebugOptionalElement(self.as_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("PsqlAlias").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlAlias> for SyntaxNode {
    fn from(n: PsqlAlias) -> Self {
        n.syntax
    }
}
impl From<PsqlAlias> for SyntaxElement {
    fn from(n: PsqlAlias) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlArrayExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ARRAY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ARRAY_EXPRESSION
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
impl std::fmt::Debug for PsqlArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlArrayExpression")
                .field(
                    "array_token",
                    &support::DebugSyntaxResult(self.array_token()),
                )
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("items", &self.items())
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlArrayExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlArrayExpression> for SyntaxNode {
    fn from(n: PsqlArrayExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlArrayExpression> for SyntaxElement {
    fn from(n: PsqlArrayExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlArraySubscriptExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ARRAY_SUBSCRIPT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ARRAY_SUBSCRIPT_EXPRESSION
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
impl std::fmt::Debug for PsqlArraySubscriptExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlArraySubscriptExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field("index", &support::DebugSyntaxResult(self.index()))
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlArraySubscriptExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlArraySubscriptExpression> for SyntaxNode {
    fn from(n: PsqlArraySubscriptExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlArraySubscriptExpression> for SyntaxElement {
    fn from(n: PsqlArraySubscriptExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlBetweenExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BETWEEN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BETWEEN_EXPRESSION
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
impl std::fmt::Debug for PsqlBetweenExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlBetweenExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field(
                    "between_token",
                    &support::DebugSyntaxResult(self.between_token()),
                )
                .field("low", &support::DebugSyntaxResult(self.low()))
                .field("and_token", &support::DebugSyntaxResult(self.and_token()))
                .field("high", &support::DebugSyntaxResult(self.high()))
                .finish()
        } else {
            f.debug_struct("PsqlBetweenExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlBetweenExpression> for SyntaxNode {
    fn from(n: PsqlBetweenExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlBetweenExpression> for SyntaxElement {
    fn from(n: PsqlBetweenExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlBinaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BINARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BINARY_EXPRESSION
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
impl std::fmt::Debug for PsqlBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlBinaryExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("PsqlBinaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlBinaryExpression> for SyntaxNode {
    fn from(n: PsqlBinaryExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlBinaryExpression> for SyntaxElement {
    fn from(n: PsqlBinaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlBooleanLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOOLEAN_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOOLEAN_LITERAL_EXPRESSION
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
impl std::fmt::Debug for PsqlBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlBooleanLiteralExpression")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("PsqlBooleanLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlBooleanLiteralExpression> for SyntaxNode {
    fn from(n: PsqlBooleanLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlBooleanLiteralExpression> for SyntaxElement {
    fn from(n: PsqlBooleanLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCallExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CALL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CALL_EXPRESSION
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
impl std::fmt::Debug for PsqlCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCallExpression")
                .field("schema", &support::DebugOptionalElement(self.schema()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("arguments", &self.arguments())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlCallExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCallExpression> for SyntaxNode {
    fn from(n: PsqlCallExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlCallExpression> for SyntaxElement {
    fn from(n: PsqlCallExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCaseElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CASE_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CASE_ELSE_CLAUSE
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
impl std::fmt::Debug for PsqlCaseElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCaseElseClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("result", &support::DebugSyntaxResult(self.result()))
                .finish()
        } else {
            f.debug_struct("PsqlCaseElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCaseElseClause> for SyntaxNode {
    fn from(n: PsqlCaseElseClause) -> Self {
        n.syntax
    }
}
impl From<PsqlCaseElseClause> for SyntaxElement {
    fn from(n: PsqlCaseElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCaseExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CASE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CASE_EXPRESSION
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
impl std::fmt::Debug for PsqlCaseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCaseExpression")
                .field("case_token", &support::DebugSyntaxResult(self.case_token()))
                .field(
                    "expression",
                    &support::DebugOptionalElement(self.expression()),
                )
                .field("when_clauses", &self.when_clauses())
                .field(
                    "else_clause",
                    &support::DebugOptionalElement(self.else_clause()),
                )
                .field("end_token", &support::DebugSyntaxResult(self.end_token()))
                .finish()
        } else {
            f.debug_struct("PsqlCaseExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCaseExpression> for SyntaxNode {
    fn from(n: PsqlCaseExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlCaseExpression> for SyntaxElement {
    fn from(n: PsqlCaseExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCaseWhenClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CASE_WHEN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CASE_WHEN_CLAUSE
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
impl std::fmt::Debug for PsqlCaseWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCaseWhenClause")
                .field("when_token", &support::DebugSyntaxResult(self.when_token()))
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field("then_token", &support::DebugSyntaxResult(self.then_token()))
                .field("result", &support::DebugSyntaxResult(self.result()))
                .finish()
        } else {
            f.debug_struct("PsqlCaseWhenClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCaseWhenClause> for SyntaxNode {
    fn from(n: PsqlCaseWhenClause) -> Self {
        n.syntax
    }
}
impl From<PsqlCaseWhenClause> for SyntaxElement {
    fn from(n: PsqlCaseWhenClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCastExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CAST_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CAST_EXPRESSION
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
impl std::fmt::Debug for PsqlCastExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCastExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "double_colon_token",
                    &support::DebugSyntaxResult(self.double_colon_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("PsqlCastExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCastExpression> for SyntaxNode {
    fn from(n: PsqlCastExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlCastExpression> for SyntaxElement {
    fn from(n: PsqlCastExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCastFunctionExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CAST_FUNCTION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CAST_FUNCTION_EXPRESSION
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
impl std::fmt::Debug for PsqlCastFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCastFunctionExpression")
                .field("cast_token", &support::DebugSyntaxResult(self.cast_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlCastFunctionExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCastFunctionExpression> for SyntaxNode {
    fn from(n: PsqlCastFunctionExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlCastFunctionExpression> for SyntaxElement {
    fn from(n: PsqlCastFunctionExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlColReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_COL_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_COL_REFERENCE
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
impl std::fmt::Debug for PsqlColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlColReference")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("PsqlColReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlColReference> for SyntaxNode {
    fn from(n: PsqlColReference) -> Self {
        n.syntax
    }
}
impl From<PsqlColReference> for SyntaxElement {
    fn from(n: PsqlColReference) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlColumnList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_COLUMN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_COLUMN_LIST
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
impl std::fmt::Debug for PsqlColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlColumnList")
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
            f.debug_struct("PsqlColumnList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlColumnList> for SyntaxNode {
    fn from(n: PsqlColumnList) -> Self {
        n.syntax
    }
}
impl From<PsqlColumnList> for SyntaxElement {
    fn from(n: PsqlColumnList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlCteDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CTE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CTE_DEFINITION
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
impl std::fmt::Debug for PsqlCteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlCteDefinition")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("columns", &support::DebugOptionalElement(self.columns()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlCteDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlCteDefinition> for SyntaxNode {
    fn from(n: PsqlCteDefinition) -> Self {
        n.syntax
    }
}
impl From<PsqlCteDefinition> for SyntaxElement {
    fn from(n: PsqlCteDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlDataBaseName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_DATA_BASE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_DATA_BASE_NAME
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
impl std::fmt::Debug for PsqlDataBaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlDataBaseName")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .finish()
        } else {
            f.debug_struct("PsqlDataBaseName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlDataBaseName> for SyntaxNode {
    fn from(n: PsqlDataBaseName) -> Self {
        n.syntax
    }
}
impl From<PsqlDataBaseName> for SyntaxElement {
    fn from(n: PsqlDataBaseName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlDeleteStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_DELETE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_DELETE_STATEMENT
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
impl std::fmt::Debug for PsqlDeleteStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlDeleteStatement")
                .field(
                    "with_clause",
                    &support::DebugOptionalElement(self.with_clause()),
                )
                .field(
                    "delete_token",
                    &support::DebugSyntaxResult(self.delete_token()),
                )
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("using", &support::DebugOptionalElement(self.using()))
                .field(
                    "where_clause",
                    &support::DebugOptionalElement(self.where_clause()),
                )
                .field(
                    "returning_clause",
                    &support::DebugOptionalElement(self.returning_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlDeleteStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlDeleteStatement> for SyntaxNode {
    fn from(n: PsqlDeleteStatement) -> Self {
        n.syntax
    }
}
impl From<PsqlDeleteStatement> for SyntaxElement {
    fn from(n: PsqlDeleteStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlDeleteUsingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_DELETE_USING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_DELETE_USING_CLAUSE
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
impl std::fmt::Debug for PsqlDeleteUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlDeleteUsingClause")
                .field(
                    "using_token",
                    &support::DebugSyntaxResult(self.using_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlDeleteUsingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlDeleteUsingClause> for SyntaxNode {
    fn from(n: PsqlDeleteUsingClause) -> Self {
        n.syntax
    }
}
impl From<PsqlDeleteUsingClause> for SyntaxElement {
    fn from(n: PsqlDeleteUsingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlDoNothingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_DO_NOTHING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_DO_NOTHING_CLAUSE
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
impl std::fmt::Debug for PsqlDoNothingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlDoNothingClause")
                .field("do_token", &support::DebugSyntaxResult(self.do_token()))
                .field(
                    "nothing_token",
                    &support::DebugSyntaxResult(self.nothing_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlDoNothingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlDoNothingClause> for SyntaxNode {
    fn from(n: PsqlDoNothingClause) -> Self {
        n.syntax
    }
}
impl From<PsqlDoNothingClause> for SyntaxElement {
    fn from(n: PsqlDoNothingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlDoUpdateClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_DO_UPDATE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_DO_UPDATE_CLAUSE
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
impl std::fmt::Debug for PsqlDoUpdateClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlDoUpdateClause")
                .field("do_token", &support::DebugSyntaxResult(self.do_token()))
                .field(
                    "update_token",
                    &support::DebugSyntaxResult(self.update_token()),
                )
                .field("set_clause", &support::DebugSyntaxResult(self.set_clause()))
                .field(
                    "where_clause",
                    &support::DebugOptionalElement(self.where_clause()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlDoUpdateClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlDoUpdateClause> for SyntaxNode {
    fn from(n: PsqlDoUpdateClause) -> Self {
        n.syntax
    }
}
impl From<PsqlDoUpdateClause> for SyntaxElement {
    fn from(n: PsqlDoUpdateClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlFromClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_FROM_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_FROM_CLAUSE
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
impl std::fmt::Debug for PsqlFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlFromClause")
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlFromClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlFromClause> for SyntaxNode {
    fn from(n: PsqlFromClause) -> Self {
        n.syntax
    }
}
impl From<PsqlFromClause> for SyntaxElement {
    fn from(n: PsqlFromClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlFromItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_FROM_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_FROM_ITEM
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
impl std::fmt::Debug for PsqlFromItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlFromItem")
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field("joins", &self.joins())
                .finish()
        } else {
            f.debug_struct("PsqlFromItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlFromItem> for SyntaxNode {
    fn from(n: PsqlFromItem) -> Self {
        n.syntax
    }
}
impl From<PsqlFromItem> for SyntaxElement {
    fn from(n: PsqlFromItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlFunctionBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_FUNCTION_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_FUNCTION_BINDING
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
impl std::fmt::Debug for PsqlFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlFunctionBinding")
                .field("schema", &support::DebugOptionalElement(self.schema()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("arguments", &self.arguments())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("PsqlFunctionBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlFunctionBinding> for SyntaxNode {
    fn from(n: PsqlFunctionBinding) -> Self {
        n.syntax
    }
}
impl From<PsqlFunctionBinding> for SyntaxElement {
    fn from(n: PsqlFunctionBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlGroupByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_GROUP_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_GROUP_BY_CLAUSE
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
impl std::fmt::Debug for PsqlGroupByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlGroupByClause")
                .field(
                    "group_by_token",
                    &support::DebugSyntaxResult(self.group_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlGroupByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlGroupByClause> for SyntaxNode {
    fn from(n: PsqlGroupByClause) -> Self {
        n.syntax
    }
}
impl From<PsqlGroupByClause> for SyntaxElement {
    fn from(n: PsqlGroupByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlHavingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_HAVING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_HAVING_CLAUSE
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
impl std::fmt::Debug for PsqlHavingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlHavingClause")
                .field(
                    "having_token",
                    &support::DebugSyntaxResult(self.having_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("PsqlHavingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlHavingClause> for SyntaxNode {
    fn from(n: PsqlHavingClause) -> Self {
        n.syntax
    }
}
impl From<PsqlHavingClause> for SyntaxElement {
    fn from(n: PsqlHavingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlInExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_IN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_IN_EXPRESSION
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
impl std::fmt::Debug for PsqlInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlInExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field("in_token", &support::DebugSyntaxResult(self.in_token()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("PsqlInExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlInExpression> for SyntaxNode {
    fn from(n: PsqlInExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlInExpression> for SyntaxElement {
    fn from(n: PsqlInExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlInValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_IN_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_IN_VALUE_LIST
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
impl std::fmt::Debug for PsqlInValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlInValueList")
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
            f.debug_struct("PsqlInValueList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlInValueList> for SyntaxNode {
    fn from(n: PsqlInValueList) -> Self {
        n.syntax
    }
}
impl From<PsqlInValueList> for SyntaxElement {
    fn from(n: PsqlInValueList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlInsertStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_INSERT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_INSERT_STATEMENT
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
impl std::fmt::Debug for PsqlInsertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlInsertStatement")
                .field(
                    "with_clause",
                    &support::DebugOptionalElement(self.with_clause()),
                )
                .field(
                    "insert_token",
                    &support::DebugSyntaxResult(self.insert_token()),
                )
                .field("into_token", &support::DebugSyntaxResult(self.into_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("columns", &support::DebugOptionalElement(self.columns()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field(
                    "on_conflict_clause",
                    &support::DebugOptionalElement(self.on_conflict_clause()),
                )
                .field(
                    "returning_clause",
                    &support::DebugOptionalElement(self.returning_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlInsertStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlInsertStatement> for SyntaxNode {
    fn from(n: PsqlInsertStatement) -> Self {
        n.syntax
    }
}
impl From<PsqlInsertStatement> for SyntaxElement {
    fn from(n: PsqlInsertStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlInsertValues {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_INSERT_VALUES as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_INSERT_VALUES
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
impl std::fmt::Debug for PsqlInsertValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlInsertValues")
                .field(
                    "values_token",
                    &support::DebugSyntaxResult(self.values_token()),
                )
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
            f.debug_struct("PsqlInsertValues").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlInsertValues> for SyntaxNode {
    fn from(n: PsqlInsertValues) -> Self {
        n.syntax
    }
}
impl From<PsqlInsertValues> for SyntaxElement {
    fn from(n: PsqlInsertValues) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlIsNullExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_IS_NULL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_IS_NULL_EXPRESSION
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
impl std::fmt::Debug for PsqlIsNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlIsNullExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field("is_token", &support::DebugSyntaxResult(self.is_token()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field("null_token", &support::DebugSyntaxResult(self.null_token()))
                .finish()
        } else {
            f.debug_struct("PsqlIsNullExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlIsNullExpression> for SyntaxNode {
    fn from(n: PsqlIsNullExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlIsNullExpression> for SyntaxElement {
    fn from(n: PsqlIsNullExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlJoinClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_JOIN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_JOIN_CLAUSE
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
impl std::fmt::Debug for PsqlJoinClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlJoinClause")
                .field(
                    "join_type",
                    &support::DebugOptionalElement(self.join_type()),
                )
                .field(
                    "outer_token",
                    &support::DebugOptionalElement(self.outer_token()),
                )
                .field("join_token", &support::DebugSyntaxResult(self.join_token()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field("on_token", &support::DebugOptionalElement(self.on_token()))
                .field(
                    "condition",
                    &support::DebugOptionalElement(self.condition()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlJoinClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlJoinClause> for SyntaxNode {
    fn from(n: PsqlJoinClause) -> Self {
        n.syntax
    }
}
impl From<PsqlJoinClause> for SyntaxElement {
    fn from(n: PsqlJoinClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlLikeExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_LIKE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_LIKE_EXPRESSION
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
impl std::fmt::Debug for PsqlLikeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlLikeExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("pattern", &support::DebugSyntaxResult(self.pattern()))
                .finish()
        } else {
            f.debug_struct("PsqlLikeExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlLikeExpression> for SyntaxNode {
    fn from(n: PsqlLikeExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlLikeExpression> for SyntaxElement {
    fn from(n: PsqlLikeExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlLimitClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_LIMIT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_LIMIT_CLAUSE
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
impl std::fmt::Debug for PsqlLimitClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlLimitClause")
                .field(
                    "limit_token",
                    &support::DebugSyntaxResult(self.limit_token()),
                )
                .field(
                    "limit_count",
                    &support::DebugSyntaxResult(self.limit_count()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlLimitClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlLimitClause> for SyntaxNode {
    fn from(n: PsqlLimitClause) -> Self {
        n.syntax
    }
}
impl From<PsqlLimitClause> for SyntaxElement {
    fn from(n: PsqlLimitClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlLogicalExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_LOGICAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_LOGICAL_EXPRESSION
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
impl std::fmt::Debug for PsqlLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlLogicalExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("PsqlLogicalExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlLogicalExpression> for SyntaxNode {
    fn from(n: PsqlLogicalExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlLogicalExpression> for SyntaxElement {
    fn from(n: PsqlLogicalExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_NAME
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
impl std::fmt::Debug for PsqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlName> for SyntaxNode {
    fn from(n: PsqlName) -> Self {
        n.syntax
    }
}
impl From<PsqlName> for SyntaxElement {
    fn from(n: PsqlName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlNullLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_NULL_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_NULL_LITERAL_EXPRESSION
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
impl std::fmt::Debug for PsqlNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlNullLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlNullLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlNullLiteralExpression> for SyntaxNode {
    fn from(n: PsqlNullLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlNullLiteralExpression> for SyntaxElement {
    fn from(n: PsqlNullLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlNumberLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_NUMBER_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_NUMBER_LITERAL_EXPRESSION
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
impl std::fmt::Debug for PsqlNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlNumberLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlNumberLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlNumberLiteralExpression> for SyntaxNode {
    fn from(n: PsqlNumberLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlNumberLiteralExpression> for SyntaxElement {
    fn from(n: PsqlNumberLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlOffsetClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_OFFSET_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_OFFSET_CLAUSE
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
impl std::fmt::Debug for PsqlOffsetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlOffsetClause")
                .field(
                    "offset_token",
                    &support::DebugSyntaxResult(self.offset_token()),
                )
                .field("start", &support::DebugSyntaxResult(self.start()))
                .finish()
        } else {
            f.debug_struct("PsqlOffsetClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlOffsetClause> for SyntaxNode {
    fn from(n: PsqlOffsetClause) -> Self {
        n.syntax
    }
}
impl From<PsqlOffsetClause> for SyntaxElement {
    fn from(n: PsqlOffsetClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlOnConflictClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ON_CONFLICT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ON_CONFLICT_CLAUSE
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
impl std::fmt::Debug for PsqlOnConflictClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlOnConflictClause")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "conflict_token",
                    &support::DebugSyntaxResult(self.conflict_token()),
                )
                .field("target", &support::DebugOptionalElement(self.target()))
                .field("action", &support::DebugSyntaxResult(self.action()))
                .finish()
        } else {
            f.debug_struct("PsqlOnConflictClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlOnConflictClause> for SyntaxNode {
    fn from(n: PsqlOnConflictClause) -> Self {
        n.syntax
    }
}
impl From<PsqlOnConflictClause> for SyntaxElement {
    fn from(n: PsqlOnConflictClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlOnConstraintClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ON_CONSTRAINT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ON_CONSTRAINT_CLAUSE
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
impl std::fmt::Debug for PsqlOnConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlOnConstraintClause")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "constraint_token",
                    &support::DebugSyntaxResult(self.constraint_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("PsqlOnConstraintClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlOnConstraintClause> for SyntaxNode {
    fn from(n: PsqlOnConstraintClause) -> Self {
        n.syntax
    }
}
impl From<PsqlOnConstraintClause> for SyntaxElement {
    fn from(n: PsqlOnConstraintClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlOrderByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ORDER_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ORDER_BY_CLAUSE
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
impl std::fmt::Debug for PsqlOrderByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlOrderByClause")
                .field(
                    "order_by_token",
                    &support::DebugSyntaxResult(self.order_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlOrderByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlOrderByClause> for SyntaxNode {
    fn from(n: PsqlOrderByClause) -> Self {
        n.syntax
    }
}
impl From<PsqlOrderByClause> for SyntaxElement {
    fn from(n: PsqlOrderByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlOrderByExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ORDER_BY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ORDER_BY_EXPRESSION
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
impl std::fmt::Debug for PsqlOrderByExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlOrderByExpression")
                .field("item", &support::DebugSyntaxResult(self.item()))
                .field("order", &support::DebugOptionalElement(self.order()))
                .finish()
        } else {
            f.debug_struct("PsqlOrderByExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlOrderByExpression> for SyntaxNode {
    fn from(n: PsqlOrderByExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlOrderByExpression> for SyntaxElement {
    fn from(n: PsqlOrderByExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlParenthesizedExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_PARENTHESIZED_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_PARENTHESIZED_EXPRESSION
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
impl std::fmt::Debug for PsqlParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlParenthesizedExpression")
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
            f.debug_struct("PsqlParenthesizedExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlParenthesizedExpression> for SyntaxNode {
    fn from(n: PsqlParenthesizedExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlParenthesizedExpression> for SyntaxElement {
    fn from(n: PsqlParenthesizedExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlReturningClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_RETURNING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_RETURNING_CLAUSE
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
impl std::fmt::Debug for PsqlReturningClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlReturningClause")
                .field(
                    "returning_token",
                    &support::DebugSyntaxResult(self.returning_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlReturningClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlReturningClause> for SyntaxNode {
    fn from(n: PsqlReturningClause) -> Self {
        n.syntax
    }
}
impl From<PsqlReturningClause> for SyntaxElement {
    fn from(n: PsqlReturningClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ROOT
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
impl std::fmt::Debug for PsqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlRoot")
                .field("stmt", &self.stmt())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("PsqlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlRoot> for SyntaxNode {
    fn from(n: PsqlRoot) -> Self {
        n.syntax
    }
}
impl From<PsqlRoot> for SyntaxElement {
    fn from(n: PsqlRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSelectClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SELECT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SELECT_CLAUSE
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
impl std::fmt::Debug for PsqlSelectClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSelectClause")
                .field(
                    "select_token",
                    &support::DebugSyntaxResult(self.select_token()),
                )
                .field("list", &self.list())
                .finish()
        } else {
            f.debug_struct("PsqlSelectClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSelectClause> for SyntaxNode {
    fn from(n: PsqlSelectClause) -> Self {
        n.syntax
    }
}
impl From<PsqlSelectClause> for SyntaxElement {
    fn from(n: PsqlSelectClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSelectExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SELECT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SELECT_EXPRESSION
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
impl std::fmt::Debug for PsqlSelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSelectExpression")
                .field("expr", &support::DebugSyntaxResult(self.expr()))
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("PsqlSelectExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSelectExpression> for SyntaxNode {
    fn from(n: PsqlSelectExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlSelectExpression> for SyntaxElement {
    fn from(n: PsqlSelectExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSelectStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SELECT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SELECT_STATEMENT
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
impl std::fmt::Debug for PsqlSelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSelectStatement")
                .field(
                    "with_clause",
                    &support::DebugOptionalElement(self.with_clause()),
                )
                .field(
                    "select_clause",
                    &support::DebugSyntaxResult(self.select_clause()),
                )
                .field(
                    "from_clause",
                    &support::DebugOptionalElement(self.from_clause()),
                )
                .field(
                    "where_clause",
                    &support::DebugOptionalElement(self.where_clause()),
                )
                .field(
                    "group_by_clause",
                    &support::DebugOptionalElement(self.group_by_clause()),
                )
                .field(
                    "having_clause",
                    &support::DebugOptionalElement(self.having_clause()),
                )
                .field("set_operations", &self.set_operations())
                .field(
                    "order_by_clause",
                    &support::DebugOptionalElement(self.order_by_clause()),
                )
                .field(
                    "limit_clause",
                    &support::DebugOptionalElement(self.limit_clause()),
                )
                .field(
                    "offset_clause",
                    &support::DebugOptionalElement(self.offset_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlSelectStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSelectStatement> for SyntaxNode {
    fn from(n: PsqlSelectStatement) -> Self {
        n.syntax
    }
}
impl From<PsqlSelectStatement> for SyntaxElement {
    fn from(n: PsqlSelectStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSetClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SET_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SET_CLAUSE
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
impl std::fmt::Debug for PsqlSetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSetClause")
                .field("set_token", &support::DebugSyntaxResult(self.set_token()))
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlSetClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSetClause> for SyntaxNode {
    fn from(n: PsqlSetClause) -> Self {
        n.syntax
    }
}
impl From<PsqlSetClause> for SyntaxElement {
    fn from(n: PsqlSetClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSetItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SET_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SET_ITEM
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
impl std::fmt::Debug for PsqlSetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSetItem")
                .field("column", &support::DebugSyntaxResult(self.column()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("expr", &support::DebugSyntaxResult(self.expr()))
                .finish()
        } else {
            f.debug_struct("PsqlSetItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSetItem> for SyntaxNode {
    fn from(n: PsqlSetItem) -> Self {
        n.syntax
    }
}
impl From<PsqlSetItem> for SyntaxElement {
    fn from(n: PsqlSetItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSetOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SET_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SET_OPERATION
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
impl std::fmt::Debug for PsqlSetOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSetOperation")
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field(
                    "quantifier",
                    &support::DebugOptionalElement(self.quantifier()),
                )
                .field(
                    "select_clause",
                    &support::DebugSyntaxResult(self.select_clause()),
                )
                .field(
                    "from_clause",
                    &support::DebugOptionalElement(self.from_clause()),
                )
                .field(
                    "where_clause",
                    &support::DebugOptionalElement(self.where_clause()),
                )
                .field(
                    "group_by_clause",
                    &support::DebugOptionalElement(self.group_by_clause()),
                )
                .field(
                    "having_clause",
                    &support::DebugOptionalElement(self.having_clause()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlSetOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSetOperation> for SyntaxNode {
    fn from(n: PsqlSetOperation) -> Self {
        n.syntax
    }
}
impl From<PsqlSetOperation> for SyntaxElement {
    fn from(n: PsqlSetOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlShemaName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SHEMA_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SHEMA_NAME
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
impl std::fmt::Debug for PsqlShemaName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlShemaName")
                .field("base", &support::DebugOptionalElement(self.base()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .finish()
        } else {
            f.debug_struct("PsqlShemaName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlShemaName> for SyntaxNode {
    fn from(n: PsqlShemaName) -> Self {
        n.syntax
    }
}
impl From<PsqlShemaName> for SyntaxElement {
    fn from(n: PsqlShemaName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlStar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_STAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_STAR
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
impl std::fmt::Debug for PsqlStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlStar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlStar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlStar> for SyntaxNode {
    fn from(n: PsqlStar) -> Self {
        n.syntax
    }
}
impl From<PsqlStar> for SyntaxElement {
    fn from(n: PsqlStar) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_STRING_LITERAL_EXPRESSION
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
impl std::fmt::Debug for PsqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlStringLiteralExpression> for SyntaxNode {
    fn from(n: PsqlStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlStringLiteralExpression> for SyntaxElement {
    fn from(n: PsqlStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSubqueryBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SUBQUERY_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SUBQUERY_BINDING
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
impl std::fmt::Debug for PsqlSubqueryBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSubqueryBinding")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("PsqlSubqueryBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSubqueryBinding> for SyntaxNode {
    fn from(n: PsqlSubqueryBinding) -> Self {
        n.syntax
    }
}
impl From<PsqlSubqueryBinding> for SyntaxElement {
    fn from(n: PsqlSubqueryBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlSubqueryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SUBQUERY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SUBQUERY_EXPRESSION
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
impl std::fmt::Debug for PsqlSubqueryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlSubqueryExpression")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlSubqueryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlSubqueryExpression> for SyntaxNode {
    fn from(n: PsqlSubqueryExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlSubqueryExpression> for SyntaxElement {
    fn from(n: PsqlSubqueryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTableBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TABLE_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TABLE_BINDING
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
impl std::fmt::Debug for PsqlTableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTableBinding")
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("PsqlTableBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTableBinding> for SyntaxNode {
    fn from(n: PsqlTableBinding) -> Self {
        n.syntax
    }
}
impl From<PsqlTableBinding> for SyntaxElement {
    fn from(n: PsqlTableBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTableColReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TABLE_COL_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TABLE_COL_REFERENCE
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
impl std::fmt::Debug for PsqlTableColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTableColReference")
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("PsqlTableColReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTableColReference> for SyntaxNode {
    fn from(n: PsqlTableColReference) -> Self {
        n.syntax
    }
}
impl From<PsqlTableColReference> for SyntaxElement {
    fn from(n: PsqlTableColReference) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTableName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TABLE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TABLE_NAME
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
impl std::fmt::Debug for PsqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTableName")
                .field("schema", &support::DebugOptionalElement(self.schema()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("PsqlTableName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTableName> for SyntaxNode {
    fn from(n: PsqlTableName) -> Self {
        n.syntax
    }
}
impl From<PsqlTableName> for SyntaxElement {
    fn from(n: PsqlTableName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTypeArguments {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TYPE_ARGUMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TYPE_ARGUMENTS
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
impl std::fmt::Debug for PsqlTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTypeArguments")
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
            f.debug_struct("PsqlTypeArguments").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTypeArguments> for SyntaxNode {
    fn from(n: PsqlTypeArguments) -> Self {
        n.syntax
    }
}
impl From<PsqlTypeArguments> for SyntaxElement {
    fn from(n: PsqlTypeArguments) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTypeArraySuffix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TYPE_ARRAY_SUFFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TYPE_ARRAY_SUFFIX
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
impl std::fmt::Debug for PsqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTypeArraySuffix")
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlTypeArraySuffix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTypeArraySuffix> for SyntaxNode {
    fn from(n: PsqlTypeArraySuffix) -> Self {
        n.syntax
    }
}
impl From<PsqlTypeArraySuffix> for SyntaxElement {
    fn from(n: PsqlTypeArraySuffix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlTypeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TYPE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TYPE_NAME
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
impl std::fmt::Debug for PsqlTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlTypeName")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("args", &support::DebugOptionalElement(self.args()))
                .field(
                    "array_suffix",
                    &support::DebugOptionalElement(self.array_suffix()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlTypeName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlTypeName> for SyntaxNode {
    fn from(n: PsqlTypeName) -> Self {
        n.syntax
    }
}
impl From<PsqlTypeName> for SyntaxElement {
    fn from(n: PsqlTypeName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlUnaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_UNARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_UNARY_EXPRESSION
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
impl std::fmt::Debug for PsqlUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlUnaryExpression")
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .finish()
        } else {
            f.debug_struct("PsqlUnaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlUnaryExpression> for SyntaxNode {
    fn from(n: PsqlUnaryExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlUnaryExpression> for SyntaxElement {
    fn from(n: PsqlUnaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlUpdateStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_UPDATE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_UPDATE_STATEMENT
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
impl std::fmt::Debug for PsqlUpdateStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlUpdateStatement")
                .field(
                    "with_clause",
                    &support::DebugOptionalElement(self.with_clause()),
                )
                .field(
                    "update_token",
                    &support::DebugSyntaxResult(self.update_token()),
                )
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("set_clause", &support::DebugSyntaxResult(self.set_clause()))
                .field(
                    "where_clause",
                    &support::DebugOptionalElement(self.where_clause()),
                )
                .field(
                    "returning_clause",
                    &support::DebugOptionalElement(self.returning_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlUpdateStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlUpdateStatement> for SyntaxNode {
    fn from(n: PsqlUpdateStatement) -> Self {
        n.syntax
    }
}
impl From<PsqlUpdateStatement> for SyntaxElement {
    fn from(n: PsqlUpdateStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlWhereClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WHERE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WHERE_CLAUSE
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
impl std::fmt::Debug for PsqlWhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlWhereClause")
                .field(
                    "where_token",
                    &support::DebugSyntaxResult(self.where_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("PsqlWhereClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlWhereClause> for SyntaxNode {
    fn from(n: PsqlWhereClause) -> Self {
        n.syntax
    }
}
impl From<PsqlWhereClause> for SyntaxElement {
    fn from(n: PsqlWhereClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlWindowFunctionExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WINDOW_FUNCTION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WINDOW_FUNCTION_EXPRESSION
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
impl std::fmt::Debug for PsqlWindowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlWindowFunctionExpression")
                .field("call", &support::DebugSyntaxResult(self.call()))
                .field("over_token", &support::DebugSyntaxResult(self.over_token()))
                .field("window", &support::DebugSyntaxResult(self.window()))
                .finish()
        } else {
            f.debug_struct("PsqlWindowFunctionExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlWindowFunctionExpression> for SyntaxNode {
    fn from(n: PsqlWindowFunctionExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlWindowFunctionExpression> for SyntaxElement {
    fn from(n: PsqlWindowFunctionExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlWindowPartitionByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WINDOW_PARTITION_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WINDOW_PARTITION_BY_CLAUSE
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
impl std::fmt::Debug for PsqlWindowPartitionByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlWindowPartitionByClause")
                .field(
                    "partition_by_token",
                    &support::DebugSyntaxResult(self.partition_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("PsqlWindowPartitionByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlWindowPartitionByClause> for SyntaxNode {
    fn from(n: PsqlWindowPartitionByClause) -> Self {
        n.syntax
    }
}
impl From<PsqlWindowPartitionByClause> for SyntaxElement {
    fn from(n: PsqlWindowPartitionByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlWindowSpecification {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WINDOW_SPECIFICATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WINDOW_SPECIFICATION
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
impl std::fmt::Debug for PsqlWindowSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlWindowSpecification")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "partition_by_clause",
                    &support::DebugOptionalElement(self.partition_by_clause()),
                )
                .field(
                    "order_by_clause",
                    &support::DebugOptionalElement(self.order_by_clause()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("PsqlWindowSpecification").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlWindowSpecification> for SyntaxNode {
    fn from(n: PsqlWindowSpecification) -> Self {
        n.syntax
    }
}
impl From<PsqlWindowSpecification> for SyntaxElement {
    fn from(n: PsqlWindowSpecification) -> Self {
        n.syntax.into()
    }
}
impl AstNode for PsqlWithClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WITH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WITH_CLAUSE
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
impl std::fmt::Debug for PsqlWithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("PsqlWithClause")
                .field("with_token", &support::DebugSyntaxResult(self.with_token()))
                .field(
                    "recursive_token",
                    &support::DebugOptionalElement(self.recursive_token()),
                )
                .field("ctes", &self.ctes())
                .finish()
        } else {
            f.debug_struct("PsqlWithClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<PsqlWithClause> for SyntaxNode {
    fn from(n: PsqlWithClause) -> Self {
        n.syntax
    }
}
impl From<PsqlWithClause> for SyntaxElement {
    fn from(n: PsqlWithClause) -> Self {
        n.syntax.into()
    }
}
impl From<PsqlDoNothingClause> for AnyPsqlConflictAction {
    fn from(node: PsqlDoNothingClause) -> Self {
        Self::PsqlDoNothingClause(node)
    }
}
impl From<PsqlDoUpdateClause> for AnyPsqlConflictAction {
    fn from(node: PsqlDoUpdateClause) -> Self {
        Self::PsqlDoUpdateClause(node)
    }
}
impl AstNode for AnyPsqlConflictAction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        PsqlDoNothingClause::KIND_SET.union(PsqlDoUpdateClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, PSQL_DO_NOTHING_CLAUSE | PSQL_DO_UPDATE_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_DO_NOTHING_CLAUSE => Self::PsqlDoNothingClause(PsqlDoNothingClause { syntax }),
            PSQL_DO_UPDATE_CLAUSE => Self::PsqlDoUpdateClause(PsqlDoUpdateClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlDoNothingClause(it) => &it.syntax,
            Self::PsqlDoUpdateClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlDoNothingClause(it) => it.syntax,
            Self::PsqlDoUpdateClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlConflictAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlDoNothingClause(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlDoUpdateClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlConflictAction> for SyntaxNode {
    fn from(n: AnyPsqlConflictAction) -> Self {
        match n {
            AnyPsqlConflictAction::PsqlDoNothingClause(it) => it.into(),
            AnyPsqlConflictAction::PsqlDoUpdateClause(it) => it.into(),
        }
    }
}
impl From<AnyPsqlConflictAction> for SyntaxElement {
    fn from(n: AnyPsqlConflictAction) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlColumnList> for AnyPsqlConflictTarget {
    fn from(node: PsqlColumnList) -> Self {
        Self::PsqlColumnList(node)
    }
}
impl From<PsqlOnConstraintClause> for AnyPsqlConflictTarget {
    fn from(node: PsqlOnConstraintClause) -> Self {
        Self::PsqlOnConstraintClause(node)
    }
}
impl AstNode for AnyPsqlConflictTarget {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        PsqlColumnList::KIND_SET.union(PsqlOnConstraintClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, PSQL_COLUMN_LIST | PSQL_ON_CONSTRAINT_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_COLUMN_LIST => Self::PsqlColumnList(PsqlColumnList { syntax }),
            PSQL_ON_CONSTRAINT_CLAUSE => {
                Self::PsqlOnConstraintClause(PsqlOnConstraintClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlColumnList(it) => &it.syntax,
            Self::PsqlOnConstraintClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlColumnList(it) => it.syntax,
            Self::PsqlOnConstraintClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlConflictTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlColumnList(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlOnConstraintClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlConflictTarget> for SyntaxNode {
    fn from(n: AnyPsqlConflictTarget) -> Self {
        match n {
            AnyPsqlConflictTarget::PsqlColumnList(it) => it.into(),
            AnyPsqlConflictTarget::PsqlOnConstraintClause(it) => it.into(),
        }
    }
}
impl From<AnyPsqlConflictTarget> for SyntaxElement {
    fn from(n: AnyPsqlConflictTarget) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlArrayExpression> for AnyPsqlExpression {
    fn from(node: PsqlArrayExpression) -> Self {
        Self::PsqlArrayExpression(node)
    }
}
impl From<PsqlArraySubscriptExpression> for AnyPsqlExpression {
    fn from(node: PsqlArraySubscriptExpression) -> Self {
        Self::PsqlArraySubscriptExpression(node)
    }
}
impl From<PsqlBetweenExpression> for AnyPsqlExpression {
    fn from(node: PsqlBetweenExpression) -> Self {
        Self::PsqlBetweenExpression(node)
    }
}
impl From<PsqlBinaryExpression> for AnyPsqlExpression {
    fn from(node: PsqlBinaryExpression) -> Self {
        Self::PsqlBinaryExpression(node)
    }
}
impl From<PsqlCallExpression> for AnyPsqlExpression {
    fn from(node: PsqlCallExpression) -> Self {
        Self::PsqlCallExpression(node)
    }
}
impl From<PsqlCaseExpression> for AnyPsqlExpression {
    fn from(node: PsqlCaseExpression) -> Self {
        Self::PsqlCaseExpression(node)
    }
}
impl From<PsqlCastExpression> for AnyPsqlExpression {
    fn from(node: PsqlCastExpression) -> Self {
        Self::PsqlCastExpression(node)
    }
}
impl From<PsqlCastFunctionExpression> for AnyPsqlExpression {
    fn from(node: PsqlCastFunctionExpression) -> Self {
        Self::PsqlCastFunctionExpression(node)
    }
}
impl From<PsqlColReference> for AnyPsqlExpression {
    fn from(node: PsqlColReference) -> Self {
        Self::PsqlColReference(node)
    }
}
impl From<PsqlInExpression> for AnyPsqlExpression {
    fn from(node: PsqlInExpression) -> Self {
        Self::PsqlInExpression(node)
    }
}
impl From<PsqlIsNullExpression> for AnyPsqlExpression {
    fn from(node: PsqlIsNullExpression) -> Self {
        Self::PsqlIsNullExpression(node)
    }
}
impl From<PsqlLikeExpression> for AnyPsqlExpression {
    fn from(node: PsqlLikeExpression) -> Self {
        Self::PsqlLikeExpression(node)
    }
}
impl From<PsqlLogicalExpression> for AnyPsqlExpression {
    fn from(node: PsqlLogicalExpression) -> Self {
        Self::PsqlLogicalExpression(node)
    }
}
impl From<PsqlName> for AnyPsqlExpression {
    fn from(node: PsqlName) -> Self {
        Self::PsqlName(node)
    }
}
impl From<PsqlParenthesizedExpression> for AnyPsqlExpression {
    fn from(node: PsqlParenthesizedExpression) -> Self {
        Self::PsqlParenthesizedExpression(node)
    }
}
impl From<PsqlStar> for AnyPsqlExpression {
    fn from(node: PsqlStar) -> Self {
        Self::PsqlStar(node)
    }
}
impl From<PsqlSubqueryExpression> for AnyPsqlExpression {
    fn from(node: PsqlSubqueryExpression) -> Self {
        Self::PsqlSubqueryExpression(node)
    }
}
impl From<PsqlTableColReference> for AnyPsqlExpression {
    fn from(node: PsqlTableColReference) -> Self {
        Self::PsqlTableColReference(node)
    }
}
impl From<PsqlUnaryExpression> for AnyPsqlExpression {
    fn from(node: PsqlUnaryExpression) -> Self {
        Self::PsqlUnaryExpression(node)
    }
}
impl From<PsqlWindowFunctionExpression> for AnyPsqlExpression {
    fn from(node: PsqlWindowFunctionExpression) -> Self {
        Self::PsqlWindowFunctionExpression(node)
    }
}
impl AstNode for AnyPsqlExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnyPsqlLiteralExpression::KIND_SET
        .union(PsqlArrayExpression::KIND_SET)
        .union(PsqlArraySubscriptExpression::KIND_SET)
        .union(PsqlBetweenExpression::KIND_SET)
        .union(PsqlBinaryExpression::KIND_SET)
        .union(PsqlCallExpression::KIND_SET)
        .union(PsqlCaseExpression::KIND_SET)
        .union(PsqlCastExpression::KIND_SET)
        .union(PsqlCastFunctionExpression::KIND_SET)
        .union(PsqlColReference::KIND_SET)
        .union(PsqlInExpression::KIND_SET)
        .union(PsqlIsNullExpression::KIND_SET)
        .union(PsqlLikeExpression::KIND_SET)
        .union(PsqlLogicalExpression::KIND_SET)
        .union(PsqlName::KIND_SET)
        .union(PsqlParenthesizedExpression::KIND_SET)
        .union(PsqlStar::KIND_SET)
        .union(PsqlSubqueryExpression::KIND_SET)
        .union(PsqlTableColReference::KIND_SET)
        .union(PsqlUnaryExpression::KIND_SET)
        .union(PsqlWindowFunctionExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            PSQL_ARRAY_EXPRESSION
            | PSQL_ARRAY_SUBSCRIPT_EXPRESSION
            | PSQL_BETWEEN_EXPRESSION
            | PSQL_BINARY_EXPRESSION
            | PSQL_CALL_EXPRESSION
            | PSQL_CASE_EXPRESSION
            | PSQL_CAST_EXPRESSION
            | PSQL_CAST_FUNCTION_EXPRESSION
            | PSQL_COL_REFERENCE
            | PSQL_IN_EXPRESSION
            | PSQL_IS_NULL_EXPRESSION
            | PSQL_LIKE_EXPRESSION
            | PSQL_LOGICAL_EXPRESSION
            | PSQL_NAME
            | PSQL_PARENTHESIZED_EXPRESSION
            | PSQL_STAR
            | PSQL_SUBQUERY_EXPRESSION
            | PSQL_TABLE_COL_REFERENCE
            | PSQL_UNARY_EXPRESSION
            | PSQL_WINDOW_FUNCTION_EXPRESSION => true,
            k if AnyPsqlLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_ARRAY_EXPRESSION => Self::PsqlArrayExpression(PsqlArrayExpression { syntax }),
            PSQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                Self::PsqlArraySubscriptExpression(PsqlArraySubscriptExpression { syntax })
            }
            PSQL_BETWEEN_EXPRESSION => {
                Self::PsqlBetweenExpression(PsqlBetweenExpression { syntax })
            }
            PSQL_BINARY_EXPRESSION => Self::PsqlBinaryExpression(PsqlBinaryExpression { syntax }),
            PSQL_CALL_EXPRESSION => Self::PsqlCallExpression(PsqlCallExpression { syntax }),
            PSQL_CASE_EXPRESSION => Self::PsqlCaseExpression(PsqlCaseExpression { syntax }),
            PSQL_CAST_EXPRESSION => Self::PsqlCastExpression(PsqlCastExpression { syntax }),
            PSQL_CAST_FUNCTION_EXPRESSION => {
                Self::PsqlCastFunctionExpression(PsqlCastFunctionExpression { syntax })
            }
            PSQL_COL_REFERENCE => Self::PsqlColReference(PsqlColReference { syntax }),
            PSQL_IN_EXPRESSION => Self::PsqlInExpression(PsqlInExpression { syntax }),
            PSQL_IS_NULL_EXPRESSION => Self::PsqlIsNullExpression(PsqlIsNullExpression { syntax }),
            PSQL_LIKE_EXPRESSION => Self::PsqlLikeExpression(PsqlLikeExpression { syntax }),
            PSQL_LOGICAL_EXPRESSION => {
                Self::PsqlLogicalExpression(PsqlLogicalExpression { syntax })
            }
            PSQL_NAME => Self::PsqlName(PsqlName { syntax }),
            PSQL_PARENTHESIZED_EXPRESSION => {
                Self::PsqlParenthesizedExpression(PsqlParenthesizedExpression { syntax })
            }
            PSQL_STAR => Self::PsqlStar(PsqlStar { syntax }),
            PSQL_SUBQUERY_EXPRESSION => {
                Self::PsqlSubqueryExpression(PsqlSubqueryExpression { syntax })
            }
            PSQL_TABLE_COL_REFERENCE => {
                Self::PsqlTableColReference(PsqlTableColReference { syntax })
            }
            PSQL_UNARY_EXPRESSION => Self::PsqlUnaryExpression(PsqlUnaryExpression { syntax }),
            PSQL_WINDOW_FUNCTION_EXPRESSION => {
                Self::PsqlWindowFunctionExpression(PsqlWindowFunctionExpression { syntax })
            }
            _ => {
                if let Some(any_psql_literal_expression) = AnyPsqlLiteralExpression::cast(syntax) {
                    return Some(Self::AnyPsqlLiteralExpression(any_psql_literal_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlArrayExpression(it) => &it.syntax,
            Self::PsqlArraySubscriptExpression(it) => &it.syntax,
            Self::PsqlBetweenExpression(it) => &it.syntax,
            Self::PsqlBinaryExpression(it) => &it.syntax,
            Self::PsqlCallExpression(it) => &it.syntax,
            Self::PsqlCaseExpression(it) => &it.syntax,
            Self::PsqlCastExpression(it) => &it.syntax,
            Self::PsqlCastFunctionExpression(it) => &it.syntax,
            Self::PsqlColReference(it) => &it.syntax,
            Self::PsqlInExpression(it) => &it.syntax,
            Self::PsqlIsNullExpression(it) => &it.syntax,
            Self::PsqlLikeExpression(it) => &it.syntax,
            Self::PsqlLogicalExpression(it) => &it.syntax,
            Self::PsqlName(it) => &it.syntax,
            Self::PsqlParenthesizedExpression(it) => &it.syntax,
            Self::PsqlStar(it) => &it.syntax,
            Self::PsqlSubqueryExpression(it) => &it.syntax,
            Self::PsqlTableColReference(it) => &it.syntax,
            Self::PsqlUnaryExpression(it) => &it.syntax,
            Self::PsqlWindowFunctionExpression(it) => &it.syntax,
            Self::AnyPsqlLiteralExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlArrayExpression(it) => it.syntax,
            Self::PsqlArraySubscriptExpression(it) => it.syntax,
            Self::PsqlBetweenExpression(it) => it.syntax,
            Self::PsqlBinaryExpression(it) => it.syntax,
            Self::PsqlCallExpression(it) => it.syntax,
            Self::PsqlCaseExpression(it) => it.syntax,
            Self::PsqlCastExpression(it) => it.syntax,
            Self::PsqlCastFunctionExpression(it) => it.syntax,
            Self::PsqlColReference(it) => it.syntax,
            Self::PsqlInExpression(it) => it.syntax,
            Self::PsqlIsNullExpression(it) => it.syntax,
            Self::PsqlLikeExpression(it) => it.syntax,
            Self::PsqlLogicalExpression(it) => it.syntax,
            Self::PsqlName(it) => it.syntax,
            Self::PsqlParenthesizedExpression(it) => it.syntax,
            Self::PsqlStar(it) => it.syntax,
            Self::PsqlSubqueryExpression(it) => it.syntax,
            Self::PsqlTableColReference(it) => it.syntax,
            Self::PsqlUnaryExpression(it) => it.syntax,
            Self::PsqlWindowFunctionExpression(it) => it.syntax,
            Self::AnyPsqlLiteralExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnyPsqlExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnyPsqlLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlArrayExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlArraySubscriptExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlBetweenExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlCallExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlCaseExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlCastExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlCastFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlColReference(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlInExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlIsNullExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlLikeExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlName(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlStar(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlTableColReference(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlWindowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlExpression> for SyntaxNode {
    fn from(n: AnyPsqlExpression) -> Self {
        match n {
            AnyPsqlExpression::AnyPsqlLiteralExpression(it) => it.into(),
            AnyPsqlExpression::PsqlArrayExpression(it) => it.into(),
            AnyPsqlExpression::PsqlArraySubscriptExpression(it) => it.into(),
            AnyPsqlExpression::PsqlBetweenExpression(it) => it.into(),
            AnyPsqlExpression::PsqlBinaryExpression(it) => it.into(),
            AnyPsqlExpression::PsqlCallExpression(it) => it.into(),
            AnyPsqlExpression::PsqlCaseExpression(it) => it.into(),
            AnyPsqlExpression::PsqlCastExpression(it) => it.into(),
            AnyPsqlExpression::PsqlCastFunctionExpression(it) => it.into(),
            AnyPsqlExpression::PsqlColReference(it) => it.into(),
            AnyPsqlExpression::PsqlInExpression(it) => it.into(),
            AnyPsqlExpression::PsqlIsNullExpression(it) => it.into(),
            AnyPsqlExpression::PsqlLikeExpression(it) => it.into(),
            AnyPsqlExpression::PsqlLogicalExpression(it) => it.into(),
            AnyPsqlExpression::PsqlName(it) => it.into(),
            AnyPsqlExpression::PsqlParenthesizedExpression(it) => it.into(),
            AnyPsqlExpression::PsqlStar(it) => it.into(),
            AnyPsqlExpression::PsqlSubqueryExpression(it) => it.into(),
            AnyPsqlExpression::PsqlTableColReference(it) => it.into(),
            AnyPsqlExpression::PsqlUnaryExpression(it) => it.into(),
            AnyPsqlExpression::PsqlWindowFunctionExpression(it) => it.into(),
        }
    }
}
impl From<AnyPsqlExpression> for SyntaxElement {
    fn from(n: AnyPsqlExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlFunctionBinding> for AnyPsqlFromExpression {
    fn from(node: PsqlFunctionBinding) -> Self {
        Self::PsqlFunctionBinding(node)
    }
}
impl From<PsqlSubqueryBinding> for AnyPsqlFromExpression {
    fn from(node: PsqlSubqueryBinding) -> Self {
        Self::PsqlSubqueryBinding(node)
    }
}
impl From<PsqlTableBinding> for AnyPsqlFromExpression {
    fn from(node: PsqlTableBinding) -> Self {
        Self::PsqlTableBinding(node)
    }
}
impl AstNode for AnyPsqlFromExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = PsqlFunctionBinding::KIND_SET
        .union(PsqlSubqueryBinding::KIND_SET)
        .union(PsqlTableBinding::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            PSQL_FUNCTION_BINDING | PSQL_SUBQUERY_BINDING | PSQL_TABLE_BINDING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_FUNCTION_BINDING => Self::PsqlFunctionBinding(PsqlFunctionBinding { syntax }),
            PSQL_SUBQUERY_BINDING => Self::PsqlSubqueryBinding(PsqlSubqueryBinding { syntax }),
            PSQL_TABLE_BINDING => Self::PsqlTableBinding(PsqlTableBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlFunctionBinding(it) => &it.syntax,
            Self::PsqlSubqueryBinding(it) => &it.syntax,
            Self::PsqlTableBinding(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlFunctionBinding(it) => it.syntax,
            Self::PsqlSubqueryBinding(it) => it.syntax,
            Self::PsqlTableBinding(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlFromExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlFunctionBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlSubqueryBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlTableBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlFromExpression> for SyntaxNode {
    fn from(n: AnyPsqlFromExpression) -> Self {
        match n {
            AnyPsqlFromExpression::PsqlFunctionBinding(it) => it.into(),
            AnyPsqlFromExpression::PsqlSubqueryBinding(it) => it.into(),
            AnyPsqlFromExpression::PsqlTableBinding(it) => it.into(),
        }
    }
}
impl From<AnyPsqlFromExpression> for SyntaxElement {
    fn from(n: AnyPsqlFromExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlInValueList> for AnyPsqlInSource {
    fn from(node: PsqlInValueList) -> Self {
        Self::PsqlInValueList(node)
    }
}
impl From<PsqlSubqueryExpression> for AnyPsqlInSource {
    fn from(node: PsqlSubqueryExpression) -> Self {
        Self::PsqlSubqueryExpression(node)
    }
}
impl AstNode for AnyPsqlInSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        PsqlInValueList::KIND_SET.union(PsqlSubqueryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, PSQL_IN_VALUE_LIST | PSQL_SUBQUERY_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_IN_VALUE_LIST => Self::PsqlInValueList(PsqlInValueList { syntax }),
            PSQL_SUBQUERY_EXPRESSION => {
                Self::PsqlSubqueryExpression(PsqlSubqueryExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlInValueList(it) => &it.syntax,
            Self::PsqlSubqueryExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlInValueList(it) => it.syntax,
            Self::PsqlSubqueryExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlInSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlInValueList(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlInSource> for SyntaxNode {
    fn from(n: AnyPsqlInSource) -> Self {
        match n {
            AnyPsqlInSource::PsqlInValueList(it) => it.into(),
            AnyPsqlInSource::PsqlSubqueryExpression(it) => it.into(),
        }
    }
}
impl From<AnyPsqlInSource> for SyntaxElement {
    fn from(n: AnyPsqlInSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlInsertValues> for AnyPsqlInsertSource {
    fn from(node: PsqlInsertValues) -> Self {
        Self::PsqlInsertValues(node)
    }
}
impl From<PsqlSelectStatement> for AnyPsqlInsertSource {
    fn from(node: PsqlSelectStatement) -> Self {
        Self::PsqlSelectStatement(node)
    }
}
impl AstNode for AnyPsqlInsertSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        PsqlInsertValues::KIND_SET.union(PsqlSelectStatement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, PSQL_INSERT_VALUES | PSQL_SELECT_STATEMENT)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_INSERT_VALUES => Self::PsqlInsertValues(PsqlInsertValues { syntax }),
            PSQL_SELECT_STATEMENT => Self::PsqlSelectStatement(PsqlSelectStatement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlInsertValues(it) => &it.syntax,
            Self::PsqlSelectStatement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlInsertValues(it) => it.syntax,
            Self::PsqlSelectStatement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlInsertSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlInsertValues(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlSelectStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlInsertSource> for SyntaxNode {
    fn from(n: AnyPsqlInsertSource) -> Self {
        match n {
            AnyPsqlInsertSource::PsqlInsertValues(it) => it.into(),
            AnyPsqlInsertSource::PsqlSelectStatement(it) => it.into(),
        }
    }
}
impl From<AnyPsqlInsertSource> for SyntaxElement {
    fn from(n: AnyPsqlInsertSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlBooleanLiteralExpression> for AnyPsqlLiteralExpression {
    fn from(node: PsqlBooleanLiteralExpression) -> Self {
        Self::PsqlBooleanLiteralExpression(node)
    }
}
impl From<PsqlNullLiteralExpression> for AnyPsqlLiteralExpression {
    fn from(node: PsqlNullLiteralExpression) -> Self {
        Self::PsqlNullLiteralExpression(node)
    }
}
impl From<PsqlNumberLiteralExpression> for AnyPsqlLiteralExpression {
    fn from(node: PsqlNumberLiteralExpression) -> Self {
        Self::PsqlNumberLiteralExpression(node)
    }
}
impl From<PsqlStringLiteralExpression> for AnyPsqlLiteralExpression {
    fn from(node: PsqlStringLiteralExpression) -> Self {
        Self::PsqlStringLiteralExpression(node)
    }
}
impl AstNode for AnyPsqlLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = PsqlBooleanLiteralExpression::KIND_SET
        .union(PsqlNullLiteralExpression::KIND_SET)
        .union(PsqlNumberLiteralExpression::KIND_SET)
        .union(PsqlStringLiteralExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            PSQL_BOOLEAN_LITERAL_EXPRESSION
                | PSQL_NULL_LITERAL_EXPRESSION
                | PSQL_NUMBER_LITERAL_EXPRESSION
                | PSQL_STRING_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_BOOLEAN_LITERAL_EXPRESSION => {
                Self::PsqlBooleanLiteralExpression(PsqlBooleanLiteralExpression { syntax })
            }
            PSQL_NULL_LITERAL_EXPRESSION => {
                Self::PsqlNullLiteralExpression(PsqlNullLiteralExpression { syntax })
            }
            PSQL_NUMBER_LITERAL_EXPRESSION => {
                Self::PsqlNumberLiteralExpression(PsqlNumberLiteralExpression { syntax })
            }
            PSQL_STRING_LITERAL_EXPRESSION => {
                Self::PsqlStringLiteralExpression(PsqlStringLiteralExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlBooleanLiteralExpression(it) => &it.syntax,
            Self::PsqlNullLiteralExpression(it) => &it.syntax,
            Self::PsqlNumberLiteralExpression(it) => &it.syntax,
            Self::PsqlStringLiteralExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlBooleanLiteralExpression(it) => it.syntax,
            Self::PsqlNullLiteralExpression(it) => it.syntax,
            Self::PsqlNumberLiteralExpression(it) => it.syntax,
            Self::PsqlStringLiteralExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlLiteralExpression> for SyntaxNode {
    fn from(n: AnyPsqlLiteralExpression) -> Self {
        match n {
            AnyPsqlLiteralExpression::PsqlBooleanLiteralExpression(it) => it.into(),
            AnyPsqlLiteralExpression::PsqlNullLiteralExpression(it) => it.into(),
            AnyPsqlLiteralExpression::PsqlNumberLiteralExpression(it) => it.into(),
            AnyPsqlLiteralExpression::PsqlStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<AnyPsqlLiteralExpression> for SyntaxElement {
    fn from(n: AnyPsqlLiteralExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlSelectExpression> for AnyPsqlSelectItem {
    fn from(node: PsqlSelectExpression) -> Self {
        Self::PsqlSelectExpression(node)
    }
}
impl From<PsqlStar> for AnyPsqlSelectItem {
    fn from(node: PsqlStar) -> Self {
        Self::PsqlStar(node)
    }
}
impl AstNode for AnyPsqlSelectItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        PsqlSelectExpression::KIND_SET.union(PsqlStar::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, PSQL_SELECT_EXPRESSION | PSQL_STAR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_SELECT_EXPRESSION => Self::PsqlSelectExpression(PsqlSelectExpression { syntax }),
            PSQL_STAR => Self::PsqlStar(PsqlStar { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlSelectExpression(it) => &it.syntax,
            Self::PsqlStar(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlSelectExpression(it) => it.syntax,
            Self::PsqlStar(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlSelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlSelectExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlStar(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlSelectItem> for SyntaxNode {
    fn from(n: AnyPsqlSelectItem) -> Self {
        match n {
            AnyPsqlSelectItem::PsqlSelectExpression(it) => it.into(),
            AnyPsqlSelectItem::PsqlStar(it) => it.into(),
        }
    }
}
impl From<AnyPsqlSelectItem> for SyntaxElement {
    fn from(n: AnyPsqlSelectItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<PsqlDeleteStatement> for AnyPsqlStatement {
    fn from(node: PsqlDeleteStatement) -> Self {
        Self::PsqlDeleteStatement(node)
    }
}
impl From<PsqlInsertStatement> for AnyPsqlStatement {
    fn from(node: PsqlInsertStatement) -> Self {
        Self::PsqlInsertStatement(node)
    }
}
impl From<PsqlSelectStatement> for AnyPsqlStatement {
    fn from(node: PsqlSelectStatement) -> Self {
        Self::PsqlSelectStatement(node)
    }
}
impl From<PsqlUpdateStatement> for AnyPsqlStatement {
    fn from(node: PsqlUpdateStatement) -> Self {
        Self::PsqlUpdateStatement(node)
    }
}
impl AstNode for AnyPsqlStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = PsqlDeleteStatement::KIND_SET
        .union(PsqlInsertStatement::KIND_SET)
        .union(PsqlSelectStatement::KIND_SET)
        .union(PsqlUpdateStatement::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            PSQL_DELETE_STATEMENT
                | PSQL_INSERT_STATEMENT
                | PSQL_SELECT_STATEMENT
                | PSQL_UPDATE_STATEMENT
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            PSQL_DELETE_STATEMENT => Self::PsqlDeleteStatement(PsqlDeleteStatement { syntax }),
            PSQL_INSERT_STATEMENT => Self::PsqlInsertStatement(PsqlInsertStatement { syntax }),
            PSQL_SELECT_STATEMENT => Self::PsqlSelectStatement(PsqlSelectStatement { syntax }),
            PSQL_UPDATE_STATEMENT => Self::PsqlUpdateStatement(PsqlUpdateStatement { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::PsqlDeleteStatement(it) => &it.syntax,
            Self::PsqlInsertStatement(it) => &it.syntax,
            Self::PsqlSelectStatement(it) => &it.syntax,
            Self::PsqlUpdateStatement(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::PsqlDeleteStatement(it) => it.syntax,
            Self::PsqlInsertStatement(it) => it.syntax,
            Self::PsqlSelectStatement(it) => it.syntax,
            Self::PsqlUpdateStatement(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnyPsqlStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PsqlDeleteStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlInsertStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlSelectStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::PsqlUpdateStatement(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnyPsqlStatement> for SyntaxNode {
    fn from(n: AnyPsqlStatement) -> Self {
        match n {
            AnyPsqlStatement::PsqlDeleteStatement(it) => it.into(),
            AnyPsqlStatement::PsqlInsertStatement(it) => it.into(),
            AnyPsqlStatement::PsqlSelectStatement(it) => it.into(),
            AnyPsqlStatement::PsqlUpdateStatement(it) => it.into(),
        }
    }
}
impl From<AnyPsqlStatement> for SyntaxElement {
    fn from(n: AnyPsqlStatement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnyPsqlConflictAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlConflictTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlFromExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlInSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlInsertSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlSelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnyPsqlStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlArraySubscriptExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlBetweenExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCaseElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCaseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCaseWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCastExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCastFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlCteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlDataBaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlDeleteStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlDeleteUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlDoNothingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlDoUpdateClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlFromItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlGroupByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlHavingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlInValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlInsertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlInsertValues {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlIsNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlJoinClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlLikeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlLimitClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlOffsetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlOnConflictClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlOnConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlOrderByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlOrderByExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlReturningClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSelectClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSetOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlShemaName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSubqueryBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlSubqueryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTableColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlUpdateStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlWhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlWindowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlWindowPartitionByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlWindowSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for PsqlWithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogus {
    syntax: SyntaxNode,
}
impl PsqlBogus {
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
impl AstNode for PsqlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS
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
impl std::fmt::Debug for PsqlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogus> for SyntaxNode {
    fn from(n: PsqlBogus) -> Self {
        n.syntax
    }
}
impl From<PsqlBogus> for SyntaxElement {
    fn from(n: PsqlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusAssignment {
    syntax: SyntaxNode,
}
impl PsqlBogusAssignment {
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
impl AstNode for PsqlBogusAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_ASSIGNMENT
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
impl std::fmt::Debug for PsqlBogusAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusAssignment")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusAssignment> for SyntaxNode {
    fn from(n: PsqlBogusAssignment) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusAssignment> for SyntaxElement {
    fn from(n: PsqlBogusAssignment) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusBinding {
    syntax: SyntaxNode,
}
impl PsqlBogusBinding {
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
impl AstNode for PsqlBogusBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_BINDING
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
impl std::fmt::Debug for PsqlBogusBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusBinding")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusBinding> for SyntaxNode {
    fn from(n: PsqlBogusBinding) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusBinding> for SyntaxElement {
    fn from(n: PsqlBogusBinding) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusExpression {
    syntax: SyntaxNode,
}
impl PsqlBogusExpression {
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
impl AstNode for PsqlBogusExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_EXPRESSION
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
impl std::fmt::Debug for PsqlBogusExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusExpression> for SyntaxNode {
    fn from(n: PsqlBogusExpression) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusExpression> for SyntaxElement {
    fn from(n: PsqlBogusExpression) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusMember {
    syntax: SyntaxNode,
}
impl PsqlBogusMember {
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
impl AstNode for PsqlBogusMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_MEMBER
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
impl std::fmt::Debug for PsqlBogusMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusMember")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusMember> for SyntaxNode {
    fn from(n: PsqlBogusMember) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusMember> for SyntaxElement {
    fn from(n: PsqlBogusMember) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusParameter {
    syntax: SyntaxNode,
}
impl PsqlBogusParameter {
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
impl AstNode for PsqlBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_PARAMETER
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
impl std::fmt::Debug for PsqlBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusParameter> for SyntaxNode {
    fn from(n: PsqlBogusParameter) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusParameter> for SyntaxElement {
    fn from(n: PsqlBogusParameter) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct PsqlBogusStatement {
    syntax: SyntaxNode,
}
impl PsqlBogusStatement {
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
impl AstNode for PsqlBogusStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_BOGUS_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_BOGUS_STATEMENT
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
impl std::fmt::Debug for PsqlBogusStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PsqlBogusStatement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<PsqlBogusStatement> for SyntaxNode {
    fn from(n: PsqlBogusStatement) -> Self {
        n.syntax
    }
}
impl From<PsqlBogusStatement> for SyntaxElement {
    fn from(n: PsqlBogusStatement) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnyPsqlBogusNode = PsqlBogus | PsqlBogusAssignment | PsqlBogusBinding | PsqlBogusExpression | PsqlBogusMember | PsqlBogusParameter | PsqlBogusStatement }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlCaseWhenClauseList {
    syntax_list: SyntaxList,
}
impl PsqlCaseWhenClauseList {
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
impl AstNode for PsqlCaseWhenClauseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CASE_WHEN_CLAUSE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CASE_WHEN_CLAUSE_LIST
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
impl Serialize for PsqlCaseWhenClauseList {
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
impl AstNodeList for PsqlCaseWhenClauseList {
    type Language = Language;
    type Node = PsqlCaseWhenClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlCaseWhenClauseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlCaseWhenClauseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &PsqlCaseWhenClauseList {
    type Item = PsqlCaseWhenClause;
    type IntoIter = AstNodeListIterator<Language, PsqlCaseWhenClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for PsqlCaseWhenClauseList {
    type Item = PsqlCaseWhenClause;
    type IntoIter = AstNodeListIterator<Language, PsqlCaseWhenClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlColumnNameList {
    syntax_list: SyntaxList,
}
impl PsqlColumnNameList {
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
impl AstNode for PsqlColumnNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_COLUMN_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_COLUMN_NAME_LIST
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
impl Serialize for PsqlColumnNameList {
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
impl AstSeparatedList for PsqlColumnNameList {
    type Language = Language;
    type Node = PsqlName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlColumnNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlColumnNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlColumnNameList {
    type Item = SyntaxResult<PsqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlColumnNameList {
    type Item = SyntaxResult<PsqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlCteDefinitionList {
    syntax_list: SyntaxList,
}
impl PsqlCteDefinitionList {
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
impl AstNode for PsqlCteDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_CTE_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_CTE_DEFINITION_LIST
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
impl Serialize for PsqlCteDefinitionList {
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
impl AstSeparatedList for PsqlCteDefinitionList {
    type Language = Language;
    type Node = PsqlCteDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlCteDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlCteDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlCteDefinitionList {
    type Item = SyntaxResult<PsqlCteDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlCteDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlCteDefinitionList {
    type Item = SyntaxResult<PsqlCteDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlCteDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlExpressionList {
    syntax_list: SyntaxList,
}
impl PsqlExpressionList {
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
impl AstNode for PsqlExpressionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_EXPRESSION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_EXPRESSION_LIST
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
impl Serialize for PsqlExpressionList {
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
impl AstSeparatedList for PsqlExpressionList {
    type Language = Language;
    type Node = AnyPsqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlExpressionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlExpressionList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlExpressionList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlFromItemList {
    syntax_list: SyntaxList,
}
impl PsqlFromItemList {
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
impl AstNode for PsqlFromItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_FROM_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_FROM_ITEM_LIST
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
impl Serialize for PsqlFromItemList {
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
impl AstSeparatedList for PsqlFromItemList {
    type Language = Language;
    type Node = PsqlFromItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlFromItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlFromItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlFromItemList {
    type Item = SyntaxResult<PsqlFromItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlFromItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlFromItemList {
    type Item = SyntaxResult<PsqlFromItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlFromItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlGroupByItemList {
    syntax_list: SyntaxList,
}
impl PsqlGroupByItemList {
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
impl AstNode for PsqlGroupByItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_GROUP_BY_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_GROUP_BY_ITEM_LIST
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
impl Serialize for PsqlGroupByItemList {
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
impl AstSeparatedList for PsqlGroupByItemList {
    type Language = Language;
    type Node = AnyPsqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlGroupByItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlGroupByItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlGroupByItemList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlGroupByItemList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlJoinClauseList {
    syntax_list: SyntaxList,
}
impl PsqlJoinClauseList {
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
impl AstNode for PsqlJoinClauseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_JOIN_CLAUSE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_JOIN_CLAUSE_LIST
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
impl Serialize for PsqlJoinClauseList {
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
impl AstNodeList for PsqlJoinClauseList {
    type Language = Language;
    type Node = PsqlJoinClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlJoinClauseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlJoinClauseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &PsqlJoinClauseList {
    type Item = PsqlJoinClause;
    type IntoIter = AstNodeListIterator<Language, PsqlJoinClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for PsqlJoinClauseList {
    type Item = PsqlJoinClause;
    type IntoIter = AstNodeListIterator<Language, PsqlJoinClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlOrderByExpressionList {
    syntax_list: SyntaxList,
}
impl PsqlOrderByExpressionList {
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
impl AstNode for PsqlOrderByExpressionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_ORDER_BY_EXPRESSION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_ORDER_BY_EXPRESSION_LIST
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
impl Serialize for PsqlOrderByExpressionList {
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
impl AstSeparatedList for PsqlOrderByExpressionList {
    type Language = Language;
    type Node = PsqlOrderByExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlOrderByExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlOrderByExpressionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlOrderByExpressionList {
    type Item = SyntaxResult<PsqlOrderByExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlOrderByExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlOrderByExpressionList {
    type Item = SyntaxResult<PsqlOrderByExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlOrderByExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlSelectItemList {
    syntax_list: SyntaxList,
}
impl PsqlSelectItemList {
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
impl AstNode for PsqlSelectItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SELECT_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SELECT_ITEM_LIST
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
impl Serialize for PsqlSelectItemList {
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
impl AstSeparatedList for PsqlSelectItemList {
    type Language = Language;
    type Node = AnyPsqlSelectItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlSelectItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlSelectItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlSelectItemList {
    type Item = SyntaxResult<AnyPsqlSelectItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlSelectItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlSelectItemList {
    type Item = SyntaxResult<AnyPsqlSelectItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlSelectItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlSetItemList {
    syntax_list: SyntaxList,
}
impl PsqlSetItemList {
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
impl AstNode for PsqlSetItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SET_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SET_ITEM_LIST
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
impl Serialize for PsqlSetItemList {
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
impl AstSeparatedList for PsqlSetItemList {
    type Language = Language;
    type Node = PsqlSetItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlSetItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlSetItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlSetItemList {
    type Item = SyntaxResult<PsqlSetItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlSetItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlSetItemList {
    type Item = SyntaxResult<PsqlSetItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlSetItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlSetOperationList {
    syntax_list: SyntaxList,
}
impl PsqlSetOperationList {
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
impl AstNode for PsqlSetOperationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_SET_OPERATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_SET_OPERATION_LIST
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
impl Serialize for PsqlSetOperationList {
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
impl AstNodeList for PsqlSetOperationList {
    type Language = Language;
    type Node = PsqlSetOperation;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlSetOperationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlSetOperationList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &PsqlSetOperationList {
    type Item = PsqlSetOperation;
    type IntoIter = AstNodeListIterator<Language, PsqlSetOperation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for PsqlSetOperationList {
    type Item = PsqlSetOperation;
    type IntoIter = AstNodeListIterator<Language, PsqlSetOperation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlStatementList {
    syntax_list: SyntaxList,
}
impl PsqlStatementList {
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
impl AstNode for PsqlStatementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_STATEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_STATEMENT_LIST
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
impl Serialize for PsqlStatementList {
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
impl AstNodeList for PsqlStatementList {
    type Language = Language;
    type Node = AnyPsqlStatement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlStatementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlStatementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &PsqlStatementList {
    type Item = AnyPsqlStatement;
    type IntoIter = AstNodeListIterator<Language, AnyPsqlStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for PsqlStatementList {
    type Item = AnyPsqlStatement;
    type IntoIter = AstNodeListIterator<Language, AnyPsqlStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlTypeArgumentList {
    syntax_list: SyntaxList,
}
impl PsqlTypeArgumentList {
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
impl AstNode for PsqlTypeArgumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_TYPE_ARGUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_TYPE_ARGUMENT_LIST
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
impl Serialize for PsqlTypeArgumentList {
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
impl AstSeparatedList for PsqlTypeArgumentList {
    type Language = Language;
    type Node = PsqlNumberLiteralExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlTypeArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlTypeArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlTypeArgumentList {
    type Item = SyntaxResult<PsqlNumberLiteralExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlNumberLiteralExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlTypeArgumentList {
    type Item = SyntaxResult<PsqlNumberLiteralExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, PsqlNumberLiteralExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct PsqlWindowPartitionByItemList {
    syntax_list: SyntaxList,
}
impl PsqlWindowPartitionByItemList {
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
impl AstNode for PsqlWindowPartitionByItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(PSQL_WINDOW_PARTITION_BY_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == PSQL_WINDOW_PARTITION_BY_ITEM_LIST
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
impl Serialize for PsqlWindowPartitionByItemList {
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
impl AstSeparatedList for PsqlWindowPartitionByItemList {
    type Language = Language;
    type Node = AnyPsqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for PsqlWindowPartitionByItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("PsqlWindowPartitionByItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for PsqlWindowPartitionByItemList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &PsqlWindowPartitionByItemList {
    type Item = SyntaxResult<AnyPsqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnyPsqlExpression>;
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
