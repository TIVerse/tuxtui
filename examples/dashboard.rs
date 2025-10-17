//! Dashboard example showcasing multiple widgets

use crossterm::event::{self, Event, KeyCode};
use std::time::{Duration, Instant};
use tuxtui::prelude::*;
use tuxtui::widgets::{
    block::{Block, BorderType},
    barchart::{Bar, BarChart},
    gauge::Gauge,
    list::{List, ListItem, ListState},
    sparkline::Sparkline,
    table::{Row, Table, TableState},
};

struct App {
    cpu_data: Vec<u64>,
    memory_percent: u16,
    network_rx: Vec<u64>,
    network_tx: Vec<u64>,
    processes: Vec<(String, String, String)>,
    list_state: ListState,
    table_state: TableState,
    tick_count: u64,
}

impl App {
    fn new() -> Self {
        let mut list_state = ListState::default();
        list_state.select(Some(0));
        
        Self {
            cpu_data: vec![10, 20, 30, 40, 30, 20, 25, 35, 45, 40],
            memory_percent: 65,
            network_rx: vec![5, 10, 8, 12, 15, 18, 20, 17, 14, 16],
            network_tx: vec![3, 7, 5, 9, 10, 12, 14, 11, 9, 10],
            processes: vec![
                ("nginx".to_string(), "1234".to_string(), "2.3%".to_string()),
                ("postgres".to_string(), "5678".to_string(), "5.1%".to_string()),
                ("redis".to_string(), "9012".to_string(), "1.8%".to_string()),
                ("node".to_string(), "3456".to_string(), "4.2%".to_string()),
            ],
            list_state,
            table_state: TableState::default(),
            tick_count: 0,
        }
    }

    fn on_tick(&mut self) {
        self.tick_count += 1;
        
        // Simulate CPU usage
        let new_cpu = ((self.tick_count * 7) % 100) as u64;
        self.cpu_data.push(new_cpu);
        if self.cpu_data.len() > 50 {
            self.cpu_data.remove(0);
        }

        // Simulate memory
        self.memory_percent = ((50 + (self.tick_count % 40)) % 100) as u16;

        // Simulate network
        let new_rx = ((self.tick_count * 3) % 25) as u64;
        let new_tx = ((self.tick_count * 2) % 20) as u64;
        
        self.network_rx.push(new_rx);
        self.network_tx.push(new_tx);
        
        if self.network_rx.len() > 30 {
            self.network_rx.remove(0);
            self.network_tx.remove(0);
        }
    }
}

fn main() -> std::io::Result<()> {
    let mut terminal = tuxtui::init()?;
    let mut app = App::new();
    let mut last_tick = Instant::now();
    let tick_rate = Duration::from_millis(500);

    let result = run(&mut terminal, &mut app, &mut last_tick, tick_rate);
    tuxtui::restore()?;
    result
}

fn run(
    terminal: &mut tuxtui::DefaultTerminal,
    app: &mut App,
    last_tick: &mut Instant,
    tick_rate: Duration,
) -> std::io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let area = frame.area();

            // Main layout
            let mut main_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),
                    Constraint::Fill(1),
                ]);

            let chunks = main_layout.split(area);

            // Title
            let title = Block::default()
                .title("System Dashboard (q: quit, ↑/↓: navigate)")
                .borders(BorderType::All)
                .style(Style::default().fg(Color::Cyan));
            frame.render_widget(title, chunks[0]);

            // Content area - split into top and bottom
            let mut content_layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)]);

            let content_chunks = content_layout.split(chunks[1]);

            // Top row - split into 3 columns
            let mut top_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(33),
                    Constraint::Percentage(33),
                    Constraint::Percentage(34),
                ]);

            let top_chunks = top_layout.split(content_chunks[0]);

            // CPU Chart
            let cpu_block = Block::default()
                .title("CPU Usage")
                .borders(BorderType::All);
            let cpu_inner = cpu_block.inner(top_chunks[0]);
            frame.render_widget(cpu_block, top_chunks[0]);

            let cpu_sparkline = Sparkline::default()
                .data(&app.cpu_data)
                .style(Style::default().fg(Color::Green));
            frame.render_widget(cpu_sparkline, cpu_inner);

            // Memory Gauge
            let mem_block = Block::default()
                .title("Memory")
                .borders(BorderType::All);
            let mem_inner = mem_block.inner(top_chunks[1]);
            frame.render_widget(mem_block, top_chunks[1]);

            let memory_gauge = Gauge::default()
                .percent(app.memory_percent)
                .label(format!("{}%", app.memory_percent))
                .gauge_style(Style::default().fg(Color::Yellow));
            frame.render_widget(memory_gauge, mem_inner);

            // Network BarChart
            let net_block = Block::default()
                .title("Network (RX/TX)")
                .borders(BorderType::All);
            let net_inner = net_block.inner(top_chunks[2]);
            frame.render_widget(net_block, top_chunks[2]);

            let bars = [
                Bar::new(*app.network_rx.last().unwrap_or(&0))
                    .label("RX")
                    .style(Style::default().fg(Color::Blue)),
                Bar::new(*app.network_tx.last().unwrap_or(&0))
                    .label("TX")
                    .style(Style::default().fg(Color::Magenta)),
            ];
            
            let barchart = BarChart::new()
                .data(&bars)
                .bar_width(5)
                .bar_gap(2);
            frame.render_widget(barchart, net_inner);

            // Bottom row - split into 2 columns
            let mut bottom_layout = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)]);

            let bottom_chunks = bottom_layout.split(content_chunks[1]);

            // Process Table
            let table_block = Block::default()
                .title("Top Processes")
                .borders(BorderType::All);
            let table_inner = table_block.inner(bottom_chunks[0]);
            frame.render_widget(table_block, bottom_chunks[0]);

            let rows: Vec<Row> = app
                .processes
                .iter()
                .map(|(name, pid, cpu)| {
                    Row::new(vec![name.as_str(), pid.as_str(), cpu.as_str()])
                })
                .collect();

            let table = Table::new(
                rows,
                [
                    Constraint::Fill(1),
                    Constraint::Length(8),
                    Constraint::Length(8),
                ],
            )
            .header(Row::new(vec!["Name", "PID", "CPU"]).style(Style::default().fg(Color::Yellow)));

            table.render_stateful(table_inner, frame.buffer_mut(), &mut app.table_state);

            // Activity Log
            let log_block = Block::default()
                .title("Activity Log")
                .borders(BorderType::All);
            let log_inner = log_block.inner(bottom_chunks[1]);
            frame.render_widget(log_block, bottom_chunks[1]);

            let log_items = vec![
                ListItem::new("System started"),
                ListItem::new("Services initialized"),
                ListItem::new("Network connected"),
                ListItem::new("All systems nominal"),
            ];

            let log_list = List::new(log_items)
                .highlight_style(Style::default().bg(Color::DarkGray));

            log_list.render_stateful(log_inner, frame.buffer_mut(), &mut app.list_state);
        })?;

        // Handle events
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break Ok(()),
                    KeyCode::Down => app.list_state.select_next(4),
                    KeyCode::Up => app.list_state.select_previous(4),
                    _ => {}
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            *last_tick = Instant::now();
        }
    }
}
