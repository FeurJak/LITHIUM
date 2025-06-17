use super::{CallStack, Location};
use codespan_reporting::files::Files;

#[derive(Debug, Copy, Clone)]
pub struct ReportedErrors {
    pub error_count: u32,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum DiagnosticKind {
    Error,
    Bug,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomLabel {
    pub message: String,
    pub location: Location,
}

impl CustomLabel {
    fn new(message: String, location: Location) -> CustomLabel {
        CustomLabel { message, location }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CustomDiagnostic {
    pub file: file_map::FileId,
    pub message: String,
    pub secondaries: Vec<CustomLabel>,
    pub notes: Vec<String>,
    pub kind: DiagnosticKind,
    pub deprecated: bool,
    pub unnecessary: bool,
    pub call_stack: CallStack,
}

impl CustomDiagnostic {
    pub fn from_message(msg: &str, file: file_map::FileId) -> CustomDiagnostic {
        Self {
            file,
            message: msg.to_owned(),
            secondaries: Vec::new(),
            notes: Vec::new(),
            kind: DiagnosticKind::Error,
            deprecated: false,
            unnecessary: false,
            call_stack: Default::default(),
        }
    }

    fn simple_with_kind(
        primary_message: String,
        secondary_message: String,
        secondary_location: Location,
        kind: DiagnosticKind,
    ) -> CustomDiagnostic {
        CustomDiagnostic {
            file: secondary_location.file,
            message: primary_message,
            secondaries: vec![CustomLabel::new(secondary_message, secondary_location)],
            notes: Vec::new(),
            kind,
            deprecated: false,
            unnecessary: false,
            call_stack: Default::default(),
        }
    }

    pub fn simple_error(
        primary_message: String,
        secondary_message: String,
        secondary_location: Location,
    ) -> CustomDiagnostic {
        Self::simple_with_kind(
            primary_message,
            secondary_message,
            secondary_location,
            DiagnosticKind::Error,
        )
    }

    pub fn simple_warning(
        primary_message: String,
        secondary_message: String,
        secondary_location: Location,
    ) -> CustomDiagnostic {
        Self::simple_with_kind(
            primary_message,
            secondary_message,
            secondary_location,
            DiagnosticKind::Warning,
        )
    }

    pub fn simple_info(
        primary_message: String,
        secondary_message: String,
        secondary_location: Location,
    ) -> CustomDiagnostic {
        Self::simple_with_kind(
            primary_message,
            secondary_message,
            secondary_location,
            DiagnosticKind::Info,
        )
    }

    pub fn simple_bug(
        primary_message: String,
        secondary_message: String,
        secondary_location: Location,
    ) -> CustomDiagnostic {
        CustomDiagnostic {
            file: secondary_location.file,
            message: primary_message,
            secondaries: vec![CustomLabel::new(secondary_message, secondary_location)],
            notes: Vec::new(),
            kind: DiagnosticKind::Bug,
            deprecated: false,
            unnecessary: false,
            call_stack: Default::default(),
        }
    }

    pub fn with_call_stack(mut self, call_stack: Vec<Location>) -> Self {
        self.call_stack = call_stack;
        self
    }

    pub fn add_note(&mut self, message: String) {
        self.notes.push(message);
    }

    pub fn add_secondary(&mut self, message: String, location: Location) {
        self.secondaries.push(CustomLabel::new(message, location));
    }

    pub fn is_error(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Error)
    }

    pub fn is_warning(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Warning)
    }

    pub fn is_info(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Info)
    }

    pub fn is_bug(&self) -> bool {
        matches!(self.kind, DiagnosticKind::Bug)
    }
}

impl std::fmt::Display for CustomDiagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)?;

        for secondary in &self.secondaries {
            write!(f, "\nsecondary: {}", secondary.message)?;
        }

        for note in &self.notes {
            write!(f, "\nnote: {note}")?;
        }

        Ok(())
    }
}

impl CustomDiagnostic {
    pub fn report<'files>(
        &self,
        files: &'files impl Files<'files, FileId = file_map::FileId>,
        deny_warnings: bool,
    ) -> bool {
        crate::report(files, self, deny_warnings)
    }
}
