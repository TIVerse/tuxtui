//! Benchmarks for rendering performance

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::layout::{Constraint, Direction, Layout};
use tuxtui_core::style::{Color, Style};
use tuxtui_core::text::{Line, Span, Text};

fn bench_buffer_creation(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer");
    
    for size in [10, 50, 100, 200].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            b.iter(|| {
                Buffer::empty(Rect::new(0, 0, size, size))
            });
        });
    }
    
    group.finish();
}

fn bench_buffer_set_string(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_set_string");
    
    for len in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(len), len, |b, &len| {
            let mut buffer = Buffer::empty(Rect::new(0, 0, 100, 100));
            let text = "x".repeat(len);
            let style = Style::default().fg(Color::Blue);
            
            b.iter(|| {
                buffer.set_string(0, 0, black_box(&text), style);
            });
        });
    }
    
    group.finish();
}

fn bench_buffer_diff(c: &mut Criterion) {
    let mut group = c.benchmark_group("buffer_diff");
    
    for change_ratio in [0.1, 0.5, 0.9].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}%", (change_ratio * 100.0) as u32)),
            change_ratio,
            |b, &ratio| {
                let size = 80;
                let mut old_buffer = Buffer::empty(Rect::new(0, 0, size, 24));
                let mut new_buffer = Buffer::empty(Rect::new(0, 0, size, 24));
                
                // Fill with content
                for y in 0..24 {
                    old_buffer.set_string(0, y, &"a".repeat(size as usize), Style::default());
                    new_buffer.set_string(0, y, &"a".repeat(size as usize), Style::default());
                }
                
                // Change some cells
                let changes = (size as f32 * 24.0 * ratio) as usize;
                for i in 0..changes {
                    let x = i % size as usize;
                    let y = i / size as usize;
                    new_buffer.set(x as u16, y as u16, "b", Style::default());
                }
                
                b.iter(|| {
                    old_buffer.diff(black_box(&new_buffer))
                });
            },
        );
    }
    
    group.finish();
}

fn bench_layout_split(c: &mut Criterion) {
    let mut group = c.benchmark_group("layout");
    
    for constraint_count in [3, 5, 10, 20].iter() {
        group.bench_with_input(
            BenchmarkId::from_parameter(constraint_count),
            constraint_count,
            |b, &count| {
                let area = Rect::new(0, 0, 100, 50);
                let constraints: Vec<Constraint> = (0..count)
                    .map(|_| Constraint::Fill(1))
                    .collect();
                
                b.iter(|| {
                    let mut layout = Layout::default()
                        .direction(Direction::Vertical)
                        .constraints(constraints.clone());
                    black_box(layout.split(area))
                });
            },
        );
    }
    
    group.finish();
}

fn bench_text_width_calculation(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_width");
    
    for len in [10, 50, 100, 500].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(len), len, |b, &len| {
            let text = "x".repeat(len);
            let span = Span::raw(&text);
            
            b.iter(|| {
                black_box(span.width())
            });
        });
    }
    
    group.finish();
}

fn bench_text_composition(c: &mut Criterion) {
    c.bench_function("text_from_string", |b| {
        let content = "Line 1\nLine 2\nLine 3\nLine 4\nLine 5";
        
        b.iter(|| {
            Text::from(black_box(content))
        });
    });
    
    c.bench_function("text_from_lines", |b| {
        b.iter(|| {
            Text::from(vec![
                Line::from("Line 1"),
                Line::from("Line 2"),
                Line::from("Line 3"),
                Line::from("Line 4"),
                Line::from("Line 5"),
            ])
        });
    });
}

fn bench_style_merging(c: &mut Criterion) {
    c.bench_function("style_patch", |b| {
        let base = Style::default().fg(Color::Red).bg(Color::Black);
        let patch = Style::default().fg(Color::Blue);
        
        b.iter(|| {
            black_box(base).patch(black_box(patch))
        });
    });
}

criterion_group!(
    benches,
    bench_buffer_creation,
    bench_buffer_set_string,
    bench_buffer_diff,
    bench_layout_split,
    bench_text_width_calculation,
    bench_text_composition,
    bench_style_merging
);

criterion_main!(benches);
