//! Double-buffered terminal cell storage with efficient diffing.

use crate::geometry::Rect;
use crate::style::Style;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use unicode_width::UnicodeWidthStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A single cell in the terminal buffer.
///
/// Each cell stores a grapheme cluster, style, and skip flag for wide characters.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cell {
    /// The symbol (grapheme cluster) to display
    pub symbol: String,
    /// The style for this cell
    pub style: Style,
    /// Skip rendering flag (for wide character continuations)
    pub skip: bool,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: String::from(" "),
            style: Style::default(),
            skip: false,
        }
    }
}

impl Cell {
    /// Create a new cell with the given symbol and style.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::buffer::Cell;
    /// use tuxtui_core::style::Style;
    ///
    /// let cell = Cell::new("x", Style::default());
    /// ```
    #[must_use]
    pub fn new(symbol: impl Into<String>, style: Style) -> Self {
        Self {
            symbol: symbol.into(),
            style,
            skip: false,
        }
    }

    /// Reset the cell to a space with default style.
    pub fn reset(&mut self) {
        self.symbol.clear();
        self.symbol.push(' ');
        self.style = Style::default();
        self.skip = false;
    }

    /// Set the symbol for this cell.
    pub fn set_symbol(&mut self, symbol: impl Into<String>) {
        self.symbol = symbol.into();
    }

    /// Set the style for this cell.
    pub fn set_style(&mut self, style: Style) {
        self.style = style;
    }

    /// Get the display width of the symbol (1 or 2 for wide characters).
    #[must_use]
    pub fn width(&self) -> usize {
        self.symbol.width()
    }
}

/// A buffer representing the terminal screen.
///
/// The buffer is a rectangular grid of [`Cell`]s that can be efficiently
/// diffed to minimize terminal updates.
///
/// # Example
///
/// ```
/// use tuxtui_core::buffer::Buffer;
/// use tuxtui_core::geometry::Rect;
/// use tuxtui_core::style::{Color, Style};
///
/// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
/// buffer.set_string(0, 0, "Hello", Style::default().fg(Color::Green));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Buffer {
    /// The area covered by this buffer
    pub area: Rect,
    /// The cells in this buffer (row-major order)
    pub content: Vec<Cell>,
}

impl Buffer {
    /// Create an empty buffer with the given area.
    ///
    /// All cells are initialized to spaces with default style.
    #[must_use]
    pub fn empty(area: Rect) -> Self {
        let cell_count = area.area() as usize;
        Self {
            area,
            content: vec![Cell::default(); cell_count],
        }
    }

    /// Create a buffer filled with a specific cell.
    #[must_use]
    pub fn filled(area: Rect, cell: &Cell) -> Self {
        let cell_count = area.area() as usize;
        Self {
            area,
            content: vec![cell.clone(); cell_count],
        }
    }

    /// Get the index into the content vector for the given coordinates.
    ///
    /// Returns `None` if the coordinates are out of bounds.
    #[must_use]
    pub const fn index_of(&self, x: u16, y: u16) -> Option<usize> {
        if x >= self.area.x
            && x < self.area.x + self.area.width
            && y >= self.area.y
            && y < self.area.y + self.area.height
        {
            let row = (y - self.area.y) as usize;
            let col = (x - self.area.x) as usize;
            Some(row * self.area.width as usize + col)
        } else {
            None
        }
    }

    /// Get a reference to the cell at the given coordinates.
    #[must_use]
    pub fn get(&self, x: u16, y: u16) -> Option<&Cell> {
        self.index_of(x, y).and_then(|i| self.content.get(i))
    }

    /// Get a mutable reference to the cell at the given coordinates.
    pub fn get_mut(&mut self, x: u16, y: u16) -> Option<&mut Cell> {
        if let Some(i) = self.index_of(x, y) {
            self.content.get_mut(i)
        } else {
            None
        }
    }

