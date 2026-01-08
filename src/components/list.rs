//! List component for displaying scrollable rows of data.
//!
//! This module provides a scrollable list component with SwiftUI-like behavior:
//! - Automatic scroll-to-selection when navigating with keyboard
//! - Click-to-select items with mouse
//! - Keyboard navigation (↑/↓ arrows, Cmd+↑/↓ for jump)
//! - Enter triggers confirm callback
//! - Virtualized rendering for performance (only visible items are rendered)
//!
//! # Example
//!
//! ```ignore
//! // Store scroll handle in your view for scroll-to-selection
//! struct MyView {
//!     items: Vec<Item>,
//!     selected: Entity<State<Option<usize>>>,
//!     scroll_handle: UniformListScrollHandle,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let items = self.items.clone();
//!         let selected_index = self.selected.read(cx).get();
//!
//!         List::new("my-list", self.items.len(), move |index, selected, _window, _cx| {
//!             div()
//!                 .p(px(12.0))
//!                 .bg(if selected { hsla(0.6, 0.8, 0.9, 1.0) } else { white() })
//!                 .child(items[index].name.clone())
//!                 .into_any_element()
//!         })
//!         .track_scroll(self.scroll_handle.clone())
//!         .selection(State::binding(&self.selected, cx))
//!         .on_confirm(|index, _window, _cx| {
//!             println!("Confirmed item {}", index);
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
pub use gpui::UniformListScrollHandle;
pub use gpui::ScrollStrategy;

/// Selection mode for the list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SelectionMode {
    /// No selection allowed.
    None,
    /// Only one item can be selected at a time.
    #[default]
    Single,
    /// Multiple items can be selected.
    Multiple,
}

/// Visual style for the list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ListStyle {
    /// Plain style with full-width rows and minimal styling.
    #[default]
    Plain,
    /// Inset style with rounded corners and inset from edges.
    Inset,
    /// Sidebar style optimized for navigation sidebars.
    Sidebar,
}

/// A scrollable list component with SwiftUI-like behavior.
///
/// List uses GPUI's `uniform_list` internally for virtualized rendering and
/// automatic scroll-to-selection support. Items are rendered on-demand via
/// a callback function, which receives the item index and whether it's selected.
///
/// # Features
///
/// - **Virtualization**: Only visible items are rendered for performance
/// - **Click selection**: Click any item to select it
/// - **Keyboard navigation**: Arrow keys navigate, Enter confirms
/// - **Scroll-to-selection**: Automatically scrolls to keep selection visible
///
/// # Example
///
/// ```ignore
/// List::new("results", items.len(), |index, selected, window, cx| {
///     ResultRow::new(&items[index])
///         .selected(selected)
///         .into_any_element()
/// })
/// .selection(selection_binding)
/// .on_confirm(|index, window, cx| { /* launch app */ })
/// ```
pub struct List {
    id: ElementId,
    item_count: usize,
    render_item: Rc<dyn Fn(usize, bool, &mut Window, &mut App) -> AnyElement>,
    scroll_handle: Option<UniformListScrollHandle>,
    selection_binding: Option<Binding<Option<usize>>>,
    selection_mode: SelectionMode,
    on_confirm: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    focus_handle: Option<FocusHandle>,
    search_field: Option<AnyElement>,
    style: ListStyle,
}

