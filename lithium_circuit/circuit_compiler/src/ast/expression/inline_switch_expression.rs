use super::Expression;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct InlineSwitchExpression {
    pub condition: Expression,
    pub if_true: Box<Expression>,
    pub if_false: Box<Expression>,
}
