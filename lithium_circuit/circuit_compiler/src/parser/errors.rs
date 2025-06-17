use iter_extended::vecmap;
use reporter::{CustomDiagnostic as Diagnostic, Location, Span};
use thiserror::Error;

#[derive(Error, Clone, Debug, PartialEq, Eq)]
pub enum ParserErrorReason {
    #[error("Illegal Expression {expression}")]
    IllegalExpression { expression: String },
    #[error("Unrecognized Include")]
    UnrecognizedInclude,
    #[error("Expected `;`")]
    MissingSemicolon,
    #[error("Unexpected `;`")]
    UnexpectedSemicolon,
    #[error("Unclosed Comment")]
    UnclosedComment,
    #[error("Invalid left-hand side of assignment")]
    InvalidLeftHandSideOfAssignment,
    #[error("Invalid right-hand side of assignment")]
    InvalidRightHandSideOfAssignment,
    #[error("`unconstrained` cannot be applied to this item")]
    UnconstrainedNotApplicable,
    #[error("Invalid pattern")]
    InvalidPattern,
    #[error("Expected a function body (`{{ ... }}`), not `;`")]
    ExpectedFunctionBody,
    #[error("Expected a ; separating these two statements")]
    MissingSeparatingSemi,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParserError {
    reason: Option<ParserErrorReason>,
    location: Location,
}

impl ParserError {
    pub fn span(&self) -> Span {
        self.location.span
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn reason(&self) -> Option<&ParserErrorReason> {
        self.reason.as_ref()
    }

    pub fn is_warning(&self) -> bool {
        let diagnostic: Diagnostic = self.into();
        diagnostic.is_warning()
    }

    pub fn empty(location: Location) -> Self {
        Self { reason: None, location }
    }
    pub fn with_reason(reason: &ParserErrorReason, location: Location) -> Self {
        Self { reason: Some(reason.clone()), location }
    }
    pub fn illegal_expression(expression: String, location: Location) -> Self {
        Self {
            reason: Some(ParserErrorReason::IllegalExpression { expression: expression }),
            location,
        }
    }
    pub fn unrecognized_include(location: Location) -> Self {
        Self { reason: Some(ParserErrorReason::UnrecognizedInclude), location }
    }
    pub fn missing_semicolon(location: Location) -> Self {
        Self { reason: Some(ParserErrorReason::MissingSemicolon), location }
    }
    pub fn unexpected_semicolon(location: Location) -> Self {
        Self { reason: Some(ParserErrorReason::UnexpectedSemicolon), location }
    }
    pub fn unclosed_comment(location: Location) -> Self {
        Self { reason: Some(ParserErrorReason::UnclosedComment), location }
    }
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let reason_str: String = if self.reason.is_none() {
            "".to_string()
        } else {
            format!("\nreason: {}", Diagnostic::from(self))
        };
        write!(f, "{}", reason_str)
    }
}

impl<'a> From<&'a ParserError> for Diagnostic {
    fn from(error: &'a ParserError) -> Diagnostic {
        match &error.reason {
            Some(reason) => match reason {
                ParserErrorReason::IllegalExpression { expression } => Diagnostic::simple_error(
                    "Caught Illegal Expression".into(),
                    expression.into(),
                    error.location(),
                ),
                ParserErrorReason::UnrecognizedInclude => Diagnostic::simple_error(
                    "Unrecognized Include".into(),
                    format!("check for missing semicolon (;)"),
                    error.location(),
                ),
                ParserErrorReason::MissingSemicolon => Diagnostic::simple_error(
                    "Missing Semicolon".into(),
                    "".into(),
                    error.location(),
                ),
                other => {
                    Diagnostic::simple_error(format!("{other}"), String::new(), error.location())
                }
            },
            None => {
                let primary = error.to_string();
                Diagnostic::simple_error(primary, String::new(), error.location())
            }
        }
    }
}