impl List {
    /// Creates a new list with the given id, item count, and render function.
    ///
    /// The render function is called for each visible item and receives:
    /// - `index`: The item index (0-based)
    /// - `selected`: Whether this item is currently selected
    /// - `window`: The window context
    /// - `cx`: The app context
    ///
    /// # Example
    ///
    /// ```ignore
    /// List::new("my-list", items.len(), |index, selected, window, cx| {
    ///     div()
    ///         .child(items[index].name.clone())
    ///         .bg(if selected { blue() } else { white() })
    ///         .into_any_element()
    /// })
    /// ```
    pub fn new(
        id: impl Into<ElementId>,
        item_count: usize,
        render_item: impl Fn(usize, bool, &mut Window, &mut App) -> AnyElement + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            item_count,
            render_item: Rc::new(render_item),
            scroll_handle: None,
            selection_binding: None,
            selection_mode: SelectionMode::Single,
            on_confirm: None,
            focus_handle: None,
            search_field: None,
            style: ListStyle::default(),
        }
    }

    /// Sets a two-way binding for the selected index.
    ///
    /// When the user clicks an item or navigates with keyboard, the binding
    /// is automatically updated. The list also reads the current selection
    /// from the binding to highlight the selected item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// List::new("items", count, render)
    ///     .selection(State::binding(&self.selected, cx))
    /// ```
    pub fn selection(mut self, binding: Binding<Option<usize>>) -> Self {
        self.selection_binding = Some(binding);
        self
    }

    /// Sets the selection mode for the list.
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Connects a scroll handle for scroll-to-selection support.
    ///
    /// When selection changes (via keyboard or click), the list automatically
    /// scrolls to keep the selected item visible.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Store in your view struct:
    /// scroll_handle: UniformListScrollHandle::new()
    ///
    /// // Pass to list:
    /// List::new("items", count, render)
    ///     .track_scroll(self.scroll_handle.clone())
    /// ```
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }

    /// Sets the handler called when the user presses Enter on a selected item.
    pub fn on_confirm(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_confirm = Some(Rc::new(handler));
        self
    }

    /// Makes the list focusable and enables keyboard navigation.
    ///
    /// When focusable, the list responds to:
    /// - Arrow keys (↑/↓) to navigate between items
    /// - Cmd+↑/Cmd+↓ to jump to first/last item
    /// - Enter to trigger the on_confirm callback
    pub fn focusable(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
        self
    }

    /// Adds a search field element to the top of the list.
    ///
    /// The search field is rendered above the list items in a styled header.
    pub fn search_field(mut self, field: impl IntoElement) -> Self {
        self.search_field = Some(field.into_any_element());
        self
    }

    /// Sets the visual style of the list.
    pub fn style(mut self, style: ListStyle) -> Self {
        self.style = style;
        self
    }
}

impl IntoElement for List {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id = self.id.clone();
        let item_count = self.item_count;
        let render_item = self.render_item.clone();
        let selection_binding = self.selection_binding.clone();
        let selection_mode = self.selection_mode;
        let on_confirm = self.on_confirm.clone();
        let scroll_handle = self.scroll_handle.clone();

        // Shared state for selection that can be accessed in closures
        let current_selection = Rc::new(RefCell::new(None::<usize>));
        let current_selection_for_render = current_selection.clone();
        let current_selection_for_keydown = current_selection.clone();

        // Style-specific settings
        let (bg_color, corner_radius, inset) = match self.style {
            ListStyle::Plain => (hsla(0.0, 0.0, 1.0, 1.0), px(0.0), px(0.0)),
            ListStyle::Inset => (hsla(0.0, 0.0, 1.0, 1.0), px(8.0), px(16.0)),
            ListStyle::Sidebar => (hsla(0.0, 0.0, 0.97, 1.0), px(6.0), px(8.0)),
        };

        // Create the uniform list with click-to-select items
        let selection_binding_for_render = selection_binding.clone();
        let scroll_handle_for_render = scroll_handle.clone();
        let scroll_handle_for_keydown = scroll_handle.clone();

