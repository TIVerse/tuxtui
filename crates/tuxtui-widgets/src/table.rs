//! Table widget for rendering tabular data.

use alloc::vec::Vec;
use tuxtui_core::buffer::Buffer;
use tuxtui_core::geometry::Rect;
use tuxtui_core::layout::Constraint;
use tuxtui_core::style::{Style, Stylize};
use tuxtui_core::terminal::Widget;
use tuxtui_core::text::Line;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// A table row.
///
/// # Example
///
/// ```
/// use tuxtui_widgets::table::Row;
///
/// let row = Row::new(vec!["Cell 1", "Cell 2", "Cell 3"]);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Row<'a> {
    cells: Vec<Line<'a>>,
    style: Style,
    height: u16,
}

impl<'a> Row<'a> {
    /// Create a new row.
    #[must_use]
    pub fn new<T>(cells: T) -> Self
    where
        T: IntoIterator,
        T::Item: Into<Line<'a>>,
    {
        Self {
            cells: cells.into_iter().map(Into::into).collect(),
            style: Style::default(),
            height: 1,
        }
    }

    /// Set the style for this row.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the height for this row.
    #[must_use]
    pub const fn height(mut self, height: u16) -> Self {
        self.height = height;
        self
    }

    /// Get the cells in this row.
    #[must_use]
    pub fn cells(&self) -> &[Line<'a>] {
        &self.cells
    }
}

impl<'a> Stylize for Row<'a> {
    fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }
}

/// Table state for tracking selection.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct TableState {
    selected: Option<usize>,
    offset: usize,
}

impl TableState {
    /// Create a new table state.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            selected: None,
            offset: 0,
        }
    }

    /// Get the selected row index.
    #[must_use]
    pub const fn selected(&self) -> Option<usize> {
        self.selected
    }

    /// Select a row.
    pub fn select(&mut self, index: Option<usize>) {
        self.selected = index;
    }

    /// Select the next row.
    pub fn select_next(&mut self, rows_len: usize) {
        if rows_len == 0 {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => (i + 1) % rows_len,
            None => 0,
        });
    }

    /// Select the previous row.
    pub fn select_previous(&mut self, rows_len: usize) {
        if rows_len == 0 {
            return;
        }
        self.selected = Some(match self.selected {
            Some(i) => {
                if i == 0 {
                    rows_len - 1
                } else {
                    i - 1
                }
            }
            None => rows_len - 1,
        });
    }

    /// Get the scroll offset.
    #[must_use]
    pub const fn offset(&self) -> usize {
        self.offset
    }

    /// Set the scroll offset.
    pub fn set_offset(&mut self, offset: usize) {
        self.offset = offset;
    }
}

/// A table widget.
///
/// Tables display data in rows and columns with optional headers.
///
/// # Example
///
/// ```
/// use tuxtui_core::prelude::*;
/// use tuxtui_widgets::table::{Table, Row};
///
/// let rows = vec![
///     Row::new(vec!["Row1", "Data1", "Value1"]),
///     Row::new(vec!["Row2", "Data2", "Value2"]),
/// ];
///
/// let table = Table::new(rows, [Constraint::Length(10), Constraint::Fill(1), Constraint::Length(10)])
///     .header(Row::new(vec!["Name", "Data", "Value"]));
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Table<'a> {
    rows: Vec<Row<'a>>,
    widths: Vec<Constraint>,
    header: Option<Row<'a>>,
    style: Style,
    highlight_style: Style,
    column_spacing: u16,
}

impl<'a> Table<'a> {
    /// Create a new table.
    #[must_use]
    pub fn new<R, C>(rows: R, widths: C) -> Self
    where
        R: IntoIterator,
        R::Item: Into<Row<'a>>,
        C: IntoIterator,
        C::Item: Into<Constraint>,
    {
        Self {
            rows: rows.into_iter().map(Into::into).collect(),
            widths: widths.into_iter().map(Into::into).collect(),
            header: None,
            style: Style::default(),
            highlight_style: Style::default(),
            column_spacing: 1,
        }
    }

    /// Set the table header.
    #[must_use]
    pub fn header(mut self, header: Row<'a>) -> Self {
        self.header = Some(header);
        self
    }

