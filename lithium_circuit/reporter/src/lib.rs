mod call_stack;
mod position;
mod reporter;

pub use call_stack::CallStack;
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::Files,
    term,
    term::termcolor::{ColorChoice, StandardStream},
};
use std::io::IsTerminal;

pub use position::{located::Located, location::Location, span::Span};
pub use reporter::{CustomDiagnostic, DiagnosticKind, ReportedErrors};

pub fn report_all<'files>(
    files: &'files impl Files<'files, FileId = file_map::FileId>,
    diagnostics: &[CustomDiagnostic],
    deny_warnings: bool,
    silence_warnings: bool,
) -> ReportedErrors {
    let (warnings_and_bugs, mut errors): (Vec<_>, _) =
        diagnostics.iter().partition(|item| !item.is_error());

    let (warnings, mut bugs): (Vec<_>, _) =
        warnings_and_bugs.iter().partition(|item| item.is_warning());
    let mut diagnostics = if silence_warnings { Vec::new() } else { warnings };
    diagnostics.append(&mut bugs);
    diagnostics.append(&mut errors);

    let error_count =
        diagnostics.iter().map(|error| error.report(files, deny_warnings) as u32).sum();

    ReportedErrors { error_count }
}

pub fn report<'files>(
    files: &'files impl Files<'files, FileId = file_map::FileId>,
    custom_diagnostic: &CustomDiagnostic,
    deny_warnings: bool,
) -> bool {
    let color_choice =
        if std::io::stderr().is_terminal() { ColorChoice::Auto } else { ColorChoice::Never };
    let writer = StandardStream::stderr(color_choice);
    let config = term::Config::default();

    let stack_trace = stack_trace(files, &custom_diagnostic.call_stack);
    let diagnostic = convert_diagnostic(custom_diagnostic, stack_trace, deny_warnings);
    term::emit(&mut writer.lock(), &config, files, &diagnostic).unwrap();

    deny_warnings || custom_diagnostic.is_error()
}

fn convert_diagnostic(
    cd: &CustomDiagnostic,
    stack_trace: String,
    deny_warnings: bool,
) -> Diagnostic<file_map::FileId> {
    let diagnostic = match (cd.kind, deny_warnings) {
        (DiagnosticKind::Warning, false) => Diagnostic::warning(),
        (DiagnosticKind::Info, _) => Diagnostic::note(),
        (DiagnosticKind::Bug, ..) => Diagnostic::bug(),
        _ => Diagnostic::error(),
    };

    let secondary_labels = cd
        .secondaries
        .iter()
        .map(|custom_label| {
            let location = custom_label.location;
            let span = location.span;
            let start_span = span.start() as usize;
            let end_span = span.end() as usize;
            let file = location.file;
            Label::secondary(file, start_span..end_span).with_message(&custom_label.message)
        })
        .collect();

    let mut notes = cd.notes.clone();
    notes.push(stack_trace);

    diagnostic.with_message(&cd.message).with_labels(secondary_labels).with_notes(notes)
}

pub fn stack_trace<'files>(
    files: &'files impl Files<'files, FileId = file_map::FileId>,
    call_stack: &[Location],
) -> String {
    if call_stack.is_empty() {
        return String::new();
    }

    let mut result = "Call stack:\n".to_string();

    for (i, call_item) in call_stack.iter().enumerate() {
        let path = files.name(call_item.file).expect("should get file path");
        let source = files.source(call_item.file).expect("should get file source");

        let (line, column) = line_and_column_from_span(source.as_ref(), &call_item.span);
        result += &format!("{}. {}:{}:{}\n", i + 1, path, line, column);
    }

    result
}

pub fn line_and_column_from_span(source: &str, span: &Span) -> (u32, u32) {
    let mut line = 1;
    let mut column = 0;

    for (i, char) in source.chars().enumerate() {
        column += 1;

        if char == '\n' {
            line += 1;
            column = 0;
        }

        if span.start() <= i as u32 {
            break;
        }
    }

    (line, column)
}
