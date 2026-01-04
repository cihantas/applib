//! Table row component for displaying data in tabular format.
//!
//! This module provides a row component designed to work with the Table component.
//! Rows collect cell content which the Table component then renders with appropriate
//! column widths.

use gpui::*;

/// A table row component that holds cell content for tabular display.
///
/// TableRow is designed to be used with the Table component. It stores cell content
/// and styling information, while the Table component handles the actual layout
/// based on column definitions.
///
/// # Example
///
/// ```ignore
/// TableRow::new("row-1")
///     .selected(true)
///     .on_click(cx.listener(|this, _event, _window, cx| {
///         // Handle row selection
///     }))
///     .cell(div().child("Cell 1"))
///     .cell(div().child("Cell 2"))
///     .cell(div().child("Cell 3"))
/// ```
pub struct TableRow {
    pub(crate) id: ElementId,
    pub(crate) cells: Vec<AnyElement>,
    pub(crate) selected: bool,
    pub(crate) on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TableRow {
    /// Creates a new table row with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            cells: Vec::new(),
            selected: false,
            on_click: None,
        }
    }

    /// Sets whether the row is selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the click handler for this row.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Adds a cell to this row.
    ///
    /// Cells are rendered in the order they are added, with widths determined
    /// by the Table's column definitions.
    pub fn cell(mut self, cell: impl IntoElement) -> Self {
        self.cells.push(cell.into_any_element());
        self
    }
}
