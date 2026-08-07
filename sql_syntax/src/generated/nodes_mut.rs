//! Generated file, do not edit by hand, see `xtask/codegen`

use crate::{SqlSyntaxToken as SyntaxToken, generated::nodes::*};
use biome_rowan::AstNode;
use std::iter::once;
impl PsqlArrayExpression {
    pub fn with_array_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlArraySubscriptExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_index(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlCastExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_double_colon_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlCreateFunctionStatement {
    pub fn with_create_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_or_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_replace_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_kind_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_parameters(self, element: PsqlFunctionParameterList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(Some(element.into()))),
        )
    }
    pub fn with_returns_clause(self, element: Option<PsqlReturnsClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            8usize..=8usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_leading_options(self, element: PsqlFunctionOptionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(9usize..=9usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_as_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(10usize..=10usize, once(Some(element.into()))),
        )
    }
    pub fn with_body(self, element: SqlStringLiteralExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(11usize..=11usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_trailing_options(self, element: PsqlFunctionOptionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(12usize..=12usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            13usize..=13usize,
            once(element.map(|element| element.into())),
        ))
    }
}
impl PsqlCreatePolicyStatement {
    pub fn with_create_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_policy_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_for_clause(self, element: Option<PsqlPolicyForClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_using_clause(self, element: Option<PsqlPolicyUsingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_with_check_clause(self, element: Option<PsqlPolicyWithCheckClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            7usize..=7usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlCreateTriggerStatement {
    pub fn with_create_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_trigger_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_timing_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_events(self, element: PsqlTriggerEventList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_referencing_clause(self, element: Option<PsqlTriggerReferencingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            7usize..=7usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_for_each_clause(self, element: Option<PsqlTriggerForEachClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            8usize..=8usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_when_clause(self, element: Option<PsqlTriggerWhenClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            9usize..=9usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_execute_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(10usize..=10usize, once(Some(element.into()))),
        )
    }
    pub fn with_function_kind_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(11usize..=11usize, once(Some(element.into()))),
        )
    }
    pub fn with_function(self, element: SqlCallExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(12usize..=12usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            13usize..=13usize,
            once(element.map(|element| element.into())),
        ))
    }
}
impl PsqlCteMaterializedHint {
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_materialized_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlDeleteUsingClause {
    pub fn with_using_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlFromItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlDistinctOnClause {
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlDoNothingClause {
    pub fn with_do_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_nothing_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlDoUpdateClause {
    pub fn with_do_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_update_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_set_clause(self, element: SqlSetClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_where_clause(self, element: Option<SqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlDropFunctionParameters {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlTypeNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl PsqlDropPolicyStatement {
    pub fn with_drop_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_policy_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlDropTriggerStatement {
    pub fn with_drop_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_trigger_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_drop_behavior_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(element.map(|element| element.into()))),
        )
    }
}
impl PsqlFilterClause {
    pub fn with_filter_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_where_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
}
impl PsqlFunctionParameter {
    pub fn with_mode_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_name(self, element: Option<SqlName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_default(self, element: Option<PsqlParameterDefault>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlIntervalExpression {
    pub fn with_interval_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: SqlStringLiteralExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlJoinUsingClause {
    pub fn with_using_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_columns(self, element: SqlColumnList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlLanguageOption {
    pub fn with_language_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlLimitClause {
    pub fn with_limit_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_limit_count(self, element: AnySqlLimitValue) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlOnConflictClause {
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_conflict_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_target(self, element: Option<AnySqlConflictTarget>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_action(self, element: AnySqlConflictAction) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlOnConstraintClause {
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_constraint_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlParameterDefault {
    pub fn with_marker_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlPolicyForClause {
    pub fn with_for_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_command_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlPolicyUsingClause {
    pub fn with_using_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlPolicyWithCheckClause {
    pub fn with_with_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_check_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
}
impl PsqlReturningClause {
    pub fn with_returning_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlSelectItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlReturnsClause {
    pub fn with_returns_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_ty(self, element: AnySqlReturnsType) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlReturnsNullOption {
    pub fn with_returns_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_first_null_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_second_null_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_input_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
}
impl PsqlReturnsSetofClause {
    pub fn with_setof_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlReturnsTableClause {
    pub fn with_table_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_columns(self, element: PsqlReturnsTableColumnList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlReturnsTableColumn {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlReturnsTriggerClause {
    pub fn with_trigger_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlSecurityOption {
    pub fn with_security_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlStrictOption {
    pub fn with_strict_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl PsqlSubstringExpression {
    pub fn with_name_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_from_clause(self, element: Option<PsqlSubstringFromClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_for_clause(self, element: Option<PsqlSubstringForClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
}
impl PsqlSubstringForClause {
    pub fn with_for_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlSubstringFromClause {
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlTildeArrayExpression {
    pub fn with_array_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_open_tilde_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
    pub fn with_close_tilde_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
}
impl PsqlTildeArraySuffix {
    pub fn with_open_tilde_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_close_tilde_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlTriggerEvent {
    pub fn with_or_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_kind_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_of_clause(self, element: Option<PsqlTriggerUpdateOfClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl PsqlTriggerForEachClause {
    pub fn with_for_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_each_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_granularity_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl PsqlTriggerReferencingClause {
    pub fn with_referencing_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlTriggerReferencingItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlTriggerReferencingItem {
    pub fn with_which_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_table_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_as_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlTriggerUpdateOfClause {
    pub fn with_of_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_columns(self, element: SqlColumnNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlTriggerWhenClause {
    pub fn with_when_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlTypeArraySuffix {
    pub fn with_l_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_r_brack_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl PsqlViewOption {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eq_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_value(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl PsqlViewOptions {
    pub fn with_with_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: PsqlViewOptionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl PsqlVolatilityOption {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlAlias {
    pub fn with_as_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_value(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_columns(self, element: Option<SqlAliasColumnList>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlAliasColumnDefinition {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_ty(self, element: Option<SqlTypeName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlAliasColumnList {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlAliasColumnDefinitionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlAnyAllExpression {
    pub fn with_quantifier_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_source(self, element: AnySqlAnyAllSource) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlBetweenExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_between_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_low(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_and_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
    pub fn with_high(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlBinaryExpression {
    pub fn with_left(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_right(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlBooleanLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlCallExpression {
    pub fn with_schema(self, element: Option<SqlShemaName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_arguments(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
    pub fn with_filter_clause(self, element: Option<PsqlFilterClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlCaseElseClause {
    pub fn with_else_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_result(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlCaseExpression {
    pub fn with_case_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: Option<AnySqlExpression>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_when_clauses(self, element: SqlCaseWhenClauseList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_else_clause(self, element: Option<SqlCaseElseClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_end_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
}
impl SqlCaseWhenClause {
    pub fn with_when_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_then_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_result(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlCastFunctionExpression {
    pub fn with_cast_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_as_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
}
impl SqlColReference {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlColumnDefinition {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_ty(self, element: SqlTypeName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlColumnList {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlColumnNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlCreateTableStatement {
    pub fn with_create_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_table_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_name(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into()))),
        )
    }
    pub fn with_columns(self, element: SqlColumnDefinitionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(Some(element.into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(9usize..=9usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlCreateViewStatement {
    pub fn with_create_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_or_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_replace_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_view_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_options(self, element: Option<PsqlViewOptions>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_as_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into()))),
        )
    }
    pub fn with_query(self, element: SqlSelectStatement) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlCteDefinition {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_columns(self, element: Option<SqlColumnList>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_as_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_materialized(self, element: Option<PsqlCteMaterializedHint>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into()))),
        )
    }
    pub fn with_query(self, element: AnySqlStatement) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into()))),
        )
    }
}
impl SqlDataBaseName {
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl SqlDeleteStatement {
    pub fn with_with_clause(self, element: Option<SqlWithClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_delete_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_using(self, element: Option<PsqlDeleteUsingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<SqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_returning_clause(self, element: Option<PsqlReturningClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlDropFunctionStatement {
    pub fn with_drop_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_kind_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_parameters(self, element: Option<PsqlDropFunctionParameters>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_drop_behavior_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlDropTableStatement {
    pub fn with_drop_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_table_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_tables(self, element: SqlTableNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_drop_behavior_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlDropViewStatement {
    pub fn with_drop_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_view_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_if_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_exists_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_views(self, element: SqlTableNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_drop_behavior_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlEmptyStatement {
    pub fn with_semicolon_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlExistsExpression {
    pub fn with_exists_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_subquery(self, element: SqlSubqueryExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlFetchClause {
    pub fn with_fetch_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_quantifier_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_count(self, element: Option<AnySqlLimitValue>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_row_or_rows_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_tail(self, element: AnySqlFetchTail) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlFetchOnlyTail {
    pub fn with_only_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlFetchWithTiesTail {
    pub fn with_with_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_ties_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl SqlFromClause {
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlFromItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlFromItem {
    pub fn with_source(self, element: AnySqlFromExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_joins(self, element: SqlJoinClauseList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlFunctionBinding {
    pub fn with_lateral_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_schema(self, element: Option<SqlShemaName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_arguments(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_alias(self, element: Option<SqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlGrantStatement {
    pub fn with_grant_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_all_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_on_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_table_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_objects(self, element: SqlTableNameList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_to_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into()))),
        )
    }
    pub fn with_grantees(self, element: SqlGranteeList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlGroupByClause {
    pub fn with_group_by_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlGroupByItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlHavingClause {
    pub fn with_having_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlInExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_in_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_source(self, element: AnySqlInSource) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlInValueList {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlInsertStatement {
    pub fn with_with_clause(self, element: Option<SqlWithClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_insert_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_into_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_columns(self, element: Option<SqlColumnList>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_source(self, element: AnySqlInsertSource) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(5usize..=5usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_conflict_clause(self, element: Option<PsqlOnConflictClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_returning_clause(self, element: Option<PsqlReturningClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            7usize..=7usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(8usize..=8usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlIsNullExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_is_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_null_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl SqlJoinClause {
    pub fn with_join_type_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_outer_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_join_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_source(self, element: AnySqlFromExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_on_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(4usize..=4usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_condition(self, element: Option<AnySqlExpression>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_using_clause(self, element: Option<PsqlJoinUsingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlLikeExpression {
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_not_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
    pub fn with_pattern(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlLogicalExpression {
    pub fn with_left(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_right(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlName {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlNullLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlNumberLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlOffsetClause {
    pub fn with_offset_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_start(self, element: AnySqlLimitValue) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlOrderByClause {
    pub fn with_order_by_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlOrderByExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlOrderByExpression {
    pub fn with_item(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_order_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlParameterExpression {
    pub fn with_colon_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_name_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
}
impl SqlParenthesizedExpression {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlParenthesizedJoinBinding {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_source(self, element: AnySqlFromExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_joins(self, element: SqlJoinClauseList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_alias(self, element: Option<SqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlPrecisionModifier {
    pub fn with_precision_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlRoot {
    pub fn with_bom_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_stmt(self, element: SqlStatementList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eof_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlSelectAllQuantifier {
    pub fn with_all_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlSelectClause {
    pub fn with_select_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_quantifier(self, element: Option<AnySqlSelectQuantifier>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_list(self, element: SqlSelectItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlSelectDistinctQuantifier {
    pub fn with_distinct_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_on_clause(self, element: Option<PsqlDistinctOnClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlSelectExpression {
    pub fn with_expr(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_alias(self, element: Option<SqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlSelectStatement {
    pub fn with_with_clause(self, element: Option<SqlWithClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_select_clause(self, element: SqlSelectClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_from_clause(self, element: Option<SqlFromClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<SqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_group_by_clause(self, element: Option<SqlGroupByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_having_clause(self, element: Option<SqlHavingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_set_operations(self, element: SqlSetOperationList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(6usize..=6usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_order_by_clause(self, element: Option<SqlOrderByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            7usize..=7usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_limit_clause(self, element: Option<PsqlLimitClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            8usize..=8usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_offset_clause(self, element: Option<SqlOffsetClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            9usize..=9usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_fetch_clause(self, element: Option<SqlFetchClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            10usize..=10usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            11usize..=11usize,
            once(element.map(|element| element.into())),
        ))
    }
}
impl SqlSetClause {
    pub fn with_set_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlSetItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlSetItem {
    pub fn with_column(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_eq_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_expr(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlSetOperation {
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_quantifier_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_select_clause(self, element: SqlSelectClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_from_clause(self, element: Option<SqlFromClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<SqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_group_by_clause(self, element: Option<SqlGroupByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_having_clause(self, element: Option<SqlHavingClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlShemaName {
    pub fn with_base(self, element: Option<SqlDataBaseName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlStar {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlStringLiteralExpression {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlSubqueryBinding {
    pub fn with_lateral_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_query(self, element: AnySqlSubqueryBody) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
    pub fn with_alias(self, element: Option<SqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlSubqueryExpression {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_query(self, element: AnySqlSubqueryBody) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlTableBinding {
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_alias(self, element: Option<SqlAlias>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlTableColReference {
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_name(self, element: SqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlTableName {
    pub fn with_schema(self, element: Option<SqlShemaName>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_name(self, element: AnySqlName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlTableStar {
    pub fn with_table(self, element: SqlTableName) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_dot_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_star(self, element: SqlStar) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlTildeName {
    pub fn with_value_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlTimeZoneModifier {
    pub fn with_with_or_without_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_time_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_zone_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlTypeArguments {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlTypeArgumentList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlTypeName {
    pub fn with_name_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_args(self, element: Option<SqlTypeArguments>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_modifier(self, element: Option<AnySqlTypeModifier>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_array_suffix(self, element: Option<AnySqlTypeArraySuffix>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            3usize..=3usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
}
impl SqlUnaryExpression {
    pub fn with_operator_token_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_expression(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlUpdateFromClause {
    pub fn with_from_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlFromItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlUpdateStatement {
    pub fn with_with_clause(self, element: Option<SqlWithClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_update_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_table(self, element: SqlTableBinding) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_set_clause(self, element: SqlSetClause) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_from_clause(self, element: Option<SqlUpdateFromClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            4usize..=4usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_where_clause(self, element: Option<SqlWhereClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            5usize..=5usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_returning_clause(self, element: Option<PsqlReturningClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            6usize..=6usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(7usize..=7usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlValuesClause {
    pub fn with_with_clause(self, element: Option<SqlWithClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            0usize..=0usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_values_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_rows(self, element: SqlValuesRowList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_semicolon_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(element.map(|element| element.into()))),
        )
    }
}
impl SqlValuesRow {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlExpressionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into()))),
        )
    }
}
impl SqlVaryingModifier {
    pub fn with_varying_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
}
impl SqlWhereClause {
    pub fn with_where_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_condition(self, element: AnySqlExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlWindowFunctionExpression {
    pub fn with_call(self, element: SqlCallExpression) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into_syntax().into()))),
        )
    }
    pub fn with_over_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into()))),
        )
    }
    pub fn with_window(self, element: SqlWindowSpecification) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlWindowPartitionByClause {
    pub fn with_partition_by_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_items(self, element: SqlWindowPartitionByItemList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(Some(element.into_syntax().into()))),
        )
    }
}
impl SqlWindowSpecification {
    pub fn with_l_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_partition_by_clause(self, element: Option<SqlWindowPartitionByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            1usize..=1usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_order_by_clause(self, element: Option<SqlOrderByClause>) -> Self {
        Self::unwrap_cast(self.syntax.splice_slots(
            2usize..=2usize,
            once(element.map(|element| element.into_syntax().into())),
        ))
    }
    pub fn with_r_paren_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(3usize..=3usize, once(Some(element.into()))),
        )
    }
}
impl SqlWithClause {
    pub fn with_with_token(self, element: SyntaxToken) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(0usize..=0usize, once(Some(element.into()))),
        )
    }
    pub fn with_recursive_token(self, element: Option<SyntaxToken>) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(1usize..=1usize, once(element.map(|element| element.into()))),
        )
    }
    pub fn with_ctes(self, element: SqlCteDefinitionList) -> Self {
        Self::unwrap_cast(
            self.syntax
                .splice_slots(2usize..=2usize, once(Some(element.into_syntax().into()))),
        )
    }
}
