//! Tabs widget for tabbed navigation.

use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::{Style, Stylize};
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::Line;
use unicode_width::UnicodeWidthStr;

/// A tabs widget for navigation.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::tabs::Tabs;
///
/// let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3"])
///     .select(1)
///     .style(Style::default().fg(Color::White))
///     .highlight_style(Style::default().fg(Color::Yellow));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tabs<'a> {
    titles: Vec<Line<'a>>,
    selected: Option<usize>,
    style: Style,
    highlight_style: Style,
    divider: &'static str,
}

impl<'a> Tabs<'a> {
    /// Create new tabs.
    #[must_use]
    pub fn new<T>(titles: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Line<'a>>,
    {
        Self {
            titles: titles.into_iter().map(Into::into).collect(),
            selected: None,
            style: Style::default(),
            highlight_style: Style::default(),
            divider: " │ ",
        }
    }

    /// Select a tab by index.
    #[must_use]
    pub const fn select(mut self, index: usize) -> Self {
        self.selected = Some(index);
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the highlight style for the selected tab.
    #[must_use]
    pub const fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Set the divider between tabs.
    #[must_use]
    pub const fn divider(mut self, divider: &'static str) -> Self {
        self.divider = divider;
        self
    }
}

impl<'a> Stylize for Tabs<'a> {
    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Tabs<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 || self.titles.is_empty() {
            return;
        }

        let mut x = area.left();
        let y = area.top();

        for (i, title) in self.titles.iter().enumerate() {
            if x >= area.right() {
                break;
            }

            let is_selected = self.selected == Some(i);
            let tab_style = if is_selected {
                self.style.patch(self.highlight_style)
            } else {
                self.style
            };

            // Render title
            for span in &title.spans {
                let span_style = tab_style.patch(span.style);
                x = buf.set_string(x, y, &span.content, span_style);
                if x >= area.right() {
                    break;
                }
            }

            // Render divider (except after last tab)
            if i < self.titles.len() - 1 && x < area.right() {
                x = buf.set_string(x, y, self.divider, self.style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tuxtui_core::style::Color;

    #[test]
    fn test_tabs_creation() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2", "Tab3"]);
        assert_eq!(tabs.titles.len(), 3);
    }

    #[test]
    fn test_tabs_selection() {
        let tabs = Tabs::new(vec!["Tab1", "Tab2"]).select(1);
        assert_eq!(tabs.selected, Some(1));
    }
}
