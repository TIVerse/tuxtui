//! Convenient re-exports for widgets.
//!
//! # Example
//!
//! ```
//! use tuxtui_widgets::prelude::*;
//!
//! let block = Block::default().borders(BorderType::All);
//! let paragraph = Paragraph::new("Hello!");
//! ```

#[cfg(feature = "block")]
pub use crate::block::{Block, BorderType, Borders, Title, TitlePosition};

#[cfg(feature = "paragraph")]
pub use crate::paragraph::{Paragraph, Scroll, Wrap};

#[cfg(feature = "list")]
pub use crate::list::{List, ListItem, ListMarker, ListState};

#[cfg(feature = "table")]
pub use crate::table::{Row, Table, TableState};

#[cfg(feature = "tabs")]
pub use crate::tabs::Tabs;

#[cfg(feature = "gauge")]
pub use crate::gauge::Gauge;

#[cfg(feature = "barchart")]
pub use crate::barchart::{Bar, BarChart};

#[cfg(feature = "sparkline")]
pub use crate::sparkline::Sparkline;

#[cfg(feature = "chart")]
pub use crate::chart::{Chart, DataPoint, Dataset};

#[cfg(feature = "scrollbar")]
pub use crate::scrollbar::{Scrollbar, ScrollbarOrientation};

#[cfg(feature = "canvas")]
pub use crate::canvas::{Canvas, CanvasContext, Shape};

pub use crate::tree::{Tree, TreeNode, TreeState, TreeSymbols};
pub use crate::input::{TextInput, InputState};
pub use crate::popup::{Popup, Modal};
