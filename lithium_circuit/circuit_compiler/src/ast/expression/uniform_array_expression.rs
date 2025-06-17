use super::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UniformArrayExpression {
    pub value: Box<Expression>,
    pub dimension: Box<Expression>,
}
