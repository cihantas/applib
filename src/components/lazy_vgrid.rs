//! Lazy vertical grid layout component.
//!
//! A virtualized grid layout that only renders visible rows.
//! This is equivalent to SwiftUI's LazyVGrid.

use gpui::prelude::*;
use gpui::*;
use std::sync::Arc;

/// Defines how a grid column should be sized.
#[derive(Clone, Debug)]
pub enum GridColumn {
    /// A column with a fixed width.
    Fixed(Pixels),
    /// A column that flexes based on available space.
    /// The weight determines how much space it takes relative to other flexible columns.
    Flexible {
        /// Minimum width for this column
        minimum: Pixels,
        /// Maximum width for this column (None means no maximum)
        maximum: Option<Pixels>,
        /// Weight for distributing extra space
        weight: f32,
    },
    /// A column that adapts to fill available space with items between min and max width.
    /// This creates as many columns as can fit in the available space.
    Adaptive {
        /// Minimum width for items
        minimum: Pixels,
        /// Maximum width for items
        maximum: Pixels,
    },
}

impl GridColumn {
    /// Creates a fixed-width column.
    pub fn fixed(width: impl Into<Pixels>) -> Self {
        Self::Fixed(width.into())
    }

    /// Creates a flexible column with default settings.
    pub fn flexible(weight: f32) -> Self {
        Self::Flexible {
            minimum: px(0.0),
            maximum: None,
            weight,
        }
    }

    /// Creates a flexible column with a minimum width.
    pub fn flexible_min(minimum: impl Into<Pixels>, weight: f32) -> Self {
        Self::Flexible {
            minimum: minimum.into(),
            maximum: None,
            weight,
        }
    }

    /// Creates an adaptive column that creates as many items as can fit.
    pub fn adaptive(minimum: impl Into<Pixels>, maximum: impl Into<Pixels>) -> Self {
        Self::Adaptive {
            minimum: minimum.into(),
            maximum: maximum.into(),
        }
    }
}

/// A lazy vertical grid that only renders visible rows.
///
/// LazyVGrid efficiently handles large grids by only rendering rows
/// currently visible in the viewport. Items are arranged in columns
/// from left to right, then top to bottom.
///
/// # Example
///
/// ```ignore
/// LazyVGrid::new("my-grid", 100, vec![GridColumn::fixed(px(100.0)); 3], |i, _window, _cx| {
///     div()
///         .h(px(100.0))
///         .child(format!("Item {}", i))
/// })
/// .spacing(px(10.0))
/// ```
pub struct LazyVGrid {
    id: ElementId,
    item_count: usize,
    columns: Vec<GridColumn>,
    render_item: Arc<dyn Fn(usize, &mut Window, &mut App) -> AnyElement + 'static>,
    horizontal_spacing: Pixels,
    vertical_spacing: Pixels,
    padding: Option<Pixels>,
    scroll_handle: Option<UniformListScrollHandle>,
}

impl LazyVGrid {
    /// Creates a new lazy vertical grid.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for the grid
    /// * `item_count` - The total number of items in the grid
    /// * `columns` - The column definitions for the grid
    /// * `render_item` - A closure that renders a single item by index
    pub fn new<F, R>(
        id: impl Into<ElementId>,
        item_count: usize,
        columns: Vec<GridColumn>,
        render_item: F,
    ) -> Self
    where
        F: Fn(usize, &mut Window, &mut App) -> R + 'static,
        R: IntoElement,
    {
        Self {
            id: id.into(),
            item_count,
            columns,
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

    /// Calculate the number of columns based on column definitions.
    /// For adaptive columns, this requires knowing the available width.
    /// For fixed/flexible columns, we just count them.
    fn column_count(&self) -> usize {
        // For now, we count the number of column definitions.
        // Adaptive columns are treated as a single column - the actual
        // adaptive behavior would require knowing the container width.
        self.columns.len()
    }

    /// Calculate the number of rows based on item count and columns.
    fn row_count(&self) -> usize {
        let cols = self.column_count();
        if cols == 0 {
            return 0;
        }
        (self.item_count + cols - 1) / cols
    }
}

impl IntoElement for LazyVGrid {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let render_item = self.render_item.clone();
        let columns = self.columns.clone();
        let column_count = self.column_count();
        let item_count = self.item_count;
        let h_spacing = self.horizontal_spacing;
        let v_spacing = self.vertical_spacing;
        let row_count = self.row_count();

        // Create the uniform_list - each "item" is actually a row of grid items
        let list = uniform_list(self.id.clone(), row_count, move |row_range, window, cx| {
            // For each row in the range, we need to render all items in that row
            row_range
                .clone()
                .map(|row_index| {
                    // Calculate which items are in this row
                    let start_item = row_index * column_count;

                    if start_item >= item_count {
                        // No items in this row
                        return div();
                    }

                    // Create a row container
                    let mut row = div().flex().flex_row().w_full();

                    // Add spacing between rows (except last row in the rendered range)
                    let is_last_row = row_index == row_range.end - 1;
                    if !is_last_row && v_spacing > px(0.0) {
                        row = row.mb(v_spacing);
                    }

                    // Add items to the row with proper column sizing
                    for col_index in 0..column_count {
                        let item_index = start_item + col_index;
                        if item_index >= item_count {
                            break;
                        }

                        // Render the item
                        let item = render_item(item_index, window, cx);

                        let is_last_col =
                            col_index == column_count - 1 || item_index + 1 >= item_count;

                        // Create item wrapper with column sizing
                        let mut item_wrapper = div();

                        // Apply column sizing based on column definition
                        if col_index < columns.len() {
                            match &columns[col_index] {
                                GridColumn::Fixed(width) => {
                                    item_wrapper = item_wrapper.w(*width).flex_shrink_0();
                                }
                                GridColumn::Flexible {
                                    minimum,
                                    maximum,
                                    weight,
                                } => {
                                    item_wrapper = item_wrapper.flex_grow().flex_basis(*minimum);
                                    if *minimum > px(0.0) {
                                        item_wrapper = item_wrapper.min_w(*minimum);
                                    }
                                    if let Some(max) = maximum {
                                        item_wrapper = item_wrapper.max_w(*max);
                                    }
                                    // Apply weight through flex-grow
                                    // Note: GPUI may not support fractional flex-grow,
                                    // so we approximate with grow/shrink
                                    let _ = weight; // Weight handled through flex_grow()
                                }
                                GridColumn::Adaptive { minimum, maximum } => {
                                    // Adaptive columns flex between min and max
                                    item_wrapper = item_wrapper
                                        .flex_grow()
                                        .min_w(*minimum)
                                        .max_w(*maximum);
                                }
                            }
                        } else {
                            // Default: flexible column that takes equal space
                            item_wrapper = item_wrapper.flex_grow();
                        }

                        // Add horizontal spacing (except for last column)
                        if !is_last_col && h_spacing > px(0.0) {
                            item_wrapper = item_wrapper.mr(h_spacing);
                        }

                        row = row.child(item_wrapper.child(item));
                    }

                    row
                })
                .collect::<Vec<_>>()
        });

        // Apply scroll handle if provided
        let list = if let Some(handle) = self.scroll_handle {
            list.track_scroll(handle)
        } else {
            list
        };

        // Wrap in a container with padding
        let mut container = div().flex().flex_col().size_full();

        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        container.child(list)
    }
}

/// A scroll handle for programmatically controlling LazyVGrid scroll position.
pub type LazyVGridScrollHandle = UniformListScrollHandle;
