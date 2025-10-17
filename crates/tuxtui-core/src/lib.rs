//! # tuxtui-core
//!
//! Core types and traits for the tuxtui Terminal UI library.
//!
//! This crate provides the foundational building blocks for terminal user interfaces:
//! - **Style**: Colors, modifiers, and styling primitives
//! - **Text**: Rich text with spans, lines, and paragraphs
//! - **Buffer**: Double-buffered terminal cell storage with efficient diffing
//! - **Layout**: Flexible constraint-based layout engine with caching
//! - **Backend**: Platform-agnostic terminal abstraction trait
//! - **Theme**: Themable UI components with serialization support
//!
//! ## Features
//!
//! - `std` (default): Enable standard library support
//! - `layout-cache`: Enable LRU caching for layout calculations
//! - `serde`: Enable serialization/deserialization
//! - `palette`: Enable advanced color manipulation with HSL/HSLuv
//! - `portable-atomic`: Use portable atomics for no-std compatibility
//! - `anstyle`: Enable anstyle conversions
//! - `underline-color`: Enable colored underlines
//! - `scrolling-regions`: Enable terminal scrolling region support
//!
//! ## Example
//!
//! ```rust
//! use tuxtui_core::{buffer::Buffer, geometry::Rect, style::{Color, Style}};
//!
//! let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
//! let style = Style::default().fg(Color::Blue);
//! buffer.set_string(0, 0, "Hello", style);
//! ```

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

extern crate alloc;

pub mod backend;
pub mod buffer;
pub mod event;
pub mod geometry;
pub mod layout;
pub mod prelude;
pub mod style;
pub mod symbols;
pub mod terminal;
pub mod text;
pub mod theme;
pub mod util;
pub mod viewport;

#[cfg(test)]
mod tests;
