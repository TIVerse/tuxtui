//! Platform-agnostic terminal backend trait.

use crate::buffer::{Buffer, Cell};
use crate::geometry::{Position, Rect};
use crate::style::Style;
use core::fmt;

/// A terminal backend abstraction.
///
/// Backends implement low-level terminal operations like clearing, cursor
/// management, and cell rendering.
///
/// # Example
///
/// ```ignore
/// use tuxtui_core::backend::Backend;
///
/// fn render<B: Backend>(backend: &mut B) -> Result<(), B::Error> {
///     backend.clear()?;
///     backend.set_cursor(0, 0)?;
///     backend.flush()?;
///     Ok(())
/// }
/// ```
pub trait Backend {
    /// The error type for this backend.
    type Error: fmt::Debug + fmt::Display;

    /// Get the size of the terminal in columns and rows.
    fn size(&self) -> Result<Rect, Self::Error>;

    /// Clear the entire screen.
    fn clear(&mut self) -> Result<(), Self::Error>;

    /// Clear a specific region.
    fn clear_region(&mut self, region: Rect) -> Result<(), Self::Error> {
        // Default implementation using individual cell writes
        for y in region.top()..region.bottom() {
            for x in region.left()..region.right() {
                self.draw_cell(x, y, &Cell::default())?;
            }
        }
        Ok(())
    }

    /// Hide the cursor.
    fn hide_cursor(&mut self) -> Result<(), Self::Error>;

    /// Show the cursor.
    fn show_cursor(&mut self) -> Result<(), Self::Error>;

    /// Get the current cursor position.
    fn get_cursor(&mut self) -> Result<Position, Self::Error>;

    /// Set the cursor position.
    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error>;

    /// Draw a single cell at the given position.
    fn draw_cell(&mut self, x: u16, y: u16, cell: &Cell) -> Result<(), Self::Error>;

    /// Set the current style for subsequent operations.
    fn set_style(&mut self, style: Style) -> Result<(), Self::Error>;

    /// Reset all styling to default.
    fn reset_style(&mut self) -> Result<(), Self::Error>;

    /// Flush any buffered output to the terminal.
    fn flush(&mut self) -> Result<(), Self::Error>;

    /// Enable raw mode.
    fn enable_raw_mode(&mut self) -> Result<(), Self::Error>;

    /// Disable raw mode.
    fn disable_raw_mode(&mut self) -> Result<(), Self::Error>;

    /// Enter alternate screen.
    fn enter_alternate_screen(&mut self) -> Result<(), Self::Error>;

    /// Leave alternate screen.
    fn leave_alternate_screen(&mut self) -> Result<(), Self::Error>;

    /// Enable mouse capture (if supported).
    #[cfg(feature = "scrolling-regions")]
    fn set_scroll_region(&mut self, top: u16, bottom: u16) -> Result<(), Self::Error> {
        let _ = (top, bottom);
        Ok(())
    }

    /// Clear scroll region (if supported).
    #[cfg(feature = "scrolling-regions")]
    fn clear_scroll_region(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

/// A test backend for unit testing and snapshot testing.
///
/// Records operations and maintains a virtual terminal buffer.
///
/// # Example
///
/// ```
/// use tuxtui_core::backend::{Backend, TestBackend};
/// use tuxtui_core::geometry::Rect;
///
/// let mut backend = TestBackend::new(80, 24);
/// backend.clear().unwrap();
/// let size = backend.size().unwrap();
/// assert_eq!(size.width, 80);
/// assert_eq!(size.height, 24);
/// ```
pub struct TestBackend {
    width: u16,
    height: u16,
    buffer: Buffer,
    cursor_visible: bool,
    cursor_position: Position,
}

impl TestBackend {
    /// Create a new test backend with the given dimensions.
    #[must_use]
    pub fn new(width: u16, height: u16) -> Self {
        Self {
            width,
            height,
            buffer: Buffer::empty(Rect::new(0, 0, width, height)),
            cursor_visible: true,
            cursor_position: Position::new(0, 0),
        }
    }

    /// Get the current buffer content.
    #[must_use]
    pub fn buffer(&self) -> &Buffer {
        &self.buffer
    }

    /// Get a mutable reference to the buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        &mut self.buffer
    }

    /// Resize the test backend.
    pub fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.buffer.resize(Rect::new(0, 0, width, height));
    }

    /// Get the cursor visibility state.
    #[must_use]
    pub const fn is_cursor_visible(&self) -> bool {
        self.cursor_visible
    }

    /// Assert that the buffer contains the expected string at the given position.
    ///
    /// # Panics
    ///
    /// Panics if the buffer content doesn't match.
    pub fn assert_buffer_equals(&self, expected: &str) {
        let actual = format!("{}", self.buffer);
        assert_eq!(actual.trim(), expected.trim(), "Buffer mismatch");
    }
}

impl Backend for TestBackend {
    type Error = TestBackendError;

    fn size(&self) -> Result<Rect, Self::Error> {
        Ok(Rect::new(0, 0, self.width, self.height))
    }

    fn clear(&mut self) -> Result<(), Self::Error> {
        self.buffer.clear();
        Ok(())
    }

    fn clear_region(&mut self, region: Rect) -> Result<(), Self::Error> {
        self.buffer.clear_region(region);
        Ok(())
    }

    fn hide_cursor(&mut self) -> Result<(), Self::Error> {
        self.cursor_visible = false;
        Ok(())
    }

    fn show_cursor(&mut self) -> Result<(), Self::Error> {
        self.cursor_visible = true;
        Ok(())
    }

    fn get_cursor(&mut self) -> Result<Position, Self::Error> {
        Ok(self.cursor_position)
    }

    fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), Self::Error> {
        self.cursor_position = Position::new(x, y);
        Ok(())
    }

    fn draw_cell(&mut self, x: u16, y: u16, cell: &Cell) -> Result<(), Self::Error> {
        self.buffer.set(x, y, cell.symbol.as_str(), cell.style);
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

/// Error type for test backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TestBackendError {
    /// Generic error
    Generic(alloc::string::String),
}

impl fmt::Display for TestBackendError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic(msg) => write!(f, "{msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for TestBackendError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_backend_size() {
        let backend = TestBackend::new(80, 24);
        let size = backend.size().unwrap();
        assert_eq!(size.width, 80);
        assert_eq!(size.height, 24);
    }

    #[test]
    fn test_backend_cursor() {
        let mut backend = TestBackend::new(80, 24);
        backend.set_cursor(10, 5).unwrap();
        let pos = backend.get_cursor().unwrap();
        assert_eq!(pos.x, 10);
        assert_eq!(pos.y, 5);
    }

    #[test]
    fn test_backend_clear() {
        let mut backend = TestBackend::new(10, 5);
        backend.buffer_mut().set(0, 0, "X", Style::default());
        backend.clear().unwrap();
        assert_eq!(backend.buffer().get(0, 0).unwrap().symbol, " ");
    }
}
