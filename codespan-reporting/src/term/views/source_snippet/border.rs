use codespan::LineNumber;
use std::io;
use termcolor::WriteColor;

use crate::term::Config;

/// The top-left corner of a source line.
///
/// ```text
/// ┌──
/// ```
pub struct BorderTopLeft {
    gutter_padding: usize,
}

impl BorderTopLeft {
    pub fn new(gutter_padding: usize) -> BorderTopLeft {
        BorderTopLeft { gutter_padding }
    }

    pub fn emit(&self, writer: &mut impl WriteColor, config: &Config) -> io::Result<()> {
        let pad = self.gutter_padding;
        let top_left = config.source_border_top_left_char;
        let top = config.source_border_top_char;

        write!(writer, "{space: >pad$}", space = "", pad = pad + 1)?;

        writer.set_color(&config.styles.source_border)?;
        write!(writer, "{top_left}", top_left = top_left)?;
        write!(writer, "{top}{top}", top = top)?;
        writer.reset()?;

        write!(writer, " ")?;

        Ok(())
    }
}

/// The top border of a source line.
///
/// ```text
/// ───
/// ```
pub struct BorderTop {}

impl BorderTop {
    pub fn new() -> BorderTop {
        BorderTop {}
    }

    pub fn emit(&self, writer: &mut impl WriteColor, config: &Config) -> io::Result<()> {
        let top = config.source_border_top_char;

        write!(writer, " ")?;
        writer.set_color(&config.styles.source_border)?;
        write!(writer, "{top}{top}{top}", top = top)?;
        writer.reset()?;

        Ok(())
    }
}

/// The left-hand border of a source line.
///
/// ```text
///  23 │
/// ```
pub struct BorderLeft {
    line_number: Option<LineNumber>,
    gutter_padding: usize,
}

impl BorderLeft {
    pub fn new(line_number: impl Into<Option<LineNumber>>, gutter_padding: usize) -> BorderLeft {
        let line_number = line_number.into();
        BorderLeft {
            line_number,
            gutter_padding,
        }
    }

    pub fn emit(&self, writer: &mut impl WriteColor, config: &Config) -> io::Result<()> {
        let pad = self.gutter_padding;
        match self.line_number {
            None => {
                write!(writer, "{space: >pad$}", space = "", pad = pad)?;
            },
            Some(line_number) => {
                writer.set_color(&config.styles.line_number)?;
                write!(writer, "{line: >pad$}", line = line_number, pad = pad)?;
                writer.reset()?;
            },
        }
        write!(writer, " ")?;

        writer.set_color(&config.styles.source_border)?;
        write!(writer, "{left}", left = config.source_border_left_char)?;
        writer.reset()?;

        Ok(())
    }
}
