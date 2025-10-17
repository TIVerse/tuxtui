You are an expert Rust library generator. Create a complete, publish-ready, multi-crate Cargo workspace for a high-quality Terminal UI (TUI) library named `tuxtui`, inspired by and compatible with the concepts of `ratatui`, but more enhanced and powerful.

The result must compile and test successfully across Linux, macOS, and Windows. It must be ready for immediate publication to crates.io and should include comprehensive documentation, examples, CI, and release automation.

Follow all instructions precisely.

## Project Metadata

- Name: tuxtui
- Organization: Tonmoy Infrastructure & Vision
- Author: Eshan Roy <eshanized@proton.me>
- License: MIT
- Repository: https://github.com/TIVerse/tuxtui
- Homepage: https://github.com/TIVerse/tuxtui
- Documentation: https://docs.rs/tuxtui (after publish)
- Keywords: tui, terminal, dashboard, cli, ui
- Categories: command-line-interface
- Edition: 2024
- MSRV (Minimum Supported Rust Version): 1.85.0
- Crate-level badges (README): crates.io version, docs.rs, CI, codecov, license, MSRV

## High-level Goals

- Provide immediate-mode rendering, diff-based terminal updates, and robust backends.
- Offer a modular architecture with strong compile-time boundaries to reduce compile times and enable downstream libraries to depend only on what they need.
- Include a curated widget set with rich styling, layout improvements, and a theme system.
- Provide a high-level application harness (optional for app devs) with idiomatic patterns for event loops and async usage.
- Deliver robust testing utilities (including a test backend and snapshot testing) and performance benchmarks.
- Ship extensive examples (apps and concepts), first-class docs, and a clean API with careful feature gating.
- Emphasize performance, ergonomics, and extensibility (plugin-friendly design).

## Workspace Layout

Create a Cargo workspace at the repo root with this structure:

```
tuxtui/
├─ Cargo.toml                # workspace, shared metadata, lints, MSRV
├─ Cargo.lock                # committed for workspace consistency
├─ README.md                 # root readme (aggregates crates, quickstart, links)
├─ LICENSE                   # MIT
├─ CHANGELOG.md              # conventional commits + release automation
├─ CONTRIBUTING.md
├─ CODE_OF_CONDUCT.md
├─ SECURITY.md
├─ RELEASE.md                # release instructions + policies
├─ rust-toolchain.toml       # pins channel if needed (stable)
├─ rustfmt.toml
├─ clippy.toml
├─ deny.toml                 # cargo-deny policies
├─ codecov.yml               # coverage
├─ cliff.toml                # git-cliff config for changelog
├─ release-plz.toml          # release-plz monorepo config
├─ .editorconfig
├─ .github/
│  ├─ ISSUE_TEMPLATE/
│  │  ├─ bug_report.yml
│  │  └─ feature_request.yml
│  ├─ PULL_REQUEST_TEMPLATE.md
│  └─ workflows/
│     ├─ ci.yml              # fmt + clippy + test matrix + coverage + deny + typos
│     ├─ docs.yml            # rustdoc build check
│     └─ release-plz.yml     # automated release PRs
├─ xtask/                    # developer tasks (lint, fmt, docs, release checks)
│  ├─ Cargo.toml
│  └─ src/main.rs
├─ examples/
│  ├─ README.md
│  ├─ apps/
│  │  ├─ hello_world/
│  │  ├─ todo_list/
│  │  ├─ tracing_viewer/
│  │  ├─ popup/
│  │  ├─ input_form/
│  │  ├─ weather/
│  │  └─ async_github/
│  └─ concepts/
│     ├─ layout_constraints/
│     ├─ flex/
│     ├─ chart/
│     ├─ canvas_drawing/
│     ├─ color_explorer/
│     ├─ scrollbar/
│     ├─ table/
│     ├─ user_input/
│     └─ widgetref_container/
└─ crates/
   ├─ tuxtui/                # main end-user crate
   ├─ tuxtui-core/           # core types & traits
   ├─ tuxtui-widgets/        # official widgets
   ├─ tuxtui-crossterm/      # crossterm backend (version-gated)
   ├─ tuxtui-termion/        # termion backend
   ├─ tuxtui-termwiz/        # termwiz backend
   └─ tuxtui-macros/         # macros (e.g., border!)
```

## Crate-by-crate Requirements

