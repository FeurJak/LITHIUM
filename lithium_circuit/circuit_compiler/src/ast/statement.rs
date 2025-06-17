mod assign_statement;
mod path;
mod while_statement;

use super::{Ident, expression::Expression};
pub use path::Path;
use reporter::Location;
use while_statement::WhileStatement;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Statement {
    pub kind: StatementKind,
    pub location: Location,
}

// impl Statement {
//     pub fn add_semicolon(
//         mut self,
//         semi: Option<Token>,
//         location: Location,
//         last_statement_in_block: bool,
//         emit_error: &mut dyn FnMut(ParserError),
//     ) -> Self {
//         self.kind = self.kind.add_semicolon(semi, location, last_statement_in_block, emit_error);
//         self
//     }

//     /// Returns the innermost location that gives this statement its type.
//     pub fn type_location(&self) -> Location {
//         match &self.kind {
//             StatementKind::Expression(expression) => expression.type_location(),
//             StatementKind::Comptime(statement) => statement.type_location(),
//             StatementKind::Let(..)
//             | StatementKind::Assign(..)
//             | StatementKind::For(..)
//             | StatementKind::Loop(..)
//             | StatementKind::While(..)
//             | StatementKind::Break
//             | StatementKind::Continue
//             | StatementKind::Semi(..)
//             | StatementKind::Interned(..)
//             | StatementKind::Error => self.location,
//         }
//     }
// }

/// AST node for statements. Statements are always within a block { }
/// of some kind and are terminated via a Semicolon, except if the statement
/// ends in a block.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum StatementKind {
    While(WhileStatement),
    // Expression(Expression),
    // Assign(AssignStatement),
    // For(ForLoopStatement),
    // Loop(Expression, Location /* loop keyword location */),
    // While(WhileStatement),
    // This statement is the result of a recovered parse error.
    // To avoid issuing multiple errors in later steps, it should
    // be skipped in any future analysis if possible.
    Error,
}

// impl StatementKind {
//     pub fn add_semicolon(
//         self,
//         semi: Option<Token>,
//         location: Location,
//         last_statement_in_block: bool,
//         emit_error: &mut dyn FnMut(ParserError),
//     ) -> Self {
//         let missing_semicolon =
//             ParserError::with_reason(ParserErrorReason::MissingSeparatingSemi, location);

//         match self {
//             StatementKind::Let(_) => {
//                 // To match rust, a let statement always requires a semicolon, even at the end of a block
//                 if semi.is_none() {
//                     let reason = ParserErrorReason::MissingSemicolonAfterLet;
//                     emit_error(ParserError::with_reason(reason, location));
//                 }
//                 self
//             }
//             StatementKind::Assign(_)
//             | StatementKind::Semi(_)
//             | StatementKind::Break
//             | StatementKind::Continue
//             | StatementKind::Error => {
//                 // These statements can omit the semicolon if they are the last statement in a block
//                 if !last_statement_in_block && semi.is_none() {
//                     emit_error(missing_semicolon);
//                 }
//                 self
//             }
//             StatementKind::Comptime(mut statement) => {
//                 *statement =
//                     statement.add_semicolon(semi, location, last_statement_in_block, emit_error);
//                 StatementKind::Comptime(statement)
//             }
//             // A semicolon on a for loop, loop or while is optional and does nothing
//             StatementKind::For(_) | StatementKind::Loop(..) | StatementKind::While(..) => self,

//             // No semicolon needed for a resolved statement
//             StatementKind::Interned(_) => self,

//             StatementKind::Expression(expr) => {
//                 match (&expr.kind, semi, last_statement_in_block) {
//                     // Semicolons are optional for these expressions
//                     (ExpressionKind::Block(_), semi, _)
//                     | (ExpressionKind::Unsafe(..), semi, _)
//                     | (ExpressionKind::Interned(..), semi, _)
//                     | (ExpressionKind::InternedStatement(..), semi, _)
//                     | (ExpressionKind::Match(..), semi, _)
//                     | (ExpressionKind::If(_), semi, _) => {
//                         if semi.is_some() {
//                             StatementKind::Semi(expr)
//                         } else {
//                             StatementKind::Expression(expr)
//                         }
//                     }
//                     (_, None, false) => {
//                         emit_error(missing_semicolon);
//                         StatementKind::Expression(expr)
//                     }
//                     (_, Some(_), _) => StatementKind::Semi(expr),
//                     (_, None, true) => StatementKind::Expression(expr),
//                 }
//             }
//         }
//     }
// }
