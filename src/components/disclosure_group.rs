//! Disclosure group component with collapsible functionality.
//!
//! A collapsible section with a header and child items, matching SwiftUI's DisclosureGroup.

use gpui::prelude::*;
use gpui::*;

/// A disclosure group (collapsible section).
///
/// Displays a section header with a disclosure triangle and child items.
/// State can be managed externally via binding, or internally when wrapped
/// in a stateful container.
///
/// # Example
///
/// ```ignore
/// // With external state management:
/// DisclosureGroup::new("group-id", "Branches", is_expanded, move |expanded, cx| {
///     on_toggle(expanded, cx);
/// })
///     .child(SidebarItem::new("main", "main").bold(true))
///     .child(SidebarItem::new("dev", "develop"))
/// ```
pub struct DisclosureGroup {
    id: ElementId,
    title: SharedString,
    expanded: bool,
    on_toggle: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
    children: Vec<AnyElement>,
}

impl DisclosureGroup {
    /// Creates a new disclosure group with the given title.
    ///
    /// - `id`: Unique identifier for the group header
    /// - `title`: Display title for the section
    /// - `expanded`: Current expanded state
    /// - `on_toggle`: Callback when the header is clicked
    pub fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        expanded: bool,
        on_toggle: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            expanded,
            on_toggle: Some(Box::new(on_toggle)),
            children: Vec::new(),
        }
    }

    /// Creates a disclosure group without a toggle callback (static display only).
    pub fn new_static(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        expanded: bool,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            expanded,
            on_toggle: None,
            children: Vec::new(),
        }
    }

    /// Adds a child element to the group.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the group.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl IntoElement for DisclosureGroup {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let expanded = self.expanded;
        let new_expanded = !expanded;

        let mut header = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .px(px(8.0))
            .py(px(4.0))
            .cursor_pointer()
            .hover(|style| style.bg(hsla(0.0, 0.0, 0.0, 0.03)))
            .child(
                div()
                    .text_xs()
                    .text_color(hsla(0.0, 0.0, 0.45, 1.0))
                    .child(if expanded { "▼" } else { "▶" }),
            )
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(hsla(0.0, 0.0, 0.45, 1.0))
                    .child(self.title),
            );

        // Add click handler if provided
        if let Some(on_toggle) = self.on_toggle {
            header = header.on_click(move |_event, window, cx| {
                on_toggle(new_expanded, window, cx);
            });
        }

        // Build children container
        let mut children_container = div()
            .flex()
            .flex_col()
            .gap(px(1.0))
            .pl(px(8.0)); // Indent children

        // When collapsed, hide children
        if !expanded {
            children_container = children_container.h(px(0.0)).overflow_hidden();
        }

        for child in self.children {
            children_container = children_container.child(child);
        }

        div()
            .flex()
            .flex_col()
            .gap(px(2.0))
            .child(header)
            .child(children_container)
    }
}