### 1) crates/tuxtui (end-user facade)
- Purpose: The primary crate app developers depend on. Re-exports key items from other crates and provides convenience APIs.
- API:
  - `init()` and `restore()` helpers (like ratatui) with safe panic hooks to restore terminal.
  - `Terminal<B: Backend>` orchestrator, `Frame`, diff-based rendering pipeline.
  - Re-export `CrosstermBackend`, `TermionBackend`, `TermwizBackend` if their features are enabled.
  - Idiomatic prelude module for common imports.
- Features:
  - Default: `crossterm`, `underline-color`.
  - Backends (enable only one for apps): `crossterm` (default), `termion`, `termwiz`.
  - Cross-cutting: `serde`, `palette`, `layout-cache`, `portable-atomic`, `macros`, `all-widgets`, `widget-calendar`, `scrolling-regions`.
  - Unstable (gated): `unstable`, `unstable-rendered-line-info`, `unstable-widget-ref`, `unstable-backend-writer`.
- Docs:
  - Crate root docs with Quickstart, Hello World, Feature Flags, Backends, Layout, Text/Styling, Widgets, Testing, and Examples index.
  - Code snippets compile with doctests.

### 2) crates/tuxtui-core
- Purpose: Core data types and traits usable by widget libraries without pulling backends or heavy deps.
- Content:
  - `Rect`, `Buffer`, `Cell`, `Style`, `Color`, `Modifier`, `Stylize` trait.
  - `Text`, `Line`, `Span` and helpers; rich text formatting API.
  - `Layout` with constraints, flex, negative spacing overlap, and caching hooks.
  - `Backend` trait with associated `Error`, `size`, `clear_region`, cursor management, scrolling region hooks when enabled.
  - Theme system (see Enhancements below).
- Features:
  - `std` (default off for embedding), `layout-cache`, `serde`, `palette`, `portable-atomic`, `anstyle`, `underline-color`, `scrolling-regions`.
- Tests:
  - Unit tests for layout, text width/line metrics, style merging, diff correctness.

### 3) crates/tuxtui-widgets
- Purpose: Official widget implementations.
- Widgets:
  - Core set: `Block`, `Paragraph`, `List`, `Table`, `Tabs`, `Gauge`, `BarChart`, `Sparkline`, `Chart`, `Scrollbar`, `Canvas`.
  - Calendar widget behind `widget-calendar` (requires `time`).
  - Enhancements: virtualization for large lists/tables, precise border overlap handling, richer highlight customization (row/column/cell), optional HSLuv/palette-based styling.
- Features:
  - Default: `all-widgets`.
  - `serde`, `calendar` (`widget-calendar`), `unstable-rendered-line-info`.
- Docs:
  - Each widget has examples and screenshots/recordings (vhs scripts in `examples/vhs` optional).

### 4) Backend crates
- crates/tuxtui-crossterm
  - Features: `crossterm_0_28`, `crossterm_0_29` (default latest), `serde`, `underline-color`, `scrolling-regions`, `unstable`, `unstable-backend-writer`.
- crates/tuxtui-termion
  - Features: `serde`, `scrolling-regions`, `unstable`, `unstable-backend-writer`.
- crates/tuxtui-termwiz
  - Features: `serde`, `underline-color`, `scrolling-regions`.
- All backends implement `Backend` requirements and pass backend conformance tests. Provide detailed docs on raw mode, alternate screen, mouse capture, and platform quirks.

### 5) crates/tuxtui-macros
- Provide `border!` macro (parity with ratatui), and optional future-friendly macros (e.g., style DSL) while keeping MSRV stable.
- Include compile tests via `trybuild`.

## Enhancements Over ratatui

- Theme System:
  - Define `tuxtui-theme` module in `tuxtui-core` (no extra crate) enabling:
    - Structs: `Theme`, `PaletteTheme`, `WidgetTheme`.
    - (De)serialization via `serde` (TOML/YAML/JSON).
    - Runtime application of themes to widgets and global defaults.
- Advanced Layout:
  - Negative spacing overlap baked-in with robust border overlap rendering.
  - Pluggable layout cache strategy (`layout-cache` feature) with `NonZeroUsize` capacity and perf tests.
- High-level App Harness (optional):
  - `App` builder in `tuxtui` with:
    - Tick rate, input handling hooks, clean shutdown, error handling, and async compatibility via `tokio` feature (optional; keep core sync).
