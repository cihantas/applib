//! Table component for displaying data in columns with keyboard navigation.
//!
//! This module provides a table component similar to SwiftUI's Table or NSTableView,
//! designed for displaying structured data with aligned columns, selection, and
//! keyboard navigation.
//!
//! # Example
//!
//! ```ignore
//! // Store scroll handle and focus handle in your view
//! struct MyView {
//!     commits: Vec<Commit>,
//!     selected: Entity<State<Option<usize>>>,
//!     scroll_handle: UniformListScrollHandle,
//!     focus_handle: FocusHandle,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let commits = self.commits.clone();
//!
//!         Table::new("commits", self.commits.len(), move |index, selected, _window, _cx| {
//!             let commit = &commits[index];
//!             vec![
//!                 div().child(commit.message.clone()).into_any_element(),
//!                 div().child(commit.author.clone()).into_any_element(),
//!                 div().child(commit.hash.clone()).into_any_element(),
//!             ]
//!         })
//!         .columns([
//!             TableColumn::flex(),
//!             TableColumn::fixed(px(150.0)),
//!             TableColumn::fixed(px(80.0)),
//!         ])
//!         .selection(State::binding(&self.selected, cx))
//!         .focusable(self.focus_handle.clone())
//!         .track_scroll(self.scroll_handle.clone())
//!         .on_confirm(|index, _window, _cx| {
//!             println!("Confirmed row {}", index);
//!         })
//!     }
//! }
//! ```

use gpui::prelude::*;
use gpui::*;
use std::cell::RefCell;
use std::rc::Rc;

use crate::state::Binding;

// Re-export scroll types for convenience
pub use gpui::ScrollStrategy;
pub use gpui::UniformListScrollHandle;

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

/// A table component for displaying data in aligned columns with keyboard navigation.
///
/// Table uses GPUI's `uniform_list` internally for virtualized rendering and
/// automatic scroll-to-selection support. Rows are rendered on-demand via
/// a callback function, which receives the row index and whether it's selected.
///
/// # Features
///
/// - **Virtualization**: Only visible rows are rendered for performance
/// - **Click selection**: Click any row to select it
/// - **Keyboard navigation**: Arrow keys navigate, Enter confirms
/// - **Scroll-to-selection**: Automatically scrolls to keep selection visible
/// - **Column layout**: Fixed and flexible column widths
///
/// # Example
///
/// ```ignore
/// Table::new("data", items.len(), |index, selected, window, cx| {
///     vec![
///         div().child(items[index].name.clone()).into_any_element(),
///         div().child(items[index].value.clone()).into_any_element(),
///     ]
/// })
/// .columns([TableColumn::flex(), TableColumn::fixed(px(100.0))])
/// .selection(selection_binding)
/// .on_confirm(|index, window, cx| { /* handle confirm */ })
/// ```
pub struct Table {
    id: ElementId,
    row_count: usize,
    render_cells: Rc<dyn Fn(usize, bool, &mut Window, &mut App) -> Vec<AnyElement>>,
    columns: Vec<TableColumn>,
    scroll_handle: Option<UniformListScrollHandle>,
    selection_binding: Option<Binding<Option<usize>>>,
    on_confirm: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    focus_handle: Option<FocusHandle>,
}

