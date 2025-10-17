//! # tuxtui-crossterm
//!
//! Crossterm backend implementation for tuxtui.
//!
//! This crate provides a backend implementation using the `crossterm` crate
//! for cross-platform terminal manipulation.
//!
//! ## Features
//!
//! - `crossterm_0_28`: Use crossterm 0.28
//! - `crossterm_0_29` (default): Use crossterm 0.29
//! - `serde`: Enable serialization support
//! - `underline-color`: Enable colored underlines
//! - `scrolling-regions`: Enable scrolling region support
//! - `unstable`: Enable unstable features
//! - `unstable-backend-writer`: Enable unstable backend writer API
//!
//! ## Example
//!
//! ```no_run
//! use tuxtui_crossterm::CrosstermBackend;
//! use tuxtui_core::terminal::Terminal;
//! use std::io::stdout;
//!
//! let backend = CrosstermBackend::new(stdout());
//! let mut terminal = Terminal::new(backend).unwrap();
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use crossterm::{
    cursor, execute, queue,
    style::{self, Attribute, Color as CColor, SetAttribute, SetBackgroundColor, SetForegroundColor},
    terminal::{self, Clear, ClearType},
};
use std::io::{self, Write};
use std::vec;
use std::format;
use tuxtui_core::backend::Backend;
use tuxtui_core::buffer::Cell;
use tuxtui_core::geometry::{Position, Rect};
use tuxtui_core::style::{Color, Modifier, Style};

/// Crossterm backend.
///
/// Wraps a writer (typically stdout) and uses crossterm for terminal operations.
pub struct CrosstermBackend<W: Write> {
    writer: W,
}

impl<W: Write> CrosstermBackend<W> {
    /// Create a new crossterm backend.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use tuxtui_crossterm::CrosstermBackend;
    /// use std::io::stdout;
    ///
    /// let backend = CrosstermBackend::new(stdout());
    /// ```
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    /// Get a reference to the writer.
    pub fn writer(&self) -> &W {
        &self.writer
    }

    /// Get a mutable reference to the writer.
    pub fn writer_mut(&mut self) -> &mut W {
        &mut self.writer
    }

    fn convert_color(color: Color) -> CColor {
        match color {
            Color::Reset => CColor::Reset,
            Color::Black => CColor::Black,
            Color::Red => CColor::DarkRed,
            Color::Green => CColor::DarkGreen,
            Color::Yellow => CColor::DarkYellow,
            Color::Blue => CColor::DarkBlue,
            Color::Magenta => CColor::DarkMagenta,
            Color::Cyan => CColor::DarkCyan,
            Color::White => CColor::Grey,
            Color::Gray => CColor::DarkGrey,
            Color::LightRed => CColor::Red,
            Color::LightGreen => CColor::Green,
            Color::LightYellow => CColor::Yellow,
            Color::LightBlue => CColor::Blue,
            Color::LightMagenta => CColor::Magenta,
            Color::LightCyan => CColor::Cyan,
            Color::LightGray => CColor::White,
            Color::Indexed(i) => CColor::AnsiValue(i),
            Color::Rgb(r, g, b) => CColor::Rgb { r, g, b },
        }
    }

    fn apply_modifiers(&mut self, modifiers: Modifier) -> io::Result<()> {
        if modifiers.contains(Modifier::BOLD) {
            queue!(self.writer, SetAttribute(Attribute::Bold))?;
        }
        if modifiers.contains(Modifier::DIM) {
            queue!(self.writer, SetAttribute(Attribute::Dim))?;
        }
        if modifiers.contains(Modifier::ITALIC) {
            queue!(self.writer, SetAttribute(Attribute::Italic))?;
        }
        if modifiers.contains(Modifier::UNDERLINED) {
            queue!(self.writer, SetAttribute(Attribute::Underlined))?;
        }
        if modifiers.contains(Modifier::SLOW_BLINK) {
            queue!(self.writer, SetAttribute(Attribute::SlowBlink))?;
        }
        if modifiers.contains(Modifier::RAPID_BLINK) {
            queue!(self.writer, SetAttribute(Attribute::RapidBlink))?;
        }
        if modifiers.contains(Modifier::REVERSED) {
            queue!(self.writer, SetAttribute(Attribute::Reverse))?;
        }
        if modifiers.contains(Modifier::HIDDEN) {
            queue!(self.writer, SetAttribute(Attribute::Hidden))?;
        }
        if modifiers.contains(Modifier::CROSSED_OUT) {
            queue!(self.writer, SetAttribute(Attribute::CrossedOut))?;
        }
        Ok(())
    }
}

