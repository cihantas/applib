//! Lazy horizontal grid layout component.
//!
//! A virtualized grid layout that only renders visible columns.
//! This is equivalent to SwiftUI's LazyHGrid.

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

use super::lazy_vgrid::GridColumn;

/// Defines how a grid row should be sized.
/// This is the same as GridColumn but semantically represents rows for LazyHGrid.
pub type GridRow = GridColumn;

/// A lazy horizontal grid that only renders visible columns.
///
/// LazyHGrid efficiently handles large grids by only rendering columns
/// currently visible in the viewport. Items are arranged in rows
/// from top to bottom, then left to right.
///
/// # Example
///
/// ```ignore
/// LazyHGrid::new("my-hgrid", 100, vec![GridRow::fixed(px(50.0)); 3], |i, _window, _cx| {
///     div()
///         .w(px(100.0))
///         .child(format!("Item {}", i))
/// })
/// .spacing(px(10.0))
/// ```
pub struct LazyHGrid {
    id: ElementId,
    item_count: usize,
    rows: Vec<GridRow>,
    render_item: Arc<dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static>,
    horizontal_spacing: Pixels,
    vertical_spacing: Pixels,
    padding: Option<Pixels>,
    scroll_handle: Option<UniformListScrollHandle>,
}

impl LazyHGrid {
    /// Creates a new lazy horizontal grid.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for the grid
    /// * `item_count` - The total number of items in the grid
    /// * `rows` - The row definitions for the grid
    /// * `render_item` - A closure that renders a single item by index
    pub fn new<F, R>(
        id: impl Into<ElementId>,
        item_count: usize,
        rows: Vec<GridRow>,
        render_item: F,
    ) -> Self
    where
        F: Fn(usize, &mut Window, &mut App) -> R + 'static,
        R: IntoElement,
    {
        Self {
            id: id.into(),
            item_count,
            rows,
            render_item: Arc::new(move |index, window, cx| {
                render_item(index, window, cx).into_any_element()
            }),
            horizontal_spacing: px(0.0),
            vertical_spacing: px(0.0),
            padding: None,
            scroll_handle: None,
        }
    }

    /// Sets both horizontal and vertical spacing.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        let spacing = spacing.into();
        self.horizontal_spacing = spacing;
        self.vertical_spacing = spacing;
        self
    }

    /// Sets the horizontal spacing between columns.
    pub fn horizontal_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.horizontal_spacing = spacing.into();
        self
    }

    /// Sets the vertical spacing between rows.
    pub fn vertical_spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.vertical_spacing = spacing.into();
        self
    }

    /// Sets spacing to 0.75rem (12px).
    pub fn spacing_3(mut self) -> Self {
        self.horizontal_spacing = px(12.0);
        self.vertical_spacing = px(12.0);
        self
    }

    /// Sets spacing to 1.5rem (24px).
    pub fn spacing_6(mut self) -> Self {
        self.horizontal_spacing = px(24.0);
        self.vertical_spacing = px(24.0);
        self
    }

    /// Sets padding around all children to 0.25rem (4px).
    pub fn p_1(mut self) -> Self {
        self.padding = Some(px(4.0));
        self
    }

    /// Sets padding around all children to 0.5rem (8px).
    pub fn p_2(mut self) -> Self {
        self.padding = Some(px(8.0));
        self
    }

    /// Sets padding around all children to 0.75rem (12px).
    pub fn p_3(mut self) -> Self {
        self.padding = Some(px(12.0));
        self
    }

    /// Sets padding around all children to 1rem (16px).
    pub fn p_4(mut self) -> Self {
        self.padding = Some(px(16.0));
        self
    }

    /// Sets custom padding around all children.
    pub fn p(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Sets a scroll handle to programmatically control scroll position.
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }

    /// Calculate the number of rows based on row definitions.
    fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// Calculate the number of columns based on item count and rows.
    fn column_count(&self) -> usize {
        let rows = self.row_count();
        if rows == 0 {
            return 0;
        }
        (self.item_count + rows - 1) / rows
    }
}

impl IntoElement for LazyHGrid {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let render_item = self.render_item.clone();
        let rows_config = self.rows.clone();
        let row_count = self.row_count();
        let item_count = self.item_count;
        let h_spacing = self.horizontal_spacing;
        let v_spacing = self.vertical_spacing;
        let column_count = self.column_count();

        // Create the uniform_list - each "item" is actually a column of grid items
        let list = uniform_list(
            self.id.clone(),
            column_count,
            move |col_range, window, cx| {
                // For each column in the range, we need to render all items in that column
                col_range
                    .clone()
                    .map(|col_index| {
                        // Create a column container (vertical stack)
                        let mut column = div().flex().flex_col().flex_shrink_0();

                        // Add horizontal spacing between columns (except last column in range)
                        let is_last_col = col_index == col_range.end - 1;
                        if !is_last_col && h_spacing > px(0.0) {
                            column = column.mr(h_spacing);
                        }

                        // Add items for each row in this column
                        for row_index in 0..row_count {
                            // In LazyHGrid, items flow top-to-bottom, then left-to-right
                            // So item at (col, row) has index: col * row_count + row
                            let item_index = col_index * row_count + row_index;
                            if item_index >= item_count {
                                break;
                            }

                            // Render the item
                            let item = render_item(item_index, window, cx);

                            let is_last_row =
                                row_index == row_count - 1 || item_index + 1 >= item_count;

                            // Create item wrapper with row sizing
                            let mut item_wrapper = div();

                            // Apply row sizing based on row definition
                            if row_index < rows_config.len() {
                                match &rows_config[row_index] {
                                    GridRow::Fixed(height) => {
                                        item_wrapper = item_wrapper.h(*height).flex_shrink_0();
                                    }
                                    GridRow::Flexible {
                                        minimum, maximum, ..
                                    } => {
                                        item_wrapper = item_wrapper.flex_grow().flex_basis(*minimum);
                                        if *minimum > px(0.0) {
                                            item_wrapper = item_wrapper.min_h(*minimum);
                                        }
                                        if let Some(max) = maximum {
                                            item_wrapper = item_wrapper.max_h(*max);
                                        }
                                    }
                                    GridRow::Adaptive { minimum, maximum } => {
                                        item_wrapper = item_wrapper
                                            .flex_grow()
                                            .min_h(*minimum)
                                            .max_h(*maximum);
                                    }
                                }
                            } else {
                                // Default: flexible row that takes equal space
                                item_wrapper = item_wrapper.flex_grow();
                            }

                            // Add vertical spacing (except for last row)
                            if !is_last_row && v_spacing > px(0.0) {
                                item_wrapper = item_wrapper.mb(v_spacing);
                            }

                            column = column.child(item_wrapper.child(item));
                        }

                        column
                    })
                    .collect::<Vec<_>>()
            },
        );

        // Apply scroll handle if provided
        let list = if let Some(handle) = self.scroll_handle {
            list.track_scroll(handle)
        } else {
            list
        };

        // Wrap in a container with padding and horizontal scroll
        let mut container = div()
            .id((self.id, "container"))
            .flex()
            .flex_row()
            .size_full()
            .overflow_x_scroll();

        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        container.child(list)
    }
}

/// A scroll handle for programmatically controlling LazyHGrid scroll position.
pub type LazyHGridScrollHandle = UniformListScrollHandle;
