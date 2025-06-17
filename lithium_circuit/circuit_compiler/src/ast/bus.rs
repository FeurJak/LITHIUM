use super::{Ident, expression::BlockExpression};
use reporter::Location;

/// Bus represents a goup of signals:
/// ```
/// <bus> {NAME}
/// (
///     {PARAMETERS}
/// )
/// {
///     {BODY}
/// }
/// ```
/// Example:
///
/// ```
/// bus Data(n){
///     signal byte[n];
/// }
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bus {
    pub def: BusDefinition,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct BusDefinition {
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