impl<W: Write> Backend for CrosstermBackend<W> {
    type Error = io::Error;

    fn size(&self) -> Result<Rect, Self::Error> {
        let (width, height) = terminal::size()?;
        Ok(Rect::new(0, 0, width, height))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        execute!(self.writer, Clear(ClearType::All))
    }

    fn clear_region(&mut self, region: Rect) -> Result<(), Self::Error> {
        for y in region.top()..region.bottom() {
            queue!(self.writer, cursor::MoveTo(region.left(), y))?;
            for _ in region.left()..region.right() {
                queue!(self.writer, style::Print(" "))?;
            }
        }
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        execute!(self.writer, cursor::Hide)
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        execute!(self.writer, cursor::Show)
    }

    fn get_cursor(&mut self) -> Result<Position, Self::Error> {
        // Crossterm doesn't have a simple position() function
        // We'll return a default for now
        Ok(Position::new(0, 0))
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error> {
        queue!(self.writer, cursor::MoveTo(x, y))?;
        Ok(())
    }

    fn draw_cell(&mut self, x: u16, y: u16, cell: &Cell) -> Result<(), Self::Error> {
        if cell.skip {
            return Ok(());
        }

        queue!(self.writer, cursor::MoveTo(x, y))?;

        if let Some(fg) = cell.style.fg {
            queue!(self.writer, SetForegroundColor(Self::convert_color(fg)))?;
        }
        if let Some(bg) = cell.style.bg {
            queue!(self.writer, SetBackgroundColor(Self::convert_color(bg)))?;
        }

        self.apply_modifiers(cell.style.add_modifier)?;

        queue!(self.writer, style::Print(&cell.symbol))?;

        // Reset if we applied any modifiers
        if !cell.style.add_modifier.is_empty() || cell.style.fg.is_some() || cell.style.bg.is_some() {
            queue!(self.writer, SetAttribute(Attribute::Reset))?;
        }

        Ok(())
    }

    fn set_style(&mut self, style: Style) -> Result<(), Self::Error> {
        if let Some(fg) = style.fg {
            queue!(self.writer, SetForegroundColor(Self::convert_color(fg)))?;
        }
        if let Some(bg) = style.bg {
            queue!(self.writer, SetBackgroundColor(Self::convert_color(bg)))?;
        }
        self.apply_modifiers(style.add_modifier)?;
        Ok(())
    }

    fn reset_style(&mut self) -> Result<(), Self::Error> {
        queue!(self.writer, SetAttribute(Attribute::Reset))?;
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.writer.flush()
    }

    fn enable_raw_mode(&mut self) -> Result<(), Self::Error> {
        terminal::enable_raw_mode()
    }

    fn disable_raw_mode(&mut self) -> Result<(), Self::Error> {
        terminal::disable_raw_mode()
    }

    fn enter_alternate_screen(&mut self) -> Result<(), Self::Error> {
        execute!(self.writer, terminal::EnterAlternateScreen)
    }

    fn leave_alternate_screen(&mut self) -> Result<(), Self::Error> {
        execute!(self.writer, terminal::LeaveAlternateScreen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_backend_creation() {
        let cursor = Cursor::new(Vec::new());
        let backend = CrosstermBackend::new(cursor);
        assert!(backend.writer().get_ref().is_empty());
    }

    #[test]
    fn test_color_conversion() {
        assert!(matches!(
            CrosstermBackend::<Vec<u8>>::convert_color(Color::Red),
            CColor::DarkRed
        ));
        assert!(matches!(
            CrosstermBackend::<Vec<u8>>::convert_color(Color::Rgb(255, 128, 0)),
            CColor::Rgb { r: 255, g: 128, b: 0 }
        ));
    }
}
