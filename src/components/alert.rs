//! Alert component for modal dialogs.
//!
//! A modal dialog for displaying important messages with action buttons.

use gpui::prelude::*;
use gpui::*;

/// Icon types for alert dialogs.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertIcon {
    /// Informational alert
    #[default]
    Info,
    /// Warning alert
    Warning,
    /// Error/critical alert
    Error,
}

impl AlertIcon {
    /// Returns the Unicode character for this icon.
    fn as_str(&self) -> &'static str {
        match self {
            AlertIcon::Info => "ℹ",
            AlertIcon::Warning => "⚠",
            AlertIcon::Error => "⊘",
        }
    }

    /// Returns the color for this icon type.
    fn color(&self) -> Hsla {
        match self {
            AlertIcon::Info => hsla(211.0 / 360.0, 0.95, 0.53, 1.0), // Blue
            AlertIcon::Warning => hsla(45.0 / 360.0, 0.95, 0.50, 1.0), // Yellow/Orange
            AlertIcon::Error => hsla(0.0, 0.85, 0.55, 1.0),           // Red
        }
    }
}

/// Role/style for alert buttons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlertButtonRole {
    /// Standard button
    #[default]
    Default,
    /// Cancel button (secondary styling)
    Cancel,
    /// Destructive action (red styling)
    Destructive,
}

/// A button for use within an Alert dialog.
pub struct AlertButton {
    label: SharedString,
    role: AlertButtonRole,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl AlertButton {
    /// Creates a new alert button with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Default,
            on_click: None,
        }
    }

    /// Creates a cancel button.
    pub fn cancel(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Cancel,
            on_click: None,
        }
    }

    /// Creates a destructive action button.
    pub fn destructive(
        label: impl Into<SharedString>,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        Self {
            label: label.into(),
            role: AlertButtonRole::Destructive,
            on_click: Some(Box::new(handler)),
        }
    }

    /// Sets the click handler for this button.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Renders the button as an element.
    fn render(self, id: ElementId) -> Stateful<Div> {
        let (bg, bg_hover, bg_active, border, border_hover, text_color) = match self.role {
            AlertButtonRole::Default => (
                hsla(211.0 / 360.0, 0.95, 0.53, 1.0),
                hsla(211.0 / 360.0, 0.95, 0.48, 1.0),
                hsla(211.0 / 360.0, 0.95, 0.40, 1.0),
                hsla(211.0 / 360.0, 0.80, 0.45, 1.0),
                hsla(211.0 / 360.0, 0.80, 0.40, 1.0),
                gpui::white(),
            ),
            AlertButtonRole::Cancel => (
                hsla(0.0, 0.0, 0.97, 1.0),
                hsla(0.0, 0.0, 0.93, 1.0),
                hsla(0.0, 0.0, 0.88, 1.0),
                hsla(0.0, 0.0, 0.78, 1.0),
                hsla(0.0, 0.0, 0.72, 1.0),
                hsla(0.0, 0.0, 0.15, 1.0),
            ),
            AlertButtonRole::Destructive => (
                hsla(0.0, 0.85, 0.55, 1.0),
                hsla(0.0, 0.85, 0.50, 1.0),
                hsla(0.0, 0.85, 0.42, 1.0),
                hsla(0.0, 0.70, 0.45, 1.0),
                hsla(0.0, 0.70, 0.40, 1.0),
                gpui::white(),
            ),
        };

        let base = div()
            .id(id)
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
            .bg(bg)
            .border_1()
            .border_color(border)
            .text_color(text_color)
            .cursor_pointer()
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }])
            .hover(move |style| style.bg(bg_hover).border_color(border_hover))
            .active(move |style| {
                style.bg(bg_active).shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.12),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(1.0),
                    spread_radius: px(0.0),
                }])
            })
            .child(self.label);

        if let Some(handler) = self.on_click {
            base.on_click(move |event, window, cx| {
                handler(event, window, cx);
            })
        } else {
            base
        }
    }
}