- Rich Text:
  - Utilities to convert lightweight markup (e.g., simple inline tags) into `Text/Line/Span`.
- Performance:
  - Diff algorithm micro-optimizations and benchmarks.
  - Minimal allocations, leveraging `compact_str`/`smallvec` if warranted (be conservative).
- Testing UX:
  - `TestBackend` with helpers to assert frames, lines, and widget state.
  - Snapshot testing recipes (`insta`).
  - Debug-friendly `Debug` impls for `Text`, `Line`, `Span`, `Style` that round-trip.

## Dependencies (indicative, use latest compatible stable)

- Backends:
  - crossterm = "0.29"
  - termion = "4"
  - termwiz = "0.23"
- Core:
  - serde = { version = "1", features = ["derive"] }
  - palette = "0.7"
  - time = { version = "0.3", default-features = false }
  - unicode-segmentation = "1"
  - unicode-width = ">=0.2.0, <=0.2.1"
  - compact_str, itertools, lru, bitflags, anstyle (optional conversions)
  - thiserror, strum, rand (dev), pretty_assertions (dev), rstest (dev)
  - tracing (optional), tokio (optional for async examples)
- Keep features minimal by default; users opt-in.

## Root Workspace Cargo.toml

- Use `[workspace.package]` to set common metadata (authors, license, homepage, repository, readme, documentation, categories, keywords).
- Set `edition = "2024"` and `rust-version = "1.85.0"`.
- Lints: Forbid `unsafe_code` by default; configure clippy pedantic exceptions reasonably.
- Profiles: reasonable dev/test defaults; bench profile with LTO for stable benchmarks.

## Documentation

- Root README.md (top-level):
  - Project overview, goals, quickstart, feature matrix, backends, examples listing, migration guide from ratatui, badges, and links.
- Each crate README with crate-specific scope, features, and examples.
- Crate-level API docs with runnable doctests and links to examples.
- `examples/README.md` indexing every example with short descriptions and run commands.
- Contribution documentation:
  - CONTRIBUTING.md: small focused PRs, conventional commits, CI requirements, code style, testing expectations.
  - CODE_OF_CONDUCT.md and SECURITY.md present.

## Examples

- Provide a rich set under `examples/`:
  - Apps: hello_world, todo_list, popup, input_form, tracing_viewer, weather, async_github.
  - Concepts: layout constraints explorer, flex demo, chart, canvas drawing, color explorer, scrollbar, table, user input, widgetref container.
- Each example builds and runs with the default backend and documents how to switch backends.

## Testing & Benchmarks

- Unit tests across crates (core layout/text, widgets behavior).
- Integration tests using `TestBackend`.
- Snapshot testing recipes via `insta` (include instructions and some sample snapshots).
- Benchmarks (criterion) for diffing, layout solve, text wrapping.

## CI/CD

- .github/workflows/ci.yml:
  - Jobs: fmt, clippy (deny warnings), test matrix (OS × Rust), docs build, coverage (codecov), cargo-deny, typos, msrv check.
- .github/workflows/docs.yml: rustdoc check (and possibly link-checker).
- .github/workflows/release-plz.yml: automates release PRs and changelog using conventional commits.
- Configure codecov.yml.
- Ensure CI artifacts include coverage reports.

## Release & Versioning

- Conventional commits.
- git-cliff for CHANGELOG.md.
- release-plz monorepo configuration, generating tag-per-crate releases.
- Ensure `cargo publish --dry-run` passes for each crate.
- Breaking changes documented in CHANGELOG and a `BREAKING-CHANGES.md` section inside root README or dedicated file.

## Backwards Compatibility & Migration

- Provide a “Migrate from ratatui” section in the README with:
  - Full replace (`ratatui::` → `tuxtui::`).
  - Drop-in alias (`tui = { package = "tuxtui", ... }`) example.
  - Feature parity notes and any known differences (e.g., improved overlap rendering, theme system).

## Policies

- MSRV policy documented (1.85.0); bump policy only with minor releases and breaking-note.
- Security policy in SECURITY.md.
- Maintainers section listing author; organization attribution in README.

## Quality Gates (Acceptance Criteria)

