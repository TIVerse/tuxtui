//! Simple hello world example for tuxtui

use crossterm::event::{self, Event, KeyCode};
use tuxtui::prelude::*;
use tuxtui::widgets::block::{Block, BorderType};

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    let result = run(&mut terminal);
    tuxtui::restore()?;
    result
}

fn run(terminal: &mut tuxtui::DefaultTerminal) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();
            
            let block = Block::default()
                .title("Hello tuxtui!")
                .borders(BorderType::All);
            
            let inner = block.inner(area);
            frame.render_widget(block, area);
            
            let text = "Press 'q' to quit";
            frame.render_widget(text, inner);
        })?;

        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break Ok(());
            }
        }
    }
}
