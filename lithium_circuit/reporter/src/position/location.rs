use super::span::Span;
use file_map::FileId;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Deserialize, Serialize)]
pub struct Location {
    pub span: Span,
    pub file: FileId,
}

impl Location {
    pub fn new(span: Span, file: FileId) -> Self {
        Self { span, file }
    }
    pub fn dummy() -> Self {
        Self { span: Span::default(), file: FileId::dummy() }
    }
    pub fn contains(&self, other: &Location) -> bool {
        self.file == other.file && self.span.contains(&other.span)
    }
    #[must_use]
    pub fn merge(self, other: Location) -> Location {
        if self.file == other.file {
            Location::new(self.span.merge(other.span), self.file)
        } else {
            self
        }
    }
}

impl Ord for Location {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.file, self.span).cmp(&(other.file, other.span))
    }
}

impl PartialOrd for Location {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
