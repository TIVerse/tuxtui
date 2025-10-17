# tuxtui

[![Crates.io](https://img.shields.io/crates/v/tuxtui.svg)](https://crates.io/crates/tuxtui)
[![Documentation](https://docs.rs/tuxtui/badge.svg)](https://docs.rs/tuxtui)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Build Status](https://github.com/TIVerse/tuxtui/workflows/CI/badge.svg)](https://github.com/TIVerse/tuxtui/actions)
[![MSRV](https://img.shields.io/badge/MSRV-1.85.0-blue)](https://www.rust-lang.org)

A powerful and enhanced Terminal UI library for Rust, inspired by [ratatui](https://ratatui.rs).

**tuxtui** provides a rich set of widgets, a flexible layout system, and multiple backend support for building beautiful terminal user interfaces.

## ✨ Features

- 🎨 **Rich Widget Set**: Block, Paragraph, List, Table, Tabs, Gauge, Charts, Canvas, and more
- 📐 **Flexible Layout**: Constraint-based layout engine with caching and border overlap support
- 🎭 **Theme System**: Built-in theme support with serialization (TOML/YAML/JSON)
- 🔌 **Multi-Backend**: crossterm (default), termion, termwiz
- 🎯 **Type-Safe**: Strong type safety with generic widgets and builders
- 🧪 **Test-Friendly**: TestBackend for snapshot testing
- 🚀 **Performance**: Efficient diffing algorithm, minimal allocations
- 📦 **Modular**: Separate crates for core, widgets, and backends

## 🚀 Quick Start

Add tuxtui to your `Cargo.toml`:

```toml
[dependencies]
tuxtui = "0.1"
crossterm = "0.29"
```

### Hello World Example

```rust
use crossterm::event::{self, Event};
use tuxtui::{DefaultTerminal, Frame};

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    let result = run(&mut terminal);
    tuxtui::restore()?;
    result
}

fn run(terminal: &mut DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame: &mut Frame| {
            frame.render_widget("hello tuxtui!", frame.area());
        })?;
        if matches!(event::read()?, Event::Key(_)) {
            break Ok(());
        }
    }
}
```

Run with:

```bash
cargo run --example hello_world
```

## 📦 Workspace Structure

The project is organized as a multi-crate workspace:

- **`tuxtui`**: Main end-user facade crate
- **`tuxtui-core`**: Core types, traits, and primitives
- **`tuxtui-widgets`**: Official widget implementations
- **`tuxtui-crossterm`**: Crossterm backend
- **`tuxtui-termion`**: Termion backend (Unix)
- **`tuxtui-termwiz`**: Termwiz backend
- **`tuxtui-macros`**: Procedural macros

## 🎨 Widgets

tuxtui includes a comprehensive widget library:

| Widget | Description |
|--------|-------------|
| **Block** | Bordered containers with titles |
| **Paragraph** | Rich text with wrapping and alignment |
| **List** | Selectable item lists with markers |
| **Table** | Tabular data with row/column selection |
| **Tabs** | Tab navigation |
| **Gauge** | Progress indicators |
| **BarChart** | Bar chart visualization |
| **Sparkline** | Compact line charts |
| **Chart** | Full-featured charts with axes |
| **Scrollbar** | Scrollbars for scrollable content |
| **Canvas** | Low-level drawing with Braille characters |

## 🎯 Feature Flags

### Backend Selection (choose one)

- `crossterm` (default): Cross-platform backend
- `termion`: Unix backend
- `termwiz`: Alternative backend

### Optional Features

- `serde`: Serialization support
- `palette`: Advanced color manipulation (HSL/HSLuv)
- `layout-cache`: LRU caching for layouts
- `underline-color`: Colored underlines
- `all-widgets`: Enable all widgets
- `widget-calendar`: Calendar widget (requires `time`)
- `macros`: Convenience macros

## 🔄 Migrating from ratatui

tuxtui is designed to be largely compatible with ratatui:

```toml
# Replace:
# ratatui = "0.28"

# With:
tuxtui = "0.1"
```

Most code works with minimal changes—just replace `ratatui::` with `tuxtui::`.

## 🏗️ Architecture

tuxtui uses a layered architecture:

1. **Core Layer** (`tuxtui-core`): Fundamental types (Buffer, Style, Layout, Backend trait)
2. **Widget Layer** (`tuxtui-widgets`): Widget implementations
3. **Backend Layer**: Platform-specific terminal manipulation
4. **Facade Layer** (`tuxtui`): High-level API and convenience functions

This design enables:
- Widget libraries to depend only on `tuxtui-core`
- Applications to choose their backend
- Reduced compile times through modularity

## 📚 Examples

Check out the `examples/` directory for more examples:

- `hello_world`: Basic example
- `widgets_demo`: Showcase of various widgets
- `todo_list`: Interactive todo list app
- `layout_demo`: Layout engine demonstration

Run any example with:

```bash
cargo run --example <example_name>
```

## 🧪 Testing

tuxtui includes a `TestBackend` for unit testing:

```rust
use tuxtui_core::backend::TestBackend;
use tuxtui_core::terminal::Terminal;

let backend = TestBackend::new(80, 24);
let mut terminal = Terminal::new(backend).unwrap();

terminal.draw(|frame| {
    // Render widgets
}).unwrap();

// Assert buffer contents
```

## 📖 Documentation

- [API Documentation](https://docs.rs/tuxtui)
- [User Guide](https://github.com/TIVerse/tuxtui/wiki) (coming soon)
- [Examples](examples/)

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 👤 Author

**Eshan Roy** <eshanized@proton.me>

**Tonmoy Infrastructure & Vision**

## 🙏 Acknowledgments

Inspired by [ratatui](https://ratatui.rs) and the Rust TUI ecosystem.

## 🔗 Links

- [Repository](https://github.com/TIVerse/tuxtui)
- [Crates.io](https://crates.io/crates/tuxtui)
- [Documentation](https://docs.rs/tuxtui)
- [Issues](https://github.com/TIVerse/tuxtui/issues)

---

**Built with ❤️ by the Tonmoy Infrastructure & Vision team**
