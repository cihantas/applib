//! Navigation split view component for master-detail navigation.
//!
//! This module provides a navigation split view component with SwiftUI-like behavior:
//! - Sidebar column for navigation items (fixed width, scrollable)
//! - Detail column that displays content based on selection
//! - Selection state management via bindings
//!
//! # Example
//!
//! ```ignore
//! NavigationSplitView::new("nav-split")
//!     .sidebar(
//!         VStack::new()
//!             .child(SidebarItem::new("item-1", "First Item"))
//!             .child(SidebarItem::new("item-2", "Second Item"))
//!     )
//!     .detail(
//!         div().child("Select an item")
//!     )
//! ```

use gpui::prelude::*;
use gpui::*;

/// Default sidebar width in pixels.
const DEFAULT_SIDEBAR_WIDTH: f32 = 240.0;

/// A navigation split view component with sidebar and detail columns.
///
/// NavigationSplitView provides a two-column layout commonly used for
/// master-detail interfaces, where the sidebar contains navigation items
/// and the detail column shows the selected content.
///
/// # Features
///
/// - **Sidebar column**: Fixed width, scrollable, with source list styling
/// - **Detail column**: Flexible width, shows content based on selection
/// - **Divider**: Visual separator between columns
///
/// # Example
///
/// ```ignore
/// NavigationSplitView::new("file-browser")
///     .sidebar_width(px(280.0))
///     .sidebar(
///         VStack::new()
///             .child(SidebarItem::new("docs", "Documents").selected(true))
///             .child(SidebarItem::new("downloads", "Downloads"))
///     )
///     .detail(
///         div().child("Document contents here")
///     )
/// ```
pub struct NavigationSplitView {
    id: ElementId,
    sidebar_width: Pixels,
    sidebar_content: Option<AnyElement>,
    detail_content: Option<AnyElement>,
    divider_color: Hsla,
    sidebar_background: Hsla,
    detail_background: Hsla,
}

impl NavigationSplitView {
    /// Creates a new navigation split view with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            sidebar_width: px(DEFAULT_SIDEBAR_WIDTH),
            sidebar_content: None,
            detail_content: None,
            // Default colors matching macOS source list style
            divider_color: hsla(0.0, 0.0, 0.80, 1.0),
            sidebar_background: hsla(210.0 / 360.0, 0.08, 0.93, 1.0),
            detail_background: hsla(0.0, 0.0, 1.0, 1.0),
        }
    }

    /// Sets the sidebar width.
    ///
    /// Default is 240 pixels.
    pub fn sidebar_width(mut self, width: impl Into<Pixels>) -> Self {
        self.sidebar_width = width.into();
        self
    }

    /// Sets the sidebar content.
    ///
    /// The sidebar typically contains navigation items, lists, or other
    /// selectable content.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .sidebar(
    ///     List::new("sidebar-list", items.len(), |idx, selected, w, cx| {
    ///         // Render list items
    ///     })
    /// )
    /// ```
    pub fn sidebar(mut self, content: impl IntoElement) -> Self {
        self.sidebar_content = Some(content.into_any_element());
        self
    }

    /// Sets the detail content.
    ///
    /// The detail area typically shows the content for the currently
    /// selected sidebar item.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .detail(
    ///     div()
    ///         .p(px(16.0))
    ///         .child("Detail content here")
    /// )
    /// ```
    pub fn detail(mut self, content: impl IntoElement) -> Self {
        self.detail_content = Some(content.into_any_element());
        self
    }

    /// Sets the divider color between sidebar and detail.
    pub fn divider_color(mut self, color: Hsla) -> Self {
        self.divider_color = color;
        self
    }

    /// Sets the sidebar background color.
    ///
    /// Default is a light blue-gray matching macOS source list style.
    pub fn sidebar_background(mut self, color: Hsla) -> Self {
        self.sidebar_background = color;
        self
    }

    /// Sets the detail area background color.
    ///
    /// Default is white.
    pub fn detail_background(mut self, color: Hsla) -> Self {
        self.detail_background = color;
        self
    }
}

impl IntoElement for NavigationSplitView {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Sidebar column with scroll support
        let mut sidebar = div()
            .id((self.id.clone(), "nav-sidebar"))
            .flex()
            .flex_col()
            .w(self.sidebar_width)
            .h_full()
            .flex_shrink_0()
            .bg(self.sidebar_background)
            .overflow_y_scroll();

        if let Some(content) = self.sidebar_content {
            sidebar = sidebar.child(content);
        }

        // Detail column
        let mut detail = div()
            .flex()
            .flex_col()
            .flex_1()
            .h_full()
            .overflow_hidden()
            .bg(self.detail_background);

        if let Some(content) = self.detail_content {
            detail = detail.child(content);
        }

        // Divider
        let divider = div()
            .w(px(1.0))
            .h_full()
            .bg(self.divider_color)
            .flex_shrink_0();

        // Build the main container
        div()
            .id(self.id)
            .flex()
            .flex_row()
            .size_full()
            .child(sidebar)
            .child(divider)
            .child(detail)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_split_view_creation() {
        let view = NavigationSplitView::new("test-nav");
        assert_eq!(view.sidebar_width, px(DEFAULT_SIDEBAR_WIDTH));
        assert!(view.sidebar_content.is_none());
        assert!(view.detail_content.is_none());
    }

    #[test]
    fn test_sidebar_width() {
        let view = NavigationSplitView::new("test-nav").sidebar_width(px(300.0));
        assert_eq!(view.sidebar_width, px(300.0));
    }

    #[test]
    fn test_custom_colors() {
        let custom_divider = hsla(0.5, 0.5, 0.5, 1.0);
        let custom_sidebar_bg = hsla(0.0, 0.0, 0.9, 1.0);
        let custom_detail_bg = hsla(0.0, 0.0, 0.95, 1.0);

        let view = NavigationSplitView::new("test-nav")
            .divider_color(custom_divider)
            .sidebar_background(custom_sidebar_bg)
            .detail_background(custom_detail_bg);

        assert_eq!(view.divider_color, custom_divider);
        assert_eq!(view.sidebar_background, custom_sidebar_bg);
        assert_eq!(view.detail_background, custom_detail_bg);
    }
}
