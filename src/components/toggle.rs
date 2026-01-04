//! Toggle/Switch component for boolean on/off control.
//!
//! This module provides a toggle component that supports both checkbox and switch styles.

use gpui::prelude::*;
use gpui::*;

/// Style variants for the Toggle component.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ToggleStyle {
    /// Checkbox style - square box with checkmark (default)
    #[default]
    Checkbox,
    /// Switch style - sliding pill with circle indicator
    Switch,
}

/// A toggle component for on/off control.
///
/// Supports both checkbox (square with checkmark) and switch (sliding pill) styles.
///
/// # Example
///
/// ```ignore
/// // Checkbox style (default)
/// Toggle::new("notifications", "Enable notifications", is_enabled)
///     .on_change(cx.listener(|this, is_on, _window, cx| {
///         this.notifications_enabled = *is_on;
///         cx.notify();
///     }))
///
/// // Switch style
/// Toggle::new("dark-mode", "Dark mode", is_dark)
///     .style(ToggleStyle::Switch)
///     .on_change(cx.listener(|this, is_on, _window, cx| {
///         this.dark_mode = *is_on;
///         cx.notify();
///     }))
/// ```
pub struct Toggle {
    id: ElementId,
    label: SharedString,
    is_on: bool,
    style: ToggleStyle,
    disabled: bool,
    on_change: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl Toggle {
    /// Creates a new toggle with the given id, label, and initial state.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>, is_on: bool) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            is_on,
            style: ToggleStyle::default(),
            disabled: false,
            on_change: None,
        }
    }

    /// Sets the toggle style (Checkbox or Switch).
    pub fn style(mut self, style: ToggleStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets whether the toggle is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler for this toggle.
    ///
    /// The handler receives a reference to the new on/off state.
    pub fn on_change(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Renders the checkbox style toggle.
    fn render_checkbox(&self) -> Div {
        let is_on = self.is_on;
        let disabled = self.disabled;

        // Checkbox box dimensions
        let box_size = px(14.0);

        // Colors
        let box_bg = if is_on {
            hsla(211.0 / 360.0, 0.95, 0.53, 1.0) // Blue when on
        } else {
            hsla(0.0, 0.0, 1.0, 1.0) // White when off
        };

        let box_border = if is_on {
            hsla(211.0 / 360.0, 0.80, 0.45, 1.0) // Darker blue border when on
        } else {
            hsla(0.0, 0.0, 0.70, 1.0) // Gray border when off
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

        // Add checkmark if on
        if is_on {
            checkbox_box.child(
                div()
                    .text_color(gpui::white())
                    .text_size(px(11.0))
                    .font_weight(FontWeight::BOLD)
                    .child("\u{2713}"), // Unicode checkmark
            )
        } else {
            checkbox_box
        }
    }

    /// Renders the switch style toggle.
    fn render_switch(&self) -> Div {
        let is_on = self.is_on;
        let disabled = self.disabled;

        // Switch dimensions
        let track_width = px(36.0);
        let track_height = px(20.0);
        let knob_size = px(16.0);
        let knob_margin = px(2.0);

        // Colors
        let track_bg_on = hsla(211.0 / 360.0, 0.95, 0.53, 1.0); // Blue when on
        let track_bg_off = hsla(0.0, 0.0, 0.78, 1.0); // Gray when off
        let track_bg_disabled = hsla(0.0, 0.0, 0.90, 1.0);

        let knob_bg = hsla(0.0, 0.0, 1.0, 1.0); // White knob
        let knob_bg_disabled = hsla(0.0, 0.0, 0.96, 1.0);

        let track_border_on = hsla(211.0 / 360.0, 0.80, 0.45, 1.0);
        let track_border_off = hsla(0.0, 0.0, 0.68, 1.0);
        let track_border_disabled = hsla(0.0, 0.0, 0.82, 1.0);

        // Build the track
        let track_bg = if disabled {
            track_bg_disabled
        } else if is_on {
            track_bg_on
        } else {
            track_bg_off
        };

        let track_border = if disabled {
            track_border_disabled
        } else if is_on {
            track_border_on
        } else {
            track_border_off
        };

        // Build the knob with position based on state
        let knob = div()
            .size(knob_size)
            .rounded(knob_size)
            .bg(if disabled { knob_bg_disabled } else { knob_bg })
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.15),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        // Build the track container
        let track = div()
            .w(track_width)
            .h(track_height)
            .rounded(track_height)
            .border_1()
            .bg(track_bg)
            .border_color(track_border)
            .flex()
            .items_center()
            .px(knob_margin);

        // Position knob based on on/off state
        if is_on {
            track.justify_end().child(knob)
        } else {
            track.justify_start().child(knob)
        }
    }
}

impl IntoElement for Toggle {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let is_on = self.is_on;
        let disabled = self.disabled;

        // Text color
        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0) // Gray when disabled
        } else {
            hsla(0.0, 0.0, 0.20, 1.0) // Dark gray (primary text)
        };

        // Hover background
        let hover_bg = hsla(0.0, 0.0, 0.97, 1.0);

        // Render the toggle control based on style
        let toggle_control = match self.style {
            ToggleStyle::Checkbox => self.render_checkbox(),
            ToggleStyle::Switch => self.render_switch(),
        };

        // Build the label
        let label = div()
            .text_sm()
            .text_color(text_color)
            .ml(px(8.0))
            .child(self.label);

        // Build the container
        let container = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .gap(px(2.0))
            .py(px(2.0))
            .px(px(4.0))
            .rounded(px(4.0));

        let container = if disabled {
            container.cursor_default()
        } else {
            container
                .cursor_pointer()
                .hover(move |style| style.bg(hover_bg))
        };

        let container = container.child(toggle_control).child(label);

        // Add click handler if provided and not disabled
        if let Some(handler) = self.on_change {
            if !disabled {
                let new_state = !is_on;
                container.on_click(move |_event, window, cx| {
                    handler(&new_state, window, cx);
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
    fn test_toggle_creation() {
        let toggle = Toggle::new("test", "Test Toggle", false);
        assert!(!toggle.is_on);
        assert!(!toggle.disabled);
        assert_eq!(toggle.style, ToggleStyle::Checkbox);
    }

    #[test]
    fn test_toggle_on() {
        let toggle = Toggle::new("test", "Test", true);
        assert!(toggle.is_on);
    }

    #[test]
    fn test_toggle_disabled() {
        let toggle = Toggle::new("test", "Test", false).disabled(true);
        assert!(toggle.disabled);
    }

    #[test]
    fn test_toggle_switch_style() {
        let toggle = Toggle::new("test", "Test", false).style(ToggleStyle::Switch);
        assert_eq!(toggle.style, ToggleStyle::Switch);
    }
}
