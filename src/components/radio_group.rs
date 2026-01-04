//! RadioGroup component for mutually exclusive option selection.
//!
//! This module provides a radio group component for selecting one option from multiple choices.

use gpui::prelude::*;
use gpui::*;
use std::rc::Rc;

/// A single option in a radio group.
#[derive(Clone)]
struct RadioOption {
    value: SharedString,
    label: SharedString,
}

/// A radio group component for mutually exclusive selection.
///
/// # Example
///
/// ```ignore
/// RadioGroup::new("local-changes")
///     .label("Local changes")
///     .option("none", "Don't change")
///     .option("stash", "Stash and reapply")
///     .option("discard", "Discard")
///     .selected("none")
///     .on_change(cx.listener(|this, value, _window, cx| {
///         this.local_changes = value.to_string();
///         cx.notify();
///     }))
/// ```
pub struct RadioGroup {
    id: ElementId,
    label: Option<SharedString>,
    options: Vec<RadioOption>,
    selected: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&SharedString, &mut Window, &mut App) + 'static>>,
}

impl RadioGroup {
    /// Creates a new radio group with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            label: None,
            options: Vec::new(),
            selected: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Sets the label for the radio group (displayed above the options).
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Adds an option to the radio group.
    pub fn option(mut self, value: impl Into<SharedString>, label: impl Into<SharedString>) -> Self {
        self.options.push(RadioOption {
            value: value.into(),
            label: label.into(),
        });
        self
    }

    /// Sets the currently selected value.
    pub fn selected(mut self, value: impl Into<SharedString>) -> Self {
        self.selected = Some(value.into());
        self
    }

    /// Sets whether the radio group is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler for this radio group.
    ///
    /// The handler receives a reference to the newly selected value.
    pub fn on_change(
        mut self,
        handler: impl Fn(&SharedString, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for RadioGroup {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let disabled = self.disabled;
        let selected = self.selected.clone();
        let on_change = self.on_change;

        // Radio button dimensions
        let radio_size = px(14.0);
        let inner_size = px(6.0);

        // Colors
        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0) // Gray when disabled
        } else {
            hsla(0.0, 0.0, 0.20, 1.0) // Dark gray (primary text)
        };

        let label_color = hsla(0.0, 0.0, 0.35, 1.0); // Slightly lighter for group label

        // Build the container
        let mut container = div()
            .id(self.id)
            .flex()
            .flex_col()
            .gap(px(4.0));

        // Add group label if provided
        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(label_color)
                    .mb(px(4.0))
                    .child(label),
            );
        }

        // Add radio options
        for (index, option) in self.options.into_iter().enumerate() {
            let is_selected = selected.as_ref() == Some(&option.value);
            let option_value = option.value.clone();

            // Colors for this radio button
            let radio_bg = if is_selected {
                hsla(211.0 / 360.0, 0.95, 0.53, 1.0) // Blue when selected
            } else {
                hsla(0.0, 0.0, 1.0, 1.0) // White when not selected
            };

            let radio_border = if is_selected {
                hsla(211.0 / 360.0, 0.80, 0.45, 1.0) // Darker blue border when selected
            } else {
                hsla(0.0, 0.0, 0.70, 1.0) // Gray border when not selected
            };

            let radio_bg_hover = if is_selected {
                hsla(211.0 / 360.0, 0.95, 0.48, 1.0) // Slightly darker blue on hover
            } else {
                hsla(0.0, 0.0, 0.97, 1.0) // Slight gray on hover
            };

            let disabled_bg = hsla(0.0, 0.0, 0.90, 1.0);
            let disabled_border = hsla(0.0, 0.0, 0.82, 1.0);

            // Build the radio circle
            let radio_circle = if disabled {
                div()
                    .size(radio_size)
                    .rounded_full()
                    .border_1()
                    .bg(disabled_bg)
                    .border_color(disabled_border)
                    .flex()
                    .items_center()
                    .justify_center()
            } else {
                div()
                    .size(radio_size)
                    .rounded_full()
                    .border_1()
                    .bg(radio_bg)
                    .border_color(radio_border)
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

            // Add inner circle if selected
            let radio_circle = if is_selected {
                radio_circle.child(
                    div()
                        .size(inner_size)
                        .rounded_full()
                        .bg(gpui::white()),
                )
            } else {
                radio_circle
            };

            // Build the option label
            let option_label = div()
                .text_sm()
                .text_color(text_color)
                .ml(px(6.0))
                .child(option.label);

            // Build the option row
            let mut option_row = div()
                .id(("radio-option", index))
                .flex()
                .flex_row()
                .items_center()
                .py(px(2.0));

            option_row = if disabled {
                option_row.cursor_default()
            } else {
                option_row
                    .cursor_pointer()
                    .hover(move |style| style.bg(radio_bg_hover))
            };

            option_row = option_row.child(radio_circle).child(option_label);

            // Add click handler if provided and not disabled
            if let Some(ref handler) = on_change {
                if !disabled {
                    let handler = handler.clone();
                    let value = option_value;
                    option_row = option_row.on_click(move |_event, window, cx| {
                        handler(&value, window, cx);
                    });
                }
            }

            container = container.child(option_row);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radio_group_creation() {
        let group = RadioGroup::new("test");
        assert!(group.label.is_none());
        assert!(group.options.is_empty());
        assert!(group.selected.is_none());
        assert!(!group.disabled);
    }

    #[test]
    fn test_radio_group_with_options() {
        let group = RadioGroup::new("test")
            .option("a", "Option A")
            .option("b", "Option B");
        assert_eq!(group.options.len(), 2);
    }

    #[test]
    fn test_radio_group_with_label() {
        let group = RadioGroup::new("test")
            .label("Select an option");
        assert!(group.label.is_some());
    }

    #[test]
    fn test_radio_group_selected() {
        let group = RadioGroup::new("test")
            .option("a", "Option A")
            .selected("a");
        assert_eq!(group.selected, Some("a".into()));
    }

    #[test]
    fn test_radio_group_disabled() {
        let group = RadioGroup::new("test").disabled(true);
        assert!(group.disabled);
    }
}
