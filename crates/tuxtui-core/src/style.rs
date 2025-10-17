//! Style primitives for terminal text and widgets.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Error type for color parsing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseColorError {
    input: alloc::string::String,
}

impl core::fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "invalid color string: '{}'", self.input)
    }
}

#[cfg(feature = "std")]
impl std::error::Error for ParseColorError {}

/// Terminal colors supporting indexed, RGB, and named colors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Color {
    /// Reset to default terminal color
    Reset,
    /// Black (0)
    Black,
    /// Red (1)
    Red,
    /// Green (2)
    Green,
    /// Yellow (3)
    Yellow,
    /// Blue (4)
    Blue,
    /// Magenta (5)
    Magenta,
    /// Cyan (6)
    Cyan,
    /// White/Gray (7)
    White,
    /// Bright black/gray (8)
    Gray,
    /// Bright red (9)
    LightRed,
    /// Bright green (10)
    LightGreen,
    /// Bright yellow (11)
    LightYellow,
    /// Bright blue (12)
    LightBlue,
    /// Bright magenta (13)
    LightMagenta,
    /// Bright cyan (14)
    LightCyan,
    /// Bright white (15)
    LightGray,
    /// 8-bit indexed color (0-255)
    Indexed(u8),
    /// 24-bit RGB color
    Rgb(u8, u8, u8),
}

impl Color {
    /// Create an RGB color.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::style::Color;
    ///
    /// let color = Color::rgb(255, 128, 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb(r, g, b)
    }

    /// Create an indexed color (0-255).
    #[inline]
    #[must_use]
    pub const fn indexed(index: u8) -> Self {
        Self::Indexed(index)
    }

    /// Parse a color from a string.
    ///
    /// Supports:
    /// - Named colors: "red", "blue", "green", etc.
    /// - Hex colors: "#FF0000", "#F00"
    /// - RGB: "rgb(255, 0, 0)"
    /// - Indexed: "0" through "255"
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::style::Color;
    ///
    /// let red = Color::from_str("red").unwrap();
    /// let hex = Color::from_str("#FF0000").unwrap();
    /// let rgb = Color::from_str("rgb(255, 0, 0)").unwrap();
    /// ```
    pub fn from_str(s: &str) -> Result<Self, ParseColorError> {
        let s = s.trim().to_lowercase();
        
        // Named colors
        match s.as_str() {
            "reset" => return Ok(Self::Reset),
            "black" => return Ok(Self::Black),
            "red" => return Ok(Self::Red),
            "green" => return Ok(Self::Green),
            "yellow" => return Ok(Self::Yellow),
            "blue" => return Ok(Self::Blue),
            "magenta" => return Ok(Self::Magenta),
            "cyan" => return Ok(Self::Cyan),
            "white" => return Ok(Self::White),
            "gray" | "grey" => return Ok(Self::Gray),
            "lightred" | "light_red" => return Ok(Self::LightRed),
            "lightgreen" | "light_green" => return Ok(Self::LightGreen),
            "lightyellow" | "light_yellow" => return Ok(Self::LightYellow),
            "lightblue" | "light_blue" => return Ok(Self::LightBlue),
            "lightmagenta" | "light_magenta" => return Ok(Self::LightMagenta),
            "lightcyan" | "light_cyan" => return Ok(Self::LightCyan),
            "lightgray" | "light_gray" | "lightgrey" | "light_grey" => return Ok(Self::LightGray),
            _ => {}
        }

        // Hex colors (#RGB or #RRGGBB)
        if let Some(hex) = s.strip_prefix('#') {
            return Self::parse_hex(hex).ok_or_else(|| ParseColorError {
                input: s.into(),
            });
        }

        // RGB format: rgb(r, g, b)
        if let Some(rgb) = s.strip_prefix("rgb(") {
            if let Some(rgb) = rgb.strip_suffix(')') {
                return Self::parse_rgb(rgb).ok_or_else(|| ParseColorError {
                    input: s.into(),
                });
            }
        }

        // Indexed color (0-255)
        if let Ok(index) = s.parse::<u8>() {
            return Ok(Self::Indexed(index));
        }

        Err(ParseColorError { input: s.into() })
    }

    fn parse_hex(hex: &str) -> Option<Self> {
        match hex.len() {
            3 => {
                // #RGB -> #RRGGBB
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Self::Rgb(r, g, b))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::Rgb(r, g, b))
            }
            _ => None,
        }
    }

    fn parse_rgb(rgb: &str) -> Option<Self> {
        let parts: alloc::vec::Vec<&str> = rgb.split(',').map(str::trim).collect();
        if parts.len() != 3 {
            return None;
        }
        let r = parts[0].parse().ok()?;
        let g = parts[1].parse().ok()?;
        let b = parts[2].parse().ok()?;
        Some(Self::Rgb(r, g, b))
    }
}

