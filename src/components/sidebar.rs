//! Sidebar container component.
//!
//! A sidebar for navigation and organization.

use gpui::prelude::*;
use gpui::*;

/// A sidebar container.
///
/// Provides a vertical navigation panel with source list styling,
/// including gradient background and proper spacing.
///
/// # Example
///
/// ```ignore
/// Sidebar::new()
///     .child(SidebarItem::new("local", "Local Changes"))
///     .child(SidebarSection::new("branches", "BRANCHES"))
/// ```
pub struct Sidebar {
    width: Pixels,
    children: Vec<AnyElement>,
}

impl Sidebar {
    /// Creates a new sidebar with default width (200px).
    pub fn new() -> Self {
        Self {
            width: px(200.0),
            children: Vec::new(),
        }
    }

    /// Sets the sidebar width.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// Adds a child element to the sidebar.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the sidebar.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for Sidebar {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Sidebar {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_col()
            .w(self.width)
            .h_full()
            .gap(px(4.0))
            .p(px(8.0))
            // Source list background - light blue-gray
            .bg(hsla(210.0 / 360.0, 0.08, 0.93, 1.0))
            // Right border to separate from content
            .border_r_1()
            .border_color(hsla(0.0, 0.0, 0.80, 1.0))
            .children(self.children)
    }
}
