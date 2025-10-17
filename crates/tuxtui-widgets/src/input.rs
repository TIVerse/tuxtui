//! Text input widgets for user input.

use alloc::string::String;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::{Modifier, Style, Stylize};
use tuxtui_core::terminal::Widget;
use unicode_width::UnicodeWidthStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// State for a text input widget.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct InputState {
    /// The text content
    pub value: String,
    /// Cursor position (grapheme index)
    pub cursor: usize,
    /// Scroll offset for long text
    pub offset: usize,
}

impl Default for InputState {
    fn default() -> Self {
        Self::new()
    }
}

impl InputState {
    /// Create a new input state.
    #[must_use]
    pub fn new() -> Self {
        Self {
            value: String::new(),
            cursor: 0,
            offset: 0,
        }
    }

    /// Create input state with initial value.
    #[must_use]
    pub fn with_value(value: String) -> Self {
        let cursor = value.chars().count();
        Self {
            value,
            cursor,
            offset: 0,
        }
    }

    /// Get the current value.
    #[must_use]
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Insert a character at the cursor position.
    pub fn insert_char(&mut self, c: char) {
        let char_idx = self.grapheme_index_to_char_index(self.cursor);
        self.value.insert(char_idx, c);
        self.cursor += 1;
    }

    /// Delete the character before the cursor.
    pub fn delete_char(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
            let char_idx = self.grapheme_index_to_char_index(self.cursor);
            self.value.remove(char_idx);
        }
    }

    /// Move cursor left.
    pub fn move_cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    /// Move cursor right.
    pub fn move_cursor_right(&mut self) {
        let len = self.value.chars().count();
        if self.cursor < len {
            self.cursor += 1;
        }
    }

    /// Move cursor to start.
    pub fn move_cursor_start(&mut self) {
        self.cursor = 0;
        self.offset = 0;
    }

    /// Move cursor to end.
    pub fn move_cursor_end(&mut self) {
        self.cursor = self.value.chars().count();
    }

    /// Clear all content.
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor = 0;
        self.offset = 0;
    }

    /// Helper to convert grapheme index to char index.
    fn grapheme_index_to_char_index(&self, grapheme_idx: usize) -> usize {
        self.value.chars().take(grapheme_idx).count()
    }
}

/// A text input widget.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::input::{TextInput, InputState};
///
/// let mut state = InputState::new();
/// let input = TextInput::default()
///     .placeholder("Enter text...")
///     .style(Style::default().fg(Color::White));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TextInput<'a> {
    placeholder: Option<&'a str>,
    style: Style,
    placeholder_style: Style,
    cursor_style: Style,
    show_cursor: bool,
    mask_char: Option<char>,
}

impl<'a> Default for TextInput<'a> {
    fn default() -> Self {
        Self {
            placeholder: None,
            style: Style::new(),
            placeholder_style: Style::new().add_modifier(Modifier::DIM),
            cursor_style: Style::new().add_modifier(Modifier::REVERSED),
            show_cursor: true,
            mask_char: None,
        }
    }
}

impl<'a> TextInput<'a> {
    /// Create a new text input.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the placeholder text.
    #[must_use]
    pub const fn placeholder(mut self, placeholder: &'a str) -> Self {
        self.placeholder = Some(placeholder);
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the placeholder style.
    #[must_use]
    pub const fn placeholder_style(mut self, style: Style) -> Self {
        self.placeholder_style = style;
        self
    }

    /// Set the cursor style.
    #[must_use]
    pub const fn cursor_style(mut self, style: Style) -> Self {
        self.cursor_style = style;
        self
    }

    /// Set whether to show the cursor.
    #[must_use]
    pub const fn show_cursor(mut self, show: bool) -> Self {
        self.show_cursor = show;
        self
    }

    /// Set a mask character for password input.
    ///
    /// When set, all characters will be displayed as this character.
    /// Use `Some('*')` or `Some('•')` for password fields.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_widgets::input::TextInput;
    ///
    /// let password_input = TextInput::default()
    ///     .mask_char(Some('*'))
    ///     .placeholder("Enter password...");
    /// ```
    #[must_use]
    pub const fn mask_char(mut self, mask: Option<char>) -> Self {
        self.mask_char = mask;
        self
    }

    /// Render the input with state.
    pub fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut InputState) {
        if area.area() == 0 {
            return;
        }

        let y = area.top();
        let width = area.width as usize;

        // Display placeholder if empty
        if state.value.is_empty() {
            if let Some(placeholder) = self.placeholder {
                let placeholder_style = self.style.patch(self.placeholder_style);
                buf.set_string(area.left(), y, placeholder, placeholder_style);
            }

            // Show cursor at beginning if enabled
            if self.show_cursor {
                buf.set(area.left(), y, " ", self.cursor_style);
            }
            return;
        }

        // Calculate visible portion
        let chars: alloc::vec::Vec<char> = state.value.chars().collect();

        // Adjust offset to keep cursor visible
        if state.cursor < state.offset {
            state.offset = state.cursor;
        } else if state.cursor > state.offset + width {
            state.offset = state.cursor.saturating_sub(width);
        }

        let visible_start = state.offset;
        let visible_end = (state.offset + width).min(chars.len());
        let visible_text: String = chars[visible_start..visible_end].iter().collect();

        // Render text (with masking if enabled)
        let mut x = area.left();
        for (i, ch) in visible_text.chars().enumerate() {
            let global_idx = visible_start + i;
            let style = if self.show_cursor && global_idx == state.cursor {
                self.style.patch(self.cursor_style)
            } else {
                self.style
            };

            // Use mask character if set, otherwise show actual character
            let display_char = self.mask_char.unwrap_or(ch);
            let ch_str = alloc::string::ToString::to_string(&display_char);
            x = buf.set_string(x, y, &ch_str, style);
            if x >= area.right() {
                break;
            }
        }

        // Show cursor at end if needed
        if self.show_cursor && state.cursor == chars.len() && x < area.right() {
            buf.set(x, y, " ", self.cursor_style);
        }
    }
}

impl Widget for TextInput<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = InputState::default();
        self.render_stateful(area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::string::ToString;

    #[test]
    fn test_input_state_insert() {
        let mut state = InputState::new();
        state.insert_char('a');
        state.insert_char('b');
        assert_eq!(state.value(), "ab");
        assert_eq!(state.cursor, 2);
    }

    #[test]
    fn test_input_state_delete() {
        let mut state = InputState::with_value("hello".to_string());
        state.delete_char();
        assert_eq!(state.value(), "hell");
        assert_eq!(state.cursor, 4);
    }

    #[test]
    fn test_input_state_cursor_movement() {
        let mut state = InputState::with_value("test".to_string());
        assert_eq!(state.cursor, 4);

        state.move_cursor_left();
        assert_eq!(state.cursor, 3);

        state.move_cursor_start();
        assert_eq!(state.cursor, 0);

        state.move_cursor_end();
        assert_eq!(state.cursor, 4);
    }

    #[test]
    fn test_input_state_clear() {
        let mut state = InputState::with_value("test".to_string());
        state.clear();
        assert_eq!(state.value(), "");
        assert_eq!(state.cursor, 0);
    }
}
