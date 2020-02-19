//! Diagnostic data structures.

use codespan::{FileId, Span};
#[cfg(feature = "serialization")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

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

/// A diagnostic title.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Title {
    /// An optional code that identifies this diagnostic.
    pub code: Option<String>,
    /// The main message associated with this diagnostic.
    ///
    /// These should not include line breaks.
    pub message: String,
}

impl Title {
    /// Create a new diagnostic title.
    pub fn new(message: impl Into<String>) -> Title {
        Title {
            message: message.into(),
            code: None,
        }
    }

    /// Add an error code to the diagnostic title.
    pub fn with_code(mut self, code: impl Into<String>) -> Title {
        self.code = Some(code.into());
        self
    }
}

impl From<String> for Title {
    fn from(message: String) -> Title {
        Title::new(message)
    }
}

impl From<&str> for Title {
    fn from(message: &str) -> Title {
        Title::new(message)
    }
}

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub enum LabelStyle {
    /// Labels that describe the primary cause of a diagnostic.
    Primary,
    /// Labels that provide additional context for a diagnostic.
    Secondary,
}

/// A label describing an underlined region of code associated with a diagnostic.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Label {
    /// The style of the label.
    pub style: LabelStyle,
    /// The file that we are labelling.
    pub file_id: FileId,
    /// The span we are going to include in the final snippet.
    pub span: Span,
    /// A message to provide some additional information for the underlined
    /// code. These should not include line breaks.
    pub message: String,
}

impl Label {
    /// Create a new label.
    pub fn new(
        style: LabelStyle,
        file_id: FileId,
        span: impl Into<Span>,
        message: impl Into<String>,
    ) -> Label {
        Label {
            style,
            file_id,
            span: span.into(),
            message: message.into(),
        }
    }

    /// Create a new primary label.
    pub fn primary(file_id: FileId, span: impl Into<Span>, message: impl Into<String>) -> Label {
        Label::new(LabelStyle::Primary, file_id, span, message)
    }

    /// Create a new secondary label.
    pub fn secondary(file_id: FileId, span: impl Into<Span>, message: impl Into<String>) -> Label {
        Label::new(LabelStyle::Secondary, file_id, span, message)
    }
}

/// Represents a diagnostic message that can provide information like errors and
/// warnings to the user.
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serialization", derive(Serialize, Deserialize))]
pub struct Diagnostic {
    /// The overall severity of the diagnostic
    pub severity: Severity,
    /// The title of the diagnostic.
    pub title: Option<Title>,
    /// Source labels that describe the cause of the diagnostic.
    pub labels: Vec<Label>,
    /// Notes that are associated with the primary cause of the diagnostic.
    /// These can include line breaks for improved formatting.
    pub notes: Vec<String>,
}

impl Diagnostic {
    /// Create a new diagnostic.
    pub fn new(severity: Severity) -> Diagnostic {
        Diagnostic {
            severity,
            title: None,
            labels: Vec::new(),
            notes: Vec::new(),
        }
    }

    /// Create a new diagnostic with a severity of `Severity::Bug`.
    pub fn bug() -> Diagnostic {
        Diagnostic::new(Severity::Bug)
    }

    /// Create a new diagnostic with a severity of `Severity::Error`.
    pub fn error() -> Diagnostic {
        Diagnostic::new(Severity::Error)
    }

    /// Create a new diagnostic with a severity of `Severity::Warning`.
    pub fn warning() -> Diagnostic {
        Diagnostic::new(Severity::Warning)
    }

    /// Create a new diagnostic with a severity of `Severity::Note`.
    pub fn note() -> Diagnostic {
        Diagnostic::new(Severity::Note)
    }

    /// Create a new diagnostic with a severity of `Severity::Help`.
    pub fn help() -> Diagnostic {
        Diagnostic::new(Severity::Help)
    }

    /// Add a title to the diagnostic.
    pub fn with_title(mut self, title: impl Into<Title>) -> Diagnostic {
        self.title = Some(title.into());
        self
    }

    /// Add some labels to the diagnostic.
    pub fn with_labels(mut self, labels: Vec<Label>) -> Diagnostic {
        self.labels = labels;
        self
    }

    /// Add some notes to the diagnostic.
    pub fn with_notes(mut self, notes: Vec<String>) -> Diagnostic {
        self.notes = notes;
        self
    }
}
