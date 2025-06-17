use super::Expression;

// Anonymous Components:
// signal out <== A(n)(in[0],in[1]);
// component temp_a = A(n);
// temp_a.a <== in[0];
// temp_a.b <== in[1];
// out <== temp_a.c;
//
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AnonymousComponentExpression {
    // A
    pub template: Box<Expression>,
    // (n)
    pub params: Vec<Expression>,
    // (in[0], in[1])
    pub signals: Vec<Expression>,
}
