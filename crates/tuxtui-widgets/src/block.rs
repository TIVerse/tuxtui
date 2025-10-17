//! Block widget for creating bordered containers with titles.

use alloc::string::String;
use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::{Margin, Rect};
use tuxtui_core::style::{Style, Stylize};
use tuxtui_core::symbols::{DOUBLE, LineStyle, NORMAL, ROUNDED, THICK};
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::{Line, Span};

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Border type for a block.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum BorderType {
    /// No borders
    None,
    /// All borders
    All,
    /// Top border only
    Top,
    /// Bottom border only
    Bottom,
    /// Left border only
    Left,
    /// Right border only
    Right,
}

/// Border style preset.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Borders {
    /// Normal single-line borders
    Normal,
    /// Rounded corner borders
    Rounded,
    /// Double-line borders
    Double,
    /// Thick borders
    Thick,
    /// Custom line style
    Custom(LineStyle),
}

impl Borders {
    /// Get the line style for this border preset.
    #[must_use]
    pub const fn line_style(self) -> LineStyle {
        match self {
            Self::Normal => NORMAL,
            Self::Rounded => ROUNDED,
            Self::Double => DOUBLE,
            Self::Thick => THICK,
            Self::Custom(style) => style,
        }
    }
}

/// Title position on a block border.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum TitlePosition {
    /// Top-left
    TopLeft,
    /// Top-center
    TopCenter,
    /// Top-right
    TopRight,
    /// Bottom-left
    BottomLeft,
    /// Bottom-center
    BottomCenter,
    /// Bottom-right
    BottomRight,
}

/// A title with position for a block.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Title<'a> {
    /// The content of the title
    pub content: Line<'a>,
    /// Position of the title
    pub position: TitlePosition,
}

impl<'a> Title<'a> {
    /// Create a new title.
    #[must_use]
    pub fn new<T: Into<Line<'a>>>(content: T) -> Self {
        Self {
            content: content.into(),
            position: TitlePosition::TopLeft,
        }
    }

    /// Set the position of the title.
    #[must_use]
    pub const fn position(mut self, position: TitlePosition) -> Self {
        self.position = position;
        self
    }
}

impl<'a, T: Into<Line<'a>>> From<T> for Title<'a> {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}

/// A bordered block widget.
///
/// Blocks provide a container with borders and optional titles.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::block::{Block, BorderType};
///
/// let block = Block::default()
///     .title("My Block")
///     .borders(BorderType::All)
///     .style(Style::default().fg(Color::Blue));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'a> {
    titles: Vec<Title<'a>>,
    borders: BorderType,
    border_style: Borders,
    style: Style,
    padding: Margin,
}

impl<'a> Default for Block<'a> {
    fn default() -> Self {
        Self {
            titles: Vec::new(),
            borders: BorderType::None,
            border_style: Borders::Normal,
            style: Style::default(),
            padding: Margin::new(0, 0),
        }
    }
}

impl<'a> Block<'a> {
    /// Create a new block.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the border type.
    #[must_use]
    pub const fn borders(mut self, borders: BorderType) -> Self {
        self.borders = borders;
        self
    }

    /// Set the border style.
    #[must_use]
    pub const fn border_style(mut self, style: Borders) -> Self {
        self.border_style = style;
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set padding.
    #[must_use]
    pub const fn padding(mut self, padding: Margin) -> Self {
        self.padding = padding;
        self
    }

    /// Add a title.
    #[must_use]
    pub fn title<T: Into<Title<'a>>>(mut self, title: T) -> Self {
        self.titles.push(title.into());
        self
    }

    /// Calculate the inner area of the block (excluding borders and padding).
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::prelude::*;
    /// use tuxtui_widgets::block::{Block, BorderType};
    ///
    /// let block = Block::default().borders(BorderType::All);
    /// let area = Rect::new(0, 0, 10, 10);
    /// let inner = block.inner(area);
    /// assert_eq!(inner, Rect::new(1, 1, 8, 8));
    /// ```
    #[must_use]
    pub fn inner(&self, area: Rect) -> Rect {
        let mut inner = area;

        // Account for borders
        if matches!(
            self.borders,
            BorderType::All | BorderType::Left | BorderType::Right
        ) {
            inner = inner.inner(Margin::new(1, 0));
        }
        if matches!(
            self.borders,
            BorderType::All | BorderType::Top | BorderType::Bottom
        ) {
            inner = inner.inner(Margin::new(0, 1));
        }

        // Account for padding
        inner.inner(self.padding)
    }
}