        let list_element = uniform_list(id.clone(), item_count, move |range, window, cx| {
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
                    let item_element = render_item(index, is_selected, window, cx);

                    // Wrap each item with click handler for selection
                    let selection_binding_for_click = selection_binding_for_render.clone();
                    let scroll_handle_for_click = scroll_handle_for_render.clone();

                    div()
                        .id(("list-item", index))
                        .w_full()
                        .cursor_pointer()
                        .child(item_element)
                        .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                            // Update selection on click
                            if let Some(ref binding) = selection_binding_for_click {
                                binding.set(Some(index), cx);
                            }
                            // Scroll to clicked item
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
            .bg(bg_color)
            .overflow_hidden(); // Clip children to rounded corners

        // Apply inset margins for non-plain styles
        if inset != px(0.0) {
            container = container.mx(inset);
        }

        // Apply rounded corners
        if corner_radius != px(0.0) {
            container = container.rounded(corner_radius);
        }

        // Add keyboard navigation if focus handle is provided
        if let Some(handle) = self.focus_handle {
            let selection_binding_for_key = selection_binding.clone();

            container = container.track_focus(&handle).on_key_down(
                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                    if selection_mode == SelectionMode::None {
                        return;
                    }

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
                                if current + 1 < item_count {
                                    new_selection = Some(current + 1);
                                }
                            } else if item_count > 0 {
                                new_selection = Some(0);
                            }
                        }
                        "up" if !event.keystroke.modifiers.platform => {
                            if let Some(current) = current {
                                if current > 0 {
                                    new_selection = Some(current - 1);
                                }
                            } else if item_count > 0 {
                                new_selection = Some(item_count - 1);
                            }
                        }
                        "down" if event.keystroke.modifiers.platform => {
                            if item_count > 0 {
                                new_selection = Some(item_count - 1);
                            }
                        }
                        "up" if event.keystroke.modifiers.platform => {
                            if item_count > 0 {
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

        // Build final layout
        if let Some(search_field) = self.search_field {
            // With search field: search header + list
            // Note: Apply top corner radius to search header to match parent's rounded corners
            // Use 12px as a sensible default that works well with common window corner radii
            let search_corner_radius = px(12.0);

            container
                .child(
                    div()
                        .p(px(12.0))
                        .border_b_1()
                        .border_color(hsla(0.0, 0.0, 0.90, 1.0))
                        .bg(hsla(0.0, 0.0, 0.98, 1.0))
                        .rounded_t(search_corner_radius) // Round top corners only
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap(px(8.0))
                                .child(
                                    div()
                                        .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                                        .text_sm()
                                        .child("🔍"),
                                )
                                .child(div().flex_1().child(search_field)),
                        ),
                )
                .child(list_element)
        } else {
            // Just the list
            container.child(list_element)
        }
    }
}

/// A section within a list for grouping related items.
///
/// Note: ListSection is designed for use with the older children-based List API.
/// For the new callback-based List, handle sections in your render callback.
pub struct ListSection {
    id: ElementId,
    header: Option<SharedString>,
    footer: Option<SharedString>,
    collapsed: bool,
    children: Vec<AnyElement>,
}

impl ListSection {
    /// Creates a new list section with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            header: None,
            footer: None,
            collapsed: false,
            children: Vec::new(),
        }
    }

    /// Sets the header text for this section.
    pub fn header(mut self, header: impl Into<SharedString>) -> Self {
        self.header = Some(header.into());
        self
    }

    /// Sets the footer text for this section.
    pub fn footer(mut self, footer: impl Into<SharedString>) -> Self {
        self.footer = Some(footer.into());
        self
    }

    /// Sets whether the section is collapsed.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Adds a child element to the section.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the section.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl IntoElement for ListSection {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let mut container = div().id(self.id).flex().flex_col().w_full();

        // Add header if present
        if let Some(header) = self.header {
            let header_element = div()
                .px(px(16.0))
                .py(px(8.0))
                .bg(hsla(0.0, 0.0, 0.97, 1.0))
                .border_b_1()
                .border_color(hsla(0.0, 0.0, 0.90, 1.0))
                .child(
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                        .child(header.to_uppercase()),
                );
            container = container.child(header_element);
        }

        // Add children if not collapsed
        if !self.collapsed {
            for child in self.children {
                container = container.child(child);
            }
        }

        // Add footer if present
        if let Some(footer) = self.footer {
            let footer_element = div()
                .px(px(16.0))
                .py(px(6.0))
                .bg(hsla(0.0, 0.0, 0.97, 1.0))
                .child(
                    div()
                        .text_size(px(11.0))
                        .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                        .child(footer),
                );
            container = container.child(footer_element);
        }

        container
    }
}
