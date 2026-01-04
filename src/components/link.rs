//! Link component for clickable hyperlinks.
//!
//! This module provides a Link component that displays text as a clickable
//! hyperlink that opens URLs in the default browser.

use gpui::prelude::*;
use gpui::*;

/// Link text color (blue).
fn link_color() -> Hsla {
    hsla(211.0 / 360.0, 0.95, 0.53, 1.0)
}

/// Link hover color (slightly darker blue).
fn link_hover_color() -> Hsla {
    hsla(211.0 / 360.0, 0.95, 0.45, 1.0)
}

/// A clickable hyperlink component that opens URLs in the default browser.
///
/// # Example
///
/// ```ignore
/// // Simple text link
/// Link::new("Visit Website", "https://example.com")
///
/// // With custom content
/// Link::with_content("https://example.com", |link| {
///     HStack::new()
///         .child(Text::new("Open"))
///         .child(Text::new("→"))
/// })
/// ```
pub struct Link {
    id: ElementId,
    url: SharedString,
    label: Option<SharedString>,
    content: Option<AnyElement>,
}

impl Link {
    /// Creates a new link with text label and URL.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        url: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            label: Some(label.into()),
            content: None,
        }
    }

    /// Creates a link with custom content.
    ///
    /// The closure receives the link for configuration and should return
    /// the element to display as the link content.
    pub fn with_content(
        id: impl Into<ElementId>,
        url: impl Into<SharedString>,
        content: impl IntoElement,
    ) -> Self {
        Self {
            id: id.into(),
            url: url.into(),
            label: None,
            content: Some(content.into_any_element()),
        }
    }
}

impl IntoElement for Link {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let url = self.url.clone();
        let color = link_color();
        let hover_color = link_hover_color();

        let base = div()
            .id(self.id)
            .cursor_pointer()
            .text_color(color)
            .hover(move |style| style.text_color(hover_color).text_decoration_1())
            .on_click(move |_event, _window, _cx| {
                let _ = open::that(url.as_ref());
            });

        if let Some(content) = self.content {
            base.child(content)
        } else if let Some(label) = self.label {
            base.child(label)
        } else {
            base.child(self.url)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_creation() {
        let link = Link::new("test-link", "Click me", "https://example.com");
        assert_eq!(link.url.as_ref(), "https://example.com");
        assert_eq!(link.label, Some("Click me".into()));
        assert!(link.content.is_none());
    }

    #[test]
    fn test_link_with_content() {
        let link = Link::with_content("test-link", "https://example.com", div().child("Custom"));
        assert_eq!(link.url.as_ref(), "https://example.com");
        assert!(link.label.is_none());
        assert!(link.content.is_some());
    }
}