impl<'a> Stylize for Block<'a> {
    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Block<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        let symbols = self.border_style.line_style();

        // Render borders
        match self.borders {
            BorderType::None => {}
            BorderType::All => {
                // Corners
                buf.set(area.left(), area.top(), symbols.top_left, self.style);
                buf.set(area.right() - 1, area.top(), symbols.top_right, self.style);
                buf.set(
                    area.left(),
                    area.bottom() - 1,
                    symbols.bottom_left,
                    self.style,
                );
                buf.set(
                    area.right() - 1,
                    area.bottom() - 1,
                    symbols.bottom_right,
                    self.style,
                );

                // Horizontal borders
                for x in (area.left() + 1)..(area.right() - 1) {
                    buf.set(x, area.top(), symbols.horizontal, self.style);
                    buf.set(x, area.bottom() - 1, symbols.horizontal, self.style);
                }

                // Vertical borders
                for y in (area.top() + 1)..(area.bottom() - 1) {
                    buf.set(area.left(), y, symbols.vertical, self.style);
                    buf.set(area.right() - 1, y, symbols.vertical, self.style);
                }
            }
            BorderType::Top => {
                for x in area.left()..area.right() {
                    buf.set(x, area.top(), symbols.horizontal, self.style);
                }
            }
            BorderType::Bottom => {
                for x in area.left()..area.right() {
                    buf.set(x, area.bottom() - 1, symbols.horizontal, self.style);
                }
            }
            BorderType::Left => {
                for y in area.top()..area.bottom() {
                    buf.set(area.left(), y, symbols.vertical, self.style);
                }
            }
            BorderType::Right => {
                for y in area.top()..area.bottom() {
                    buf.set(area.right() - 1, y, symbols.vertical, self.style);
                }
            }
        }

        // Render titles
        for title in &self.titles {
            let title_width = title.content.width() as u16;
            let (x, y) = match title.position {
                TitlePosition::TopLeft => (area.left() + 1, area.top()),
                TitlePosition::TopCenter => {
                    let center = area.left() + area.width / 2;
                    (center.saturating_sub(title_width / 2), area.top())
                }
                TitlePosition::TopRight => {
                    (area.right().saturating_sub(title_width + 1), area.top())
                }
                TitlePosition::BottomLeft => (area.left() + 1, area.bottom() - 1),
                TitlePosition::BottomCenter => {
                    let center = area.left() + area.width / 2;
                    (center.saturating_sub(title_width / 2), area.bottom() - 1)
                }
                TitlePosition::BottomRight => (
                    area.right().saturating_sub(title_width + 1),
                    area.bottom() - 1,
                ),
            };

            let mut current_x = x;
            for span in &title.content.spans {
                let span_style = self.style.patch(span.style);
                current_x = buf.set_string(current_x, y, &span.content, span_style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tuxtui_core::backend::TestBackend;
    use tuxtui_core::style::Color;
    use tuxtui_core::terminal::Terminal;

    #[test]
    fn test_block_inner() {
        let block = Block::default().borders(BorderType::All);
        let area = Rect::new(0, 0, 10, 10);
        let inner = block.inner(area);
        assert_eq!(inner, Rect::new(1, 1, 8, 8));
    }

    #[test]
    fn test_block_with_padding() {
        let block = Block::default()
            .borders(BorderType::All)
            .padding(Margin::new(1, 1));
        let area = Rect::new(0, 0, 10, 10);
        let inner = block.inner(area);
        assert_eq!(inner, Rect::new(2, 2, 6, 6));
    }

    #[test]
    fn test_block_render() {
        let backend = TestBackend::new(10, 5);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let block = Block::default().title("Test").borders(BorderType::All);
                frame.render_widget(block, frame.area());
            })
            .unwrap();
    }
}
