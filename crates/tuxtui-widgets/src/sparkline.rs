//! Sparkline widget for compact data visualization.

use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::symbols;
use tuxtui_core::terminal::Widget;

/// A sparkline widget for rendering compact charts.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::sparkline::Sparkline;
///
/// let data = vec![1, 2, 3, 5, 8, 13, 21];
/// let sparkline = Sparkline::default()
///     .data(&data)
///     .style(Style::default().fg(Color::Cyan));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sparkline<'a> {
    data: &'a [u64],
    style: Style,
    max: Option<u64>,
}

impl<'a> Default for Sparkline<'a> {
    fn default() -> Self {
        Self {
            data: &[],
            style: Style::default(),
            max: None,
        }
    }
}

impl<'a> Sparkline<'a> {
    /// Create a new sparkline.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data: &[],
            style: Style::default(),
            max: None,
        }
    }

    /// Set the data to display.
    #[must_use]
    pub const fn data(mut self, data: &'a [u64]) -> Self {
        self.data = data;
        self
    }

    /// Set the style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the maximum value (for scaling).
    #[must_use]
    pub const fn max(mut self, max: u64) -> Self {
        self.max = Some(max);
        self
    }
}

impl Widget for Sparkline<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 || self.data.is_empty() {
            return;
        }

        let max_value = self.max.unwrap_or_else(|| {
            self.data.iter().copied().max().unwrap_or(1)
        });

        if max_value == 0 {
            return;
        }

        let height = area.height as u64;
        let width = area.width.min(self.data.len() as u16);

        for (i, &value) in self.data.iter().take(width as usize).enumerate() {
            let bar_height = (value * height * 8 / max_value).min(height * 8);
            let full_blocks = (bar_height / 8) as u16;
            let remainder = (bar_height % 8) as usize;

            let x = area.left() + i as u16;

            // Draw full blocks
            for j in 0..full_blocks {
                let y = area.bottom() - 1 - j;
                if y >= area.top() {
                    buf.set(x, y, symbols::BAR_FULL, self.style);
                }
            }

            // Draw partial block
            if remainder > 0 && full_blocks < area.height {
                let y = area.bottom() - 1 - full_blocks;
                if y >= area.top() {
                    let symbol = symbols::BLOCKS[remainder];
                    buf.set(x, y, symbol, self.style);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparkline_creation() {
        let data = vec![1, 2, 3, 4, 5];
        let sparkline = Sparkline::default().data(&data);
        assert_eq!(sparkline.data.len(), 5);
    }

    #[test]
    fn test_sparkline_max() {
        let data = vec![1, 2, 3];
        let sparkline = Sparkline::default().data(&data).max(10);
        assert_eq!(sparkline.max, Some(10));
    }
}
