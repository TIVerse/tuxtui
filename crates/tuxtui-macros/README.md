# tuxtui-macros

Procedural macros for the tuxtui Terminal UI library.

## Overview

This crate provides procedural macros to enhance the developer experience when using tuxtui. It includes derive macros and function-like macros for common patterns in TUI development.

## Features

- Derive macros for widget state management
- Helper macros for reducing boilerplate
- Code generation for common TUI patterns

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
tuxtui-macros = "0.1"
```

Or use the main tuxtui crate with the `macros` feature:

```toml
[dependencies]
tuxtui = { version = "0.1", features = ["macros"] }
```

## Documentation

For detailed documentation, see [docs.rs/tuxtui-macros](https://docs.rs/tuxtui-macros).

## License

MIT - See [LICENSE](../../LICENSE) for details.

## Repository

https://github.com/TIVerse/tuxtui
