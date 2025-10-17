# tuxtui v0.1.0 - Quick Reference Guide

**New Features Quick Start**

---

## 🎨 Color Parsing

```rust
use tuxtui::prelude::*;

// Named colors
let red = Color::from_str("red")?;
let blue = Color::from_str("lightblue")?;

// Hex colors
let orange = Color::from_str("#FF8800")?;
let purple = Color::from_str("#F0F")?;  // Short form

// RGB format
let green = Color::from_str("rgb(0, 255, 0)")?;

// Indexed (0-255)
let indexed = Color::from_str("42")?;

// Use in styles
let style = Style::default()
    .fg(Color::from_str("#FF5733")?)
    .bg(Color::from_str("blue")?);
```

---

## 📐 Position Arithmetic

```rust
use tuxtui::prelude::*;

let pos1 = Position::new(10, 20);
let pos2 = Position::new(5, 3);

// Addition
let sum = pos1 + pos2;  // Position { x: 15, y: 23 }

// Subtraction
let diff = pos1 - pos2; // Position { x: 5, y: 17 }

// In-place
let mut pos = Position::new(10, 10);
pos += Position::new(5, 5);

// Distance
let distance = pos1.distance_to(pos2);
```

---

## 🔒 Password Input (Masked Text)

```rust
use tuxtui::prelude::*;
use tuxtui::widgets::input::{TextInput, InputState};

let mut password_state = InputState::new();

let password_field = TextInput::default()
    .mask_char(Some('*'))              // Mask with asterisks
    .placeholder("Enter password...")
    .style(Style::default().fg(Color::Yellow));

// Or use bullet points
let pin_field = TextInput::default()
    .mask_char(Some('•'))
    .placeholder("PIN...");

// Render in your frame
password_field.render_stateful(area, buf, &mut password_state);
```

---

## ✂️ Text Truncation

```rust
use tuxtui::prelude::*;

let long_text = Line::from("This is a very long line that needs truncation");

// Default ellipsis ("...")
let truncated = long_text.truncate(20, None);

// Custom ellipsis
let truncated = long_text.truncate(20, Some(" […]"));
let truncated = long_text.truncate(20, Some("…"));

// Works with styled text
let styled = Line::from(vec![
    Span::styled("Hello ", Style::default().fg(Color::Blue)),
    Span::styled("World!", Style::default().fg(Color::Red)),
]);
let truncated = styled.truncate(8, Some("…"));
```

---

## 📜 List Start Corner (Reverse Rendering)

```rust
use tuxtui::prelude::*;
use tuxtui::widgets::list::{List, ListItem, Corner};

let messages = vec![
    ListItem::new("Message 1"),
    ListItem::new("Message 2"),
    ListItem::new("Message 3"),
];

// Normal rendering (top to bottom)
let list = List::new(messages.clone())
    .start_corner(Corner::TopLeft);

// Reverse rendering (bottom to top) - perfect for chat!
let chat_list = List::new(messages)
    .start_corner(Corner::BottomLeft)
    .highlight_style(Style::default().bg(Color::Blue));
```

---

## ☑️ Multi-Select Lists

```rust
use tuxtui::prelude::*;
use tuxtui::widgets::list::{List, ListState};

let mut state = ListState::default();

// Toggle selection (e.g., on Space key press)
if key.code == KeyCode::Char(' ') {
    if let Some(idx) = state.selected() {
        state.toggle_selection(idx);
    }
}

// Check if item is selected
if state.is_selected(0) {
    println!("Item 0 is selected!");
}

// Get all selections
let selected_indices = state.selected_items(); // &[0, 2, 5]

// Clear all selections
state.clear_selections();

// Select multiple at once
state.select_multiple(vec![1, 3, 5, 7]);

// Render with custom style for selected items
let list = List::new(items)
    .highlight_style(Style::default().bg(Color::Blue));
```

---

## 🖱️ Mouse Events

```rust
use tuxtui::prelude::*;
use crossterm::event::{self, Event};

loop {
    if let Event::Mouse(mouse) = event::read()? {
        let mouse_event = MouseEvent::new(
            MouseEventKind::from(mouse.kind),
            mouse.column,
            mouse.row,
        );

        // Check if click is in area
        if mouse_event.is_click_in(button_area) {
            println!("Button clicked!");
        }

        // Or check specific position
        if mouse_event.is_click_at(10, 20) {
            println!("Clicked at (10, 20)!");
        }
    }
}
```

---

## 📊 ViewportState (Advanced Scrolling)

