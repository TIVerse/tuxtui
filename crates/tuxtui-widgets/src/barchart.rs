//! Bar chart widget for data visualization.

use alloc::string::String;
use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::symbols;
use tuxtui_core::terminal::Widget;

/// A bar in a bar chart.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bar<'a> {
    value: u64,
    label: Option<&'a str>,
    style: Style,
}

impl<'a> Bar<'a> {
    /// Create a new bar.
    #[must_use]
    pub const fn new(value: u64) -> Self {
        Self {
            value,
            label: None,
            style: Style::new(),
        }
    }

    /// Set the label for this bar.
    #[must_use]
    pub const fn label(mut self, label: &'a str) -> Self {
        self.label = Some(label);
        self
    }

    /// Set the style for this bar.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// A bar chart widget.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::barchart::{BarChart, Bar};
///
/// let bars = vec![
///     Bar::new(10).label("Jan"),
///     Bar::new(20).label("Feb"),
///     Bar::new(15).label("Mar"),
/// ];
///
/// let chart = BarChart::default()
///     .data(&bars)
///     .bar_width(3)
///     .bar_gap(1);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarChart<'a> {
    bars: &'a [Bar<'a>],
    style: Style,
    bar_width: u16,
    bar_gap: u16,
    max: Option<u64>,
}

impl<'a> Default for BarChart<'a> {
    fn default() -> Self {
        Self {
            bars: &[],
            style: Style::default(),
            bar_width: 3,
            bar_gap: 1,
            max: None,
        }
    }
}

impl<'a> BarChart<'a> {
    /// Create a new bar chart.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            bars: &[],
            style: Style::new(),
            bar_width: 3,
            bar_gap: 1,
            max: None,
        }
    }

    /// Set the bars data.
    #[must_use]
    pub const fn data(mut self, bars: &'a [Bar<'a>]) -> Self {
        self.bars = bars;
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the bar width.
    #[must_use]
    pub const fn bar_width(mut self, width: u16) -> Self {
        self.bar_width = width;
        self
    }

    /// Set the gap between bars.
    #[must_use]
    pub const fn bar_gap(mut self, gap: u16) -> Self {
        self.bar_gap = gap;
        self
    }

    /// Set the maximum value for scaling.
    #[must_use]
    pub const fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }
}

impl Widget for BarChart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 || self.bars.is_empty() {
            return;
        }

        let max_value = self.max.unwrap_or_else(|| {
            self.bars.iter().map(|b| b.value).max().unwrap_or(1)
        });

        if max_value == 0 {
            return;
        }

        let chart_height = area.height.saturating_sub(2); // Reserve 2 rows for labels
        let mut x = area.left();

        for bar in self.bars {
            if x + self.bar_width > area.right() {
                break;
            }

            let bar_height = ((bar.value * chart_height as u64) / max_value) as u16;
            let bar_style = self.style.patch(bar.style);

            // Draw bar
            for dy in 0..bar_height {
                let y = area.top() + chart_height - dy - 1;
                for dx in 0..self.bar_width {
                    buf.set(x + dx, y, symbols::BAR_FULL, bar_style);
                }
            }

            // Draw label
            if let Some(label) = bar.label {
                let label_y = area.top() + chart_height;
                let label_x = x + (self.bar_width.saturating_sub(label.len() as u16)) / 2;
                buf.set_string(label_x, label_y, label, self.style);
            }

            x += self.bar_width + self.bar_gap;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_barchart_creation() {
        let bars = [Bar::new(10), Bar::new(20), Bar::new(30)];
        let chart = BarChart::default().data(&bars);
        assert_eq!(chart.bars.len(), 3);
    }

    #[test]
    fn test_bar_with_label() {
        let bar = Bar::new(42).label("Test");
        assert_eq!(bar.value, 42);
        assert_eq!(bar.label, Some("Test"));
    }
}
