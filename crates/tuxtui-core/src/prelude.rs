//! Convenient re-exports for common types and traits.
//!
//! The prelude module provides a convenient way to import commonly used
//! items from the `tuxtui-core` crate.
//!
//! # Example
//!
//! ```
//! use tuxtui_core::prelude::*;
//!
//! let rect = Rect::new(0, 0, 10, 10);
//! let style = Style::default().fg(Color::Blue);
//! ```

pub use crate::backend::{Backend, TestBackend};
pub use crate::buffer::{Buffer, Cell};
pub use crate::event::{KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
pub use crate::geometry::{Alignment, Margin, Position, Rect};
pub use crate::layout::{Constraint, Direction, Flex, Layout, Spacing};
pub use crate::style::{Color, Modifier, Style, Stylize};
pub use crate::symbols;
pub use crate::terminal::{Frame, Terminal, Widget};
pub use crate::text::{Line, Span, Text};
pub use crate::theme::{PaletteTheme, Theme, WidgetTheme};
