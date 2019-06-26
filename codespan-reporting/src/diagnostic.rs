//! Diagnostic data structures.

#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

use crate::Files;

/// A severity level for diagnostic messages.
///
/// These are ordered in the following way:
///
/// ```rust
/// use codespan_reporting::diagnostic::Severity;
///
/// assert!(Severity::Bug > Severity::Error);
/// assert!(Severity::Error > Severity::Warning);
/// assert!(Severity::Warning > Severity::Note);
/// assert!(Severity::Note > Severity::Help);
/// ```
#[derive(Copy, Clone, PartialEq, Hash, Debug)]
#[cfg_attr(feature = "memory_usage", derive(heapsize_derive::HeapSizeOf))]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum Severity {
    /// An unexpected bug.
    Bug,
    /// An error.
    Error,
    /// A warning.
    Warning,
    /// A note.
    Note,
    /// A help message.
    Help,
}

impl Severity {
    /// We want bugs to be the maximum severity, errors next, etc...
    fn to_cmp_int(self) -> u8 {
        match self {
            Severity::Bug => 5,
            Severity::Error => 4,
            Severity::Warning => 3,
            Severity::Note => 2,
            Severity::Help => 1,
        }
    }
}

impl PartialOrd for Severity {
    fn partial_cmp(&self, other: &Severity) -> Option<Ordering> {
        u8::partial_cmp(&self.to_cmp_int(), &other.to_cmp_int())
    }
}

/// A label describing an underlined region of code associated with a diagnostic.
// FIXME: #[derive(Clone, Debug)]
#[cfg_attr(feature = "memory_usage", derive(heapsize_derive::HeapSizeOf))]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Label<F: Files> {
    /// The file that we are labelling.
    pub file_id: F::FileId,
    /// The span we are going to include in the final snippet.
    pub span: F::Span,
    /// A message to provide some additional information for the underlined
    /// code. These should not include line breaks.
    pub message: String,
}

impl<F: Files> Label<F> {
    /// Create a new label.
    pub fn new(
        file_id: F::FileId,
        span: impl Into<F::Span>,
        message: impl Into<String>,
    ) -> Label<F> {
        Label {
            file_id,
            span: span.into(),
            message: message.into(),
        }
    }
}

/// Represents a diagnostic message that can provide information like errors and
/// warnings to the user.
// FIXME: #[derive(Clone, Debug)]
#[cfg_attr(feature = "memory_usage", derive(heapsize_derive::HeapSizeOf))]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Diagnostic<F: Files> {
    /// The overall severity of the diagnostic
    pub severity: Severity,
    /// An optional code that identifies this diagnostic.
    pub code: Option<String>,
    /// The main message associated with this diagnostic.
    ///
    /// These should not include line breaks, and the message should be specific
    /// enough to make sense when paired only with the location given by the
    /// `primary_label`.
    pub message: String,
    /// A label that describes the primary cause of this diagnostic.
    pub primary_label: Label<F>,
    /// Notes that are associated with the primary cause of the diagnostic.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
    /// Secondary labels that provide additional context for the diagnostic.
    pub secondary_labels: Vec<Label<F>>,
}

impl<F: Files> Diagnostic<F> {
    /// Create a new diagnostic.
    pub fn new(
        severity: Severity,
        message: impl Into<String>,
        primary_label: Label<F>,
    ) -> Diagnostic<F> {
        Diagnostic {
            severity,
            code: None,
            message: message.into(),
            primary_label,
            notes: Vec::new(),
            secondary_labels: Vec::new(),
        }
    }

    /// Create a new diagnostic with a severity of `Severity::Bug`.
    pub fn new_bug(message: impl Into<String>, primary_label: Label<F>) -> Diagnostic<F> {
        Diagnostic::new(Severity::Bug, message, primary_label)
    }

    /// Create a new diagnostic with a severity of `Severity::Error`.
    pub fn new_error(message: impl Into<String>, primary_label: Label<F>) -> Diagnostic<F> {
        Diagnostic::new(Severity::Error, message, primary_label)
    }

    /// Create a new diagnostic with a severity of `Severity::Warning`.
    pub fn new_warning(message: impl Into<String>, primary_label: Label<F>) -> Diagnostic<F> {
        Diagnostic::new(Severity::Warning, message, primary_label)
    }

    /// Create a new diagnostic with a severity of `Severity::Note`.
    pub fn new_note(message: impl Into<String>, primary_label: Label<F>) -> Diagnostic<F> {
        Diagnostic::new(Severity::Note, message, primary_label)
    }

    /// Create a new diagnostic with a severity of `Severity::Help`.
    pub fn new_help(message: impl Into<String>, primary_label: Label<F>) -> Diagnostic<F> {
        Diagnostic::new(Severity::Help, message, primary_label)
    }

    /// Add an error code to the diagnostic.
    pub fn with_code(mut self, code: impl Into<String>) -> Diagnostic<F> {
        self.code = Some(code.into());
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_notes(mut self, notes: Vec<String>) -> Diagnostic<F> {
        self.notes = notes;
        self
    }

    /// Add some secondary labels to the diagnostic.
    pub fn with_secondary_labels(
        mut self,
        labels: impl IntoIterator<Item = Label<F>>,
    ) -> Diagnostic<F> {
        self.secondary_labels.extend(labels);
        self
    }
}
