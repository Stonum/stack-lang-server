//! Generated file, do not edit by hand, see `xtask/codegen`

#![allow(unused_mut)]
use biome_rowan::{
    AstNode, ParsedChildren, RawNodeSlots, RawSyntaxNode, SyntaxFactory, SyntaxKind,
};
use sql_syntax::{SqlSyntaxKind, SqlSyntaxKind::*, T, *};
#[derive(Debug)]
pub struct SqlSyntaxFactory;
impl SyntaxFactory for SqlSyntaxFactory {
    type Kind = SqlSyntaxKind;
    fn make_syntax(
        kind: Self::Kind,
        children: ParsedChildren<Self::Kind>,
    ) -> RawSyntaxNode<Self::Kind> {
        match kind {
            SQL_BOGUS | SQL_BOGUS_ASSIGNMENT | SQL_BOGUS_BINDING | SQL_BOGUS_EXPRESSION
            | SQL_BOGUS_MEMBER | SQL_BOGUS_PARAMETER | SQL_BOGUS_STATEMENT => {
                RawSyntaxNode::new(kind, children.into_iter().map(Some))
            }
            PSQL_ARRAY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![array]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ARRAY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ARRAY_EXPRESSION, children)
            }
            PSQL_ARRAY_SUBSCRIPT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ARRAY_SUBSCRIPT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ARRAY_SUBSCRIPT_EXPRESSION, children)
            }
            PSQL_CAST_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [::]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_CAST_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_CAST_EXPRESSION, children)
            }
            PSQL_CREATE_FUNCTION_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<14usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![create]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![or]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![replace]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![function] | T![procedure])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlFunctionParameterList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlReturnsClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlFunctionOptionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlStringLiteralExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlFunctionOptionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_CREATE_FUNCTION_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_CREATE_FUNCTION_STATEMENT, children)
            }
            PSQL_CREATE_POLICY_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![create]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![policy]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlPolicyForClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlPolicyUsingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlPolicyWithCheckClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_CREATE_POLICY_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_CREATE_POLICY_STATEMENT, children)
            }
            PSQL_CREATE_TRIGGER_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<14usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![create]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![trigger]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![before] | T![after])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerEventList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerReferencingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerForEachClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerWhenClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![execute]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![function] | T![procedure])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlCallExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_CREATE_TRIGGER_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_CREATE_TRIGGER_STATEMENT, children)
            }
            PSQL_CTE_MATERIALIZED_HINT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![materialized]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_CTE_MATERIALIZED_HINT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_CTE_MATERIALIZED_HINT, children)
            }
            PSQL_DELETE_USING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![using]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFromItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DELETE_USING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DELETE_USING_CLAUSE, children)
            }
            PSQL_DISTINCT_ON_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DISTINCT_ON_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DISTINCT_ON_CLAUSE, children)
            }
            PSQL_DO_NOTHING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![do]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![nothing]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DO_NOTHING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DO_NOTHING_CLAUSE, children)
            }
            PSQL_DO_UPDATE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![do]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![update]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSetClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWhereClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DO_UPDATE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DO_UPDATE_CLAUSE, children)
            }
            PSQL_DROP_FUNCTION_PARAMETERS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTypeNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DROP_FUNCTION_PARAMETERS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DROP_FUNCTION_PARAMETERS, children)
            }
            PSQL_DROP_POLICY_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![drop]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![policy]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DROP_POLICY_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DROP_POLICY_STATEMENT, children)
            }
            PSQL_DROP_TRIGGER_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![drop]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![trigger]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![cascade] | T![restrict])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_DROP_TRIGGER_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_DROP_TRIGGER_STATEMENT, children)
            }
            PSQL_FILTER_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![filter]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![where]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_FILTER_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_FILTER_CLAUSE, children)
            }
            PSQL_FUNCTION_PARAMETER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![in] | T![out] | T![inout])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlParameterDefault::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_FUNCTION_PARAMETER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_FUNCTION_PARAMETER, children)
            }
            PSQL_INTERVAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![interval]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlStringLiteralExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_INTERVAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_INTERVAL_EXPRESSION, children)
            }
            PSQL_JOIN_USING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![using]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_JOIN_USING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_JOIN_USING_CLAUSE, children)
            }
            PSQL_LANGUAGE_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![language]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_LANGUAGE_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_LANGUAGE_OPTION, children)
            }
            PSQL_LIMIT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![limit]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlLimitValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_LIMIT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_LIMIT_CLAUSE, children)
            }
            PSQL_ON_CONFLICT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![conflict]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlConflictTarget::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlConflictAction::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ON_CONFLICT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ON_CONFLICT_CLAUSE, children)
            }
            PSQL_ON_CONSTRAINT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![constraint]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_ON_CONSTRAINT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_ON_CONSTRAINT_CLAUSE, children)
            }
            PSQL_PARAMETER_DEFAULT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![default] | T ! [=])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_PARAMETER_DEFAULT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_PARAMETER_DEFAULT, children)
            }
            PSQL_POLICY_FOR_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![for]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        T![all] | T![select] | T![insert] | T![update] | T![delete]
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_POLICY_FOR_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_POLICY_FOR_CLAUSE, children)
            }
            PSQL_POLICY_USING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![using]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_POLICY_USING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_POLICY_USING_CLAUSE, children)
            }
            PSQL_POLICY_WITH_CHECK_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![with]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![check]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_POLICY_WITH_CHECK_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_POLICY_WITH_CHECK_CLAUSE, children)
            }
            PSQL_RETURNING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![returning]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSelectItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNING_CLAUSE, children)
            }
            PSQL_RETURNS_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![returns]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlReturnsType::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_CLAUSE, children)
            }
            PSQL_RETURNS_NULL_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![returns]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![null]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![null]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![input]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_NULL_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_NULL_OPTION, children)
            }
            PSQL_RETURNS_SETOF_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![setof]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_SETOF_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_SETOF_CLAUSE, children)
            }
            PSQL_RETURNS_TABLE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![table]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlReturnsTableColumnList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_TABLE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_TABLE_CLAUSE, children)
            }
            PSQL_RETURNS_TABLE_COLUMN => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_TABLE_COLUMN.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_TABLE_COLUMN, children)
            }
            PSQL_RETURNS_TRIGGER_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![trigger]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_RETURNS_TRIGGER_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_RETURNS_TRIGGER_CLAUSE, children)
            }
            PSQL_SECURITY_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![security]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![definer] | T![invoker])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SECURITY_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SECURITY_OPTION, children)
            }
            PSQL_STRICT_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![strict]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_STRICT_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_STRICT_OPTION, children)
            }
            PSQL_SUBSTRING_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == IDENT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlSubstringFromClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlSubstringForClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SUBSTRING_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SUBSTRING_EXPRESSION, children)
            }
            PSQL_SUBSTRING_FOR_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![for]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SUBSTRING_FOR_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SUBSTRING_FOR_CLAUSE, children)
            }
            PSQL_SUBSTRING_FROM_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![from]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_SUBSTRING_FROM_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_SUBSTRING_FROM_CLAUSE, children)
            }
            PSQL_TILDE_ARRAY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![array]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [~]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [~]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TILDE_ARRAY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TILDE_ARRAY_EXPRESSION, children)
            }
            PSQL_TILDE_ARRAY_SUFFIX => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [~]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [~]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TILDE_ARRAY_SUFFIX.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TILDE_ARRAY_SUFFIX, children)
            }
            PSQL_TRIGGER_EVENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![or]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![insert] | T![update] | T![delete])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerUpdateOfClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_EVENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_EVENT, children)
            }
            PSQL_TRIGGER_FOR_EACH_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![for]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![each]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![row] | T![statement])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_FOR_EACH_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_FOR_EACH_CLAUSE, children)
            }
            PSQL_TRIGGER_REFERENCING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![referencing]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlTriggerReferencingItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_REFERENCING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_REFERENCING_CLAUSE, children)
            }
            PSQL_TRIGGER_REFERENCING_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == IDENT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![table]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_REFERENCING_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_REFERENCING_ITEM, children)
            }
            PSQL_TRIGGER_UPDATE_OF_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![of]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_UPDATE_OF_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_UPDATE_OF_CLAUSE, children)
            }
            PSQL_TRIGGER_WHEN_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![when]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TRIGGER_WHEN_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TRIGGER_WHEN_CLAUSE, children)
            }
            PSQL_TYPE_ARRAY_SUFFIX => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['[']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![']']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_TYPE_ARRAY_SUFFIX.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_TYPE_ARRAY_SUFFIX, children)
            }
            PSQL_VIEW_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [=]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_VIEW_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_VIEW_OPTION, children)
            }
            PSQL_VIEW_OPTIONS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![with]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlViewOptionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_VIEW_OPTIONS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_VIEW_OPTIONS, children)
            }
            PSQL_VOLATILITY_OPTION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![immutable] | T![stable] | T![volatile])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        PSQL_VOLATILITY_OPTION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(PSQL_VOLATILITY_OPTION, children)
            }
            SQL_ALIAS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAliasColumnList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ALIAS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ALIAS, children)
            }
            SQL_ALIAS_COLUMN_DEFINITION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ALIAS_COLUMN_DEFINITION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ALIAS_COLUMN_DEFINITION, children)
            }
            SQL_ALIAS_COLUMN_LIST => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAliasColumnDefinitionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ALIAS_COLUMN_LIST.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ALIAS_COLUMN_LIST, children)
            }
            SQL_ANY_ALL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![any] | T![all] | T![some])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlAnyAllSource::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ANY_ALL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ANY_ALL_EXPRESSION, children)
            }
            SQL_BETWEEN_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![between]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![and]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_BETWEEN_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_BETWEEN_EXPRESSION, children)
            }
            SQL_BINARY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        T ! [<]
                            | T ! [>]
                            | T ! [<=]
                            | T ! [>=]
                            | T ! [=]
                            | T ! [!=]
                            | T ! [<>]
                            | T ! [+]
                            | T ! [-]
                            | T ! [*]
                            | T ! [/]
                            | T ! [%]
                            | T ! [~]
                            | T ! [!~]
                            | T ! [&]
                            | T ! [|]
                            | T ! [^]
                            | T ! [>>]
                            | T ! [<<]
                            | T ! [~*]
                            | T ! [!~*]
                            | T ! [~~]
                            | T ! [~~*]
                            | T ! [!~~]
                            | T ! [!~~*]
                            | T ! [||]
                            | T ! [->]
                            | T ! [->>]
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_BINARY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_BINARY_EXPRESSION, children)
            }
            SQL_BOOLEAN_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![true] | T![false])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_BOOLEAN_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_BOOLEAN_LITERAL_EXPRESSION, children)
            }
            SQL_CALL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlShemaName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlFilterClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CALL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CALL_EXPRESSION, children)
            }
            SQL_CASE_ELSE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![else]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CASE_ELSE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CASE_ELSE_CLAUSE, children)
            }
            SQL_CASE_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![case]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlCaseWhenClauseList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlCaseElseClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![end]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CASE_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CASE_EXPRESSION, children)
            }
            SQL_CASE_WHEN_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![when]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![then]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CASE_WHEN_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CASE_WHEN_CLAUSE, children)
            }
            SQL_CAST_FUNCTION_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<6usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![cast]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CAST_FUNCTION_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CAST_FUNCTION_EXPRESSION, children)
            }
            SQL_COL_REFERENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_COL_REFERENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_COL_REFERENCE, children)
            }
            SQL_COLUMN_DEFINITION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_COLUMN_DEFINITION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_COLUMN_DEFINITION, children)
            }
            SQL_COLUMN_LIST => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_COLUMN_LIST.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_COLUMN_LIST, children)
            }
            SQL_CREATE_TABLE_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<10usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![create]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![table]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnDefinitionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CREATE_TABLE_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CREATE_TABLE_STATEMENT, children)
            }
            SQL_CREATE_VIEW_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![create]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![or]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![replace]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![view]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlViewOptions::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSelectStatement::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CREATE_VIEW_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CREATE_VIEW_STATEMENT, children)
            }
            SQL_CTE_DEFINITION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![as]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlCteMaterializedHint::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlStatement::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_CTE_DEFINITION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_CTE_DEFINITION, children)
            }
            SQL_DATA_BASE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_DATA_BASE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_DATA_BASE_NAME, children)
            }
            SQL_DELETE_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlWithClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![delete]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![from]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableBinding::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlDeleteUsingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWhereClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlReturningClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_DELETE_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_DELETE_STATEMENT, children)
            }
            SQL_DROP_FUNCTION_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![drop]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![function] | T![procedure])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlDropFunctionParameters::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![cascade] | T![restrict])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_DROP_FUNCTION_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_DROP_FUNCTION_STATEMENT, children)
            }
            SQL_DROP_TABLE_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![drop]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![table]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![cascade] | T![restrict])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_DROP_TABLE_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_DROP_TABLE_STATEMENT, children)
            }
            SQL_DROP_VIEW_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![drop]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![view]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![if]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![cascade] | T![restrict])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_DROP_VIEW_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_DROP_VIEW_STATEMENT, children)
            }
            SQL_EMPTY_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_EMPTY_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_EMPTY_STATEMENT, children)
            }
            SQL_EXISTS_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![exists]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSubqueryExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_EXISTS_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_EXISTS_EXPRESSION, children)
            }
            SQL_FETCH_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![fetch]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![first] | T![next])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlLimitValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![row] | T![rows])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlFetchTail::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FETCH_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FETCH_CLAUSE, children)
            }
            SQL_FETCH_ONLY_TAIL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![only]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FETCH_ONLY_TAIL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FETCH_ONLY_TAIL, children)
            }
            SQL_FETCH_WITH_TIES_TAIL => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![with]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![ties]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FETCH_WITH_TIES_TAIL.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FETCH_WITH_TIES_TAIL, children)
            }
            SQL_FROM_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![from]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFromItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FROM_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FROM_CLAUSE, children)
            }
            SQL_FROM_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlFromExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlJoinClauseList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FROM_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FROM_ITEM, children)
            }
            SQL_FUNCTION_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![lateral]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlShemaName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAlias::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_FUNCTION_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_FUNCTION_BINDING, children)
            }
            SQL_GRANT_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![grant]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![all]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![table]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableNameList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![to]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlGranteeList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_GRANT_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_GRANT_STATEMENT, children)
            }
            SQL_GROUP_BY_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![group_by]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlGroupByItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_GROUP_BY_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_GROUP_BY_CLAUSE, children)
            }
            SQL_HAVING_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![having]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_HAVING_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_HAVING_CLAUSE, children)
            }
            SQL_IN_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![in]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlInSource::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_IN_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_IN_EXPRESSION, children)
            }
            SQL_IN_VALUE_LIST => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_IN_VALUE_LIST.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_IN_VALUE_LIST, children)
            }
            SQL_INSERT_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<9usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlWithClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![insert]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![into]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableBinding::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlColumnList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlInsertSource::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlOnConflictClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlReturningClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_INSERT_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_INSERT_STATEMENT, children)
            }
            SQL_IS_NULL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![is]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![null]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_IS_NULL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_IS_NULL_EXPRESSION, children)
            }
            SQL_JOIN_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        T![inner] | T![left] | T![right] | T![full] | T![cross]
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![outer]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![join]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlFromExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![on]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlJoinUsingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_JOIN_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_JOIN_CLAUSE, children)
            }
            SQL_LIKE_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![not]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![like] | T![ilike])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_LIKE_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_LIKE_EXPRESSION, children)
            }
            SQL_LOGICAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![and] | T![or])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_LOGICAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_LOGICAL_EXPRESSION, children)
            }
            SQL_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == IDENT
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(SQL_NAME.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(SQL_NAME, children)
            }
            SQL_NULL_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![null]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_NULL_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_NULL_LITERAL_EXPRESSION, children)
            }
            SQL_NUMBER_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == SQL_NUMBER_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_NUMBER_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_NUMBER_LITERAL_EXPRESSION, children)
            }
            SQL_OFFSET_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![offset]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlLimitValue::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_OFFSET_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_OFFSET_CLAUSE, children)
            }
            SQL_ORDER_BY_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![order_by]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlOrderByExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ORDER_BY_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ORDER_BY_CLAUSE, children)
            }
            SQL_ORDER_BY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![asc] | T![desc])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_ORDER_BY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_ORDER_BY_EXPRESSION, children)
            }
            SQL_PARAMETER_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [:]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), IDENT | SQL_NUMBER_LITERAL)
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_PARAMETER_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_PARAMETER_EXPRESSION, children)
            }
            SQL_PARENTHESIZED_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_PARENTHESIZED_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_PARENTHESIZED_EXPRESSION, children)
            }
            SQL_PARENTHESIZED_JOIN_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlFromExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlJoinClauseList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAlias::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_PARENTHESIZED_JOIN_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_PARENTHESIZED_JOIN_BINDING, children)
            }
            SQL_PRECISION_MODIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![precision]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_PRECISION_MODIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_PRECISION_MODIFIER, children)
            }
            SQL_ROOT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![UNICODE_BOM]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlStatementList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![EOF]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(SQL_ROOT.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(SQL_ROOT, children)
            }
            SQL_SELECT_ALL_QUANTIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![all]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SELECT_ALL_QUANTIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SELECT_ALL_QUANTIFIER, children)
            }
            SQL_SELECT_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![select]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlSelectQuantifier::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSelectItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SELECT_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SELECT_CLAUSE, children)
            }
            SQL_SELECT_DISTINCT_QUANTIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![distinct]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlDistinctOnClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SELECT_DISTINCT_QUANTIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SELECT_DISTINCT_QUANTIFIER, children)
            }
            SQL_SELECT_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAlias::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SELECT_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SELECT_EXPRESSION, children)
            }
            SQL_SELECT_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<12usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlWithClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSelectClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFromClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWhereClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlGroupByClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlHavingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSetOperationList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlOrderByClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlLimitClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlOffsetClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFetchClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SELECT_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SELECT_STATEMENT, children)
            }
            SQL_SET_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![set]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSetItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SET_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SET_CLAUSE, children)
            }
            SQL_SET_ITEM => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [=]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SET_ITEM.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SET_ITEM, children)
            }
            SQL_SET_OPERATION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<7usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![union] | T![intersect] | T![except])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![all] | T![distinct])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSelectClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFromClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWhereClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlGroupByClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlHavingClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SET_OPERATION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SET_OPERATION, children)
            }
            SQL_SHEMA_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlDataBaseName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SHEMA_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SHEMA_NAME, children)
            }
            SQL_STAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T ! [*]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(SQL_STAR.to_bogus(), children.into_iter().map(Some));
                }
                slots.into_node(SQL_STAR, children)
            }
            SQL_STRING_LITERAL_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == SQL_STRING_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_STRING_LITERAL_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_STRING_LITERAL_EXPRESSION, children)
            }
            SQL_SUBQUERY_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<5usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![lateral]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlSubqueryBody::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAlias::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SUBQUERY_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SUBQUERY_BINDING, children)
            }
            SQL_SUBQUERY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlSubqueryBody::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_SUBQUERY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_SUBQUERY_EXPRESSION, children)
            }
            SQL_TABLE_BINDING => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlAlias::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TABLE_BINDING.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TABLE_BINDING, children)
            }
            SQL_TABLE_COL_REFERENCE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TABLE_COL_REFERENCE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TABLE_COL_REFERENCE, children)
            }
            SQL_TABLE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlShemaName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TABLE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TABLE_NAME, children)
            }
            SQL_TABLE_STAR => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlTableName::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [.]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlStar::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TABLE_STAR.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TABLE_STAR, children)
            }
            SQL_TILDE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == SQL_TILDE_NAME_LITERAL
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TILDE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TILDE_NAME, children)
            }
            SQL_TIME_ZONE_MODIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![with] | T![without])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![time]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![zone]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TIME_ZONE_MODIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TIME_ZONE_MODIFIER, children)
            }
            SQL_TYPE_ARGUMENTS => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeArgumentList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TYPE_ARGUMENTS.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TYPE_ARGUMENTS, children)
            }
            SQL_TYPE_NAME => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(
                        element.kind(),
                        IDENT
                            | T![integer]
                            | T![bigint]
                            | T![varchar]
                            | T![char]
                            | T![text]
                            | T![boolean]
                            | T![date]
                            | T![time]
                            | T![timestamp]
                            | T![interval]
                            | T![numeric]
                            | T![decimal]
                            | T![double]
                            | T![real]
                            | T![json]
                            | T![jsonb]
                            | T![uuid]
                            | T![array]
                            | T![bytea]
                            | T![bit]
                    )
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTypeArguments::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlTypeModifier::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlTypeArraySuffix::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_TYPE_NAME.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_TYPE_NAME, children)
            }
            SQL_UNARY_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && matches!(element.kind(), T![not] | T ! [-] | T ! [+])
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_UNARY_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_UNARY_EXPRESSION, children)
            }
            SQL_UPDATE_FROM_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![from]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlFromItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_UPDATE_FROM_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_UPDATE_FROM_CLAUSE, children)
            }
            SQL_UPDATE_STATEMENT => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<8usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlWithClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![update]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlTableBinding::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlSetClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlUpdateFromClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWhereClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && PsqlReturningClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_UPDATE_STATEMENT.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_UPDATE_STATEMENT, children)
            }
            SQL_VALUES_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlWithClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![values]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlValuesRowList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T ! [;]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_VALUES_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_VALUES_CLAUSE, children)
            }
            SQL_VALUES_ROW => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlExpressionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_VALUES_ROW.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_VALUES_ROW, children)
            }
            SQL_VARYING_MODIFIER => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<1usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![varying]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_VARYING_MODIFIER.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_VARYING_MODIFIER, children)
            }
            SQL_WHERE_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![where]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && AnySqlExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_WHERE_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_WHERE_CLAUSE, children)
            }
            SQL_WINDOW_FUNCTION_EXPRESSION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && SqlCallExpression::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![over]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWindowSpecification::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_WINDOW_FUNCTION_EXPRESSION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_WINDOW_FUNCTION_EXPRESSION, children)
            }
            SQL_WINDOW_PARTITION_BY_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<2usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![partition_by]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWindowPartitionByItemList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_WINDOW_PARTITION_BY_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_WINDOW_PARTITION_BY_CLAUSE, children)
            }
            SQL_WINDOW_SPECIFICATION => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<4usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T!['(']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlWindowPartitionByClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlOrderByClause::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![')']
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_WINDOW_SPECIFICATION.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_WINDOW_SPECIFICATION, children)
            }
            SQL_WITH_CLAUSE => {
                let mut elements = (&children).into_iter();
                let mut slots: RawNodeSlots<3usize> = RawNodeSlots::default();
                let mut current_element = elements.next();
                if let Some(element) = &current_element
                    && element.kind() == T![with]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && element.kind() == T![recursive]
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if let Some(element) = &current_element
                    && SqlCteDefinitionList::can_cast(element.kind())
                {
                    slots.mark_present();
                    current_element = elements.next();
                }
                slots.next_slot();
                if current_element.is_some() {
                    return RawSyntaxNode::new(
                        SQL_WITH_CLAUSE.to_bogus(),
                        children.into_iter().map(Some),
                    );
                }
                slots.into_node(SQL_WITH_CLAUSE, children)
            }
            PSQL_FUNCTION_OPTION_LIST => {
                Self::make_node_list_syntax(kind, children, AnySqlFunctionOption::can_cast)
            }
            PSQL_FUNCTION_PARAMETER_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                PsqlFunctionParameter::can_cast,
                T ! [,],
                false,
            ),
            PSQL_RETURNS_TABLE_COLUMN_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                PsqlReturnsTableColumn::can_cast,
                T ! [,],
                false,
            ),
            PSQL_TRIGGER_EVENT_LIST => {
                Self::make_node_list_syntax(kind, children, PsqlTriggerEvent::can_cast)
            }
            PSQL_TRIGGER_REFERENCING_ITEM_LIST => {
                Self::make_node_list_syntax(kind, children, PsqlTriggerReferencingItem::can_cast)
            }
            PSQL_TYPE_NAME_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlTypeName::can_cast,
                T ! [,],
                false,
            ),
            PSQL_VIEW_OPTION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                PsqlViewOption::can_cast,
                T ! [,],
                false,
            ),
            SQL_ALIAS_COLUMN_DEFINITION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlAliasColumnDefinition::can_cast,
                T ! [,],
                false,
            ),
            SQL_CASE_WHEN_CLAUSE_LIST => {
                Self::make_node_list_syntax(kind, children, SqlCaseWhenClause::can_cast)
            }
            SQL_COLUMN_DEFINITION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlColumnDefinition::can_cast,
                T ! [,],
                false,
            ),
            SQL_COLUMN_NAME_LIST => {
                Self::make_separated_list_syntax(kind, children, SqlName::can_cast, T ! [,], false)
            }
            SQL_CTE_DEFINITION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlCteDefinition::can_cast,
                T ! [,],
                false,
            ),
            SQL_EXPRESSION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnySqlExpression::can_cast,
                T ! [,],
                false,
            ),
            SQL_FROM_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlFromItem::can_cast,
                T ! [,],
                false,
            ),
            SQL_GRANTEE_LIST => {
                Self::make_separated_list_syntax(kind, children, SqlName::can_cast, T ! [,], false)
            }
            SQL_GROUP_BY_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnySqlExpression::can_cast,
                T ! [,],
                true,
            ),
            SQL_JOIN_CLAUSE_LIST => {
                Self::make_node_list_syntax(kind, children, SqlJoinClause::can_cast)
            }
            SQL_ORDER_BY_EXPRESSION_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlOrderByExpression::can_cast,
                T ! [,],
                false,
            ),
            SQL_SELECT_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnySqlSelectItem::can_cast,
                T ! [,],
                true,
            ),
            SQL_SET_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlSetItem::can_cast,
                T ! [,],
                false,
            ),
            SQL_SET_OPERATION_LIST => {
                Self::make_node_list_syntax(kind, children, SqlSetOperation::can_cast)
            }
            SQL_STATEMENT_LIST => {
                Self::make_node_list_syntax(kind, children, AnySqlStatement::can_cast)
            }
            SQL_TABLE_NAME_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlTableName::can_cast,
                T ! [,],
                false,
            ),
            SQL_TYPE_ARGUMENT_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlNumberLiteralExpression::can_cast,
                T ! [,],
                false,
            ),
            SQL_VALUES_ROW_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                SqlValuesRow::can_cast,
                T ! [,],
                false,
            ),
            SQL_WINDOW_PARTITION_BY_ITEM_LIST => Self::make_separated_list_syntax(
                kind,
                children,
                AnySqlExpression::can_cast,
                T ! [,],
                false,
            ),
            _ => unreachable!("Is {:?} a token?", kind),
        }
    }
}
