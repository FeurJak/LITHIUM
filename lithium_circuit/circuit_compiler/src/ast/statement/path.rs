use super::Ident;
use iter_extended::vecmap;
use reporter::{Located, Location, Span};
use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum PathKind {
    Program,
    Dep,
    Plain,
    Super,
    //Resolved(CrateId),
}

impl Display for PathKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PathKind::Program => write!(f, "program"),
            PathKind::Dep => write!(f, "dep"),
            PathKind::Super => write!(f, "super"),
            PathKind::Plain => write!(f, "plain"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Path {
    pub segments: Vec<PathSegment>,
    pub kind: PathKind,
    pub location: Location,
    // The location of `kind` (this is the same as `location` for plain kinds)
    pub kind_location: Location,
}

impl Path {
    pub fn plain(segments: Vec<PathSegment>, location: Location) -> Self {
        Self { segments, location, kind: PathKind::Plain, kind_location: location }
    }

    pub fn pop(&mut self) -> PathSegment {
        self.segments.pop().unwrap()
    }

    fn join(mut self, ident: Ident) -> Path {
        self.segments.push(PathSegment::from(ident));
        self
    }

    pub fn from_single(name: String, location: Location) -> Path {
        let segment = Ident::from(Located::from(location, name));
        Path::from_ident(segment)
    }

    pub fn from_ident(name: Ident) -> Path {
        let location = name.location();
        Path::plain(vec![PathSegment::from(name)], location)
    }

    pub fn span(&self) -> Span {
        self.location.span
    }

    pub fn last_segment(&self) -> PathSegment {
        assert!(!self.segments.is_empty());
        self.segments.last().unwrap().clone()
    }

    pub fn last_ident(&self) -> Ident {
        self.last_segment().ident
    }

    pub fn first_name(&self) -> Option<&str> {
        self.segments.first().map(|segment| segment.ident.as_str())
    }

    pub fn last_name(&self) -> &str {
        assert!(!self.segments.is_empty());
        self.segments.last().unwrap().ident.as_str()
    }

    pub fn is_ident(&self) -> bool {
        self.kind == PathKind::Plain && self.segments.len() == 1
    }

    pub fn as_ident(&self) -> Option<&Ident> {
        if !self.is_ident() {
            return None;
        }
        self.segments.first().map(|segment| &segment.ident)
    }

    pub fn is_empty(&self) -> bool {
        self.segments.is_empty() && self.kind == PathKind::Plain
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let segments = vecmap(&self.segments, ToString::to_string);
        if self.kind == PathKind::Plain {
            write!(f, "{}", segments.join("::"))
        } else {
            write!(f, "{}::{}", self.kind, segments.join("::"))
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct PathSegment {
    pub ident: Ident,
    pub location: Location,
}

impl PathSegment {
    pub fn turbofish_span(&self) -> Span {
        if self.ident.location().file == self.location.file {
            Span::from(self.ident.span().end()..self.location.span.end())
        } else {
            self.location.span
        }
    }

    pub fn turbofish_location(&self) -> Location {
        Location::new(self.turbofish_span(), self.location.file)
    }
}

impl From<Ident> for PathSegment {
    fn from(ident: Ident) -> PathSegment {
        let location = ident.location();
        PathSegment { ident, location }
    }
}

impl Display for PathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.ident.fmt(f)?;
        Ok(())
    }
}
