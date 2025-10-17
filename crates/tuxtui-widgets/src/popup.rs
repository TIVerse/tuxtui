//! Popup and modal widgets.

use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::{Alignment, Rect};
use tuxtui_core::style::Style;
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::Text;

/// A popup widget that centers content over the background.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::popup::Popup;
/// use tuxtui_widgets::block::{Block, BorderType};
///
/// let popup = Popup::new()
///     .percent_x(50)
///     .percent_y(50);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Popup {
    percent_x: u16,
    percent_y: u16,
    clear_background: bool,
    background_style: Option<Style>,
}

impl Default for Popup {
    fn default() -> Self {
        Self {
            percent_x: 60,
            percent_y: 50,
            clear_background: true,
            background_style: None,
        }
    }
}

impl Popup {
    /// Create a new popup.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the horizontal size as percentage of the screen.
    #[must_use]
    pub fn percent_x(mut self, percent: u16) -> Self {
        self.percent_x = percent.min(100);
        self
    }

    /// Set the vertical size as percentage of the screen.
    #[must_use]
    pub fn percent_y(mut self, percent: u16) -> Self {
        self.percent_y = percent.min(100);
        self
    }

    /// Set whether to clear the background.
    #[must_use]
    pub const fn clear_background(mut self, clear: bool) -> Self {
        self.clear_background = clear;
        self
    }

    /// Set the background style.
    #[must_use]
    pub const fn background_style(mut self, style: Style) -> Self {
        self.background_style = Some(style);
        self
    }

    /// Calculate the centered area for the popup.
    #[must_use]
    pub fn area(&self, full_area: Rect) -> Rect {
        let width = (full_area.width * self.percent_x) / 100;
        let height = (full_area.height * self.percent_y) / 100;

        let x = full_area.x + (full_area.width.saturating_sub(width)) / 2;
        let y = full_area.y + (full_area.height.saturating_sub(height)) / 2;

        Rect::new(x, y, width, height)
    }

    /// Render a widget inside the popup area.
    pub fn render_widget<W: Widget>(&self, area: Rect, buf: &mut Buffer, widget: W) {
        // Clear/style background if requested
        if self.clear_background {
            let style = self.background_style.unwrap_or_default();
            for y in area.top()..area.bottom() {
                for x in area.left()..area.right() {
                    buf.set(x, y, " ", style);
                }
            }
        }

        let popup_area = self.area(area);
        widget.render(popup_area, buf);
    }
}

/// A modal dialog widget.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::popup::Modal;
///
/// let modal = Modal::new("Confirm", "Are you sure?")
///     .buttons(&["Yes", "No"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Modal<'a> {
    title: &'a str,
    content: Text<'a>,
    buttons: &'a [&'a str],
    selected_button: usize,
    style: Style,
    button_style: Style,
    selected_style: Style,
}

impl<'a> Modal<'a> {
    /// Create a new modal.
    #[must_use]
    pub fn new<T: Into<Text<'a>>>(title: &'a str, content: T) -> Self {
        Self {
            title,
            content: content.into(),
            buttons: &["OK"],
            selected_button: 0,
            style: Style::new(),
            button_style: Style::new(),
            selected_style: Style::new().bg(tuxtui_core::style::Color::Blue),
        }
    }

    /// Set the buttons.
    #[must_use]
    pub const fn buttons(mut self, buttons: &'a [&'a str]) -> Self {
        self.buttons = buttons;
        self
    }

    /// Set the selected button index.
    #[must_use]
    pub const fn selected_button(mut self, index: usize) -> Self {
        self.selected_button = index;
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the button style.
    #[must_use]
    pub const fn button_style(mut self, style: Style) -> Self {
        self.button_style = style;
        self
    }

    /// Set the selected button style.
    #[must_use]
    pub const fn selected_style(mut self, style: Style) -> Self {
        self.selected_style = style;
        self
    }

    /// Get the currently selected button index.
    #[must_use]
    pub const fn get_selected(&self) -> usize {
        self.selected_button
    }

    /// Select the next button.
    pub fn next_button(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        self.selected_button = (self.selected_button + 1) % self.buttons.len();
    }

    /// Select the previous button.
    pub fn previous_button(&mut self) {
        if self.buttons.is_empty() {
            return;
        }
        if self.selected_button == 0 {
            self.selected_button = self.buttons.len() - 1;
        } else {
            self.selected_button -= 1;
        }
    }
}

impl Widget for Modal<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        use crate::block::{Block, BorderType};
        use crate::paragraph::Paragraph;

        // Render border
        let block = Block::default()
            .title(self.title)
            .borders(BorderType::All)
            .style(self.style);

        let inner = block.inner(area);
        block.render(area, buf);

        if inner.area() == 0 {
            return;
        }

        // Split into content and button areas
        let button_height = 3;
        let content_height = inner.height.saturating_sub(button_height);

        let content_area = Rect::new(inner.x, inner.y, inner.width, content_height);
        let button_area = Rect::new(
            inner.x,
            inner.y + content_height,
            inner.width,
            button_height,
        );

        // Render content
        let paragraph = Paragraph::new(self.content)
            .alignment(Alignment::Center)
            .style(self.style);
        paragraph.render(content_area, buf);

        // Render buttons
        if !self.buttons.is_empty() {
            let button_width: u16 = 10;
            let spacing: u16 = 2;
            let total_width = self.buttons.len() as u16 * (button_width + spacing);

            let start_x = button_area.x + (button_area.width.saturating_sub(total_width)) / 2;
            let button_y = button_area.y + 1;

            for (idx, &button_text) in self.buttons.iter().enumerate() {
                let x = start_x + (idx as u16 * (button_width + spacing));

                let button_style = if idx == self.selected_button {
                    self.button_style.patch(self.selected_style)
                } else {
                    self.button_style
                };

                // Render button background
                for i in 0..button_width {
                    if x + i < button_area.right() {
                        buf.set(x + i, button_y, " ", button_style);
                    }
                }

                // Center button text
                let text_start = x + (button_width.saturating_sub(button_text.len() as u16)) / 2;
                buf.set_string(text_start, button_y, button_text, button_style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popup_area() {
        let popup = Popup::new().percent_x(50).percent_y(50);
        let full_area = Rect::new(0, 0, 100, 50);
        let popup_area = popup.area(full_area);

        assert_eq!(popup_area.width, 50);
        assert_eq!(popup_area.height, 25);
        assert_eq!(popup_area.x, 25);
        assert_eq!(popup_area.y, 12);
    }

    #[test]
    fn test_modal_button_navigation() {
        let mut modal = Modal::new("Test", "Content").buttons(&["Yes", "No", "Cancel"]);

        assert_eq!(modal.get_selected(), 0);

        modal.next_button();
        assert_eq!(modal.get_selected(), 1);

        modal.next_button();
        assert_eq!(modal.get_selected(), 2);

        modal.next_button();
        assert_eq!(modal.get_selected(), 0);

        modal.previous_button();
        assert_eq!(modal.get_selected(), 2);
    }
}
