//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(dead_code)]
#![allow(unused)]
use crate::{
    SqlLanguage as Language, SqlSyntaxElement as SyntaxElement,
    SqlSyntaxElementChildren as SyntaxElementChildren,
    SqlSyntaxKind::{self as SyntaxKind, *},
    SqlSyntaxList as SyntaxList, SqlSyntaxNode as SyntaxNode, SqlSyntaxToken as SyntaxToken,
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
pub struct SqlAlias {
    pub(crate) syntax: SyntaxNode,
}
impl SqlAlias {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlAliasFields {
        SqlAliasFields {
            as_token: self.as_token(),
            value: self.value(),
            columns: self.columns(),
        }
    }
    pub fn as_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn columns(&self) -> Option<SqlAliasColumnList> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlAlias {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlAliasFields {
    pub as_token: Option<SyntaxToken>,
    pub value: SyntaxResult<SqlName>,
    pub columns: Option<SqlAliasColumnList>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlAliasColumnDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl SqlAliasColumnDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlAliasColumnDefinitionFields {
        SqlAliasColumnDefinitionFields {
            name: self.name(),
            ty: self.ty(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> Option<SqlTypeName> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlAliasColumnDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlAliasColumnDefinitionFields {
    pub name: SyntaxResult<SqlName>,
    pub ty: Option<SqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlAliasColumnList {
    pub(crate) syntax: SyntaxNode,
}
impl SqlAliasColumnList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlAliasColumnListFields {
        SqlAliasColumnListFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlAliasColumnDefinitionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlAliasColumnList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlAliasColumnListFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlAliasColumnDefinitionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlAnyAllExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlAnyAllExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlAnyAllExpressionFields {
        SqlAnyAllExpressionFields {
            quantifier: self.quantifier(),
            source: self.source(),
        }
    }
    pub fn quantifier(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn source(&self) -> SyntaxResult<AnySqlAnyAllSource> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlAnyAllExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlAnyAllExpressionFields {
    pub quantifier: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnySqlAnyAllSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlArrayExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlArrayExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlArrayExpressionFields {
        SqlArrayExpressionFields {
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
    pub fn items(&self) -> SqlExpressionList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlArrayExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlArrayExpressionFields {
    pub array_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub items: SqlExpressionList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlArraySubscriptExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlArraySubscriptExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlArraySubscriptExpressionFields {
        SqlArraySubscriptExpressionFields {
            expression: self.expression(),
            l_brack_token: self.l_brack_token(),
            index: self.index(),
            r_brack_token: self.r_brack_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn index(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlArraySubscriptExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlArraySubscriptExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub index: SyntaxResult<AnySqlExpression>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlBetweenExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlBetweenExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlBetweenExpressionFields {
        SqlBetweenExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            between_token: self.between_token(),
            low: self.low(),
            and_token: self.and_token(),
            high: self.high(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn between_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn low(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn and_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn high(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 5usize)
    }
}
impl Serialize for SqlBetweenExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlBetweenExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub between_token: SyntaxResult<SyntaxToken>,
    pub low: SyntaxResult<AnySqlExpression>,
    pub and_token: SyntaxResult<SyntaxToken>,
    pub high: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlBinaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlBinaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlBinaryExpressionFields {
        SqlBinaryExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlBinaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlBinaryExpressionFields {
    pub left: SyntaxResult<AnySqlExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlBooleanLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlBooleanLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlBooleanLiteralExpressionFields {
        SqlBooleanLiteralExpressionFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlBooleanLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlBooleanLiteralExpressionFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCallExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCallExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCallExpressionFields {
        SqlCallExpressionFields {
            schema: self.schema(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
            filter_clause: self.filter_clause(),
        }
    }
    pub fn schema(&self) -> Option<SqlShemaName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn arguments(&self) -> SqlExpressionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn filter_clause(&self) -> Option<SqlFilterClause> {
        support::node(&self.syntax, 5usize)
    }
}
impl Serialize for SqlCallExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCallExpressionFields {
    pub schema: Option<SqlShemaName>,
    pub name: SyntaxResult<AnySqlName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: SqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub filter_clause: Option<SqlFilterClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCaseElseClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCaseElseClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCaseElseClauseFields {
        SqlCaseElseClauseFields {
            else_token: self.else_token(),
            result: self.result(),
        }
    }
    pub fn else_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn result(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlCaseElseClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCaseElseClauseFields {
    pub else_token: SyntaxResult<SyntaxToken>,
    pub result: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCaseExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCaseExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCaseExpressionFields {
        SqlCaseExpressionFields {
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
    pub fn expression(&self) -> Option<AnySqlExpression> {
        support::node(&self.syntax, 1usize)
    }
    pub fn when_clauses(&self) -> SqlCaseWhenClauseList {
        support::list(&self.syntax, 2usize)
    }
    pub fn else_clause(&self) -> Option<SqlCaseElseClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn end_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for SqlCaseExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCaseExpressionFields {
    pub case_token: SyntaxResult<SyntaxToken>,
    pub expression: Option<AnySqlExpression>,
    pub when_clauses: SqlCaseWhenClauseList,
    pub else_clause: Option<SqlCaseElseClause>,
    pub end_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCaseWhenClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCaseWhenClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCaseWhenClauseFields {
        SqlCaseWhenClauseFields {
            when_token: self.when_token(),
            condition: self.condition(),
            then_token: self.then_token(),
            result: self.result(),
        }
    }
    pub fn when_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn then_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn result(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlCaseWhenClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCaseWhenClauseFields {
    pub when_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
    pub then_token: SyntaxResult<SyntaxToken>,
    pub result: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCastExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCastExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCastExpressionFields {
        SqlCastExpressionFields {
            expression: self.expression(),
            double_colon_token: self.double_colon_token(),
            ty: self.ty(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn double_colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlCastExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCastExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub double_colon_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<SqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCastFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCastFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCastFunctionExpressionFields {
        SqlCastFunctionExpressionFields {
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
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for SqlCastFunctionExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCastFunctionExpressionFields {
    pub cast_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnySqlExpression>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<SqlTypeName>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlColReference {
    pub(crate) syntax: SyntaxNode,
}
impl SqlColReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlColReferenceFields {
        SqlColReferenceFields { name: self.name() }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
}
impl Serialize for SqlColReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlColReferenceFields {
    pub name: SyntaxResult<SqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlColumnDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl SqlColumnDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlColumnDefinitionFields {
        SqlColumnDefinitionFields {
            name: self.name(),
            ty: self.ty(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlColumnDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlColumnDefinitionFields {
    pub name: SyntaxResult<SqlName>,
    pub ty: SyntaxResult<SqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlColumnList {
    pub(crate) syntax: SyntaxNode,
}
impl SqlColumnList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlColumnListFields {
        SqlColumnListFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlColumnNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlColumnList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlColumnListFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlColumnNameList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCreateFunctionStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCreateFunctionStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCreateFunctionStatementFields {
        SqlCreateFunctionStatementFields {
            create_token: self.create_token(),
            or_token: self.or_token(),
            replace_token: self.replace_token(),
            kind: self.kind(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            parameters: self.parameters(),
            r_paren_token: self.r_paren_token(),
            returns_clause: self.returns_clause(),
            leading_options: self.leading_options(),
            as_token: self.as_token(),
            body: self.body(),
            trailing_options: self.trailing_options(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn create_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn replace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn parameters(&self) -> SqlFunctionParameterList {
        support::list(&self.syntax, 6usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 7usize)
    }
    pub fn returns_clause(&self) -> Option<SqlReturnsClause> {
        support::node(&self.syntax, 8usize)
    }
    pub fn leading_options(&self) -> SqlFunctionOptionList {
        support::list(&self.syntax, 9usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 10usize)
    }
    pub fn body(&self) -> SyntaxResult<SqlStringLiteralExpression> {
        support::required_node(&self.syntax, 11usize)
    }
    pub fn trailing_options(&self) -> SqlFunctionOptionList {
        support::list(&self.syntax, 12usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 13usize)
    }
}
impl Serialize for SqlCreateFunctionStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCreateFunctionStatementFields {
    pub create_token: SyntaxResult<SyntaxToken>,
    pub or_token: Option<SyntaxToken>,
    pub replace_token: Option<SyntaxToken>,
    pub kind: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnySqlName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub parameters: SqlFunctionParameterList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub returns_clause: Option<SqlReturnsClause>,
    pub leading_options: SqlFunctionOptionList,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub body: SyntaxResult<SqlStringLiteralExpression>,
    pub trailing_options: SqlFunctionOptionList,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCreatePolicyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCreatePolicyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCreatePolicyStatementFields {
        SqlCreatePolicyStatementFields {
            create_token: self.create_token(),
            policy_token: self.policy_token(),
            name: self.name(),
            on_token: self.on_token(),
            table: self.table(),
            for_clause: self.for_clause(),
            using_clause: self.using_clause(),
            with_check_clause: self.with_check_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn create_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn policy_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn for_clause(&self) -> Option<SqlPolicyForClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn using_clause(&self) -> Option<SqlPolicyUsingClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn with_check_clause(&self) -> Option<SqlPolicyWithCheckClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 8usize)
    }
}
impl Serialize for SqlCreatePolicyStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCreatePolicyStatementFields {
    pub create_token: SyntaxResult<SyntaxToken>,
    pub policy_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableName>,
    pub for_clause: Option<SqlPolicyForClause>,
    pub using_clause: Option<SqlPolicyUsingClause>,
    pub with_check_clause: Option<SqlPolicyWithCheckClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCreateTableStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCreateTableStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCreateTableStatementFields {
        SqlCreateTableStatementFields {
            create_token: self.create_token(),
            table_token: self.table_token(),
            if_token: self.if_token(),
            not_token: self.not_token(),
            exists_token: self.exists_token(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            columns: self.columns(),
            r_paren_token: self.r_paren_token(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn create_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn table_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn columns(&self) -> SqlColumnDefinitionList {
        support::list(&self.syntax, 7usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 8usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 9usize)
    }
}
impl Serialize for SqlCreateTableStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCreateTableStatementFields {
    pub create_token: SyntaxResult<SyntaxToken>,
    pub table_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub not_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub name: SyntaxResult<SqlTableName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub columns: SqlColumnDefinitionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCreateTriggerStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCreateTriggerStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCreateTriggerStatementFields {
        SqlCreateTriggerStatementFields {
            create_token: self.create_token(),
            trigger_token: self.trigger_token(),
            name: self.name(),
            timing: self.timing(),
            events: self.events(),
            on_token: self.on_token(),
            table: self.table(),
            referencing_clause: self.referencing_clause(),
            for_each_clause: self.for_each_clause(),
            when_clause: self.when_clause(),
            execute_token: self.execute_token(),
            function_kind: self.function_kind(),
            function: self.function(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn create_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn trigger_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn timing(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn events(&self) -> SqlTriggerEventList {
        support::list(&self.syntax, 4usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn referencing_clause(&self) -> Option<SqlTriggerReferencingClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn for_each_clause(&self) -> Option<SqlTriggerForEachClause> {
        support::node(&self.syntax, 8usize)
    }
    pub fn when_clause(&self) -> Option<SqlTriggerWhenClause> {
        support::node(&self.syntax, 9usize)
    }
    pub fn execute_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 10usize)
    }
    pub fn function_kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 11usize)
    }
    pub fn function(&self) -> SyntaxResult<SqlCallExpression> {
        support::required_node(&self.syntax, 12usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 13usize)
    }
}
impl Serialize for SqlCreateTriggerStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCreateTriggerStatementFields {
    pub create_token: SyntaxResult<SyntaxToken>,
    pub trigger_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<AnySqlName>,
    pub timing: SyntaxResult<SyntaxToken>,
    pub events: SqlTriggerEventList,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableName>,
    pub referencing_clause: Option<SqlTriggerReferencingClause>,
    pub for_each_clause: Option<SqlTriggerForEachClause>,
    pub when_clause: Option<SqlTriggerWhenClause>,
    pub execute_token: SyntaxResult<SyntaxToken>,
    pub function_kind: SyntaxResult<SyntaxToken>,
    pub function: SyntaxResult<SqlCallExpression>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCreateViewStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCreateViewStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCreateViewStatementFields {
        SqlCreateViewStatementFields {
            create_token: self.create_token(),
            or_token: self.or_token(),
            replace_token: self.replace_token(),
            view_token: self.view_token(),
            name: self.name(),
            options: self.options(),
            as_token: self.as_token(),
            query: self.query(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn create_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn replace_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn view_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn options(&self) -> Option<SqlViewOptions> {
        support::node(&self.syntax, 5usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
    pub fn query(&self) -> SyntaxResult<SqlSelectStatement> {
        support::required_node(&self.syntax, 7usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 8usize)
    }
}
impl Serialize for SqlCreateViewStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCreateViewStatementFields {
    pub create_token: SyntaxResult<SyntaxToken>,
    pub or_token: Option<SyntaxToken>,
    pub replace_token: Option<SyntaxToken>,
    pub view_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlTableName>,
    pub options: Option<SqlViewOptions>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<SqlSelectStatement>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCteDefinition {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCteDefinition {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCteDefinitionFields {
        SqlCteDefinitionFields {
            name: self.name(),
            columns: self.columns(),
            as_token: self.as_token(),
            materialized: self.materialized(),
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn columns(&self) -> Option<SqlColumnList> {
        support::node(&self.syntax, 1usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn materialized(&self) -> Option<SqlCteMaterializedHint> {
        support::node(&self.syntax, 3usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn query(&self) -> SyntaxResult<AnySqlStatement> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 6usize)
    }
}
impl Serialize for SqlCteDefinition {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCteDefinitionFields {
    pub name: SyntaxResult<SqlName>,
    pub columns: Option<SqlColumnList>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub materialized: Option<SqlCteMaterializedHint>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnySqlStatement>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlCteMaterializedHint {
    pub(crate) syntax: SyntaxNode,
}
impl SqlCteMaterializedHint {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlCteMaterializedHintFields {
        SqlCteMaterializedHintFields {
            not_token: self.not_token(),
            materialized_token: self.materialized_token(),
        }
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn materialized_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlCteMaterializedHint {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlCteMaterializedHintFields {
    pub not_token: Option<SyntaxToken>,
    pub materialized_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDataBaseName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDataBaseName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDataBaseNameFields {
        SqlDataBaseNameFields {
            name: self.name(),
            dot_token: self.dot_token(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlDataBaseName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDataBaseNameFields {
    pub name: SyntaxResult<SqlName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDeleteStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDeleteStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDeleteStatementFields {
        SqlDeleteStatementFields {
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
    pub fn with_clause(&self) -> Option<SqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn delete_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn using(&self) -> Option<SqlDeleteUsingClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn where_clause(&self) -> Option<SqlWhereClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn returning_clause(&self) -> Option<SqlReturningClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for SqlDeleteStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDeleteStatementFields {
    pub with_clause: Option<SqlWithClause>,
    pub delete_token: SyntaxResult<SyntaxToken>,
    pub from_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableBinding>,
    pub using: Option<SqlDeleteUsingClause>,
    pub where_clause: Option<SqlWhereClause>,
    pub returning_clause: Option<SqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDeleteUsingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDeleteUsingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDeleteUsingClauseFields {
        SqlDeleteUsingClauseFields {
            using_token: self.using_token(),
            items: self.items(),
        }
    }
    pub fn using_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlFromItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlDeleteUsingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDeleteUsingClauseFields {
    pub using_token: SyntaxResult<SyntaxToken>,
    pub items: SqlFromItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDistinctOnClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDistinctOnClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDistinctOnClauseFields {
        SqlDistinctOnClauseFields {
            on_token: self.on_token(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> SqlExpressionList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlDistinctOnClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDistinctOnClauseFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDoNothingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDoNothingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDoNothingClauseFields {
        SqlDoNothingClauseFields {
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
impl Serialize for SqlDoNothingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDoNothingClauseFields {
    pub do_token: SyntaxResult<SyntaxToken>,
    pub nothing_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDoUpdateClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDoUpdateClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDoUpdateClauseFields {
        SqlDoUpdateClauseFields {
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
    pub fn set_clause(&self) -> SyntaxResult<SqlSetClause> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn where_clause(&self) -> Option<SqlWhereClause> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlDoUpdateClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDoUpdateClauseFields {
    pub do_token: SyntaxResult<SyntaxToken>,
    pub update_token: SyntaxResult<SyntaxToken>,
    pub set_clause: SyntaxResult<SqlSetClause>,
    pub where_clause: Option<SqlWhereClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropFunctionParameters {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropFunctionParameters {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropFunctionParametersFields {
        SqlDropFunctionParametersFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlTypeNameList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlDropFunctionParameters {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropFunctionParametersFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlTypeNameList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropFunctionStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropFunctionStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropFunctionStatementFields {
        SqlDropFunctionStatementFields {
            drop_token: self.drop_token(),
            kind: self.kind(),
            if_token: self.if_token(),
            exists_token: self.exists_token(),
            name: self.name(),
            parameters: self.parameters(),
            drop_behavior: self.drop_behavior(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn drop_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn parameters(&self) -> Option<SqlDropFunctionParameters> {
        support::node(&self.syntax, 5usize)
    }
    pub fn drop_behavior(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for SqlDropFunctionStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropFunctionStatementFields {
    pub drop_token: SyntaxResult<SyntaxToken>,
    pub kind: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub name: SyntaxResult<AnySqlName>,
    pub parameters: Option<SqlDropFunctionParameters>,
    pub drop_behavior: Option<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropPolicyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropPolicyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropPolicyStatementFields {
        SqlDropPolicyStatementFields {
            drop_token: self.drop_token(),
            policy_token: self.policy_token(),
            if_token: self.if_token(),
            exists_token: self.exists_token(),
            name: self.name(),
            on_token: self.on_token(),
            table: self.table(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn drop_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn policy_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for SqlDropPolicyStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropPolicyStatementFields {
    pub drop_token: SyntaxResult<SyntaxToken>,
    pub policy_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableName>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropTableStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropTableStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropTableStatementFields {
        SqlDropTableStatementFields {
            drop_token: self.drop_token(),
            table_token: self.table_token(),
            if_token: self.if_token(),
            exists_token: self.exists_token(),
            tables: self.tables(),
            drop_behavior: self.drop_behavior(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn drop_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn table_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn tables(&self) -> SqlTableNameList {
        support::list(&self.syntax, 4usize)
    }
    pub fn drop_behavior(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 5usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 6usize)
    }
}
impl Serialize for SqlDropTableStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropTableStatementFields {
    pub drop_token: SyntaxResult<SyntaxToken>,
    pub table_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub tables: SqlTableNameList,
    pub drop_behavior: Option<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropTriggerStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropTriggerStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropTriggerStatementFields {
        SqlDropTriggerStatementFields {
            drop_token: self.drop_token(),
            trigger_token: self.trigger_token(),
            if_token: self.if_token(),
            exists_token: self.exists_token(),
            name: self.name(),
            on_token: self.on_token(),
            table: self.table(),
            drop_behavior: self.drop_behavior(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn drop_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn trigger_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 4usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 6usize)
    }
    pub fn drop_behavior(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 8usize)
    }
}
impl Serialize for SqlDropTriggerStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropTriggerStatementFields {
    pub drop_token: SyntaxResult<SyntaxToken>,
    pub trigger_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub name: SyntaxResult<AnySqlName>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableName>,
    pub drop_behavior: Option<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlDropViewStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlDropViewStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlDropViewStatementFields {
        SqlDropViewStatementFields {
            drop_token: self.drop_token(),
            view_token: self.view_token(),
            if_token: self.if_token(),
            exists_token: self.exists_token(),
            views: self.views(),
            drop_behavior: self.drop_behavior(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn drop_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn view_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn if_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 2usize)
    }
    pub fn exists_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn views(&self) -> SqlTableNameList {
        support::list(&self.syntax, 4usize)
    }
    pub fn drop_behavior(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 5usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 6usize)
    }
}
impl Serialize for SqlDropViewStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlDropViewStatementFields {
    pub drop_token: SyntaxResult<SyntaxToken>,
    pub view_token: SyntaxResult<SyntaxToken>,
    pub if_token: Option<SyntaxToken>,
    pub exists_token: Option<SyntaxToken>,
    pub views: SqlTableNameList,
    pub drop_behavior: Option<SyntaxToken>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlEmptyStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlEmptyStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlEmptyStatementFields {
        SqlEmptyStatementFields {
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn semicolon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlEmptyStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlEmptyStatementFields {
    pub semicolon_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlExistsExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlExistsExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlExistsExpressionFields {
        SqlExistsExpressionFields {
            exists_token: self.exists_token(),
            subquery: self.subquery(),
        }
    }
    pub fn exists_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn subquery(&self) -> SyntaxResult<SqlSubqueryExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlExistsExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlExistsExpressionFields {
    pub exists_token: SyntaxResult<SyntaxToken>,
    pub subquery: SyntaxResult<SqlSubqueryExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFetchClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFetchClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFetchClauseFields {
        SqlFetchClauseFields {
            fetch_token: self.fetch_token(),
            quantifier: self.quantifier(),
            count: self.count(),
            row_or_rows: self.row_or_rows(),
            tail: self.tail(),
        }
    }
    pub fn fetch_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn quantifier(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn count(&self) -> Option<AnySqlLimitValue> {
        support::node(&self.syntax, 2usize)
    }
    pub fn row_or_rows(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn tail(&self) -> SyntaxResult<AnySqlFetchTail> {
        support::required_node(&self.syntax, 4usize)
    }
}
impl Serialize for SqlFetchClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFetchClauseFields {
    pub fetch_token: SyntaxResult<SyntaxToken>,
    pub quantifier: SyntaxResult<SyntaxToken>,
    pub count: Option<AnySqlLimitValue>,
    pub row_or_rows: SyntaxResult<SyntaxToken>,
    pub tail: SyntaxResult<AnySqlFetchTail>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFetchOnlyTail {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFetchOnlyTail {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFetchOnlyTailFields {
        SqlFetchOnlyTailFields {
            only_token: self.only_token(),
        }
    }
    pub fn only_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlFetchOnlyTail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFetchOnlyTailFields {
    pub only_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFetchWithTiesTail {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFetchWithTiesTail {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFetchWithTiesTailFields {
        SqlFetchWithTiesTailFields {
            with_token: self.with_token(),
            ties_token: self.ties_token(),
        }
    }
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ties_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlFetchWithTiesTail {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFetchWithTiesTailFields {
    pub with_token: SyntaxResult<SyntaxToken>,
    pub ties_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFilterClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFilterClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFilterClauseFields {
        SqlFilterClauseFields {
            filter_token: self.filter_token(),
            l_paren_token: self.l_paren_token(),
            where_token: self.where_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn filter_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn where_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for SqlFilterClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFilterClauseFields {
    pub filter_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub where_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFromClauseFields {
        SqlFromClauseFields {
            from_token: self.from_token(),
            items: self.items(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlFromItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlFromClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFromClauseFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub items: SqlFromItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFromItem {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFromItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFromItemFields {
        SqlFromItemFields {
            source: self.source(),
            joins: self.joins(),
        }
    }
    pub fn source(&self) -> SyntaxResult<AnySqlFromExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn joins(&self) -> SqlJoinClauseList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlFromItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFromItemFields {
    pub source: SyntaxResult<AnySqlFromExpression>,
    pub joins: SqlJoinClauseList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFunctionBinding {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFunctionBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFunctionBindingFields {
        SqlFunctionBindingFields {
            lateral_token: self.lateral_token(),
            schema: self.schema(),
            name: self.name(),
            l_paren_token: self.l_paren_token(),
            arguments: self.arguments(),
            r_paren_token: self.r_paren_token(),
            alias: self.alias(),
        }
    }
    pub fn lateral_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn schema(&self) -> Option<SqlShemaName> {
        support::node(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn arguments(&self) -> SqlExpressionList {
        support::list(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn alias(&self) -> Option<SqlAlias> {
        support::node(&self.syntax, 6usize)
    }
}
impl Serialize for SqlFunctionBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFunctionBindingFields {
    pub lateral_token: Option<SyntaxToken>,
    pub schema: Option<SqlShemaName>,
    pub name: SyntaxResult<AnySqlName>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub arguments: SqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub alias: Option<SqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlFunctionParameter {
    pub(crate) syntax: SyntaxNode,
}
impl SqlFunctionParameter {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlFunctionParameterFields {
        SqlFunctionParameterFields {
            mode: self.mode(),
            name: self.name(),
            ty: self.ty(),
            default: self.default(),
        }
    }
    pub fn mode(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> Option<SqlName> {
        support::node(&self.syntax, 1usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn default(&self) -> Option<SqlParameterDefault> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlFunctionParameter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlFunctionParameterFields {
    pub mode: Option<SyntaxToken>,
    pub name: Option<SqlName>,
    pub ty: SyntaxResult<SqlTypeName>,
    pub default: Option<SqlParameterDefault>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlGrantStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlGrantStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlGrantStatementFields {
        SqlGrantStatementFields {
            grant_token: self.grant_token(),
            all_token: self.all_token(),
            on_token: self.on_token(),
            table_token: self.table_token(),
            objects: self.objects(),
            to_token: self.to_token(),
            grantees: self.grantees(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn grant_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn all_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn table_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
    pub fn objects(&self) -> SqlTableNameList {
        support::list(&self.syntax, 4usize)
    }
    pub fn to_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
    pub fn grantees(&self) -> SqlGranteeList {
        support::list(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for SqlGrantStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlGrantStatementFields {
    pub grant_token: SyntaxResult<SyntaxToken>,
    pub all_token: SyntaxResult<SyntaxToken>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub table_token: Option<SyntaxToken>,
    pub objects: SqlTableNameList,
    pub to_token: SyntaxResult<SyntaxToken>,
    pub grantees: SqlGranteeList,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlGroupByClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlGroupByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlGroupByClauseFields {
        SqlGroupByClauseFields {
            group_by_token: self.group_by_token(),
            items: self.items(),
        }
    }
    pub fn group_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlGroupByItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlGroupByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlGroupByClauseFields {
    pub group_by_token: SyntaxResult<SyntaxToken>,
    pub items: SqlGroupByItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlHavingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlHavingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlHavingClauseFields {
        SqlHavingClauseFields {
            having_token: self.having_token(),
            condition: self.condition(),
        }
    }
    pub fn having_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlHavingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlHavingClauseFields {
    pub having_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlInExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlInExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlInExpressionFields {
        SqlInExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            in_token: self.in_token(),
            source: self.source(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn in_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn source(&self) -> SyntaxResult<AnySqlInSource> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlInExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlInExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub in_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnySqlInSource>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlInValueList {
    pub(crate) syntax: SyntaxNode,
}
impl SqlInValueList {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlInValueListFields {
        SqlInValueListFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlInValueList {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlInValueListFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlInsertStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlInsertStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlInsertStatementFields {
        SqlInsertStatementFields {
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
    pub fn with_clause(&self) -> Option<SqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn insert_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn into_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableBinding> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn columns(&self) -> Option<SqlColumnList> {
        support::node(&self.syntax, 4usize)
    }
    pub fn source(&self) -> SyntaxResult<AnySqlInsertSource> {
        support::required_node(&self.syntax, 5usize)
    }
    pub fn on_conflict_clause(&self) -> Option<SqlOnConflictClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn returning_clause(&self) -> Option<SqlReturningClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 8usize)
    }
}
impl Serialize for SqlInsertStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlInsertStatementFields {
    pub with_clause: Option<SqlWithClause>,
    pub insert_token: SyntaxResult<SyntaxToken>,
    pub into_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableBinding>,
    pub columns: Option<SqlColumnList>,
    pub source: SyntaxResult<AnySqlInsertSource>,
    pub on_conflict_clause: Option<SqlOnConflictClause>,
    pub returning_clause: Option<SqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlIntervalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlIntervalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlIntervalExpressionFields {
        SqlIntervalExpressionFields {
            interval_token: self.interval_token(),
            value: self.value(),
        }
    }
    pub fn interval_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SqlStringLiteralExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlIntervalExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlIntervalExpressionFields {
    pub interval_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SqlStringLiteralExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlIsNullExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlIsNullExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlIsNullExpressionFields {
        SqlIsNullExpressionFields {
            expression: self.expression(),
            is_token: self.is_token(),
            not_token: self.not_token(),
            null_token: self.null_token(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
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
impl Serialize for SqlIsNullExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlIsNullExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub is_token: SyntaxResult<SyntaxToken>,
    pub not_token: Option<SyntaxToken>,
    pub null_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlJoinClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlJoinClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlJoinClauseFields {
        SqlJoinClauseFields {
            join_type: self.join_type(),
            outer_token: self.outer_token(),
            join_token: self.join_token(),
            source: self.source(),
            on_token: self.on_token(),
            condition: self.condition(),
            using_clause: self.using_clause(),
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
    pub fn source(&self) -> SyntaxResult<AnySqlFromExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn on_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 4usize)
    }
    pub fn condition(&self) -> Option<AnySqlExpression> {
        support::node(&self.syntax, 5usize)
    }
    pub fn using_clause(&self) -> Option<SqlJoinUsingClause> {
        support::node(&self.syntax, 6usize)
    }
}
impl Serialize for SqlJoinClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlJoinClauseFields {
    pub join_type: Option<SyntaxToken>,
    pub outer_token: Option<SyntaxToken>,
    pub join_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnySqlFromExpression>,
    pub on_token: Option<SyntaxToken>,
    pub condition: Option<AnySqlExpression>,
    pub using_clause: Option<SqlJoinUsingClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlJoinUsingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlJoinUsingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlJoinUsingClauseFields {
        SqlJoinUsingClauseFields {
            using_token: self.using_token(),
            columns: self.columns(),
        }
    }
    pub fn using_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn columns(&self) -> SyntaxResult<SqlColumnList> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlJoinUsingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlJoinUsingClauseFields {
    pub using_token: SyntaxResult<SyntaxToken>,
    pub columns: SyntaxResult<SqlColumnList>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlLanguageOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlLanguageOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlLanguageOptionFields {
        SqlLanguageOptionFields {
            language_token: self.language_token(),
            name: self.name(),
        }
    }
    pub fn language_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlLanguageOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlLanguageOptionFields {
    pub language_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlLikeExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlLikeExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlLikeExpressionFields {
        SqlLikeExpressionFields {
            expression: self.expression(),
            not_token: self.not_token(),
            operator_token: self.operator_token(),
            pattern: self.pattern(),
        }
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn not_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn pattern(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlLikeExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlLikeExpressionFields {
    pub expression: SyntaxResult<AnySqlExpression>,
    pub not_token: Option<SyntaxToken>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub pattern: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlLimitClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlLimitClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlLimitClauseFields {
        SqlLimitClauseFields {
            limit_token: self.limit_token(),
            limit_count: self.limit_count(),
        }
    }
    pub fn limit_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn limit_count(&self) -> SyntaxResult<AnySqlLimitValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlLimitClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlLimitClauseFields {
    pub limit_token: SyntaxResult<SyntaxToken>,
    pub limit_count: SyntaxResult<AnySqlLimitValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlLogicalExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlLogicalExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlLogicalExpressionFields {
        SqlLogicalExpressionFields {
            left: self.left(),
            operator_token: self.operator_token(),
            right: self.right(),
        }
    }
    pub fn left(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn right(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlLogicalExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlLogicalExpressionFields {
    pub left: SyntaxResult<AnySqlExpression>,
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub right: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlNameFields {
        SqlNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlNullLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlNullLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlNullLiteralExpressionFields {
        SqlNullLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlNullLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlNullLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlNumberLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlNumberLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlNumberLiteralExpressionFields {
        SqlNumberLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlNumberLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlNumberLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlOffsetClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlOffsetClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlOffsetClauseFields {
        SqlOffsetClauseFields {
            offset_token: self.offset_token(),
            start: self.start(),
        }
    }
    pub fn offset_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn start(&self) -> SyntaxResult<AnySqlLimitValue> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlOffsetClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlOffsetClauseFields {
    pub offset_token: SyntaxResult<SyntaxToken>,
    pub start: SyntaxResult<AnySqlLimitValue>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlOnConflictClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlOnConflictClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlOnConflictClauseFields {
        SqlOnConflictClauseFields {
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
    pub fn target(&self) -> Option<AnySqlConflictTarget> {
        support::node(&self.syntax, 2usize)
    }
    pub fn action(&self) -> SyntaxResult<AnySqlConflictAction> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlOnConflictClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlOnConflictClauseFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub conflict_token: SyntaxResult<SyntaxToken>,
    pub target: Option<AnySqlConflictTarget>,
    pub action: SyntaxResult<AnySqlConflictAction>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlOnConstraintClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlOnConstraintClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlOnConstraintClauseFields {
        SqlOnConstraintClauseFields {
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
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlOnConstraintClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlOnConstraintClauseFields {
    pub on_token: SyntaxResult<SyntaxToken>,
    pub constraint_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlOrderByClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlOrderByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlOrderByClauseFields {
        SqlOrderByClauseFields {
            order_by_token: self.order_by_token(),
            items: self.items(),
        }
    }
    pub fn order_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlOrderByExpressionList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlOrderByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlOrderByClauseFields {
    pub order_by_token: SyntaxResult<SyntaxToken>,
    pub items: SqlOrderByExpressionList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlOrderByExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlOrderByExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlOrderByExpressionFields {
        SqlOrderByExpressionFields {
            item: self.item(),
            order: self.order(),
        }
    }
    pub fn item(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn order(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlOrderByExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlOrderByExpressionFields {
    pub item: SyntaxResult<AnySqlExpression>,
    pub order: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlParameterDefault {
    pub(crate) syntax: SyntaxNode,
}
impl SqlParameterDefault {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlParameterDefaultFields {
        SqlParameterDefaultFields {
            marker: self.marker(),
            value: self.value(),
        }
    }
    pub fn marker(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlParameterDefault {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlParameterDefaultFields {
    pub marker: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlParameterExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlParameterExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlParameterExpressionFields {
        SqlParameterExpressionFields {
            colon_token: self.colon_token(),
            name: self.name(),
        }
    }
    pub fn colon_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlParameterExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlParameterExpressionFields {
    pub colon_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlParenthesizedExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlParenthesizedExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlParenthesizedExpressionFields {
        SqlParenthesizedExpressionFields {
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlParenthesizedExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlParenthesizedExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnySqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlParenthesizedJoinBinding {
    pub(crate) syntax: SyntaxNode,
}
impl SqlParenthesizedJoinBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlParenthesizedJoinBindingFields {
        SqlParenthesizedJoinBindingFields {
            l_paren_token: self.l_paren_token(),
            source: self.source(),
            joins: self.joins(),
            r_paren_token: self.r_paren_token(),
            alias: self.alias(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn source(&self) -> SyntaxResult<AnySqlFromExpression> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn joins(&self) -> SqlJoinClauseList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn alias(&self) -> Option<SqlAlias> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for SqlParenthesizedJoinBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlParenthesizedJoinBindingFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub source: SyntaxResult<AnySqlFromExpression>,
    pub joins: SqlJoinClauseList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub alias: Option<SqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlPolicyForClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlPolicyForClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlPolicyForClauseFields {
        SqlPolicyForClauseFields {
            for_token: self.for_token(),
            command: self.command(),
        }
    }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn command(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlPolicyForClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlPolicyForClauseFields {
    pub for_token: SyntaxResult<SyntaxToken>,
    pub command: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlPolicyUsingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlPolicyUsingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlPolicyUsingClauseFields {
        SqlPolicyUsingClauseFields {
            using_token: self.using_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn using_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlPolicyUsingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlPolicyUsingClauseFields {
    pub using_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlPolicyWithCheckClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlPolicyWithCheckClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlPolicyWithCheckClauseFields {
        SqlPolicyWithCheckClauseFields {
            with_token: self.with_token(),
            check_token: self.check_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn check_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for SqlPolicyWithCheckClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlPolicyWithCheckClauseFields {
    pub with_token: SyntaxResult<SyntaxToken>,
    pub check_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlPrecisionModifier {
    pub(crate) syntax: SyntaxNode,
}
impl SqlPrecisionModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlPrecisionModifierFields {
        SqlPrecisionModifierFields {
            precision_token: self.precision_token(),
        }
    }
    pub fn precision_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlPrecisionModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlPrecisionModifierFields {
    pub precision_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturningClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturningClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturningClauseFields {
        SqlReturningClauseFields {
            returning_token: self.returning_token(),
            items: self.items(),
        }
    }
    pub fn returning_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlSelectItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlReturningClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturningClauseFields {
    pub returning_token: SyntaxResult<SyntaxToken>,
    pub items: SqlSelectItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsClauseFields {
        SqlReturnsClauseFields {
            returns_token: self.returns_token(),
            ty: self.ty(),
        }
    }
    pub fn returns_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<AnySqlReturnsType> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlReturnsClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsClauseFields {
    pub returns_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<AnySqlReturnsType>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsNullOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsNullOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsNullOptionFields {
        SqlReturnsNullOptionFields {
            returns_token: self.returns_token(),
            first_null_token: self.first_null_token(),
            on_token: self.on_token(),
            second_null_token: self.second_null_token(),
            input_token: self.input_token(),
        }
    }
    pub fn returns_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn first_null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn on_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn second_null_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn input_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
}
impl Serialize for SqlReturnsNullOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsNullOptionFields {
    pub returns_token: SyntaxResult<SyntaxToken>,
    pub first_null_token: SyntaxResult<SyntaxToken>,
    pub on_token: SyntaxResult<SyntaxToken>,
    pub second_null_token: SyntaxResult<SyntaxToken>,
    pub input_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsSetofClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsSetofClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsSetofClauseFields {
        SqlReturnsSetofClauseFields {
            setof_token: self.setof_token(),
            ty: self.ty(),
        }
    }
    pub fn setof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlReturnsSetofClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsSetofClauseFields {
    pub setof_token: SyntaxResult<SyntaxToken>,
    pub ty: SyntaxResult<SqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsTableClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsTableClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsTableClauseFields {
        SqlReturnsTableClauseFields {
            table_token: self.table_token(),
            l_paren_token: self.l_paren_token(),
            columns: self.columns(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn table_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn columns(&self) -> SqlReturnsTableColumnList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlReturnsTableClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsTableClauseFields {
    pub table_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub columns: SqlReturnsTableColumnList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsTableColumn {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsTableColumn {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsTableColumnFields {
        SqlReturnsTableColumnFields {
            name: self.name(),
            ty: self.ty(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn ty(&self) -> SyntaxResult<SqlTypeName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlReturnsTableColumn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsTableColumnFields {
    pub name: SyntaxResult<SqlName>,
    pub ty: SyntaxResult<SqlTypeName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlReturnsTriggerClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlReturnsTriggerClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlReturnsTriggerClauseFields {
        SqlReturnsTriggerClauseFields {
            trigger_token: self.trigger_token(),
        }
    }
    pub fn trigger_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlReturnsTriggerClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlReturnsTriggerClauseFields {
    pub trigger_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlRoot {
    pub(crate) syntax: SyntaxNode,
}
impl SqlRoot {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlRootFields {
        SqlRootFields {
            bom_token: self.bom_token(),
            stmt: self.stmt(),
            eof_token: self.eof_token(),
        }
    }
    pub fn bom_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn stmt(&self) -> SqlStatementList {
        support::list(&self.syntax, 1usize)
    }
    pub fn eof_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlRoot {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlRootFields {
    pub bom_token: Option<SyntaxToken>,
    pub stmt: SqlStatementList,
    pub eof_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSecurityOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSecurityOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSecurityOptionFields {
        SqlSecurityOptionFields {
            security_token: self.security_token(),
            value: self.value(),
        }
    }
    pub fn security_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSecurityOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSecurityOptionFields {
    pub security_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSelectAllQuantifier {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSelectAllQuantifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSelectAllQuantifierFields {
        SqlSelectAllQuantifierFields {
            all_token: self.all_token(),
        }
    }
    pub fn all_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlSelectAllQuantifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSelectAllQuantifierFields {
    pub all_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSelectClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSelectClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSelectClauseFields {
        SqlSelectClauseFields {
            select_token: self.select_token(),
            quantifier: self.quantifier(),
            list: self.list(),
        }
    }
    pub fn select_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn quantifier(&self) -> Option<AnySqlSelectQuantifier> {
        support::node(&self.syntax, 1usize)
    }
    pub fn list(&self) -> SqlSelectItemList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for SqlSelectClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSelectClauseFields {
    pub select_token: SyntaxResult<SyntaxToken>,
    pub quantifier: Option<AnySqlSelectQuantifier>,
    pub list: SqlSelectItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSelectDistinctQuantifier {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSelectDistinctQuantifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSelectDistinctQuantifierFields {
        SqlSelectDistinctQuantifierFields {
            distinct_token: self.distinct_token(),
            on_clause: self.on_clause(),
        }
    }
    pub fn distinct_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn on_clause(&self) -> Option<SqlDistinctOnClause> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSelectDistinctQuantifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSelectDistinctQuantifierFields {
    pub distinct_token: SyntaxResult<SyntaxToken>,
    pub on_clause: Option<SqlDistinctOnClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSelectExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSelectExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSelectExpressionFields {
        SqlSelectExpressionFields {
            expr: self.expr(),
            alias: self.alias(),
        }
    }
    pub fn expr(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn alias(&self) -> Option<SqlAlias> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSelectExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSelectExpressionFields {
    pub expr: SyntaxResult<AnySqlExpression>,
    pub alias: Option<SqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSelectStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSelectStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSelectStatementFields {
        SqlSelectStatementFields {
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
            fetch_clause: self.fetch_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<SqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn select_clause(&self) -> SyntaxResult<SqlSelectClause> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn from_clause(&self) -> Option<SqlFromClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn where_clause(&self) -> Option<SqlWhereClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn group_by_clause(&self) -> Option<SqlGroupByClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn having_clause(&self) -> Option<SqlHavingClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn set_operations(&self) -> SqlSetOperationList {
        support::list(&self.syntax, 6usize)
    }
    pub fn order_by_clause(&self) -> Option<SqlOrderByClause> {
        support::node(&self.syntax, 7usize)
    }
    pub fn limit_clause(&self) -> Option<SqlLimitClause> {
        support::node(&self.syntax, 8usize)
    }
    pub fn offset_clause(&self) -> Option<SqlOffsetClause> {
        support::node(&self.syntax, 9usize)
    }
    pub fn fetch_clause(&self) -> Option<SqlFetchClause> {
        support::node(&self.syntax, 10usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 11usize)
    }
}
impl Serialize for SqlSelectStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSelectStatementFields {
    pub with_clause: Option<SqlWithClause>,
    pub select_clause: SyntaxResult<SqlSelectClause>,
    pub from_clause: Option<SqlFromClause>,
    pub where_clause: Option<SqlWhereClause>,
    pub group_by_clause: Option<SqlGroupByClause>,
    pub having_clause: Option<SqlHavingClause>,
    pub set_operations: SqlSetOperationList,
    pub order_by_clause: Option<SqlOrderByClause>,
    pub limit_clause: Option<SqlLimitClause>,
    pub offset_clause: Option<SqlOffsetClause>,
    pub fetch_clause: Option<SqlFetchClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSetClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSetClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSetClauseFields {
        SqlSetClauseFields {
            set_token: self.set_token(),
            items: self.items(),
        }
    }
    pub fn set_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlSetItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSetClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSetClauseFields {
    pub set_token: SyntaxResult<SyntaxToken>,
    pub items: SqlSetItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSetItem {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSetItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSetItemFields {
        SqlSetItemFields {
            column: self.column(),
            eq_token: self.eq_token(),
            expr: self.expr(),
        }
    }
    pub fn column(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expr(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlSetItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSetItemFields {
    pub column: SyntaxResult<SqlName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub expr: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSetOperation {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSetOperation {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSetOperationFields {
        SqlSetOperationFields {
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
    pub fn select_clause(&self) -> SyntaxResult<SqlSelectClause> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn from_clause(&self) -> Option<SqlFromClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn where_clause(&self) -> Option<SqlWhereClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn group_by_clause(&self) -> Option<SqlGroupByClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn having_clause(&self) -> Option<SqlHavingClause> {
        support::node(&self.syntax, 6usize)
    }
}
impl Serialize for SqlSetOperation {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSetOperationFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub quantifier: Option<SyntaxToken>,
    pub select_clause: SyntaxResult<SqlSelectClause>,
    pub from_clause: Option<SqlFromClause>,
    pub where_clause: Option<SqlWhereClause>,
    pub group_by_clause: Option<SqlGroupByClause>,
    pub having_clause: Option<SqlHavingClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlShemaName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlShemaName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlShemaNameFields {
        SqlShemaNameFields {
            base: self.base(),
            name: self.name(),
            dot_token: self.dot_token(),
        }
    }
    pub fn base(&self) -> Option<SqlDataBaseName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlShemaName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlShemaNameFields {
    pub base: Option<SqlDataBaseName>,
    pub name: SyntaxResult<SqlName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlStar {
    pub(crate) syntax: SyntaxNode,
}
impl SqlStar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlStarFields {
        SqlStarFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlStar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlStarFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlStrictOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlStrictOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlStrictOptionFields {
        SqlStrictOptionFields {
            strict_token: self.strict_token(),
        }
    }
    pub fn strict_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlStrictOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlStrictOptionFields {
    pub strict_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlStringLiteralExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlStringLiteralExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlStringLiteralExpressionFields {
        SqlStringLiteralExpressionFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlStringLiteralExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlStringLiteralExpressionFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSubqueryBinding {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSubqueryBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSubqueryBindingFields {
        SqlSubqueryBindingFields {
            lateral_token: self.lateral_token(),
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
            alias: self.alias(),
        }
    }
    pub fn lateral_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn query(&self) -> SyntaxResult<AnySqlSubqueryBody> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
    pub fn alias(&self) -> Option<SqlAlias> {
        support::node(&self.syntax, 4usize)
    }
}
impl Serialize for SqlSubqueryBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSubqueryBindingFields {
    pub lateral_token: Option<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnySqlSubqueryBody>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
    pub alias: Option<SqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSubqueryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSubqueryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSubqueryExpressionFields {
        SqlSubqueryExpressionFields {
            l_paren_token: self.l_paren_token(),
            query: self.query(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn query(&self) -> SyntaxResult<AnySqlSubqueryBody> {
        support::required_node(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlSubqueryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSubqueryExpressionFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub query: SyntaxResult<AnySqlSubqueryBody>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSubstringExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSubstringExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSubstringExpressionFields {
        SqlSubstringExpressionFields {
            name_token: self.name_token(),
            l_paren_token: self.l_paren_token(),
            expression: self.expression(),
            from_clause: self.from_clause(),
            for_clause: self.for_clause(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn name_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn from_clause(&self) -> Option<SqlSubstringFromClause> {
        support::node(&self.syntax, 3usize)
    }
    pub fn for_clause(&self) -> Option<SqlSubstringForClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for SqlSubstringExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSubstringExpressionFields {
    pub name_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnySqlExpression>,
    pub from_clause: Option<SqlSubstringFromClause>,
    pub for_clause: Option<SqlSubstringForClause>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSubstringForClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSubstringForClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSubstringForClauseFields {
        SqlSubstringForClauseFields {
            for_token: self.for_token(),
            value: self.value(),
        }
    }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSubstringForClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSubstringForClauseFields {
    pub for_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlSubstringFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlSubstringFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlSubstringFromClauseFields {
        SqlSubstringFromClauseFields {
            from_token: self.from_token(),
            value: self.value(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn value(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlSubstringFromClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlSubstringFromClauseFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTableBinding {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTableBinding {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTableBindingFields {
        SqlTableBindingFields {
            table: self.table(),
            alias: self.alias(),
        }
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn alias(&self) -> Option<SqlAlias> {
        support::node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlTableBinding {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTableBindingFields {
    pub table: SyntaxResult<SqlTableName>,
    pub alias: Option<SqlAlias>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTableColReference {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTableColReference {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTableColReferenceFields {
        SqlTableColReferenceFields {
            table: self.table(),
            dot_token: self.dot_token(),
            name: self.name(),
        }
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTableColReference {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTableColReferenceFields {
    pub table: SyntaxResult<SqlTableName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTableName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTableName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTableNameFields {
        SqlTableNameFields {
            schema: self.schema(),
            name: self.name(),
        }
    }
    pub fn schema(&self) -> Option<SqlShemaName> {
        support::node(&self.syntax, 0usize)
    }
    pub fn name(&self) -> SyntaxResult<AnySqlName> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlTableName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTableNameFields {
    pub schema: Option<SqlShemaName>,
    pub name: SyntaxResult<AnySqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTableStar {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTableStar {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTableStarFields {
        SqlTableStarFields {
            table: self.table(),
            dot_token: self.dot_token(),
            star: self.star(),
        }
    }
    pub fn table(&self) -> SyntaxResult<SqlTableName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn dot_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn star(&self) -> SyntaxResult<SqlStar> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTableStar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTableStarFields {
    pub table: SyntaxResult<SqlTableName>,
    pub dot_token: SyntaxResult<SyntaxToken>,
    pub star: SyntaxResult<SqlStar>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTildeArrayExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTildeArrayExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTildeArrayExpressionFields {
        SqlTildeArrayExpressionFields {
            array_token: self.array_token(),
            open_tilde_token: self.open_tilde_token(),
            l_brack_token: self.l_brack_token(),
            items: self.items(),
            r_brack_token: self.r_brack_token(),
            close_tilde_token: self.close_tilde_token(),
        }
    }
    pub fn array_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn open_tilde_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn items(&self) -> SqlExpressionList {
        support::list(&self.syntax, 3usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 4usize)
    }
    pub fn close_tilde_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 5usize)
    }
}
impl Serialize for SqlTildeArrayExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTildeArrayExpressionFields {
    pub array_token: SyntaxResult<SyntaxToken>,
    pub open_tilde_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub items: SqlExpressionList,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub close_tilde_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTildeArraySuffix {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTildeArraySuffix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTildeArraySuffixFields {
        SqlTildeArraySuffixFields {
            open_tilde_token: self.open_tilde_token(),
            l_brack_token: self.l_brack_token(),
            r_brack_token: self.r_brack_token(),
            close_tilde_token: self.close_tilde_token(),
        }
    }
    pub fn open_tilde_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn r_brack_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn close_tilde_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlTildeArraySuffix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTildeArraySuffixFields {
    pub open_tilde_token: SyntaxResult<SyntaxToken>,
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
    pub close_tilde_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTildeName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTildeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTildeNameFields {
        SqlTildeNameFields {
            value_token: self.value_token(),
        }
    }
    pub fn value_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlTildeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTildeNameFields {
    pub value_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTimeZoneModifier {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTimeZoneModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTimeZoneModifierFields {
        SqlTimeZoneModifierFields {
            with_or_without: self.with_or_without(),
            time_token: self.time_token(),
            zone_token: self.zone_token(),
        }
    }
    pub fn with_or_without(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn time_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn zone_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTimeZoneModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTimeZoneModifierFields {
    pub with_or_without: SyntaxResult<SyntaxToken>,
    pub time_token: SyntaxResult<SyntaxToken>,
    pub zone_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerEvent {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerEvent {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerEventFields {
        SqlTriggerEventFields {
            or_token: self.or_token(),
            kind: self.kind(),
            of_clause: self.of_clause(),
        }
    }
    pub fn or_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 0usize)
    }
    pub fn kind(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn of_clause(&self) -> Option<SqlTriggerUpdateOfClause> {
        support::node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTriggerEvent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerEventFields {
    pub or_token: Option<SyntaxToken>,
    pub kind: SyntaxResult<SyntaxToken>,
    pub of_clause: Option<SqlTriggerUpdateOfClause>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerForEachClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerForEachClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerForEachClauseFields {
        SqlTriggerForEachClauseFields {
            for_token: self.for_token(),
            each_token: self.each_token(),
            granularity: self.granularity(),
        }
    }
    pub fn for_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn each_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn granularity(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTriggerForEachClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerForEachClauseFields {
    pub for_token: SyntaxResult<SyntaxToken>,
    pub each_token: SyntaxResult<SyntaxToken>,
    pub granularity: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerReferencingClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerReferencingClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerReferencingClauseFields {
        SqlTriggerReferencingClauseFields {
            referencing_token: self.referencing_token(),
            items: self.items(),
        }
    }
    pub fn referencing_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlTriggerReferencingItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlTriggerReferencingClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerReferencingClauseFields {
    pub referencing_token: SyntaxResult<SyntaxToken>,
    pub items: SqlTriggerReferencingItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerReferencingItem {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerReferencingItem {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerReferencingItemFields {
        SqlTriggerReferencingItemFields {
            which_token: self.which_token(),
            table_token: self.table_token(),
            as_token: self.as_token(),
            name: self.name(),
        }
    }
    pub fn which_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn table_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn as_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlTriggerReferencingItem {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerReferencingItemFields {
    pub which_token: SyntaxResult<SyntaxToken>,
    pub table_token: SyntaxResult<SyntaxToken>,
    pub as_token: SyntaxResult<SyntaxToken>,
    pub name: SyntaxResult<SqlName>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerUpdateOfClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerUpdateOfClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerUpdateOfClauseFields {
        SqlTriggerUpdateOfClauseFields {
            of_token: self.of_token(),
            columns: self.columns(),
        }
    }
    pub fn of_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn columns(&self) -> SqlColumnNameList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlTriggerUpdateOfClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerUpdateOfClauseFields {
    pub of_token: SyntaxResult<SyntaxToken>,
    pub columns: SqlColumnNameList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTriggerWhenClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTriggerWhenClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTriggerWhenClauseFields {
        SqlTriggerWhenClauseFields {
            when_token: self.when_token(),
            l_paren_token: self.l_paren_token(),
            condition: self.condition(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn when_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlTriggerWhenClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTriggerWhenClauseFields {
    pub when_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTypeArguments {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTypeArguments {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTypeArgumentsFields {
        SqlTypeArgumentsFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlTypeArgumentList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlTypeArguments {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTypeArgumentsFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlTypeArgumentList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTypeArraySuffix {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTypeArraySuffix {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTypeArraySuffixFields {
        SqlTypeArraySuffixFields {
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
impl Serialize for SqlTypeArraySuffix {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTypeArraySuffixFields {
    pub l_brack_token: SyntaxResult<SyntaxToken>,
    pub r_brack_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlTypeName {
    pub(crate) syntax: SyntaxNode,
}
impl SqlTypeName {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlTypeNameFields {
        SqlTypeNameFields {
            name: self.name(),
            args: self.args(),
            modifier: self.modifier(),
            array_suffix: self.array_suffix(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn args(&self) -> Option<SqlTypeArguments> {
        support::node(&self.syntax, 1usize)
    }
    pub fn modifier(&self) -> Option<AnySqlTypeModifier> {
        support::node(&self.syntax, 2usize)
    }
    pub fn array_suffix(&self) -> Option<AnySqlTypeArraySuffix> {
        support::node(&self.syntax, 3usize)
    }
}
impl Serialize for SqlTypeName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlTypeNameFields {
    pub name: SyntaxResult<SyntaxToken>,
    pub args: Option<SqlTypeArguments>,
    pub modifier: Option<AnySqlTypeModifier>,
    pub array_suffix: Option<AnySqlTypeArraySuffix>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlUnaryExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlUnaryExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlUnaryExpressionFields {
        SqlUnaryExpressionFields {
            operator_token: self.operator_token(),
            expression: self.expression(),
        }
    }
    pub fn operator_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn expression(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlUnaryExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlUnaryExpressionFields {
    pub operator_token: SyntaxResult<SyntaxToken>,
    pub expression: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlUpdateFromClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlUpdateFromClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlUpdateFromClauseFields {
        SqlUpdateFromClauseFields {
            from_token: self.from_token(),
            items: self.items(),
        }
    }
    pub fn from_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlFromItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlUpdateFromClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlUpdateFromClauseFields {
    pub from_token: SyntaxResult<SyntaxToken>,
    pub items: SqlFromItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlUpdateStatement {
    pub(crate) syntax: SyntaxNode,
}
impl SqlUpdateStatement {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlUpdateStatementFields {
        SqlUpdateStatementFields {
            with_clause: self.with_clause(),
            update_token: self.update_token(),
            table: self.table(),
            set_clause: self.set_clause(),
            from_clause: self.from_clause(),
            where_clause: self.where_clause(),
            returning_clause: self.returning_clause(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<SqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn update_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn table(&self) -> SyntaxResult<SqlTableBinding> {
        support::required_node(&self.syntax, 2usize)
    }
    pub fn set_clause(&self) -> SyntaxResult<SqlSetClause> {
        support::required_node(&self.syntax, 3usize)
    }
    pub fn from_clause(&self) -> Option<SqlUpdateFromClause> {
        support::node(&self.syntax, 4usize)
    }
    pub fn where_clause(&self) -> Option<SqlWhereClause> {
        support::node(&self.syntax, 5usize)
    }
    pub fn returning_clause(&self) -> Option<SqlReturningClause> {
        support::node(&self.syntax, 6usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 7usize)
    }
}
impl Serialize for SqlUpdateStatement {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlUpdateStatementFields {
    pub with_clause: Option<SqlWithClause>,
    pub update_token: SyntaxResult<SyntaxToken>,
    pub table: SyntaxResult<SqlTableBinding>,
    pub set_clause: SyntaxResult<SqlSetClause>,
    pub from_clause: Option<SqlUpdateFromClause>,
    pub where_clause: Option<SqlWhereClause>,
    pub returning_clause: Option<SqlReturningClause>,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlValuesClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlValuesClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlValuesClauseFields {
        SqlValuesClauseFields {
            with_clause: self.with_clause(),
            values_token: self.values_token(),
            rows: self.rows(),
            semicolon_token: self.semicolon_token(),
        }
    }
    pub fn with_clause(&self) -> Option<SqlWithClause> {
        support::node(&self.syntax, 0usize)
    }
    pub fn values_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn rows(&self) -> SqlValuesRowList {
        support::list(&self.syntax, 2usize)
    }
    pub fn semicolon_token(&self) -> Option<SyntaxToken> {
        support::token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlValuesClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlValuesClauseFields {
    pub with_clause: Option<SqlWithClause>,
    pub values_token: SyntaxResult<SyntaxToken>,
    pub rows: SqlValuesRowList,
    pub semicolon_token: Option<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlValuesRow {
    pub(crate) syntax: SyntaxNode,
}
impl SqlValuesRow {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlValuesRowFields {
        SqlValuesRowFields {
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlExpressionList {
        support::list(&self.syntax, 1usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 2usize)
    }
}
impl Serialize for SqlValuesRow {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlValuesRowFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlExpressionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlVaryingModifier {
    pub(crate) syntax: SyntaxNode,
}
impl SqlVaryingModifier {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlVaryingModifierFields {
        SqlVaryingModifierFields {
            varying_token: self.varying_token(),
        }
    }
    pub fn varying_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlVaryingModifier {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlVaryingModifierFields {
    pub varying_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlViewOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlViewOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlViewOptionFields {
        SqlViewOptionFields {
            name: self.name(),
            eq_token: self.eq_token(),
            value: self.value(),
        }
    }
    pub fn name(&self) -> SyntaxResult<SqlName> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn eq_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn value(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlViewOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlViewOptionFields {
    pub name: SyntaxResult<SqlName>,
    pub eq_token: SyntaxResult<SyntaxToken>,
    pub value: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlViewOptions {
    pub(crate) syntax: SyntaxNode,
}
impl SqlViewOptions {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlViewOptionsFields {
        SqlViewOptionsFields {
            with_token: self.with_token(),
            l_paren_token: self.l_paren_token(),
            items: self.items(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn with_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn items(&self) -> SqlViewOptionList {
        support::list(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlViewOptions {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlViewOptionsFields {
    pub with_token: SyntaxResult<SyntaxToken>,
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub items: SqlViewOptionList,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlVolatilityOption {
    pub(crate) syntax: SyntaxNode,
}
impl SqlVolatilityOption {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlVolatilityOptionFields {
        SqlVolatilityOptionFields {
            value: self.value(),
        }
    }
    pub fn value(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
}
impl Serialize for SqlVolatilityOption {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlVolatilityOptionFields {
    pub value: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlWhereClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlWhereClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlWhereClauseFields {
        SqlWhereClauseFields {
            where_token: self.where_token(),
            condition: self.condition(),
        }
    }
    pub fn where_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn condition(&self) -> SyntaxResult<AnySqlExpression> {
        support::required_node(&self.syntax, 1usize)
    }
}
impl Serialize for SqlWhereClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlWhereClauseFields {
    pub where_token: SyntaxResult<SyntaxToken>,
    pub condition: SyntaxResult<AnySqlExpression>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlWindowFunctionExpression {
    pub(crate) syntax: SyntaxNode,
}
impl SqlWindowFunctionExpression {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlWindowFunctionExpressionFields {
        SqlWindowFunctionExpressionFields {
            call: self.call(),
            over_token: self.over_token(),
            window: self.window(),
        }
    }
    pub fn call(&self) -> SyntaxResult<SqlCallExpression> {
        support::required_node(&self.syntax, 0usize)
    }
    pub fn over_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 1usize)
    }
    pub fn window(&self) -> SyntaxResult<SqlWindowSpecification> {
        support::required_node(&self.syntax, 2usize)
    }
}
impl Serialize for SqlWindowFunctionExpression {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlWindowFunctionExpressionFields {
    pub call: SyntaxResult<SqlCallExpression>,
    pub over_token: SyntaxResult<SyntaxToken>,
    pub window: SyntaxResult<SqlWindowSpecification>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlWindowPartitionByClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlWindowPartitionByClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlWindowPartitionByClauseFields {
        SqlWindowPartitionByClauseFields {
            partition_by_token: self.partition_by_token(),
            items: self.items(),
        }
    }
    pub fn partition_by_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn items(&self) -> SqlWindowPartitionByItemList {
        support::list(&self.syntax, 1usize)
    }
}
impl Serialize for SqlWindowPartitionByClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlWindowPartitionByClauseFields {
    pub partition_by_token: SyntaxResult<SyntaxToken>,
    pub items: SqlWindowPartitionByItemList,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlWindowSpecification {
    pub(crate) syntax: SyntaxNode,
}
impl SqlWindowSpecification {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlWindowSpecificationFields {
        SqlWindowSpecificationFields {
            l_paren_token: self.l_paren_token(),
            partition_by_clause: self.partition_by_clause(),
            order_by_clause: self.order_by_clause(),
            r_paren_token: self.r_paren_token(),
        }
    }
    pub fn l_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 0usize)
    }
    pub fn partition_by_clause(&self) -> Option<SqlWindowPartitionByClause> {
        support::node(&self.syntax, 1usize)
    }
    pub fn order_by_clause(&self) -> Option<SqlOrderByClause> {
        support::node(&self.syntax, 2usize)
    }
    pub fn r_paren_token(&self) -> SyntaxResult<SyntaxToken> {
        support::required_token(&self.syntax, 3usize)
    }
}
impl Serialize for SqlWindowSpecification {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlWindowSpecificationFields {
    pub l_paren_token: SyntaxResult<SyntaxToken>,
    pub partition_by_clause: Option<SqlWindowPartitionByClause>,
    pub order_by_clause: Option<SqlOrderByClause>,
    pub r_paren_token: SyntaxResult<SyntaxToken>,
}
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SqlWithClause {
    pub(crate) syntax: SyntaxNode,
}
impl SqlWithClause {
    #[doc = r" Create an AstNode from a SyntaxNode without checking its kind"]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r" This function must be guarded with a call to [AstNode::can_cast]"]
    #[doc = r" or a match on [SyntaxNode::kind]"]
    #[inline]
    pub const unsafe fn new_unchecked(syntax: SyntaxNode) -> Self {
        Self { syntax }
    }
    pub fn as_fields(&self) -> SqlWithClauseFields {
        SqlWithClauseFields {
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
    pub fn ctes(&self) -> SqlCteDefinitionList {
        support::list(&self.syntax, 2usize)
    }
}
impl Serialize for SqlWithClause {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.as_fields().serialize(serializer)
    }
}
#[derive(Serialize)]
pub struct SqlWithClauseFields {
    pub with_token: SyntaxResult<SyntaxToken>,
    pub recursive_token: Option<SyntaxToken>,
    pub ctes: SqlCteDefinitionList,
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlAnyAllSource {
    SqlParenthesizedExpression(SqlParenthesizedExpression),
    SqlSubqueryExpression(SqlSubqueryExpression),
}
impl AnySqlAnyAllSource {
    pub fn as_sql_parenthesized_expression(&self) -> Option<&SqlParenthesizedExpression> {
        match &self {
            Self::SqlParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_subquery_expression(&self) -> Option<&SqlSubqueryExpression> {
        match &self {
            Self::SqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlConflictAction {
    SqlDoNothingClause(SqlDoNothingClause),
    SqlDoUpdateClause(SqlDoUpdateClause),
}
impl AnySqlConflictAction {
    pub fn as_sql_do_nothing_clause(&self) -> Option<&SqlDoNothingClause> {
        match &self {
            Self::SqlDoNothingClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_do_update_clause(&self) -> Option<&SqlDoUpdateClause> {
        match &self {
            Self::SqlDoUpdateClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlConflictTarget {
    SqlColumnList(SqlColumnList),
    SqlOnConstraintClause(SqlOnConstraintClause),
}
impl AnySqlConflictTarget {
    pub fn as_sql_column_list(&self) -> Option<&SqlColumnList> {
        match &self {
            Self::SqlColumnList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_on_constraint_clause(&self) -> Option<&SqlOnConstraintClause> {
        match &self {
            Self::SqlOnConstraintClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlExpression {
    AnySqlLiteralExpression(AnySqlLiteralExpression),
    SqlAnyAllExpression(SqlAnyAllExpression),
    SqlArrayExpression(SqlArrayExpression),
    SqlArraySubscriptExpression(SqlArraySubscriptExpression),
    SqlBetweenExpression(SqlBetweenExpression),
    SqlBinaryExpression(SqlBinaryExpression),
    SqlCallExpression(SqlCallExpression),
    SqlCaseExpression(SqlCaseExpression),
    SqlCastExpression(SqlCastExpression),
    SqlCastFunctionExpression(SqlCastFunctionExpression),
    SqlColReference(SqlColReference),
    SqlExistsExpression(SqlExistsExpression),
    SqlInExpression(SqlInExpression),
    SqlIntervalExpression(SqlIntervalExpression),
    SqlIsNullExpression(SqlIsNullExpression),
    SqlLikeExpression(SqlLikeExpression),
    SqlLogicalExpression(SqlLogicalExpression),
    SqlName(SqlName),
    SqlParameterExpression(SqlParameterExpression),
    SqlParenthesizedExpression(SqlParenthesizedExpression),
    SqlStar(SqlStar),
    SqlSubqueryExpression(SqlSubqueryExpression),
    SqlSubstringExpression(SqlSubstringExpression),
    SqlTableColReference(SqlTableColReference),
    SqlTableStar(SqlTableStar),
    SqlTildeArrayExpression(SqlTildeArrayExpression),
    SqlUnaryExpression(SqlUnaryExpression),
    SqlWindowFunctionExpression(SqlWindowFunctionExpression),
}
impl AnySqlExpression {
    pub fn as_any_sql_literal_expression(&self) -> Option<&AnySqlLiteralExpression> {
        match &self {
            Self::AnySqlLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_any_all_expression(&self) -> Option<&SqlAnyAllExpression> {
        match &self {
            Self::SqlAnyAllExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_array_expression(&self) -> Option<&SqlArrayExpression> {
        match &self {
            Self::SqlArrayExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_array_subscript_expression(&self) -> Option<&SqlArraySubscriptExpression> {
        match &self {
            Self::SqlArraySubscriptExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_between_expression(&self) -> Option<&SqlBetweenExpression> {
        match &self {
            Self::SqlBetweenExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_binary_expression(&self) -> Option<&SqlBinaryExpression> {
        match &self {
            Self::SqlBinaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_call_expression(&self) -> Option<&SqlCallExpression> {
        match &self {
            Self::SqlCallExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_case_expression(&self) -> Option<&SqlCaseExpression> {
        match &self {
            Self::SqlCaseExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_cast_expression(&self) -> Option<&SqlCastExpression> {
        match &self {
            Self::SqlCastExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_cast_function_expression(&self) -> Option<&SqlCastFunctionExpression> {
        match &self {
            Self::SqlCastFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_col_reference(&self) -> Option<&SqlColReference> {
        match &self {
            Self::SqlColReference(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_exists_expression(&self) -> Option<&SqlExistsExpression> {
        match &self {
            Self::SqlExistsExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_in_expression(&self) -> Option<&SqlInExpression> {
        match &self {
            Self::SqlInExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_interval_expression(&self) -> Option<&SqlIntervalExpression> {
        match &self {
            Self::SqlIntervalExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_is_null_expression(&self) -> Option<&SqlIsNullExpression> {
        match &self {
            Self::SqlIsNullExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_like_expression(&self) -> Option<&SqlLikeExpression> {
        match &self {
            Self::SqlLikeExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_logical_expression(&self) -> Option<&SqlLogicalExpression> {
        match &self {
            Self::SqlLogicalExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_name(&self) -> Option<&SqlName> {
        match &self {
            Self::SqlName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_parameter_expression(&self) -> Option<&SqlParameterExpression> {
        match &self {
            Self::SqlParameterExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_parenthesized_expression(&self) -> Option<&SqlParenthesizedExpression> {
        match &self {
            Self::SqlParenthesizedExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_star(&self) -> Option<&SqlStar> {
        match &self {
            Self::SqlStar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_subquery_expression(&self) -> Option<&SqlSubqueryExpression> {
        match &self {
            Self::SqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_substring_expression(&self) -> Option<&SqlSubstringExpression> {
        match &self {
            Self::SqlSubstringExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_table_col_reference(&self) -> Option<&SqlTableColReference> {
        match &self {
            Self::SqlTableColReference(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_table_star(&self) -> Option<&SqlTableStar> {
        match &self {
            Self::SqlTableStar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_tilde_array_expression(&self) -> Option<&SqlTildeArrayExpression> {
        match &self {
            Self::SqlTildeArrayExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_unary_expression(&self) -> Option<&SqlUnaryExpression> {
        match &self {
            Self::SqlUnaryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_window_function_expression(&self) -> Option<&SqlWindowFunctionExpression> {
        match &self {
            Self::SqlWindowFunctionExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlFetchTail {
    SqlFetchOnlyTail(SqlFetchOnlyTail),
    SqlFetchWithTiesTail(SqlFetchWithTiesTail),
}
impl AnySqlFetchTail {
    pub fn as_sql_fetch_only_tail(&self) -> Option<&SqlFetchOnlyTail> {
        match &self {
            Self::SqlFetchOnlyTail(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_fetch_with_ties_tail(&self) -> Option<&SqlFetchWithTiesTail> {
        match &self {
            Self::SqlFetchWithTiesTail(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlFromExpression {
    SqlFunctionBinding(SqlFunctionBinding),
    SqlParenthesizedJoinBinding(SqlParenthesizedJoinBinding),
    SqlSubqueryBinding(SqlSubqueryBinding),
    SqlTableBinding(SqlTableBinding),
}
impl AnySqlFromExpression {
    pub fn as_sql_function_binding(&self) -> Option<&SqlFunctionBinding> {
        match &self {
            Self::SqlFunctionBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_parenthesized_join_binding(&self) -> Option<&SqlParenthesizedJoinBinding> {
        match &self {
            Self::SqlParenthesizedJoinBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_subquery_binding(&self) -> Option<&SqlSubqueryBinding> {
        match &self {
            Self::SqlSubqueryBinding(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_table_binding(&self) -> Option<&SqlTableBinding> {
        match &self {
            Self::SqlTableBinding(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlFunctionOption {
    SqlLanguageOption(SqlLanguageOption),
    SqlReturnsNullOption(SqlReturnsNullOption),
    SqlSecurityOption(SqlSecurityOption),
    SqlStrictOption(SqlStrictOption),
    SqlVolatilityOption(SqlVolatilityOption),
}
impl AnySqlFunctionOption {
    pub fn as_sql_language_option(&self) -> Option<&SqlLanguageOption> {
        match &self {
            Self::SqlLanguageOption(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_returns_null_option(&self) -> Option<&SqlReturnsNullOption> {
        match &self {
            Self::SqlReturnsNullOption(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_security_option(&self) -> Option<&SqlSecurityOption> {
        match &self {
            Self::SqlSecurityOption(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_strict_option(&self) -> Option<&SqlStrictOption> {
        match &self {
            Self::SqlStrictOption(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_volatility_option(&self) -> Option<&SqlVolatilityOption> {
        match &self {
            Self::SqlVolatilityOption(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlInSource {
    SqlInValueList(SqlInValueList),
    SqlSubqueryExpression(SqlSubqueryExpression),
}
impl AnySqlInSource {
    pub fn as_sql_in_value_list(&self) -> Option<&SqlInValueList> {
        match &self {
            Self::SqlInValueList(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_subquery_expression(&self) -> Option<&SqlSubqueryExpression> {
        match &self {
            Self::SqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlInsertSource {
    SqlSelectStatement(SqlSelectStatement),
    SqlSubqueryExpression(SqlSubqueryExpression),
    SqlValuesClause(SqlValuesClause),
}
impl AnySqlInsertSource {
    pub fn as_sql_select_statement(&self) -> Option<&SqlSelectStatement> {
        match &self {
            Self::SqlSelectStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_subquery_expression(&self) -> Option<&SqlSubqueryExpression> {
        match &self {
            Self::SqlSubqueryExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_values_clause(&self) -> Option<&SqlValuesClause> {
        match &self {
            Self::SqlValuesClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlLimitValue {
    SqlNumberLiteralExpression(SqlNumberLiteralExpression),
    SqlParameterExpression(SqlParameterExpression),
}
impl AnySqlLimitValue {
    pub fn as_sql_number_literal_expression(&self) -> Option<&SqlNumberLiteralExpression> {
        match &self {
            Self::SqlNumberLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_parameter_expression(&self) -> Option<&SqlParameterExpression> {
        match &self {
            Self::SqlParameterExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlLiteralExpression {
    SqlBooleanLiteralExpression(SqlBooleanLiteralExpression),
    SqlNullLiteralExpression(SqlNullLiteralExpression),
    SqlNumberLiteralExpression(SqlNumberLiteralExpression),
    SqlStringLiteralExpression(SqlStringLiteralExpression),
}
impl AnySqlLiteralExpression {
    pub fn as_sql_boolean_literal_expression(&self) -> Option<&SqlBooleanLiteralExpression> {
        match &self {
            Self::SqlBooleanLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_null_literal_expression(&self) -> Option<&SqlNullLiteralExpression> {
        match &self {
            Self::SqlNullLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_number_literal_expression(&self) -> Option<&SqlNumberLiteralExpression> {
        match &self {
            Self::SqlNumberLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_string_literal_expression(&self) -> Option<&SqlStringLiteralExpression> {
        match &self {
            Self::SqlStringLiteralExpression(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlName {
    SqlName(SqlName),
    SqlTildeName(SqlTildeName),
}
impl AnySqlName {
    pub fn as_sql_name(&self) -> Option<&SqlName> {
        match &self {
            Self::SqlName(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_tilde_name(&self) -> Option<&SqlTildeName> {
        match &self {
            Self::SqlTildeName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlReturnsType {
    SqlReturnsSetofClause(SqlReturnsSetofClause),
    SqlReturnsTableClause(SqlReturnsTableClause),
    SqlReturnsTriggerClause(SqlReturnsTriggerClause),
    SqlTypeName(SqlTypeName),
}
impl AnySqlReturnsType {
    pub fn as_sql_returns_setof_clause(&self) -> Option<&SqlReturnsSetofClause> {
        match &self {
            Self::SqlReturnsSetofClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_returns_table_clause(&self) -> Option<&SqlReturnsTableClause> {
        match &self {
            Self::SqlReturnsTableClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_returns_trigger_clause(&self) -> Option<&SqlReturnsTriggerClause> {
        match &self {
            Self::SqlReturnsTriggerClause(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_type_name(&self) -> Option<&SqlTypeName> {
        match &self {
            Self::SqlTypeName(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlSelectItem {
    SqlSelectExpression(SqlSelectExpression),
    SqlStar(SqlStar),
    SqlTableStar(SqlTableStar),
}
impl AnySqlSelectItem {
    pub fn as_sql_select_expression(&self) -> Option<&SqlSelectExpression> {
        match &self {
            Self::SqlSelectExpression(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_star(&self) -> Option<&SqlStar> {
        match &self {
            Self::SqlStar(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_table_star(&self) -> Option<&SqlTableStar> {
        match &self {
            Self::SqlTableStar(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlSelectQuantifier {
    SqlSelectAllQuantifier(SqlSelectAllQuantifier),
    SqlSelectDistinctQuantifier(SqlSelectDistinctQuantifier),
}
impl AnySqlSelectQuantifier {
    pub fn as_sql_select_all_quantifier(&self) -> Option<&SqlSelectAllQuantifier> {
        match &self {
            Self::SqlSelectAllQuantifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_select_distinct_quantifier(&self) -> Option<&SqlSelectDistinctQuantifier> {
        match &self {
            Self::SqlSelectDistinctQuantifier(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlStatement {
    SqlBogusStatement(SqlBogusStatement),
    SqlCreateFunctionStatement(SqlCreateFunctionStatement),
    SqlCreatePolicyStatement(SqlCreatePolicyStatement),
    SqlCreateTableStatement(SqlCreateTableStatement),
    SqlCreateTriggerStatement(SqlCreateTriggerStatement),
    SqlCreateViewStatement(SqlCreateViewStatement),
    SqlDeleteStatement(SqlDeleteStatement),
    SqlDropFunctionStatement(SqlDropFunctionStatement),
    SqlDropPolicyStatement(SqlDropPolicyStatement),
    SqlDropTableStatement(SqlDropTableStatement),
    SqlDropTriggerStatement(SqlDropTriggerStatement),
    SqlDropViewStatement(SqlDropViewStatement),
    SqlEmptyStatement(SqlEmptyStatement),
    SqlGrantStatement(SqlGrantStatement),
    SqlInsertStatement(SqlInsertStatement),
    SqlSelectStatement(SqlSelectStatement),
    SqlUpdateStatement(SqlUpdateStatement),
    SqlValuesClause(SqlValuesClause),
}
impl AnySqlStatement {
    pub fn as_sql_bogus_statement(&self) -> Option<&SqlBogusStatement> {
        match &self {
            Self::SqlBogusStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_create_function_statement(&self) -> Option<&SqlCreateFunctionStatement> {
        match &self {
            Self::SqlCreateFunctionStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_create_policy_statement(&self) -> Option<&SqlCreatePolicyStatement> {
        match &self {
            Self::SqlCreatePolicyStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_create_table_statement(&self) -> Option<&SqlCreateTableStatement> {
        match &self {
            Self::SqlCreateTableStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_create_trigger_statement(&self) -> Option<&SqlCreateTriggerStatement> {
        match &self {
            Self::SqlCreateTriggerStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_create_view_statement(&self) -> Option<&SqlCreateViewStatement> {
        match &self {
            Self::SqlCreateViewStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_delete_statement(&self) -> Option<&SqlDeleteStatement> {
        match &self {
            Self::SqlDeleteStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_drop_function_statement(&self) -> Option<&SqlDropFunctionStatement> {
        match &self {
            Self::SqlDropFunctionStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_drop_policy_statement(&self) -> Option<&SqlDropPolicyStatement> {
        match &self {
            Self::SqlDropPolicyStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_drop_table_statement(&self) -> Option<&SqlDropTableStatement> {
        match &self {
            Self::SqlDropTableStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_drop_trigger_statement(&self) -> Option<&SqlDropTriggerStatement> {
        match &self {
            Self::SqlDropTriggerStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_drop_view_statement(&self) -> Option<&SqlDropViewStatement> {
        match &self {
            Self::SqlDropViewStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_empty_statement(&self) -> Option<&SqlEmptyStatement> {
        match &self {
            Self::SqlEmptyStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_grant_statement(&self) -> Option<&SqlGrantStatement> {
        match &self {
            Self::SqlGrantStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_insert_statement(&self) -> Option<&SqlInsertStatement> {
        match &self {
            Self::SqlInsertStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_select_statement(&self) -> Option<&SqlSelectStatement> {
        match &self {
            Self::SqlSelectStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_update_statement(&self) -> Option<&SqlUpdateStatement> {
        match &self {
            Self::SqlUpdateStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_values_clause(&self) -> Option<&SqlValuesClause> {
        match &self {
            Self::SqlValuesClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlSubqueryBody {
    SqlSelectStatement(SqlSelectStatement),
    SqlValuesClause(SqlValuesClause),
}
impl AnySqlSubqueryBody {
    pub fn as_sql_select_statement(&self) -> Option<&SqlSelectStatement> {
        match &self {
            Self::SqlSelectStatement(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_values_clause(&self) -> Option<&SqlValuesClause> {
        match &self {
            Self::SqlValuesClause(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlTypeArraySuffix {
    SqlTildeArraySuffix(SqlTildeArraySuffix),
    SqlTypeArraySuffix(SqlTypeArraySuffix),
}
impl AnySqlTypeArraySuffix {
    pub fn as_sql_tilde_array_suffix(&self) -> Option<&SqlTildeArraySuffix> {
        match &self {
            Self::SqlTildeArraySuffix(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_type_array_suffix(&self) -> Option<&SqlTypeArraySuffix> {
        match &self {
            Self::SqlTypeArraySuffix(item) => Some(item),
            _ => None,
        }
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub enum AnySqlTypeModifier {
    SqlPrecisionModifier(SqlPrecisionModifier),
    SqlTimeZoneModifier(SqlTimeZoneModifier),
    SqlVaryingModifier(SqlVaryingModifier),
}
impl AnySqlTypeModifier {
    pub fn as_sql_precision_modifier(&self) -> Option<&SqlPrecisionModifier> {
        match &self {
            Self::SqlPrecisionModifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_time_zone_modifier(&self) -> Option<&SqlTimeZoneModifier> {
        match &self {
            Self::SqlTimeZoneModifier(item) => Some(item),
            _ => None,
        }
    }
    pub fn as_sql_varying_modifier(&self) -> Option<&SqlVaryingModifier> {
        match &self {
            Self::SqlVaryingModifier(item) => Some(item),
            _ => None,
        }
    }
}
impl AstNode for SqlAlias {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ALIAS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ALIAS
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
impl std::fmt::Debug for SqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlAlias")
                .field("as_token", &support::DebugOptionalElement(self.as_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .field("columns", &support::DebugOptionalElement(self.columns()))
                .finish()
        } else {
            f.debug_struct("SqlAlias").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlAlias> for SyntaxNode {
    fn from(n: SqlAlias) -> Self {
        n.syntax
    }
}
impl From<SqlAlias> for SyntaxElement {
    fn from(n: SqlAlias) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlAliasColumnDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ALIAS_COLUMN_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ALIAS_COLUMN_DEFINITION
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
impl std::fmt::Debug for SqlAliasColumnDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlAliasColumnDefinition")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("ty", &support::DebugOptionalElement(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlAliasColumnDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlAliasColumnDefinition> for SyntaxNode {
    fn from(n: SqlAliasColumnDefinition) -> Self {
        n.syntax
    }
}
impl From<SqlAliasColumnDefinition> for SyntaxElement {
    fn from(n: SqlAliasColumnDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlAliasColumnList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ALIAS_COLUMN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ALIAS_COLUMN_LIST
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
impl std::fmt::Debug for SqlAliasColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlAliasColumnList")
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
            f.debug_struct("SqlAliasColumnList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlAliasColumnList> for SyntaxNode {
    fn from(n: SqlAliasColumnList) -> Self {
        n.syntax
    }
}
impl From<SqlAliasColumnList> for SyntaxElement {
    fn from(n: SqlAliasColumnList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlAnyAllExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ANY_ALL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ANY_ALL_EXPRESSION
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
impl std::fmt::Debug for SqlAnyAllExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlAnyAllExpression")
                .field("quantifier", &support::DebugSyntaxResult(self.quantifier()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("SqlAnyAllExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlAnyAllExpression> for SyntaxNode {
    fn from(n: SqlAnyAllExpression) -> Self {
        n.syntax
    }
}
impl From<SqlAnyAllExpression> for SyntaxElement {
    fn from(n: SqlAnyAllExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlArrayExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ARRAY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ARRAY_EXPRESSION
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
impl std::fmt::Debug for SqlArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlArrayExpression")
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
            f.debug_struct("SqlArrayExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlArrayExpression> for SyntaxNode {
    fn from(n: SqlArrayExpression) -> Self {
        n.syntax
    }
}
impl From<SqlArrayExpression> for SyntaxElement {
    fn from(n: SqlArrayExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlArraySubscriptExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ARRAY_SUBSCRIPT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ARRAY_SUBSCRIPT_EXPRESSION
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
impl std::fmt::Debug for SqlArraySubscriptExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlArraySubscriptExpression")
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
            f.debug_struct("SqlArraySubscriptExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlArraySubscriptExpression> for SyntaxNode {
    fn from(n: SqlArraySubscriptExpression) -> Self {
        n.syntax
    }
}
impl From<SqlArraySubscriptExpression> for SyntaxElement {
    fn from(n: SqlArraySubscriptExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlBetweenExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BETWEEN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BETWEEN_EXPRESSION
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
impl std::fmt::Debug for SqlBetweenExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlBetweenExpression")
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
            f.debug_struct("SqlBetweenExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlBetweenExpression> for SyntaxNode {
    fn from(n: SqlBetweenExpression) -> Self {
        n.syntax
    }
}
impl From<SqlBetweenExpression> for SyntaxElement {
    fn from(n: SqlBetweenExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlBinaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BINARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BINARY_EXPRESSION
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
impl std::fmt::Debug for SqlBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlBinaryExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("SqlBinaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlBinaryExpression> for SyntaxNode {
    fn from(n: SqlBinaryExpression) -> Self {
        n.syntax
    }
}
impl From<SqlBinaryExpression> for SyntaxElement {
    fn from(n: SqlBinaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlBooleanLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOOLEAN_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOOLEAN_LITERAL_EXPRESSION
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
impl std::fmt::Debug for SqlBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlBooleanLiteralExpression")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlBooleanLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlBooleanLiteralExpression> for SyntaxNode {
    fn from(n: SqlBooleanLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<SqlBooleanLiteralExpression> for SyntaxElement {
    fn from(n: SqlBooleanLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCallExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CALL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CALL_EXPRESSION
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
impl std::fmt::Debug for SqlCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCallExpression")
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
                .field(
                    "filter_clause",
                    &support::DebugOptionalElement(self.filter_clause()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCallExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCallExpression> for SyntaxNode {
    fn from(n: SqlCallExpression) -> Self {
        n.syntax
    }
}
impl From<SqlCallExpression> for SyntaxElement {
    fn from(n: SqlCallExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCaseElseClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CASE_ELSE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CASE_ELSE_CLAUSE
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
impl std::fmt::Debug for SqlCaseElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCaseElseClause")
                .field("else_token", &support::DebugSyntaxResult(self.else_token()))
                .field("result", &support::DebugSyntaxResult(self.result()))
                .finish()
        } else {
            f.debug_struct("SqlCaseElseClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCaseElseClause> for SyntaxNode {
    fn from(n: SqlCaseElseClause) -> Self {
        n.syntax
    }
}
impl From<SqlCaseElseClause> for SyntaxElement {
    fn from(n: SqlCaseElseClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCaseExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CASE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CASE_EXPRESSION
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
impl std::fmt::Debug for SqlCaseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCaseExpression")
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
            f.debug_struct("SqlCaseExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCaseExpression> for SyntaxNode {
    fn from(n: SqlCaseExpression) -> Self {
        n.syntax
    }
}
impl From<SqlCaseExpression> for SyntaxElement {
    fn from(n: SqlCaseExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCaseWhenClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CASE_WHEN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CASE_WHEN_CLAUSE
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
impl std::fmt::Debug for SqlCaseWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCaseWhenClause")
                .field("when_token", &support::DebugSyntaxResult(self.when_token()))
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field("then_token", &support::DebugSyntaxResult(self.then_token()))
                .field("result", &support::DebugSyntaxResult(self.result()))
                .finish()
        } else {
            f.debug_struct("SqlCaseWhenClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCaseWhenClause> for SyntaxNode {
    fn from(n: SqlCaseWhenClause) -> Self {
        n.syntax
    }
}
impl From<SqlCaseWhenClause> for SyntaxElement {
    fn from(n: SqlCaseWhenClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCastExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CAST_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CAST_EXPRESSION
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
impl std::fmt::Debug for SqlCastExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCastExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "double_colon_token",
                    &support::DebugSyntaxResult(self.double_colon_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlCastExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCastExpression> for SyntaxNode {
    fn from(n: SqlCastExpression) -> Self {
        n.syntax
    }
}
impl From<SqlCastExpression> for SyntaxElement {
    fn from(n: SqlCastExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCastFunctionExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CAST_FUNCTION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CAST_FUNCTION_EXPRESSION
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
impl std::fmt::Debug for SqlCastFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCastFunctionExpression")
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
            f.debug_struct("SqlCastFunctionExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCastFunctionExpression> for SyntaxNode {
    fn from(n: SqlCastFunctionExpression) -> Self {
        n.syntax
    }
}
impl From<SqlCastFunctionExpression> for SyntaxElement {
    fn from(n: SqlCastFunctionExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlColReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_COL_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_COL_REFERENCE
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
impl std::fmt::Debug for SqlColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlColReference")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlColReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlColReference> for SyntaxNode {
    fn from(n: SqlColReference) -> Self {
        n.syntax
    }
}
impl From<SqlColReference> for SyntaxElement {
    fn from(n: SqlColReference) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlColumnDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_COLUMN_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_COLUMN_DEFINITION
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
impl std::fmt::Debug for SqlColumnDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlColumnDefinition")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlColumnDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlColumnDefinition> for SyntaxNode {
    fn from(n: SqlColumnDefinition) -> Self {
        n.syntax
    }
}
impl From<SqlColumnDefinition> for SyntaxElement {
    fn from(n: SqlColumnDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlColumnList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_COLUMN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_COLUMN_LIST
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
impl std::fmt::Debug for SqlColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlColumnList")
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
            f.debug_struct("SqlColumnList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlColumnList> for SyntaxNode {
    fn from(n: SqlColumnList) -> Self {
        n.syntax
    }
}
impl From<SqlColumnList> for SyntaxElement {
    fn from(n: SqlColumnList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCreateFunctionStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CREATE_FUNCTION_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CREATE_FUNCTION_STATEMENT
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
impl std::fmt::Debug for SqlCreateFunctionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCreateFunctionStatement")
                .field(
                    "create_token",
                    &support::DebugSyntaxResult(self.create_token()),
                )
                .field("or_token", &support::DebugOptionalElement(self.or_token()))
                .field(
                    "replace_token",
                    &support::DebugOptionalElement(self.replace_token()),
                )
                .field("kind", &support::DebugSyntaxResult(self.kind()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("parameters", &self.parameters())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "returns_clause",
                    &support::DebugOptionalElement(self.returns_clause()),
                )
                .field("leading_options", &self.leading_options())
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("body", &support::DebugSyntaxResult(self.body()))
                .field("trailing_options", &self.trailing_options())
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCreateFunctionStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCreateFunctionStatement> for SyntaxNode {
    fn from(n: SqlCreateFunctionStatement) -> Self {
        n.syntax
    }
}
impl From<SqlCreateFunctionStatement> for SyntaxElement {
    fn from(n: SqlCreateFunctionStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCreatePolicyStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CREATE_POLICY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CREATE_POLICY_STATEMENT
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
impl std::fmt::Debug for SqlCreatePolicyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCreatePolicyStatement")
                .field(
                    "create_token",
                    &support::DebugSyntaxResult(self.create_token()),
                )
                .field(
                    "policy_token",
                    &support::DebugSyntaxResult(self.policy_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field(
                    "for_clause",
                    &support::DebugOptionalElement(self.for_clause()),
                )
                .field(
                    "using_clause",
                    &support::DebugOptionalElement(self.using_clause()),
                )
                .field(
                    "with_check_clause",
                    &support::DebugOptionalElement(self.with_check_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCreatePolicyStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCreatePolicyStatement> for SyntaxNode {
    fn from(n: SqlCreatePolicyStatement) -> Self {
        n.syntax
    }
}
impl From<SqlCreatePolicyStatement> for SyntaxElement {
    fn from(n: SqlCreatePolicyStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCreateTableStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CREATE_TABLE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CREATE_TABLE_STATEMENT
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
impl std::fmt::Debug for SqlCreateTableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCreateTableStatement")
                .field(
                    "create_token",
                    &support::DebugSyntaxResult(self.create_token()),
                )
                .field(
                    "table_token",
                    &support::DebugSyntaxResult(self.table_token()),
                )
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("columns", &self.columns())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCreateTableStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCreateTableStatement> for SyntaxNode {
    fn from(n: SqlCreateTableStatement) -> Self {
        n.syntax
    }
}
impl From<SqlCreateTableStatement> for SyntaxElement {
    fn from(n: SqlCreateTableStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCreateTriggerStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CREATE_TRIGGER_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CREATE_TRIGGER_STATEMENT
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
impl std::fmt::Debug for SqlCreateTriggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCreateTriggerStatement")
                .field(
                    "create_token",
                    &support::DebugSyntaxResult(self.create_token()),
                )
                .field(
                    "trigger_token",
                    &support::DebugSyntaxResult(self.trigger_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("timing", &support::DebugSyntaxResult(self.timing()))
                .field("events", &self.events())
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field(
                    "referencing_clause",
                    &support::DebugOptionalElement(self.referencing_clause()),
                )
                .field(
                    "for_each_clause",
                    &support::DebugOptionalElement(self.for_each_clause()),
                )
                .field(
                    "when_clause",
                    &support::DebugOptionalElement(self.when_clause()),
                )
                .field(
                    "execute_token",
                    &support::DebugSyntaxResult(self.execute_token()),
                )
                .field(
                    "function_kind",
                    &support::DebugSyntaxResult(self.function_kind()),
                )
                .field("function", &support::DebugSyntaxResult(self.function()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCreateTriggerStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCreateTriggerStatement> for SyntaxNode {
    fn from(n: SqlCreateTriggerStatement) -> Self {
        n.syntax
    }
}
impl From<SqlCreateTriggerStatement> for SyntaxElement {
    fn from(n: SqlCreateTriggerStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCreateViewStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CREATE_VIEW_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CREATE_VIEW_STATEMENT
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
impl std::fmt::Debug for SqlCreateViewStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCreateViewStatement")
                .field(
                    "create_token",
                    &support::DebugSyntaxResult(self.create_token()),
                )
                .field("or_token", &support::DebugOptionalElement(self.or_token()))
                .field(
                    "replace_token",
                    &support::DebugOptionalElement(self.replace_token()),
                )
                .field("view_token", &support::DebugSyntaxResult(self.view_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("options", &support::DebugOptionalElement(self.options()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("query", &support::DebugSyntaxResult(self.query()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCreateViewStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCreateViewStatement> for SyntaxNode {
    fn from(n: SqlCreateViewStatement) -> Self {
        n.syntax
    }
}
impl From<SqlCreateViewStatement> for SyntaxElement {
    fn from(n: SqlCreateViewStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCteDefinition {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CTE_DEFINITION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CTE_DEFINITION
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
impl std::fmt::Debug for SqlCteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCteDefinition")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("columns", &support::DebugOptionalElement(self.columns()))
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field(
                    "materialized",
                    &support::DebugOptionalElement(self.materialized()),
                )
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
            f.debug_struct("SqlCteDefinition").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCteDefinition> for SyntaxNode {
    fn from(n: SqlCteDefinition) -> Self {
        n.syntax
    }
}
impl From<SqlCteDefinition> for SyntaxElement {
    fn from(n: SqlCteDefinition) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlCteMaterializedHint {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CTE_MATERIALIZED_HINT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CTE_MATERIALIZED_HINT
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
impl std::fmt::Debug for SqlCteMaterializedHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlCteMaterializedHint")
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field(
                    "materialized_token",
                    &support::DebugSyntaxResult(self.materialized_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlCteMaterializedHint").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlCteMaterializedHint> for SyntaxNode {
    fn from(n: SqlCteMaterializedHint) -> Self {
        n.syntax
    }
}
impl From<SqlCteMaterializedHint> for SyntaxElement {
    fn from(n: SqlCteMaterializedHint) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDataBaseName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DATA_BASE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DATA_BASE_NAME
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
impl std::fmt::Debug for SqlDataBaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDataBaseName")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .finish()
        } else {
            f.debug_struct("SqlDataBaseName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDataBaseName> for SyntaxNode {
    fn from(n: SqlDataBaseName) -> Self {
        n.syntax
    }
}
impl From<SqlDataBaseName> for SyntaxElement {
    fn from(n: SqlDataBaseName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDeleteStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DELETE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DELETE_STATEMENT
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
impl std::fmt::Debug for SqlDeleteStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDeleteStatement")
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
            f.debug_struct("SqlDeleteStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDeleteStatement> for SyntaxNode {
    fn from(n: SqlDeleteStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDeleteStatement> for SyntaxElement {
    fn from(n: SqlDeleteStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDeleteUsingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DELETE_USING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DELETE_USING_CLAUSE
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
impl std::fmt::Debug for SqlDeleteUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDeleteUsingClause")
                .field(
                    "using_token",
                    &support::DebugSyntaxResult(self.using_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlDeleteUsingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDeleteUsingClause> for SyntaxNode {
    fn from(n: SqlDeleteUsingClause) -> Self {
        n.syntax
    }
}
impl From<SqlDeleteUsingClause> for SyntaxElement {
    fn from(n: SqlDeleteUsingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDistinctOnClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DISTINCT_ON_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DISTINCT_ON_CLAUSE
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
impl std::fmt::Debug for SqlDistinctOnClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDistinctOnClause")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
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
            f.debug_struct("SqlDistinctOnClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDistinctOnClause> for SyntaxNode {
    fn from(n: SqlDistinctOnClause) -> Self {
        n.syntax
    }
}
impl From<SqlDistinctOnClause> for SyntaxElement {
    fn from(n: SqlDistinctOnClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDoNothingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DO_NOTHING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DO_NOTHING_CLAUSE
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
impl std::fmt::Debug for SqlDoNothingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDoNothingClause")
                .field("do_token", &support::DebugSyntaxResult(self.do_token()))
                .field(
                    "nothing_token",
                    &support::DebugSyntaxResult(self.nothing_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDoNothingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDoNothingClause> for SyntaxNode {
    fn from(n: SqlDoNothingClause) -> Self {
        n.syntax
    }
}
impl From<SqlDoNothingClause> for SyntaxElement {
    fn from(n: SqlDoNothingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDoUpdateClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DO_UPDATE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DO_UPDATE_CLAUSE
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
impl std::fmt::Debug for SqlDoUpdateClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDoUpdateClause")
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
            f.debug_struct("SqlDoUpdateClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDoUpdateClause> for SyntaxNode {
    fn from(n: SqlDoUpdateClause) -> Self {
        n.syntax
    }
}
impl From<SqlDoUpdateClause> for SyntaxElement {
    fn from(n: SqlDoUpdateClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropFunctionParameters {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_FUNCTION_PARAMETERS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_FUNCTION_PARAMETERS
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
impl std::fmt::Debug for SqlDropFunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropFunctionParameters")
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
            f.debug_struct("SqlDropFunctionParameters").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropFunctionParameters> for SyntaxNode {
    fn from(n: SqlDropFunctionParameters) -> Self {
        n.syntax
    }
}
impl From<SqlDropFunctionParameters> for SyntaxElement {
    fn from(n: SqlDropFunctionParameters) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropFunctionStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_FUNCTION_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_FUNCTION_STATEMENT
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
impl std::fmt::Debug for SqlDropFunctionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropFunctionStatement")
                .field("drop_token", &support::DebugSyntaxResult(self.drop_token()))
                .field("kind", &support::DebugSyntaxResult(self.kind()))
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field(
                    "parameters",
                    &support::DebugOptionalElement(self.parameters()),
                )
                .field(
                    "drop_behavior",
                    &support::DebugOptionalElement(self.drop_behavior()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDropFunctionStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropFunctionStatement> for SyntaxNode {
    fn from(n: SqlDropFunctionStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDropFunctionStatement> for SyntaxElement {
    fn from(n: SqlDropFunctionStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropPolicyStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_POLICY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_POLICY_STATEMENT
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
impl std::fmt::Debug for SqlDropPolicyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropPolicyStatement")
                .field("drop_token", &support::DebugSyntaxResult(self.drop_token()))
                .field(
                    "policy_token",
                    &support::DebugSyntaxResult(self.policy_token()),
                )
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDropPolicyStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropPolicyStatement> for SyntaxNode {
    fn from(n: SqlDropPolicyStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDropPolicyStatement> for SyntaxElement {
    fn from(n: SqlDropPolicyStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropTableStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_TABLE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_TABLE_STATEMENT
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
impl std::fmt::Debug for SqlDropTableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropTableStatement")
                .field("drop_token", &support::DebugSyntaxResult(self.drop_token()))
                .field(
                    "table_token",
                    &support::DebugSyntaxResult(self.table_token()),
                )
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("tables", &self.tables())
                .field(
                    "drop_behavior",
                    &support::DebugOptionalElement(self.drop_behavior()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDropTableStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropTableStatement> for SyntaxNode {
    fn from(n: SqlDropTableStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDropTableStatement> for SyntaxElement {
    fn from(n: SqlDropTableStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropTriggerStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_TRIGGER_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_TRIGGER_STATEMENT
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
impl std::fmt::Debug for SqlDropTriggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropTriggerStatement")
                .field("drop_token", &support::DebugSyntaxResult(self.drop_token()))
                .field(
                    "trigger_token",
                    &support::DebugSyntaxResult(self.trigger_token()),
                )
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field(
                    "drop_behavior",
                    &support::DebugOptionalElement(self.drop_behavior()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDropTriggerStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropTriggerStatement> for SyntaxNode {
    fn from(n: SqlDropTriggerStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDropTriggerStatement> for SyntaxElement {
    fn from(n: SqlDropTriggerStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlDropViewStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_DROP_VIEW_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_DROP_VIEW_STATEMENT
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
impl std::fmt::Debug for SqlDropViewStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlDropViewStatement")
                .field("drop_token", &support::DebugSyntaxResult(self.drop_token()))
                .field("view_token", &support::DebugSyntaxResult(self.view_token()))
                .field("if_token", &support::DebugOptionalElement(self.if_token()))
                .field(
                    "exists_token",
                    &support::DebugOptionalElement(self.exists_token()),
                )
                .field("views", &self.views())
                .field(
                    "drop_behavior",
                    &support::DebugOptionalElement(self.drop_behavior()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlDropViewStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlDropViewStatement> for SyntaxNode {
    fn from(n: SqlDropViewStatement) -> Self {
        n.syntax
    }
}
impl From<SqlDropViewStatement> for SyntaxElement {
    fn from(n: SqlDropViewStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlEmptyStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_EMPTY_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_EMPTY_STATEMENT
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
impl std::fmt::Debug for SqlEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlEmptyStatement")
                .field(
                    "semicolon_token",
                    &support::DebugSyntaxResult(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlEmptyStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlEmptyStatement> for SyntaxNode {
    fn from(n: SqlEmptyStatement) -> Self {
        n.syntax
    }
}
impl From<SqlEmptyStatement> for SyntaxElement {
    fn from(n: SqlEmptyStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlExistsExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_EXISTS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_EXISTS_EXPRESSION
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
impl std::fmt::Debug for SqlExistsExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlExistsExpression")
                .field(
                    "exists_token",
                    &support::DebugSyntaxResult(self.exists_token()),
                )
                .field("subquery", &support::DebugSyntaxResult(self.subquery()))
                .finish()
        } else {
            f.debug_struct("SqlExistsExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlExistsExpression> for SyntaxNode {
    fn from(n: SqlExistsExpression) -> Self {
        n.syntax
    }
}
impl From<SqlExistsExpression> for SyntaxElement {
    fn from(n: SqlExistsExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFetchClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FETCH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FETCH_CLAUSE
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
impl std::fmt::Debug for SqlFetchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFetchClause")
                .field(
                    "fetch_token",
                    &support::DebugSyntaxResult(self.fetch_token()),
                )
                .field("quantifier", &support::DebugSyntaxResult(self.quantifier()))
                .field("count", &support::DebugOptionalElement(self.count()))
                .field(
                    "row_or_rows",
                    &support::DebugSyntaxResult(self.row_or_rows()),
                )
                .field("tail", &support::DebugSyntaxResult(self.tail()))
                .finish()
        } else {
            f.debug_struct("SqlFetchClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFetchClause> for SyntaxNode {
    fn from(n: SqlFetchClause) -> Self {
        n.syntax
    }
}
impl From<SqlFetchClause> for SyntaxElement {
    fn from(n: SqlFetchClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFetchOnlyTail {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FETCH_ONLY_TAIL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FETCH_ONLY_TAIL
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
impl std::fmt::Debug for SqlFetchOnlyTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFetchOnlyTail")
                .field("only_token", &support::DebugSyntaxResult(self.only_token()))
                .finish()
        } else {
            f.debug_struct("SqlFetchOnlyTail").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFetchOnlyTail> for SyntaxNode {
    fn from(n: SqlFetchOnlyTail) -> Self {
        n.syntax
    }
}
impl From<SqlFetchOnlyTail> for SyntaxElement {
    fn from(n: SqlFetchOnlyTail) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFetchWithTiesTail {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FETCH_WITH_TIES_TAIL as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FETCH_WITH_TIES_TAIL
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
impl std::fmt::Debug for SqlFetchWithTiesTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFetchWithTiesTail")
                .field("with_token", &support::DebugSyntaxResult(self.with_token()))
                .field("ties_token", &support::DebugSyntaxResult(self.ties_token()))
                .finish()
        } else {
            f.debug_struct("SqlFetchWithTiesTail").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFetchWithTiesTail> for SyntaxNode {
    fn from(n: SqlFetchWithTiesTail) -> Self {
        n.syntax
    }
}
impl From<SqlFetchWithTiesTail> for SyntaxElement {
    fn from(n: SqlFetchWithTiesTail) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFilterClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FILTER_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FILTER_CLAUSE
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
impl std::fmt::Debug for SqlFilterClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFilterClause")
                .field(
                    "filter_token",
                    &support::DebugSyntaxResult(self.filter_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field(
                    "where_token",
                    &support::DebugSyntaxResult(self.where_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlFilterClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFilterClause> for SyntaxNode {
    fn from(n: SqlFilterClause) -> Self {
        n.syntax
    }
}
impl From<SqlFilterClause> for SyntaxElement {
    fn from(n: SqlFilterClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFromClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FROM_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FROM_CLAUSE
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
impl std::fmt::Debug for SqlFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFromClause")
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlFromClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFromClause> for SyntaxNode {
    fn from(n: SqlFromClause) -> Self {
        n.syntax
    }
}
impl From<SqlFromClause> for SyntaxElement {
    fn from(n: SqlFromClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFromItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FROM_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FROM_ITEM
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
impl std::fmt::Debug for SqlFromItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFromItem")
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field("joins", &self.joins())
                .finish()
        } else {
            f.debug_struct("SqlFromItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFromItem> for SyntaxNode {
    fn from(n: SqlFromItem) -> Self {
        n.syntax
    }
}
impl From<SqlFromItem> for SyntaxElement {
    fn from(n: SqlFromItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFunctionBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FUNCTION_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FUNCTION_BINDING
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
impl std::fmt::Debug for SqlFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFunctionBinding")
                .field(
                    "lateral_token",
                    &support::DebugOptionalElement(self.lateral_token()),
                )
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
            f.debug_struct("SqlFunctionBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFunctionBinding> for SyntaxNode {
    fn from(n: SqlFunctionBinding) -> Self {
        n.syntax
    }
}
impl From<SqlFunctionBinding> for SyntaxElement {
    fn from(n: SqlFunctionBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlFunctionParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FUNCTION_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FUNCTION_PARAMETER
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
impl std::fmt::Debug for SqlFunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlFunctionParameter")
                .field("mode", &support::DebugOptionalElement(self.mode()))
                .field("name", &support::DebugOptionalElement(self.name()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .field("default", &support::DebugOptionalElement(self.default()))
                .finish()
        } else {
            f.debug_struct("SqlFunctionParameter").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlFunctionParameter> for SyntaxNode {
    fn from(n: SqlFunctionParameter) -> Self {
        n.syntax
    }
}
impl From<SqlFunctionParameter> for SyntaxElement {
    fn from(n: SqlFunctionParameter) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlGrantStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_GRANT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_GRANT_STATEMENT
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
impl std::fmt::Debug for SqlGrantStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlGrantStatement")
                .field(
                    "grant_token",
                    &support::DebugSyntaxResult(self.grant_token()),
                )
                .field("all_token", &support::DebugSyntaxResult(self.all_token()))
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "table_token",
                    &support::DebugOptionalElement(self.table_token()),
                )
                .field("objects", &self.objects())
                .field("to_token", &support::DebugSyntaxResult(self.to_token()))
                .field("grantees", &self.grantees())
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlGrantStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlGrantStatement> for SyntaxNode {
    fn from(n: SqlGrantStatement) -> Self {
        n.syntax
    }
}
impl From<SqlGrantStatement> for SyntaxElement {
    fn from(n: SqlGrantStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlGroupByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_GROUP_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_GROUP_BY_CLAUSE
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
impl std::fmt::Debug for SqlGroupByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlGroupByClause")
                .field(
                    "group_by_token",
                    &support::DebugSyntaxResult(self.group_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlGroupByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlGroupByClause> for SyntaxNode {
    fn from(n: SqlGroupByClause) -> Self {
        n.syntax
    }
}
impl From<SqlGroupByClause> for SyntaxElement {
    fn from(n: SqlGroupByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlHavingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_HAVING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_HAVING_CLAUSE
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
impl std::fmt::Debug for SqlHavingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlHavingClause")
                .field(
                    "having_token",
                    &support::DebugSyntaxResult(self.having_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("SqlHavingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlHavingClause> for SyntaxNode {
    fn from(n: SqlHavingClause) -> Self {
        n.syntax
    }
}
impl From<SqlHavingClause> for SyntaxElement {
    fn from(n: SqlHavingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlInExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_IN_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_IN_EXPRESSION
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
impl std::fmt::Debug for SqlInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlInExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field("in_token", &support::DebugSyntaxResult(self.in_token()))
                .field("source", &support::DebugSyntaxResult(self.source()))
                .finish()
        } else {
            f.debug_struct("SqlInExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlInExpression> for SyntaxNode {
    fn from(n: SqlInExpression) -> Self {
        n.syntax
    }
}
impl From<SqlInExpression> for SyntaxElement {
    fn from(n: SqlInExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlInValueList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_IN_VALUE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_IN_VALUE_LIST
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
impl std::fmt::Debug for SqlInValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlInValueList")
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
            f.debug_struct("SqlInValueList").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlInValueList> for SyntaxNode {
    fn from(n: SqlInValueList) -> Self {
        n.syntax
    }
}
impl From<SqlInValueList> for SyntaxElement {
    fn from(n: SqlInValueList) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlInsertStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_INSERT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_INSERT_STATEMENT
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
impl std::fmt::Debug for SqlInsertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlInsertStatement")
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
            f.debug_struct("SqlInsertStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlInsertStatement> for SyntaxNode {
    fn from(n: SqlInsertStatement) -> Self {
        n.syntax
    }
}
impl From<SqlInsertStatement> for SyntaxElement {
    fn from(n: SqlInsertStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlIntervalExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_INTERVAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_INTERVAL_EXPRESSION
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
impl std::fmt::Debug for SqlIntervalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlIntervalExpression")
                .field(
                    "interval_token",
                    &support::DebugSyntaxResult(self.interval_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlIntervalExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlIntervalExpression> for SyntaxNode {
    fn from(n: SqlIntervalExpression) -> Self {
        n.syntax
    }
}
impl From<SqlIntervalExpression> for SyntaxElement {
    fn from(n: SqlIntervalExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlIsNullExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_IS_NULL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_IS_NULL_EXPRESSION
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
impl std::fmt::Debug for SqlIsNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlIsNullExpression")
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field("is_token", &support::DebugSyntaxResult(self.is_token()))
                .field(
                    "not_token",
                    &support::DebugOptionalElement(self.not_token()),
                )
                .field("null_token", &support::DebugSyntaxResult(self.null_token()))
                .finish()
        } else {
            f.debug_struct("SqlIsNullExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlIsNullExpression> for SyntaxNode {
    fn from(n: SqlIsNullExpression) -> Self {
        n.syntax
    }
}
impl From<SqlIsNullExpression> for SyntaxElement {
    fn from(n: SqlIsNullExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlJoinClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_JOIN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_JOIN_CLAUSE
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
impl std::fmt::Debug for SqlJoinClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlJoinClause")
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
                .field(
                    "using_clause",
                    &support::DebugOptionalElement(self.using_clause()),
                )
                .finish()
        } else {
            f.debug_struct("SqlJoinClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlJoinClause> for SyntaxNode {
    fn from(n: SqlJoinClause) -> Self {
        n.syntax
    }
}
impl From<SqlJoinClause> for SyntaxElement {
    fn from(n: SqlJoinClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlJoinUsingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_JOIN_USING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_JOIN_USING_CLAUSE
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
impl std::fmt::Debug for SqlJoinUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlJoinUsingClause")
                .field(
                    "using_token",
                    &support::DebugSyntaxResult(self.using_token()),
                )
                .field("columns", &support::DebugSyntaxResult(self.columns()))
                .finish()
        } else {
            f.debug_struct("SqlJoinUsingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlJoinUsingClause> for SyntaxNode {
    fn from(n: SqlJoinUsingClause) -> Self {
        n.syntax
    }
}
impl From<SqlJoinUsingClause> for SyntaxElement {
    fn from(n: SqlJoinUsingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlLanguageOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_LANGUAGE_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_LANGUAGE_OPTION
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
impl std::fmt::Debug for SqlLanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlLanguageOption")
                .field(
                    "language_token",
                    &support::DebugSyntaxResult(self.language_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlLanguageOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlLanguageOption> for SyntaxNode {
    fn from(n: SqlLanguageOption) -> Self {
        n.syntax
    }
}
impl From<SqlLanguageOption> for SyntaxElement {
    fn from(n: SqlLanguageOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlLikeExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_LIKE_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_LIKE_EXPRESSION
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
impl std::fmt::Debug for SqlLikeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlLikeExpression")
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
            f.debug_struct("SqlLikeExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlLikeExpression> for SyntaxNode {
    fn from(n: SqlLikeExpression) -> Self {
        n.syntax
    }
}
impl From<SqlLikeExpression> for SyntaxElement {
    fn from(n: SqlLikeExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlLimitClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_LIMIT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_LIMIT_CLAUSE
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
impl std::fmt::Debug for SqlLimitClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlLimitClause")
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
            f.debug_struct("SqlLimitClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlLimitClause> for SyntaxNode {
    fn from(n: SqlLimitClause) -> Self {
        n.syntax
    }
}
impl From<SqlLimitClause> for SyntaxElement {
    fn from(n: SqlLimitClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlLogicalExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_LOGICAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_LOGICAL_EXPRESSION
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
impl std::fmt::Debug for SqlLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlLogicalExpression")
                .field("left", &support::DebugSyntaxResult(self.left()))
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("right", &support::DebugSyntaxResult(self.right()))
                .finish()
        } else {
            f.debug_struct("SqlLogicalExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlLogicalExpression> for SyntaxNode {
    fn from(n: SqlLogicalExpression) -> Self {
        n.syntax
    }
}
impl From<SqlLogicalExpression> for SyntaxElement {
    fn from(n: SqlLogicalExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_NAME
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
impl std::fmt::Debug for SqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlName> for SyntaxNode {
    fn from(n: SqlName) -> Self {
        n.syntax
    }
}
impl From<SqlName> for SyntaxElement {
    fn from(n: SqlName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlNullLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_NULL_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_NULL_LITERAL_EXPRESSION
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
impl std::fmt::Debug for SqlNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlNullLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlNullLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlNullLiteralExpression> for SyntaxNode {
    fn from(n: SqlNullLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<SqlNullLiteralExpression> for SyntaxElement {
    fn from(n: SqlNullLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlNumberLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_NUMBER_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_NUMBER_LITERAL_EXPRESSION
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
impl std::fmt::Debug for SqlNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlNumberLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlNumberLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlNumberLiteralExpression> for SyntaxNode {
    fn from(n: SqlNumberLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<SqlNumberLiteralExpression> for SyntaxElement {
    fn from(n: SqlNumberLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlOffsetClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_OFFSET_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_OFFSET_CLAUSE
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
impl std::fmt::Debug for SqlOffsetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlOffsetClause")
                .field(
                    "offset_token",
                    &support::DebugSyntaxResult(self.offset_token()),
                )
                .field("start", &support::DebugSyntaxResult(self.start()))
                .finish()
        } else {
            f.debug_struct("SqlOffsetClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlOffsetClause> for SyntaxNode {
    fn from(n: SqlOffsetClause) -> Self {
        n.syntax
    }
}
impl From<SqlOffsetClause> for SyntaxElement {
    fn from(n: SqlOffsetClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlOnConflictClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ON_CONFLICT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ON_CONFLICT_CLAUSE
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
impl std::fmt::Debug for SqlOnConflictClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlOnConflictClause")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "conflict_token",
                    &support::DebugSyntaxResult(self.conflict_token()),
                )
                .field("target", &support::DebugOptionalElement(self.target()))
                .field("action", &support::DebugSyntaxResult(self.action()))
                .finish()
        } else {
            f.debug_struct("SqlOnConflictClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlOnConflictClause> for SyntaxNode {
    fn from(n: SqlOnConflictClause) -> Self {
        n.syntax
    }
}
impl From<SqlOnConflictClause> for SyntaxElement {
    fn from(n: SqlOnConflictClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlOnConstraintClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ON_CONSTRAINT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ON_CONSTRAINT_CLAUSE
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
impl std::fmt::Debug for SqlOnConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlOnConstraintClause")
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "constraint_token",
                    &support::DebugSyntaxResult(self.constraint_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlOnConstraintClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlOnConstraintClause> for SyntaxNode {
    fn from(n: SqlOnConstraintClause) -> Self {
        n.syntax
    }
}
impl From<SqlOnConstraintClause> for SyntaxElement {
    fn from(n: SqlOnConstraintClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlOrderByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ORDER_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ORDER_BY_CLAUSE
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
impl std::fmt::Debug for SqlOrderByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlOrderByClause")
                .field(
                    "order_by_token",
                    &support::DebugSyntaxResult(self.order_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlOrderByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlOrderByClause> for SyntaxNode {
    fn from(n: SqlOrderByClause) -> Self {
        n.syntax
    }
}
impl From<SqlOrderByClause> for SyntaxElement {
    fn from(n: SqlOrderByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlOrderByExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ORDER_BY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ORDER_BY_EXPRESSION
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
impl std::fmt::Debug for SqlOrderByExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlOrderByExpression")
                .field("item", &support::DebugSyntaxResult(self.item()))
                .field("order", &support::DebugOptionalElement(self.order()))
                .finish()
        } else {
            f.debug_struct("SqlOrderByExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlOrderByExpression> for SyntaxNode {
    fn from(n: SqlOrderByExpression) -> Self {
        n.syntax
    }
}
impl From<SqlOrderByExpression> for SyntaxElement {
    fn from(n: SqlOrderByExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlParameterDefault {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_PARAMETER_DEFAULT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_PARAMETER_DEFAULT
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
impl std::fmt::Debug for SqlParameterDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlParameterDefault")
                .field("marker", &support::DebugSyntaxResult(self.marker()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlParameterDefault").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlParameterDefault> for SyntaxNode {
    fn from(n: SqlParameterDefault) -> Self {
        n.syntax
    }
}
impl From<SqlParameterDefault> for SyntaxElement {
    fn from(n: SqlParameterDefault) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlParameterExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_PARAMETER_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_PARAMETER_EXPRESSION
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
impl std::fmt::Debug for SqlParameterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlParameterExpression")
                .field(
                    "colon_token",
                    &support::DebugSyntaxResult(self.colon_token()),
                )
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlParameterExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlParameterExpression> for SyntaxNode {
    fn from(n: SqlParameterExpression) -> Self {
        n.syntax
    }
}
impl From<SqlParameterExpression> for SyntaxElement {
    fn from(n: SqlParameterExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlParenthesizedExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_PARENTHESIZED_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_PARENTHESIZED_EXPRESSION
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
impl std::fmt::Debug for SqlParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlParenthesizedExpression")
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
            f.debug_struct("SqlParenthesizedExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlParenthesizedExpression> for SyntaxNode {
    fn from(n: SqlParenthesizedExpression) -> Self {
        n.syntax
    }
}
impl From<SqlParenthesizedExpression> for SyntaxElement {
    fn from(n: SqlParenthesizedExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlParenthesizedJoinBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_PARENTHESIZED_JOIN_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_PARENTHESIZED_JOIN_BINDING
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
impl std::fmt::Debug for SqlParenthesizedJoinBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlParenthesizedJoinBinding")
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("source", &support::DebugSyntaxResult(self.source()))
                .field("joins", &self.joins())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("SqlParenthesizedJoinBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlParenthesizedJoinBinding> for SyntaxNode {
    fn from(n: SqlParenthesizedJoinBinding) -> Self {
        n.syntax
    }
}
impl From<SqlParenthesizedJoinBinding> for SyntaxElement {
    fn from(n: SqlParenthesizedJoinBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlPolicyForClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_POLICY_FOR_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_POLICY_FOR_CLAUSE
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
impl std::fmt::Debug for SqlPolicyForClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlPolicyForClause")
                .field("for_token", &support::DebugSyntaxResult(self.for_token()))
                .field("command", &support::DebugSyntaxResult(self.command()))
                .finish()
        } else {
            f.debug_struct("SqlPolicyForClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlPolicyForClause> for SyntaxNode {
    fn from(n: SqlPolicyForClause) -> Self {
        n.syntax
    }
}
impl From<SqlPolicyForClause> for SyntaxElement {
    fn from(n: SqlPolicyForClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlPolicyUsingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_POLICY_USING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_POLICY_USING_CLAUSE
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
impl std::fmt::Debug for SqlPolicyUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlPolicyUsingClause")
                .field(
                    "using_token",
                    &support::DebugSyntaxResult(self.using_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlPolicyUsingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlPolicyUsingClause> for SyntaxNode {
    fn from(n: SqlPolicyUsingClause) -> Self {
        n.syntax
    }
}
impl From<SqlPolicyUsingClause> for SyntaxElement {
    fn from(n: SqlPolicyUsingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlPolicyWithCheckClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_POLICY_WITH_CHECK_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_POLICY_WITH_CHECK_CLAUSE
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
impl std::fmt::Debug for SqlPolicyWithCheckClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlPolicyWithCheckClause")
                .field("with_token", &support::DebugSyntaxResult(self.with_token()))
                .field(
                    "check_token",
                    &support::DebugSyntaxResult(self.check_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlPolicyWithCheckClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlPolicyWithCheckClause> for SyntaxNode {
    fn from(n: SqlPolicyWithCheckClause) -> Self {
        n.syntax
    }
}
impl From<SqlPolicyWithCheckClause> for SyntaxElement {
    fn from(n: SqlPolicyWithCheckClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlPrecisionModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_PRECISION_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_PRECISION_MODIFIER
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
impl std::fmt::Debug for SqlPrecisionModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlPrecisionModifier")
                .field(
                    "precision_token",
                    &support::DebugSyntaxResult(self.precision_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlPrecisionModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlPrecisionModifier> for SyntaxNode {
    fn from(n: SqlPrecisionModifier) -> Self {
        n.syntax
    }
}
impl From<SqlPrecisionModifier> for SyntaxElement {
    fn from(n: SqlPrecisionModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturningClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNING_CLAUSE
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
impl std::fmt::Debug for SqlReturningClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturningClause")
                .field(
                    "returning_token",
                    &support::DebugSyntaxResult(self.returning_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlReturningClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturningClause> for SyntaxNode {
    fn from(n: SqlReturningClause) -> Self {
        n.syntax
    }
}
impl From<SqlReturningClause> for SyntaxElement {
    fn from(n: SqlReturningClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_CLAUSE
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
impl std::fmt::Debug for SqlReturnsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsClause")
                .field(
                    "returns_token",
                    &support::DebugSyntaxResult(self.returns_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlReturnsClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsClause> for SyntaxNode {
    fn from(n: SqlReturnsClause) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsClause> for SyntaxElement {
    fn from(n: SqlReturnsClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsNullOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_NULL_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_NULL_OPTION
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
impl std::fmt::Debug for SqlReturnsNullOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsNullOption")
                .field(
                    "returns_token",
                    &support::DebugSyntaxResult(self.returns_token()),
                )
                .field(
                    "first_null_token",
                    &support::DebugSyntaxResult(self.first_null_token()),
                )
                .field("on_token", &support::DebugSyntaxResult(self.on_token()))
                .field(
                    "second_null_token",
                    &support::DebugSyntaxResult(self.second_null_token()),
                )
                .field(
                    "input_token",
                    &support::DebugSyntaxResult(self.input_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlReturnsNullOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsNullOption> for SyntaxNode {
    fn from(n: SqlReturnsNullOption) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsNullOption> for SyntaxElement {
    fn from(n: SqlReturnsNullOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsSetofClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_SETOF_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_SETOF_CLAUSE
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
impl std::fmt::Debug for SqlReturnsSetofClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsSetofClause")
                .field(
                    "setof_token",
                    &support::DebugSyntaxResult(self.setof_token()),
                )
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlReturnsSetofClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsSetofClause> for SyntaxNode {
    fn from(n: SqlReturnsSetofClause) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsSetofClause> for SyntaxElement {
    fn from(n: SqlReturnsSetofClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsTableClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_TABLE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_TABLE_CLAUSE
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
impl std::fmt::Debug for SqlReturnsTableClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsTableClause")
                .field(
                    "table_token",
                    &support::DebugSyntaxResult(self.table_token()),
                )
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("columns", &self.columns())
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlReturnsTableClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsTableClause> for SyntaxNode {
    fn from(n: SqlReturnsTableClause) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsTableClause> for SyntaxElement {
    fn from(n: SqlReturnsTableClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsTableColumn {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_TABLE_COLUMN as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_TABLE_COLUMN
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
impl std::fmt::Debug for SqlReturnsTableColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsTableColumn")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("ty", &support::DebugSyntaxResult(self.ty()))
                .finish()
        } else {
            f.debug_struct("SqlReturnsTableColumn").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsTableColumn> for SyntaxNode {
    fn from(n: SqlReturnsTableColumn) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsTableColumn> for SyntaxElement {
    fn from(n: SqlReturnsTableColumn) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlReturnsTriggerClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_TRIGGER_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_TRIGGER_CLAUSE
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
impl std::fmt::Debug for SqlReturnsTriggerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlReturnsTriggerClause")
                .field(
                    "trigger_token",
                    &support::DebugSyntaxResult(self.trigger_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlReturnsTriggerClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlReturnsTriggerClause> for SyntaxNode {
    fn from(n: SqlReturnsTriggerClause) -> Self {
        n.syntax
    }
}
impl From<SqlReturnsTriggerClause> for SyntaxElement {
    fn from(n: SqlReturnsTriggerClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlRoot {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ROOT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ROOT
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
impl std::fmt::Debug for SqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlRoot")
                .field(
                    "bom_token",
                    &support::DebugOptionalElement(self.bom_token()),
                )
                .field("stmt", &self.stmt())
                .field("eof_token", &support::DebugSyntaxResult(self.eof_token()))
                .finish()
        } else {
            f.debug_struct("SqlRoot").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlRoot> for SyntaxNode {
    fn from(n: SqlRoot) -> Self {
        n.syntax
    }
}
impl From<SqlRoot> for SyntaxElement {
    fn from(n: SqlRoot) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSecurityOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SECURITY_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SECURITY_OPTION
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
impl std::fmt::Debug for SqlSecurityOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSecurityOption")
                .field(
                    "security_token",
                    &support::DebugSyntaxResult(self.security_token()),
                )
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlSecurityOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSecurityOption> for SyntaxNode {
    fn from(n: SqlSecurityOption) -> Self {
        n.syntax
    }
}
impl From<SqlSecurityOption> for SyntaxElement {
    fn from(n: SqlSecurityOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSelectAllQuantifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_ALL_QUANTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_ALL_QUANTIFIER
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
impl std::fmt::Debug for SqlSelectAllQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSelectAllQuantifier")
                .field("all_token", &support::DebugSyntaxResult(self.all_token()))
                .finish()
        } else {
            f.debug_struct("SqlSelectAllQuantifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSelectAllQuantifier> for SyntaxNode {
    fn from(n: SqlSelectAllQuantifier) -> Self {
        n.syntax
    }
}
impl From<SqlSelectAllQuantifier> for SyntaxElement {
    fn from(n: SqlSelectAllQuantifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSelectClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_CLAUSE
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
impl std::fmt::Debug for SqlSelectClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSelectClause")
                .field(
                    "select_token",
                    &support::DebugSyntaxResult(self.select_token()),
                )
                .field(
                    "quantifier",
                    &support::DebugOptionalElement(self.quantifier()),
                )
                .field("list", &self.list())
                .finish()
        } else {
            f.debug_struct("SqlSelectClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSelectClause> for SyntaxNode {
    fn from(n: SqlSelectClause) -> Self {
        n.syntax
    }
}
impl From<SqlSelectClause> for SyntaxElement {
    fn from(n: SqlSelectClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSelectDistinctQuantifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_DISTINCT_QUANTIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_DISTINCT_QUANTIFIER
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
impl std::fmt::Debug for SqlSelectDistinctQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSelectDistinctQuantifier")
                .field(
                    "distinct_token",
                    &support::DebugSyntaxResult(self.distinct_token()),
                )
                .field(
                    "on_clause",
                    &support::DebugOptionalElement(self.on_clause()),
                )
                .finish()
        } else {
            f.debug_struct("SqlSelectDistinctQuantifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSelectDistinctQuantifier> for SyntaxNode {
    fn from(n: SqlSelectDistinctQuantifier) -> Self {
        n.syntax
    }
}
impl From<SqlSelectDistinctQuantifier> for SyntaxElement {
    fn from(n: SqlSelectDistinctQuantifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSelectExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_EXPRESSION
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
impl std::fmt::Debug for SqlSelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSelectExpression")
                .field("expr", &support::DebugSyntaxResult(self.expr()))
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("SqlSelectExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSelectExpression> for SyntaxNode {
    fn from(n: SqlSelectExpression) -> Self {
        n.syntax
    }
}
impl From<SqlSelectExpression> for SyntaxElement {
    fn from(n: SqlSelectExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSelectStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_STATEMENT
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
impl std::fmt::Debug for SqlSelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSelectStatement")
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
                    "fetch_clause",
                    &support::DebugOptionalElement(self.fetch_clause()),
                )
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlSelectStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSelectStatement> for SyntaxNode {
    fn from(n: SqlSelectStatement) -> Self {
        n.syntax
    }
}
impl From<SqlSelectStatement> for SyntaxElement {
    fn from(n: SqlSelectStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSetClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SET_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SET_CLAUSE
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
impl std::fmt::Debug for SqlSetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSetClause")
                .field("set_token", &support::DebugSyntaxResult(self.set_token()))
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlSetClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSetClause> for SyntaxNode {
    fn from(n: SqlSetClause) -> Self {
        n.syntax
    }
}
impl From<SqlSetClause> for SyntaxElement {
    fn from(n: SqlSetClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSetItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SET_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SET_ITEM
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
impl std::fmt::Debug for SqlSetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSetItem")
                .field("column", &support::DebugSyntaxResult(self.column()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("expr", &support::DebugSyntaxResult(self.expr()))
                .finish()
        } else {
            f.debug_struct("SqlSetItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSetItem> for SyntaxNode {
    fn from(n: SqlSetItem) -> Self {
        n.syntax
    }
}
impl From<SqlSetItem> for SyntaxElement {
    fn from(n: SqlSetItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSetOperation {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SET_OPERATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SET_OPERATION
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
impl std::fmt::Debug for SqlSetOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSetOperation")
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
            f.debug_struct("SqlSetOperation").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSetOperation> for SyntaxNode {
    fn from(n: SqlSetOperation) -> Self {
        n.syntax
    }
}
impl From<SqlSetOperation> for SyntaxElement {
    fn from(n: SqlSetOperation) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlShemaName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SHEMA_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SHEMA_NAME
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
impl std::fmt::Debug for SqlShemaName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlShemaName")
                .field("base", &support::DebugOptionalElement(self.base()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .finish()
        } else {
            f.debug_struct("SqlShemaName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlShemaName> for SyntaxNode {
    fn from(n: SqlShemaName) -> Self {
        n.syntax
    }
}
impl From<SqlShemaName> for SyntaxElement {
    fn from(n: SqlShemaName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlStar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_STAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_STAR
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
impl std::fmt::Debug for SqlStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlStar")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlStar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlStar> for SyntaxNode {
    fn from(n: SqlStar) -> Self {
        n.syntax
    }
}
impl From<SqlStar> for SyntaxElement {
    fn from(n: SqlStar) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlStrictOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_STRICT_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_STRICT_OPTION
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
impl std::fmt::Debug for SqlStrictOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlStrictOption")
                .field(
                    "strict_token",
                    &support::DebugSyntaxResult(self.strict_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlStrictOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlStrictOption> for SyntaxNode {
    fn from(n: SqlStrictOption) -> Self {
        n.syntax
    }
}
impl From<SqlStrictOption> for SyntaxElement {
    fn from(n: SqlStrictOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlStringLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_STRING_LITERAL_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_STRING_LITERAL_EXPRESSION
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
impl std::fmt::Debug for SqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlStringLiteralExpression")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlStringLiteralExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlStringLiteralExpression> for SyntaxNode {
    fn from(n: SqlStringLiteralExpression) -> Self {
        n.syntax
    }
}
impl From<SqlStringLiteralExpression> for SyntaxElement {
    fn from(n: SqlStringLiteralExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSubqueryBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SUBQUERY_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SUBQUERY_BINDING
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
impl std::fmt::Debug for SqlSubqueryBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSubqueryBinding")
                .field(
                    "lateral_token",
                    &support::DebugOptionalElement(self.lateral_token()),
                )
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
            f.debug_struct("SqlSubqueryBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSubqueryBinding> for SyntaxNode {
    fn from(n: SqlSubqueryBinding) -> Self {
        n.syntax
    }
}
impl From<SqlSubqueryBinding> for SyntaxElement {
    fn from(n: SqlSubqueryBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSubqueryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SUBQUERY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SUBQUERY_EXPRESSION
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
impl std::fmt::Debug for SqlSubqueryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSubqueryExpression")
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
            f.debug_struct("SqlSubqueryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSubqueryExpression> for SyntaxNode {
    fn from(n: SqlSubqueryExpression) -> Self {
        n.syntax
    }
}
impl From<SqlSubqueryExpression> for SyntaxElement {
    fn from(n: SqlSubqueryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSubstringExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SUBSTRING_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SUBSTRING_EXPRESSION
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
impl std::fmt::Debug for SqlSubstringExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSubstringExpression")
                .field("name_token", &support::DebugSyntaxResult(self.name_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .field(
                    "from_clause",
                    &support::DebugOptionalElement(self.from_clause()),
                )
                .field(
                    "for_clause",
                    &support::DebugOptionalElement(self.for_clause()),
                )
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlSubstringExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSubstringExpression> for SyntaxNode {
    fn from(n: SqlSubstringExpression) -> Self {
        n.syntax
    }
}
impl From<SqlSubstringExpression> for SyntaxElement {
    fn from(n: SqlSubstringExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSubstringForClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SUBSTRING_FOR_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SUBSTRING_FOR_CLAUSE
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
impl std::fmt::Debug for SqlSubstringForClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSubstringForClause")
                .field("for_token", &support::DebugSyntaxResult(self.for_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlSubstringForClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSubstringForClause> for SyntaxNode {
    fn from(n: SqlSubstringForClause) -> Self {
        n.syntax
    }
}
impl From<SqlSubstringForClause> for SyntaxElement {
    fn from(n: SqlSubstringForClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlSubstringFromClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SUBSTRING_FROM_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SUBSTRING_FROM_CLAUSE
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
impl std::fmt::Debug for SqlSubstringFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlSubstringFromClause")
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlSubstringFromClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlSubstringFromClause> for SyntaxNode {
    fn from(n: SqlSubstringFromClause) -> Self {
        n.syntax
    }
}
impl From<SqlSubstringFromClause> for SyntaxElement {
    fn from(n: SqlSubstringFromClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTableBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TABLE_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TABLE_BINDING
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
impl std::fmt::Debug for SqlTableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTableBinding")
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("alias", &support::DebugOptionalElement(self.alias()))
                .finish()
        } else {
            f.debug_struct("SqlTableBinding").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTableBinding> for SyntaxNode {
    fn from(n: SqlTableBinding) -> Self {
        n.syntax
    }
}
impl From<SqlTableBinding> for SyntaxElement {
    fn from(n: SqlTableBinding) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTableColReference {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TABLE_COL_REFERENCE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TABLE_COL_REFERENCE
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
impl std::fmt::Debug for SqlTableColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTableColReference")
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlTableColReference").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTableColReference> for SyntaxNode {
    fn from(n: SqlTableColReference) -> Self {
        n.syntax
    }
}
impl From<SqlTableColReference> for SyntaxElement {
    fn from(n: SqlTableColReference) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTableName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TABLE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TABLE_NAME
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
impl std::fmt::Debug for SqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTableName")
                .field("schema", &support::DebugOptionalElement(self.schema()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlTableName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTableName> for SyntaxNode {
    fn from(n: SqlTableName) -> Self {
        n.syntax
    }
}
impl From<SqlTableName> for SyntaxElement {
    fn from(n: SqlTableName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTableStar {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TABLE_STAR as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TABLE_STAR
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
impl std::fmt::Debug for SqlTableStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTableStar")
                .field("table", &support::DebugSyntaxResult(self.table()))
                .field("dot_token", &support::DebugSyntaxResult(self.dot_token()))
                .field("star", &support::DebugSyntaxResult(self.star()))
                .finish()
        } else {
            f.debug_struct("SqlTableStar").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTableStar> for SyntaxNode {
    fn from(n: SqlTableStar) -> Self {
        n.syntax
    }
}
impl From<SqlTableStar> for SyntaxElement {
    fn from(n: SqlTableStar) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTildeArrayExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TILDE_ARRAY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TILDE_ARRAY_EXPRESSION
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
impl std::fmt::Debug for SqlTildeArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTildeArrayExpression")
                .field(
                    "array_token",
                    &support::DebugSyntaxResult(self.array_token()),
                )
                .field(
                    "open_tilde_token",
                    &support::DebugSyntaxResult(self.open_tilde_token()),
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
                .field(
                    "close_tilde_token",
                    &support::DebugSyntaxResult(self.close_tilde_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTildeArrayExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTildeArrayExpression> for SyntaxNode {
    fn from(n: SqlTildeArrayExpression) -> Self {
        n.syntax
    }
}
impl From<SqlTildeArrayExpression> for SyntaxElement {
    fn from(n: SqlTildeArrayExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTildeArraySuffix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TILDE_ARRAY_SUFFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TILDE_ARRAY_SUFFIX
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
impl std::fmt::Debug for SqlTildeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTildeArraySuffix")
                .field(
                    "open_tilde_token",
                    &support::DebugSyntaxResult(self.open_tilde_token()),
                )
                .field(
                    "l_brack_token",
                    &support::DebugSyntaxResult(self.l_brack_token()),
                )
                .field(
                    "r_brack_token",
                    &support::DebugSyntaxResult(self.r_brack_token()),
                )
                .field(
                    "close_tilde_token",
                    &support::DebugSyntaxResult(self.close_tilde_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTildeArraySuffix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTildeArraySuffix> for SyntaxNode {
    fn from(n: SqlTildeArraySuffix) -> Self {
        n.syntax
    }
}
impl From<SqlTildeArraySuffix> for SyntaxElement {
    fn from(n: SqlTildeArraySuffix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTildeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TILDE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TILDE_NAME
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
impl std::fmt::Debug for SqlTildeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTildeName")
                .field(
                    "value_token",
                    &support::DebugSyntaxResult(self.value_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTildeName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTildeName> for SyntaxNode {
    fn from(n: SqlTildeName) -> Self {
        n.syntax
    }
}
impl From<SqlTildeName> for SyntaxElement {
    fn from(n: SqlTildeName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTimeZoneModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TIME_ZONE_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TIME_ZONE_MODIFIER
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
impl std::fmt::Debug for SqlTimeZoneModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTimeZoneModifier")
                .field(
                    "with_or_without",
                    &support::DebugSyntaxResult(self.with_or_without()),
                )
                .field("time_token", &support::DebugSyntaxResult(self.time_token()))
                .field("zone_token", &support::DebugSyntaxResult(self.zone_token()))
                .finish()
        } else {
            f.debug_struct("SqlTimeZoneModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTimeZoneModifier> for SyntaxNode {
    fn from(n: SqlTimeZoneModifier) -> Self {
        n.syntax
    }
}
impl From<SqlTimeZoneModifier> for SyntaxElement {
    fn from(n: SqlTimeZoneModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerEvent {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_EVENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_EVENT
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
impl std::fmt::Debug for SqlTriggerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerEvent")
                .field("or_token", &support::DebugOptionalElement(self.or_token()))
                .field("kind", &support::DebugSyntaxResult(self.kind()))
                .field(
                    "of_clause",
                    &support::DebugOptionalElement(self.of_clause()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTriggerEvent").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerEvent> for SyntaxNode {
    fn from(n: SqlTriggerEvent) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerEvent> for SyntaxElement {
    fn from(n: SqlTriggerEvent) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerForEachClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_FOR_EACH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_FOR_EACH_CLAUSE
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
impl std::fmt::Debug for SqlTriggerForEachClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerForEachClause")
                .field("for_token", &support::DebugSyntaxResult(self.for_token()))
                .field("each_token", &support::DebugSyntaxResult(self.each_token()))
                .field(
                    "granularity",
                    &support::DebugSyntaxResult(self.granularity()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTriggerForEachClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerForEachClause> for SyntaxNode {
    fn from(n: SqlTriggerForEachClause) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerForEachClause> for SyntaxElement {
    fn from(n: SqlTriggerForEachClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerReferencingClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_REFERENCING_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_REFERENCING_CLAUSE
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
impl std::fmt::Debug for SqlTriggerReferencingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerReferencingClause")
                .field(
                    "referencing_token",
                    &support::DebugSyntaxResult(self.referencing_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlTriggerReferencingClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerReferencingClause> for SyntaxNode {
    fn from(n: SqlTriggerReferencingClause) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerReferencingClause> for SyntaxElement {
    fn from(n: SqlTriggerReferencingClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerReferencingItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_REFERENCING_ITEM as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_REFERENCING_ITEM
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
impl std::fmt::Debug for SqlTriggerReferencingItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerReferencingItem")
                .field(
                    "which_token",
                    &support::DebugSyntaxResult(self.which_token()),
                )
                .field(
                    "table_token",
                    &support::DebugSyntaxResult(self.table_token()),
                )
                .field("as_token", &support::DebugSyntaxResult(self.as_token()))
                .field("name", &support::DebugSyntaxResult(self.name()))
                .finish()
        } else {
            f.debug_struct("SqlTriggerReferencingItem").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerReferencingItem> for SyntaxNode {
    fn from(n: SqlTriggerReferencingItem) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerReferencingItem> for SyntaxElement {
    fn from(n: SqlTriggerReferencingItem) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerUpdateOfClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_UPDATE_OF_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_UPDATE_OF_CLAUSE
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
impl std::fmt::Debug for SqlTriggerUpdateOfClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerUpdateOfClause")
                .field("of_token", &support::DebugSyntaxResult(self.of_token()))
                .field("columns", &self.columns())
                .finish()
        } else {
            f.debug_struct("SqlTriggerUpdateOfClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerUpdateOfClause> for SyntaxNode {
    fn from(n: SqlTriggerUpdateOfClause) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerUpdateOfClause> for SyntaxElement {
    fn from(n: SqlTriggerUpdateOfClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTriggerWhenClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_WHEN_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_WHEN_CLAUSE
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
impl std::fmt::Debug for SqlTriggerWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTriggerWhenClause")
                .field("when_token", &support::DebugSyntaxResult(self.when_token()))
                .field(
                    "l_paren_token",
                    &support::DebugSyntaxResult(self.l_paren_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .field(
                    "r_paren_token",
                    &support::DebugSyntaxResult(self.r_paren_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTriggerWhenClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTriggerWhenClause> for SyntaxNode {
    fn from(n: SqlTriggerWhenClause) -> Self {
        n.syntax
    }
}
impl From<SqlTriggerWhenClause> for SyntaxElement {
    fn from(n: SqlTriggerWhenClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTypeArguments {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TYPE_ARGUMENTS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TYPE_ARGUMENTS
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
impl std::fmt::Debug for SqlTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTypeArguments")
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
            f.debug_struct("SqlTypeArguments").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTypeArguments> for SyntaxNode {
    fn from(n: SqlTypeArguments) -> Self {
        n.syntax
    }
}
impl From<SqlTypeArguments> for SyntaxElement {
    fn from(n: SqlTypeArguments) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTypeArraySuffix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TYPE_ARRAY_SUFFIX as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TYPE_ARRAY_SUFFIX
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
impl std::fmt::Debug for SqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTypeArraySuffix")
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
            f.debug_struct("SqlTypeArraySuffix").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTypeArraySuffix> for SyntaxNode {
    fn from(n: SqlTypeArraySuffix) -> Self {
        n.syntax
    }
}
impl From<SqlTypeArraySuffix> for SyntaxElement {
    fn from(n: SqlTypeArraySuffix) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlTypeName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TYPE_NAME as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TYPE_NAME
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
impl std::fmt::Debug for SqlTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlTypeName")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("args", &support::DebugOptionalElement(self.args()))
                .field("modifier", &support::DebugOptionalElement(self.modifier()))
                .field(
                    "array_suffix",
                    &support::DebugOptionalElement(self.array_suffix()),
                )
                .finish()
        } else {
            f.debug_struct("SqlTypeName").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlTypeName> for SyntaxNode {
    fn from(n: SqlTypeName) -> Self {
        n.syntax
    }
}
impl From<SqlTypeName> for SyntaxElement {
    fn from(n: SqlTypeName) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlUnaryExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_UNARY_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_UNARY_EXPRESSION
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
impl std::fmt::Debug for SqlUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlUnaryExpression")
                .field(
                    "operator_token",
                    &support::DebugSyntaxResult(self.operator_token()),
                )
                .field("expression", &support::DebugSyntaxResult(self.expression()))
                .finish()
        } else {
            f.debug_struct("SqlUnaryExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlUnaryExpression> for SyntaxNode {
    fn from(n: SqlUnaryExpression) -> Self {
        n.syntax
    }
}
impl From<SqlUnaryExpression> for SyntaxElement {
    fn from(n: SqlUnaryExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlUpdateFromClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_UPDATE_FROM_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_UPDATE_FROM_CLAUSE
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
impl std::fmt::Debug for SqlUpdateFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlUpdateFromClause")
                .field("from_token", &support::DebugSyntaxResult(self.from_token()))
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlUpdateFromClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlUpdateFromClause> for SyntaxNode {
    fn from(n: SqlUpdateFromClause) -> Self {
        n.syntax
    }
}
impl From<SqlUpdateFromClause> for SyntaxElement {
    fn from(n: SqlUpdateFromClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlUpdateStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_UPDATE_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_UPDATE_STATEMENT
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
impl std::fmt::Debug for SqlUpdateStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlUpdateStatement")
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
                    "from_clause",
                    &support::DebugOptionalElement(self.from_clause()),
                )
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
            f.debug_struct("SqlUpdateStatement").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlUpdateStatement> for SyntaxNode {
    fn from(n: SqlUpdateStatement) -> Self {
        n.syntax
    }
}
impl From<SqlUpdateStatement> for SyntaxElement {
    fn from(n: SqlUpdateStatement) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlValuesClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VALUES_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VALUES_CLAUSE
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
impl std::fmt::Debug for SqlValuesClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlValuesClause")
                .field(
                    "with_clause",
                    &support::DebugOptionalElement(self.with_clause()),
                )
                .field(
                    "values_token",
                    &support::DebugSyntaxResult(self.values_token()),
                )
                .field("rows", &self.rows())
                .field(
                    "semicolon_token",
                    &support::DebugOptionalElement(self.semicolon_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlValuesClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlValuesClause> for SyntaxNode {
    fn from(n: SqlValuesClause) -> Self {
        n.syntax
    }
}
impl From<SqlValuesClause> for SyntaxElement {
    fn from(n: SqlValuesClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlValuesRow {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VALUES_ROW as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VALUES_ROW
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
impl std::fmt::Debug for SqlValuesRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlValuesRow")
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
            f.debug_struct("SqlValuesRow").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlValuesRow> for SyntaxNode {
    fn from(n: SqlValuesRow) -> Self {
        n.syntax
    }
}
impl From<SqlValuesRow> for SyntaxElement {
    fn from(n: SqlValuesRow) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlVaryingModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VARYING_MODIFIER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VARYING_MODIFIER
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
impl std::fmt::Debug for SqlVaryingModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlVaryingModifier")
                .field(
                    "varying_token",
                    &support::DebugSyntaxResult(self.varying_token()),
                )
                .finish()
        } else {
            f.debug_struct("SqlVaryingModifier").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlVaryingModifier> for SyntaxNode {
    fn from(n: SqlVaryingModifier) -> Self {
        n.syntax
    }
}
impl From<SqlVaryingModifier> for SyntaxElement {
    fn from(n: SqlVaryingModifier) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlViewOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VIEW_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VIEW_OPTION
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
impl std::fmt::Debug for SqlViewOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlViewOption")
                .field("name", &support::DebugSyntaxResult(self.name()))
                .field("eq_token", &support::DebugSyntaxResult(self.eq_token()))
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlViewOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlViewOption> for SyntaxNode {
    fn from(n: SqlViewOption) -> Self {
        n.syntax
    }
}
impl From<SqlViewOption> for SyntaxElement {
    fn from(n: SqlViewOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlViewOptions {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VIEW_OPTIONS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VIEW_OPTIONS
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
impl std::fmt::Debug for SqlViewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlViewOptions")
                .field("with_token", &support::DebugSyntaxResult(self.with_token()))
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
            f.debug_struct("SqlViewOptions").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlViewOptions> for SyntaxNode {
    fn from(n: SqlViewOptions) -> Self {
        n.syntax
    }
}
impl From<SqlViewOptions> for SyntaxElement {
    fn from(n: SqlViewOptions) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlVolatilityOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VOLATILITY_OPTION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VOLATILITY_OPTION
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
impl std::fmt::Debug for SqlVolatilityOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlVolatilityOption")
                .field("value", &support::DebugSyntaxResult(self.value()))
                .finish()
        } else {
            f.debug_struct("SqlVolatilityOption").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlVolatilityOption> for SyntaxNode {
    fn from(n: SqlVolatilityOption) -> Self {
        n.syntax
    }
}
impl From<SqlVolatilityOption> for SyntaxElement {
    fn from(n: SqlVolatilityOption) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlWhereClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WHERE_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WHERE_CLAUSE
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
impl std::fmt::Debug for SqlWhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlWhereClause")
                .field(
                    "where_token",
                    &support::DebugSyntaxResult(self.where_token()),
                )
                .field("condition", &support::DebugSyntaxResult(self.condition()))
                .finish()
        } else {
            f.debug_struct("SqlWhereClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlWhereClause> for SyntaxNode {
    fn from(n: SqlWhereClause) -> Self {
        n.syntax
    }
}
impl From<SqlWhereClause> for SyntaxElement {
    fn from(n: SqlWhereClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlWindowFunctionExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WINDOW_FUNCTION_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WINDOW_FUNCTION_EXPRESSION
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
impl std::fmt::Debug for SqlWindowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlWindowFunctionExpression")
                .field("call", &support::DebugSyntaxResult(self.call()))
                .field("over_token", &support::DebugSyntaxResult(self.over_token()))
                .field("window", &support::DebugSyntaxResult(self.window()))
                .finish()
        } else {
            f.debug_struct("SqlWindowFunctionExpression").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlWindowFunctionExpression> for SyntaxNode {
    fn from(n: SqlWindowFunctionExpression) -> Self {
        n.syntax
    }
}
impl From<SqlWindowFunctionExpression> for SyntaxElement {
    fn from(n: SqlWindowFunctionExpression) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlWindowPartitionByClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WINDOW_PARTITION_BY_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WINDOW_PARTITION_BY_CLAUSE
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
impl std::fmt::Debug for SqlWindowPartitionByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlWindowPartitionByClause")
                .field(
                    "partition_by_token",
                    &support::DebugSyntaxResult(self.partition_by_token()),
                )
                .field("items", &self.items())
                .finish()
        } else {
            f.debug_struct("SqlWindowPartitionByClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlWindowPartitionByClause> for SyntaxNode {
    fn from(n: SqlWindowPartitionByClause) -> Self {
        n.syntax
    }
}
impl From<SqlWindowPartitionByClause> for SyntaxElement {
    fn from(n: SqlWindowPartitionByClause) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlWindowSpecification {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WINDOW_SPECIFICATION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WINDOW_SPECIFICATION
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
impl std::fmt::Debug for SqlWindowSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlWindowSpecification")
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
            f.debug_struct("SqlWindowSpecification").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlWindowSpecification> for SyntaxNode {
    fn from(n: SqlWindowSpecification) -> Self {
        n.syntax
    }
}
impl From<SqlWindowSpecification> for SyntaxElement {
    fn from(n: SqlWindowSpecification) -> Self {
        n.syntax.into()
    }
}
impl AstNode for SqlWithClause {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WITH_CLAUSE as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WITH_CLAUSE
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
impl std::fmt::Debug for SqlWithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        thread_local! { static DEPTH : std :: cell :: Cell < u8 > = const { std :: cell :: Cell :: new (0) } };
        let current_depth = DEPTH.get();
        let result = if current_depth < 16 {
            DEPTH.set(current_depth + 1);
            f.debug_struct("SqlWithClause")
                .field("with_token", &support::DebugSyntaxResult(self.with_token()))
                .field(
                    "recursive_token",
                    &support::DebugOptionalElement(self.recursive_token()),
                )
                .field("ctes", &self.ctes())
                .finish()
        } else {
            f.debug_struct("SqlWithClause").finish()
        };
        DEPTH.set(current_depth);
        result
    }
}
impl From<SqlWithClause> for SyntaxNode {
    fn from(n: SqlWithClause) -> Self {
        n.syntax
    }
}
impl From<SqlWithClause> for SyntaxElement {
    fn from(n: SqlWithClause) -> Self {
        n.syntax.into()
    }
}
impl From<SqlParenthesizedExpression> for AnySqlAnyAllSource {
    fn from(node: SqlParenthesizedExpression) -> Self {
        Self::SqlParenthesizedExpression(node)
    }
}
impl From<SqlSubqueryExpression> for AnySqlAnyAllSource {
    fn from(node: SqlSubqueryExpression) -> Self {
        Self::SqlSubqueryExpression(node)
    }
}
impl AstNode for AnySqlAnyAllSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlParenthesizedExpression::KIND_SET.union(SqlSubqueryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_PARENTHESIZED_EXPRESSION | SQL_SUBQUERY_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_PARENTHESIZED_EXPRESSION => {
                Self::SqlParenthesizedExpression(SqlParenthesizedExpression { syntax })
            }
            SQL_SUBQUERY_EXPRESSION => {
                Self::SqlSubqueryExpression(SqlSubqueryExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlParenthesizedExpression(it) => &it.syntax,
            Self::SqlSubqueryExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlParenthesizedExpression(it) => it.syntax,
            Self::SqlSubqueryExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlAnyAllSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlAnyAllSource> for SyntaxNode {
    fn from(n: AnySqlAnyAllSource) -> Self {
        match n {
            AnySqlAnyAllSource::SqlParenthesizedExpression(it) => it.into(),
            AnySqlAnyAllSource::SqlSubqueryExpression(it) => it.into(),
        }
    }
}
impl From<AnySqlAnyAllSource> for SyntaxElement {
    fn from(n: AnySqlAnyAllSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlDoNothingClause> for AnySqlConflictAction {
    fn from(node: SqlDoNothingClause) -> Self {
        Self::SqlDoNothingClause(node)
    }
}
impl From<SqlDoUpdateClause> for AnySqlConflictAction {
    fn from(node: SqlDoUpdateClause) -> Self {
        Self::SqlDoUpdateClause(node)
    }
}
impl AstNode for AnySqlConflictAction {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlDoNothingClause::KIND_SET.union(SqlDoUpdateClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_DO_NOTHING_CLAUSE | SQL_DO_UPDATE_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_DO_NOTHING_CLAUSE => Self::SqlDoNothingClause(SqlDoNothingClause { syntax }),
            SQL_DO_UPDATE_CLAUSE => Self::SqlDoUpdateClause(SqlDoUpdateClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlDoNothingClause(it) => &it.syntax,
            Self::SqlDoUpdateClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlDoNothingClause(it) => it.syntax,
            Self::SqlDoUpdateClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlConflictAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlDoNothingClause(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDoUpdateClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlConflictAction> for SyntaxNode {
    fn from(n: AnySqlConflictAction) -> Self {
        match n {
            AnySqlConflictAction::SqlDoNothingClause(it) => it.into(),
            AnySqlConflictAction::SqlDoUpdateClause(it) => it.into(),
        }
    }
}
impl From<AnySqlConflictAction> for SyntaxElement {
    fn from(n: AnySqlConflictAction) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlColumnList> for AnySqlConflictTarget {
    fn from(node: SqlColumnList) -> Self {
        Self::SqlColumnList(node)
    }
}
impl From<SqlOnConstraintClause> for AnySqlConflictTarget {
    fn from(node: SqlOnConstraintClause) -> Self {
        Self::SqlOnConstraintClause(node)
    }
}
impl AstNode for AnySqlConflictTarget {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlColumnList::KIND_SET.union(SqlOnConstraintClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_COLUMN_LIST | SQL_ON_CONSTRAINT_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_COLUMN_LIST => Self::SqlColumnList(SqlColumnList { syntax }),
            SQL_ON_CONSTRAINT_CLAUSE => {
                Self::SqlOnConstraintClause(SqlOnConstraintClause { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlColumnList(it) => &it.syntax,
            Self::SqlOnConstraintClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlColumnList(it) => it.syntax,
            Self::SqlOnConstraintClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlConflictTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlColumnList(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlOnConstraintClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlConflictTarget> for SyntaxNode {
    fn from(n: AnySqlConflictTarget) -> Self {
        match n {
            AnySqlConflictTarget::SqlColumnList(it) => it.into(),
            AnySqlConflictTarget::SqlOnConstraintClause(it) => it.into(),
        }
    }
}
impl From<AnySqlConflictTarget> for SyntaxElement {
    fn from(n: AnySqlConflictTarget) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlAnyAllExpression> for AnySqlExpression {
    fn from(node: SqlAnyAllExpression) -> Self {
        Self::SqlAnyAllExpression(node)
    }
}
impl From<SqlArrayExpression> for AnySqlExpression {
    fn from(node: SqlArrayExpression) -> Self {
        Self::SqlArrayExpression(node)
    }
}
impl From<SqlArraySubscriptExpression> for AnySqlExpression {
    fn from(node: SqlArraySubscriptExpression) -> Self {
        Self::SqlArraySubscriptExpression(node)
    }
}
impl From<SqlBetweenExpression> for AnySqlExpression {
    fn from(node: SqlBetweenExpression) -> Self {
        Self::SqlBetweenExpression(node)
    }
}
impl From<SqlBinaryExpression> for AnySqlExpression {
    fn from(node: SqlBinaryExpression) -> Self {
        Self::SqlBinaryExpression(node)
    }
}
impl From<SqlCallExpression> for AnySqlExpression {
    fn from(node: SqlCallExpression) -> Self {
        Self::SqlCallExpression(node)
    }
}
impl From<SqlCaseExpression> for AnySqlExpression {
    fn from(node: SqlCaseExpression) -> Self {
        Self::SqlCaseExpression(node)
    }
}
impl From<SqlCastExpression> for AnySqlExpression {
    fn from(node: SqlCastExpression) -> Self {
        Self::SqlCastExpression(node)
    }
}
impl From<SqlCastFunctionExpression> for AnySqlExpression {
    fn from(node: SqlCastFunctionExpression) -> Self {
        Self::SqlCastFunctionExpression(node)
    }
}
impl From<SqlColReference> for AnySqlExpression {
    fn from(node: SqlColReference) -> Self {
        Self::SqlColReference(node)
    }
}
impl From<SqlExistsExpression> for AnySqlExpression {
    fn from(node: SqlExistsExpression) -> Self {
        Self::SqlExistsExpression(node)
    }
}
impl From<SqlInExpression> for AnySqlExpression {
    fn from(node: SqlInExpression) -> Self {
        Self::SqlInExpression(node)
    }
}
impl From<SqlIntervalExpression> for AnySqlExpression {
    fn from(node: SqlIntervalExpression) -> Self {
        Self::SqlIntervalExpression(node)
    }
}
impl From<SqlIsNullExpression> for AnySqlExpression {
    fn from(node: SqlIsNullExpression) -> Self {
        Self::SqlIsNullExpression(node)
    }
}
impl From<SqlLikeExpression> for AnySqlExpression {
    fn from(node: SqlLikeExpression) -> Self {
        Self::SqlLikeExpression(node)
    }
}
impl From<SqlLogicalExpression> for AnySqlExpression {
    fn from(node: SqlLogicalExpression) -> Self {
        Self::SqlLogicalExpression(node)
    }
}
impl From<SqlName> for AnySqlExpression {
    fn from(node: SqlName) -> Self {
        Self::SqlName(node)
    }
}
impl From<SqlParameterExpression> for AnySqlExpression {
    fn from(node: SqlParameterExpression) -> Self {
        Self::SqlParameterExpression(node)
    }
}
impl From<SqlParenthesizedExpression> for AnySqlExpression {
    fn from(node: SqlParenthesizedExpression) -> Self {
        Self::SqlParenthesizedExpression(node)
    }
}
impl From<SqlStar> for AnySqlExpression {
    fn from(node: SqlStar) -> Self {
        Self::SqlStar(node)
    }
}
impl From<SqlSubqueryExpression> for AnySqlExpression {
    fn from(node: SqlSubqueryExpression) -> Self {
        Self::SqlSubqueryExpression(node)
    }
}
impl From<SqlSubstringExpression> for AnySqlExpression {
    fn from(node: SqlSubstringExpression) -> Self {
        Self::SqlSubstringExpression(node)
    }
}
impl From<SqlTableColReference> for AnySqlExpression {
    fn from(node: SqlTableColReference) -> Self {
        Self::SqlTableColReference(node)
    }
}
impl From<SqlTableStar> for AnySqlExpression {
    fn from(node: SqlTableStar) -> Self {
        Self::SqlTableStar(node)
    }
}
impl From<SqlTildeArrayExpression> for AnySqlExpression {
    fn from(node: SqlTildeArrayExpression) -> Self {
        Self::SqlTildeArrayExpression(node)
    }
}
impl From<SqlUnaryExpression> for AnySqlExpression {
    fn from(node: SqlUnaryExpression) -> Self {
        Self::SqlUnaryExpression(node)
    }
}
impl From<SqlWindowFunctionExpression> for AnySqlExpression {
    fn from(node: SqlWindowFunctionExpression) -> Self {
        Self::SqlWindowFunctionExpression(node)
    }
}
impl AstNode for AnySqlExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = AnySqlLiteralExpression::KIND_SET
        .union(SqlAnyAllExpression::KIND_SET)
        .union(SqlArrayExpression::KIND_SET)
        .union(SqlArraySubscriptExpression::KIND_SET)
        .union(SqlBetweenExpression::KIND_SET)
        .union(SqlBinaryExpression::KIND_SET)
        .union(SqlCallExpression::KIND_SET)
        .union(SqlCaseExpression::KIND_SET)
        .union(SqlCastExpression::KIND_SET)
        .union(SqlCastFunctionExpression::KIND_SET)
        .union(SqlColReference::KIND_SET)
        .union(SqlExistsExpression::KIND_SET)
        .union(SqlInExpression::KIND_SET)
        .union(SqlIntervalExpression::KIND_SET)
        .union(SqlIsNullExpression::KIND_SET)
        .union(SqlLikeExpression::KIND_SET)
        .union(SqlLogicalExpression::KIND_SET)
        .union(SqlName::KIND_SET)
        .union(SqlParameterExpression::KIND_SET)
        .union(SqlParenthesizedExpression::KIND_SET)
        .union(SqlStar::KIND_SET)
        .union(SqlSubqueryExpression::KIND_SET)
        .union(SqlSubstringExpression::KIND_SET)
        .union(SqlTableColReference::KIND_SET)
        .union(SqlTableStar::KIND_SET)
        .union(SqlTildeArrayExpression::KIND_SET)
        .union(SqlUnaryExpression::KIND_SET)
        .union(SqlWindowFunctionExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        match kind {
            SQL_ANY_ALL_EXPRESSION
            | SQL_ARRAY_EXPRESSION
            | SQL_ARRAY_SUBSCRIPT_EXPRESSION
            | SQL_BETWEEN_EXPRESSION
            | SQL_BINARY_EXPRESSION
            | SQL_CALL_EXPRESSION
            | SQL_CASE_EXPRESSION
            | SQL_CAST_EXPRESSION
            | SQL_CAST_FUNCTION_EXPRESSION
            | SQL_COL_REFERENCE
            | SQL_EXISTS_EXPRESSION
            | SQL_IN_EXPRESSION
            | SQL_INTERVAL_EXPRESSION
            | SQL_IS_NULL_EXPRESSION
            | SQL_LIKE_EXPRESSION
            | SQL_LOGICAL_EXPRESSION
            | SQL_NAME
            | SQL_PARAMETER_EXPRESSION
            | SQL_PARENTHESIZED_EXPRESSION
            | SQL_STAR
            | SQL_SUBQUERY_EXPRESSION
            | SQL_SUBSTRING_EXPRESSION
            | SQL_TABLE_COL_REFERENCE
            | SQL_TABLE_STAR
            | SQL_TILDE_ARRAY_EXPRESSION
            | SQL_UNARY_EXPRESSION
            | SQL_WINDOW_FUNCTION_EXPRESSION => true,
            k if AnySqlLiteralExpression::can_cast(k) => true,
            _ => false,
        }
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_ANY_ALL_EXPRESSION => Self::SqlAnyAllExpression(SqlAnyAllExpression { syntax }),
            SQL_ARRAY_EXPRESSION => Self::SqlArrayExpression(SqlArrayExpression { syntax }),
            SQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                Self::SqlArraySubscriptExpression(SqlArraySubscriptExpression { syntax })
            }
            SQL_BETWEEN_EXPRESSION => Self::SqlBetweenExpression(SqlBetweenExpression { syntax }),
            SQL_BINARY_EXPRESSION => Self::SqlBinaryExpression(SqlBinaryExpression { syntax }),
            SQL_CALL_EXPRESSION => Self::SqlCallExpression(SqlCallExpression { syntax }),
            SQL_CASE_EXPRESSION => Self::SqlCaseExpression(SqlCaseExpression { syntax }),
            SQL_CAST_EXPRESSION => Self::SqlCastExpression(SqlCastExpression { syntax }),
            SQL_CAST_FUNCTION_EXPRESSION => {
                Self::SqlCastFunctionExpression(SqlCastFunctionExpression { syntax })
            }
            SQL_COL_REFERENCE => Self::SqlColReference(SqlColReference { syntax }),
            SQL_EXISTS_EXPRESSION => Self::SqlExistsExpression(SqlExistsExpression { syntax }),
            SQL_IN_EXPRESSION => Self::SqlInExpression(SqlInExpression { syntax }),
            SQL_INTERVAL_EXPRESSION => {
                Self::SqlIntervalExpression(SqlIntervalExpression { syntax })
            }
            SQL_IS_NULL_EXPRESSION => Self::SqlIsNullExpression(SqlIsNullExpression { syntax }),
            SQL_LIKE_EXPRESSION => Self::SqlLikeExpression(SqlLikeExpression { syntax }),
            SQL_LOGICAL_EXPRESSION => Self::SqlLogicalExpression(SqlLogicalExpression { syntax }),
            SQL_NAME => Self::SqlName(SqlName { syntax }),
            SQL_PARAMETER_EXPRESSION => {
                Self::SqlParameterExpression(SqlParameterExpression { syntax })
            }
            SQL_PARENTHESIZED_EXPRESSION => {
                Self::SqlParenthesizedExpression(SqlParenthesizedExpression { syntax })
            }
            SQL_STAR => Self::SqlStar(SqlStar { syntax }),
            SQL_SUBQUERY_EXPRESSION => {
                Self::SqlSubqueryExpression(SqlSubqueryExpression { syntax })
            }
            SQL_SUBSTRING_EXPRESSION => {
                Self::SqlSubstringExpression(SqlSubstringExpression { syntax })
            }
            SQL_TABLE_COL_REFERENCE => Self::SqlTableColReference(SqlTableColReference { syntax }),
            SQL_TABLE_STAR => Self::SqlTableStar(SqlTableStar { syntax }),
            SQL_TILDE_ARRAY_EXPRESSION => {
                Self::SqlTildeArrayExpression(SqlTildeArrayExpression { syntax })
            }
            SQL_UNARY_EXPRESSION => Self::SqlUnaryExpression(SqlUnaryExpression { syntax }),
            SQL_WINDOW_FUNCTION_EXPRESSION => {
                Self::SqlWindowFunctionExpression(SqlWindowFunctionExpression { syntax })
            }
            _ => {
                if let Some(any_sql_literal_expression) = AnySqlLiteralExpression::cast(syntax) {
                    return Some(Self::AnySqlLiteralExpression(any_sql_literal_expression));
                }
                return None;
            }
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlAnyAllExpression(it) => &it.syntax,
            Self::SqlArrayExpression(it) => &it.syntax,
            Self::SqlArraySubscriptExpression(it) => &it.syntax,
            Self::SqlBetweenExpression(it) => &it.syntax,
            Self::SqlBinaryExpression(it) => &it.syntax,
            Self::SqlCallExpression(it) => &it.syntax,
            Self::SqlCaseExpression(it) => &it.syntax,
            Self::SqlCastExpression(it) => &it.syntax,
            Self::SqlCastFunctionExpression(it) => &it.syntax,
            Self::SqlColReference(it) => &it.syntax,
            Self::SqlExistsExpression(it) => &it.syntax,
            Self::SqlInExpression(it) => &it.syntax,
            Self::SqlIntervalExpression(it) => &it.syntax,
            Self::SqlIsNullExpression(it) => &it.syntax,
            Self::SqlLikeExpression(it) => &it.syntax,
            Self::SqlLogicalExpression(it) => &it.syntax,
            Self::SqlName(it) => &it.syntax,
            Self::SqlParameterExpression(it) => &it.syntax,
            Self::SqlParenthesizedExpression(it) => &it.syntax,
            Self::SqlStar(it) => &it.syntax,
            Self::SqlSubqueryExpression(it) => &it.syntax,
            Self::SqlSubstringExpression(it) => &it.syntax,
            Self::SqlTableColReference(it) => &it.syntax,
            Self::SqlTableStar(it) => &it.syntax,
            Self::SqlTildeArrayExpression(it) => &it.syntax,
            Self::SqlUnaryExpression(it) => &it.syntax,
            Self::SqlWindowFunctionExpression(it) => &it.syntax,
            Self::AnySqlLiteralExpression(it) => it.syntax(),
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlAnyAllExpression(it) => it.syntax,
            Self::SqlArrayExpression(it) => it.syntax,
            Self::SqlArraySubscriptExpression(it) => it.syntax,
            Self::SqlBetweenExpression(it) => it.syntax,
            Self::SqlBinaryExpression(it) => it.syntax,
            Self::SqlCallExpression(it) => it.syntax,
            Self::SqlCaseExpression(it) => it.syntax,
            Self::SqlCastExpression(it) => it.syntax,
            Self::SqlCastFunctionExpression(it) => it.syntax,
            Self::SqlColReference(it) => it.syntax,
            Self::SqlExistsExpression(it) => it.syntax,
            Self::SqlInExpression(it) => it.syntax,
            Self::SqlIntervalExpression(it) => it.syntax,
            Self::SqlIsNullExpression(it) => it.syntax,
            Self::SqlLikeExpression(it) => it.syntax,
            Self::SqlLogicalExpression(it) => it.syntax,
            Self::SqlName(it) => it.syntax,
            Self::SqlParameterExpression(it) => it.syntax,
            Self::SqlParenthesizedExpression(it) => it.syntax,
            Self::SqlStar(it) => it.syntax,
            Self::SqlSubqueryExpression(it) => it.syntax,
            Self::SqlSubstringExpression(it) => it.syntax,
            Self::SqlTableColReference(it) => it.syntax,
            Self::SqlTableStar(it) => it.syntax,
            Self::SqlTildeArrayExpression(it) => it.syntax,
            Self::SqlUnaryExpression(it) => it.syntax,
            Self::SqlWindowFunctionExpression(it) => it.syntax,
            Self::AnySqlLiteralExpression(it) => it.into_syntax(),
        }
    }
}
impl std::fmt::Debug for AnySqlExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::AnySqlLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlAnyAllExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlArrayExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlArraySubscriptExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlBetweenExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlBinaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCallExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCaseExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCastExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCastFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlColReference(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlExistsExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlInExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlIntervalExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlIsNullExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlLikeExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlLogicalExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlName(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlParameterExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlParenthesizedExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlStar(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubstringExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTableColReference(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTableStar(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTildeArrayExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlUnaryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlWindowFunctionExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlExpression> for SyntaxNode {
    fn from(n: AnySqlExpression) -> Self {
        match n {
            AnySqlExpression::AnySqlLiteralExpression(it) => it.into(),
            AnySqlExpression::SqlAnyAllExpression(it) => it.into(),
            AnySqlExpression::SqlArrayExpression(it) => it.into(),
            AnySqlExpression::SqlArraySubscriptExpression(it) => it.into(),
            AnySqlExpression::SqlBetweenExpression(it) => it.into(),
            AnySqlExpression::SqlBinaryExpression(it) => it.into(),
            AnySqlExpression::SqlCallExpression(it) => it.into(),
            AnySqlExpression::SqlCaseExpression(it) => it.into(),
            AnySqlExpression::SqlCastExpression(it) => it.into(),
            AnySqlExpression::SqlCastFunctionExpression(it) => it.into(),
            AnySqlExpression::SqlColReference(it) => it.into(),
            AnySqlExpression::SqlExistsExpression(it) => it.into(),
            AnySqlExpression::SqlInExpression(it) => it.into(),
            AnySqlExpression::SqlIntervalExpression(it) => it.into(),
            AnySqlExpression::SqlIsNullExpression(it) => it.into(),
            AnySqlExpression::SqlLikeExpression(it) => it.into(),
            AnySqlExpression::SqlLogicalExpression(it) => it.into(),
            AnySqlExpression::SqlName(it) => it.into(),
            AnySqlExpression::SqlParameterExpression(it) => it.into(),
            AnySqlExpression::SqlParenthesizedExpression(it) => it.into(),
            AnySqlExpression::SqlStar(it) => it.into(),
            AnySqlExpression::SqlSubqueryExpression(it) => it.into(),
            AnySqlExpression::SqlSubstringExpression(it) => it.into(),
            AnySqlExpression::SqlTableColReference(it) => it.into(),
            AnySqlExpression::SqlTableStar(it) => it.into(),
            AnySqlExpression::SqlTildeArrayExpression(it) => it.into(),
            AnySqlExpression::SqlUnaryExpression(it) => it.into(),
            AnySqlExpression::SqlWindowFunctionExpression(it) => it.into(),
        }
    }
}
impl From<AnySqlExpression> for SyntaxElement {
    fn from(n: AnySqlExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlFetchOnlyTail> for AnySqlFetchTail {
    fn from(node: SqlFetchOnlyTail) -> Self {
        Self::SqlFetchOnlyTail(node)
    }
}
impl From<SqlFetchWithTiesTail> for AnySqlFetchTail {
    fn from(node: SqlFetchWithTiesTail) -> Self {
        Self::SqlFetchWithTiesTail(node)
    }
}
impl AstNode for AnySqlFetchTail {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlFetchOnlyTail::KIND_SET.union(SqlFetchWithTiesTail::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_FETCH_ONLY_TAIL | SQL_FETCH_WITH_TIES_TAIL)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_FETCH_ONLY_TAIL => Self::SqlFetchOnlyTail(SqlFetchOnlyTail { syntax }),
            SQL_FETCH_WITH_TIES_TAIL => Self::SqlFetchWithTiesTail(SqlFetchWithTiesTail { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlFetchOnlyTail(it) => &it.syntax,
            Self::SqlFetchWithTiesTail(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlFetchOnlyTail(it) => it.syntax,
            Self::SqlFetchWithTiesTail(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlFetchTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlFetchOnlyTail(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlFetchWithTiesTail(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlFetchTail> for SyntaxNode {
    fn from(n: AnySqlFetchTail) -> Self {
        match n {
            AnySqlFetchTail::SqlFetchOnlyTail(it) => it.into(),
            AnySqlFetchTail::SqlFetchWithTiesTail(it) => it.into(),
        }
    }
}
impl From<AnySqlFetchTail> for SyntaxElement {
    fn from(n: AnySqlFetchTail) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlFunctionBinding> for AnySqlFromExpression {
    fn from(node: SqlFunctionBinding) -> Self {
        Self::SqlFunctionBinding(node)
    }
}
impl From<SqlParenthesizedJoinBinding> for AnySqlFromExpression {
    fn from(node: SqlParenthesizedJoinBinding) -> Self {
        Self::SqlParenthesizedJoinBinding(node)
    }
}
impl From<SqlSubqueryBinding> for AnySqlFromExpression {
    fn from(node: SqlSubqueryBinding) -> Self {
        Self::SqlSubqueryBinding(node)
    }
}
impl From<SqlTableBinding> for AnySqlFromExpression {
    fn from(node: SqlTableBinding) -> Self {
        Self::SqlTableBinding(node)
    }
}
impl AstNode for AnySqlFromExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlFunctionBinding::KIND_SET
        .union(SqlParenthesizedJoinBinding::KIND_SET)
        .union(SqlSubqueryBinding::KIND_SET)
        .union(SqlTableBinding::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_FUNCTION_BINDING
                | SQL_PARENTHESIZED_JOIN_BINDING
                | SQL_SUBQUERY_BINDING
                | SQL_TABLE_BINDING
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_FUNCTION_BINDING => Self::SqlFunctionBinding(SqlFunctionBinding { syntax }),
            SQL_PARENTHESIZED_JOIN_BINDING => {
                Self::SqlParenthesizedJoinBinding(SqlParenthesizedJoinBinding { syntax })
            }
            SQL_SUBQUERY_BINDING => Self::SqlSubqueryBinding(SqlSubqueryBinding { syntax }),
            SQL_TABLE_BINDING => Self::SqlTableBinding(SqlTableBinding { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlFunctionBinding(it) => &it.syntax,
            Self::SqlParenthesizedJoinBinding(it) => &it.syntax,
            Self::SqlSubqueryBinding(it) => &it.syntax,
            Self::SqlTableBinding(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlFunctionBinding(it) => it.syntax,
            Self::SqlParenthesizedJoinBinding(it) => it.syntax,
            Self::SqlSubqueryBinding(it) => it.syntax,
            Self::SqlTableBinding(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlFromExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlFunctionBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlParenthesizedJoinBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubqueryBinding(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTableBinding(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlFromExpression> for SyntaxNode {
    fn from(n: AnySqlFromExpression) -> Self {
        match n {
            AnySqlFromExpression::SqlFunctionBinding(it) => it.into(),
            AnySqlFromExpression::SqlParenthesizedJoinBinding(it) => it.into(),
            AnySqlFromExpression::SqlSubqueryBinding(it) => it.into(),
            AnySqlFromExpression::SqlTableBinding(it) => it.into(),
        }
    }
}
impl From<AnySqlFromExpression> for SyntaxElement {
    fn from(n: AnySqlFromExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlLanguageOption> for AnySqlFunctionOption {
    fn from(node: SqlLanguageOption) -> Self {
        Self::SqlLanguageOption(node)
    }
}
impl From<SqlReturnsNullOption> for AnySqlFunctionOption {
    fn from(node: SqlReturnsNullOption) -> Self {
        Self::SqlReturnsNullOption(node)
    }
}
impl From<SqlSecurityOption> for AnySqlFunctionOption {
    fn from(node: SqlSecurityOption) -> Self {
        Self::SqlSecurityOption(node)
    }
}
impl From<SqlStrictOption> for AnySqlFunctionOption {
    fn from(node: SqlStrictOption) -> Self {
        Self::SqlStrictOption(node)
    }
}
impl From<SqlVolatilityOption> for AnySqlFunctionOption {
    fn from(node: SqlVolatilityOption) -> Self {
        Self::SqlVolatilityOption(node)
    }
}
impl AstNode for AnySqlFunctionOption {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlLanguageOption::KIND_SET
        .union(SqlReturnsNullOption::KIND_SET)
        .union(SqlSecurityOption::KIND_SET)
        .union(SqlStrictOption::KIND_SET)
        .union(SqlVolatilityOption::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_LANGUAGE_OPTION
                | SQL_RETURNS_NULL_OPTION
                | SQL_SECURITY_OPTION
                | SQL_STRICT_OPTION
                | SQL_VOLATILITY_OPTION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_LANGUAGE_OPTION => Self::SqlLanguageOption(SqlLanguageOption { syntax }),
            SQL_RETURNS_NULL_OPTION => Self::SqlReturnsNullOption(SqlReturnsNullOption { syntax }),
            SQL_SECURITY_OPTION => Self::SqlSecurityOption(SqlSecurityOption { syntax }),
            SQL_STRICT_OPTION => Self::SqlStrictOption(SqlStrictOption { syntax }),
            SQL_VOLATILITY_OPTION => Self::SqlVolatilityOption(SqlVolatilityOption { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlLanguageOption(it) => &it.syntax,
            Self::SqlReturnsNullOption(it) => &it.syntax,
            Self::SqlSecurityOption(it) => &it.syntax,
            Self::SqlStrictOption(it) => &it.syntax,
            Self::SqlVolatilityOption(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlLanguageOption(it) => it.syntax,
            Self::SqlReturnsNullOption(it) => it.syntax,
            Self::SqlSecurityOption(it) => it.syntax,
            Self::SqlStrictOption(it) => it.syntax,
            Self::SqlVolatilityOption(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlFunctionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlLanguageOption(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlReturnsNullOption(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSecurityOption(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlStrictOption(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlVolatilityOption(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlFunctionOption> for SyntaxNode {
    fn from(n: AnySqlFunctionOption) -> Self {
        match n {
            AnySqlFunctionOption::SqlLanguageOption(it) => it.into(),
            AnySqlFunctionOption::SqlReturnsNullOption(it) => it.into(),
            AnySqlFunctionOption::SqlSecurityOption(it) => it.into(),
            AnySqlFunctionOption::SqlStrictOption(it) => it.into(),
            AnySqlFunctionOption::SqlVolatilityOption(it) => it.into(),
        }
    }
}
impl From<AnySqlFunctionOption> for SyntaxElement {
    fn from(n: AnySqlFunctionOption) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlInValueList> for AnySqlInSource {
    fn from(node: SqlInValueList) -> Self {
        Self::SqlInValueList(node)
    }
}
impl From<SqlSubqueryExpression> for AnySqlInSource {
    fn from(node: SqlSubqueryExpression) -> Self {
        Self::SqlSubqueryExpression(node)
    }
}
impl AstNode for AnySqlInSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlInValueList::KIND_SET.union(SqlSubqueryExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_IN_VALUE_LIST | SQL_SUBQUERY_EXPRESSION)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_IN_VALUE_LIST => Self::SqlInValueList(SqlInValueList { syntax }),
            SQL_SUBQUERY_EXPRESSION => {
                Self::SqlSubqueryExpression(SqlSubqueryExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlInValueList(it) => &it.syntax,
            Self::SqlSubqueryExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlInValueList(it) => it.syntax,
            Self::SqlSubqueryExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlInSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlInValueList(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlInSource> for SyntaxNode {
    fn from(n: AnySqlInSource) -> Self {
        match n {
            AnySqlInSource::SqlInValueList(it) => it.into(),
            AnySqlInSource::SqlSubqueryExpression(it) => it.into(),
        }
    }
}
impl From<AnySqlInSource> for SyntaxElement {
    fn from(n: AnySqlInSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlSelectStatement> for AnySqlInsertSource {
    fn from(node: SqlSelectStatement) -> Self {
        Self::SqlSelectStatement(node)
    }
}
impl From<SqlSubqueryExpression> for AnySqlInsertSource {
    fn from(node: SqlSubqueryExpression) -> Self {
        Self::SqlSubqueryExpression(node)
    }
}
impl From<SqlValuesClause> for AnySqlInsertSource {
    fn from(node: SqlValuesClause) -> Self {
        Self::SqlValuesClause(node)
    }
}
impl AstNode for AnySqlInsertSource {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlSelectStatement::KIND_SET
        .union(SqlSubqueryExpression::KIND_SET)
        .union(SqlValuesClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_SELECT_STATEMENT | SQL_SUBQUERY_EXPRESSION | SQL_VALUES_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_SELECT_STATEMENT => Self::SqlSelectStatement(SqlSelectStatement { syntax }),
            SQL_SUBQUERY_EXPRESSION => {
                Self::SqlSubqueryExpression(SqlSubqueryExpression { syntax })
            }
            SQL_VALUES_CLAUSE => Self::SqlValuesClause(SqlValuesClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlSelectStatement(it) => &it.syntax,
            Self::SqlSubqueryExpression(it) => &it.syntax,
            Self::SqlValuesClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlSelectStatement(it) => it.syntax,
            Self::SqlSubqueryExpression(it) => it.syntax,
            Self::SqlValuesClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlInsertSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlSelectStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSubqueryExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlValuesClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlInsertSource> for SyntaxNode {
    fn from(n: AnySqlInsertSource) -> Self {
        match n {
            AnySqlInsertSource::SqlSelectStatement(it) => it.into(),
            AnySqlInsertSource::SqlSubqueryExpression(it) => it.into(),
            AnySqlInsertSource::SqlValuesClause(it) => it.into(),
        }
    }
}
impl From<AnySqlInsertSource> for SyntaxElement {
    fn from(n: AnySqlInsertSource) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlNumberLiteralExpression> for AnySqlLimitValue {
    fn from(node: SqlNumberLiteralExpression) -> Self {
        Self::SqlNumberLiteralExpression(node)
    }
}
impl From<SqlParameterExpression> for AnySqlLimitValue {
    fn from(node: SqlParameterExpression) -> Self {
        Self::SqlParameterExpression(node)
    }
}
impl AstNode for AnySqlLimitValue {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlNumberLiteralExpression::KIND_SET.union(SqlParameterExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_NUMBER_LITERAL_EXPRESSION | SQL_PARAMETER_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_NUMBER_LITERAL_EXPRESSION => {
                Self::SqlNumberLiteralExpression(SqlNumberLiteralExpression { syntax })
            }
            SQL_PARAMETER_EXPRESSION => {
                Self::SqlParameterExpression(SqlParameterExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlNumberLiteralExpression(it) => &it.syntax,
            Self::SqlParameterExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlNumberLiteralExpression(it) => it.syntax,
            Self::SqlParameterExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlLimitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlParameterExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlLimitValue> for SyntaxNode {
    fn from(n: AnySqlLimitValue) -> Self {
        match n {
            AnySqlLimitValue::SqlNumberLiteralExpression(it) => it.into(),
            AnySqlLimitValue::SqlParameterExpression(it) => it.into(),
        }
    }
}
impl From<AnySqlLimitValue> for SyntaxElement {
    fn from(n: AnySqlLimitValue) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlBooleanLiteralExpression> for AnySqlLiteralExpression {
    fn from(node: SqlBooleanLiteralExpression) -> Self {
        Self::SqlBooleanLiteralExpression(node)
    }
}
impl From<SqlNullLiteralExpression> for AnySqlLiteralExpression {
    fn from(node: SqlNullLiteralExpression) -> Self {
        Self::SqlNullLiteralExpression(node)
    }
}
impl From<SqlNumberLiteralExpression> for AnySqlLiteralExpression {
    fn from(node: SqlNumberLiteralExpression) -> Self {
        Self::SqlNumberLiteralExpression(node)
    }
}
impl From<SqlStringLiteralExpression> for AnySqlLiteralExpression {
    fn from(node: SqlStringLiteralExpression) -> Self {
        Self::SqlStringLiteralExpression(node)
    }
}
impl AstNode for AnySqlLiteralExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlBooleanLiteralExpression::KIND_SET
        .union(SqlNullLiteralExpression::KIND_SET)
        .union(SqlNumberLiteralExpression::KIND_SET)
        .union(SqlStringLiteralExpression::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_BOOLEAN_LITERAL_EXPRESSION
                | SQL_NULL_LITERAL_EXPRESSION
                | SQL_NUMBER_LITERAL_EXPRESSION
                | SQL_STRING_LITERAL_EXPRESSION
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_BOOLEAN_LITERAL_EXPRESSION => {
                Self::SqlBooleanLiteralExpression(SqlBooleanLiteralExpression { syntax })
            }
            SQL_NULL_LITERAL_EXPRESSION => {
                Self::SqlNullLiteralExpression(SqlNullLiteralExpression { syntax })
            }
            SQL_NUMBER_LITERAL_EXPRESSION => {
                Self::SqlNumberLiteralExpression(SqlNumberLiteralExpression { syntax })
            }
            SQL_STRING_LITERAL_EXPRESSION => {
                Self::SqlStringLiteralExpression(SqlStringLiteralExpression { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlBooleanLiteralExpression(it) => &it.syntax,
            Self::SqlNullLiteralExpression(it) => &it.syntax,
            Self::SqlNumberLiteralExpression(it) => &it.syntax,
            Self::SqlStringLiteralExpression(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlBooleanLiteralExpression(it) => it.syntax,
            Self::SqlNullLiteralExpression(it) => it.syntax,
            Self::SqlNumberLiteralExpression(it) => it.syntax,
            Self::SqlStringLiteralExpression(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlBooleanLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlNullLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlNumberLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlStringLiteralExpression(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlLiteralExpression> for SyntaxNode {
    fn from(n: AnySqlLiteralExpression) -> Self {
        match n {
            AnySqlLiteralExpression::SqlBooleanLiteralExpression(it) => it.into(),
            AnySqlLiteralExpression::SqlNullLiteralExpression(it) => it.into(),
            AnySqlLiteralExpression::SqlNumberLiteralExpression(it) => it.into(),
            AnySqlLiteralExpression::SqlStringLiteralExpression(it) => it.into(),
        }
    }
}
impl From<AnySqlLiteralExpression> for SyntaxElement {
    fn from(n: AnySqlLiteralExpression) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlName> for AnySqlName {
    fn from(node: SqlName) -> Self {
        Self::SqlName(node)
    }
}
impl From<SqlTildeName> for AnySqlName {
    fn from(node: SqlTildeName) -> Self {
        Self::SqlTildeName(node)
    }
}
impl AstNode for AnySqlName {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlName::KIND_SET.union(SqlTildeName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_NAME | SQL_TILDE_NAME)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_NAME => Self::SqlName(SqlName { syntax }),
            SQL_TILDE_NAME => Self::SqlTildeName(SqlTildeName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlName(it) => &it.syntax,
            Self::SqlTildeName(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlName(it) => it.syntax,
            Self::SqlTildeName(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlName(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTildeName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlName> for SyntaxNode {
    fn from(n: AnySqlName) -> Self {
        match n {
            AnySqlName::SqlName(it) => it.into(),
            AnySqlName::SqlTildeName(it) => it.into(),
        }
    }
}
impl From<AnySqlName> for SyntaxElement {
    fn from(n: AnySqlName) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlReturnsSetofClause> for AnySqlReturnsType {
    fn from(node: SqlReturnsSetofClause) -> Self {
        Self::SqlReturnsSetofClause(node)
    }
}
impl From<SqlReturnsTableClause> for AnySqlReturnsType {
    fn from(node: SqlReturnsTableClause) -> Self {
        Self::SqlReturnsTableClause(node)
    }
}
impl From<SqlReturnsTriggerClause> for AnySqlReturnsType {
    fn from(node: SqlReturnsTriggerClause) -> Self {
        Self::SqlReturnsTriggerClause(node)
    }
}
impl From<SqlTypeName> for AnySqlReturnsType {
    fn from(node: SqlTypeName) -> Self {
        Self::SqlTypeName(node)
    }
}
impl AstNode for AnySqlReturnsType {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlReturnsSetofClause::KIND_SET
        .union(SqlReturnsTableClause::KIND_SET)
        .union(SqlReturnsTriggerClause::KIND_SET)
        .union(SqlTypeName::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_RETURNS_SETOF_CLAUSE
                | SQL_RETURNS_TABLE_CLAUSE
                | SQL_RETURNS_TRIGGER_CLAUSE
                | SQL_TYPE_NAME
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_RETURNS_SETOF_CLAUSE => {
                Self::SqlReturnsSetofClause(SqlReturnsSetofClause { syntax })
            }
            SQL_RETURNS_TABLE_CLAUSE => {
                Self::SqlReturnsTableClause(SqlReturnsTableClause { syntax })
            }
            SQL_RETURNS_TRIGGER_CLAUSE => {
                Self::SqlReturnsTriggerClause(SqlReturnsTriggerClause { syntax })
            }
            SQL_TYPE_NAME => Self::SqlTypeName(SqlTypeName { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlReturnsSetofClause(it) => &it.syntax,
            Self::SqlReturnsTableClause(it) => &it.syntax,
            Self::SqlReturnsTriggerClause(it) => &it.syntax,
            Self::SqlTypeName(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlReturnsSetofClause(it) => it.syntax,
            Self::SqlReturnsTableClause(it) => it.syntax,
            Self::SqlReturnsTriggerClause(it) => it.syntax,
            Self::SqlTypeName(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlReturnsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlReturnsSetofClause(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlReturnsTableClause(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlReturnsTriggerClause(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTypeName(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlReturnsType> for SyntaxNode {
    fn from(n: AnySqlReturnsType) -> Self {
        match n {
            AnySqlReturnsType::SqlReturnsSetofClause(it) => it.into(),
            AnySqlReturnsType::SqlReturnsTableClause(it) => it.into(),
            AnySqlReturnsType::SqlReturnsTriggerClause(it) => it.into(),
            AnySqlReturnsType::SqlTypeName(it) => it.into(),
        }
    }
}
impl From<AnySqlReturnsType> for SyntaxElement {
    fn from(n: AnySqlReturnsType) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlSelectExpression> for AnySqlSelectItem {
    fn from(node: SqlSelectExpression) -> Self {
        Self::SqlSelectExpression(node)
    }
}
impl From<SqlStar> for AnySqlSelectItem {
    fn from(node: SqlStar) -> Self {
        Self::SqlStar(node)
    }
}
impl From<SqlTableStar> for AnySqlSelectItem {
    fn from(node: SqlTableStar) -> Self {
        Self::SqlTableStar(node)
    }
}
impl AstNode for AnySqlSelectItem {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlSelectExpression::KIND_SET
        .union(SqlStar::KIND_SET)
        .union(SqlTableStar::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_SELECT_EXPRESSION | SQL_STAR | SQL_TABLE_STAR)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_SELECT_EXPRESSION => Self::SqlSelectExpression(SqlSelectExpression { syntax }),
            SQL_STAR => Self::SqlStar(SqlStar { syntax }),
            SQL_TABLE_STAR => Self::SqlTableStar(SqlTableStar { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlSelectExpression(it) => &it.syntax,
            Self::SqlStar(it) => &it.syntax,
            Self::SqlTableStar(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlSelectExpression(it) => it.syntax,
            Self::SqlStar(it) => it.syntax,
            Self::SqlTableStar(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlSelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlSelectExpression(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlStar(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTableStar(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlSelectItem> for SyntaxNode {
    fn from(n: AnySqlSelectItem) -> Self {
        match n {
            AnySqlSelectItem::SqlSelectExpression(it) => it.into(),
            AnySqlSelectItem::SqlStar(it) => it.into(),
            AnySqlSelectItem::SqlTableStar(it) => it.into(),
        }
    }
}
impl From<AnySqlSelectItem> for SyntaxElement {
    fn from(n: AnySqlSelectItem) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlSelectAllQuantifier> for AnySqlSelectQuantifier {
    fn from(node: SqlSelectAllQuantifier) -> Self {
        Self::SqlSelectAllQuantifier(node)
    }
}
impl From<SqlSelectDistinctQuantifier> for AnySqlSelectQuantifier {
    fn from(node: SqlSelectDistinctQuantifier) -> Self {
        Self::SqlSelectDistinctQuantifier(node)
    }
}
impl AstNode for AnySqlSelectQuantifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlSelectAllQuantifier::KIND_SET.union(SqlSelectDistinctQuantifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_SELECT_ALL_QUANTIFIER | SQL_SELECT_DISTINCT_QUANTIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_SELECT_ALL_QUANTIFIER => {
                Self::SqlSelectAllQuantifier(SqlSelectAllQuantifier { syntax })
            }
            SQL_SELECT_DISTINCT_QUANTIFIER => {
                Self::SqlSelectDistinctQuantifier(SqlSelectDistinctQuantifier { syntax })
            }
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlSelectAllQuantifier(it) => &it.syntax,
            Self::SqlSelectDistinctQuantifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlSelectAllQuantifier(it) => it.syntax,
            Self::SqlSelectDistinctQuantifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlSelectQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlSelectAllQuantifier(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSelectDistinctQuantifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlSelectQuantifier> for SyntaxNode {
    fn from(n: AnySqlSelectQuantifier) -> Self {
        match n {
            AnySqlSelectQuantifier::SqlSelectAllQuantifier(it) => it.into(),
            AnySqlSelectQuantifier::SqlSelectDistinctQuantifier(it) => it.into(),
        }
    }
}
impl From<AnySqlSelectQuantifier> for SyntaxElement {
    fn from(n: AnySqlSelectQuantifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlBogusStatement> for AnySqlStatement {
    fn from(node: SqlBogusStatement) -> Self {
        Self::SqlBogusStatement(node)
    }
}
impl From<SqlCreateFunctionStatement> for AnySqlStatement {
    fn from(node: SqlCreateFunctionStatement) -> Self {
        Self::SqlCreateFunctionStatement(node)
    }
}
impl From<SqlCreatePolicyStatement> for AnySqlStatement {
    fn from(node: SqlCreatePolicyStatement) -> Self {
        Self::SqlCreatePolicyStatement(node)
    }
}
impl From<SqlCreateTableStatement> for AnySqlStatement {
    fn from(node: SqlCreateTableStatement) -> Self {
        Self::SqlCreateTableStatement(node)
    }
}
impl From<SqlCreateTriggerStatement> for AnySqlStatement {
    fn from(node: SqlCreateTriggerStatement) -> Self {
        Self::SqlCreateTriggerStatement(node)
    }
}
impl From<SqlCreateViewStatement> for AnySqlStatement {
    fn from(node: SqlCreateViewStatement) -> Self {
        Self::SqlCreateViewStatement(node)
    }
}
impl From<SqlDeleteStatement> for AnySqlStatement {
    fn from(node: SqlDeleteStatement) -> Self {
        Self::SqlDeleteStatement(node)
    }
}
impl From<SqlDropFunctionStatement> for AnySqlStatement {
    fn from(node: SqlDropFunctionStatement) -> Self {
        Self::SqlDropFunctionStatement(node)
    }
}
impl From<SqlDropPolicyStatement> for AnySqlStatement {
    fn from(node: SqlDropPolicyStatement) -> Self {
        Self::SqlDropPolicyStatement(node)
    }
}
impl From<SqlDropTableStatement> for AnySqlStatement {
    fn from(node: SqlDropTableStatement) -> Self {
        Self::SqlDropTableStatement(node)
    }
}
impl From<SqlDropTriggerStatement> for AnySqlStatement {
    fn from(node: SqlDropTriggerStatement) -> Self {
        Self::SqlDropTriggerStatement(node)
    }
}
impl From<SqlDropViewStatement> for AnySqlStatement {
    fn from(node: SqlDropViewStatement) -> Self {
        Self::SqlDropViewStatement(node)
    }
}
impl From<SqlEmptyStatement> for AnySqlStatement {
    fn from(node: SqlEmptyStatement) -> Self {
        Self::SqlEmptyStatement(node)
    }
}
impl From<SqlGrantStatement> for AnySqlStatement {
    fn from(node: SqlGrantStatement) -> Self {
        Self::SqlGrantStatement(node)
    }
}
impl From<SqlInsertStatement> for AnySqlStatement {
    fn from(node: SqlInsertStatement) -> Self {
        Self::SqlInsertStatement(node)
    }
}
impl From<SqlSelectStatement> for AnySqlStatement {
    fn from(node: SqlSelectStatement) -> Self {
        Self::SqlSelectStatement(node)
    }
}
impl From<SqlUpdateStatement> for AnySqlStatement {
    fn from(node: SqlUpdateStatement) -> Self {
        Self::SqlUpdateStatement(node)
    }
}
impl From<SqlValuesClause> for AnySqlStatement {
    fn from(node: SqlValuesClause) -> Self {
        Self::SqlValuesClause(node)
    }
}
impl AstNode for AnySqlStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlBogusStatement::KIND_SET
        .union(SqlCreateFunctionStatement::KIND_SET)
        .union(SqlCreatePolicyStatement::KIND_SET)
        .union(SqlCreateTableStatement::KIND_SET)
        .union(SqlCreateTriggerStatement::KIND_SET)
        .union(SqlCreateViewStatement::KIND_SET)
        .union(SqlDeleteStatement::KIND_SET)
        .union(SqlDropFunctionStatement::KIND_SET)
        .union(SqlDropPolicyStatement::KIND_SET)
        .union(SqlDropTableStatement::KIND_SET)
        .union(SqlDropTriggerStatement::KIND_SET)
        .union(SqlDropViewStatement::KIND_SET)
        .union(SqlEmptyStatement::KIND_SET)
        .union(SqlGrantStatement::KIND_SET)
        .union(SqlInsertStatement::KIND_SET)
        .union(SqlSelectStatement::KIND_SET)
        .union(SqlUpdateStatement::KIND_SET)
        .union(SqlValuesClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_BOGUS_STATEMENT
                | SQL_CREATE_FUNCTION_STATEMENT
                | SQL_CREATE_POLICY_STATEMENT
                | SQL_CREATE_TABLE_STATEMENT
                | SQL_CREATE_TRIGGER_STATEMENT
                | SQL_CREATE_VIEW_STATEMENT
                | SQL_DELETE_STATEMENT
                | SQL_DROP_FUNCTION_STATEMENT
                | SQL_DROP_POLICY_STATEMENT
                | SQL_DROP_TABLE_STATEMENT
                | SQL_DROP_TRIGGER_STATEMENT
                | SQL_DROP_VIEW_STATEMENT
                | SQL_EMPTY_STATEMENT
                | SQL_GRANT_STATEMENT
                | SQL_INSERT_STATEMENT
                | SQL_SELECT_STATEMENT
                | SQL_UPDATE_STATEMENT
                | SQL_VALUES_CLAUSE
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_BOGUS_STATEMENT => Self::SqlBogusStatement(SqlBogusStatement { syntax }),
            SQL_CREATE_FUNCTION_STATEMENT => {
                Self::SqlCreateFunctionStatement(SqlCreateFunctionStatement { syntax })
            }
            SQL_CREATE_POLICY_STATEMENT => {
                Self::SqlCreatePolicyStatement(SqlCreatePolicyStatement { syntax })
            }
            SQL_CREATE_TABLE_STATEMENT => {
                Self::SqlCreateTableStatement(SqlCreateTableStatement { syntax })
            }
            SQL_CREATE_TRIGGER_STATEMENT => {
                Self::SqlCreateTriggerStatement(SqlCreateTriggerStatement { syntax })
            }
            SQL_CREATE_VIEW_STATEMENT => {
                Self::SqlCreateViewStatement(SqlCreateViewStatement { syntax })
            }
            SQL_DELETE_STATEMENT => Self::SqlDeleteStatement(SqlDeleteStatement { syntax }),
            SQL_DROP_FUNCTION_STATEMENT => {
                Self::SqlDropFunctionStatement(SqlDropFunctionStatement { syntax })
            }
            SQL_DROP_POLICY_STATEMENT => {
                Self::SqlDropPolicyStatement(SqlDropPolicyStatement { syntax })
            }
            SQL_DROP_TABLE_STATEMENT => {
                Self::SqlDropTableStatement(SqlDropTableStatement { syntax })
            }
            SQL_DROP_TRIGGER_STATEMENT => {
                Self::SqlDropTriggerStatement(SqlDropTriggerStatement { syntax })
            }
            SQL_DROP_VIEW_STATEMENT => Self::SqlDropViewStatement(SqlDropViewStatement { syntax }),
            SQL_EMPTY_STATEMENT => Self::SqlEmptyStatement(SqlEmptyStatement { syntax }),
            SQL_GRANT_STATEMENT => Self::SqlGrantStatement(SqlGrantStatement { syntax }),
            SQL_INSERT_STATEMENT => Self::SqlInsertStatement(SqlInsertStatement { syntax }),
            SQL_SELECT_STATEMENT => Self::SqlSelectStatement(SqlSelectStatement { syntax }),
            SQL_UPDATE_STATEMENT => Self::SqlUpdateStatement(SqlUpdateStatement { syntax }),
            SQL_VALUES_CLAUSE => Self::SqlValuesClause(SqlValuesClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlBogusStatement(it) => &it.syntax,
            Self::SqlCreateFunctionStatement(it) => &it.syntax,
            Self::SqlCreatePolicyStatement(it) => &it.syntax,
            Self::SqlCreateTableStatement(it) => &it.syntax,
            Self::SqlCreateTriggerStatement(it) => &it.syntax,
            Self::SqlCreateViewStatement(it) => &it.syntax,
            Self::SqlDeleteStatement(it) => &it.syntax,
            Self::SqlDropFunctionStatement(it) => &it.syntax,
            Self::SqlDropPolicyStatement(it) => &it.syntax,
            Self::SqlDropTableStatement(it) => &it.syntax,
            Self::SqlDropTriggerStatement(it) => &it.syntax,
            Self::SqlDropViewStatement(it) => &it.syntax,
            Self::SqlEmptyStatement(it) => &it.syntax,
            Self::SqlGrantStatement(it) => &it.syntax,
            Self::SqlInsertStatement(it) => &it.syntax,
            Self::SqlSelectStatement(it) => &it.syntax,
            Self::SqlUpdateStatement(it) => &it.syntax,
            Self::SqlValuesClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlBogusStatement(it) => it.syntax,
            Self::SqlCreateFunctionStatement(it) => it.syntax,
            Self::SqlCreatePolicyStatement(it) => it.syntax,
            Self::SqlCreateTableStatement(it) => it.syntax,
            Self::SqlCreateTriggerStatement(it) => it.syntax,
            Self::SqlCreateViewStatement(it) => it.syntax,
            Self::SqlDeleteStatement(it) => it.syntax,
            Self::SqlDropFunctionStatement(it) => it.syntax,
            Self::SqlDropPolicyStatement(it) => it.syntax,
            Self::SqlDropTableStatement(it) => it.syntax,
            Self::SqlDropTriggerStatement(it) => it.syntax,
            Self::SqlDropViewStatement(it) => it.syntax,
            Self::SqlEmptyStatement(it) => it.syntax,
            Self::SqlGrantStatement(it) => it.syntax,
            Self::SqlInsertStatement(it) => it.syntax,
            Self::SqlSelectStatement(it) => it.syntax,
            Self::SqlUpdateStatement(it) => it.syntax,
            Self::SqlValuesClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlBogusStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCreateFunctionStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCreatePolicyStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCreateTableStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCreateTriggerStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlCreateViewStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDeleteStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDropFunctionStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDropPolicyStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDropTableStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDropTriggerStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlDropViewStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlEmptyStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlGrantStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlInsertStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlSelectStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlUpdateStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlValuesClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlStatement> for SyntaxNode {
    fn from(n: AnySqlStatement) -> Self {
        match n {
            AnySqlStatement::SqlBogusStatement(it) => it.into(),
            AnySqlStatement::SqlCreateFunctionStatement(it) => it.into(),
            AnySqlStatement::SqlCreatePolicyStatement(it) => it.into(),
            AnySqlStatement::SqlCreateTableStatement(it) => it.into(),
            AnySqlStatement::SqlCreateTriggerStatement(it) => it.into(),
            AnySqlStatement::SqlCreateViewStatement(it) => it.into(),
            AnySqlStatement::SqlDeleteStatement(it) => it.into(),
            AnySqlStatement::SqlDropFunctionStatement(it) => it.into(),
            AnySqlStatement::SqlDropPolicyStatement(it) => it.into(),
            AnySqlStatement::SqlDropTableStatement(it) => it.into(),
            AnySqlStatement::SqlDropTriggerStatement(it) => it.into(),
            AnySqlStatement::SqlDropViewStatement(it) => it.into(),
            AnySqlStatement::SqlEmptyStatement(it) => it.into(),
            AnySqlStatement::SqlGrantStatement(it) => it.into(),
            AnySqlStatement::SqlInsertStatement(it) => it.into(),
            AnySqlStatement::SqlSelectStatement(it) => it.into(),
            AnySqlStatement::SqlUpdateStatement(it) => it.into(),
            AnySqlStatement::SqlValuesClause(it) => it.into(),
        }
    }
}
impl From<AnySqlStatement> for SyntaxElement {
    fn from(n: AnySqlStatement) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlSelectStatement> for AnySqlSubqueryBody {
    fn from(node: SqlSelectStatement) -> Self {
        Self::SqlSelectStatement(node)
    }
}
impl From<SqlValuesClause> for AnySqlSubqueryBody {
    fn from(node: SqlValuesClause) -> Self {
        Self::SqlValuesClause(node)
    }
}
impl AstNode for AnySqlSubqueryBody {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlSelectStatement::KIND_SET.union(SqlValuesClause::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_SELECT_STATEMENT | SQL_VALUES_CLAUSE)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_SELECT_STATEMENT => Self::SqlSelectStatement(SqlSelectStatement { syntax }),
            SQL_VALUES_CLAUSE => Self::SqlValuesClause(SqlValuesClause { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlSelectStatement(it) => &it.syntax,
            Self::SqlValuesClause(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlSelectStatement(it) => it.syntax,
            Self::SqlValuesClause(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlSubqueryBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlSelectStatement(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlValuesClause(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlSubqueryBody> for SyntaxNode {
    fn from(n: AnySqlSubqueryBody) -> Self {
        match n {
            AnySqlSubqueryBody::SqlSelectStatement(it) => it.into(),
            AnySqlSubqueryBody::SqlValuesClause(it) => it.into(),
        }
    }
}
impl From<AnySqlSubqueryBody> for SyntaxElement {
    fn from(n: AnySqlSubqueryBody) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlTildeArraySuffix> for AnySqlTypeArraySuffix {
    fn from(node: SqlTildeArraySuffix) -> Self {
        Self::SqlTildeArraySuffix(node)
    }
}
impl From<SqlTypeArraySuffix> for AnySqlTypeArraySuffix {
    fn from(node: SqlTypeArraySuffix) -> Self {
        Self::SqlTypeArraySuffix(node)
    }
}
impl AstNode for AnySqlTypeArraySuffix {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SqlTildeArraySuffix::KIND_SET.union(SqlTypeArraySuffix::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(kind, SQL_TILDE_ARRAY_SUFFIX | SQL_TYPE_ARRAY_SUFFIX)
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_TILDE_ARRAY_SUFFIX => Self::SqlTildeArraySuffix(SqlTildeArraySuffix { syntax }),
            SQL_TYPE_ARRAY_SUFFIX => Self::SqlTypeArraySuffix(SqlTypeArraySuffix { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlTildeArraySuffix(it) => &it.syntax,
            Self::SqlTypeArraySuffix(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlTildeArraySuffix(it) => it.syntax,
            Self::SqlTypeArraySuffix(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlTildeArraySuffix(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTypeArraySuffix(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlTypeArraySuffix> for SyntaxNode {
    fn from(n: AnySqlTypeArraySuffix) -> Self {
        match n {
            AnySqlTypeArraySuffix::SqlTildeArraySuffix(it) => it.into(),
            AnySqlTypeArraySuffix::SqlTypeArraySuffix(it) => it.into(),
        }
    }
}
impl From<AnySqlTypeArraySuffix> for SyntaxElement {
    fn from(n: AnySqlTypeArraySuffix) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl From<SqlPrecisionModifier> for AnySqlTypeModifier {
    fn from(node: SqlPrecisionModifier) -> Self {
        Self::SqlPrecisionModifier(node)
    }
}
impl From<SqlTimeZoneModifier> for AnySqlTypeModifier {
    fn from(node: SqlTimeZoneModifier) -> Self {
        Self::SqlTimeZoneModifier(node)
    }
}
impl From<SqlVaryingModifier> for AnySqlTypeModifier {
    fn from(node: SqlVaryingModifier) -> Self {
        Self::SqlVaryingModifier(node)
    }
}
impl AstNode for AnySqlTypeModifier {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> = SqlPrecisionModifier::KIND_SET
        .union(SqlTimeZoneModifier::KIND_SET)
        .union(SqlVaryingModifier::KIND_SET);
    fn can_cast(kind: SyntaxKind) -> bool {
        matches!(
            kind,
            SQL_PRECISION_MODIFIER | SQL_TIME_ZONE_MODIFIER | SQL_VARYING_MODIFIER
        )
    }
    fn cast(syntax: SyntaxNode) -> Option<Self> {
        let res = match syntax.kind() {
            SQL_PRECISION_MODIFIER => Self::SqlPrecisionModifier(SqlPrecisionModifier { syntax }),
            SQL_TIME_ZONE_MODIFIER => Self::SqlTimeZoneModifier(SqlTimeZoneModifier { syntax }),
            SQL_VARYING_MODIFIER => Self::SqlVaryingModifier(SqlVaryingModifier { syntax }),
            _ => return None,
        };
        Some(res)
    }
    fn syntax(&self) -> &SyntaxNode {
        match self {
            Self::SqlPrecisionModifier(it) => &it.syntax,
            Self::SqlTimeZoneModifier(it) => &it.syntax,
            Self::SqlVaryingModifier(it) => &it.syntax,
        }
    }
    fn into_syntax(self) -> SyntaxNode {
        match self {
            Self::SqlPrecisionModifier(it) => it.syntax,
            Self::SqlTimeZoneModifier(it) => it.syntax,
            Self::SqlVaryingModifier(it) => it.syntax,
        }
    }
}
impl std::fmt::Debug for AnySqlTypeModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SqlPrecisionModifier(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlTimeZoneModifier(it) => std::fmt::Debug::fmt(it, f),
            Self::SqlVaryingModifier(it) => std::fmt::Debug::fmt(it, f),
        }
    }
}
impl From<AnySqlTypeModifier> for SyntaxNode {
    fn from(n: AnySqlTypeModifier) -> Self {
        match n {
            AnySqlTypeModifier::SqlPrecisionModifier(it) => it.into(),
            AnySqlTypeModifier::SqlTimeZoneModifier(it) => it.into(),
            AnySqlTypeModifier::SqlVaryingModifier(it) => it.into(),
        }
    }
}
impl From<AnySqlTypeModifier> for SyntaxElement {
    fn from(n: AnySqlTypeModifier) -> Self {
        let node: SyntaxNode = n.into();
        node.into()
    }
}
impl std::fmt::Display for AnySqlAnyAllSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlConflictAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlConflictTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlFetchTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlFromExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlFunctionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlInSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlInsertSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlLimitValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlReturnsType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlSelectItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlSelectQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlSubqueryBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for AnySqlTypeModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlAliasColumnDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlAliasColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlAnyAllExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlArraySubscriptExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlBetweenExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlBinaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlBooleanLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCallExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCaseElseClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCaseExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCaseWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCastExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCastFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlColumnDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlColumnList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCreateFunctionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCreatePolicyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCreateTableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCreateTriggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCreateViewStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCteDefinition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlCteMaterializedHint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDataBaseName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDeleteStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDeleteUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDistinctOnClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDoNothingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDoUpdateClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropFunctionParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropFunctionStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropPolicyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropTableStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropTriggerStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlDropViewStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlEmptyStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlExistsExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFetchClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFetchOnlyTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFetchWithTiesTail {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFilterClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFromItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFunctionBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlFunctionParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlGrantStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlGroupByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlHavingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlInExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlInValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlInsertStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlIntervalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlIsNullExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlJoinClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlJoinUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlLanguageOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlLikeExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlLimitClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlLogicalExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlNullLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlNumberLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlOffsetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlOnConflictClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlOnConstraintClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlOrderByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlOrderByExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlParameterDefault {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlParameterExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlParenthesizedExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlParenthesizedJoinBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlPolicyForClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlPolicyUsingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlPolicyWithCheckClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlPrecisionModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturningClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsNullOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsSetofClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsTableClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsTableColumn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlReturnsTriggerClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlRoot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSecurityOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSelectAllQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSelectClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSelectDistinctQuantifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSelectExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSelectStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSetClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSetItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSetOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlShemaName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlStrictOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlStringLiteralExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSubqueryBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSubqueryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSubstringExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSubstringForClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlSubstringFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTableBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTableColReference {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTableName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTableStar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTildeArrayExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTildeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTildeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTimeZoneModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerForEachClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerReferencingClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerReferencingItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerUpdateOfClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTriggerWhenClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTypeArguments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTypeArraySuffix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlTypeName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlUnaryExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlUpdateFromClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlUpdateStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlValuesClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlValuesRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlVaryingModifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlViewOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlViewOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlVolatilityOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlWhereClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlWindowFunctionExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlWindowPartitionByClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlWindowSpecification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
impl std::fmt::Display for SqlWithClause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(self.syntax(), f)
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogus {
    syntax: SyntaxNode,
}
impl SqlBogus {
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
impl AstNode for SqlBogus {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS
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
impl std::fmt::Debug for SqlBogus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogus")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogus> for SyntaxNode {
    fn from(n: SqlBogus) -> Self {
        n.syntax
    }
}
impl From<SqlBogus> for SyntaxElement {
    fn from(n: SqlBogus) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusAssignment {
    syntax: SyntaxNode,
}
impl SqlBogusAssignment {
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
impl AstNode for SqlBogusAssignment {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_ASSIGNMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_ASSIGNMENT
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
impl std::fmt::Debug for SqlBogusAssignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusAssignment")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusAssignment> for SyntaxNode {
    fn from(n: SqlBogusAssignment) -> Self {
        n.syntax
    }
}
impl From<SqlBogusAssignment> for SyntaxElement {
    fn from(n: SqlBogusAssignment) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusBinding {
    syntax: SyntaxNode,
}
impl SqlBogusBinding {
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
impl AstNode for SqlBogusBinding {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_BINDING as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_BINDING
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
impl std::fmt::Debug for SqlBogusBinding {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusBinding")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusBinding> for SyntaxNode {
    fn from(n: SqlBogusBinding) -> Self {
        n.syntax
    }
}
impl From<SqlBogusBinding> for SyntaxElement {
    fn from(n: SqlBogusBinding) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusExpression {
    syntax: SyntaxNode,
}
impl SqlBogusExpression {
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
impl AstNode for SqlBogusExpression {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_EXPRESSION as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_EXPRESSION
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
impl std::fmt::Debug for SqlBogusExpression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusExpression")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusExpression> for SyntaxNode {
    fn from(n: SqlBogusExpression) -> Self {
        n.syntax
    }
}
impl From<SqlBogusExpression> for SyntaxElement {
    fn from(n: SqlBogusExpression) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusMember {
    syntax: SyntaxNode,
}
impl SqlBogusMember {
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
impl AstNode for SqlBogusMember {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_MEMBER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_MEMBER
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
impl std::fmt::Debug for SqlBogusMember {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusMember")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusMember> for SyntaxNode {
    fn from(n: SqlBogusMember) -> Self {
        n.syntax
    }
}
impl From<SqlBogusMember> for SyntaxElement {
    fn from(n: SqlBogusMember) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusParameter {
    syntax: SyntaxNode,
}
impl SqlBogusParameter {
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
impl AstNode for SqlBogusParameter {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_PARAMETER as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_PARAMETER
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
impl std::fmt::Debug for SqlBogusParameter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusParameter")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusParameter> for SyntaxNode {
    fn from(n: SqlBogusParameter) -> Self {
        n.syntax
    }
}
impl From<SqlBogusParameter> for SyntaxElement {
    fn from(n: SqlBogusParameter) -> Self {
        n.syntax.into()
    }
}
#[derive(Clone, PartialEq, Eq, Hash, Serialize)]
pub struct SqlBogusStatement {
    syntax: SyntaxNode,
}
impl SqlBogusStatement {
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
impl AstNode for SqlBogusStatement {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_BOGUS_STATEMENT as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_BOGUS_STATEMENT
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
impl std::fmt::Debug for SqlBogusStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SqlBogusStatement")
            .field("items", &DebugSyntaxElementChildren(self.items()))
            .finish()
    }
}
impl From<SqlBogusStatement> for SyntaxNode {
    fn from(n: SqlBogusStatement) -> Self {
        n.syntax
    }
}
impl From<SqlBogusStatement> for SyntaxElement {
    fn from(n: SqlBogusStatement) -> Self {
        n.syntax.into()
    }
}
biome_rowan::declare_node_union! { pub AnySqlBogusNode = SqlBogus | SqlBogusAssignment | SqlBogusBinding | SqlBogusExpression | SqlBogusMember | SqlBogusParameter | SqlBogusStatement }
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlAliasColumnDefinitionList {
    syntax_list: SyntaxList,
}
impl SqlAliasColumnDefinitionList {
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
impl AstNode for SqlAliasColumnDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ALIAS_COLUMN_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ALIAS_COLUMN_DEFINITION_LIST
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
impl Serialize for SqlAliasColumnDefinitionList {
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
impl AstSeparatedList for SqlAliasColumnDefinitionList {
    type Language = Language;
    type Node = SqlAliasColumnDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlAliasColumnDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlAliasColumnDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlAliasColumnDefinitionList {
    type Item = SyntaxResult<SqlAliasColumnDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlAliasColumnDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlAliasColumnDefinitionList {
    type Item = SyntaxResult<SqlAliasColumnDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlAliasColumnDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlCaseWhenClauseList {
    syntax_list: SyntaxList,
}
impl SqlCaseWhenClauseList {
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
impl AstNode for SqlCaseWhenClauseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CASE_WHEN_CLAUSE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CASE_WHEN_CLAUSE_LIST
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
impl Serialize for SqlCaseWhenClauseList {
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
impl AstNodeList for SqlCaseWhenClauseList {
    type Language = Language;
    type Node = SqlCaseWhenClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlCaseWhenClauseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlCaseWhenClauseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlCaseWhenClauseList {
    type Item = SqlCaseWhenClause;
    type IntoIter = AstNodeListIterator<Language, SqlCaseWhenClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlCaseWhenClauseList {
    type Item = SqlCaseWhenClause;
    type IntoIter = AstNodeListIterator<Language, SqlCaseWhenClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlColumnDefinitionList {
    syntax_list: SyntaxList,
}
impl SqlColumnDefinitionList {
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
impl AstNode for SqlColumnDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_COLUMN_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_COLUMN_DEFINITION_LIST
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
impl Serialize for SqlColumnDefinitionList {
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
impl AstSeparatedList for SqlColumnDefinitionList {
    type Language = Language;
    type Node = SqlColumnDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlColumnDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlColumnDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlColumnDefinitionList {
    type Item = SyntaxResult<SqlColumnDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlColumnDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlColumnDefinitionList {
    type Item = SyntaxResult<SqlColumnDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlColumnDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlColumnNameList {
    syntax_list: SyntaxList,
}
impl SqlColumnNameList {
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
impl AstNode for SqlColumnNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_COLUMN_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_COLUMN_NAME_LIST
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
impl Serialize for SqlColumnNameList {
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
impl AstSeparatedList for SqlColumnNameList {
    type Language = Language;
    type Node = SqlName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlColumnNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlColumnNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlColumnNameList {
    type Item = SyntaxResult<SqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlColumnNameList {
    type Item = SyntaxResult<SqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlCteDefinitionList {
    syntax_list: SyntaxList,
}
impl SqlCteDefinitionList {
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
impl AstNode for SqlCteDefinitionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_CTE_DEFINITION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_CTE_DEFINITION_LIST
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
impl Serialize for SqlCteDefinitionList {
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
impl AstSeparatedList for SqlCteDefinitionList {
    type Language = Language;
    type Node = SqlCteDefinition;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlCteDefinitionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlCteDefinitionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlCteDefinitionList {
    type Item = SyntaxResult<SqlCteDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlCteDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlCteDefinitionList {
    type Item = SyntaxResult<SqlCteDefinition>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlCteDefinition>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlExpressionList {
    syntax_list: SyntaxList,
}
impl SqlExpressionList {
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
impl AstNode for SqlExpressionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_EXPRESSION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_EXPRESSION_LIST
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
impl Serialize for SqlExpressionList {
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
impl AstSeparatedList for SqlExpressionList {
    type Language = Language;
    type Node = AnySqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlExpressionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlExpressionList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlExpressionList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlFromItemList {
    syntax_list: SyntaxList,
}
impl SqlFromItemList {
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
impl AstNode for SqlFromItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FROM_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FROM_ITEM_LIST
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
impl Serialize for SqlFromItemList {
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
impl AstSeparatedList for SqlFromItemList {
    type Language = Language;
    type Node = SqlFromItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlFromItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlFromItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlFromItemList {
    type Item = SyntaxResult<SqlFromItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlFromItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlFromItemList {
    type Item = SyntaxResult<SqlFromItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlFromItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlFunctionOptionList {
    syntax_list: SyntaxList,
}
impl SqlFunctionOptionList {
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
impl AstNode for SqlFunctionOptionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FUNCTION_OPTION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FUNCTION_OPTION_LIST
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
impl Serialize for SqlFunctionOptionList {
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
impl AstNodeList for SqlFunctionOptionList {
    type Language = Language;
    type Node = AnySqlFunctionOption;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlFunctionOptionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlFunctionOptionList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlFunctionOptionList {
    type Item = AnySqlFunctionOption;
    type IntoIter = AstNodeListIterator<Language, AnySqlFunctionOption>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlFunctionOptionList {
    type Item = AnySqlFunctionOption;
    type IntoIter = AstNodeListIterator<Language, AnySqlFunctionOption>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlFunctionParameterList {
    syntax_list: SyntaxList,
}
impl SqlFunctionParameterList {
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
impl AstNode for SqlFunctionParameterList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_FUNCTION_PARAMETER_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_FUNCTION_PARAMETER_LIST
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
impl Serialize for SqlFunctionParameterList {
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
impl AstSeparatedList for SqlFunctionParameterList {
    type Language = Language;
    type Node = SqlFunctionParameter;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlFunctionParameterList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlFunctionParameterList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlFunctionParameterList {
    type Item = SyntaxResult<SqlFunctionParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlFunctionParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlFunctionParameterList {
    type Item = SyntaxResult<SqlFunctionParameter>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlFunctionParameter>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlGranteeList {
    syntax_list: SyntaxList,
}
impl SqlGranteeList {
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
impl AstNode for SqlGranteeList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_GRANTEE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_GRANTEE_LIST
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
impl Serialize for SqlGranteeList {
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
impl AstSeparatedList for SqlGranteeList {
    type Language = Language;
    type Node = SqlName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlGranteeList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlGranteeList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlGranteeList {
    type Item = SyntaxResult<SqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlGranteeList {
    type Item = SyntaxResult<SqlName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlGroupByItemList {
    syntax_list: SyntaxList,
}
impl SqlGroupByItemList {
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
impl AstNode for SqlGroupByItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_GROUP_BY_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_GROUP_BY_ITEM_LIST
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
impl Serialize for SqlGroupByItemList {
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
impl AstSeparatedList for SqlGroupByItemList {
    type Language = Language;
    type Node = AnySqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlGroupByItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlGroupByItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlGroupByItemList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlGroupByItemList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlJoinClauseList {
    syntax_list: SyntaxList,
}
impl SqlJoinClauseList {
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
impl AstNode for SqlJoinClauseList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_JOIN_CLAUSE_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_JOIN_CLAUSE_LIST
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
impl Serialize for SqlJoinClauseList {
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
impl AstNodeList for SqlJoinClauseList {
    type Language = Language;
    type Node = SqlJoinClause;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlJoinClauseList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlJoinClauseList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlJoinClauseList {
    type Item = SqlJoinClause;
    type IntoIter = AstNodeListIterator<Language, SqlJoinClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlJoinClauseList {
    type Item = SqlJoinClause;
    type IntoIter = AstNodeListIterator<Language, SqlJoinClause>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlOrderByExpressionList {
    syntax_list: SyntaxList,
}
impl SqlOrderByExpressionList {
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
impl AstNode for SqlOrderByExpressionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_ORDER_BY_EXPRESSION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_ORDER_BY_EXPRESSION_LIST
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
impl Serialize for SqlOrderByExpressionList {
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
impl AstSeparatedList for SqlOrderByExpressionList {
    type Language = Language;
    type Node = SqlOrderByExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlOrderByExpressionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlOrderByExpressionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlOrderByExpressionList {
    type Item = SyntaxResult<SqlOrderByExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlOrderByExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlOrderByExpressionList {
    type Item = SyntaxResult<SqlOrderByExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlOrderByExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlReturnsTableColumnList {
    syntax_list: SyntaxList,
}
impl SqlReturnsTableColumnList {
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
impl AstNode for SqlReturnsTableColumnList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_RETURNS_TABLE_COLUMN_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_RETURNS_TABLE_COLUMN_LIST
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
impl Serialize for SqlReturnsTableColumnList {
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
impl AstSeparatedList for SqlReturnsTableColumnList {
    type Language = Language;
    type Node = SqlReturnsTableColumn;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlReturnsTableColumnList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlReturnsTableColumnList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlReturnsTableColumnList {
    type Item = SyntaxResult<SqlReturnsTableColumn>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlReturnsTableColumn>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlReturnsTableColumnList {
    type Item = SyntaxResult<SqlReturnsTableColumn>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlReturnsTableColumn>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlSelectItemList {
    syntax_list: SyntaxList,
}
impl SqlSelectItemList {
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
impl AstNode for SqlSelectItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SELECT_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SELECT_ITEM_LIST
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
impl Serialize for SqlSelectItemList {
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
impl AstSeparatedList for SqlSelectItemList {
    type Language = Language;
    type Node = AnySqlSelectItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlSelectItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlSelectItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlSelectItemList {
    type Item = SyntaxResult<AnySqlSelectItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlSelectItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlSelectItemList {
    type Item = SyntaxResult<AnySqlSelectItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlSelectItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlSetItemList {
    syntax_list: SyntaxList,
}
impl SqlSetItemList {
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
impl AstNode for SqlSetItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SET_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SET_ITEM_LIST
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
impl Serialize for SqlSetItemList {
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
impl AstSeparatedList for SqlSetItemList {
    type Language = Language;
    type Node = SqlSetItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlSetItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlSetItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlSetItemList {
    type Item = SyntaxResult<SqlSetItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlSetItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlSetItemList {
    type Item = SyntaxResult<SqlSetItem>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlSetItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlSetOperationList {
    syntax_list: SyntaxList,
}
impl SqlSetOperationList {
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
impl AstNode for SqlSetOperationList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_SET_OPERATION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_SET_OPERATION_LIST
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
impl Serialize for SqlSetOperationList {
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
impl AstNodeList for SqlSetOperationList {
    type Language = Language;
    type Node = SqlSetOperation;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlSetOperationList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlSetOperationList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlSetOperationList {
    type Item = SqlSetOperation;
    type IntoIter = AstNodeListIterator<Language, SqlSetOperation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlSetOperationList {
    type Item = SqlSetOperation;
    type IntoIter = AstNodeListIterator<Language, SqlSetOperation>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlStatementList {
    syntax_list: SyntaxList,
}
impl SqlStatementList {
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
impl AstNode for SqlStatementList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_STATEMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_STATEMENT_LIST
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
impl Serialize for SqlStatementList {
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
impl AstNodeList for SqlStatementList {
    type Language = Language;
    type Node = AnySqlStatement;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlStatementList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlStatementList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlStatementList {
    type Item = AnySqlStatement;
    type IntoIter = AstNodeListIterator<Language, AnySqlStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlStatementList {
    type Item = AnySqlStatement;
    type IntoIter = AstNodeListIterator<Language, AnySqlStatement>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlTableNameList {
    syntax_list: SyntaxList,
}
impl SqlTableNameList {
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
impl AstNode for SqlTableNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TABLE_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TABLE_NAME_LIST
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
impl Serialize for SqlTableNameList {
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
impl AstSeparatedList for SqlTableNameList {
    type Language = Language;
    type Node = SqlTableName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlTableNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlTableNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlTableNameList {
    type Item = SyntaxResult<SqlTableName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlTableName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlTableNameList {
    type Item = SyntaxResult<SqlTableName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlTableName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlTriggerEventList {
    syntax_list: SyntaxList,
}
impl SqlTriggerEventList {
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
impl AstNode for SqlTriggerEventList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_EVENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_EVENT_LIST
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
impl Serialize for SqlTriggerEventList {
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
impl AstNodeList for SqlTriggerEventList {
    type Language = Language;
    type Node = SqlTriggerEvent;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlTriggerEventList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlTriggerEventList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlTriggerEventList {
    type Item = SqlTriggerEvent;
    type IntoIter = AstNodeListIterator<Language, SqlTriggerEvent>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlTriggerEventList {
    type Item = SqlTriggerEvent;
    type IntoIter = AstNodeListIterator<Language, SqlTriggerEvent>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlTriggerReferencingItemList {
    syntax_list: SyntaxList,
}
impl SqlTriggerReferencingItemList {
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
impl AstNode for SqlTriggerReferencingItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TRIGGER_REFERENCING_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TRIGGER_REFERENCING_ITEM_LIST
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
impl Serialize for SqlTriggerReferencingItemList {
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
impl AstNodeList for SqlTriggerReferencingItemList {
    type Language = Language;
    type Node = SqlTriggerReferencingItem;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlTriggerReferencingItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlTriggerReferencingItemList ")?;
        f.debug_list().entries(self.iter()).finish()
    }
}
impl IntoIterator for &SqlTriggerReferencingItemList {
    type Item = SqlTriggerReferencingItem;
    type IntoIter = AstNodeListIterator<Language, SqlTriggerReferencingItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for SqlTriggerReferencingItemList {
    type Item = SqlTriggerReferencingItem;
    type IntoIter = AstNodeListIterator<Language, SqlTriggerReferencingItem>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlTypeArgumentList {
    syntax_list: SyntaxList,
}
impl SqlTypeArgumentList {
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
impl AstNode for SqlTypeArgumentList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TYPE_ARGUMENT_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TYPE_ARGUMENT_LIST
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
impl Serialize for SqlTypeArgumentList {
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
impl AstSeparatedList for SqlTypeArgumentList {
    type Language = Language;
    type Node = SqlNumberLiteralExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlTypeArgumentList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlTypeArgumentList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlTypeArgumentList {
    type Item = SyntaxResult<SqlNumberLiteralExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlNumberLiteralExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlTypeArgumentList {
    type Item = SyntaxResult<SqlNumberLiteralExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlNumberLiteralExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlTypeNameList {
    syntax_list: SyntaxList,
}
impl SqlTypeNameList {
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
impl AstNode for SqlTypeNameList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_TYPE_NAME_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_TYPE_NAME_LIST
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
impl Serialize for SqlTypeNameList {
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
impl AstSeparatedList for SqlTypeNameList {
    type Language = Language;
    type Node = SqlTypeName;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlTypeNameList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlTypeNameList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlTypeNameList {
    type Item = SyntaxResult<SqlTypeName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlTypeName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlTypeNameList {
    type Item = SyntaxResult<SqlTypeName>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlTypeName>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlValuesRowList {
    syntax_list: SyntaxList,
}
impl SqlValuesRowList {
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
impl AstNode for SqlValuesRowList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VALUES_ROW_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VALUES_ROW_LIST
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
impl Serialize for SqlValuesRowList {
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
impl AstSeparatedList for SqlValuesRowList {
    type Language = Language;
    type Node = SqlValuesRow;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlValuesRowList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlValuesRowList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlValuesRowList {
    type Item = SyntaxResult<SqlValuesRow>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlValuesRow>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlValuesRowList {
    type Item = SyntaxResult<SqlValuesRow>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlValuesRow>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlViewOptionList {
    syntax_list: SyntaxList,
}
impl SqlViewOptionList {
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
impl AstNode for SqlViewOptionList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_VIEW_OPTION_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_VIEW_OPTION_LIST
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
impl Serialize for SqlViewOptionList {
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
impl AstSeparatedList for SqlViewOptionList {
    type Language = Language;
    type Node = SqlViewOption;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlViewOptionList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlViewOptionList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlViewOptionList {
    type Item = SyntaxResult<SqlViewOption>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlViewOption>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlViewOptionList {
    type Item = SyntaxResult<SqlViewOption>;
    type IntoIter = AstSeparatedListNodesIterator<Language, SqlViewOption>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
#[derive(Clone, Eq, PartialEq, Hash)]
pub struct SqlWindowPartitionByItemList {
    syntax_list: SyntaxList,
}
impl SqlWindowPartitionByItemList {
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
impl AstNode for SqlWindowPartitionByItemList {
    type Language = Language;
    const KIND_SET: SyntaxKindSet<Language> =
        SyntaxKindSet::from_raw(RawSyntaxKind(SQL_WINDOW_PARTITION_BY_ITEM_LIST as u16));
    fn can_cast(kind: SyntaxKind) -> bool {
        kind == SQL_WINDOW_PARTITION_BY_ITEM_LIST
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
impl Serialize for SqlWindowPartitionByItemList {
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
impl AstSeparatedList for SqlWindowPartitionByItemList {
    type Language = Language;
    type Node = AnySqlExpression;
    fn syntax_list(&self) -> &SyntaxList {
        &self.syntax_list
    }
    fn into_syntax_list(self) -> SyntaxList {
        self.syntax_list
    }
}
impl Debug for SqlWindowPartitionByItemList {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("SqlWindowPartitionByItemList ")?;
        f.debug_list().entries(self.elements()).finish()
    }
}
impl IntoIterator for SqlWindowPartitionByItemList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
impl IntoIterator for &SqlWindowPartitionByItemList {
    type Item = SyntaxResult<AnySqlExpression>;
    type IntoIter = AstSeparatedListNodesIterator<Language, AnySqlExpression>;
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
