//! Integration tests for tuxtui-core.

#[cfg(test)]
mod integration {
    use crate::prelude::*;

    #[test]
    fn test_basic_rendering_pipeline() {
        let mut backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        terminal
            .draw(|frame| {
                let area = frame.area();
                frame.render_widget("Hello, tuxtui!", area);
            })
            .unwrap();
    }

    #[test]
    fn test_layout_and_buffer() {
        let area = Rect::new(0, 0, 100, 50);
        let mut layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(10),
                Constraint::Fill(1),
                Constraint::Length(5),
            ]);

        let chunks = layout.split(area);
        assert_eq!(chunks.len(), 3);

        let mut buffer = Buffer::empty(area);
        buffer.set_string(0, 0, "Test", Style::default().fg(Color::Blue));

        let cell = buffer.get(0, 0).unwrap();
        assert_eq!(cell.symbol, "T");
    }

    #[test]
    fn test_text_composition() {
        let text = Text::from(vec![
            Line::from("First line"),
            Line::styled("Second line", Style::default().fg(Color::Red)),
        ]);

        assert_eq!(text.height(), 2);
    }

    #[test]
    fn test_theme_application() {
        let theme = Theme::dark();
        assert_eq!(theme.palette.background, Color::Black);
        assert_eq!(theme.palette.foreground, Color::White);
    }
}
