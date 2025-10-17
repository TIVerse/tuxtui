//! Calendar widget (stub implementation).
//!
//! This is a placeholder for future calendar widget implementation.
//! Requires the `widget-calendar` feature flag.

use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::terminal::Widget;

/// A calendar widget.
///
/// This is currently a stub implementation.
#[derive(Debug, Clone, Default)]
pub struct Calendar {
    style: Style,
}

impl Calendar {
    /// Create a new calendar widget.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

impl Widget for Calendar {
    fn render(self, _area: Rect, _buf: &mut Buffer) {
        // Stub implementation
        // TODO: Implement full calendar widget in future version
    }
}
