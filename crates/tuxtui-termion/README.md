# tuxtui-termion

Termion backend for the tuxtui Terminal UI library.

## Overview

This crate provides a [termion](https://github.com/redox-os/termion) backend implementation for tuxtui, enabling TUI applications to run on Unix-like systems with the termion terminal library.

## Features

- Full termion backend integration
- Unix-specific optimizations
- Lightweight terminal handling

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tuxtui = { version = "0.1", features = ["termion"] }
```

## Example

```rust
use tuxtui::prelude::*;

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    
    terminal.draw(|frame| {
        // Your UI code here
    })?;
    
    tuxtui::restore()
}
```

## Platform Support

- ✅ Linux
- ✅ macOS
- ✅ BSD
- ❌ Windows (use crossterm backend instead)

## Documentation

For detailed documentation, see [docs.rs/tuxtui-termion](https://docs.rs/tuxtui-termion).

## License

MIT - See [LICENSE](../../LICENSE) for details.

## Repository

https://github.com/TIVerse/tuxtui
