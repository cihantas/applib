//! Stepper control for incrementing/decrementing numeric values.
//!
//! This module provides a stepper component with plus/minus buttons.

use gpui::prelude::*;
use gpui::*;
use std::ops::RangeInclusive;
use std::rc::Rc;

/// A stepper control for adjusting numeric values.
///
/// # Example
///
/// ```ignore
/// Stepper::new("quantity-stepper", 5, 1..=10)
///     .step(1)
///     .label("Quantity")
///     .on_change(|new_value, _window, _cx| {
///         println!("Value changed to: {}", new_value);
///     })
/// ```
pub struct Stepper {
    id: ElementId,
    value: i32,
    range: RangeInclusive<i32>,
    step: i32,
    label: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(i32, &mut Window, &mut App) + 'static>>,
}

impl Stepper {
    /// Creates a new stepper with the given id, initial value, and range.
    pub fn new(id: impl Into<ElementId>, value: i32, range: RangeInclusive<i32>) -> Self {
        Self {
            id: id.into(),
            value,
            range,
            step: 1,
            label: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Sets the step amount for increment/decrement.
    pub fn step(mut self, step: i32) -> Self {
        self.step = step;
        self
    }

    /// Sets the label text shown next to the stepper.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets whether the stepper is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler called when the value changes.
    /// The handler receives the new value.
    pub fn on_change(
        mut self,
        handler: impl Fn(i32, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Check if decrement is allowed (value > min).
    fn can_decrement(&self) -> bool {
        self.value > *self.range.start()
    }

    /// Check if increment is allowed (value < max).
    fn can_increment(&self) -> bool {
        self.value < *self.range.end()
    }
}

/// Colors for stepper buttons.
struct StepperColors {
    bg: Hsla,
    bg_hover: Hsla,
    bg_active: Hsla,
    border: Hsla,
    text: Hsla,
    text_disabled: Hsla,
    divider: Hsla,
}

impl StepperColors {
    fn new() -> Self {
        Self {
            bg: hsla(0.0, 0.0, 0.97, 1.0),
            bg_hover: hsla(0.0, 0.0, 0.93, 1.0),
            bg_active: hsla(0.0, 0.0, 0.88, 1.0),
            border: hsla(0.0, 0.0, 0.78, 1.0),
            text: hsla(0.0, 0.0, 0.20, 1.0),
            text_disabled: hsla(0.0, 0.0, 0.55, 1.0),
            divider: hsla(0.0, 0.0, 0.85, 1.0),
        }
    }

    fn disabled() -> Self {
        Self {
            bg: hsla(0.0, 0.0, 0.94, 1.0),
            bg_hover: hsla(0.0, 0.0, 0.94, 1.0),
            bg_active: hsla(0.0, 0.0, 0.94, 1.0),
            border: hsla(0.0, 0.0, 0.85, 1.0),
            text: hsla(0.0, 0.0, 0.55, 1.0),
            text_disabled: hsla(0.0, 0.0, 0.70, 1.0),
            divider: hsla(0.0, 0.0, 0.88, 1.0),
        }
    }
}

impl IntoElement for Stepper {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = if self.disabled {
            StepperColors::disabled()
        } else {
            StepperColors::new()
        };

        let disabled = self.disabled;
        let can_decrement = self.can_decrement() && !disabled;
        let can_increment = self.can_increment() && !disabled;
        let step = self.step;
        let value = self.value;
        let range = self.range.clone();

        // Create the segmented control container
        let control = div()
            .flex()
            .flex_row()
            .items_center()
            .h(px(22.0))
            .bg(colors.bg)
            .border_1()
            .border_color(colors.border)
            .rounded(px(5.0))
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.06),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(1.0),
                spread_radius: px(0.0),
            }]);

        // Decrement button (-)
        let decrement_text_color = if can_decrement {
            colors.text
        } else {
            colors.text_disabled
        };

        let mut decrement_btn = div()
            .id(("stepper-dec", 0u32))
            .flex()
            .items_center()
            .justify_center()
            .w(px(24.0))
            .h_full()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(decrement_text_color)
            .child("−"); // Use proper minus sign

        if can_decrement {
            let on_change = self.on_change.clone();
            let new_value = (value - step).max(*range.start());

            decrement_btn = decrement_btn
                .cursor_pointer()
                .hover(move |style| style.bg(colors.bg_hover))
                .active(move |style| style.bg(colors.bg_active))
                .on_click(move |_event, window, cx| {
                    if let Some(ref handler) = on_change {
                        handler(new_value, window, cx);
                    }
                });
        } else {
            decrement_btn = decrement_btn.cursor_default();
        }

        // Divider
        let divider = div()
            .w(px(1.0))
            .h(px(14.0))
            .bg(colors.divider);

        // Increment button (+)
        let increment_text_color = if can_increment {
            colors.text
        } else {
            colors.text_disabled
        };

        let mut increment_btn = div()
            .id(("stepper-inc", 0u32))
            .flex()
            .items_center()
            .justify_center()
            .w(px(24.0))
            .h_full()
            .text_sm()
            .font_weight(FontWeight::SEMIBOLD)
            .text_color(increment_text_color)
            .child("+");

        if can_increment {
            let on_change = self.on_change.clone();
            let new_value = (value + step).min(*range.end());

            increment_btn = increment_btn
                .cursor_pointer()
                .hover(move |style| style.bg(colors.bg_hover))
                .active(move |style| style.bg(colors.bg_active))
                .on_click(move |_event, window, cx| {
                    if let Some(ref handler) = on_change {
                        handler(new_value, window, cx);
                    }
                });
        } else {
            increment_btn = increment_btn.cursor_default();
        }

        let control_with_buttons = control
            .child(decrement_btn)
            .child(divider)
            .child(increment_btn);

        // Build the complete stepper with optional label
        let label_color = hsla(0.0, 0.0, 0.30, 1.0);

        if let Some(label_text) = self.label {
            div()
                .id(self.id)
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(label_color)
                        .child(label_text),
                )
                .child(control_with_buttons)
        } else {
            div()
                .id(self.id)
                .flex()
                .flex_row()
                .items_center()
                .child(control_with_buttons)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stepper_creation() {
        let stepper = Stepper::new("test", 5, 1..=10);
        assert_eq!(stepper.value, 5);
        assert_eq!(stepper.step, 1);
        assert!(!stepper.disabled);
    }

    #[test]
    fn test_stepper_with_step() {
        let stepper = Stepper::new("test", 5, 1..=10).step(2);
        assert_eq!(stepper.step, 2);
    }

    #[test]
    fn test_stepper_with_label() {
        let stepper = Stepper::new("test", 5, 1..=10).label("Quantity");
        assert_eq!(stepper.label, Some("Quantity".into()));
    }

    #[test]
    fn test_stepper_disabled() {
        let stepper = Stepper::new("test", 5, 1..=10).disabled(true);
        assert!(stepper.disabled);
    }

    #[test]
    fn test_stepper_can_decrement() {
        let stepper_at_min = Stepper::new("test", 1, 1..=10);
        assert!(!stepper_at_min.can_decrement());

        let stepper_above_min = Stepper::new("test", 5, 1..=10);
        assert!(stepper_above_min.can_decrement());
    }

    #[test]
    fn test_stepper_can_increment() {
        let stepper_at_max = Stepper::new("test", 10, 1..=10);
        assert!(!stepper_at_max.can_increment());

        let stepper_below_max = Stepper::new("test", 5, 1..=10);
        assert!(stepper_below_max.can_increment());
    }
}
