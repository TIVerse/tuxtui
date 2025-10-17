# tuxtui-crossterm

[![Crates.io](https://img.shields.io/crates/v/tuxtui-crossterm.svg)](https://crates.io/crates/tuxtui-crossterm)
[![Documentation](https://docs.rs/tuxtui-crossterm/badge.svg)](https://docs.rs/tuxtui-crossterm)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](../../LICENSE)

Crossterm backend for tuxtui.

## Usage

```rust
use tuxtui_crossterm::CrosstermBackend;
use tuxtui_core::terminal::Terminal;
use std::io::stdout;

let backend = CrosstermBackend::new(stdout());
let mut terminal = Terminal::new(backend)?;
```

## License

MIT - Copyright (c) 2024 Eshan Roy
