//! Event handling types and utilities.

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Mouse button types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseButton {
    /// Left mouse button
    Left,
    /// Right mouse button
    Right,
    /// Middle mouse button
    Middle,
}

/// Mouse event types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum MouseEventKind {
    /// Mouse button pressed
    Down(MouseButton),
    /// Mouse button released
    Up(MouseButton),
    /// Mouse dragged with button held
    Drag(MouseButton),
    /// Mouse moved without button
    Moved,
    /// Mouse wheel scrolled
    ScrollDown,
    /// Mouse wheel scrolled up
    ScrollUp,
    /// Mouse wheel scrolled left
    ScrollLeft,
    /// Mouse wheel scrolled right
    ScrollRight,
}

/// A mouse event with position.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MouseEvent {
    /// The kind of mouse event
    pub kind: MouseEventKind,
    /// Column position (0-indexed)
    pub column: u16,
    /// Row position (0-indexed)
    pub row: u16,
}

impl MouseEvent {
    /// Create a new mouse event.
    #[must_use]
    pub const fn new(kind: MouseEventKind, column: u16, row: u16) -> Self {
        Self { kind, column, row }
    }

    /// Check if this is a click event at the given position.
    #[must_use]
    pub const fn is_click_at(&self, column: u16, row: u16) -> bool {
        matches!(self.kind, MouseEventKind::Down(_)) && self.column == column && self.row == row
    }

    /// Check if this is a click event within the given area.
    #[must_use]
    pub fn is_click_in(&self, area: crate::geometry::Rect) -> bool {
        matches!(self.kind, MouseEventKind::Down(_))
            && self.column >= area.left()
            && self.column < area.right()
            && self.row >= area.top()
            && self.row < area.bottom()
    }
}

/// Keyboard modifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KeyModifiers {
    /// Shift key
    pub shift: bool,
    /// Control key
    pub ctrl: bool,
    /// Alt key
    pub alt: bool,
    /// Meta/Super/Cmd key
    pub meta: bool,
}

impl KeyModifiers {
    /// No modifiers pressed.
    pub const NONE: Self = Self {
        shift: false,
        ctrl: false,
        alt: false,
        meta: false,
    };

    /// Only shift pressed.
    pub const SHIFT: Self = Self {
        shift: true,
        ctrl: false,
        alt: false,
        meta: false,
    };

    /// Only control pressed.
    pub const CTRL: Self = Self {
        shift: false,
        ctrl: true,
        alt: false,
        meta: false,
    };

    /// Only alt pressed.
    pub const ALT: Self = Self {
        shift: false,
        ctrl: false,
        alt: true,
        meta: false,
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::Rect;

    #[test]
    fn test_mouse_event_click_detection() {
        let event = MouseEvent::new(MouseEventKind::Down(MouseButton::Left), 5, 10);
        assert!(event.is_click_at(5, 10));
        assert!(!event.is_click_at(6, 10));
    }

    #[test]
    fn test_mouse_event_area_detection() {
        let event = MouseEvent::new(MouseEventKind::Down(MouseButton::Left), 5, 10);
        let area = Rect::new(0, 0, 10, 20);
        assert!(event.is_click_in(area));

        let outside_area = Rect::new(20, 20, 10, 10);
        assert!(!event.is_click_in(outside_area));
    }
}
