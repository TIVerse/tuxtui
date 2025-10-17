//! Rich text primitives for styled terminal content.

use crate::geometry::Alignment;
use crate::style::{Style, Stylize};
use alloc::borrow::Cow;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use unicode_width::UnicodeWidthStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A styled span of text.
///
/// The most basic text primitive representing a single string with a style.
///
/// # Example
///
/// ```
/// use tuxtui_core::text::Span;
/// use tuxtui_core::style::{Color, Style};
///
/// let span = Span::styled("Hello", Style::default().fg(Color::Blue));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Span<'a> {
    /// The text content
    pub content: Cow<'a, str>,
    /// The style for this span
    pub style: Style,
}

impl<'a> Span<'a> {
    /// Create a new span with the given content and default style.
    #[must_use]
    pub fn raw<T: Into<Cow<'a, str>>>(content: T) -> Self {
        Self {
            content: content.into(),
            style: Style::default(),
        }
    }

    /// Create a new span with the given content and style.
    #[must_use]
    pub fn styled<T: Into<Cow<'a, str>>>(content: T, style: Style) -> Self {
        Self {
            content: content.into(),
            style,
        }
    }

    /// Get the display width of this span.
    #[must_use]
    pub fn width(&self) -> usize {
        self.content.width()
    }

    /// Convert this span to an owned version.
    #[must_use]
    pub fn into_owned(self) -> Span<'static> {
        Span {
            content: Cow::Owned(self.content.into_owned()),
            style: self.style,
        }
    }

    /// Patch the style of this span.
    #[must_use]
    pub fn patch_style(mut self, style: Style) -> Self {
        self.style = self.style.patch(style);
        self
    }
}

impl<'a> From<&'a str> for Span<'a> {
    fn from(s: &'a str) -> Self {
        Self::raw(s)
    }
}

impl From<String> for Span<'static> {
    fn from(s: String) -> Self {
        Self::raw(s)
    }
}

impl<'a> Stylize for Span<'a> {
    fn style(mut self, style: Style) -> Self {
        self.style = self.style.patch(style);
        self
    }
}

impl fmt::Display for Span<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content)
    }
}

/// A line of text composed of multiple styled spans.
///
/// A line represents a single row of text that can have multiple different styles.
///
/// # Example
///
/// ```
/// use tuxtui_core::text::{Line, Span};
/// use tuxtui_core::style::{Color, Style};
///
/// let line = Line::from(vec![
///     Span::styled("Hello ", Style::default().fg(Color::Blue)),
///     Span::styled("World!", Style::default().fg(Color::Red)),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Line<'a> {
    /// The spans that make up this line
    pub spans: Vec<Span<'a>>,
    /// Alignment for this line
    pub alignment: Alignment,
    /// Line style applied to all spans
    pub style: Style,
}

impl<'a> Line<'a> {
    /// Create a new empty line.
    #[must_use]
    pub fn new() -> Self {
        Self {
            spans: Vec::new(),
            alignment: Alignment::Start,
            style: Style::default(),
        }
    }

    /// Create a new line from spans.
    #[must_use]
    pub fn from_spans(spans: Vec<Span<'a>>) -> Self {
        Self {
            spans,
            alignment: Alignment::Start,
            style: Style::default(),
        }
    }

    /// Create a styled line from a string.
    #[must_use]
    pub fn styled<T: Into<Cow<'a, str>>>(content: T, style: Style) -> Self {
        Self {
            spans: alloc::vec![Span::styled(content, style)],
            alignment: Alignment::Start,
            style: Style::default(),
        }
    }

    /// Set the alignment for this line.
    #[must_use]
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Get the display width of this line.
    #[must_use]
    pub fn width(&self) -> usize {
        self.spans.iter().map(Span::width).sum()
    }

    /// Convert this line to an owned version.
    #[must_use]
    pub fn into_owned(self) -> Line<'static> {
        Line {
            spans: self.spans.into_iter().map(Span::into_owned).collect(),
            alignment: self.alignment,
            style: self.style,
        }
    }

    /// Patch the style of this line (affects all spans).
    #[must_use]
    pub fn patch_style(mut self, style: Style) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Push a span to this line.
    pub fn push_span(&mut self, span: Span<'a>) {
        self.spans.push(span);
    }
}

impl<'a> Default for Line<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a str> for Line<'a> {
    fn from(s: &'a str) -> Self {
        Self::from_spans(alloc::vec![Span::raw(s)])
    }
}

impl From<String> for Line<'static> {
    fn from(s: String) -> Self {
        Self::from_spans(alloc::vec![Span::raw(s)])
    }
}

impl<'a> From<Span<'a>> for Line<'a> {
    fn from(span: Span<'a>) -> Self {
        Self::from_spans(alloc::vec![span])
    }
}

