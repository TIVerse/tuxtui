//! # tuxtui-termion
//!
//! Termion backend implementation for tuxtui.
//!
//! This crate provides a backend implementation using the `termion` crate
//! for Unix-based terminal manipulation.
//!
//! ## Example
//!
//! ```no_run
//! use tuxtui_termion::TermionBackend;
//! use tuxtui_core::terminal::Terminal;
//! use std::io::stdout;
//!
//! let backend = TermionBackend::new(stdout());
//! let mut terminal = Terminal::new(backend).unwrap();
//! ```

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io::{self, Write};
use termion::{clear, cursor, style};
use tuxtui_core::backend::Backend;
use tuxtui_core::buffer::Cell;
use tuxtui_core::geometry::{Position, Rect};
use tuxtui_core::style::{Color as TuxColor, Modifier, Style};

/// Termion backend.
pub struct TermionBackend<W: Write> {
    writer: W,
}

impl<W: Write> TermionBackend<W> {
    /// Create a new termion backend.
    pub fn new(writer: W) -> Self {
        Self { writer }
    }

    fn convert_fg_color(&mut self, color: TuxColor) -> io::Result<()> {
        use termion::color::*;
        match color {
            TuxColor::Reset => write!(self.writer, "{}", Fg(Reset)),
            TuxColor::Black => write!(self.writer, "{}", Fg(Black)),
            TuxColor::Red => write!(self.writer, "{}", Fg(Red)),
            TuxColor::Green => write!(self.writer, "{}", Fg(Green)),
            TuxColor::Yellow => write!(self.writer, "{}", Fg(Yellow)),
            TuxColor::Blue => write!(self.writer, "{}", Fg(Blue)),
            TuxColor::Magenta => write!(self.writer, "{}", Fg(Magenta)),
            TuxColor::Cyan => write!(self.writer, "{}", Fg(Cyan)),
            TuxColor::White | TuxColor::Gray => write!(self.writer, "{}", Fg(White)),
            TuxColor::LightRed => write!(self.writer, "{}", Fg(LightRed)),
            TuxColor::LightGreen => write!(self.writer, "{}", Fg(LightGreen)),
            TuxColor::LightYellow => write!(self.writer, "{}", Fg(LightYellow)),
            TuxColor::LightBlue => write!(self.writer, "{}", Fg(LightBlue)),
            TuxColor::LightMagenta => write!(self.writer, "{}", Fg(LightMagenta)),
            TuxColor::LightCyan => write!(self.writer, "{}", Fg(LightCyan)),
            TuxColor::LightGray => write!(self.writer, "{}", Fg(LightWhite)),
            TuxColor::Indexed(i) => write!(self.writer, "{}", Fg(AnsiValue(i))),
            TuxColor::Rgb(r, g, b) => write!(self.writer, "{}", Fg(Rgb(r, g, b))),
        }
    }

    fn convert_bg_color(&mut self, color: TuxColor) -> io::Result<()> {
        use termion::color::*;
        match color {
            TuxColor::Reset => write!(self.writer, "{}", Bg(Reset)),
            TuxColor::Black => write!(self.writer, "{}", Bg(Black)),
            TuxColor::Red => write!(self.writer, "{}", Bg(Red)),
            TuxColor::Green => write!(self.writer, "{}", Bg(Green)),
            TuxColor::Yellow => write!(self.writer, "{}", Bg(Yellow)),
            TuxColor::Blue => write!(self.writer, "{}", Bg(Blue)),
            TuxColor::Magenta => write!(self.writer, "{}", Bg(Magenta)),
            TuxColor::Cyan => write!(self.writer, "{}", Bg(Cyan)),
            TuxColor::White | TuxColor::Gray => write!(self.writer, "{}", Bg(White)),
            TuxColor::LightRed => write!(self.writer, "{}", Bg(LightRed)),
            TuxColor::LightGreen => write!(self.writer, "{}", Bg(LightGreen)),
            TuxColor::LightYellow => write!(self.writer, "{}", Bg(LightYellow)),
            TuxColor::LightBlue => write!(self.writer, "{}", Bg(LightBlue)),
            TuxColor::LightMagenta => write!(self.writer, "{}", Bg(LightMagenta)),
            TuxColor::LightCyan => write!(self.writer, "{}", Bg(LightCyan)),
            TuxColor::LightGray => write!(self.writer, "{}", Bg(LightWhite)),
            TuxColor::Indexed(i) => write!(self.writer, "{}", Bg(AnsiValue(i))),
            TuxColor::Rgb(r, g, b) => write!(self.writer, "{}", Bg(Rgb(r, g, b))),
        }
    }