impl Table {
    /// Creates a new table with the given id, row count, and cell render function.
    ///
    /// The render function is called for each visible row and receives:
    /// - `index`: The row index (0-based)
    /// - `selected`: Whether this row is currently selected
    /// - `window`: The window context
    /// - `cx`: The app context
    ///
    /// The function should return a Vec of cell elements matching the column count.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Table::new("my-table", items.len(), |index, selected, window, cx| {
    ///     vec![
    ///         div().child(items[index].col1.clone()).into_any_element(),
    ///         div().child(items[index].col2.clone()).into_any_element(),
    ///     ]
    /// })
    /// ```
    pub fn new(
        id: impl Into<ElementId>,
        row_count: usize,
        render_cells: impl Fn(usize, bool, &mut Window, &mut App) -> Vec<AnyElement> + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            row_count,
            render_cells: Rc::new(render_cells),
            columns: Vec::new(),
            scroll_handle: None,
            selection_binding: None,
            on_confirm: None,
            focus_handle: None,
        }
    }

    /// Sets the column definitions for the table.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Table::new(...)
    ///     .columns([
    ///         TableColumn::flex(),           // First column grows
    ///         TableColumn::fixed(px(150.0)), // Fixed width
    ///         TableColumn::fixed(px(80.0)),  // Fixed width
    ///     ])
    /// ```
    pub fn columns(mut self, columns: impl IntoIterator<Item = TableColumn>) -> Self {
        self.columns = columns.into_iter().collect();
        self
    }

    /// Sets a two-way binding for the selected row index.
    ///
    /// When the user clicks a row or navigates with keyboard, the binding
    /// is automatically updated. The table also reads the current selection
    /// from the binding to highlight the selected row.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Table::new("items", count, render)
    ///     .selection(State::binding(&self.selected, cx))
    /// ```
    pub fn selection(mut self, binding: Binding<Option<usize>>) -> Self {
        self.selection_binding = Some(binding);
        self
    }

    /// Connects a scroll handle for scroll-to-selection support.
    ///
    /// When selection changes (via keyboard or click), the table automatically
    /// scrolls to keep the selected row visible.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Store in your view struct:
    /// scroll_handle: UniformListScrollHandle::new()
    ///
    /// // Pass to table:
    /// Table::new("items", count, render)
    ///     .track_scroll(self.scroll_handle.clone())
    /// ```
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }

    /// Sets the handler called when the user presses Enter on a selected row.
    pub fn on_confirm(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_confirm = Some(Rc::new(handler));
        self
    }

    /// Makes the table focusable and enables keyboard navigation.
    ///
    /// When focusable, the table responds to:
    /// - Arrow keys (Up/Down) to navigate between rows
    /// - Cmd+Up/Cmd+Down to jump to first/last row
    /// - Enter to trigger the on_confirm callback
    pub fn focusable(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }
}

impl IntoElement for Table {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id = self.id.clone();
        let row_count = self.row_count;
        let render_cells = self.render_cells.clone();
        let columns = Rc::new(self.columns);
        let selection_binding = self.selection_binding.clone();
        let on_confirm = self.on_confirm.clone();
        let scroll_handle = self.scroll_handle.clone();

        // Shared state for selection that can be accessed in closures
        let current_selection = Rc::new(RefCell::new(None::<usize>));
        let current_selection_for_render = current_selection.clone();
        let current_selection_for_keydown = current_selection.clone();

        // Clone for various closures
        let selection_binding_for_render = selection_binding.clone();
        let scroll_handle_for_render = scroll_handle.clone();
        let scroll_handle_for_keydown = scroll_handle.clone();
        let columns_for_render = columns.clone();

