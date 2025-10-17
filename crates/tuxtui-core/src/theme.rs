//! Theme system for consistent styling across widgets.

use crate::style::{Color, Style};
use alloc::string::String;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A complete theme specification for the TUI.
///
/// Themes provide consistent styling across all widgets.
///
/// # Example
///
/// ```
/// use tuxtui_core::theme::Theme;
/// use tuxtui_core::style::Color;
///
/// let theme = Theme::dark();
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Theme {
    /// Name of the theme
    pub name: String,
    /// Color palette
    pub palette: PaletteTheme,
    /// Widget-specific styles
    pub widgets: WidgetTheme,
}

impl Theme {
    /// Create a new theme.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            palette: PaletteTheme::default(),
            widgets: WidgetTheme::default(),
        }
    }

    /// Create a dark theme.
    #[must_use]
    pub fn dark() -> Self {
        Self {
            name: String::from("dark"),
            palette: PaletteTheme {
                background: Color::Black,
                foreground: Color::White,
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::Magenta,
                error: Color::Red,
                warning: Color::Yellow,
                success: Color::Green,
                muted: Color::Gray,
            },
            widgets: WidgetTheme::default(),
        }
    }

    /// Create a light theme.
    #[must_use]
    pub fn light() -> Self {
        Self {
            name: String::from("light"),
            palette: PaletteTheme {
                background: Color::White,
                foreground: Color::Black,
                primary: Color::Blue,
                secondary: Color::Cyan,
                accent: Color::Magenta,
                error: Color::Red,
                warning: Color::Yellow,
                success: Color::Green,
                muted: Color::Gray,
            },
            widgets: WidgetTheme::default(),
        }
    }

    /// Create a high-contrast theme.
    #[must_use]
    pub fn high_contrast() -> Self {
        Self {
            name: String::from("high-contrast"),
            palette: PaletteTheme {
                background: Color::Black,
                foreground: Color::White,
                primary: Color::LightBlue,
                secondary: Color::LightCyan,
                accent: Color::LightMagenta,
                error: Color::LightRed,
                warning: Color::LightYellow,
                success: Color::LightGreen,
                muted: Color::LightGray,
            },
            widgets: WidgetTheme::default(),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}

/// Color palette for a theme.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PaletteTheme {
    /// Background color
    pub background: Color,
    /// Foreground color
    pub foreground: Color,
    /// Primary accent color
    pub primary: Color,
    /// Secondary accent color
    pub secondary: Color,
    /// Accent color
    pub accent: Color,
    /// Error color
    pub error: Color,
    /// Warning color
    pub warning: Color,
    /// Success color
    pub success: Color,
    /// Muted/disabled color
    pub muted: Color,
}

impl Default for PaletteTheme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            primary: Color::Blue,
            secondary: Color::Cyan,
            accent: Color::Magenta,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            muted: Color::Gray,
        }
    }
}

/// Widget-specific theme styles.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct WidgetTheme {
    /// Block/border style
    pub block: Style,
    /// Selected item style
    pub selected: Style,
    /// Highlighted item style
    pub highlighted: Style,
    /// Active/focused style
    pub active: Style,
    /// Inactive style
    pub inactive: Style,
}

impl Default for WidgetTheme {
    fn default() -> Self {
        Self {
            block: Style::new().fg(Color::White),
            selected: Style::new().fg(Color::Black).bg(Color::White),
            highlighted: Style::new().fg(Color::Yellow),
            active: Style::new().fg(Color::Green),
            inactive: Style::new().fg(Color::Gray),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_theme_creation() {
        let theme = Theme::dark();
        assert_eq!(theme.name, "dark");
        assert_eq!(theme.palette.background, Color::Black);
    }

    #[test]
    fn test_light_theme() {
        let theme = Theme::light();
        assert_eq!(theme.palette.background, Color::White);
    }

    #[cfg(feature = "serde")]
    #[test]
    fn test_theme_serialization() {
        let theme = Theme::dark();
        let json = serde_json::to_string(&theme).unwrap();
        let deserialized: Theme = serde_json::from_str(&json).unwrap();
        assert_eq!(theme, deserialized);
    }
}
