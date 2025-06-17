mod bus;
mod expression;
mod function;
mod ident;
mod statement;
mod template;

use crate::parser::Item;
pub use bus::Bus;
pub use function::UnconstrainedFunction;
pub use ident::Ident;
pub use template::Template;

/// Abstract Syntax Tree (AST)
#[derive(Clone, Debug, Default)]
pub struct AST {
    pub items: Vec<Item>,
    pub inner_doc_comments: Vec<String>,
}

impl AST {
    pub fn new(items: Vec<Item>) -> Self {
        return AST { items: items, inner_doc_comments: Vec::new() };
    }
}
