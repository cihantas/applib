//! List item component for displaying rows in a list.
//!
//! This module provides a reusable list item component with support for selection,
//! hover states, and click handling.

use gpui::prelude::*;
use gpui::*;

/// A list item component.
///
/// # Example
///
/// ```ignore
/// ListItem::new("item-1")
///     .selected(true)
///     .on_click(cx.listener(|this, _event, _window, cx| {
///         // Handle click
///     }))
///     .child(div().child("List item content"))
/// ```
pub struct ListItem {
    id: ElementId,
    selected: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl ListItem {
    /// Creates a new list item with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            selected: false,
            on_click: None,
            children: Vec::new(),
        }
    }

    /// Sets whether the item is selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets the click handler for this list item.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Add a child element to this list item.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl IntoElement for ListItem {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let selected = self.selected;

        let base = div()
            .flex()
            .flex_col()
            .px(px(16.0))
            .py(px(12.0))
            .min_h(px(48.0))
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.90, 1.0));

        let styled = if selected {
            // Selected state: blue gradient
            base.bg(hsla(211.0 / 360.0, 0.95, 0.53, 1.0))
                .cursor_pointer()
        } else {
            // Unselected state: white background with hover
            base.bg(hsla(0.0, 0.0, 1.0, 1.0))
                .cursor_pointer()
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.98, 1.0)))
        };

        let mut with_children = styled.id(self.id);
        for child in self.children {
            with_children = with_children.child(child);
        }

        // Add click handler if provided
        if let Some(handler) = self.on_click {
            with_children.on_click(move |event, window, cx| {
                handler(event, window, cx);
            })
        } else {
            with_children
        }
    }
}