    /// Set the symbol and style of a cell at the given coordinates.
    ///
    /// Returns `true` if the cell was updated, `false` if out of bounds.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::buffer::Buffer;
    /// use tuxtui_core::geometry::Rect;
    /// use tuxtui_core::style::Style;
    ///
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
    /// buffer.set(5, 5, "X", Style::default());
    /// ```
    pub fn set(&mut self, x: u16, y: u16, symbol: impl Into<String>, style: Style) -> bool {
        if let Some(cell) = self.get_mut(x, y) {
            let symbol = symbol.into();
            let width = symbol.width();
            cell.symbol = symbol;
            cell.style = style;
            cell.skip = false;

            // Mark continuation cells for wide characters
            if width > 1 {
                for i in 1..width {
                    if let Some(next_cell) = self.get_mut(x + i as u16, y) {
                        next_cell.reset();
                        next_cell.skip = true;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    /// Set a string at the given position with a style.
    ///
    /// Returns the x-coordinate after the last written character.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::buffer::Buffer;
    /// use tuxtui_core::geometry::Rect;
    /// use tuxtui_core::style::{Color, Style};
    ///
    /// let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
    /// let style = Style::default().fg(Color::Blue);
    /// let end_x = buffer.set_string(0, 0, "Hello, world!", style);
    /// ```
    pub fn set_string(&mut self, x: u16, y: u16, string: &str, style: Style) -> u16 {
        let mut x = x;
        for grapheme in unicode_segmentation::UnicodeSegmentation::graphemes(string, true) {
            if x >= self.area.right() {
                break;
            }
            self.set(x, y, grapheme, style);
            x += grapheme.width() as u16;
        }
        x
    }

    /// Set a styled string with mixed styles (via spans).
    ///
    /// This is used internally by text rendering.
    pub fn set_styled_string(
        &mut self,
        x: u16,
        y: u16,
        string: &str,
        style: Style,
    ) -> u16 {
        self.set_string(x, y, string, style)
    }

    /// Clear the entire buffer.
    pub fn clear(&mut self) {
        for cell in &mut self.content {
            cell.reset();
        }
    }

    /// Clear a specific rectangular region.
    pub fn clear_region(&mut self, region: Rect) {
        let region = self.area.intersection(region);
        for y in region.top()..region.bottom() {
            for x in region.left()..region.right() {
                if let Some(cell) = self.get_mut(x, y) {
                    cell.reset();
                }
            }
        }
    }

    /// Set the style for subsequent operations (no-op for buffer).
    pub fn set_style(&mut self, _style: Style) {
        // Buffer doesn't have a global style, this is a no-op
    }

    /// Resize the buffer to a new area.
    ///
    /// Content is preserved where it overlaps; new areas are filled with default cells.
    pub fn resize(&mut self, area: Rect) {
        if area == self.area {
            return;
        }

        let mut new_buffer = Self::empty(area);
        let intersection = self.area.intersection(area);

        // Copy overlapping content
        for y in intersection.top()..intersection.bottom() {
            for x in intersection.left()..intersection.right() {
                if let Some(cell) = self.get(x, y) {
                    if let Some(idx) = new_buffer.index_of(x, y) {
                        new_buffer.content[idx] = cell.clone();
                    }
                }
            }
        }

        *self = new_buffer;
    }

    /// Merge another buffer into this one at the specified position.
    pub fn merge(&mut self, other: &Self) {
        let area = self.area.intersection(other.area);
        for y in area.top()..area.bottom() {
            for x in area.left()..area.right() {
                if let Some(cell) = other.get(x, y) {
                    if !cell.skip {
                        self.set(x, y, cell.symbol.as_str(), cell.style);
                    }
                }
            }
        }
    }

    /// Compute the differences between this buffer and another.
    ///
    /// Returns a vector of `Diff` operations representing the minimal changes.
    #[must_use]
    pub fn diff<'a>(&'a self, other: &'a Self) -> Vec<Diff<'a>> {
        let mut diffs = Vec::new();

        if self.area != other.area {
            // If areas differ, return a full redraw
            for y in other.area.top()..other.area.bottom() {
                let mut start_x = None;
                let mut current_style = None;

                for x in other.area.left()..other.area.right() {
                    if let Some(cell) = other.get(x, y) {
                        if cell.skip {
                            continue;
                        }

                        if start_x.is_none() {
                            start_x = Some(x);
                            current_style = Some(cell.style);
                        }

                        if Some(cell.style) != current_style {
                            // Style changed, flush current run
                            if let Some(sx) = start_x {
                                diffs.push(Diff {
                                    x: sx,
                                    y,
                                    cells: Vec::new(), // Simplified for now
                                });
                            }
                            start_x = Some(x);
                            current_style = Some(cell.style);
                        }
                    }
                }
            }
            return diffs;
        }

        // Row-by-row diff
        for y in self.area.top()..self.area.bottom() {
            let mut x = self.area.left();
            while x < self.area.right() {
                let old_cell = self.get(x, y);
                let new_cell = other.get(x, y);

                if old_cell != new_cell {
                    if let Some(new_cell) = new_cell {
                        diffs.push(Diff {
                            x,
                            y,
                            cells: alloc::vec![new_cell],
                        });
                    }
                }
                x += 1;
            }
        }

        diffs
    }
}

impl fmt::Display for Buffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in self.area.top()..self.area.bottom() {
            for x in self.area.left()..self.area.right() {
                if let Some(cell) = self.get(x, y) {
                    if !cell.skip {
                        write!(f, "{}", cell.symbol)?;
                    }
                }
            }
            if y < self.area.bottom() - 1 {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// A diff operation representing changes between two buffers.
#[derive(Debug, Clone)]
pub struct Diff<'a> {
    /// X coordinate
    pub x: u16,
    /// Y coordinate
    pub y: u16,
    /// Cells that changed at this position
    pub cells: Vec<&'a Cell>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_buffer_set_get() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        buffer.set(5, 5, "X", Style::default());
        
        let cell = buffer.get(5, 5).unwrap();
        assert_eq!(cell.symbol, "X");
    }

    #[test]
    fn test_buffer_set_string() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 20, 5));
        let end_x = buffer.set_string(0, 0, "Hello", Style::default());
        
        assert_eq!(end_x, 5);
        assert_eq!(buffer.get(0, 0).unwrap().symbol, "H");
        assert_eq!(buffer.get(4, 0).unwrap().symbol, "o");
    }

    #[test]
    fn test_buffer_clear() {
        let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 10));
        buffer.set(5, 5, "X", Style::default().fg(Color::Red));
        buffer.clear();
        
        let cell = buffer.get(5, 5).unwrap();
        assert_eq!(cell.symbol, " ");
        assert_eq!(cell.style, Style::default());
    }

    #[test]
    fn test_buffer_merge() {
        let mut base = Buffer::empty(Rect::new(0, 0, 10, 10));
        let mut overlay = Buffer::empty(Rect::new(0, 0, 10, 10));
        
        overlay.set(5, 5, "O", Style::default());
        base.merge(&overlay);
        
        assert_eq!(base.get(5, 5).unwrap().symbol, "O");
    }
}
