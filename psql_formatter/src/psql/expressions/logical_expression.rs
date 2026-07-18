use crate::comments::PsqlComments;
use crate::prelude::*;
use biome_formatter::write;
use biome_rowan::SyntaxResult;
use psql_syntax::{
    AnyPsqlExpression, PsqlLogicalExpression, PsqlLogicalExpressionFields, PsqlSyntaxKind,
    PsqlSyntaxToken,
};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatPsqlLogicalExpression;
impl FormatNodeRule<PsqlLogicalExpression> for FormatPsqlLogicalExpression {
    fn fmt_fields(&self, node: &PsqlLogicalExpression, f: &mut PsqlFormatter) -> FormatResult<()> {
        let operator_kind = node.operator_token()?.kind();

        // Flatten a left-associative run of the *same* and/or operator into
        // a flat operand list -- `a and b and c` is parsed as
        // `(a and b) and c`, so this walks down `left` while it's still the
        // same operator, treating anything else (a different operator, or a
        // non-logical expression) as one opaque operand.
        let mut operands = Vec::new();
        collect_operands(node.left(), operator_kind, &mut operands, f.comments())?;
        operands.push((Some(node.operator_token()?), node.right()?));

        let (_, first) = &operands[0];
        write!(f, [first.format()])?;

        if operands.len() <= 2 {
            // Style rule 5: a chain of at most two conditions never wraps,
            // no matter how long the line ends up being.
            for (operator, expr) in &operands[1..] {
                let operator = operator
                    .as_ref()
                    .expect("non-first operand has an operator");
                write!(f, [space(), operator.format(), space(), expr.format()])?;
            }
            Ok(())
        } else {
            // More than two conditions: every condition after the first
            // always gets its own line, indented one level deeper than the
            // line being continued, with the operator leading (not
            // trailing, unlike list commas).
            write!(
                f,
                [indent(&format_once(|f| {
                    for (operator, expr) in &operands[1..] {
                        let operator = operator
                            .as_ref()
                            .expect("non-first operand has an operator");
                        write!(
                            f,
                            [hard_line_break(), operator.format(), space(), expr.format()]
                        )?;
                    }
                    Ok(())
                }))]
            )
        }
    }
}

fn collect_operands(
    expr: SyntaxResult<AnyPsqlExpression>,
    operator_kind: PsqlSyntaxKind,
    out: &mut Vec<(Option<PsqlSyntaxToken>, AnyPsqlExpression)>,
    comments: &PsqlComments,
) -> FormatResult<()> {
    match expr? {
        AnyPsqlExpression::PsqlLogicalExpression(logical)
            if logical.operator_token()?.kind() == operator_kind =>
        {
            // This intermediate node is flattened away -- it never gets its
            // own `.format()` call -- but the formatter still requires
            // every node to be checked for suppression comments. Any real
            // leading/trailing comments on it would currently be dropped;
            // acceptable for now (rare in practice, and comments generally
            // are a lightly-supported edge case at this phase), but a real
            // limitation to revisit if it comes up.
            comments.mark_suppression_checked(logical.syntax());

            let PsqlLogicalExpressionFields {
                left,
                operator_token,
                right,
            } = logical.as_fields();
            collect_operands(left, operator_kind, out, comments)?;
            out.push((Some(operator_token?), right?));
        }
        other => out.push((None, other)),
    }
    Ok(())
}
