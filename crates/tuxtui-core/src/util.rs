//! Utility functions and helpers.

use unicode_width::UnicodeWidthStr;

/// Calculate the display width of a string, respecting grapheme clusters.
///
/// # Example
///
/// ```
/// use tuxtui_core::util::string_width;
///
/// assert_eq!(string_width("Hello"), 5);
/// assert_eq!(string_width("你好"), 4); // CJK characters are 2 cells wide
/// ```
#[must_use]
pub fn string_width(s: &str) -> usize {
    s.width()
}

/// Truncate a string to fit within a given width, adding an ellipsis if needed.
///
/// # Example
///
/// ```
/// use tuxtui_core::util::truncate_string;
///
/// let result = truncate_string("Hello, world!", 8);
/// assert_eq!(result, "Hello...");
/// ```
#[must_use]
pub fn truncate_string(s: &str, max_width: usize) -> alloc::string::String {
    let width = s.width();
    if width <= max_width {
        return s.to_string();
    }

    if max_width < 3 {
        return alloc::string::String::from("...");
    }

    let mut result = alloc::string::String::new();
    let mut current_width = 0;
    let target_width = max_width - 3; // Reserve space for "..."

    for grapheme in unicode_segmentation::UnicodeSegmentation::graphemes(s, true) {
        let grapheme_width = grapheme.width();
        if current_width + grapheme_width > target_width {
            break;
        }
        result.push_str(grapheme);
        current_width += grapheme_width;
    }

    result.push_str("...");
    result
}

/// Wrap text to fit within a given width.
///
/// Returns a vector of lines.
#[must_use]
pub fn wrap_text(text: &str, width: usize) -> alloc::vec::Vec<alloc::string::String> {
    let mut lines = alloc::vec::Vec::new();
    let mut current_line = alloc::string::String::new();
    let mut current_width = 0;

    for word in text.split_whitespace() {
        let word_width = word.width();

        if current_width + word_width + 1 > width {
            if !current_line.is_empty() {
                lines.push(current_line);
                current_line = alloc::string::String::new();
                current_width = 0;
            }
        }

        if !current_line.is_empty() {
            current_line.push(' ');
            current_width += 1;
        }

        current_line.push_str(word);
        current_width += word_width;
    }

    if !current_line.is_empty() {
        lines.push(current_line);
    }

    if lines.is_empty() {
        lines.push(alloc::string::String::new());
    }

    lines
}

/// Detect if the terminal likely supports truecolor (24-bit RGB).
///
/// This checks common environment variables but is not foolproof.
#[must_use]
pub fn supports_truecolor() -> bool {
    #[cfg(feature = "std")]
    {
        if let Ok(colorterm) = std::env::var("COLORTERM") {
            if colorterm == "truecolor" || colorterm == "24bit" {
                return true;
            }
        }

        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256color") || term.contains("24bit") {
                return true;
            }
        }
    }

    false
}

/// Detect the approximate color support level of the terminal.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
    /// No color support
    None,
    /// 16 colors
    Ansi16,
    /// 256 colors
    Ansi256,
    /// 24-bit RGB (truecolor)
    TrueColor,
}

/// Detect the color support level of the terminal.
#[must_use]
pub fn detect_color_support() -> ColorSupport {
    #[cfg(feature = "std")]
    {
        if supports_truecolor() {
            return ColorSupport::TrueColor;
        }

        if let Ok(term) = std::env::var("TERM") {
            if term.contains("256") {
                return ColorSupport::Ansi256;
            }
            if term != "dumb" && !term.is_empty() {
                return ColorSupport::Ansi16;
            }
        }
    }

    ColorSupport::Ansi16
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_string_width() {
        assert_eq!(string_width("Hello"), 5);
        assert_eq!(string_width(""), 0);
    }

    #[test]
    fn test_truncate_string() {
        assert_eq!(truncate_string("Hello, world!", 8), "Hello...");
        assert_eq!(truncate_string("Hi", 10), "Hi");
    }

    #[test]
    fn test_wrap_text() {
        let lines = wrap_text("Hello world this is a test", 10);
        assert!(lines.len() > 1);
        assert!(lines[0].width() <= 10);
    }

    #[test]
    fn test_color_support() {
        let support = detect_color_support();
        assert!(matches!(
            support,
            ColorSupport::None
                | ColorSupport::Ansi16
                | ColorSupport::Ansi256
                | ColorSupport::TrueColor
        ));
    }
}
