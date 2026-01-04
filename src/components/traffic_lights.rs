//! Traffic light window control buttons.
//!
//! The iconic red, yellow, and green circular buttons for window controls.
//! These appear in the top-left corner of windows.

use gpui::prelude::*;
use gpui::*;

/// Traffic light window control buttons (close, minimize, maximize).
///
/// The three colored circular buttons that appear in the top-left of windows.
///
/// # Example
///
/// ```ignore
/// TrafficLights::new()
///     .on_close(cx.listener(|_, _, _, cx| cx.quit()))
/// ```
pub struct TrafficLights {
    on_close: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_minimize: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_maximize: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl TrafficLights {
    /// Creates new traffic light buttons.
    pub fn new() -> Self {
        Self {
            on_close: None,
            on_minimize: None,
            on_maximize: None,
        }
    }

    /// Sets the handler for the close button (red).
    pub fn on_close(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_close = Some(Box::new(handler));
        self
    }

    /// Sets the handler for the minimize button (yellow).
    pub fn on_minimize(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_minimize = Some(Box::new(handler));
        self
    }

    /// Sets the handler for the maximize/zoom button (green).
    pub fn on_maximize(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_maximize = Some(Box::new(handler));
        self
    }

    /// Creates a glass ball style traffic light button.
    /// Inset look with shine and shadow layers.
    fn button(
        base_color: Hsla,
        dark_color: Hsla,
        light_color: Hsla,
        id: impl Into<ElementId>,
        handler: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    ) -> impl IntoElement {
        let button = div()
            .id(id)
            .relative()
            .w(px(12.0))
            .h(px(12.0))
            .cursor_pointer()
            // Outer shadow for inset effect
            .shadow(vec![
                // Outer dark shadow (top - inset look)
                BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.35),
                    offset: point(px(0.0), px(0.5)),
                    blur_radius: px(1.0),
                    spread_radius: px(0.0),
                },
                // Bottom highlight (subtle light underneath)
                BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.3),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                },
            ])
            // Outer ring / border
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(dark_color),
            )
            // Base color fill
            .child(
                div()
                    .absolute()
                    .inset_0()
                    .rounded(px(6.0))
                    .bg(base_color),
            )
            // Top inner shadow (darker at top for inset glass look)
            .child(
                div()
                    .absolute()
                    .top_0()
                    .left_0()
                    .w_full()
                    .h(px(6.0))
                    .rounded_t(px(6.0))
                    .bg(dark_color)
                    .opacity(0.25),
            )
            // Top shine / highlight (glass reflection)
            .child(
                div()
                    .absolute()
                    .top(px(2.0))
                    .left(px(2.0))
                    .w(px(5.0))
                    .h(px(3.0))
                    .rounded(px(2.0))
                    .bg(light_color)
                    .opacity(0.6),
            )
            // Bottom highlight (brighter at bottom)
            .child(
                div()
                    .absolute()
                    .bottom(px(1.0))
                    .left(px(2.0))
                    .w(px(8.0))
                    .h(px(4.0))
                    .rounded_b(px(4.0))
                    .bg(light_color)
                    .opacity(0.2),
            );

        if let Some(handler) = handler {
            button.on_click(handler)
        } else {
            button
        }
    }
}

impl Default for TrafficLights {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for TrafficLights {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .items_center()
            // Close button (red)
            .child(Self::button(
                hsla(4.0 / 360.0, 0.70, 0.52, 1.0),  // Base red
                hsla(4.0 / 360.0, 0.80, 0.35, 1.0),  // Dark red (shadow/border)
                hsla(0.0, 0.0, 1.0, 1.0),            // White (highlight)
                "traffic-light-close",
                self.on_close,
            ))
            // Minimize button (yellow)
            .child(Self::button(
                hsla(42.0 / 360.0, 0.85, 0.55, 1.0), // Base yellow/amber
                hsla(35.0 / 360.0, 0.80, 0.35, 1.0), // Dark amber (shadow/border)
                hsla(0.0, 0.0, 1.0, 1.0),            // White (highlight)
                "traffic-light-minimize",
                self.on_minimize,
            ))
            // Maximize button (green)
            .child(Self::button(
                hsla(120.0 / 360.0, 0.55, 0.45, 1.0), // Base green
                hsla(120.0 / 360.0, 0.65, 0.28, 1.0), // Dark green (shadow/border)
                hsla(0.0, 0.0, 1.0, 1.0),             // White (highlight)
                "traffic-light-maximize",
                self.on_maximize,
            ))
    }
}
