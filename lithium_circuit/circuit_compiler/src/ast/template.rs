use super::{Ident, expression::BlockExpression};
use reporter::Location;

// Template represents a constrained function:
// template {{NAME}}({{PARAMETERS}})
// {
//     {{BODY}}
// }
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Template {
    pub def: TemplateDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TemplateDefinition {
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
