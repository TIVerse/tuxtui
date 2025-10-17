//! Form input example with text inputs

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use tuxtui::widgets::input::{TextInput, InputState};
use tuxtui::widgets::paragraph::Paragraph;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FocusedField {
    Name,
    Email,
    Message,
}

struct App {
    name: InputState,
    email: InputState,
    message: InputState,
    focused: FocusedField,
    submitted: Option<String>,
}

impl App {
    fn new() -> Self {
        Self {
            name: InputState::new(),
            email: InputState::new(),
            message: InputState::new(),
            focused: FocusedField::Name,
            submitted: None,
        }
    }

    fn current_input(&mut self) -> &mut InputState {
        match self.focused {
            FocusedField::Name => &mut self.name,
            FocusedField::Email => &mut self.email,
            FocusedField::Message => &mut self.message,
        }
    }

    fn next_field(&mut self) {
        self.focused = match self.focused {
            FocusedField::Name => FocusedField::Email,
            FocusedField::Email => FocusedField::Message,
            FocusedField::Message => FocusedField::Name,
        };
    }

    fn previous_field(&mut self) {
        self.focused = match self.focused {
            FocusedField::Name => FocusedField::Message,
            FocusedField::Email => FocusedField::Name,
            FocusedField::Message => FocusedField::Email,
        };
    }

    fn submit(&mut self) {
        self.submitted = Some(format!(
            "Name: {}\nEmail: {}\nMessage: {}",
            self.name.value(),
            self.email.value(),
            self.message.value()
        ));
    }

    fn handle_char(&mut self, c: char) {
        self.current_input().insert_char(c);
    }

    fn handle_backspace(&mut self) {
        self.current_input().delete_char();
    }

    fn handle_left(&mut self) {
        self.current_input().move_cursor_left();
    }

    fn handle_right(&mut self) {
        self.current_input().move_cursor_right();
    }

    fn handle_home(&mut self) {
        self.current_input().move_cursor_start();
    }

    fn handle_end(&mut self) {
        self.current_input().move_cursor_end();
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
                    Constraint::Length(3),
                    Constraint::Length(5),
                    Constraint::Fill(1),
                    Constraint::Length(3),
                ]);

            let chunks = layout.split(area);

            // Title
            let title = Block::default()
                .title("Contact Form (Tab/Shift+Tab: switch, Ctrl+S: submit, q: quit)")
                .borders(BorderType::All);
            frame.render_widget(title, chunks[0]);

            // Name field
            let name_block = Block::default()
                .title("Name")
                .borders(BorderType::All)
                .style(if app.focused == FocusedField::Name {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                });
            let name_inner = name_block.inner(chunks[1]);
            frame.render_widget(name_block, chunks[1]);

            let name_input = TextInput::default()
                .placeholder("Enter your name...")
                .style(Style::default().fg(Color::White));
            name_input.render_stateful(name_inner, frame.buffer_mut(), &mut app.name);

            // Email field
            let email_block = Block::default()
                .title("Email")
                .borders(BorderType::All)
                .style(if app.focused == FocusedField::Email {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                });
            let email_inner = email_block.inner(chunks[2]);
            frame.render_widget(email_block, chunks[2]);

            let email_input = TextInput::default()
                .placeholder("your@email.com...")
                .style(Style::default().fg(Color::White));
            email_input.render_stateful(email_inner, frame.buffer_mut(), &mut app.email);

            // Message field
            let message_block = Block::default()
                .title("Message")
                .borders(BorderType::All)
                .style(if app.focused == FocusedField::Message {
                    Style::default().fg(Color::Yellow)
                } else {
                    Style::default()
                });
            let message_inner = message_block.inner(chunks[3]);
            frame.render_widget(message_block, chunks[3]);

            let message_input = TextInput::default()
                .placeholder("Your message...")
                .style(Style::default().fg(Color::White));
            message_input.render_stateful(message_inner, frame.buffer_mut(), &mut app.message);

            // Result area
            if let Some(result) = &app.submitted {
                let result_text = Text::from(format!("Submitted:\n{}", result));
                let result_para = Paragraph::new(result_text)
                    .style(Style::default().fg(Color::Green));
                let result_block = Block::default()
                    .title("Submission Result")
                    .borders(BorderType::All);
                let result_inner = result_block.inner(chunks[4]);
                frame.render_widget(result_block, chunks[4]);
                frame.render_widget(result_para, result_inner);
            } else {
                let help_block = Block::default()
                    .title("Help")
                    .borders(BorderType::All);
                frame.render_widget(help_block, chunks[4]);
                let help_text = Text::from("Press Ctrl+S to submit the form");
                let help_para = Paragraph::new(help_text);
                let help_inner = help_block.inner(chunks[4]);
                frame.render_widget(help_para, help_inner);
            }
        })?;

        if let Event::Key(KeyEvent {
            code,
            modifiers,
            ..
        }) = event::read()?
        {
            match code {
                KeyCode::Char('q') => break Ok(()),
                KeyCode::Char('s') if modifiers.contains(KeyModifiers::CONTROL) => {
                    app.submit();
                }
                KeyCode::Tab if modifiers.contains(KeyModifiers::SHIFT) => {
                    app.previous_field();
                }
                KeyCode::Tab => {
                    app.next_field();
                }
                KeyCode::Char(c) => {
                    app.handle_char(c);
                }
                KeyCode::Backspace => {
                    app.handle_backspace();
                }
                KeyCode::Left => {
                    app.handle_left();
                }
                KeyCode::Right => {
                    app.handle_right();
                }
                KeyCode::Home => {
                    app.handle_home();
                }
                KeyCode::End => {
                    app.handle_end();
                }
                _ => {}
            }
        }
    }
}
