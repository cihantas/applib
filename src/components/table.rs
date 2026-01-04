//! Table component for displaying data in columns.
//!
//! This module provides a table component similar to SwiftUI's Table or NSTableView,
//! designed for displaying structured data with aligned columns.

use crate::components::table_row::TableRow;
use gpui::prelude::*;
use gpui::*;

/// Column definition for table layout.
///
/// Defines how a column should be sized within the table.
#[derive(Clone)]
pub enum TableColumn {
    /// Column with fixed pixel width.
    Fixed(Pixels),
    /// Column that grows to fill available space.
    /// Multiple flex columns share space proportionally.
    Flex,
}

impl TableColumn {
    /// Creates a fixed-width column.
    pub fn fixed(width: Pixels) -> Self {
        Self::Fixed(width)
    }

    /// Creates a flexible column that grows to fill available space.
    pub fn flex() -> Self {
        Self::Flex
    }
}

/// A table component for displaying data in aligned columns.
///
/// Table manages column definitions and renders rows with consistent column widths.
/// It provides selection states and hover effects.
///
/// # Example
///
/// ```ignore
/// let mut table = Table::new("commits")
///     .column(TableColumn::flex())           // Message column (grows)
///     .column(TableColumn::fixed(px(150.0))) // Author column
///     .column(TableColumn::fixed(px(80.0)))  // Hash column
///     .column(TableColumn::fixed(px(120.0))); // Date column
///
/// for (index, commit) in commits.iter().enumerate() {
///     table = table.row(
///         TableRow::new(("commit", index))
///             .selected(selected_index == Some(index))
///             .on_click(cx.listener(move |this, _event, _window, cx| {
///                 this.selected_index = Some(index);
///                 cx.notify();
///             }))
///             .cell(div().child(commit.message.clone()))
///             .cell(div().child(commit.author.clone()))
///             .cell(div().child(commit.hash.clone()))
///             .cell(div().child(commit.date.clone()))
///     );
/// }
/// ```
pub struct Table {
    id: ElementId,
    columns: Vec<TableColumn>,
    rows: Vec<TableRow>,
}

impl Table {
    /// Creates a new table with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }

    /// Adds a column definition to the table.
    ///
    /// Columns are rendered in the order they are added.
    pub fn column(mut self, column: TableColumn) -> Self {
        self.columns.push(column);
        self
    }

    /// Adds a row to the table.
    ///
    /// The number of cells in the row should match the number of columns.
    pub fn row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
        self
    }
}

impl IntoElement for Table {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let columns = self.columns;

        let mut container = div()
            .id(self.id)
            .flex()
            .flex_col()
            .flex_1()
            .bg(hsla(0.0, 0.0, 1.0, 1.0));

        for row in self.rows {
            // Base row styling
            let base = div()
                .flex()
                .flex_row()
                .id(row.id)
                .px(px(16.0))
                .py(px(2.0))
                .min_h(px(22.0))
                .border_b_1()
                .border_color(if row.selected {
                    hsla(211.0 / 360.0, 0.95, 0.48, 1.0)
                } else {
                    hsla(0.0, 0.0, 0.90, 1.0)
                });

            // Apply selection styling
            let styled = if row.selected {
                base.bg(hsla(211.0 / 360.0, 0.95, 0.53, 1.0))
                    .cursor_pointer()
            } else {
                base.bg(hsla(0.0, 0.0, 1.0, 1.0))
                    .cursor_pointer()
                    .hover(|style| style.bg(hsla(0.0, 0.0, 0.98, 1.0)))
            };

            // Add cells with column widths
            let mut row_with_cells = styled;
            for (i, cell) in row.cells.into_iter().enumerate() {
                let cell_container = match columns.get(i) {
                    Some(TableColumn::Fixed(width)) => div()
                        .w(*width)
                        .flex_shrink_0()
                        .overflow_hidden(),
                    Some(TableColumn::Flex) | None => div()
                        .flex_1()
                        .overflow_hidden(),
                };
                row_with_cells = row_with_cells.child(cell_container.child(cell));
            }

            // Add click handler if provided
            let final_row = if let Some(handler) = row.on_click {
                row_with_cells.on_click(handler)
            } else {
                row_with_cells
            };

            container = container.child(final_row);
        }

        container
    }
}
