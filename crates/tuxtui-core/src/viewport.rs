//! Viewport state management for scrollable content.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// State for managing a scrollable viewport.
///
/// This is useful for implementing scrolling in widgets like paragraphs,
/// lists, and tables.
///
/// # Example
///
/// ```
/// use tuxtui_core::viewport::ViewportState;
///
/// let mut viewport = ViewportState::new()
///     .content_length(100)
///     .viewport_height(20);
///
/// viewport.scroll_down();
/// assert_eq!(viewport.offset(), 1);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ViewportState {
    /// Current scroll offset (first visible line/item)
    offset: usize,
    /// Total content length (total lines/items)
    content_length: usize,
    /// Viewport height (visible lines/items)
    viewport_height: usize,
    /// Selected item index (optional)
    selected: Option<usize>,
}

impl Default for ViewportState {
    fn default() -> Self {
        Self::new()
    }
}

impl ViewportState {
    /// Create a new viewport state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            offset: 0,
            content_length: 0,
            viewport_height: 0,
            selected: None,
        }
    }

    /// Set the total content length.
    #[must_use]
    pub const fn content_length(mut self, length: usize) -> Self {
        self.content_length = length;
        self
    }

    /// Set the viewport height.
    #[must_use]
    pub const fn viewport_height(mut self, height: usize) -> Self {
        self.viewport_height = height;
        self
    }

    /// Get the current offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Set the offset directly.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset.min(self.max_offset());
    }

    /// Get the maximum possible offset.
    #[must_use]
    pub const fn max_offset(&self) -> usize {
        if self.content_length > self.viewport_height {
            self.content_length - self.viewport_height
        } else {
            0
        }
    }

    /// Scroll down by one line.
    pub fn scroll_down(&mut self) {
        if self.offset < self.max_offset() {
            self.offset += 1;
        }
    }

    /// Scroll up by one line.
    pub fn scroll_up(&mut self) {
        if self.offset > 0 {
            self.offset -= 1;
        }
    }

    /// Scroll down by a page.
    pub fn page_down(&mut self) {
        self.offset = (self.offset + self.viewport_height).min(self.max_offset());
    }

    /// Scroll up by a page.
    pub fn page_up(&mut self) {
        self.offset = self.offset.saturating_sub(self.viewport_height);
    }

    /// Scroll to the top.
    pub fn scroll_to_top(&mut self) {
        self.offset = 0;
    }

    /// Scroll to the bottom.
    pub fn scroll_to_bottom(&mut self) {
        self.offset = self.max_offset();
    }

    /// Get the selected item index.
    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Select an item by index.
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
        if let Some(idx) = index {
            self.ensure_visible(idx);
        }
    }

    /// Select the next item.
    pub fn select_next(&mut self) {
        let next = match self.selected {
            Some(i) if i + 1 < self.content_length => i + 1,
            Some(_) => self.content_length.saturating_sub(1),
            None if self.content_length > 0 => 0,
            None => return,
        };
        self.select(Some(next));
    }

    /// Select the previous item.
    pub fn select_previous(&mut self) {
        let prev = match self.selected {
            Some(i) if i > 0 => i - 1,
            Some(_) => 0,
            None if self.content_length > 0 => self.content_length - 1,
            None => return,
        };
        self.select(Some(prev));
    }

    /// Ensure the given index is visible in the viewport.
    pub fn ensure_visible(&mut self, index: usize) {
        if index < self.offset {
            self.offset = index;
        } else if index >= self.offset + self.viewport_height {
            self.offset = index.saturating_sub(self.viewport_height - 1);
        }
    }

    /// Check if the viewport is scrollable.
    #[must_use]
    pub const fn is_scrollable(&self) -> bool {
        self.content_length > self.viewport_height
    }

    /// Get the visible range (start, end) indices.
    #[must_use]
    pub const fn visible_range(&self) -> (usize, usize) {
        let start = self.offset;
        let end = if self.offset + self.viewport_height > self.content_length {
            self.content_length
        } else {
            self.offset + self.viewport_height
        };
        (start, end)
    }

    /// Get the scroll percentage (0.0 to 1.0).
    #[must_use]
    pub fn scroll_percentage(&self) -> f64 {
        if self.max_offset() == 0 {
            0.0
        } else {
            self.offset as f64 / self.max_offset() as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_viewport_scrolling() {
        let mut viewport = ViewportState::new()
            .content_length(100)
            .viewport_height(10);

        assert_eq!(viewport.offset(), 0);
        assert_eq!(viewport.max_offset(), 90);

        viewport.scroll_down();
        assert_eq!(viewport.offset(), 1);

        viewport.scroll_to_bottom();
        assert_eq!(viewport.offset(), 90);

        viewport.scroll_up();
        assert_eq!(viewport.offset(), 89);
    }

    #[test]
    fn test_viewport_selection() {
        let mut viewport = ViewportState::new()
            .content_length(100)
            .viewport_height(10);

        viewport.select(Some(50));
        assert_eq!(viewport.selected(), Some(50));
        assert_eq!(viewport.offset(), 41); // Ensures 50 is visible

        viewport.select_next();
        assert_eq!(viewport.selected(), Some(51));

        viewport.select_previous();
        assert_eq!(viewport.selected(), Some(50));
    }

    #[test]
    fn test_viewport_page_navigation() {
        let mut viewport = ViewportState::new()
            .content_length(100)
            .viewport_height(10);

        viewport.page_down();
        assert_eq!(viewport.offset(), 10);

        viewport.page_down();
        assert_eq!(viewport.offset(), 20);

        viewport.page_up();
        assert_eq!(viewport.offset(), 10);
    }
}
