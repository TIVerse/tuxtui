//! # tuxtui-widgets
//!
//! Official widget implementations for the tuxtui Terminal UI library.
//!
//! This crate provides a comprehensive set of widgets for building terminal
//! user interfaces, including:
//!
//! - **Block**: Bordered containers with titles
//! - **Paragraph**: Rich text rendering with wrapping
//! - **List**: Selectable item lists with markers
//! - **Table**: Tabular data with row/column/cell selection
//! - **Tabs**: Tab navigation widgets
//! - **Gauge**: Progress indicators (linear and radial)
//! - **BarChart**: Bar chart visualization
//! - **Sparkline**: Compact line charts
//! - **Chart**: Full-featured charts with axes and datasets
//! - **Scrollbar**: Scrollbars for scrollable content
//! - **Canvas**: Low-level drawing canvas
//!
//! ## Features
//!
//! - `all-widgets` (default): Enable all widgets
//! - `widget-calendar`: Enable calendar widget (requires `time` crate)
//! - `serde`: Enable serialization for widget state
//! - `unstable-rendered-line-info`: Enable experimental line info API
//!
//! ## Example
//!
//! ```rust
//! use tuxtui_core::prelude::*;
//! use tuxtui_widgets::block::{Block, BorderType};
//!
//! let block = Block::default()
//!     .title("My Block")
//!     .borders(BorderType::All);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

#[cfg(feature = "block")]
pub mod block;

#[cfg(feature = "paragraph")]
pub mod paragraph;

#[cfg(feature = "list")]
pub mod list;

#[cfg(feature = "table")]
pub mod table;

#[cfg(feature = "tabs")]
pub mod tabs;

#[cfg(feature = "gauge")]
pub mod gauge;

#[cfg(feature = "barchart")]
pub mod barchart;

#[cfg(feature = "sparkline")]
pub mod sparkline;

#[cfg(feature = "chart")]
pub mod chart;

#[cfg(feature = "scrollbar")]
pub mod scrollbar;

#[cfg(feature = "canvas")]
pub mod canvas;

#[cfg(feature = "widget-calendar")]
#[cfg_attr(docsrs, doc(cfg(feature = "widget-calendar")))]
pub mod calendar;

pub mod input;
pub mod popup;
pub mod tree;

pub mod prelude;
