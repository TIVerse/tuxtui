//! List widget for rendering selectable items.

use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::{Style, Stylize};
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::Line;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// List item marker style.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListMarker {
    /// Bullet point
    Bullet,
    /// Numbered (1., 2., 3., ...)
    Numbered,
    /// Custom marker
    Custom(&'static str),
}

/// A single list item.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::list::ListItem;
///
/// let item = ListItem::new("Item 1")
///     .style(Style::default().fg(Color::Green));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListItem<'a> {
    content: Line<'a>,
    style: Style,
}

impl<'a> ListItem<'a> {
    /// Create a new list item.
    #[must_use]
    pub fn new<T: Into<Line<'a>>>(content: T) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Set the style for this item.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Get the content of this item.
    #[must_use]
    pub const fn content(&self) -> &Line<'a> {
        &self.content
    }
}

impl<'a, T: Into<Line<'a>>> From<T> for ListItem<'a> {
    fn from(content: T) -> Self {
        Self::new(content)
    }
}

impl<'a> Stylize for ListItem<'a> {
    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// State for a stateful list widget.
///
/// Tracks the currently selected item and scroll offset.
///
/// # Example
///
/// ```
/// use tuxtui_widgets::list::ListState;
///
/// let mut state = ListState::default();
/// state.select(Some(0));
/// assert_eq!(state.selected(), Some(0));
/// ```
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct ListState {
    selected: Option<usize>,
    offset: usize,
}

impl ListState {
    /// Create a new list state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            selected: None,
            offset: 0,
        }
    }

    /// Get the selected item index.
    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Select an item.
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select the next item.
    pub fn select_next(&mut self, items_len: usize) {
        if items_len == 0 {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % items_len,
            None => 0,
        });
    }

    /// Select the previous item.
    pub fn select_previous(&mut self, items_len: usize) {
        if items_len == 0 {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => {
                if i == 0 {
                    items_len - 1
                } else {
                    i - 1
                }
            }
            None => items_len - 1,
        });
    }

    /// Get the scroll offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Set the scroll offset.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

/// A list widget.
///
/// Lists display a collection of items with optional selection highlighting.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::list::{List, ListItem};
///
/// let items = vec![
///     ListItem::new("Item 1"),
///     ListItem::new("Item 2"),
///     ListItem::new("Item 3"),
/// ];
///
/// let list = List::new(items)
///     .highlight_style(Style::default().bg(Color::Blue));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'a> {
    items: Vec<ListItem<'a>>,
    style: Style,
    highlight_style: Style,
    highlight_symbol: Option<&'static str>,
    marker: Option<ListMarker>,
}

impl<'a> List<'a> {
    /// Create a new list.
    #[must_use]
    pub fn new<T>(items: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<ListItem<'a>>,
    {
        Self {
            items: items.into_iter().map(Into::into).collect(),
            style: Style::default(),
            highlight_style: Style::default(),
            highlight_symbol: Some(">> "),
            marker: None,
        }
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the highlight style for the selected item.
    #[must_use]
    pub const fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Set the highlight symbol.
    #[must_use]
    pub const fn highlight_symbol(mut self, symbol: &'static str) -> Self {
        self.highlight_symbol = Some(symbol);
        self
    }

    /// Set the marker style for items.
    #[must_use]
    pub const fn marker(mut self, marker: ListMarker) -> Self {
        self.marker = Some(marker);
        self
    }

    /// Render the list with state.
    pub fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        if area.area() == 0 || self.items.is_empty() {
            return;
        }

        // Adjust offset to ensure selected item is visible
        if let Some(selected) = state.selected() {
            if selected < state.offset {
                state.offset = selected;
            } else if selected >= state.offset + area.height as usize {
                state.offset = selected.saturating_sub(area.height as usize - 1);
            }
        }

        let visible_items = &self.items[state.offset.min(self.items.len())..];

        for (i, item) in visible_items.iter().enumerate().take(area.height as usize) {
            let y = area.top() + i as u16;
            let item_index = state.offset + i;
            let is_selected = state.selected() == Some(item_index);

            let item_style = if is_selected {
                self.style.patch(self.highlight_style).patch(item.style)
            } else {
                self.style.patch(item.style)
            };

            let mut x = area.left();

            // Render highlight symbol
            if is_selected {
                if let Some(symbol) = self.highlight_symbol {
                    x = buf.set_string(x, y, symbol, item_style);
                }
            } else if let Some(marker) = self.marker {
                match marker {
                    ListMarker::Bullet => {
                        x = buf.set_string(x, y, "• ", item_style);
                    }
                    ListMarker::Numbered => {
                        let numbered = alloc::format!("{}. ", item_index + 1);
                        x = buf.set_string(x, y, &numbered, item_style);
                    }
                    ListMarker::Custom(s) => {
                        x = buf.set_string(x, y, s, item_style);
                    }
                }
            }

            // Render item content
            for span in &item.content.spans {
                let span_style = item_style.patch(span.style);
                x = buf.set_string(x, y, &span.content, span_style);
                if x >= area.right() {
                    break;
                }
            }
        }
    }
}

impl Widget for List<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = ListState::default();
        self.render_stateful(area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_list_creation() {
        let items = vec!["Item 1", "Item 2", "Item 3"];
        let list = List::new(items);
        assert_eq!(list.items.len(), 3);
    }

    #[test]
    fn test_list_state() {
        let mut state = ListState::default();
        assert_eq!(state.selected(), None);

        state.select(Some(1));
        assert_eq!(state.selected(), Some(1));

        state.select_next(5);
        assert_eq!(state.selected(), Some(2));

        state.select_previous(5);
        assert_eq!(state.selected(), Some(1));
    }

    #[test]
    fn test_list_state_wrap() {
        let mut state = ListState::default();
        state.select(Some(4));
        state.select_next(5);
        assert_eq!(state.selected(), Some(0));

        state.select(Some(0));
        state.select_previous(5);
        assert_eq!(state.selected(), Some(4));
    }
}