- All crates build with `cargo build --workspace` on stable.
- All tests pass with `cargo test --workspace`.
- `cargo doc --workspace --no-deps` succeeds; doctests pass.
- `cargo fmt --all --check` and `cargo clippy --all-targets --all-features -D warnings` pass.
- `cargo deny check` passes (licenses, bans, advisories).
- `cargo publish --dry-run` passes for each publishable crate.
- Examples compile and run; README code blocks compile as doctests where feasible.
- No `unsafe` (forbid unsafe) unless explicitly justified and documented.

## Files to Generate (Non-code)

- LICENSE: MIT (full text).
- README.md files for root and each crate with badges and quickstart.
- CONTRIBUTING.md, CODE_OF_CONDUCT.md, SECURITY.md, CHANGELOG.md.
- ISSUE and PR templates.
- rustfmt, clippy, deny, cliff, release-plz, codecov configs.
- xtask CLI for developer flow (format, lint, test, docs, release checks).

## Hello World (README snippet to include)

Provide a minimal example in the main README using crossterm (default):

```rust
use crossterm::event::{self, Event};
use tuxtui::{DefaultTerminal, Frame};

fn main() -> std::io::Result<()> {
    let mut terminal: DefaultTerminal = tuxtui::init();
    let result = run(&mut terminal);
    tuxtui::restore();
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

Also show how to switch backends via features (termion/termwiz) in the README.

## Publishing

- Ensure crate metadata is correct:
  - `authors = ["Eshan Roy <eshanized@proton.me>"]`
  - `license = "MIT"`
  - `repository = "https://github.com/TIVerse/tuxtui"`
  - `homepage = "https://github.com/TIVerse/tuxtui"`
  - `documentation = "https://docs.rs/tuxtui"`
  - `readme = "README.md"`
  - `keywords`, `categories` as above.
- Use `exclude` to keep repo-only assets out of the crate tarball.
- Provide `package.metadata.docs.rs` to build with all-features where appropriate (core/widgets/backends) and `--cfg docsrs`.

## Final Step

- Run and pass:
  - `cargo build --workspace`
  - `cargo test --workspace`
  - `cargo doc --workspace --no-deps`
  - `cargo publish --dry-run` for all publishable crates

Deliver the full workspace with all crates, code, docs, examples, CI, and configs as specified above.

## Massive Specification (Expanded)

This section expands the full scope and depth of the tuxtui project for an approximately 35k SLOC workspace (±20%). Implement everything below unless marked as optional. The generated code must be original and not copy/paste from any other project. Preserve the MIT license for all crates. Use no unsafe code unless explicitly justified (none expected in v1).

### Vision & Scope

- Build a first-class, modular TUI framework providing:
  - Immediate-mode rendering and efficient buffer diffs.
  - Robust multi-backend abstraction with feature-gated integrations.
  - A comprehensive, themable widget suite and layout engine with border overlap support.
  - An optional high-level application harness for both sync and async.
  - Strong developer ergonomics, testing utilities, and performance tooling.
  - Carefully documented public APIs, with stability guarantees and feature gating.

### Legal & Attribution

- License: MIT for all crates and files.
- Author: `Eshan Roy <eshanized@proton.me>`; Organization: `Tonmoy Infrastructure & Vision`.
- Repository: `https://github.com/TIVerse/tuxtui`.
- Implementation must be original. Do not include assets, code, or content from other repositories unless compatible with MIT and properly attributed.

### Global Non-Functional Requirements

- MSRV: 1.85.0, edition 2024.
- Cross-platform: Linux, macOS, Windows (including modern Windows terminals).
- Performance: Avoid unnecessary allocations; ensure O(n) diff complexity per frame where n is number of changed cells.
- Unicode: Respect grapheme clusters and display width (CJK wide characters, emoji). Use `unicode-segmentation` and `unicode-width` strategy consistent with terminal expectations.
- Truecolor: Support 24-bit color; detect capabilities conservatively (do not assume truecolor in all environments) with opt-in overrides.
- No-std readiness (future): Keep core types organized such that `tuxtui-core` can work with `std` feature off (not mandatory to ship no-std today but structure should not preclude it).
- Safety: Forbid unsafe code.

---

## Architecture Deep Dive

The workspace consists of multiple crates with clear responsibilities. Provide crate-level READMEs and API docs that mirror the structure below.

### crates/tuxtui-core (foundation)

Modules and responsibilities:
- `prelude`: Common imports (types, traits) for convenience; minimal.
- `style`:
  - Types: `Color`, `Modifier`, `Style` with builder-style API and `Stylize` trait.
  - Color support: RGB, 256, named, and optional HSL/HSLuv via `palette` (feature-gated).
  - Underline color behind `underline-color` feature.
