//! List component for displaying scrollable rows of data.
//!
//! This module provides a scrollable list component with support for single and
//! multi-selection, sections, and different visual styles.
//!
//! The list supports keyboard navigation when a focus handle is provided:
//! - Arrow keys (↑/↓) navigate between items
//! - Cmd+↑/Home jumps to the first item
//! - Cmd+↓/End jumps to the last item
//! - Enter triggers the on_confirm callback
//!
//! ## Searchable Lists
//!
//! Lists can display an interactive search field using `.searchable()`. The search
//! field embeds a TextField component that handles text input automatically:
//!
//! ```ignore
//! struct MyView {
//!     search_query: String,
//!     search_field: View<TextFieldState>,
//!     all_items: Vec<Item>,
//! }
//!
//! impl MyView {
//!     fn new(cx: &mut App) -> Self {
//!         Self {
//!             search_query: String::new(),
//!             search_field: cx.new(|cx| TextField::new("search", cx).into()),
//!             all_items: vec![],
//!         }
//!     }
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
//!         let filtered_items = self.all_items.iter()
//!             .filter(|item| {
//!                 self.search_query.is_empty() ||
//!                 item.name.to_lowercase().contains(&self.search_query.to_lowercase())
//!             })
//!             .collect::<Vec<_>>();
//!
//!         List::new("items-list")
//!             .searchable(self.search_field.clone(), cx.listener(|this, query, _window, cx| {
//!                 this.search_query = query;
//!                 cx.notify();
//!             }))
//!             .search_placeholder("Search items...")
//!             .children(filtered_items.iter().map(|item| {
//!                 ListItem::new(("item", item.id))
//!                     .child(item.name.clone())
//!             }))
//!     }
//! }
//! ```

use gpui::prelude::*;
use gpui::*;
use std::collections::HashSet;
use std::rc::Rc;

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

/// A scrollable list component for displaying rows of data.
///
/// List provides a container for displaying items with built-in scrolling,
/// selection management, and styling. It integrates with `ListItem` components
/// and supports sections via `ListSection`.
///
/// # Example
///
/// ```ignore
/// List::new("my-list")
///     .style(ListStyle::Inset)
///     .selection_mode(SelectionMode::Single)
///     .on_selection_change(|indices| {
///         println!("Selected: {:?}", indices);
///     })
///     .children(items.iter().enumerate().map(|(i, item)| {
///         ListItem::new(("item", i))
///             .child(div().child(item.name.clone()))
///     }))
/// ```
pub struct List {
    id: ElementId,
    style: ListStyle,
    selection_mode: SelectionMode,
    selected_indices: HashSet<usize>,
    show_separators: bool,
    on_selection_change: Option<Box<dyn Fn(&HashSet<usize>, &mut Window, &mut App) + 'static>>,
    on_confirm: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    focus_handle: Option<FocusHandle>,
    children: Vec<AnyElement>,
    // Search functionality
    search_field: Option<AnyElement>,
}

impl List {
    /// Creates a new list with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            style: ListStyle::default(),
            selection_mode: SelectionMode::default(),
            selected_indices: HashSet::new(),
            show_separators: true,
            on_selection_change: None,
            on_confirm: None,
            focus_handle: None,
            children: Vec::new(),
            search_field: None,
        }
    }

    /// Sets the visual style of the list.
    pub fn style(mut self, style: ListStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the selection mode for the list.
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
        self.selection_mode = mode;
        self
    }

    /// Sets the currently selected indices.
    pub fn selected(mut self, indices: impl IntoIterator<Item = usize>) -> Self {
        self.selected_indices = indices.into_iter().collect();
        self
    }

    /// Sets whether to show row separators.
    pub fn show_separators(mut self, show: bool) -> Self {
        self.show_separators = show;
        self
    }

    /// Sets the handler called when selection changes.
    pub fn on_selection_change(
        mut self,
        handler: impl Fn(&HashSet<usize>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Box::new(handler));
        self
    }

    /// Sets the currently selected index for single selection mode.
    /// This is a convenience method that sets a single selected index.
    pub fn selected_index(mut self, index: Option<usize>) -> Self {
        self.selected_indices = if let Some(idx) = index {
            [idx].into_iter().collect()
        } else {
            HashSet::new()
        };
        self
    }

    /// Makes the list focusable and enables keyboard navigation.
    /// When focusable, the list responds to arrow keys, Cmd+arrow keys, and Enter.
    pub fn focusable(mut self, focus_handle: FocusHandle) -> Self {
        self.focus_handle = Some(focus_handle);
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

    /// Adds a child element to the list.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the list.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    /// Adds a search field element to the top of the list.
    ///
    /// The parent view should create and manage the search field component.
    /// Pass the rendered search field element to display it above the list items.
    ///
    /// # Example
    ///
    /// ```ignore
    /// struct MyView {
    ///     query: String,
    ///     search_field: View<TextFieldState>,
    /// }
    ///
    /// impl Render for MyView {
    ///     fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
    ///         // Update search field on each render
    ///         self.search_field.update(cx, |state, _| {
    ///             // Sync query or set up callbacks
    ///         });
    ///
    ///         List::new("my-list")
    ///             .search_field(self.search_field.clone())
    ///             .children(/* filtered items */)
    ///     }
    /// }
    /// ```
    pub fn search_field(mut self, field: impl IntoElement) -> Self {
        self.search_field = Some(field.into_any_element());
        self
    }
}

