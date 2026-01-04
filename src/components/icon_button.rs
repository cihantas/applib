//! Compact icon-only button component for GPUI.
//!
//! This module provides a compact button for inline use, such as action buttons
//! next to file names or toolbar icons.

use gpui::prelude::*;
use gpui::*;

/// Size variants for icon buttons.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IconButtonSize {
    /// Small size (20x20) - for inline use next to text
    #[default]
    Small,
    /// Medium size (24x24) - for toolbars
    Medium,
}

/// Style variants for icon buttons.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum IconButtonStyle {
    /// Ghost style - minimal chrome, appears on hover
    #[default]
    Ghost,
    /// Filled style - more prominent with background
    Filled,
}

/// A compact icon-only button component.
///
/// # Example
///
/// ```ignore
/// IconButton::new("stage-btn", "+")
///     .style(IconButtonStyle::Ghost)
///     .tooltip("Stage file")
///     .on_click(cx.listener(|this, _event, _window, cx| {
///         // Handle click
///     }))
/// ```
pub struct IconButton {
    id: ElementId,
    icon: SharedString,
    size: IconButtonSize,
    style: IconButtonStyle,
    tooltip: Option<SharedString>,
    disabled: bool,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl IconButton {
    /// Creates a new icon button with the given id and icon text.
    ///
    /// For now, the icon parameter accepts text characters like "+", "-", "×", "✓".
    /// This can be evolved to support proper icon enums or SVG later.
    pub fn new(id: impl Into<ElementId>, icon: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            icon: icon.into(),
            size: IconButtonSize::default(),
            style: IconButtonStyle::default(),
            tooltip: None,
            disabled: false,
            on_click: None,
        }
    }

    /// Sets the button size.
    pub fn size(mut self, size: IconButtonSize) -> Self {
        self.size = size;
        self
    }

    /// Sets the button style.
    pub fn style(mut self, style: IconButtonStyle) -> Self {
        self.style = style;
        self
    }

    /// Creates a ghost (minimal) style button.
    pub fn ghost(mut self) -> Self {
        self.style = IconButtonStyle::Ghost;
        self
    }

    /// Creates a filled (prominent) style button.
    pub fn filled(mut self) -> Self {
        self.style = IconButtonStyle::Filled;
        self
    }

    /// Sets the tooltip text (shown on hover).
    pub fn tooltip(mut self, tooltip: impl Into<SharedString>) -> Self {
        self.tooltip = Some(tooltip.into());
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

    fn dimensions(&self) -> (Pixels, Pixels) {
        match self.size {
            IconButtonSize::Small => (px(20.0), px(20.0)),
            IconButtonSize::Medium => (px(24.0), px(24.0)),
        }
    }

    fn colors(&self) -> IconButtonColors {
        match self.style {
            IconButtonStyle::Ghost => IconButtonColors {
                bg: hsla(0.0, 0.0, 0.0, 0.0),
                bg_hover: hsla(0.0, 0.0, 0.0, 0.06),
                bg_active: hsla(0.0, 0.0, 0.0, 0.10),
                text: hsla(0.0, 0.0, 0.40, 1.0),
                text_hover: hsla(0.0, 0.0, 0.20, 1.0),
            },
            IconButtonStyle::Filled => IconButtonColors {
                bg: hsla(0.0, 0.0, 0.94, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.90, 1.0),
                bg_active: hsla(0.0, 0.0, 0.85, 1.0),
                text: hsla(0.0, 0.0, 0.30, 1.0),
                text_hover: hsla(0.0, 0.0, 0.15, 1.0),
            },
        }
    }
}

struct IconButtonColors {
    bg: Hsla,
    bg_hover: Hsla,
    bg_active: Hsla,
    text: Hsla,
    text_hover: Hsla,
}

impl IntoElement for IconButton {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = self.colors();
        let (width, height) = self.dimensions();
        let disabled = self.disabled;
        let tooltip_text = self.tooltip.clone();

        // Build the inner button element
        let build_button = |id: ElementId| {
            let base = div()
                .id(id)
                .flex()
                .items_center()
                .justify_center()
                .w(width)
                .h(height)
                .rounded(px(4.0))
                .text_sm()
                .font_weight(FontWeight::MEDIUM);

            let styled = if disabled {
                base.bg(hsla(0.0, 0.0, 0.0, 0.0))
                    .text_color(hsla(0.0, 0.0, 0.70, 1.0))
                    .cursor_default()
            } else {
                base.bg(colors.bg)
                    .text_color(colors.text)
                    .cursor_pointer()
                    .hover(move |style| style.bg(colors.bg_hover).text_color(colors.text_hover))
            };

            // Add shadow for filled style
            if self.style == IconButtonStyle::Filled && !disabled {
                styled.shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.06),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(1.0),
                    spread_radius: px(0.0),
                }])
            } else {
                styled
            }
        };

        // Wrap with tooltip if provided, using group hover pattern
        if let Some(text) = tooltip_text {
            // Build button with inner ID when wrapped
            let button_id: ElementId = ("icon-btn-inner", 0u32).into();
            let button = build_button(button_id).child(self.icon.clone());

            let with_active = if disabled {
                button.active(|style| style)
            } else {
                button.active(move |style| style.bg(colors.bg_active))
            };

            // Add click handler if provided and not disabled
            let with_click = if let Some(handler) = self.on_click {
                if !disabled {
                    with_active.on_click(move |event, window, cx| {
                        handler(event, window, cx);
                    })
                } else {
                    with_active
                }
            } else {
                with_active
            };

            // Tooltip styling (dark background, light text - system tooltip style)
            let tooltip_bg = hsla(0.0, 0.0, 0.15, 0.95);
            let tooltip_text_color = hsla(0.0, 0.0, 0.95, 1.0);
            let tooltip_border = hsla(0.0, 0.0, 0.25, 1.0);

            let tooltip_panel = div()
                .absolute()
                .top(px(-30.0))
                .left(px(-4.0))
                .px(px(8.0))
                .py(px(4.0))
                .bg(tooltip_bg)
                .text_color(tooltip_text_color)
                .text_xs()
                .rounded(px(4.0))
                .border_1()
                .border_color(tooltip_border)
                .shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.3),
                    offset: point(px(0.0), px(2.0)),
                    blur_radius: px(6.0),
                    spread_radius: px(0.0),
                }])
                .whitespace_nowrap()
                .child(text);

            div()
                .id(self.id)
                .relative()
                .group("")
                .child(with_click)
                .child(
                    div()
                        .invisible()
                        .group_hover("", |style| style.visible())
                        .child(tooltip_panel),
                )
        } else {
            // No tooltip - build button with the main ID
            let button = build_button(self.id).child(self.icon);

            let with_active = if disabled {
                button.active(|style| style)
            } else {
                button.active(move |style| style.bg(colors.bg_active))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_icon_button_creation() {
        let button = IconButton::new("test", "+");
        assert_eq!(button.style, IconButtonStyle::Ghost);
        assert_eq!(button.size, IconButtonSize::Small);
        assert!(!button.disabled);
    }

    #[test]
    fn test_icon_button_filled() {
        let button = IconButton::new("test", "+").filled();
        assert_eq!(button.style, IconButtonStyle::Filled);
    }

    #[test]
    fn test_icon_button_medium() {
        let button = IconButton::new("test", "+").size(IconButtonSize::Medium);
        assert_eq!(button.size, IconButtonSize::Medium);
    }
}
