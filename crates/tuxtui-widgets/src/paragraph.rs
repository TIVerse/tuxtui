//! Paragraph widget for rendering rich text with wrapping.

use alloc::vec::Vec;
use alloc::format;
use alloc::string::{String, ToString};
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::{Alignment, Rect};
use tuxtui_core::style::Style;
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::{Line, Text};
use unicode_width::UnicodeWidthStr;

/// Text wrapping strategy.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Wrap {
    /// No wrapping (truncate)
    NoWrap,
    /// Wrap at word boundaries
    Word,
    /// Wrap at character boundaries
    Char,
}

/// Scroll offset for a paragraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Scroll {
    /// Vertical scroll offset (lines)
    pub vertical: u16,
    /// Horizontal scroll offset (columns)
    pub horizontal: u16,
}

impl Scroll {
    /// Create a new scroll offset.
    #[must_use]
    pub const fn new(vertical: u16, horizontal: u16) -> Self {
        Self {
            vertical,
            horizontal,
        }
    }
}

/// A paragraph widget for rendering text.
///
/// Paragraphs support rich text, alignment, and wrapping strategies.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::paragraph::{Paragraph, Wrap};
///
/// let text = Text::from("Hello, world!\nThis is a paragraph.");
/// let paragraph = Paragraph::new(text)
///     .wrap(Wrap::Word)
///     .alignment(Alignment::Center);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Paragraph<'a> {
    text: Text<'a>,
    style: Style,
    wrap: Option<Wrap>,
    scroll: Scroll,
    alignment: Alignment,
}

impl<'a> Paragraph<'a> {
    /// Create a new paragraph from text.
    #[must_use]
    pub fn new<T: Into<Text<'a>>>(text: T) -> Self {
        Self {
            text: text.into(),
            style: Style::default(),
            wrap: None,
            scroll: Scroll::default(),
            alignment: Alignment::Start,
        }
    }

    /// Set the style for the paragraph.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the wrapping strategy.
    #[must_use]
    pub const fn wrap(mut self, wrap: Wrap) -> Self {
        self.wrap = Some(wrap);
        self
    }

    /// Set the scroll offset.
    #[must_use]
    pub const fn scroll(mut self, scroll: Scroll) -> Self {
        self.scroll = scroll;
        self
    }

    /// Set the alignment.
    #[must_use]
    pub const fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    fn wrap_lines(&self, lines: &[Line<'a>], width: u16) -> Vec<Line<'a>> {
        let mut wrapped = Vec::new();

        for line in lines {
            if let Some(wrap_mode) = self.wrap {
                match wrap_mode {
                    Wrap::NoWrap => {
                        wrapped.push(line.clone());
                    }
                    Wrap::Word => {
                        let line_text = format!("{line}");
                        let words: Vec<String> = line_text.split_whitespace().map(|s| s.to_string()).collect();
                        let mut current_line = Line::new();
                        let mut current_width = 0;

                        for word in words {
                            let word_width = word.width();
                            if current_width + word_width + 1 > width as usize && current_width > 0 {
                                wrapped.push(current_line);
                                current_line = Line::new();
                                current_width = 0;
                            }

                            if current_width > 0 {
                                current_line.push_span(" ".into());
                                current_width += 1;
                            }
                            current_line.push_span(word.into());
                            current_width += word_width;
                        }

                        if !current_line.spans.is_empty() {
                            wrapped.push(current_line);
                        }
                    }
                    Wrap::Char => {
                        let line_text = format!("{line}");
                        let mut current_line = Line::new();
                        let mut current_width = 0;

                        for grapheme in unicode_segmentation::UnicodeSegmentation::graphemes(
                            line_text.as_str(),
                            true,
                        ) {
                            let grapheme_width = grapheme.width();
                            if current_width + grapheme_width > width as usize {
                                wrapped.push(current_line);
                                current_line = Line::new();
                                current_width = 0;
                            }

                            current_line.push_span(grapheme.to_string().into());
                            current_width += grapheme_width;
                        }

                        if !current_line.spans.is_empty() {
                            wrapped.push(current_line);
                        }
                    }
                }
            } else {
                wrapped.push(line.clone());
            }
        }

        wrapped
    }
}

impl Widget for Paragraph<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        // Apply base style
        buf.set_style(self.style);

        // Wrap lines if needed
        let lines = self.wrap_lines(&self.text.lines, area.width);

        // Apply scroll offset
        let start_line = self.scroll.vertical as usize;
        let visible_lines = &lines[start_line.min(lines.len())..];

        // Render lines
        for (i, line) in visible_lines.iter().enumerate().take(area.height as usize) {
            let y = area.top() + i as u16;
            let line_width = line.width();

            let x = match self.alignment {
                Alignment::Start => area.left(),
                Alignment::Center => {
                    area.left() + (area.width.saturating_sub(line_width as u16)) / 2
                }
                Alignment::End => area.left() + area.width.saturating_sub(line_width as u16),
            };

            let mut current_x = x.saturating_sub(self.scroll.horizontal);
            for span in &line.spans {
                let span_style = self.style.patch(line.style).patch(span.style);
                current_x = buf.set_string(current_x, y, &span.content, span_style);
                if current_x >= area.right() {
                    break;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tuxtui_core::style::Color;

    #[test]
    fn test_paragraph_creation() {
        let text = Text::from("Hello, world!");
        let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Blue));
        assert_eq!(paragraph.alignment, Alignment::Start);
    }

    #[test]
    fn test_paragraph_wrap() {
        let text = Text::from("Hello world this is a long line");
        let paragraph = Paragraph::new(text).wrap(Wrap::Word);
        assert_eq!(paragraph.wrap, Some(Wrap::Word));
    }
}
