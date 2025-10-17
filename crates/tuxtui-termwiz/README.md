# tuxtui-termwiz

Termwiz backend for the tuxtui Terminal UI library.

## Overview

This crate provides a [termwiz](https://github.com/wez/wezterm/tree/main/termwiz) backend implementation for tuxtui, enabling TUI applications to run with the termwiz terminal library.

## Features

- Full termwiz backend integration
- Advanced terminal capabilities
- Cross-platform support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tuxtui = { version = "0.1", features = ["termwiz"] }
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
- ✅ Windows
- ✅ BSD

## Documentation

For detailed documentation, see [docs.rs/tuxtui-termwiz](https://docs.rs/tuxtui-termwiz).

## License

MIT - See [LICENSE](../../LICENSE) for details.

## Repository

https://github.com/TIVerse/tuxtui
