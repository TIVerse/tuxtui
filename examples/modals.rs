//! Modal and popup example

use crossterm::event::{self, Event, KeyCode};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use tuxtui::widgets::popup::{Modal, Popup};
use tuxtui::widgets::paragraph::Paragraph;

enum ModalType {
    None,
    Confirm,
    Info,
    Warning,
}

struct App {
    show_modal: ModalType,
    modal_selected: usize,
    result: String,
}

impl App {
    fn new() -> Self {
        Self {
            show_modal: ModalType::None,
            modal_selected: 0,
            result: String::new(),
        }
    }

    fn show_confirm(&mut self) {
        self.show_modal = ModalType::Confirm;
        self.modal_selected = 0;
    }

    fn show_info(&mut self) {
        self.show_modal = ModalType::Info;
        self.modal_selected = 0;
    }

    fn show_warning(&mut self) {
        self.show_modal = ModalType::Warning;
        self.modal_selected = 0;
    }

    fn close_modal(&mut self, confirmed: bool) {
        if confirmed {
            self.result = match self.show_modal {
                ModalType::Confirm => "Confirmed!".to_string(),
                ModalType::Info => "Info acknowledged".to_string(),
                ModalType::Warning => "Warning accepted".to_string(),
                ModalType::None => String::new(),
            };
        } else {
            self.result = "Cancelled".to_string();
        }
        self.show_modal = ModalType::None;
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

            // Main content
            let block = Block::default()
                .title("Modal/Popup Demo (q: quit)")
                .borders(BorderType::All);

            let inner = block.inner(area);
            frame.render_widget(block, area);

            // Instructions
            let instructions = Text::from(vec![
                Line::from("Press keys to show different modals:"),
                Line::from(""),
                Line::from("c - Confirmation Dialog"),
                Line::from("i - Information Dialog"),
                Line::from("w - Warning Dialog"),
                Line::from(""),
                Line::from(format!("Last result: {}", app.result)),
                Line::from(""),
                Line::from("In modal:"),
                Line::from("  Tab/←/→ - Navigate buttons"),
                Line::from("  Enter - Confirm"),
                Line::from("  Esc - Cancel"),
            ]);

            let paragraph = Paragraph::new(instructions);
            frame.render_widget(paragraph, inner);

            // Render modal if active
            match app.show_modal {
                ModalType::None => {}
                ModalType::Confirm => {
                    let popup = Popup::new()
                        .percent_x(60)
                        .percent_y(30)
                        .background_style(Style::default().bg(Color::Black).add_modifier(Modifier::DIM));

                    let mut modal = Modal::new(
                        "Confirm Action",
                        Text::from(vec![
                            Line::from("Are you sure you want to proceed?"),
                            Line::from("This action cannot be undone."),
                        ]),
                    )
                    .buttons(&["Yes", "No"])
                    .selected_button(app.modal_selected)
                    .style(Style::default().bg(Color::Blue).fg(Color::White))
                    .selected_style(Style::default().bg(Color::Yellow).fg(Color::Black));

                    let modal_area = popup.area(area);
                    popup.render_widget(area, frame.buffer_mut(), modal);
                }
                ModalType::Info => {
                    let popup = Popup::new()
                        .percent_x(50)
                        .percent_y(25)
                        .background_style(Style::default().bg(Color::Black).add_modifier(Modifier::DIM));

                    let modal = Modal::new(
                        "Information",
                        "This is an informational message.\n\nEverything is working as expected!",
                    )
                    .buttons(&["OK"])
                    .style(Style::default().bg(Color::Cyan).fg(Color::Black));

                    popup.render_widget(area, frame.buffer_mut(), modal);
                }
                ModalType::Warning => {
                    let popup = Popup::new()
                        .percent_x(55)
                        .percent_y(28)
                        .background_style(Style::default().bg(Color::Black).add_modifier(Modifier::DIM));

                    let mut modal = Modal::new(
                        "⚠ Warning",
                        Text::from(vec![
                            Line::from("This is a warning message!"),
                            Line::from(""),
                            Line::from("Please review before continuing."),
                        ]),
                    )
                    .buttons(&["Continue", "Cancel"])
                    .selected_button(app.modal_selected)
                    .style(Style::default().bg(Color::Yellow).fg(Color::Black))
                    .selected_style(Style::default().bg(Color::Red).fg(Color::White));

                    popup.render_widget(area, frame.buffer_mut(), modal);
                }
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match app.show_modal {
                ModalType::None => match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Char('c') => app.show_confirm(),
                    KeyCode::Char('i') => app.show_info(),
                    KeyCode::Char('w') => app.show_warning(),
                    _ => {}
                },
                _ => match key.code {
                    KeyCode::Enter => {
                        let confirmed = app.modal_selected == 0;
                        app.close_modal(confirmed);
                    }
                    KeyCode::Esc => app.close_modal(false),
                    KeyCode::Tab | KeyCode::Right => {
                        app.modal_selected = if app.modal_selected == 0 { 1 } else { 0 };
                    }
                    KeyCode::Left => {
                        app.modal_selected = if app.modal_selected == 0 { 1 } else { 0 };
                    }
                    _ => {}
                },
            }
        }
    }
}