```rust
use tuxtui_core::viewport::ViewportState;

let mut viewport = ViewportState::new()
    .content_length(100)    // Total items
    .viewport_height(20);   // Visible items

// Scrolling
viewport.scroll_down();
viewport.scroll_up();
viewport.page_down();
viewport.page_up();
viewport.scroll_to_top();
viewport.scroll_to_bottom();

// Selection with auto-scroll
viewport.select(Some(50));  // Automatically ensures visible
viewport.select_next();
viewport.select_previous();

// Get info
let offset = viewport.offset();
let (start, end) = viewport.visible_range();
let progress = viewport.scroll_percentage(); // 0.0 to 1.0
```

---

## 🎨 Layout Helpers

```rust
use tuxtui::prelude::*;

// Old way (still works)
let layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([Constraint::Fill(1), Constraint::Length(3)]);

// New way (cleaner!)
let layout = Layout::vertical([
    Constraint::Fill(1),
    Constraint::Length(3),
]);

let layout = Layout::horizontal([
    Constraint::Percentage(50),
    Constraint::Percentage(50),
]);

// All constraint types available
let layout = Layout::vertical([
    Constraint::Length(5),      // Fixed size
    Constraint::Min(10),        // At least 10
    Constraint::Max(20),        // At most 20
    Constraint::Percentage(30), // 30% of space
    Constraint::Ratio(1, 3),    // 1/3 of space
    Constraint::Fill(1),        // Fill remaining
]);
```

---

## 🔧 Complete Example: Password Login Form

```rust
use tuxtui::prelude::*;
use tuxtui::widgets::input::{TextInput, InputState};
use tuxtui::widgets::block::{Block, BorderType};
use crossterm::event::{self, Event, KeyCode};

struct LoginForm {
    username: InputState,
    password: InputState,
    active_field: usize,
}

impl LoginForm {
    fn new() -> Self {
        Self {
            username: InputState::new(),
            password: InputState::new(),
            active_field: 0,
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.area();
        
        let chunks = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Length(3),
            Constraint::Fill(1),
        ]).split(area);

        // Username field
        let username_block = Block::default()
            .title("Username")
            .borders(BorderType::All)
            .style(if self.active_field == 0 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });
        
        let username_input = TextInput::default()
            .placeholder("Enter username...")
            .show_cursor(self.active_field == 0);
        
        frame.render_widget(username_block, chunks[1]);
        let inner = chunks[1].inner(&Margin::new(1, 1));
        username_input.render_stateful(inner, frame.buffer_mut(), &mut self.username.clone());

        // Password field
        let password_block = Block::default()
            .title("Password")
            .borders(BorderType::All)
            .style(if self.active_field == 1 {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });
        
        let password_input = TextInput::default()
            .mask_char(Some('*'))  // Mask password!
            .placeholder("Enter password...")
            .show_cursor(self.active_field == 1);
        
        frame.render_widget(password_block, chunks[2]);
        let inner = chunks[2].inner(&Margin::new(1, 1));
        password_input.render_stateful(inner, frame.buffer_mut(), &mut self.password.clone());
    }

    fn handle_input(&mut self, key: KeyCode) {
        match key {
            KeyCode::Tab => {
                self.active_field = (self.active_field + 1) % 2;
            }
            KeyCode::Enter => {
                if self.active_field == 1 {
                    println!("Login: {} / {}", 
                        self.username.value(), 
                        self.password.value());
                }
            }
            _ => {
                if self.active_field == 0 {
                    // Handle username input
                } else {
                    // Handle password input
                }
            }
        }
    }
}
```

---

## 📚 All New Features Summary

| Feature | Module | Use Case |
|---------|--------|----------|
| `Color::from_str()` | `tuxtui_core::style` | Parse colors from config |
| Position arithmetic | `tuxtui_core::geometry` | Calculate positions |
| `TextInput::mask_char()` | `tuxtui_widgets::input` | Password fields |
| `Line::truncate()` | `tuxtui_core::text` | Limit text width |
| `List::start_corner()` | `tuxtui_widgets::list` | Reverse rendering |
| Multi-select | `tuxtui_widgets::list` | Select multiple items |
| `ViewportState` | `tuxtui_core::viewport` | Advanced scrolling |
| `MouseEvent` | `tuxtui_core::event` | Mouse handling |
| Layout helpers | `tuxtui_core::layout` | Cleaner API |

---

## 🔗 See Also

- **IMPLEMENTATION_COMPLETE.md** - Detailed feature documentation
- **MISSING_FEATURES.md** - What's implemented and what's pending
- **STATUS.md** - Overall project status
- **WHATS_NEW.md** - Version 0.1.0 changelog
- **examples/** - Working example applications

---

**Happy coding with tuxtui! 🎉**