    fn apply_modifiers(&mut self, modifiers: Modifier) -> io::Result<()> {
        if modifiers.contains(Modifier::BOLD) {
            write!(self.writer, "{}", style::Bold)?;
        }
        if modifiers.contains(Modifier::DIM) {
            write!(self.writer, "{}", style::Faint)?;
        }
        if modifiers.contains(Modifier::ITALIC) {
            write!(self.writer, "{}", style::Italic)?;
        }
        if modifiers.contains(Modifier::UNDERLINED) {
            write!(self.writer, "{}", style::Underline)?;
        }
        if modifiers.contains(Modifier::SLOW_BLINK) {
            write!(self.writer, "{}", style::Blink)?;
        }
        if modifiers.contains(Modifier::REVERSED) {
            write!(self.writer, "{}", style::Invert)?;
        }
        if modifiers.contains(Modifier::CROSSED_OUT) {
            write!(self.writer, "{}", style::CrossedOut)?;
        }
        Ok(())
    }
}

impl<W: Write> Backend for TermionBackend<W> {
    type Error = io::Error;

    fn size(&self) -> Result<Rect, Self::Error> {
        let (width, height) = termion::terminal_size()?;
        Ok(Rect::new(0, 0, width, height))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", clear::All)
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", cursor::Hide)
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", cursor::Show)
    }

    fn get_cursor(&mut self) -> Result<Position, Self::Error> {
        Ok(Position::new(0, 0)) // Termion doesn't easily support cursor position query
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error> {
        write!(self.writer, "{}", cursor::Goto(x + 1, y + 1))
    }

    fn draw_cell(&mut self, x: u16, y: u16, cell: &Cell) -> Result<(), Self::Error> {
        if cell.skip {
            return Ok(());
        }

        write!(self.writer, "{}", cursor::Goto(x + 1, y + 1))?;

        if let Some(fg) = cell.style.fg {
            self.convert_fg_color(fg)?;
        }
        if let Some(bg) = cell.style.bg {
            self.convert_bg_color(bg)?;
        }

        self.apply_modifiers(cell.style.add_modifier)?;

        write!(self.writer, "{}", cell.symbol)?;
        write!(self.writer, "{}", style::Reset)?;

        Ok(())
    }

    fn set_style(&mut self, style: Style) -> Result<(), Self::Error> {
        if let Some(fg) = style.fg {
            self.convert_fg_color(fg)?;
        }
        if let Some(bg) = style.bg {
            self.convert_bg_color(bg)?;
        }
        self.apply_modifiers(style.add_modifier)?;
        Ok(())
    }

    fn reset_style(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", style::Reset)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.writer.flush()
    }

    fn enable_raw_mode(&mut self) -> Result<(), Self::Error> {
        // Termion handles raw mode through RawTerminal wrapper
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> Result<(), Self::Error> {
        // Termion handles raw mode through RawTerminal wrapper
        Ok(())
    }

    fn enter_alternate_screen(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", termion::screen::ToAlternateScreen)
    }

    fn leave_alternate_screen(&mut self) -> Result<(), Self::Error> {
        write!(self.writer, "{}", termion::screen::ToMainScreen)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_creation() {
        let buffer = Vec::new();
        let _backend = TermionBackend::new(buffer);
    }
}
