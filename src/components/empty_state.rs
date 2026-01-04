//! Empty state component for displaying "no data" scenarios.
//!
//! This module provides an empty state component similar to SwiftUI's ContentUnavailableView,
//! used to communicate when content is unavailable or empty.

use gpui::prelude::*;
use gpui::*;

use super::Icon;

/// An empty state component for displaying "no data" scenarios.
///
/// Similar to SwiftUI's ContentUnavailableView, this component displays
/// a centered message with an optional icon, description, and action button
/// when there is no content to show.
///
/// # Example
///
/// ```ignore
/// // Basic empty state
/// EmptyState::new("no-results")
///     .icon(Icon::MagnifyingGlass)
///     .title("No Results")
///     .description("Try a different search term")
///
/// // With action button
/// EmptyState::new("no-data")
///     .icon(Icon::Document)
///     .title("No Documents")
///     .description("Create your first document to get started")
///     .action(
///         Button::new("create", "Create Document")
///             .primary()
///             .on_click(|_e, _w, cx| { /* ... */ })
///     )
/// ```
pub struct EmptyState {
    id: ElementId,
    icon: Option<Icon>,
    title: SharedString,
    description: Option<SharedString>,
    action: Option<AnyElement>,
}

impl EmptyState {
    /// Creates a new empty state with the given id.
    ///
    /// The id is required for GPUI's element tracking system.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            icon: None,
            title: "No Data".into(),
            description: None,
            action: None,
        }
    }

    /// Sets the icon to display at the top of the empty state.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the title text.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = title.into();
        self
    }

    /// Sets the description text (optional).
    ///
    /// The description appears below the title in a smaller, lighter font.
    pub fn description(mut self, description: impl Into<SharedString>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Sets an optional action button or element.
    ///
    /// The action appears below the description and is typically used
    /// to help users get started or resolve the empty state.
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl IntoElement for EmptyState {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Colors
        let icon_color = hsla(0.0, 0.0, 0.65, 1.0); // Medium gray for icon
        let title_color = hsla(0.0, 0.0, 0.20, 1.0); // Dark gray for title
        let description_color = hsla(0.0, 0.0, 0.50, 1.0); // Lighter gray for description

        let mut content = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(16.0));

        // Add icon if provided
        if let Some(icon) = self.icon {
            content = content.child(
                div()
                    .text_color(icon_color)
                    .text_size(px(48.0)) // Large icon
                    .child(icon.as_str()),
            );
        }

        // Title
        content = content.child(
            div()
                .text_color(title_color)
                .text_size(px(15.0))
                .font_weight(FontWeight::SEMIBOLD)
                .child(self.title),
        );

        // Description if provided
        if let Some(description) = self.description {
            content = content.child(
                div()
                    .text_color(description_color)
                    .text_size(px(13.0))
                    .text_align(TextAlign::Center)
                    .max_w(px(300.0)) // Limit width for better readability
                    .child(description),
            );
        }

        // Action button if provided
        if let Some(action) = self.action {
            content = content.child(
                div()
                    .pt(px(8.0)) // Extra spacing above action
                    .child(action),
            );
        }

        // Container that centers everything vertically and horizontally
        div()
            .id(self.id)
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .child(content)
    }
}
