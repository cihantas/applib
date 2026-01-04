//! Badge component for displaying counts and labels.
//!
//! A small rounded badge for showing counts next to items.

use gpui::prelude::*;
use gpui::*;

/// A badge component for displaying counts or short labels.
///
/// Typically used to show counts next to sidebar items or toolbar buttons.
///
/// # Example
///
/// ```ignore
/// Badge::new("3")
/// ```
pub struct Badge {
    label: SharedString,
}

impl Badge {
    /// Creates a new badge with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
        }
    }
}

impl IntoElement for Badge {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .items_center()
            .justify_center()
            .px(px(6.0))
            .h(px(16.0))
            .min_w(px(20.0))
            .rounded(px(8.0))
            .bg(hsla(0.0, 0.0, 0.55, 1.0)) // Gray badge
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(hsla(0.0, 0.0, 1.0, 1.0)) // White text
                    .child(self.label),
            )
    }
}
