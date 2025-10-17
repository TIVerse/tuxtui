# tuxtui

[![Crates.io](https://img.shields.io/crates/v/tuxtui.svg)](https://crates.io/crates/tuxtui)
[![Documentation](https://docs.rs/tuxtui/badge.svg)](https://docs.rs/tuxtui)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A powerful and enhanced Terminal UI library for Rust, inspired by ratatui.

## Features

- 🎨 **16+ Built-in Widgets** - Tree, TextInput, Modal/Popup, List, Table, Charts, and more
- 🎭 **Multiple Backends** - crossterm (default), termion, termwiz
- 🎨 **Theme System** - Serializable themes with TOML/JSON/YAML support
- 🖱️ **Mouse Support** - Full mouse event handling
- 📜 **Advanced Scrolling** - ViewportState for complex scrolling scenarios
- 🔒 **Password Fields** - Built-in masked text input
- ☑️ **Multi-Select Lists** - Native multi-selection support
- 🎨 **Color Parsing** - Parse colors from strings (hex, RGB, named)
- 📐 **Layout Helpers** - Ergonomic `Layout::vertical()` and `Layout::horizontal()`
- 🔧 **Modern Rust** - Edition 2024, MSRV 1.85.0, 100% safe code

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
tuxtui = "0.1"
crossterm = "0.29"
```

### Hello World

```rust
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use crossterm::event::{self, Event, KeyCode};

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    
    loop {
        terminal.draw(|frame| {
            let block = Block::default()
                .title("Hello tuxtui!")
                .borders(BorderType::All);
            frame.render_widget(block, frame.area());
        })?;
        
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
        }
    }
    
    tuxtui::restore()
}
```

## Examples

See the [examples](https://github.com/TIVerse/tuxtui/tree/master/examples) directory for more:

- **hello_world** - Basic introduction
- **widgets_demo** - Widget showcase
- **file_explorer** - Tree widget with navigation
- **form_input** - Multi-field forms with validation
- **dashboard** - Real-time system monitoring
- **todo_list** - Full task management app
- **mouse_events** - Interactive mouse handling
- **modals** - Dialog and popup demonstrations

## Widgets

| Widget | Description |
|--------|-------------|
| Block | Bordered containers with titles |
| Paragraph | Rich text with wrapping |
| List | Selectable lists with multi-select |
| Table | Tabular data with selection |
| Tree | Hierarchical navigation |
| TextInput | Text input with cursor (supports passwords) |
| Modal/Popup | Dialogs and popups |
| Tabs | Tab navigation |
| Gauge | Progress indicators |
| BarChart | Bar chart visualization |
| Sparkline | Compact line charts |
| Chart | Full-featured charts |
| Scrollbar | Scrollbars for content |
| Canvas | Low-level drawing |

## Backend Selection

tuxtui supports multiple terminal backends:

```toml
# Default: crossterm (cross-platform)
tuxtui = { version = "0.1", features = ["crossterm"] }

# Termion (Unix-only, lightweight)
tuxtui = { version = "0.1", features = ["termion"] }

# Termwiz (cross-platform, advanced)
tuxtui = { version = "0.1", features = ["termwiz"] }
```

## Optional Features

```toml
[dependencies]
tuxtui = { version = "0.1", features = [
    "serde",              # Serialization support
    "macros",             # Procedural macros
    "all-widgets",        # All widgets (default)
    "widget-calendar",    # Calendar widget
    "underline-color",    # Colored underlines
] }
```

## Advantages Over Ratatui

- ✅ **Built-in advanced widgets** (Tree, TextInput, Modal)
- ✅ **Theme system** with serialization
- ✅ **ViewportState** for easier scrolling
- ✅ **Multi-select lists** out of the box
- ✅ **Layout helpers** for cleaner code
- ✅ **Color parsing** from strings
- ✅ **Password input** support

## Documentation

- **API Documentation:** [docs.rs/tuxtui](https://docs.rs/tuxtui)
- **Repository:** [github.com/TIVerse/tuxtui](https://github.com/TIVerse/tuxtui)
- **Examples:** [examples/](https://github.com/TIVerse/tuxtui/tree/master/examples)

## Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](https://github.com/TIVerse/tuxtui/blob/master/CONTRIBUTING.md).

## License

MIT License - see [LICENSE](https://github.com/TIVerse/tuxtui/blob/master/LICENSE) for details.

## Acknowledgments

Inspired by the excellent [ratatui](https://github.com/ratatui-org/ratatui) project.

---

**Built with ❤️ by [Tonmoy Infrastructure & Vision](https://github.com/TIVerse)**
