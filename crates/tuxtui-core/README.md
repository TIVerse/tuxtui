# tuxtui-core

[![Crates.io](https://img.shields.io/crates/v/tuxtui-core.svg)](https://crates.io/crates/tuxtui-core)
[![Documentation](https://docs.rs/tuxtui-core/badge.svg)](https://docs.rs/tuxtui-core)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../../LICENSE)

Core types and traits for the tuxtui Terminal UI library.

## Overview

`tuxtui-core` provides the foundational building blocks for terminal user interfaces:

- **Style**: Colors, modifiers, and styling primitives
- **Text**: Rich text with spans, lines, and paragraphs  
- **Buffer**: Double-buffered terminal cell storage with efficient diffing
- **Layout**: Flexible constraint-based layout engine with caching
- **Backend**: Platform-agnostic terminal abstraction trait
- **Theme**: Themable UI components with serialization support

## Features

- `std` (default): Enable standard library support
- `layout-cache`: Enable LRU caching for layout calculations
- `serde`: Enable serialization/deserialization
- `palette`: Enable advanced color manipulation with HSL/HSLuv
- `portable-atomic`: Use portable atomics for no-std compatibility
- `anstyle`: Enable anstyle conversions
- `underline-color`: Enable colored underlines
- `scrolling-regions`: Enable terminal scrolling region support

## Example

```rust
use tuxtui_core::{buffer::Buffer, geometry::Rect, style::{Color, Style}};

let mut buffer = Buffer::empty(Rect::new(0, 0, 10, 5));
let style = Style::default().fg(Color::Blue);
buffer.set_string(0, 0, "Hello", style);
```

## Usage in Libraries

Widget libraries can depend on `tuxtui-core` without pulling in heavy backend dependencies:

```toml
[dependencies]
tuxtui-core = { version = "0.1", default-features = false }
```

## License

MIT - See [LICENSE](../../../LICENSE) for details.

Copyright (c) 2024 Eshan Roy <eshanized@proton.me>

Tonmoy Infrastructure & Vision
