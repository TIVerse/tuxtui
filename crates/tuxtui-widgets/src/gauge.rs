//! Gauge widget for displaying progress.

use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::symbols;
use tuxtui_core::terminal::Widget;

/// A gauge (progress bar) widget.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::gauge::Gauge;
///
/// let gauge = Gauge::default()
///     .percent(75)
///     .label("75%")
///     .style(Style::default().fg(Color::Yellow));
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Gauge<'a> {
    percent: u16,
    label: Option<&'a str>,
    style: Style,
    gauge_style: Style,
}

impl<'a> Gauge<'a> {
    /// Create a new gauge.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            percent: 0,
            label: None,
            style: Style::new(),
            gauge_style: Style::new(),
        }
    }

    /// Set the percentage (0-100).
    #[must_use]
    pub const fn percent(mut self, percent: u16) -> Self {
        self.percent = if percent > 100 { 100 } else { percent };
        self
    }

    /// Set the label.
    #[must_use]
    pub const fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the gauge fill style.
    #[must_use]
    pub const fn gauge_style(mut self, style: Style) -> Self {
        self.gauge_style = style;
        self
    }
}

impl Widget for Gauge<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        // Calculate filled width
        let filled_width = (area.width as u32 * self.percent as u32 / 100) as u16;

        // Render filled portion
        for y in area.top()..area.bottom() {
            for x in area.left()..area.left() + filled_width {
                buf.set(x, y, symbols::BAR_FULL, self.gauge_style);
            }
        }

        // Render unfilled portion
        for y in area.top()..area.bottom() {
            for x in (area.left() + filled_width)..area.right() {
                buf.set(x, y, " ", self.style);
            }
        }

        // Render label (centered)
        if let Some(label) = self.label {
            let label_width = label.len() as u16;
            if label_width <= area.width {
                let x = area.left() + (area.width - label_width) / 2;
                let y = area.top() + area.height / 2;
                buf.set_string(x, y, label, self.style);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tuxtui_core::style::Color;

    #[test]
    fn test_gauge_percent() {
        let gauge = Gauge::default().percent(50);
        assert_eq!(gauge.percent, 50);

        let gauge = Gauge::default().percent(150);
        assert_eq!(gauge.percent, 100);
    }
}