- `text`:
  - Types: `Span<'a>`, `Line<'a>`, `Text<'a>` with lifetimes and conversions from `&str`, `String`, `Cow<'a, str>`.
  - APIs for alignment (left/center/right), wrapping strategies (word, char, no-wrap), trimming, and width calculations.
  - Debug representations that can round-trip for snapshot testing.
- `buffer`:
  - `Cell` structure: char/grapheme storage, style, flags.
  - `Buffer` abstraction with random access, region fills, diffing utilities, and line abstractions.
  - Efficient diff output model (enumeration of operations) used by backends.
- `layout`:
  - `Rect`, `Margin`, `Alignment` (horizontal/vertical), `Constraint` (Length/Min/Max/Fill/Ratio), `Flex` modes.
  - `Spacing`: positive space and negative overlap; caching hooks behind `layout-cache` feature (capacity via `NonZeroUsize`).
  - `Layout` solver producing arrays/slices of `Rect` with optional spacer rects for debugging.
- `backend`:
  - `Backend` trait with associated `Error` type and methods: size, clear, clear_region, hide/show cursor, get/set cursor, set/clear styles, write cells/strings, scroll region ops (feature-gated), enable/disable raw, enter/leave alternate screen, mouse capture ops (optional), flush.
  - `TestBackend` implementation for tests and snapshots.
- `terminal`:
  - `Terminal<B: Backend>` orchestrates frame lifecycle, holds viewport, optional panic hook integration.
  - `TerminalOptions` (alternate screen on/off, panic hook enable, initial cursor visibility, vsync-ish throttling interval).
  - `Frame<'a>` providing `area()`, region splitting helpers, and `render_widget`/`render_stateful_widget`.
- `theme`:
  - `Theme`, `PaletteTheme`, `WidgetTheme` with serde (TOML/YAML/JSON) when `serde` feature enabled.
  - Global registry and per-widget styling application.
- `symbols`:
  - Box drawing sets, border presets (SIMPLE, DOUBLE, ROUNDED, THICK), scrollbar glyphs, sparkline bars, etc.
- `util`:
  - Width calculations, grapheme iterators, color support detection, small helpers for conversions.

Features (tuxtui-core):
- `std` (enables std-dependent code), `layout-cache`, `serde`, `palette`, `portable-atomic`, `anstyle`, `underline-color`, `scrolling-regions`.

### crates/tuxtui (end-user facade)

Responsibilities:
- Public entrypoint for app developers.
- Re-export main types: `Terminal`, `Frame`, text/style/layout primitives, widgets prelude.
- Provide `init()` and `restore()` helpers (crossterm by default) with panic-safe restore.
- Provide feature-gated re-exports: `CrosstermBackend`, `TermionBackend`, `TermwizBackend`.
- Optional `app` module (feature `harness`) offering an application harness (see below).

Application Harness (feature `harness`):
- Sync harness:
  - `App::builder()` for tick rate, event polling, initial app state, error handling hooks.
  - Event loop integrating input events and draw callbacks.
- Async harness (feature `tokio`):
  - `App::run_async()` compatible with Tokio; uses channels for events and a draw scheduler.
- Typed events:
  - `Event` enum with Key/Mouse/Resize/Tick; sourced from backend crates (crossterm default).
- Graceful shutdown, restore, and error surfaces.

### crates/tuxtui-widgets (official widgets)

Module layout and widgets:
- `block`: Borders, titles (multiple), title alignment and spans, padding, styles, border presets, z-order aware overlap drawing.
- `paragraph`: Rich text rendering with wrapping strategies, alignment per `Line`, selection highlighting, scroll offsets.
- `list`: Items with selection, virtualization interfaces for large lists, markers (symbols), per-item styles, separators.
- `table`: Rows/columns with selection state (row/column/cell), column widths (fixed, ratio, auto), virtualization, sticky headers.
- `tabs`: Deselectable tabs (Option<usize>), titles as `Line`, highlight styles, scrolling tabs when overflow.
- `gauge`: Linear and radial variants (radial optional), label customization.
- `barchart`: Multi-series, per-bar style, baseline support.
- `sparkline`: Item-first rendering, empty bar styling distinct from zero, color gradients.
- `chart`: Axes, labels (Into<Line>), legends, multiple datasets, scales.
- `scrollbar`: Horizontal/vertical, thumb sizing policies, track/arrow symbols.
- `canvas`: Shapes (rect, circle, polyline, bezier optional), layers, draw callbacks, mouse drawing example support.
- `tree` (new): Hierarchical rendering (file explorer), expand/collapse state, virtualization.
- `statusbar` (new): One-line bar with sections, left/center/right alignment, dynamic spans.
- `tooltip` and `context_menu` (new): Overlay widgets rendered in top layer.
- `logo` (tuxtui logo widget) example included.

