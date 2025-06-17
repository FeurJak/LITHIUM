use super::{location::Location, span::Span};
use std::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

#[derive(Eq, Debug, Clone)]
pub struct Located<T> {
    pub contents: T,
    location: Location,
}

impl<T: PartialEq> PartialEq<Located<T>> for Located<T> {
    fn eq(&self, other: &Located<T>) -> bool {
        self.contents == other.contents
    }
}

impl<T: PartialOrd> PartialOrd<Located<T>> for Located<T> {
    fn partial_cmp(&self, other: &Located<T>) -> Option<Ordering> {
        self.contents.partial_cmp(&other.contents)
    }
}

impl<T: Ord> Ord for Located<T> {
    fn cmp(&self, other: &Located<T>) -> Ordering {
        self.contents.cmp(&other.contents)
    }
}

impl<T: Default> Default for Located<T> {
    fn default() -> Self {
        Self { contents: Default::default(), location: Location::dummy() }
    }
}

impl<T: Hash> Hash for Located<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contents.hash(state);
    }
}

impl<T> Located<T> {
    pub fn from(location: Location, contents: T) -> Located<T> {
        Located { location, contents }
    }

    pub fn span(&self) -> Span {
        self.location.span
    }

    pub fn location(&self) -> Location {
        self.location
    }
}
