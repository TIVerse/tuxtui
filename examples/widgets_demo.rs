//! Demo showcasing various widgets

use crossterm::event::{self, Event, KeyCode};
use tuxtui::prelude::*;
use tuxtui::widgets::{
    block::{Block, BorderType},
    gauge::Gauge,
    list::{List, ListItem, ListState},
    paragraph::Paragraph,
};

struct App {
    list_state: ListState,
    progress: u16,
}

impl App {
    fn new() -> Self {
        let mut state = ListState::default();
        state.select(Some(0));
        Self {
            list_state: state,
            progress: 0,
        }
    }

    fn on_tick(&mut self) {
        self.progress = (self.progress + 5) % 101;
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

            // Split the screen
            let mut layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Fill(1),
                    Constraint::Length(3),
                ]);

            let chunks = layout.split(area);

            // Title block
            let title = Block::default()
                .title("Widget Demo")
                .borders(BorderType::All);
            frame.render_widget(title, chunks[0]);

            // Middle section - split horizontally
            let mut middle_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]);

            let middle_chunks = middle_layout.split(chunks[1]);

            // List widget
            let items = vec![
                ListItem::new("Item 1"),
                ListItem::new("Item 2"),
                ListItem::new("Item 3"),
                ListItem::new("Item 4"),
            ];

            let list = List::new(items)
                .highlight_style(Style::default().bg(Color::Blue).fg(Color::White));

            let list_block = Block::default()
                .title("List")
                .borders(BorderType::All);

            let list_inner = list_block.inner(middle_chunks[0]);
            frame.render_widget(list_block, middle_chunks[0]);
            list.render_stateful(list_inner, frame.buffer_mut(), &mut app.list_state);

            // Paragraph widget
            let text = Text::from(vec![
                Line::from("Welcome to tuxtui!"),
                Line::from(""),
                Line::from("A powerful TUI library."),
            ]);

            let paragraph = Paragraph::new(text);
            let para_block = Block::default()
                .title("Info")
                .borders(BorderType::All);

            let para_inner = para_block.inner(middle_chunks[1]);
            frame.render_widget(para_block, middle_chunks[1]);
            frame.render_widget(paragraph, para_inner);

            // Progress gauge
            let gauge = Gauge::default()
                .percent(app.progress)
                .label(format!("{}%", app.progress))
                .gauge_style(Style::default().fg(Color::Green));

            let gauge_block = Block::default()
                .title("Progress")
                .borders(BorderType::All);

            let gauge_inner = gauge_block.inner(chunks[2]);
            frame.render_widget(gauge_block, chunks[2]);
            frame.render_widget(gauge, gauge_inner);
        })?;

        app.on_tick();

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Up => app.list_state.select_previous(4),
                    KeyCode::Down => app.list_state.select_next(4),
                    _ => {}
                }
            }
        }
    }
}
