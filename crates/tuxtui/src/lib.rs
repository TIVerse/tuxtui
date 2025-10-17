//! # tuxtui
//!
//! A powerful and enhanced Terminal UI library for Rust.
//!
//! `tuxtui` is a library for building rich terminal user interfaces, inspired by
//! [ratatui](https://ratatui.rs) but with enhanced features and a modular architecture.
//!
//! ## Quick Start
//!
//! ```no_run
//! use tuxtui::prelude::*;
//! use tuxtui::widgets::block::{Block, BorderType};
//! use crossterm::event::{self, Event};
//!
//! fn main() -> std::io::Result<()> {
//!     let mut terminal = tuxtui::init()?;
//!     let result = run(&mut terminal);
//!     tuxtui::restore()?;
//!     result
//! }
//!
//! fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
//!     loop {
//!         terminal.draw(|frame| {
//!             let area = frame.area();
//!             let block = Block::default()
//!                 .title("Hello tuxtui!")
//!                 .borders(BorderType::All);
//!             frame.render_widget(block, area);
//!         })?;
//!
//!         if matches!(event::read()?, Event::Key(_)) {
//!             break Ok(());
//!         }
//!     }
//! }
//! ```
//!
//! ## Features
//!
//! ### Backend Selection
//!
//! - `crossterm` (default): Cross-platform backend using crossterm
//! - `termion`: Unix backend using termion
//! - `termwiz`: Backend using termwiz
//!
//! ### Optional Features
//!
//! - `serde`: Enable serialization/deserialization
//! - `palette`: Advanced color manipulation
//! - `layout-cache`: LRU caching for layout calculations
//! - `underline-color`: Colored underlines
//! - `all-widgets`: Enable all widgets
//! - `widget-calendar`: Calendar widget (requires `time` crate)
//! - `macros`: Convenience macros
//!
//! ## Architecture
//!
//! tuxtui is organized into several crates:
//!
//! - `tuxtui-core`: Core types, traits, and primitives
//! - `tuxtui-widgets`: Official widget implementations
//! - `tuxtui-crossterm`: Crossterm backend
//! - `tuxtui-termion`: Termion backend
//! - `tuxtui-termwiz`: Termwiz backend
//! - `tuxtui-macros`: Procedural macros
//!
//! ## Migrating from ratatui
//!
//! tuxtui is designed to be largely compatible with ratatui:
//!
//! ```toml
//! # Replace this:
//! # ratatui = "0.28"
//!
//! # With this:
//! tuxtui = "0.1"
//! ```
//!
//! Most code should work with minimal changes. Replace `ratatui::` with `tuxtui::`
//! in your imports.

#![cfg_attr(docsrs, feature(doc_cfg))]
#![forbid(unsafe_code)]
#![warn(missing_docs)]

// Re-export core types
pub use tuxtui_core::{
    backend, buffer, geometry, layout, prelude as core_prelude, style, symbols, terminal, text,
    theme, util,
};

// Re-export widgets
pub use tuxtui_widgets as widgets;

// Re-export backend based on features
#[cfg(feature = "crossterm")]
#[cfg_attr(docsrs, doc(cfg(feature = "crossterm")))]
pub use tuxtui_crossterm::CrosstermBackend;

#[cfg(feature = "termion")]
#[cfg_attr(docsrs, doc(cfg(feature = "termion")))]
pub use tuxtui_termion::TermionBackend;

#[cfg(feature = "termwiz")]
#[cfg_attr(docsrs, doc(cfg(feature = "termwiz")))]
pub use tuxtui_termwiz::TermwizBackend;

// Re-export macros
#[cfg(feature = "macros")]
#[cfg_attr(docsrs, doc(cfg(feature = "macros")))]
pub use tuxtui_macros::*;

/// Convenient prelude for common imports.
///
/// # Example
///
/// ```
/// use tuxtui::prelude::*;
/// ```
pub mod prelude {
    pub use crate::backend::{Backend, TestBackend};
    pub use crate::buffer::{Buffer, Cell};
    pub use crate::geometry::{Alignment, Margin, Position, Rect};
    pub use crate::layout::{Constraint, Direction, Flex, Layout, Spacing};
    pub use crate::style::{Color, Modifier, Style, Stylize};
    pub use crate::terminal::{Frame, Terminal, TerminalOptions, Widget};
    pub use crate::text::{Line, Span, Text};
    pub use crate::theme::{PaletteTheme, Theme, WidgetTheme};

    #[cfg(feature = "crossterm")]
    pub use crate::CrosstermBackend;

    #[cfg(feature = "termion")]
    pub use crate::TermionBackend;

    #[cfg(feature = "termwiz")]
    pub use crate::TermwizBackend;
}

// Type aliases for convenience
#[cfg(feature = "crossterm")]
/// Default terminal type using crossterm backend.
pub type DefaultTerminal = terminal::Terminal<CrosstermBackend<std::io::Stdout>>;

#[cfg(all(feature = "termion", not(feature = "crossterm")))]
/// Default terminal type using termion backend.
pub type DefaultTerminal = terminal::Terminal<TermionBackend<std::io::Stdout>>;

#[cfg(all(
    feature = "termwiz",
    not(feature = "crossterm"),
    not(feature = "termion")
))]
/// Default terminal type using termwiz backend.
pub type DefaultTerminal = terminal::Terminal<TermwizBackend>;

/// Initialize a terminal with default settings.
///
/// This is a convenience function that:
/// - Creates a backend (crossterm by default)
/// - Enables raw mode
/// - Enters alternate screen
/// - Hides the cursor
/// - Clears the terminal
///
/// # Panics
///
/// A panic hook is installed to restore the terminal on panic.
///
/// # Example
///
/// ```no_run
/// use tuxtui;
///
/// fn main() -> std::io::Result<()> {
///     let mut terminal = tuxtui::init()?;
///     // Use terminal...
///     tuxtui::restore()?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "crossterm")]
pub fn init() -> std::io::Result<DefaultTerminal> {
    use std::io::stdout;

    // Install panic hook to restore terminal
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = restore();
        hook(info);
    }));

    let backend = CrosstermBackend::new(stdout());
    terminal::Terminal::new(backend)
}

/// Restore the terminal to its original state.
///
/// This function should be called before exiting to:
/// - Disable raw mode
/// - Leave alternate screen
/// - Show the cursor
///
/// # Example
///
/// ```no_run
/// use tuxtui;
///
/// fn main() -> std::io::Result<()> {
///     let mut terminal = tuxtui::init()?;
///     // Use terminal...
///     tuxtui::restore()?;
///     Ok(())
/// }
/// ```
#[cfg(feature = "crossterm")]
pub fn restore() -> std::io::Result<()> {
    use crossterm::{
        execute,
        terminal::{LeaveAlternateScreen, disable_raw_mode},
    };
    use std::io::stdout;

    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    Ok(())
}

/// A convenience type for the main frame rendering callback.
pub type FrameDrawFn<'a> = Box<dyn FnMut(&mut terminal::Frame<'_>) + 'a>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::*;

    #[test]
    fn test_terminal_with_test_backend() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                assert_eq!(area.width, 80);
                assert_eq!(area.height, 24);
            })
            .unwrap();
    }

    #[test]
    fn test_prelude_imports() {
        let _rect = Rect::new(0, 0, 10, 10);
        let _style = Style::default().fg(Color::Blue);
        let _text = Text::from("Hello");
    }
}
