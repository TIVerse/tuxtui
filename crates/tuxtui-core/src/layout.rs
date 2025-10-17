//! Flexible constraint-based layout engine with caching.

use crate::geometry::Rect;
use alloc::vec::Vec;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

#[cfg(feature = "layout-cache")]
use core::num::NonZeroUsize;
#[cfg(feature = "layout-cache")]
use lru::LruCache;

/// Layout constraints for sizing components.
///
/// Constraints define how space should be distributed among layout elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Constraint {
    /// Fixed length in cells
    Length(u16),
    /// Minimum length
    Min(u16),
    /// Maximum length
    Max(u16),
    /// Proportional fill with weight
    Fill(u16),
    /// Ratio of total space (numerator, denominator)
    Ratio(u16, u16),
    /// Percentage of total space (0-100)
    Percentage(u16),
}

impl Constraint {
    /// Apply this constraint to the given available space.
    #[must_use]
    pub fn apply(self, available: u16) -> u16 {
        match self {
            Self::Length(len) => len.min(available),
            Self::Min(min) => min.min(available),
            Self::Max(max) => available.min(max),
            Self::Fill(_) => available,
            Self::Ratio(num, den) => {
                if den == 0 {
                    0
                } else {
                    ((available as u32 * num as u32) / den as u32) as u16
                }
            }
            Self::Percentage(pct) => ((available as u32 * pct as u32) / 100) as u16,
        }
    }
}

/// Flex layout modes for distributing space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Flex {
    /// Place items at the start
    Start,
    /// Center items
    Center,
    /// Place items at the end
    End,
    /// Distribute space evenly between items
    SpaceBetween,
    /// Distribute space evenly around items
    SpaceAround,
}

impl Default for Flex {
    fn default() -> Self {
        Self::Start
    }
}

/// Spacing between layout elements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Spacing {
    /// Positive spacing (gap)
    Gap(u16),
    /// Negative spacing (overlap)
    Overlap(u16),
}

impl Default for Spacing {
    fn default() -> Self {
        Self::Gap(0)
    }
}

/// Direction for layout flow.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum Direction {
    /// Horizontal layout (left to right)
    Horizontal,
    /// Vertical layout (top to bottom)
    Vertical,
}

/// A layout engine for dividing rectangular areas using constraints.
///
/// # Example
///
/// ```
/// use tuxtui_core::layout::{Layout, Constraint, Direction};
/// use tuxtui_core::geometry::Rect;
///
/// let area = Rect::new(0, 0, 100, 50);
/// let chunks = Layout::default()
///     .direction(Direction::Vertical)
///     .constraints([
///         Constraint::Length(10),
///         Constraint::Fill(1),
///         Constraint::Length(5),
///     ])
///     .split(area);
/// ```
#[derive(Debug, Clone)]
pub struct Layout {
    direction: Direction,
    constraints: Vec<Constraint>,
    flex: Flex,
    spacing: Spacing,
    #[cfg(feature = "layout-cache")]
    cache: Option<LruCache<LayoutCacheKey, Vec<Rect>>>,
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            direction: Direction::Vertical,
            constraints: Vec::new(),
            flex: Flex::default(),
            spacing: Spacing::default(),
            #[cfg(feature = "layout-cache")]
            cache: None,
        }
    }
}

impl Layout {
    /// Create a new layout.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the layout direction.
    #[must_use]
    pub fn direction(mut self, direction: Direction) -> Self {
        self.direction = direction;
        self
    }

    /// Set the constraints.
    #[must_use]
    pub fn constraints<I>(mut self, constraints: I) -> Self
    where
        I: IntoIterator,
        I::Item: Into<Constraint>,
    {
        self.constraints = constraints.into_iter().map(Into::into).collect();
        self
    }

    /// Set the flex mode.
    #[must_use]
    pub fn flex(mut self, flex: Flex) -> Self {
        self.flex = flex;
        self
    }

    /// Set the spacing.
    #[must_use]
    pub fn spacing(mut self, spacing: Spacing) -> Self {
        self.spacing = spacing;
        self
    }

    /// Enable caching with the given capacity.
    #[cfg(feature = "layout-cache")]
    #[must_use]
    pub fn cache(mut self, capacity: NonZeroUsize) -> Self {
        self.cache = Some(LruCache::new(capacity));
        self
    }

    /// Split the given area according to the constraints.
    ///
    /// Returns a vector of rectangles, one for each constraint.
    #[must_use]
    pub fn split(&mut self, area: Rect) -> Vec<Rect> {
        #[cfg(feature = "layout-cache")]
        {
            let key = LayoutCacheKey {
                area,
                direction: self.direction,
                constraints: self.constraints.clone(),
                flex: self.flex,
                spacing: self.spacing,
            };

            if let Some(cache) = &mut self.cache {
                if let Some(rects) = cache.get(&key) {
                    return rects.clone();
                }
            }

            let rects = self.calculate_layout(area);

            if let Some(cache) = &mut self.cache {
                cache.put(key, rects.clone());
            }

            rects
        }

        #[cfg(not(feature = "layout-cache"))]
        self.calculate_layout(area)
    }

