use super::{Ident, expression::BlockExpression};
use reporter::Location;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UnconstrainedFunction {
    pub def: UnconstrainedFunctionDefinition,
}

// UnconstrainedFunction represents a unconstrained function:
// function {{NAME}}({{PARAMETERS}})
// {
//     {{BODY}}
// }
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct UnconstrainedFunctionDefinition {
    pub name: Ident,
    pub parameters: Vec<Param>,
    pub body: BlockExpression,
    pub location: Location,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Param {
    pub identifier: Ident,
    pub location: Location,
}