impl<'a> From<Vec<Span<'a>>> for Line<'a> {
    fn from(spans: Vec<Span<'a>>) -> Self {
        Self::from_spans(spans)
    }
}

impl<'a> Stylize for Line<'a> {
    fn style(self, style: Style) -> Self {
        self.patch_style(style)
    }
}

impl fmt::Display for Line<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for span in &self.spans {
            write!(f, "{span}")?;
        }
        Ok(())
    }
}

/// Multi-line rich text.
///
/// Text can contain multiple lines, each with its own spans and alignment.
///
/// # Example
///
/// ```
/// use tuxtui_core::text::{Text, Line};
/// use tuxtui_core::style::{Color, Style};
///
/// let text = Text::from(vec![
///     Line::from("First line"),
///     Line::styled("Second line", Style::default().fg(Color::Blue)),
/// ]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Text<'a> {
    /// The lines that make up this text
    pub lines: Vec<Line<'a>>,
    /// Overall style applied to all lines
    pub style: Style,
}

impl<'a> Text<'a> {
    /// Create new empty text.
    #[must_use]
    pub fn new() -> Self {
        Self {
            lines: Vec::new(),
            style: Style::default(),
        }
    }

    /// Create text from lines.
    #[must_use]
    pub fn from_lines(lines: Vec<Line<'a>>) -> Self {
        Self {
            lines,
            style: Style::default(),
        }
    }

    /// Create styled text from a string.
    #[must_use]
    pub fn styled<T: Into<Cow<'a, str>>>(content: T, style: Style) -> Self {
        let content = content.into();
        let lines: Vec<Line<'a>> = match content {
            Cow::Borrowed(s) => s.lines().map(|line| Line::styled(line, style)).collect(),
            Cow::Owned(s) => s.lines().map(|line| Line::styled(line.to_string(), style)).collect(),
        };
        Self {
            lines,
            style,
        }
    }

    /// Get the width of the widest line.
    #[must_use]
    pub fn width(&self) -> usize {
        self.lines.iter().map(Line::width).max().unwrap_or(0)
    }

    /// Get the number of lines.
    #[must_use]
    pub fn height(&self) -> usize {
        self.lines.len()
    }

    /// Convert this text to an owned version.
    #[must_use]
    pub fn into_owned(self) -> Text<'static> {
        Text {
            lines: self.lines.into_iter().map(Line::into_owned).collect(),
            style: self.style,
        }
    }

    /// Patch the style of this text (affects all lines).
    #[must_use]
    pub fn patch_style(mut self, style: Style) -> Self {
        self.style = self.style.patch(style);
        self
    }

    /// Push a line to this text.
    pub fn push_line(&mut self, line: Line<'a>) {
        self.lines.push(line);
    }

    /// Extend this text with more lines.
    pub fn extend_lines(&mut self, lines: impl IntoIterator<Item = Line<'a>>) {
        self.lines.extend(lines);
    }
}

impl<'a> Default for Text<'a> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> From<&'a str> for Text<'a> {
    fn from(s: &'a str) -> Self {
        let lines = s.lines().map(Line::from).collect();
        Self {
            lines,
            style: Style::default(),
        }
    }
}

impl From<String> for Text<'static> {
    fn from(s: String) -> Self {
        let lines: Vec<Line<'static>> = s
            .lines()
            .map(|line| Line::from(line.to_string()))
            .collect();
        Self {
            lines,
            style: Style::default(),
        }
    }
}

impl<'a> From<Line<'a>> for Text<'a> {
    fn from(line: Line<'a>) -> Self {
        Self::from_lines(alloc::vec![line])
    }
}

impl<'a> From<Vec<Line<'a>>> for Text<'a> {
    fn from(lines: Vec<Line<'a>>) -> Self {
        Self::from_lines(lines)
    }
}

impl<'a> Stylize for Text<'a> {
    fn style(self, style: Style) -> Self {
        self.patch_style(style)
    }
}

impl fmt::Display for Text<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, line) in self.lines.iter().enumerate() {
            if i > 0 {
                writeln!(f)?;
            }
            write!(f, "{line}")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::style::Color;

    #[test]
    fn test_span_width() {
        let span = Span::raw("Hello");
        assert_eq!(span.width(), 5);
    }

    #[test]
    fn test_line_width() {
        let line = Line::from(vec![
            Span::raw("Hello "),
            Span::raw("World"),
        ]);
        assert_eq!(line.width(), 11);
    }

    #[test]
    fn test_text_dimensions() {
        let text = Text::from("Hello\nWorld\n!");
        assert_eq!(text.height(), 3);
        assert_eq!(text.width(), 5);
    }

    #[test]
    fn test_stylize() {
        use crate::style::Stylize;
        let span = Span::raw("test").red().bold();
        assert_eq!(span.style.fg, Some(Color::Red));
    }
}
