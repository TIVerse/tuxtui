//! Todo list application example

use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use tuxtui::widgets::input::{InputState, TextInput};
use tuxtui::widgets::list::{List, ListItem, ListState};
use tuxtui::widgets::paragraph::Paragraph;

#[derive(Debug, Clone)]
struct TodoItem {
    text: String,
    completed: bool,
}

enum AppMode {
    Normal,
    Insert,
}

struct App {
    todos: Vec<TodoItem>,
    list_state: ListState,
    input_state: InputState,
    mode: AppMode,
}

impl App {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));

        Self {
            todos: vec![
                TodoItem {
                    text: "Learn Rust".to_string(),
                    completed: true,
                },
                TodoItem {
                    text: "Build a TUI app".to_string(),
                    completed: false,
                },
                TodoItem {
                    text: "Publish to crates.io".to_string(),
                    completed: false,
                },
            ],
            list_state,
            input_state: InputState::new(),
            mode: AppMode::Normal,
        }
    }

    fn toggle_current(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.todos.len() {
                self.todos[selected].completed = !self.todos[selected].completed;
            }
        }
    }

    fn delete_current(&mut self) {
        if let Some(selected) = self.list_state.selected() {
            if selected < self.todos.len() {
                self.todos.remove(selected);
                if self.todos.is_empty() {
                    self.list_state.select(None);
                } else if selected >= self.todos.len() {
                    self.list_state.select(Some(self.todos.len() - 1));
                }
            }
        }
    }

    fn add_todo(&mut self) {
        let text = self.input_state.value().trim().to_string();
        if !text.is_empty() {
            self.todos.push(TodoItem {
                text,
                completed: false,
            });
            self.input_state.clear();
            self.mode = AppMode::Normal;
            self.list_state.select(Some(self.todos.len() - 1));
        }
    }

    fn stats(&self) -> (usize, usize) {
        let completed = self.todos.iter().filter(|t| t.completed).count();
        (completed, self.todos.len())
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    let mut app = App::new();

    let result = run(&mut terminal, &mut app);
    tuxtui::restore()?;
    result
}

fn run(terminal: &mut tuxtui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout
            let mut layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Fill(1),
                    Constraint::Length(3),
                    Constraint::Length(5),
                ]);

            let chunks = layout.split(area);

            // Title
            let (completed, total) = app.stats();
            let title = Block::default()
                .title(format!(
                    "Todo List ({}/{} completed)",
                    completed, total
                ))
                .borders(BorderType::All)
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(title, chunks[0]);

            // Todo list
            let list_block = Block::default()
                .title("Tasks")
                .borders(BorderType::All);
            let list_inner = list_block.inner(chunks[1]);
            frame.render_widget(list_block, chunks[1]);

            let items: Vec<ListItem> = app
                .todos
                .iter()
                .map(|todo| {
                    let prefix = if todo.completed { "[✓] " } else { "[ ] " };
                    let style = if todo.completed {
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::CROSSED_OUT)
                    } else {
                        Style::default().fg(Color::White)
                    };
                    ListItem::new(format!("{}{}", prefix, todo.text)).style(style)
                })
                .collect();

            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            list.render_stateful(list_inner, frame.buffer_mut(), &mut app.list_state);

            // Input area
            let input_block = Block::default()
                .title("Add New Task (i: insert, Esc: cancel)")
                .borders(BorderType::All)
                .style(match app.mode {
                    AppMode::Insert => Style::default().fg(Color::Yellow),
                    AppMode::Normal => Style::default(),
                });
            let input_inner = input_block.inner(chunks[2]);
            frame.render_widget(input_block, chunks[2]);

            let input = TextInput::default()
                .placeholder("Enter task description...")
                .style(Style::default().fg(Color::White))
                .show_cursor(matches!(app.mode, AppMode::Insert));
            input.render_stateful(input_inner, frame.buffer_mut(), &mut app.input_state);

            // Help
            let help_text = match app.mode {
                AppMode::Normal => {
                    "↑/↓: navigate | Space: toggle | d: delete | i: insert | q: quit"
                }
                AppMode::Insert => "Enter: add | Esc: cancel | Type to add new task",
            };

            let help = Paragraph::new(Text::from(help_text))
                .style(Style::default().fg(Color::Gray));
            let help_block = Block::default()
                .title("Help")
                .borders(BorderType::All);
            let help_inner = help_block.inner(chunks[3]);
            frame.render_widget(help_block, chunks[3]);
            frame.render_widget(help, help_inner);
        })?;

        if let Event::Key(KeyEvent { code, .. }) = event::read()? {
            match app.mode {
                AppMode::Normal => match code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('i') => {
                        app.mode = AppMode::Insert;
                        app.input_state.clear();
                    }
                    KeyCode::Char(' ') => app.toggle_current(),
                    KeyCode::Char('d') => app.delete_current(),
                    KeyCode::Down => app.list_state.select_next(app.todos.len()),
                    KeyCode::Up => app.list_state.select_previous(app.todos.len()),
                    _ => {}
                },
                AppMode::Insert => match code {
                    KeyCode::Esc => {
                        app.mode = AppMode::Normal;
                        app.input_state.clear();
                    }
                    KeyCode::Enter => app.add_todo(),
                    KeyCode::Char(c) => app.input_state.insert_char(c),
                    KeyCode::Backspace => app.input_state.delete_char(),
                    KeyCode::Left => app.input_state.move_cursor_left(),
                    KeyCode::Right => app.input_state.move_cursor_right(),
                    KeyCode::Home => app.input_state.move_cursor_start(),
                    KeyCode::End => app.input_state.move_cursor_end(),
                    _ => {}
                },
            }
        }
    }
}
