use super::{ByteSpan, Position};
use serde::{Deserialize, Serialize};
use std::{
    hash::{Hash, Hasher},
    ops::Range,
};

#[derive(
    PartialEq, PartialOrd, Eq, Ord, Hash, Debug, Copy, Clone, Default, Deserialize, Serialize,
)]
pub struct Span(ByteSpan);

impl Span {
    pub fn inclusive(start: u32, end: u32) -> Span {
        Span(ByteSpan::from(start..end + 1))
    }
    pub fn single_char(start: u32) -> Span {
        Span::inclusive(start, start)
    }

    pub fn empty(position: u32) -> Span {
        Span::from(position..position)
    }
    #[must_use]
    pub fn merge(self, other: Span) -> Span {
        Span(self.0.merge(other.0))
    }
    pub fn to_byte_span(self) -> ByteSpan {
        self.0
    }
    pub fn start(&self) -> u32 {
        self.0.start().into()
    }
    pub fn end(&self) -> u32 {
        self.0.end().into()
    }
    pub fn contains(&self, other: &Span) -> bool {
        self.start() <= other.start() && self.end() >= other.end()
    }
    pub fn intersects(&self, other: &Span) -> bool {
        self.end() >= other.start() && self.start() <= other.end()
    }
    pub fn is_smaller(&self, other: &Span) -> bool {
        let self_distance = self.end() - self.start();
        let other_distance = other.end() - other.start();
        self_distance < other_distance
    }
}

impl From<Span> for Range<usize> {
    fn from(span: Span) -> Self {
        span.0.into()
    }
}

impl From<Range<u32>> for Span {
    fn from(Range { start, end }: Range<u32>) -> Self {
        Self(ByteSpan::new(start, end))
    }
}

impl From<Range<usize>> for Span {
    fn from(Range { start, end }: Range<usize>) -> Self {
        Self(ByteSpan::new(start as u32, end as u32))
    }
}

impl From<Range<i32>> for Span {
    fn from(Range { start, end }: Range<i32>) -> Self {
        Self(ByteSpan::new(start as u32, end as u32))
    }
}

#[derive(PartialOrd, Eq, Ord, Debug, Clone, Default)]
pub struct Spanned<T> {
    pub contents: T,
    span: Span,
}

impl<T: PartialEq> PartialEq<Spanned<T>> for Spanned<T> {
    fn eq(&self, other: &Spanned<T>) -> bool {
        self.contents == other.contents
    }
}

impl<T: Hash> Hash for Spanned<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.contents.hash(state);
    }
}

impl<T> Spanned<T> {
    pub fn from_position(start: Position, end: Position, contents: T) -> Spanned<T> {
        Spanned { span: Span::inclusive(start, end), contents }
    }

    pub fn from(t_span: Span, contents: T) -> Spanned<T> {
        Spanned { span: t_span, contents }
    }

    pub fn span(&self) -> Span {
        self.span
    }
}

impl<T> std::borrow::Borrow<T> for Spanned<T> {
    fn borrow(&self) -> &T {
        &self.contents
    }
}
