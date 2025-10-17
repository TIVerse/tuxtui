//! Geometric primitives for terminal layout.

use core::fmt;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A position in the terminal grid.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Position {
    /// X coordinate (column)
    pub x: u16,
    /// Y coordinate (row)
    pub y: u16,
}

impl Position {
    /// Create a new position.
    #[inline]
    #[must_use]
    pub const fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Calculate the distance to another position.
    #[inline]
    #[must_use]
    pub fn distance_to(self, other: Self) -> f64 {
        let dx = (other.x as f64) - (self.x as f64);
        let dy = (other.y as f64) - (self.y as f64);
        (dx * dx + dy * dy).sqrt()
    }
}

impl core::ops::Add for Position {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_add(rhs.x),
            y: self.y.saturating_add(rhs.y),
        }
    }
}

impl core::ops::Sub for Position {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x.saturating_sub(rhs.x),
            y: self.y.saturating_sub(rhs.y),
        }
    }
}

impl core::ops::AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x.saturating_add(rhs.x);
        self.y = self.y.saturating_add(rhs.y);
    }
}

impl core::ops::SubAssign for Position {
    fn sub_assign(&mut self, rhs: Self) {
        self.x = self.x.saturating_sub(rhs.x);
        self.y = self.y.saturating_sub(rhs.y);
    }
}

/// A rectangular region in the terminal.
///
/// Represents a region with position (x, y) and dimensions (width, height).
/// All coordinates use zero-based indexing.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Rect {
    /// X coordinate (column) of the top-left corner
    pub x: u16,
    /// Y coordinate (row) of the top-left corner
    pub y: u16,
    /// Width in columns
    pub width: u16,
    /// Height in rows
    pub height: u16,
}

impl Rect {
    /// Create a new rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::Rect;
    ///
    /// let rect = Rect::new(0, 0, 80, 24);
    /// assert_eq!(rect.width, 80);
    /// assert_eq!(rect.height, 24);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Create a zero-sized rectangle at the origin.
    #[inline]
    #[must_use]
    pub const fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    /// Get the area (width × height) of the rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::Rect;
    ///
    /// let rect = Rect::new(0, 0, 10, 5);
    /// assert_eq!(rect.area(), 50);
    /// ```
    #[inline]
    #[must_use]
    pub const fn area(self) -> u32 {
        self.width as u32 * self.height as u32
    }

