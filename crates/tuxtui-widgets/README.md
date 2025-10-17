# tuxtui-widgets

[![Crates.io](https://img.shields.io/crates/v/tuxtui-widgets.svg)](https://crates.io/crates/tuxtui-widgets)
[![Documentation](https://docs.rs/tuxtui-widgets/badge.svg)](https://docs.rs/tuxtui-widgets)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

Official widget implementations for tuxtui.

## Widgets

- **Block**: Bordered containers with titles
- **Paragraph**: Rich text with wrapping
- **List**: Selectable lists
- **Table**: Tabular data
- **Tabs**: Tab navigation
- **Gauge**: Progress bars
- **BarChart**: Bar charts
- **Sparkline**: Compact charts
- **Chart**: Full-featured charts
- **Scrollbar**: Scrollbars
- **Canvas**: Drawing canvas

## Usage

```rust
use tuxtui_widgets::block::{Block, BorderType};

let block = Block::default()
    .title("My Block")
    .borders(BorderType::All);
```

## License

MIT - Copyright (c) 2024 Eshan Roy
