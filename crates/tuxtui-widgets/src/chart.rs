//! Chart widget for plotting data with axes.

use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::terminal::Widget;

/// A data point in a chart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DataPoint {
    /// X coordinate
    pub x: f64,
    /// Y coordinate
    pub y: f64,
}

impl DataPoint {
    /// Create a new data point.
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

/// A dataset for a chart.
#[derive(Debug, Clone, PartialEq)]
pub struct Dataset<'a> {
    name: &'a str,
    data: &'a [DataPoint],
    style: Style,
    marker: char,
}

impl<'a> Dataset<'a> {
    /// Create a new dataset.
    #[must_use]
    pub const fn new(name: &'a str, data: &'a [DataPoint]) -> Self {
        Self {
            name,
            data,
            style: Style::new(),
            marker: '•',
        }
    }

    /// Set the style for this dataset.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the marker character.
    #[must_use]
    pub const fn marker(mut self, marker: char) -> Self {
        self.marker = marker;
        self
    }
}

/// A chart widget with axes and datasets.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::chart::{Chart, Dataset, DataPoint};
///
/// let data = vec![
///     DataPoint::new(0.0, 0.0),
///     DataPoint::new(1.0, 1.0),
///     DataPoint::new(2.0, 4.0),
/// ];
///
/// let dataset = Dataset::new("Series", &data);
/// let chart = Chart::default()
///     .datasets(&[dataset])
///     .x_bounds([0.0, 2.0])
///     .y_bounds([0.0, 4.0]);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct Chart<'a> {
    datasets: &'a [Dataset<'a>],
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    style: Style,
}

impl<'a> Default for Chart<'a> {
    fn default() -> Self {
        Self {
            datasets: &[],
            x_bounds: [0.0, 1.0],
            y_bounds: [0.0, 1.0],
            style: Style::default(),
        }
    }
}

impl<'a> Chart<'a> {
    /// Create a new chart.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            datasets: &[],
            x_bounds: [0.0, 1.0],
            y_bounds: [0.0, 1.0],
            style: Style::new(),
        }
    }

    /// Set the datasets to display.
    #[must_use]
    pub const fn datasets(mut self, datasets: &'a [Dataset<'a>]) -> Self {
        self.datasets = datasets;
        self
    }

    /// Set the X-axis bounds.
    #[must_use]
    pub const fn x_bounds(mut self, bounds: [f64; 2]) -> Self {
        self.x_bounds = bounds;
        self
    }

    /// Set the Y-axis bounds.
    #[must_use]
    pub const fn y_bounds(mut self, bounds: [f64; 2]) -> Self {
        self.y_bounds = bounds;
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    fn map_x(&self, x: f64, area: Rect) -> Option<u16> {
        let [x_min, x_max] = self.x_bounds;
        if x < x_min || x > x_max {
            return None;
        }
        let ratio = (x - x_min) / (x_max - x_min);
        Some(area.left() + (ratio * area.width as f64) as u16)
    }

    fn map_y(&self, y: f64, area: Rect) -> Option<u16> {
        let [y_min, y_max] = self.y_bounds;
        if y < y_min || y > y_max {
            return None;
        }
        let ratio = (y - y_min) / (y_max - y_min);
        Some(area.bottom() - 1 - (ratio * area.height as f64) as u16)
    }
}

impl Widget for Chart<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        // Draw datasets
        for dataset in self.datasets {
            for point in dataset.data {
                if let (Some(x), Some(y)) = (self.map_x(point.x, area), self.map_y(point.y, area)) {
                    if x >= area.left() && x < area.right() && y >= area.top() && y < area.bottom()
                    {
                        let marker_str = alloc::string::String::from(dataset.marker);
                        buf.set(x, y, &marker_str, dataset.style);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chart_creation() {
        let data = [DataPoint::new(0.0, 0.0), DataPoint::new(1.0, 1.0)];
        let dataset = Dataset::new("Test", &data);
        let datasets = [dataset];
        let chart = Chart::default().datasets(&datasets);
        assert_eq!(chart.datasets.len(), 1);
    }

    #[test]
    fn test_datapoint() {
        let point = DataPoint::new(1.5, 2.5);
        assert_eq!(point.x, 1.5);
        assert_eq!(point.y, 2.5);
    }
}