        // Create the uniform list with click-to-select rows
        let list_element = uniform_list(id.clone(), row_count, move |range, window, cx| {
            // Update current selection from binding
            let selected_index = if let Some(ref binding) = selection_binding_for_render {
                let sel = binding.get(cx);
                *current_selection_for_render.borrow_mut() = sel;
                sel
            } else {
                *current_selection_for_render.borrow()
            };

            range
                .map(|index| {
                    let is_selected = selected_index == Some(index);
                    let cells = render_cells(index, is_selected, window, cx);

                    // Build row with column layout
                    // Fixed height + truncated text = consistent row heights = reliable borders
                    let row_height = px(24.0);
                    let border_color = hsla(0.0, 0.0, 0.85, 1.0);

                    let mut row = div()
                        .id(("table-row", index))
                        .flex()
                        .flex_row()
                        .items_center()
                        .w_full()
                        .h(row_height)
                        .px(px(16.0))
                        .border_b_1()
                        .border_color(border_color)
                        .cursor_pointer();

                    // Apply selection styling
                    row = if is_selected {
                        row.bg(hsla(211.0 / 360.0, 0.95, 0.53, 1.0))
                    } else {
                        row.bg(hsla(0.0, 0.0, 1.0, 1.0))
                            .hover(|style| style.bg(hsla(0.0, 0.0, 0.98, 1.0)))
                    };

                    // Add cells with column widths
                    // Each cell is truncated with ellipsis (like SwiftUI's .lineLimit(1))
                    for (i, cell) in cells.into_iter().enumerate() {
                        let cell_container = match columns_for_render.get(i) {
                            Some(TableColumn::Fixed(width)) => div()
                                .w(*width)
                                .flex_shrink_0()
                                .truncate(),
                            Some(TableColumn::Flex) | None => div().flex_1().truncate(),
                        };
                        row = row.child(cell_container.child(cell));
                    }

                    // Add click handler for selection
                    let selection_binding_for_click = selection_binding_for_render.clone();
                    let scroll_handle_for_click = scroll_handle_for_render.clone();

                    // Use a bottom border on the container instead of a separate separator element
                    // This avoids sub-pixel rendering issues with thin separate divs
                    row.on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                        if let Some(ref binding) = selection_binding_for_click {
                            binding.set(Some(index), cx);
                        }
                        if let Some(ref handle) = scroll_handle_for_click {
                            handle.scroll_to_item(index, ScrollStrategy::Center);
                        }
                        window.refresh();
                    })
                    .into_any_element()
                })
                .collect()
        });

        // Apply scroll handle if provided
        let list_element = if let Some(handle) = scroll_handle.clone() {
            list_element.track_scroll(handle)
        } else {
            list_element
        };

        // Style the list
        let list_element = list_element.flex_1().w_full();

        // Create container
        let mut container = div()
            .id(id)
            .flex()
            .flex_col()
            .flex_1()
            .size_full()
            .bg(hsla(0.0, 0.0, 1.0, 1.0))
            .overflow_hidden();

        // Add keyboard navigation if focus handle is provided
        if let Some(handle) = self.focus_handle {
            let selection_binding_for_key = selection_binding.clone();

            container = container.track_focus(&handle).on_key_down(
                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                    // Get current selection
                    let current = if let Some(ref binding) = selection_binding_for_key {
                        binding.get(cx)
                    } else {
                        *current_selection_for_keydown.borrow()
                    };

                    let mut new_selection: Option<usize> = None;

                    match event.keystroke.key.as_str() {
                        "down" if !event.keystroke.modifiers.platform => {
                            if let Some(current) = current {
                                if current + 1 < row_count {
                                    new_selection = Some(current + 1);
                                }
                            } else if row_count > 0 {
                                new_selection = Some(0);
                            }
                        }
                        "up" if !event.keystroke.modifiers.platform => {
                            if let Some(current) = current {
                                if current > 0 {
                                    new_selection = Some(current - 1);
                                }
                            } else if row_count > 0 {
                                new_selection = Some(row_count - 1);
                            }
                        }
                        "down" if event.keystroke.modifiers.platform => {
                            if row_count > 0 {
                                new_selection = Some(row_count - 1);
                            }
                        }
                        "up" if event.keystroke.modifiers.platform => {
                            if row_count > 0 {
                                new_selection = Some(0);
                            }
                        }
                        "enter" => {
                            if let Some(current) = current {
                                if let Some(ref handler) = on_confirm {
                                    handler(current, window, cx);
                                }
                            }
                            return;
                        }
                        _ => return,
                    }

                    // Update selection and scroll
                    if let Some(new_idx) = new_selection {
                        if current != Some(new_idx) {
                            // Update binding
                            if let Some(ref binding) = selection_binding_for_key {
                                binding.set(Some(new_idx), cx);
                            }
                            *current_selection_for_keydown.borrow_mut() = Some(new_idx);

                            // Scroll to new selection
                            if let Some(ref handle) = scroll_handle_for_keydown {
                                handle.scroll_to_item(new_idx, ScrollStrategy::Center);
                            }

                            window.refresh();
                        }
                    }
                },
            );
        }

        container.child(list_element)
    }
}
