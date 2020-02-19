use std::io;
use termcolor::WriteColor;

use crate::diagnostic::{Severity, Title};
use crate::term::Config;

use super::NewLine;

/// Diagnostic header.
///
/// ```text
/// error[E0001]: unexpected type in `+` application
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Header<'a> {
    severity: Severity,
    title: &'a Title,
}

impl<'a> Header<'a> {
    pub fn new(severity: Severity, title: &'a Title) -> Header<'a> {
        Header { severity, title }
    }

    fn severity_name(&self) -> &'static str {
        match self.severity {
            Severity::Bug => "bug",
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Help => "help",
            Severity::Note => "note",
        }
    }

    pub fn emit(&self, writer: &mut (impl WriteColor + ?Sized), config: &Config) -> io::Result<()> {
        // Write severity name
        //
        // ```text
        // error
        // ```
        writer.set_color(config.styles.header(self.severity))?;
        write!(writer, "{}", self.severity_name())?;
        if let Some(code) = &self.title.code {
            // Write error code
            //
            // ```text
            // [E0001]
            // ```
            write!(writer, "[{}]", code)?;
        }

        // Write diagnostic message
        //
        // ```text
        // : unexpected type in `+` application
        // ```
        writer.set_color(&config.styles.header_message)?;
        write!(writer, ": {}", self.title.message)?;
        writer.reset()?;

        NewLine::new().emit(writer, config)?;

        Ok(())
    }
}
