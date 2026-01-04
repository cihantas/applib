//! Button components for GPUI.
//!
//! This module provides button components with primary and secondary styles.

use gpui::prelude::*;
use gpui::*;

/// Button style variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ButtonStyle {
    /// Primary action button (blue) - used for default/main actions
    Primary,
    /// Secondary button (gray) - used for cancel/alternative actions
    #[default]
    Secondary,
}

/// A button component.
///
/// # Example
///
/// ```ignore
/// Button::new("my-button", "Click Me")
///     .style(ButtonStyle::Primary)
///     .on_click(cx.listener(|this, _event, _window, cx| {
///         // Handle click
///     }))
/// ```
pub struct Button {
    id: ElementId,
    label: SharedString,
    style: ButtonStyle,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Button {
    /// Creates a new button with the given id and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            style: ButtonStyle::default(),
            disabled: false,
            on_click: None,
        }
    }

    /// Sets the button style.
    pub fn style(mut self, style: ButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Creates a primary (blue) button.
    pub fn primary(mut self) -> Self {
        self.style = ButtonStyle::Primary;
        self
    }

    /// Creates a secondary (gray) button.
    pub fn secondary(mut self) -> Self {
        self.style = ButtonStyle::Secondary;
        self
    }

    /// Sets whether the button is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the click handler for this button.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    fn colors(&self) -> ButtonColors {
        match self.style {
            ButtonStyle::Primary => ButtonColors {
                bg: hsla(211.0 / 360.0, 0.95, 0.53, 1.0),
                bg_hover: hsla(211.0 / 360.0, 0.95, 0.48, 1.0),
                bg_active: hsla(211.0 / 360.0, 0.95, 0.40, 1.0),
                border: hsla(211.0 / 360.0, 0.80, 0.45, 1.0),
                border_hover: hsla(211.0 / 360.0, 0.80, 0.40, 1.0),
                text: gpui::white(),
            },
            ButtonStyle::Secondary => ButtonColors {
                bg: hsla(0.0, 0.0, 0.97, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.93, 1.0),
                bg_active: hsla(0.0, 0.0, 0.88, 1.0),
                border: hsla(0.0, 0.0, 0.78, 1.0),
                border_hover: hsla(0.0, 0.0, 0.72, 1.0),
                text: hsla(0.0, 0.0, 0.15, 1.0),
            },
        }
    }
}

struct ButtonColors {
    bg: Hsla,
    bg_hover: Hsla,
    bg_active: Hsla,
    border: Hsla,
    border_hover: Hsla,
    text: Hsla,
}

impl IntoElement for Button {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = self.colors();
        let disabled = self.disabled;

        let base = div()
            .flex()
            .items_center()
            .justify_center()
            .px_4()
            .py_1()
            .min_w(px(80.0))
            .h(px(24.0))
            .rounded(px(6.0))
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        let styled = if disabled {
            base.bg(hsla(0.0, 0.0, 0.90, 1.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.82, 1.0))
                .text_color(hsla(0.0, 0.0, 0.55, 1.0))
                .cursor_default()
        } else {
            base.bg(colors.bg)
                .border_1()
                .border_color(colors.border)
                .text_color(colors.text)
                .cursor_pointer()
                .hover(move |style| {
                    style
                        .bg(colors.bg_hover)
                        .border_color(colors.border_hover)
                })
        };

        let with_id = styled.child(self.label).id(self.id);

        let with_active = if disabled {
            with_id.active(|style| style)
        } else {
            with_id.active(move |style| {
                style.bg(colors.bg_active).shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(1.0),
                    spread_radius: px(0.0),
                }])
            })
        };

        // Add click handler if provided and not disabled
        if let Some(handler) = self.on_click {
            if !disabled {
                with_active.on_click(move |event, window, cx| {
                    handler(event, window, cx);
                })
            } else {
                with_active
            }
        } else {
            with_active
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_button_creation() {
        let button = Button::new("test", "Test Button");
        assert_eq!(button.style, ButtonStyle::Secondary);
        assert!(!button.disabled);
    }

    #[test]
    fn test_button_primary() {
        let button = Button::new("test", "Test").primary();
        assert_eq!(button.style, ButtonStyle::Primary);
    }
}
