use super::Expression;
use reporter::Location;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct WhileStatement {
    pub condition: Expression,
    pub body: Expression,
    pub while_keyword_location: Location,
}
