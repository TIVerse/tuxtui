//! Symbol sets for box drawing and UI elements.

/// Line style for borders and separators.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LineStyle {
    /// Horizontal line
    pub horizontal: &'static str,
    /// Vertical line
    pub vertical: &'static str,
    /// Top-left corner
    pub top_left: &'static str,
    /// Top-right corner
    pub top_right: &'static str,
    /// Bottom-left corner
    pub bottom_left: &'static str,
    /// Bottom-right corner
    pub bottom_right: &'static str,
    /// Vertical and right intersection
    pub vertical_right: &'static str,
    /// Vertical and left intersection
    pub vertical_left: &'static str,
    /// Horizontal and down intersection
    pub horizontal_down: &'static str,
    /// Horizontal and up intersection
    pub horizontal_up: &'static str,
    /// Cross intersection
    pub cross: &'static str,
}

/// Simple single-line borders (ASCII-compatible).
pub const SIMPLE: LineStyle = LineStyle {
    horizontal: "-",
    vertical: "|",
    top_left: "+",
    top_right: "+",
    bottom_left: "+",
    bottom_right: "+",
    vertical_right: "+",
    vertical_left: "+",
    horizontal_down: "+",
    horizontal_up: "+",
    cross: "+",
};

/// Single-line box drawing characters.
pub const NORMAL: LineStyle = LineStyle {
    horizontal: "─",
    vertical: "│",
    top_left: "┌",
    top_right: "┐",
    bottom_left: "└",
    bottom_right: "┘",
    vertical_right: "├",
    vertical_left: "┤",
    horizontal_down: "┬",
    horizontal_up: "┴",
    cross: "┼",
};

/// Rounded corner box drawing.
pub const ROUNDED: LineStyle = LineStyle {
    horizontal: "─",
    vertical: "│",
    top_left: "╭",
    top_right: "╮",
    bottom_left: "╰",
    bottom_right: "╯",
    vertical_right: "├",
    vertical_left: "┤",
    horizontal_down: "┬",
    horizontal_up: "┴",
    cross: "┼",
};

/// Double-line box drawing.
pub const DOUBLE: LineStyle = LineStyle {
    horizontal: "═",
    vertical: "║",
    top_left: "╔",
    top_right: "╗",
    bottom_left: "╚",
    bottom_right: "╝",
    vertical_right: "╠",
    vertical_left: "╣",
    horizontal_down: "╦",
    horizontal_up: "╩",
    cross: "╬",
};

/// Thick line style.
pub const THICK: LineStyle = LineStyle {
    horizontal: "━",
    vertical: "┃",
    top_left: "┏",
    top_right: "┓",
    bottom_left: "┗",
    bottom_right: "┛",
    vertical_right: "┣",
    vertical_left: "┫",
    horizontal_down: "┳",
    horizontal_up: "┻",
    cross: "╋",
};

/// Scrollbar symbols.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScrollbarSymbols {
    /// The track symbol
    pub track: &'static str,
    /// The thumb symbol
    pub thumb: &'static str,
    /// Begin arrow
    pub begin: &'static str,
    /// End arrow
    pub end: &'static str,
}

/// Default scrollbar symbols.
pub const SCROLLBAR_DEFAULT: ScrollbarSymbols = ScrollbarSymbols {
    track: "│",
    thumb: "█",
    begin: "▲",
    end: "▼",
};

/// Scrollbar with block symbols.
pub const SCROLLBAR_BLOCK: ScrollbarSymbols = ScrollbarSymbols {
    track: "░",
    thumb: "█",
    begin: "▲",
    end: "▼",
};

/// Bar chart symbols.
pub const BAR_FULL: &str = "█";
/// Seven-eighths filled bar symbol.
pub const BAR_SEVEN_EIGHTHS: &str = "▉";
/// Three-quarters filled bar symbol.
pub const BAR_THREE_QUARTERS: &str = "▊";
/// Five-eighths filled bar symbol.
pub const BAR_FIVE_EIGHTHS: &str = "▋";
/// Half filled bar symbol.
pub const BAR_HALF: &str = "▌";
/// Three-eighths filled bar symbol.
pub const BAR_THREE_EIGHTHS: &str = "▍";
/// Quarter filled bar symbol.
pub const BAR_QUARTER: &str = "▎";
/// One-eighth filled bar symbol.
pub const BAR_ONE_EIGHTH: &str = "▏";

/// Block symbols for different fill levels.
pub const BLOCKS: [&str; 9] = [
    " ",
    BAR_ONE_EIGHTH,
    BAR_QUARTER,
    BAR_THREE_EIGHTHS,
    BAR_HALF,
    BAR_FIVE_EIGHTHS,
    BAR_THREE_QUARTERS,
    BAR_SEVEN_EIGHTHS,
    BAR_FULL,
];

/// Dot symbols for charts.
pub const DOT: &str = "•";
/// Filled bullet point symbol.
pub const BULLET: &str = "●";
/// Empty circle symbol.
pub const CIRCLE: &str = "○";

/// Marker symbols for lists.
pub const MARKER_DOT: &str = "•";
/// Arrow marker symbol.
pub const MARKER_ARROW: &str = "→";
/// Angle bracket marker symbol.
pub const MARKER_ANGLE: &str = "❯";
/// Check mark symbol.
pub const MARKER_CHECK: &str = "✓";

/// Braille patterns for high-resolution canvas rendering.
pub mod braille {
    /// Get a braille character for the given pattern.
    ///
    /// The bits represent dots in this layout:
    /// ```text
    /// 0 3
    /// 1 4
    /// 2 5
    /// 6 7
    /// ```
    #[must_use]
    pub fn char_from_bits(bits: u8) -> char {
        // Braille pattern base is U+2800
        char::from_u32(0x2800 + bits as u32).unwrap_or('?')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_styles() {
        assert_eq!(NORMAL.horizontal, "─");
        assert_eq!(ROUNDED.top_left, "╭");
        assert_eq!(DOUBLE.vertical, "║");
    }

    #[test]
    fn test_braille() {
        let c = braille::char_from_bits(0b11111111);
        assert_eq!(c, '⣿');
    }
}
