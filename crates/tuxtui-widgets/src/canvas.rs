//! Canvas widget for low-level drawing.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::style::Style;
use tuxtui_core::symbols::braille;
use tuxtui_core::terminal::Widget;

/// A shape that can be drawn on a canvas.
pub trait Shape {
    /// Draw this shape into the canvas context.
    fn draw(&self, ctx: &mut CanvasContext);
}

/// Canvas drawing context.
pub struct CanvasContext {
    /// X bounds [min, max]
    pub x_bounds: [f64; 2],
    /// Y bounds [min, max]
    pub y_bounds: [f64; 2],
    /// Canvas area
    pub area: Rect,
    /// Braille grid (4x wider, 2x taller than area for sub-cell resolution)
    grid: Vec<Vec<bool>>,
    /// Style for drawing
    pub style: Style,
}

impl CanvasContext {
    /// Create a new canvas context.
    #[must_use]
    pub fn new(area: Rect, x_bounds: [f64; 2], y_bounds: [f64; 2]) -> Self {
        let grid_width = area.width as usize * 2;
        let grid_height = area.height as usize * 4;
        let grid = vec![vec![false; grid_width]; grid_height];

        Self {
            x_bounds,
            y_bounds,
            area,
            grid,
            style: Style::new(),
        }
    }

    /// Map a world coordinate to grid coordinate.
    fn map_to_grid(&self, x: f64, y: f64) -> Option<(usize, usize)> {
        let [x_min, x_max] = self.x_bounds;
        let [y_min, y_max] = self.y_bounds;

        if x < x_min || x > x_max || y < y_min || y > y_max {
            return None;
        }

        let x_ratio = (x - x_min) / (x_max - x_min);
        let y_ratio = (y - y_min) / (y_max - y_min);

        let gx = (x_ratio * self.grid[0].len() as f64) as usize;
        let gy = ((1.0 - y_ratio) * self.grid.len() as f64) as usize;

        if gx < self.grid[0].len() && gy < self.grid.len() {
            Some((gx, gy))
        } else {
            None
        }
    }

    /// Set a pixel in the grid.
    pub fn draw_point(&mut self, x: f64, y: f64) {
        if let Some((gx, gy)) = self.map_to_grid(x, y) {
            self.grid[gy][gx] = true;
        }
    }

    /// Draw a line between two points.
    pub fn draw_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        // Bresenham-like algorithm in floating point
        let steps = 100;
        for i in 0..=steps {
            let t = i as f64 / steps as f64;
            let x = x1 + t * (x2 - x1);
            let y = y1 + t * (y2 - y1);
            self.draw_point(x, y);
        }
    }

    /// Draw a rectangle.
    pub fn draw_rectangle(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        self.draw_line(x1, y1, x2, y1);
        self.draw_line(x2, y1, x2, y2);
        self.draw_line(x2, y2, x1, y2);
        self.draw_line(x1, y2, x1, y1);
    }

    /// Render the canvas to a buffer.
    pub fn render(&self, buf: &mut Buffer) {
        for cell_y in 0..self.area.height {
            for cell_x in 0..self.area.width {
                let mut bits = 0u8;

                // Each braille character represents a 2x4 grid
                for dy in 0..4 {
                    for dx in 0..2 {
                        let gx = cell_x as usize * 2 + dx;
                        let gy = cell_y as usize * 4 + dy;

                        if gy < self.grid.len() && gx < self.grid[0].len() && self.grid[gy][gx] {
                            let bit_index = match (dx, dy) {
                                (0, 0) => 0,
                                (0, 1) => 1,
                                (0, 2) => 2,
                                (1, 0) => 3,
                                (1, 1) => 4,
                                (1, 2) => 5,
                                (0, 3) => 6,
                                (1, 3) => 7,
                                _ => 0,
                            };
                            bits |= 1 << bit_index;
                        }
                    }
                }

                let ch = braille::char_from_bits(bits);
                let ch_str = alloc::string::String::from(ch);
                buf.set(
                    self.area.left() + cell_x,
                    self.area.top() + cell_y,
                    &ch_str,
                    self.style,
                );
            }
        }
    }
}

/// A canvas widget for custom drawing.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::canvas::Canvas;
///
/// let canvas = Canvas::default()
///     .x_bounds([0.0, 10.0])
///     .y_bounds([0.0, 10.0])
///     .paint(|ctx| {
///         ctx.draw_line(0.0, 0.0, 10.0, 10.0);
///     });
/// ```
pub struct Canvas<'a> {
    x_bounds: [f64; 2],
    y_bounds: [f64; 2],
    style: Style,
    painter: Option<&'a dyn Fn(&mut CanvasContext)>,
}

impl<'a> Default for Canvas<'a> {
    fn default() -> Self {
        Self {
            x_bounds: [0.0, 1.0],
            y_bounds: [0.0, 1.0],
            style: Style::new(),
            painter: None,
        }
    }
}

impl<'a> Canvas<'a> {
    /// Create a new canvas.
    #[must_use]
    pub fn new() -> Self {
        Self {
            x_bounds: [0.0, 1.0],
            y_bounds: [0.0, 1.0],
            style: Style::new(),
            painter: None,
        }
    }

    /// Set the X bounds.
    #[must_use]
    pub const fn x_bounds(mut self, bounds: [f64; 2]) -> Self {
        self.x_bounds = bounds;
        self
    }

    /// Set the Y bounds.
    #[must_use]
    pub const fn y_bounds(mut self, bounds: [f64; 2]) -> Self {
        self.y_bounds = bounds;
        self
    }

    /// Set the style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the paint function.
    #[must_use]
    pub const fn paint(mut self, painter: &'a dyn Fn(&mut CanvasContext)) -> Self {
        self.painter = Some(painter);
        self
    }
}

impl Widget for Canvas<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.area() == 0 {
            return;
        }

        let mut ctx = CanvasContext::new(area, self.x_bounds, self.y_bounds);
        ctx.style = self.style;

        if let Some(painter) = self.painter {
            painter(&mut ctx);
        }

        ctx.render(buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_context() {
        let area = Rect::new(0, 0, 10, 10);
        let mut ctx = CanvasContext::new(area, [0.0, 10.0], [0.0, 10.0]);
        ctx.draw_point(5.0, 5.0);
    }
}