Widget State & Traits:
- `StatefulWidget`/`StatefulWidgetRef` (feature-gated for unstable ref traits).
- Standard state types: `ListState`, `TableState`, `TabsState`, `TreeState`, with serde behind feature.

Features (tuxtui-widgets):
- Default `all-widgets` includes everything except costly/rare widgets.
- `widget-calendar` with `time` dependency.
- `serde`, `unstable-rendered-line-info` for paragraph line metrics.

### Backend crates

Shared goals:
- Implement `Backend` trait using terminal-specific APIs.
- Provide raw mode, alternate screen, mouse capture helpers.
- Expose version-pin feature flags to avoid dependency conflicts for libraries (e.g., `crossterm_0_28`, `crossterm_0_29`).

`tuxtui-crossterm`:
- Default latest crossterm; features: `serde`, `underline-color`, `scrolling-regions`, `unstable`, `unstable-backend-writer`.

`tuxtui-termion`:
- Features: `serde`, `scrolling-regions`, `unstable`, `unstable-backend-writer`.

`tuxtui-termwiz`:
- Features: `serde`, `underline-color`, `scrolling-regions`.

Test Backend:
- Provide `TestBackend` in core with capture buffers and utilities for asserting frames and diffs.

### tuxtui-macros

- `border!` macro compatible with style/type system.
- Optional future style DSL macros (keep minimal for v1).
- Compile tests using `trybuild`.

---

## Rendering Pipeline & Diffing

- Frame lifecycle: Begin frame → render widgets to `Buffer` → compute diff against previous buffer → emit backend ops → flush → end frame.
- Diff strategy: row-wise minimal updates; batch same-style runs; avoid redundant style resets.
- Scrolling regions (feature `scrolling-regions`): use terminal scroll regions to implement operations (e.g., `insert_before`) without flicker.
- Cursor management: hide during draw, restore as configured.

## Layout Engine Details

- Constraints: `Length(u16)`, `Min(u16)`, `Max(u16)`, `Fill(u16 weight)`, `Ratio(num, den)`.
- Flex: Start, End, Center, SpaceBetween, SpaceAround (follow documented semantics); document edge cases.
- Spacing: positive space or negative overlap; robust border overlap drawing order.
- Cache: keyed by `(Rect, Constraints, Flex, Spacing)`; size limited when `layout-cache` enabled.

## Text & Styling Details

- Rich text pipeline converts sequences of `Span`→`Line`→`Text`; alignment per line; wrap algorithms honoring grapheme clusters and widths.
- Style merging precedence (widget style → text style → span style).
- Color fallbacks for terminals without truecolor.
- Hyperlink support (where backend supports; otherwise no-op formatting).

## Theme System

- `Theme` structure with base `PaletteTheme` (colors, emphasis) and per-widget overrides (`WidgetTheme`).
- Load/save via serde; apply at runtime; example themes provided (dark, light, high-contrast, solarized).
- Docs on layering and merging rules.

## App Harness (optional)

- Sync and async (Tokio) variants with typed events and tick scheduling.
- Hooks: `on_event`, `on_tick`, `on_error`, `on_exit`.
- Graceful shutdown and restore on panic.
- Example: todo_list app with both sync and async harness variants.

---

## Feature Flags Inventory (finalized)

- Global (in tuxtui facade):
  - `crossterm` (default), `termion`, `termwiz` (choose one for apps)
  - `serde`, `palette`, `layout-cache`, `portable-atomic`, `macros`, `all-widgets`, `widget-calendar`, `scrolling-regions`, `underline-color`
  - Unstable: `unstable`, `unstable-rendered-line-info`, `unstable-widget-ref`, `unstable-backend-writer`
- Core and widgets/backends mirror above with crate-appropriate subsets.
- Document each feature in crate docs and README tables.

## Public API Surface (high-level)

