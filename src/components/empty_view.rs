//! An invisible view that takes no space.
//!
//! EmptyView is useful for conditional rendering where both branches must return
//! the same type. It renders nothing and takes no space in layout.

use gpui::prelude::*;
use gpui::*;

/// An invisible view that renders nothing and takes no space.
///
/// This is equivalent to SwiftUI's EmptyView.
///
/// # Example
///
/// ```ignore
/// if show_content {
///     content_view.into_any_element()
/// } else {
///     EmptyView::new().into_any_element()
/// }
/// ```
#[derive(Default)]
pub struct EmptyView;

impl EmptyView {
    /// Creates a new empty view.
    pub fn new() -> Self {
        Self
    }
}

impl IntoElement for EmptyView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
    }
}