impl Default for Color {
    fn default() -> Self {
        Self::Reset
    }
}

bitflags::bitflags! {
    /// Text style modifiers (bold, italic, underline, etc.).
    ///
    /// Multiple modifiers can be combined using bitwise OR.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::style::Modifier;
    ///
    /// let mods = Modifier::BOLD | Modifier::ITALIC;
    /// assert!(mods.contains(Modifier::BOLD));
    /// ```
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
    pub struct Modifier: u16 {
        /// Bold text
        const BOLD              = 0b0000_0000_0001;
        /// Dimmed text
        const DIM               = 0b0000_0000_0010;
        /// Italic text
        const ITALIC            = 0b0000_0000_0100;
        /// Underlined text
        const UNDERLINED        = 0b0000_0000_1000;
        /// Slow blink
        const SLOW_BLINK        = 0b0000_0001_0000;
        /// Rapid blink
        const RAPID_BLINK       = 0b0000_0010_0000;
        /// Reverse video (swap fg/bg)
        const REVERSED          = 0b0000_0100_0000;
        /// Hidden/invisible text
        const HIDDEN            = 0b0000_1000_0000;
        /// Strikethrough text
        const CROSSED_OUT       = 0b0001_0000_0000;
    }
}

impl Default for Modifier {
    fn default() -> Self {
        Self::empty()
    }
}

/// A complete style specification for text or widgets.
///
/// Styles can be composed and merged, with later values taking precedence.
///
/// # Example
///
/// ```
/// use tuxtui_core::style::{Color, Style, Modifier};
///
/// let style = Style::default()
///     .fg(Color::Blue)
///     .bg(Color::Black)
///     .add_modifier(Modifier::BOLD);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Style {
    /// Foreground color
    pub fg: Option<Color>,
    /// Background color
    pub bg: Option<Color>,
    /// Underline color (if `underline-color` feature enabled)
    #[cfg(feature = "underline-color")]
    pub underline_color: Option<Color>,
    /// Text modifiers
    pub add_modifier: Modifier,
    /// Modifiers to remove
    pub sub_modifier: Modifier,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            fg: None,
            bg: None,
            #[cfg(feature = "underline-color")]
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }
}

impl Style {
    /// Create a new default style.
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self {
            fg: None,
            bg: None,
            #[cfg(feature = "underline-color")]
            underline_color: None,
            add_modifier: Modifier::empty(),
            sub_modifier: Modifier::empty(),
        }
    }

    /// Set the foreground color.
    #[inline]
    #[must_use]
    pub const fn fg(mut self, color: Color) -> Self {
        self.fg = Some(color);
        self
    }

    /// Set the background color.
    #[inline]
    #[must_use]
    pub const fn bg(mut self, color: Color) -> Self {
        self.bg = Some(color);
        self
    }

    /// Set the underline color (requires `underline-color` feature).
    #[cfg(feature = "underline-color")]
    #[inline]
    #[must_use]
    pub const fn underline_color(mut self, color: Color) -> Self {
        self.underline_color = Some(color);
        self
    }

    /// Add modifiers.
    #[inline]
    #[must_use]
    pub const fn add_modifier(mut self, modifier: Modifier) -> Self {
        self.add_modifier = self.add_modifier.union(modifier);
        self
    }

    /// Remove modifiers.
    #[inline]
    #[must_use]
    pub const fn remove_modifier(mut self, modifier: Modifier) -> Self {
        self.sub_modifier = self.sub_modifier.union(modifier);
        self
    }

    /// Reset the style to default.
    #[inline]
    #[must_use]
    pub const fn reset() -> Self {
        Self::new()
    }

    /// Patch this style with another, taking non-None values from `other`.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::style::{Color, Style};
    ///
    /// let base = Style::default().fg(Color::Red);
    /// let patch = Style::default().bg(Color::Blue);
    /// let merged = base.patch(patch);
    ///
    /// assert_eq!(merged.fg, Some(Color::Red));
    /// assert_eq!(merged.bg, Some(Color::Blue));
    /// ```
    #[must_use]
    pub const fn patch(mut self, other: Self) -> Self {
        if other.fg.is_some() {
            self.fg = other.fg;
        }
        if other.bg.is_some() {
            self.bg = other.bg;
        }
        #[cfg(feature = "underline-color")]
        if other.underline_color.is_some() {
            self.underline_color = other.underline_color;
        }
        self.add_modifier = self.add_modifier.union(other.add_modifier);
        self.sub_modifier = self.sub_modifier.union(other.sub_modifier);
        self
    }
}

