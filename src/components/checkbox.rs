//! Checkbox component for boolean toggle with label.
//!
//! This module provides a checkbox component for boolean selection.

use gpui::prelude::*;
use gpui::*;

/// A checkbox component.
///
/// # Example
///
/// ```ignore
/// Checkbox::new("checkout-after-create", "Check out after create")
///     .checked(self.checkout_after_create)
///     .on_change(cx.listener(|this, checked, _window, cx| {
///         this.checkout_after_create = *checked;
///         cx.notify();
///     }))
/// ```
pub struct Checkbox {
    id: ElementId,
    label: SharedString,
    checked: bool,
    disabled: bool,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Checkbox {
    /// Creates a new checkbox with the given id and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            checked: false,
            disabled: false,
            on_change: None,
        }
    }

    /// Sets whether the checkbox is checked.
    pub fn checked(mut self, checked: bool) -> Self {
        self.checked = checked;
        self
    }

    /// Sets whether the checkbox is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler for this checkbox.
    ///
    /// The handler receives a reference to the new checked state.
    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Checkbox {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let checked = self.checked;
        let disabled = self.disabled;

        // Checkbox box dimensions
        let box_size = px(14.0);

        // Colors
        let box_bg = if checked {
            hsla(211.0 / 360.0, 0.95, 0.53, 1.0) // Blue when checked
        } else {
            hsla(0.0, 0.0, 1.0, 1.0) // White when unchecked
        };

        let box_border = if checked {
            hsla(211.0 / 360.0, 0.80, 0.45, 1.0) // Darker blue border when checked
        } else {
            hsla(0.0, 0.0, 0.70, 1.0) // Gray border when unchecked
        };

        let box_bg_hover = if checked {
            hsla(211.0 / 360.0, 0.95, 0.48, 1.0) // Slightly darker blue on hover
        } else {
            hsla(0.0, 0.0, 0.97, 1.0) // Slight gray on hover
        };

        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0) // Gray when disabled
        } else {
            hsla(0.0, 0.0, 0.20, 1.0) // Dark gray (primary text)
        };

        let disabled_bg = hsla(0.0, 0.0, 0.90, 1.0);
        let disabled_border = hsla(0.0, 0.0, 0.82, 1.0);

        // Build the checkbox box
        let checkbox_box = if disabled {
            div()
                .size(box_size)
                .rounded(px(3.0))
                .border_1()
                .bg(disabled_bg)
                .border_color(disabled_border)
                .flex()
                .items_center()
                .justify_center()
        } else {
            div()
                .size(box_size)
                .rounded(px(3.0))
                .border_1()
                .bg(box_bg)
                .border_color(box_border)
                .flex()
                .items_center()
                .justify_center()
                .shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.08),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(1.0),
                    spread_radius: px(0.0),
                }])
        };

        // Add checkmark if checked (using a simple check character)
        let checkbox_box = if checked {
            checkbox_box.child(
                div()
                    .text_color(gpui::white())
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .child("\u{2713}"), // Unicode checkmark
            )
        } else {
            checkbox_box
        };

        // Build the label
        let label = div()
            .text_sm()
            .text_color(text_color)
            .ml(px(6.0))
            .child(self.label);

        // Build the container
        let container = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0));

        let container = if disabled {
            container.cursor_default()
        } else {
            container
                .cursor_pointer()
                .hover(move |style| style.bg(box_bg_hover))
        };

        let container = container.child(checkbox_box).child(label);

        // Add click handler if provided and not disabled
        if let Some(handler) = self.on_change {
            if !disabled {
                let new_checked = !checked;
                container.on_click(move |_event, window, cx| {
                    handler(&new_checked, window, cx);
                })
            } else {
                container
            }
        } else {
            container
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkbox_creation() {
        let checkbox = Checkbox::new("test", "Test Checkbox");
        assert!(!checkbox.checked);
        assert!(!checkbox.disabled);
    }

    #[test]
    fn test_checkbox_checked() {
        let checkbox = Checkbox::new("test", "Test").checked(true);
        assert!(checkbox.checked);
    }

    #[test]
    fn test_checkbox_disabled() {
        let checkbox = Checkbox::new("test", "Test").disabled(true);
        assert!(checkbox.disabled);
    }
}