    /// Set the overall style.
    #[must_use]
    pub const fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Set the highlight style for selected rows.
    #[must_use]
    pub const fn highlight_style(mut self, style: Style) -> Self {
        self.highlight_style = style;
        self
    }

    /// Set the column spacing.
    #[must_use]
    pub const fn column_spacing(mut self, spacing: u16) -> Self {
        self.column_spacing = spacing;
        self
    }

    fn calculate_column_widths(&self, available_width: u16) -> Vec<u16> {
        let spacing_total = self
            .column_spacing
            .saturating_mul(self.widths.len().saturating_sub(1) as u16);
        let available = available_width.saturating_sub(spacing_total);

        let mut widths = Vec::with_capacity(self.widths.len());
        let mut fixed_width = 0u16;
        let mut fill_count = 0u32;

        for constraint in &self.widths {
            match constraint {
                Constraint::Length(len) => {
                    widths.push(*len);
                    fixed_width = fixed_width.saturating_add(*len);
                }
                Constraint::Fill(weight) => {
                    widths.push(0);
                    fill_count += *weight as u32;
                }
                _ => {
                    let width = constraint.apply(available);
                    widths.push(width);
                    fixed_width = fixed_width.saturating_add(width);
                }
            }
        }

        let remaining = available.saturating_sub(fixed_width);
        if fill_count > 0 {
            for (i, constraint) in self.widths.iter().enumerate() {
                if let Constraint::Fill(weight) = constraint {
                    widths[i] = ((remaining as u32 * *weight as u32) / fill_count) as u16;
                }
            }
        }

        widths
    }

    /// Render the table with state.
    pub fn render_stateful(self, area: Rect, buf: &mut Buffer, state: &mut TableState) {
        if area.area() == 0 {
            return;
        }

        let widths = self.calculate_column_widths(area.width);
        let mut y = area.top();

        // Render header
        if let Some(header) = &self.header {
            if y < area.bottom() {
                self.render_row(
                    &header.cells,
                    &widths,
                    area.left(),
                    y,
                    area.right(),
                    self.style.patch(header.style),
                    buf,
                );
                y += header.height;
            }
        }

        // Adjust offset
        if let Some(selected) = state.selected() {
            if selected < state.offset {
                state.offset = selected;
            }
        }

        // Render rows
        let visible_rows = &self.rows[state.offset.min(self.rows.len())..];
        for (i, row) in visible_rows.iter().enumerate() {
            if y >= area.bottom() {
                break;
            }

            let row_index = state.offset + i;
            let is_selected = state.selected() == Some(row_index);
            let row_style = if is_selected {
                self.style.patch(self.highlight_style).patch(row.style)
            } else {
                self.style.patch(row.style)
            };

            self.render_row(
                &row.cells,
                &widths,
                area.left(),
                y,
                area.right(),
                row_style,
                buf,
            );
            y += row.height;
        }
    }

    fn render_row(
        &self,
        cells: &[Line<'a>],
        widths: &[u16],
        left: u16,
        y: u16,
        right: u16,
        style: Style,
        buf: &mut Buffer,
    ) {
        let mut x = left;
        for (cell, &width) in cells.iter().zip(widths.iter()) {
            if x >= right {
                break;
            }

            let cell_width = width.min(right.saturating_sub(x));
            let mut cell_x = x;

            for span in &cell.spans {
                let span_style = style.patch(span.style);
                cell_x = buf.set_string(cell_x, y, &span.content, span_style);
                if cell_x >= x + cell_width {
                    break;
                }
            }

            x = x.saturating_add(width).saturating_add(self.column_spacing);
        }
    }
}

impl Widget for Table<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = TableState::default();
        self.render_stateful(area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn test_table_creation() {
        let rows = vec![Row::new(vec!["A", "B"]), Row::new(vec!["C", "D"])];
        let table = Table::new(rows, [Constraint::Fill(1), Constraint::Fill(1)]);
        assert_eq!(table.rows.len(), 2);
    }

    #[test]
    fn test_table_state() {
        let mut state = TableState::default();
        state.select(Some(0));
        assert_eq!(state.selected(), Some(0));

        state.select_next(3);
        assert_eq!(state.selected(), Some(1));
    }
}
