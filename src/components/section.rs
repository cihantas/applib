//! Section component for grouping content with header/footer.
//!
//! A container component for grouping related content with optional header and footer text,
//! similar to SwiftUI's Section within List or Form.

use gpui::prelude::*;
use gpui::*;

/// A section container for grouping content with header and footer.
///
/// Section provides a way to group related content with optional header and footer text.
/// It's commonly used within List or Form views to organize content into logical groups.
///
/// # Example
///
/// ```ignore
/// Section::new()
///     .header("Recent Items")
///     .footer("Showing 10 of 50")
///     .child(ListItem::new("item-1").child("First item"))
///     .child(ListItem::new("item-2").child("Second item"))
/// ```
///
/// With custom header view:
/// ```ignore
/// Section::new()
///     .header_view(div().child("Custom Header").text_color(red()))
///     .child(content)
/// ```
pub struct Section {
    header: Option<SharedString>,
    header_view: Option<AnyElement>,
    footer: Option<SharedString>,
    footer_view: Option<AnyElement>,
    children: Vec<AnyElement>,
}

impl Section {
    /// Creates a new empty section.
    pub fn new() -> Self {
        Self {
            header: None,
            header_view: None,
            footer: None,
            footer_view: None,
            children: Vec::new(),
        }
    }

    /// Sets the header text for the section.
    ///
    /// The header is displayed in uppercase, smaller font, gray color.
    pub fn header(mut self, text: impl Into<SharedString>) -> Self {
        self.header = Some(text.into());
        self.header_view = None;
        self
    }

    /// Sets a custom header view for the section.
    ///
    /// Use this when you need more control over the header appearance.
    /// This overrides any text set via `header()`.
    pub fn header_view(mut self, view: impl IntoElement) -> Self {
        self.header_view = Some(view.into_any_element());
        self.header = None;
        self
    }

    /// Sets the footer text for the section.
    ///
    /// The footer is displayed in smaller font, gray color.
    pub fn footer(mut self, text: impl Into<SharedString>) -> Self {
        self.footer = Some(text.into());
        self.footer_view = None;
        self
    }

    /// Sets a custom footer view for the section.
    ///
    /// Use this when you need more control over the footer appearance.
    /// This overrides any text set via `footer()`.
    pub fn footer_view(mut self, view: impl IntoElement) -> Self {
        self.footer_view = Some(view.into_any_element());
        self.footer = None;
        self
    }

    /// Adds a child element to the section content.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the section content.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for Section {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Section {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut container = div().flex().flex_col();

        // Header section
        if let Some(header_view) = self.header_view {
            container = container.child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .child(header_view),
            );
        } else if let Some(header_text) = self.header {
            // Note: GPUI doesn't have text-transform, so uppercase must be done at call site
            container = container.child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .child(
                        div()
                            .text_xs()
                            .font_weight(FontWeight::MEDIUM)
                            .text_color(hsla(0.0, 0.0, 0.45, 1.0))
                            .child(header_text),
                    ),
            );
        }

        // Content section
        let mut content = div().flex().flex_col();
        for child in self.children {
            content = content.child(child);
        }
        container = container.child(content);

        // Footer section
        if let Some(footer_view) = self.footer_view {
            container = container.child(
                div()
                    .px(px(16.0))
                    .py(px(8.0))
                    .child(footer_view),
            );
        } else if let Some(footer_text) = self.footer {
            container = container.child(
                div()
                    .px(px(16.0))
                    .py(px(6.0))
                    .child(
                        div()
                            .text_xs()
                            .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                            .child(footer_text),
                    ),
            );
        }

        container
    }
}
