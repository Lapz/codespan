//! Terminal back-end for emitting diagnostics.

use codespan::Files;
use std::io;
use std::str::FromStr;
use termcolor::{ColorChoice, WriteColor};

use crate::diagnostic::{Diagnostic, LabelStyle};

mod config;
mod views;

pub use termcolor;

pub use self::config::{Chars, Config, Styles};

/// Emit a diagnostic using the given writer, context, config, and files.
pub fn emit<Source: AsRef<str>>(
    writer: &mut (impl WriteColor + ?Sized),
    config: &Config,
    files: &Files<Source>,
    diagnostic: &Diagnostic,
) -> io::Result<()> {
    use std::collections::BTreeMap;

    use self::views::MarkStyle;
    use self::views::{Header, NewLine, Note, SourceSnippet};

    // Emit the title
    //
    // ```text
    // error[E0001]: unexpected type in `+` application
    // ```
    if let Some(title) = &diagnostic.title {
        Header::new(diagnostic.severity, title).emit(writer, config)?;
        NewLine::new().emit(writer, config)?;
    }

    // Group labels by file

    let mut label_groups = BTreeMap::new();

    for label in &diagnostic.labels {
        let mark_style = match label.style {
            LabelStyle::Primary => MarkStyle::Primary(diagnostic.severity),
            LabelStyle::Secondary => MarkStyle::Secondary,
        };
        label_groups
            .entry(label.file_id)
            .or_insert(vec![])
            .push((label, mark_style));
    }

    // Emit the snippets, starting with the one that contains the primary label

    for (file_id, labels) in label_groups {
        // FIXME: consistent gutter padding
        SourceSnippet::new(file_id, labels).emit(files, writer, config)?;
    }

    // Additional notes
    //
    // ```text
    // = expected type `Int`
    //      found type `String`
    // ```
    for note in &diagnostic.notes {
        let gutter_padding = 0; // TODO: use gutter padding from emitting source snippets
        Note::new(gutter_padding, &note).emit(writer, config)?;
    }
    NewLine::new().emit(writer, config)?;

    Ok(())
}

/// A command line argument that configures the coloring of the output.
///
/// This can be used with command line argument parsers like `clap` or `structopt`.
///
/// # Example
///
/// ```rust
/// use structopt::StructOpt;
/// use codespan_reporting::term::termcolor::StandardStream;
/// use codespan_reporting::term::ColorArg;
///
/// #[derive(Debug, StructOpt)]
/// #[structopt(name = "groovey-app")]
/// pub struct Opts {
///     /// Configure coloring of output
///     #[structopt(
///         long = "color",
///         parse(try_from_str),
///         default_value = "auto",
///         possible_values = ColorArg::VARIANTS,
///         case_insensitive = true
///     )]
///     pub color: ColorArg,
/// }
///
/// fn main() {
///     let opts = Opts::from_args();
///     let writer = StandardStream::stderr(opts.color.into());
/// }
/// ```
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ColorArg(pub ColorChoice);

impl ColorArg {
    /// Allowed values the argument.
    ///
    /// This is useful for generating documentation via `clap` or `structopt`'s
    /// `possible_values` configuration.
    pub const VARIANTS: &'static [&'static str] = &["auto", "always", "ansi", "never"];
}

impl FromStr for ColorArg {
    type Err = &'static str;

    fn from_str(src: &str) -> Result<ColorArg, &'static str> {
        match src {
            _ if src.eq_ignore_ascii_case("auto") => Ok(ColorArg(ColorChoice::Auto)),
            _ if src.eq_ignore_ascii_case("always") => Ok(ColorArg(ColorChoice::Always)),
            _ if src.eq_ignore_ascii_case("ansi") => Ok(ColorArg(ColorChoice::AlwaysAnsi)),
            _ if src.eq_ignore_ascii_case("never") => Ok(ColorArg(ColorChoice::Never)),
            _ => Err("valid values: auto, always, ansi, never"),
        }
    }
}

impl Into<ColorChoice> for ColorArg {
    fn into(self) -> ColorChoice {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::diagnostic::Label;

    #[test]
    fn unsized_emit() {
        let mut files = Files::new();

        let id = files.add("test", "");
        emit(
            &mut termcolor::NoColor::new(Vec::<u8>::new()) as &mut dyn WriteColor,
            &Config::default(),
            &files,
            &Diagnostic::bug().with_labels(vec![Label::primary(id, codespan::Span::default(), "")]),
        )
        .unwrap();
    }
}