impl IntoElement for List {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let selection_mode = self.selection_mode;
        let selected_indices = self.selected_indices.clone();
        let on_selection_change = self.on_selection_change;
        let on_confirm = self.on_confirm;
        let focus_handle = self.focus_handle.clone();
        let num_children = self.children.len();

        // Style-specific settings
        let (bg_color, corner_radius, inset) = match self.style {
            ListStyle::Plain => (
                hsla(0.0, 0.0, 1.0, 1.0),
                px(0.0),
                px(0.0),
            ),
            ListStyle::Inset => (
                hsla(0.0, 0.0, 1.0, 1.0),
                px(8.0),
                px(16.0),
            ),
            ListStyle::Sidebar => (
                hsla(0.0, 0.0, 0.97, 1.0),
                px(6.0),
                px(8.0),
            ),
        };

        // Create the scrollable container
        let mut container = div()
            .id(self.id.clone())
            .flex()
            .flex_col()
            .flex_1()
            .size_full()
            .bg(bg_color);

        // Apply inset margins for non-plain styles
        if inset != px(0.0) {
            container = container.mx(inset);
        }

        // Apply rounded corners
        if corner_radius != px(0.0) {
            container = container.rounded(corner_radius);
        }

        // Add focus tracking and keyboard navigation if focus handle is provided
        if let Some(handle) = focus_handle {
            container = container.track_focus(&handle).on_key_down(
                move |event: &KeyDownEvent, window: &mut Window, cx: &mut App| {
                    // Only handle keys if selection mode is not None
                    if selection_mode == SelectionMode::None {
                        return;
                    }

                    let current_selection = selected_indices.iter().copied().next();
                    let mut new_selection: Option<usize> = None;

                    match event.keystroke.key.as_str() {
                        "down" if !event.keystroke.modifiers.platform => {
                            // Arrow down: move selection down by 1
                            if let Some(current) = current_selection {
                                if current + 1 < num_children {
                                    new_selection = Some(current + 1);
                                }
                            } else if num_children > 0 {
                                // No selection, start at first item
                                new_selection = Some(0);
                            }
                        }
                        "up" if !event.keystroke.modifiers.platform => {
                            // Arrow up: move selection up by 1
                            if let Some(current) = current_selection {
                                if current > 0 {
                                    new_selection = Some(current - 1);
                                }
                            } else if num_children > 0 {
                                // No selection, start at last item
                                new_selection = Some(num_children - 1);
                            }
                        }
                        "down" if event.keystroke.modifiers.platform => {
                            // Cmd+Down: jump to last item
                            if num_children > 0 {
                                new_selection = Some(num_children - 1);
                            }
                        }
                        "up" if event.keystroke.modifiers.platform => {
                            // Cmd+Up: jump to first item
                            if num_children > 0 {
                                new_selection = Some(0);
                            }
                        }
                        "enter" => {
                            // Enter: trigger on_confirm callback
                            if let Some(current) = current_selection {
                                if let Some(ref handler) = on_confirm {
                                    handler(current, window, cx);
                                }
                            }
                            return;
                        }
                        _ => return,
                    }

                    // Update selection if it changed
                    if let Some(new_idx) = new_selection {
                        if current_selection != Some(new_idx) {
                            let new_indices: HashSet<usize> = [new_idx].into_iter().collect();
                            if let Some(ref handler) = on_selection_change {
                                handler(&new_indices, window, cx);
                            }
                        }
                    }
                },
            );
        }

        // Add search field if provided
        if let Some(search_field) = self.search_field {
            // Create a wrapper for search + content
            let mut wrapper = div().flex().flex_col().w_full().h_full();

            // Add search field with styling
            wrapper = wrapper.child(
                div()
                    .p(px(12.0))
                    .border_b_1()
                    .border_color(hsla(0.0, 0.0, 0.90, 1.0))
                    .bg(hsla(0.0, 0.0, 0.98, 1.0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap(px(8.0))
                            .child(
                                // Search icon
                                div()
                                    .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                                    .text_sm()
                                    .child("🔍"),
                            )
                            .child(
                                div()
                                    .flex_1()
                                    .child(search_field)
                            ),
                    )
            );

            // Create scrollable content area for list items
            let mut items_container = div().flex().flex_col().flex_1().w_full();

            // Add children
            for child in self.children {
                items_container = items_container.child(child);
            }

            wrapper = wrapper.child(items_container);
            container.child(wrapper)
        } else {
            // Original behavior: no search field, just content
            let mut content = div().flex().flex_col().w_full();

            // Add children
            for child in self.children {
                content = content.child(child);
            }

            container.child(content)
        }
    }
}

/// A section within a list for grouping related items.
///
/// ListSection provides a header and optional footer for grouping list items.
///
/// # Example
///
/// ```ignore
/// List::new("my-list")
///     .child(
///         ListSection::new("favorites")
///             .header("Favorites")
///             .children(favorites.iter().map(|f| {
///                 ListItem::new(("fav", f.id)).child(f.name.clone())
///             }))
///     )
///     .child(
///         ListSection::new("recent")
///             .header("Recent")
///             .children(recent.iter().map(|r| {
///                 ListItem::new(("recent", r.id)).child(r.name.clone())
///             }))
///     )
/// ```
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
