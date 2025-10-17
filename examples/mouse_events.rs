//! Mouse event handling example

use crossterm::event::{self, Event, MouseEventKind as CrosstermMouseKind};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};
use tuxtui::widgets::paragraph::Paragraph;

struct Button {
    label: String,
    area: Rect,
    clicked: bool,
}

impl Button {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            area: Rect::default(),
            clicked: false,
        }
    }

    fn set_area(&mut self, area: Rect) {
        self.area = area;
    }

    fn is_clicked(&self, x: u16, y: u16) -> bool {
        x >= self.area.left()
            && x < self.area.right()
            && y >= self.area.top()
            && y < self.area.bottom()
    }

    fn render(&self, frame: &mut Frame) {
        let style = if self.clicked {
            Style::default().bg(Color::Green).fg(Color::Black)
        } else {
            Style::default().bg(Color::Blue).fg(Color::White)
        };

        let block = Block::default()
            .borders(BorderType::All)
            .style(style);

        let inner = block.inner(self.area);
        frame.render_widget(block, self.area);

        let text = Text::from(self.label.as_str());
        let paragraph = Paragraph::new(text)
            .alignment(Alignment::Center)
            .style(style);
        frame.render_widget(paragraph, inner);
    }
}

struct App {
    buttons: Vec<Button>,
    last_click: String,
    mouse_pos: (u16, u16),
    click_count: usize,
}

impl App {
    fn new() -> Self {
        Self {
            buttons: vec![
                Button::new("Button 1"),
                Button::new("Button 2"),
                Button::new("Button 3"),
                Button::new("Click Me!"),
            ],
            last_click: "None".to_string(),
            mouse_pos: (0, 0),
            click_count: 0,
        }
    }

    fn handle_mouse_event(&mut self, col: u16, row: u16, kind: CrosstermMouseKind) {
        self.mouse_pos = (col, row);

        if matches!(kind, CrosstermMouseKind::Down(_)) {
            self.click_count += 1;

            // Reset all buttons
            for button in &mut self.buttons {
                button.clicked = false;
            }

            // Check which button was clicked
            for button in &mut self.buttons {
                if button.is_clicked(col, row) {
                    button.clicked = true;
                    self.last_click = button.label.clone();
                    break;
                }
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // Enable mouse capture
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::EnableMouseCapture
    )?;

    let mut terminal = tuxtui::init()?;
    let mut app = App::new();

    let result = run(&mut terminal, &mut app);

    // Disable mouse capture
    crossterm::execute!(
        std::io::stdout(),
        crossterm::event::DisableMouseCapture
    )?;

    tuxtui::restore()?;
    result
}

fn run(terminal: &mut tuxtui::DefaultTerminal, app: &mut App) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout
            let mut layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(5),
            ]);

            let chunks = layout.split(area);

            // Title
            let title = Block::default()
                .title("Mouse Event Demo (q: quit, click buttons!)")
                .borders(BorderType::All);
            frame.render_widget(title, chunks[0]);

            // Button area
            let button_block = Block::default()
                .title("Interactive Buttons")
                .borders(BorderType::All);

            let button_inner = button_block.inner(chunks[1]);
            frame.render_widget(button_block, chunks[1]);

            // Layout buttons in a grid
            let mut button_layout = Layout::vertical([
                Constraint::Fill(1),
                Constraint::Length(5),
                Constraint::Length(5),
                Constraint::Fill(1),
            ]);

            let rows = button_layout.split(button_inner);

            let mut col_layout = Layout::horizontal([
                Constraint::Fill(1),
                Constraint::Length(15),
                Constraint::Length(15),
                Constraint::Fill(1),
            ]);

            let cols = col_layout.split(rows[1]);

            // Set button areas
            app.buttons[0].set_area(cols[1]);
            app.buttons[1].set_area(cols[2]);

            let cols2 = col_layout.split(rows[2]);
            app.buttons[2].set_area(cols2[1]);
            app.buttons[3].set_area(cols2[2]);

            // Render buttons
            for button in &app.buttons {
                button.render(frame);
            }

            // Info area
            let info_text = Text::from(vec![
                Line::from(format!("Mouse Position: ({}, {})", app.mouse_pos.0, app.mouse_pos.1)),
                Line::from(format!("Last Clicked: {}", app.last_click)),
                Line::from(format!("Total Clicks: {}", app.click_count)),
            ]);

            let info = Paragraph::new(info_text);
            let info_block = Block::default()
                .title("Info")
                .borders(BorderType::All);

            let info_inner = info_block.inner(chunks[2]);
            frame.render_widget(info_block, chunks[2]);
            frame.render_widget(info, info_inner);
        })?;

        match event::read()? {
            Event::Key(key) => {
                if matches!(key.code, crossterm::event::KeyCode::Char('q')) {
                    break Ok(());
                }
            }
            Event::Mouse(mouse) => {
                app.handle_mouse_event(mouse.column, mouse.row, mouse.kind);
            }
            _ => {}
        }
    }
}
