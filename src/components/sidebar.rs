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
/// Sidebar fills its parent container (SwiftUI pattern). When used in a
/// SplitView, the split view controls the width.
///
/// # Example
///
/// ```ignore
/// Sidebar::new()
///     .child(SidebarItem::new("local", "Local Changes"))
///     .child(SidebarSection::new("branches", "BRANCHES"))
/// ```
pub struct Sidebar {
    children: Vec<AnyElement>,
}

impl Sidebar {
    /// Creates a new sidebar.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
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
            .size_full()
            .gap(px(4.0))
            .p(px(8.0))
            // Source list background - light blue-gray
            // Note: No border - SplitView handles the separator (SwiftUI pattern)
            .bg(hsla(210.0 / 360.0, 0.08, 0.93, 1.0))
            .children(self.children)
    }
}