Document signatures and primary methods in rustdoc for:
- `Terminal<B>`, `TerminalOptions`, `Frame<'_>`.
- `Backend` trait methods and expectations; error handling model.
- Text: `Span`, `Line`, `Text` conversions; `Stylize` methods.
- Layout: `Rect`, `Layout`, `Constraint`, `Flex`, `Spacing`, helpers.
- Widgets: constructors, builder methods, state types; `render`/`render_ref` patterns where applicable.

## Widgets Matrix (v1 scope)

Ship the following with docs and examples:
- Block, Paragraph, List (+VirtualList), Table (+VirtualTable), Tabs, Gauge (linear; radial optional), BarChart, Sparkline, Chart, Scrollbar, Canvas, Tree, StatusBar, Tooltip, ContextMenu, Logo.
- Provide per-widget README sections with screenshots (optional vhs scripts) and example code.

## Examples (expanded catalog)

Add at least the following examples under `examples/` with READMEs:
- Apps: `hello_world`, `todo_list`, `popup`, `input_form`, `tracing_viewer`, `weather`, `async_github`, `file_explorer` (Tree), `process_monitor` (gauges/charts), `log_viewer` (virtualized list/table), `theme_switcher`, `paint_canvas` (mouse drawing), `dashboard` (multi-pane layout), `editor_like` (scroll, cursor), `music_player` (tabs, lists, progress).
- Concepts: `layout_constraints`, `flex`, `chart`, `canvas_drawing`, `color_explorer`, `scrollbar`, `table`, `user_input`, `widgetref_container`, `overlap_borders_demo`, `themes_demo`.
- Ensure each runs with crossterm by default and documents how to switch backends.

## Testing Strategy

- Unit tests for core types (layout solver, width calcs, diff engine, theme merging).
- Widget tests covering rendering paths using `TestBackend` snapshots.
- Snapshot testing with `insta` (store snapshots under `tests/snapshots/`).
- Property-based tests (optional) for layout and text wrapping invariants with `proptest`.
- Backend conformance tests: ensure each backend passes the same set of rendering/diff expectations.

## Benchmarks

- `criterion` benchmarks for: layout solve, text wrapping (short/long), diffing on varying change ratios, widget rendering hot paths (Paragraph, List, Table).
- Include HTML report generation; CI can skip but keep benches compiling.

## Documentation & Site (optional)

- Crate docs serve as primary documentation; ensure all public items are documented.
- Optionally include an `mdbook` or static site under `docs/` with: Concepts, Tutorials, API links, Examples gallery.

## CI/CD (detailed)

- Matrix: {ubuntu-latest, macos-latest, windows-latest} × {stable, 1.85.0}.
- Jobs: fmt, clippy (deny warnings), tests (all features and minimal default-features), docs build (no-deps), coverage (tarpaulin on Linux), cargo-deny, typos, msrv check.
- Release automation via `release-plz` with changelog from `git-cliff` and tag-per-crate strategy.

## Release & Versioning

- Semantic versioning per crate. Keep cross-crate compat guarantees documented in `ARCHITECTURE.md`.
- Document breaking changes in `CHANGELOG.md` and `BREAKING-CHANGES.md` (root).
- Ensure `cargo publish --dry-run` passes per crate and root release instructions in `RELEASE.md`.

## Security & Policies

- SECURITY.md with vulnerability reporting policy.
- CODE_OF_CONDUCT.md and CONTRIBUTING.md (small focused PRs, conventional commits, docs/tests expected).
- Forbid unsafe; if introduced later, must be justified, audited, and covered by tests.

## Roadmap (post-v1 ideas)

- No-std support in core.
- Additional backends (e.g., termwiz advanced features).
- Rich text markup parser crate (optional sub-crate) for markdown-ish rendering.
- Accessibility: high-contrast themes, screen-reader-friendly modes (where possible).
- Performance profiles and renderer pipelines with pluggable strategies.

## SLOC & Quality Bars

- Target ~35k SLOC across all crates, tests, and examples (±20%).
- Ensure high test coverage on core and widgets critical paths; provide coverage badge via codecov.
- All docs and examples compile; doctests enabled.

## Final Acceptance (Expanded)

- Build, test, doc, lint, deny, typos all pass in CI.
- Examples runnable and documented.
- Public API documented and feature flags tables present.
- Dry-run publish passes for all publishable crates.

