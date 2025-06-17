use super::{Expression, Ident};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssignStatement {
    pub lvalue: LValue,
    pub expression: Expression,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum LValue {
    Variable(Ident),
    ConstrainedSignal(Ident),
    UnConstrainedSignal(Ident),
}
