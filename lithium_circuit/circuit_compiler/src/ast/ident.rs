use reporter::{Located, Location, Span};
use std::fmt::Display;

#[derive(Eq, Debug, Clone)]
pub struct Ident(Located<String>);

impl Ident {
    pub fn new(text: String, location: Location) -> Ident {
        Ident(Located::from(location, text))
    }

    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    pub fn location(&self) -> Location {
        self.0.location()
    }

    pub fn span(&self) -> Span {
        self.0.span()
    }

    pub fn as_str(&self) -> &str {
        &self.0.contents
    }

    pub fn as_string(&self) -> &String {
        &self.0.contents
    }

    pub fn into_string(self) -> String {
        self.0.contents
    }
}

impl PartialEq<Ident> for Ident {
    fn eq(&self, other: &Ident) -> bool {
        self.as_str() == other.as_str()
    }
}

impl PartialOrd for Ident {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Ident {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl PartialEq<str> for Ident {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl std::hash::Hash for Ident {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state);
    }
}

impl Display for Ident {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl From<Located<String>> for Ident {
    fn from(a: Located<String>) -> Ident {
        Ident(a)
    }
}

impl From<String> for Ident {
    fn from(a: String) -> Ident {
        Located::from(Location::dummy(), a).into()
    }
}
impl From<&str> for Ident {
    fn from(a: &str) -> Ident {
        Ident::from(a.to_owned())
    }
}
