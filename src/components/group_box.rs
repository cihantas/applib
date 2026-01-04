//! Group box component for visual grouping of related content.
//!
//! A labeled container that groups related controls together, similar to
//! SwiftUI's GroupBox or HTML's fieldset/legend pattern.

use gpui::prelude::*;
use gpui::*;

/// A labeled container for grouping related content.
///
/// GroupBox provides a bordered container with an optional title that visually
/// groups related controls together with subtle borders and rounded corners.
///
/// # Example
///
/// ```ignore
/// GroupBox::new()
///     .title("Settings")
///     .child(Toggle::new("opt1", "Option 1", enabled, |_, _, _| {}))
///     .child(Toggle::new("opt2", "Option 2", enabled, |_, _, _| {}))
/// ```
///
/// Without a title:
/// ```ignore
/// GroupBox::new()
///     .child(content1)
///     .child(content2)
/// ```
pub struct GroupBox {
    title: Option<SharedString>,
    children: Vec<AnyElement>,
    padding: Pixels,
}

impl GroupBox {
    /// Creates a new group box.
    pub fn new() -> Self {
        Self {
            title: None,
            children: Vec::new(),
            padding: px(12.0),
        }
    }

    /// Sets the title displayed at the top of the group box.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the padding inside the content area.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = padding.into();
        self
    }

    /// Adds a child element to the group box.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the group box.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for GroupBox {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for GroupBox {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        // Colors
        let border_color = hsla(0.0, 0.0, 0.82, 1.0);
        let bg_color = hsla(0.0, 0.0, 0.98, 1.0);
        let title_color = hsla(0.0, 0.0, 0.35, 1.0);
        let title_bg = hsla(0.0, 0.0, 0.97, 1.0);

        // Build content container
        let mut content = div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .p(self.padding);

        for child in self.children {
            content = content.child(child);
        }

        // Build the main container with border
        let mut container = div()
            .flex()
            .flex_col()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(6.0))
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.04),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        // Add title if present
        if let Some(title) = self.title {
            let title_element = div()
                .flex()
                .flex_row()
                .items_center()
                .px(px(12.0))
                .py(px(6.0))
                .bg(title_bg)
                .border_b_1()
                .border_color(border_color)
                .rounded_t(px(5.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(title_color)
                        .child(title),
                );

            container = container.child(title_element).child(content);
        } else {
            // No title - just apply top rounding to content area
            content = content.rounded(px(5.0));
            container = container.child(content);
        }

        container
    }
}
