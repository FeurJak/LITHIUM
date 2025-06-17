use super::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct CallExpression {
    pub func: Box<Expression>,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BusCallExpression {
    pub bus: Expression,
    pub arguments: Vec<Expression>,
}
