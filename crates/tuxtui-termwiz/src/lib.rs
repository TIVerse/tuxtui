//! # tuxtui-termwiz
//!
//! Termwiz backend implementation for tuxtui.
//!
//! This crate provides a backend implementation using the `termwiz` crate.

#![forbid(unsafe_code)]
#![warn(missing_docs)]

use std::io;
use tuxtui_core::backend::Backend;
use tuxtui_core::buffer::Cell;
use tuxtui_core::geometry::{Position, Rect};
use tuxtui_core::style::Style;

/// Termwiz backend (stub implementation).
pub struct TermwizBackend;

impl TermwizBackend {
    /// Create a new termwiz backend.
    pub fn new() -> Self {
        Self
    }
}

impl Default for TermwizBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl Backend for TermwizBackend {
    type Error = io::Error;

    fn size(&self) -> Result<Rect, Self::Error> {
        Ok(Rect::new(0, 0, 80, 24))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<Position, Self::Error> {
        Ok(Position::new(0, 0))
    }

    fn set_cursor(&mut self, _x: u16, _y: u16) -> Result<(), Self::Error> {
        Ok(())
    }

    fn draw_cell(&mut self, _x: u16, _y: u16, _cell: &Cell) -> Result<(), Self::Error> {
        Ok(())
    }

    fn set_style(&mut self, _style: Style) -> Result<(), Self::Error> {
        Ok(())
    }

    fn reset_style(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enable_raw_mode(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn disable_raw_mode(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn enter_alternate_screen(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }

    fn leave_alternate_screen(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}
