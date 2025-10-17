//! Scrollbar widget for scrollable content.

use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::symbols::{SCROLLBAR_DEFAULT, ScrollbarSymbols};
use tuxtui_core::terminal::Widget;

/// Scrollbar orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollbarOrientation {
    /// Vertical scrollbar
    Vertical,
    /// Horizontal scrollbar
    Horizontal,
}

/// A scrollbar widget.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::scrollbar::{Scrollbar, ScrollbarOrientation};
///
/// let scrollbar = Scrollbar::default()
///     .orientation(ScrollbarOrientation::Vertical)
///     .position(10)
///     .content_length(100);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Scrollbar {
    orientation: ScrollbarOrientation,
    position: usize,
    content_length: usize,
    viewport_length: usize,
    style: Style,
    symbols: ScrollbarSymbols,
}

impl Default for Scrollbar {
    fn default() -> Self {
        Self {
            orientation: ScrollbarOrientation::Vertical,
            position: 0,
            content_length: 0,
            viewport_length: 0,
            style: Style::default(),
            symbols: SCROLLBAR_DEFAULT,
        }
    }
}

impl Scrollbar {
    /// Create a new scrollbar.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            orientation: ScrollbarOrientation::Vertical,
            position: 0,
            content_length: 0,
            viewport_length: 0,
            style: Style::new(),
            symbols: SCROLLBAR_DEFAULT,
        }
    }

    /// Set the orientation.
    #[must_use]
    pub const fn orientation(mut self, orientation: ScrollbarOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set the scroll position.
    #[must_use]
    pub const fn position(mut self, position: usize) -> Self {
        self.position = position;
        self
    }

    /// Set the total content length.
    #[must_use]
    pub const fn content_length(mut self, length: usize) -> Self {
        self.content_length = length;
        self
    }

    /// Set the viewport length.
    #[must_use]
    pub const fn viewport_length(mut self, length: usize) -> Self {
        self.viewport_length = length;
        self
    }

    /// Set the style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the symbols.
    #[must_use]
    pub const fn symbols(mut self, symbols: ScrollbarSymbols) -> Self {
        self.symbols = symbols;
        self
    }
}

impl Widget for Scrollbar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 || self.content_length == 0 {
            return;
        }

        match self.orientation {
            ScrollbarOrientation::Vertical => {
                let track_height = area.height as usize;
                if track_height == 0 {
                    return;
                }

                let thumb_size =
                    ((track_height * self.viewport_length) / self.content_length).max(1);
                let thumb_position = if self.content_length > self.viewport_length {
                    (self.position * (track_height - thumb_size))
                        / (self.content_length - self.viewport_length)
                } else {
                    0
                };

                for y in 0..track_height {
                    let symbol = if y >= thumb_position && y < thumb_position + thumb_size {
                        self.symbols.thumb
                    } else {
                        self.symbols.track
                    };
                    buf.set(area.left(), area.top() + y as u16, symbol, self.style);
                }
            }
            ScrollbarOrientation::Horizontal => {
                let track_width = area.width as usize;
                if track_width == 0 {
                    return;
                }

                let thumb_size =
                    ((track_width * self.viewport_length) / self.content_length).max(1);
                let thumb_position = if self.content_length > self.viewport_length {
                    (self.position * (track_width - thumb_size))
                        / (self.content_length - self.viewport_length)
                } else {
                    0
                };

                for x in 0..track_width {
                    let symbol = if x >= thumb_position && x < thumb_position + thumb_size {
                        self.symbols.thumb
                    } else {
                        self.symbols.track
                    };
                    buf.set(area.left() + x as u16, area.top(), symbol, self.style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scrollbar_creation() {
        let scrollbar = Scrollbar::default()
            .position(10)
            .content_length(100)
            .viewport_length(20);

        assert_eq!(scrollbar.position, 10);
        assert_eq!(scrollbar.content_length, 100);
    }
}
