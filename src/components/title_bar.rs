//! Window title bar component.
//!
//! A title bar component with gradient background, traffic light buttons,
//! and centered title text. Supports window dragging.

use super::TrafficLights;
use gpui::prelude::*;
use gpui::*;

const TITLE_BAR_HEIGHT: f32 = 22.0;

/// A window title bar.
///
/// Displays a title bar with:
/// - Subtle gradient background (lighter at top)
/// - Traffic light buttons (close, minimize, maximize) on the left
/// - Centered window title
/// - Window dragging support (click and drag to move window)
///
/// # Example
///
/// ```ignore
/// TitleBar::new("My Window")
///     .draggable(true)
///     .on_close(cx.listener(|_, _, _, cx| cx.quit()))
/// ```
pub struct TitleBar {
    title: SharedString,
    draggable: bool,
    on_close: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_minimize: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_maximize: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TitleBar {
    /// Creates a new title bar with the given title.
    ///
    /// By default, dragging is enabled.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            draggable: true,
            on_close: None,
            on_minimize: None,
            on_maximize: None,
        }
    }

    /// Sets whether the title bar can be dragged to move the window.
    ///
    /// Defaults to `true`.
    pub fn draggable(mut self, draggable: bool) -> Self {
        self.draggable = draggable;
        self
    }

    /// Sets the handler for the close button.
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Sets the handler for the minimize button.
    pub fn on_minimize(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_minimize = Some(Box::new(handler));
        self
    }

    /// Sets the handler for the maximize button.
    pub fn on_maximize(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_maximize = Some(Box::new(handler));
        self
    }
}

impl IntoElement for TitleBar {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        // Create traffic lights with handlers
        let mut traffic_lights = TrafficLights::new();
        if let Some(handler) = self.on_close {
            traffic_lights = traffic_lights.on_close(handler);
        }
        if let Some(handler) = self.on_minimize {
            traffic_lights = traffic_lights.on_minimize(handler);
        }
        if let Some(handler) = self.on_maximize {
            traffic_lights = traffic_lights.on_maximize(handler);
        }

        let draggable = self.draggable;

        // Center: Title with inset shadow effect (draggable area)
        // White text 1px below creates embossed/inset look
        let title = self.title.clone();
        let mut center = div()
            .id("title-bar-center")
            .relative()
            .flex()
            .flex_1()
            .h_full()
            .items_center()
            .justify_center()
            .child(
                div()
                    .relative()
                    // White shadow text (1px below)
                    .child(
                        div()
                            .absolute()
                            .top(px(1.0))
                            .left_0()
                            .w_full()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(hsla(0.0, 0.0, 1.0, 0.7))
                            .child(title.clone()),
                    )
                    // Foreground text (dark)
                    .child(
                        div()
                            .relative()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .text_color(hsla(0.0, 0.0, 0.30, 1.0))
                            .child(title),
                    ),
            );

        // Right: Spacer for symmetry (also draggable)
        let mut right = div()
            .id("title-bar-right")
            .relative()
            .w(px(100.0))
            .h_full();

        // Add drag handlers if draggable
        if draggable {
            center = center
                .cursor(CursorStyle::Arrow)
                .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                    window.start_window_move();
                });

            right = right
                .cursor(CursorStyle::Arrow)
                .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                    window.start_window_move();
                });
        }

        // Title bar styling - native GPU gradient
        div()
            .relative()
            .flex()
            .flex_row()
            .items_center()
            .h(px(TITLE_BAR_HEIGHT))
            .w_full()
            .px(px(10.0))
            // Smooth gradient: #F3EEF0 at top to #BDBDBD at bottom
            // Angle 180.0 = top-to-bottom direction
            .bg(linear_gradient(
                180.0,
                linear_color_stop(hsla(0.933, 0.17, 0.94, 1.0), 0.0),
                linear_color_stop(hsla(0.0, 0.0, 0.741, 1.0), 1.0),
            ))
            // Top highlight line (1px bright line at very top edge)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .w_full()
                    .h(px(1.0))
                    .bg(hsla(0.0, 0.0, 1.0, 0.5)),
            )
            // Bottom border for depth
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.50, 1.0))
            // Left: Traffic lights (not draggable)
            .child(
                div()
                    .relative()
                    .flex()
                    .flex_row()
                    .w(px(100.0))
                    .child(traffic_lights),
            )
            // Center: Title (draggable)
            .child(center)
            // Right: Spacer (draggable)
            .child(right)
    }
}