/// A modal alert dialog component.
///
/// Alert displays an important message with optional icon and action buttons.
/// Cancel is placed on the left and primary action on the right.
///
/// # Example
///
/// ```ignore
/// Alert::new("Delete Item?")
///     .message("This action cannot be undone.")
///     .icon(AlertIcon::Warning)
///     .button(AlertButton::cancel("Cancel"))
///     .button(AlertButton::destructive("Delete", |_, _, _| {
///         // Handle delete
///     }))
///     .on_dismiss(cx.listener(|this, _event, _window, cx| {
///         this.show_alert = false;
///         cx.notify();
///     }))
/// ```
pub struct Alert {
    id: ElementId,
    title: SharedString,
    message: Option<SharedString>,
    icon: Option<AlertIcon>,
    buttons: Vec<AlertButton>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Alert {
    /// Creates a new alert with the given title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            id: "alert".into(),
            title: title.into(),
            message: None,
            icon: None,
            buttons: Vec::new(),
            on_dismiss: None,
        }
    }

    /// Sets the element ID for the alert.
    pub fn id(mut self, id: impl Into<ElementId>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the message text displayed below the title.
    pub fn message(mut self, message: impl Into<SharedString>) -> Self {
        self.message = Some(message.into());
        self
    }

    /// Sets the icon displayed in the alert.
    pub fn icon(mut self, icon: AlertIcon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Adds a button to the alert.
    ///
    /// Buttons are displayed in order, with Cancel on the left
    /// and primary/destructive actions on the right.
    pub fn button(mut self, button: AlertButton) -> Self {
        self.buttons.push(button);
        self
    }

    /// Sets the dismiss handler, called when the backdrop is clicked or Escape is pressed.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Alert {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Colors
        let backdrop_color = hsla(0.0, 0.0, 0.0, 0.4);
        let panel_bg = hsla(0.0, 0.0, 0.97, 1.0);
        let title_color = hsla(0.0, 0.0, 0.15, 1.0);
        let message_color = hsla(0.0, 0.0, 0.40, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let separator_color = hsla(0.0, 0.0, 0.88, 1.0);

        // Build icon element
        let icon_element = self.icon.map(|icon| {
            div()
                .flex()
                .items_center()
                .justify_center()
                .w(px(48.0))
                .h(px(48.0))
                .rounded(px(24.0))
                .bg(icon.color().opacity(0.15))
                .text_color(icon.color())
                .text_2xl()
                .child(icon.as_str())
        });

        // Build content area
        let mut content = div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(12.0))
            .px(px(24.0))
            .py(px(20.0));

        // Add icon if present
        if let Some(icon_el) = icon_element {
            content = content.child(icon_el);
        }

        // Add title
        content = content.child(
            div()
                .text_base()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(title_color)
                .text_center()
                .child(self.title),
        );

        // Add message if present
        if let Some(message) = self.message {
            content = content.child(
                div()
                    .text_sm()
                    .text_color(message_color)
                    .text_center()
                    .max_w(px(280.0))
                    .child(message),
            );
        }

        // Build buttons bar
        let buttons_bar = if !self.buttons.is_empty() {
            let mut buttons_row = div()
                .flex()
                .flex_row()
                .justify_center()
                .gap(px(8.0))
                .w_full()
                .px(px(24.0))
                .py(px(16.0))
                .border_t_1()
                .border_color(separator_color)
                .bg(hsla(0.0, 0.0, 0.95, 1.0))
                .rounded_b(px(10.0));

            for (index, button) in self.buttons.into_iter().enumerate() {
                let button_id: ElementId = ("alert-button", index).into();
                buttons_row = buttons_row.child(button.render(button_id));
            }

            Some(buttons_row)
        } else {
            None
        };

        // Build the dialog panel
        let mut panel = div()
            .flex()
            .flex_col()
            .w(px(320.0))
            .bg(panel_bg)
            .rounded(px(10.0))
            .border_1()
            .border_color(border_color)
            .shadow(vec![
                // Outer shadow for depth
                BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.25),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                // Inner highlight at top
                BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.5),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                },
            ]);

        // Add content
        panel = panel.child(content);

        // Add buttons bar if present
        if let Some(buttons) = buttons_bar {
            panel = panel.child(buttons);
        }

        // Build the backdrop with click-to-dismiss
        let mut backdrop = div()
            .id(self.id)
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(backdrop_color);

        // Add dismiss handler to backdrop
        if let Some(handler) = self.on_dismiss {
            backdrop = backdrop.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        // Add the panel to the backdrop
        // The panel stops click propagation to avoid dismissing when clicking inside
        backdrop.child(
            panel.on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                cx.stop_propagation();
            }),
        )
    }
}