/// A trait for types that can be styled.
///
/// This provides a fluent API for applying styles to text and widgets.
///
/// # Example
///
/// ```
/// use tuxtui_core::style::{Color, Stylize};
///
/// let text = "Hello".blue().bold();
/// ```
pub trait Stylize: Sized {
    /// Apply a style to this item.
    fn style(self, style: Style) -> Self;

    /// Set the foreground color.
    #[inline]
    fn fg(self, color: Color) -> Self {
        self.style(Style::default().fg(color))
    }

    /// Set the background color.
    #[inline]
    fn bg(self, color: Color) -> Self {
        self.style(Style::default().bg(color))
    }

    /// Make the text black.
    #[inline]
    fn black(self) -> Self {
        self.fg(Color::Black)
    }

    /// Make the text red.
    #[inline]
    fn red(self) -> Self {
        self.fg(Color::Red)
    }

    /// Make the text green.
    #[inline]
    fn green(self) -> Self {
        self.fg(Color::Green)
    }

    /// Make the text yellow.
    #[inline]
    fn yellow(self) -> Self {
        self.fg(Color::Yellow)
    }

    /// Make the text blue.
    #[inline]
    fn blue(self) -> Self {
        self.fg(Color::Blue)
    }

    /// Make the text magenta.
    #[inline]
    fn magenta(self) -> Self {
        self.fg(Color::Magenta)
    }

    /// Make the text cyan.
    #[inline]
    fn cyan(self) -> Self {
        self.fg(Color::Cyan)
    }

    /// Make the text white.
    #[inline]
    fn white(self) -> Self {
        self.fg(Color::White)
    }

    /// Make the text gray.
    #[inline]
    fn gray(self) -> Self {
        self.fg(Color::Gray)
    }

    /// Make the text bold.
    #[inline]
    fn bold(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::BOLD))
    }

    /// Make the text dim.
    #[inline]
    fn dim(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::DIM))
    }

    /// Make the text italic.
    #[inline]
    fn italic(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::ITALIC))
    }

    /// Make the text underlined.
    #[inline]
    fn underlined(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::UNDERLINED))
    }

    /// Make the text blink slowly.
    #[inline]
    fn slow_blink(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::SLOW_BLINK))
    }

    /// Make the text blink rapidly.
    #[inline]
    fn rapid_blink(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::RAPID_BLINK))
    }

    /// Reverse the foreground and background colors.
    #[inline]
    fn reversed(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::REVERSED))
    }

    /// Make the text hidden.
    #[inline]
    fn hidden(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::HIDDEN))
    }

    /// Make the text crossed out.
    #[inline]
    fn crossed_out(self) -> Self {
        self.style(Style::default().add_modifier(Modifier::CROSSED_OUT))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_rgb() {
        let color = Color::rgb(255, 128, 64);
        assert_eq!(color, Color::Rgb(255, 128, 64));
    }

    #[test]
    fn test_modifier_bitflags() {
        let mods = Modifier::BOLD | Modifier::ITALIC;
        assert!(mods.contains(Modifier::BOLD));
        assert!(mods.contains(Modifier::ITALIC));
        assert!(!mods.contains(Modifier::UNDERLINED));
    }

    #[test]
    fn test_style_patch() {
        let base = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
        let patch = Style::default()
            .bg(Color::Blue)
            .add_modifier(Modifier::ITALIC);
        let merged = base.patch(patch);

        assert_eq!(merged.fg, Some(Color::Red));
        assert_eq!(merged.bg, Some(Color::Blue));
        assert!(merged.add_modifier.contains(Modifier::BOLD));
        assert!(merged.add_modifier.contains(Modifier::ITALIC));
    }
}
