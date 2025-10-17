//! Terminal management and frame orchestration.

use crate::backend::Backend;
use crate::buffer::Buffer;
use crate::geometry::Rect;

/// Options for configuring a terminal.
#[derive(Debug, Clone)]
pub struct TerminalOptions {
    /// Enable alternate screen
    pub alternate_screen: bool,
    /// Hide cursor during rendering
    pub hide_cursor: bool,
}

impl Default for TerminalOptions {
    fn default() -> Self {
        Self {
            alternate_screen: true,
            hide_cursor: true,
        }
    }
}

/// A terminal interface managing rendering and buffering.
///
/// The terminal orchestrates frame rendering, maintains double buffers,
/// and coordinates with the backend.
///
/// # Example
///
/// ```ignore
/// use tuxtui_core::terminal::Terminal;
/// use tuxtui_core::backend::TestBackend;
///
/// let backend = TestBackend::new(80, 24);
/// let mut terminal = Terminal::new(backend).unwrap();
/// terminal.draw(|frame| {
///     // Render widgets
/// }).unwrap();
/// ```
pub struct Terminal<B: Backend> {
    backend: B,
    buffers: [Buffer; 2],
    current: usize,
    hidden_cursor: bool,
}

impl<B: Backend> Terminal<B> {
    /// Create a new terminal with the given backend.
    pub fn new(backend: B) -> Result<Self, B::Error> {
        Self::with_options(backend, TerminalOptions::default())
    }

    /// Create a new terminal with options.
    pub fn with_options(mut backend: B, options: TerminalOptions) -> Result<Self, B::Error> {
        let size = backend.size()?;

        if options.alternate_screen {
            backend.enter_alternate_screen()?;
        }

        if options.hide_cursor {
            backend.hide_cursor()?;
        }

        backend.enable_raw_mode()?;
        backend.clear()?;
        backend.flush()?;

        Ok(Self {
            backend,
            buffers: [Buffer::empty(size), Buffer::empty(size)],
            current: 0,
            hidden_cursor: options.hide_cursor,
        })
    }

    /// Get the size of the terminal.
    pub fn size(&self) -> Result<Rect, B::Error> {
        self.backend.size()
    }

    /// Get the current viewport.
    #[must_use]
    pub fn viewport(&self) -> Rect {
        self.buffers[self.current].area
    }

    /// Clear the terminal.
    pub fn clear(&mut self) -> Result<(), B::Error> {
        self.backend.clear()?;
        self.buffers[self.current].clear();
        Ok(())
    }

    /// Draw a frame using the provided closure.
    ///
    /// # Example
    ///
    /// ```ignore
    /// terminal.draw(|frame| {
    ///     let area = frame.area();
    ///     frame.render_widget(widget, area);
    /// })?;
    /// ```
    pub fn draw<F>(&mut self, render: F) -> Result<(), B::Error>
    where
        F: FnOnce(&mut Frame<'_>),
    {
        // Check for resize
        let size = self.backend.size()?;
        if size != self.buffers[self.current].area {
            self.resize(size)?;
        }

        let next = (self.current + 1) % 2;
        self.buffers[next].clear();

        // Render to next buffer
        let mut frame = Frame {
            buffer: &mut self.buffers[next],
            area: size,
        };
        render(&mut frame);

        // Compute diff and render
        let diff = self.buffers[self.current].diff(&self.buffers[next]);
        for change in diff {
            for cell in change.cells {
                self.backend.draw_cell(change.x, change.y, cell)?;
            }
        }

        self.backend.flush()?;
        self.current = next;

        Ok(())
    }

    /// Resize the terminal buffers.
    fn resize(&mut self, size: Rect) -> Result<(), B::Error> {
        self.buffers[0].resize(size);
        self.buffers[1].resize(size);
        self.backend.clear()?;
        Ok(())
    }

    /// Show the cursor.
    pub fn show_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.show_cursor()?;
        self.hidden_cursor = false;
        Ok(())
    }

    /// Hide the cursor.
    pub fn hide_cursor(&mut self) -> Result<(), B::Error> {
        self.backend.hide_cursor()?;
        self.hidden_cursor = true;
        Ok(())
    }

    /// Set the cursor position.
    pub fn set_cursor(&mut self, x: u16, y: u16) -> Result<(), B::Error> {
        self.backend.set_cursor(x, y)
    }

    /// Get mutable access to the backend.
    pub fn backend_mut(&mut self) -> &mut B {
        &mut self.backend
    }

    /// Flush the backend.
    pub fn flush(&mut self) -> Result<(), B::Error> {
        self.backend.flush()
    }
}

impl<B: Backend> Drop for Terminal<B> {
    fn drop(&mut self) {
        let _ = self.backend.disable_raw_mode();
        let _ = self.backend.leave_alternate_screen();
        if self.hidden_cursor {
            let _ = self.backend.show_cursor();
        }
        let _ = self.backend.flush();
    }
}

/// A frame for rendering widgets.
///
/// Frames provide access to a buffer and the rendering area during a draw call.
pub struct Frame<'a> {
    buffer: &'a mut Buffer,
    area: Rect,
}

impl<'a> Frame<'a> {
    /// Get the rendering area.
    #[must_use]
    pub const fn area(&self) -> Rect {
        self.area
    }

    /// Get mutable access to the buffer.
    pub fn buffer_mut(&mut self) -> &mut Buffer {
        self.buffer
    }

    /// Render a widget at the given area.
    pub fn render_widget<W>(&mut self, widget: W, area: Rect)
    where
        W: Widget,
    {
        widget.render(area, self.buffer);
    }
}

/// A widget that can be rendered to a buffer.
pub trait Widget {
    /// Render this widget into the given area of the buffer.
    fn render(self, area: Rect, buf: &mut Buffer);
}

/// Implement Widget for string slices for convenience.
impl Widget for &str {
    fn render(self, area: Rect, buf: &mut Buffer) {
        if area.height > 0 {
            buf.set_string(area.x, area.y, self, crate::style::Style::default());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::backend::TestBackend;

    #[test]
    fn test_terminal_creation() {
        let backend = TestBackend::new(80, 24);
        let terminal = Terminal::new(backend);
        assert!(terminal.is_ok());
    }

    #[test]
    fn test_terminal_draw() {
        let backend = TestBackend::new(80, 24);
        let mut terminal = Terminal::new(backend).unwrap();

        let result = terminal.draw(|frame| {
            let area = frame.area();
            assert_eq!(area.width, 80);
            assert_eq!(area.height, 24);
        });

        assert!(result.is_ok());
    }
}