    /// Check if the rectangle is empty (zero width or height).
    #[inline]
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }

    /// Get the left edge x coordinate.
    #[inline]
    #[must_use]
    pub const fn left(self) -> u16 {
        self.x
    }

    /// Get the right edge x coordinate (exclusive).
    #[inline]
    #[must_use]
    pub const fn right(self) -> u16 {
        self.x.saturating_add(self.width)
    }

    /// Get the top edge y coordinate.
    #[inline]
    #[must_use]
    pub const fn top(self) -> u16 {
        self.y
    }

    /// Get the bottom edge y coordinate (exclusive).
    #[inline]
    #[must_use]
    pub const fn bottom(self) -> u16 {
        self.y.saturating_add(self.height)
    }

    /// Get the position of the top-left corner.
    #[inline]
    #[must_use]
    pub const fn position(self) -> Position {
        Position::new(self.x, self.y)
    }

    /// Check if this rectangle contains a position.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::{Rect, Position};
    ///
    /// let rect = Rect::new(0, 0, 10, 10);
    /// assert!(rect.contains(Position::new(5, 5)));
    /// assert!(!rect.contains(Position::new(15, 5)));
    /// ```
    #[must_use]
    pub const fn contains(self, pos: Position) -> bool {
        pos.x >= self.x && pos.x < self.right() && pos.y >= self.y && pos.y < self.bottom()
    }

    /// Compute the intersection of two rectangles.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::Rect;
    ///
    /// let a = Rect::new(0, 0, 10, 10);
    /// let b = Rect::new(5, 5, 10, 10);
    /// let intersection = a.intersection(b);
    /// assert_eq!(intersection, Rect::new(5, 5, 5, 5));
    /// ```
    #[must_use]
    pub const fn intersection(self, other: Self) -> Self {
        let x1 = if self.x > other.x { self.x } else { other.x };
        let y1 = if self.y > other.y { self.y } else { other.y };
        let x2 = if self.right() < other.right() {
            self.right()
        } else {
            other.right()
        };
        let y2 = if self.bottom() < other.bottom() {
            self.bottom()
        } else {
            other.bottom()
        };
        Self::new(
            x1,
            y1,
            x2.saturating_sub(x1),
            y2.saturating_sub(y1),
        )
    }

    /// Check if this rectangle fully contains another rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::Rect;
    ///
    /// let outer = Rect::new(0, 0, 10, 10);
    /// let inner = Rect::new(2, 2, 5, 5);
    /// assert!(outer.contains_rect(inner));
    /// ```
    #[must_use]
    pub const fn contains_rect(self, other: Self) -> bool {
        other.x >= self.x
            && other.y >= self.y
            && other.right() <= self.right()
            && other.bottom() <= self.bottom()
    }

    /// Compute the union of two rectangles.
    #[must_use]
    pub const fn union(self, other: Self) -> Self {
        let x1 = if self.x < other.x { self.x } else { other.x };
        let y1 = if self.y < other.y { self.y } else { other.y };
        let x2 = if self.right() > other.right() {
            self.right()
        } else {
            other.right()
        };
        let y2 = if self.bottom() > other.bottom() {
            self.bottom()
        } else {
            other.bottom()
        };
        Self::new(x1, y1, x2 - x1, y2 - y1)
    }

    /// Apply a margin (padding) inset to the rectangle.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::{Rect, Margin};
    ///
    /// let rect = Rect::new(0, 0, 10, 10);
    /// let inner = rect.inner(Margin::new(1, 1));
    /// assert_eq!(inner, Rect::new(1, 1, 8, 8));
    /// ```
    #[must_use]
    pub const fn inner(self, margin: Margin) -> Self {
        let doubled_horizontal = margin.horizontal.saturating_mul(2);
        let doubled_vertical = margin.vertical.saturating_mul(2);
        Self::new(
            self.x.saturating_add(margin.horizontal),
            self.y.saturating_add(margin.vertical),
            self.width.saturating_sub(doubled_horizontal),
            self.height.saturating_sub(doubled_vertical),
        )
    }

    /// Clamp this rectangle to fit within another rectangle.
    #[must_use]
    pub const fn clamp(self, other: Self) -> Self {
        self.intersection(other)
    }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}x{}+{}+{}", self.width, self.height, self.x, self.y)
    }
}

/// Margin (padding) specification for rectangular insets.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Margin {
    /// Horizontal margin (left and right)
    pub horizontal: u16,
    /// Vertical margin (top and bottom)
    pub vertical: u16,
}

impl Margin {
    /// Create a new margin.
    ///
    /// # Example
    ///
    /// ```
    /// use tuxtui_core::geometry::Margin;
    ///
    /// let margin = Margin::new(1, 2);
    /// assert_eq!(margin.horizontal, 1);
    /// assert_eq!(margin.vertical, 2);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new(horizontal: u16, vertical: u16) -> Self {
        Self {
            horizontal,
            vertical,
        }
    }

    /// Create a uniform margin.
    #[inline]
    #[must_use]
    pub const fn uniform(value: u16) -> Self {
        Self::new(value, value)
    }
}

/// Alignment along an axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Alignment {
    /// Align to the start (left/top)
    Start,
    /// Center alignment
    Center,
    /// Align to the end (right/bottom)
    End,
}

impl Default for Alignment {
    fn default() -> Self {
        Self::Start
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rect_area() {
        assert_eq!(Rect::new(0, 0, 10, 10).area(), 100);
        assert_eq!(Rect::new(0, 0, 0, 10).area(), 0);
    }

    #[test]
    fn rect_contains() {
        let rect = Rect::new(5, 5, 10, 10);
        assert!(rect.contains(Position::new(5, 5)));
        assert!(rect.contains(Position::new(14, 14)));
        assert!(!rect.contains(Position::new(4, 5)));
        assert!(!rect.contains(Position::new(15, 5)));
    }

    #[test]
    fn rect_intersection() {
        let a = Rect::new(0, 0, 10, 10);
        let b = Rect::new(5, 5, 10, 10);
        assert_eq!(a.intersection(b), Rect::new(5, 5, 5, 5));

        let c = Rect::new(20, 20, 10, 10);
        assert_eq!(a.intersection(c), Rect::new(20, 20, 0, 0));
    }

    #[test]
    fn rect_inner() {
        let rect = Rect::new(0, 0, 10, 10);
        let inner = rect.inner(Margin::new(1, 1));
        assert_eq!(inner, Rect::new(1, 1, 8, 8));
    }
}