    fn calculate_layout(&self, area: Rect) -> Vec<Rect> {
        if self.constraints.is_empty() {
            return Vec::new();
        }

        let (total_space, cross_size) = match self.direction {
            Direction::Horizontal => (area.width, area.height),
            Direction::Vertical => (area.height, area.width),
        };

        // Calculate sizes based on constraints
        let mut sizes = Vec::with_capacity(self.constraints.len());
        let mut fixed_space = 0u16;
        let mut fill_weights = 0u32;

        // First pass: calculate fixed sizes and count fill weights
        for constraint in &self.constraints {
            match constraint {
                Constraint::Length(len) => {
                    sizes.push(*len);
                    fixed_space = fixed_space.saturating_add(*len);
                }
                Constraint::Min(min) => {
                    sizes.push(*min);
                    fixed_space = fixed_space.saturating_add(*min);
                }
                Constraint::Max(max) => {
                    sizes.push(total_space.min(*max));
                    fixed_space = fixed_space.saturating_add(total_space.min(*max));
                }
                Constraint::Ratio(num, den) => {
                    let size = if *den == 0 {
                        0
                    } else {
                        ((total_space as u32 * *num as u32) / *den as u32) as u16
                    };
                    sizes.push(size);
                    fixed_space = fixed_space.saturating_add(size);
                }
                Constraint::Percentage(pct) => {
                    let size = ((total_space as u32 * *pct as u32) / 100) as u16;
                    sizes.push(size);
                    fixed_space = fixed_space.saturating_add(size);
                }
                Constraint::Fill(weight) => {
                    sizes.push(0); // Placeholder
                    fill_weights += *weight as u32;
                }
            }
        }

        // Calculate spacing adjustments
        let spacing_total = if self.constraints.len() > 1 {
            match self.spacing {
                Spacing::Gap(gap) => gap.saturating_mul((self.constraints.len() - 1) as u16),
                Spacing::Overlap(overlap) => {
                    0u16.saturating_sub(overlap.saturating_mul((self.constraints.len() - 1) as u16))
                }
            }
        } else {
            0
        };

        let available_for_fill = total_space
            .saturating_sub(fixed_space)
            .saturating_sub(spacing_total);

        // Second pass: distribute remaining space to Fill constraints
        if fill_weights > 0 {
            for (i, constraint) in self.constraints.iter().enumerate() {
                if let Constraint::Fill(weight) = constraint {
                    let fill_size =
                        ((available_for_fill as u32 * *weight as u32) / fill_weights) as u16;
                    sizes[i] = fill_size;
                }
            }
        }

        // Build rectangles with flex
        let used_space: u16 = sizes.iter().sum();
        let flex_space = total_space.saturating_sub(used_space);

        let (mut x, mut y) = match self.flex {
            Flex::Start => (area.x, area.y),
            Flex::Center => match self.direction {
                Direction::Horizontal => (area.x + flex_space / 2, area.y),
                Direction::Vertical => (area.x, area.y + flex_space / 2),
            },
            Flex::End => match self.direction {
                Direction::Horizontal => (area.x + flex_space, area.y),
                Direction::Vertical => (area.x, area.y + flex_space),
            },
            Flex::SpaceBetween | Flex::SpaceAround => (area.x, area.y),
        };

        let mut rects = Vec::with_capacity(self.constraints.len());
        let gap = match self.spacing {
            Spacing::Gap(g) => g,
            Spacing::Overlap(o) => 0u16.saturating_sub(o),
        };

        for size in sizes {
            let rect = match self.direction {
                Direction::Horizontal => Rect::new(x, y, size, cross_size),
                Direction::Vertical => Rect::new(x, y, cross_size, size),
            };
            rects.push(rect);

            match self.direction {
                Direction::Horizontal => x = x.saturating_add(size).saturating_add(gap),
                Direction::Vertical => y = y.saturating_add(size).saturating_add(gap),
            }
        }

        rects
    }
}

#[cfg(feature = "layout-cache")]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct LayoutCacheKey {
    area: Rect,
    direction: Direction,
    constraints: Vec<Constraint>,
    flex: Flex,
    spacing: Spacing,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constraint_apply() {
        assert_eq!(Constraint::Length(10).apply(100), 10);
        assert_eq!(Constraint::Min(50).apply(100), 50);
        assert_eq!(Constraint::Max(50).apply(100), 50);
        assert_eq!(Constraint::Ratio(1, 2).apply(100), 50);
        assert_eq!(Constraint::Percentage(50).apply(100), 50);
    }

    #[test]
    fn test_layout_split() {
        let area = Rect::new(0, 0, 100, 100);
        let mut layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(20),
            ]);

        let rects = layout.split(area);
        assert_eq!(rects.len(), 3);
        assert_eq!(rects[0].height, 10);
        assert_eq!(rects[2].height, 20);
    }

    #[test]
    fn test_layout_horizontal() {
        let area = Rect::new(0, 0, 100, 50);
        let mut layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]);

        let rects = layout.split(area);
        assert_eq!(rects.len(), 2);
        assert_eq!(rects[0].width, 50);
        assert_eq!(rects[1].width, 50);
    }
}
