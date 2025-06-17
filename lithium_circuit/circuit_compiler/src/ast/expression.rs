mod anonymous_component_expression;
mod binary_operation;
mod call_expression;
mod infix_expression;
mod inline_switch_expression;
mod prefix_expression;
mod unary_operation;
mod uniform_array_expression;

use super::statement::{Path, Statement};
use binary_operation::BinaryOp;
use num_bigint::BigInt;
use reporter::Location;
use unary_operation::UnaryOp;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Expression {
    pub kind: ExpressionKind,
    pub location: Location,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum ExpressionKind {
    Number(BigInt),
    Variable(Path),
    Infix(Box<infix_expression::InfixExpression>),
    Prefix(Box<prefix_expression::PrefixExpression>),
    InlineSwitch(Box<inline_switch_expression::InlineSwitchExpression>),
    Call(Box<call_expression::CallExpression>),
    BusCall(Box<call_expression::BusCallExpression>),
    AnonymousComponent(Box<anonymous_component_expression::AnonymousComponentExpression>),
    ArrayInLine(Vec<Expression>),
    Tuple(Vec<Expression>),
    UniformArray(Box<uniform_array_expression::UniformArrayExpression>),
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BlockExpression {
    pub statements: Vec<Statement>,
}

impl BlockExpression {
    pub fn is_empty(&self) -> bool {
        self.statements.is_empty()
    }
}
