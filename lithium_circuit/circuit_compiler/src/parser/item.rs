use crate::ast::{Bus, Template, UnconstrainedFunction};
use file_map::FileId;
use reporter::{Location, Span};

#[derive(Clone, Debug)]
pub struct Item {
    pub kind: ItemKind,
    pub location: Location,
    pub doc_comments: Vec<String>,
}

impl Item {
    pub fn new_include(include: String, start: usize, end: usize, file_id: usize) -> Item {
        return Item {
            kind: ItemKind::new_include(include),
            location: Location::new(Span::from(start..end), FileId::from(file_id)),
            doc_comments: Vec::new(),
        };
    }
}

#[derive(Clone, Debug)]
pub enum ItemKind {
    Include(String),
    Template(Template),
    Function(UnconstrainedFunction),
    Bus(Bus),
}

impl ItemKind {
    pub fn new_include(include: String) -> ItemKind {
        return ItemKind::Include(include);
    }
}
