//! Slider component for selecting a value from a range.
//!
//! A control for selecting a continuous value from a range, similar to SwiftUI's Slider.

use gpui::prelude::*;
use gpui::*;
use std::cell::Cell;
use std::ops::RangeInclusive;
use std::rc::Rc;

/// A slider control for selecting a value from a continuous range.
///
/// This is a builder pattern for creating a slider. For stateful use with
/// mouse drag support, convert to `SliderState` using `into()`.
///
/// # Example
///
/// ```ignore
/// // Simple slider (visual display)
/// Slider::new("volume-slider", 50.0, 0.0..=100.0)
///     .step(1.0)
///     .label("Volume")
///     .on_change(|new_value, _window, _cx| {
///         println!("Value changed to: {}", new_value);
///     })
///
/// // With full state management (supports dragging)
/// // In a parent view's Render:
/// let slider_entity = cx.new(|_| SliderState::from(
///     Slider::new("volume", self.volume, 0.0..=100.0)
///         .on_change(cx.listener(|this, value, _window, cx| {
///             this.volume = *value;
///             cx.notify();
///         }))
/// ));
/// ```
pub struct Slider {
    id: ElementId,
    value: f64,
    range: RangeInclusive<f64>,
    step: Option<f64>,
    label: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(f64, &mut Window, &mut App) + 'static>>,
}

impl Slider {
    /// Creates a new slider with the given id, initial value, and range.
    pub fn new(id: impl Into<ElementId>, value: f64, range: RangeInclusive<f64>) -> Self {
        let clamped_value = value.clamp(*range.start(), *range.end());
        Self {
            id: id.into(),
            value: clamped_value,
            range,
            step: None,
            label: None,
            disabled: false,
            on_change: None,
        }
    }

    /// Sets the step increment for the slider.
    /// When set, values will snap to the nearest step.
    pub fn step(mut self, step: f64) -> Self {
        self.step = Some(step);
        self
    }

    /// Sets the label text shown next to the slider.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets whether the slider is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler called when the value changes.
    /// The handler receives the new value.
    pub fn on_change(
        mut self,
        handler: impl Fn(f64, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

/// Colors for the slider component.
struct SliderColors {
    track_bg: Hsla,
    track_fill: Hsla,
    track_border: Hsla,
    thumb_bg: Hsla,
    thumb_border: Hsla,
    label: Hsla,
}

impl SliderColors {
    fn new() -> Self {
        Self {
            // Colors: subtle gradient track with glossy thumb
            track_bg: hsla(0.0, 0.0, 0.85, 1.0),
            track_fill: hsla(211.0 / 360.0, 0.95, 0.53, 1.0), // Blue
            track_border: hsla(0.0, 0.0, 0.70, 1.0),
            thumb_bg: hsla(0.0, 0.0, 1.0, 1.0), // White
            thumb_border: hsla(0.0, 0.0, 0.60, 1.0),
            label: hsla(0.0, 0.0, 0.30, 1.0),
        }
    }

    fn disabled() -> Self {
        Self {
            track_bg: hsla(0.0, 0.0, 0.90, 1.0),
            track_fill: hsla(0.0, 0.0, 0.75, 1.0),
            track_border: hsla(0.0, 0.0, 0.82, 1.0),
            thumb_bg: hsla(0.0, 0.0, 0.94, 1.0),
            thumb_border: hsla(0.0, 0.0, 0.78, 1.0),
            label: hsla(0.0, 0.0, 0.55, 1.0),
        }
    }
}

/// Calculate the normalized position (0.0 to 1.0) from a value.
fn value_to_position(value: f64, range: &RangeInclusive<f64>) -> f64 {
    let range_size = range.end() - range.start();
    if range_size == 0.0 {
        return 0.0;
    }
    (value - range.start()) / range_size
}

/// Calculate value from a normalized position and apply step snapping.
fn position_to_value(position: f64, range: &RangeInclusive<f64>, step: Option<f64>) -> f64 {
    let range_size = range.end() - range.start();
    let raw_value = range.start() + (position.clamp(0.0, 1.0) * range_size);

    // Apply step snapping if step is set
    let value = if let Some(step) = step {
        let steps = ((raw_value - range.start()) / step).round();
        range.start() + (steps * step)
    } else {
        raw_value
    };

    // Clamp to range
    value.clamp(*range.start(), *range.end())
}

/// Track dimensions.
const TRACK_WIDTH: f32 = 200.0;
const TRACK_HEIGHT: f32 = 6.0;
const THUMB_SIZE: f32 = 18.0;

impl IntoElement for Slider {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = if self.disabled {
            SliderColors::disabled()
        } else {
            SliderColors::new()
        };

        let disabled = self.disabled;
        let position = value_to_position(self.value, &self.range);

        // Track dimensions
        let track_height = px(TRACK_HEIGHT);
        let thumb_size = px(THUMB_SIZE);
        let track_width = px(TRACK_WIDTH);

        // Build the track background (unfilled portion)
        let track_bg = div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h(track_height)
            .rounded(track_height)
            .bg(colors.track_bg)
            .border_1()
            .border_color(colors.track_border);

        // Build the filled portion of the track
        let track_fill = div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .h(track_height)
            .rounded(track_height)
            .bg(colors.track_fill)
            .w(relative(position as f32));

        // Calculate thumb position
        // Thumb needs to be centered on the track, offset by position
        let thumb_offset = track_height / 2.0 - thumb_size / 2.0;
        let thumb_left_percent = position as f32;

        // Build the thumb (draggable circle)
        let thumb = div()
            .absolute()
            .top(thumb_offset)
            .left(relative(thumb_left_percent))
            .ml(-thumb_size / 2.0) // Center the thumb
            .size(thumb_size)
            .rounded(thumb_size)
            .bg(colors.thumb_bg)
            .border_1()
            .border_color(colors.thumb_border)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.15),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        // Track container with relative positioning for absolute children
        let mut track_container = div()
            .id(("slider-track", 0u32))
            .relative()
            .w(track_width)
            .h(thumb_size)
            .flex()
            .items_center()
            .child(track_bg)
            .child(track_fill)
            .child(thumb);

        // Make the slider interactive if not disabled
        if !disabled {
            track_container = track_container.cursor_pointer();
        } else {
            track_container = track_container.cursor_default();
        }

        // Build the complete slider with optional label
        let label_color = colors.label;

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
                .child(track_container)
        } else {
            div()
                .id(self.id)
                .flex()
                .flex_row()
                .items_center()
                .child(track_container)
        }
    }
}

/// Internal state for a Slider rendered as a stateful view with full mouse drag support.
///
/// This provides proper mouse tracking including drag operations.
///
/// # Example
///
/// ```ignore
/// // In your view struct:
/// struct MyView {
///     slider: Entity<SliderState>,
/// }
///
/// impl MyView {
///     fn new(cx: &mut Context<Self>) -> Self {
///         Self {
///             slider: cx.new(|_| SliderState::from(
///                 Slider::new("my-slider", 50.0, 0.0..=100.0)
///             )),
///         }
///     }
/// }
///
/// impl Render for MyView {
///     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
///         div().child(self.slider.clone())
///     }
/// }
/// ```
pub struct SliderState {
    id: ElementId,
    value: f64,
    range: RangeInclusive<f64>,
    step: Option<f64>,
    label: Option<SharedString>,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(&f64, &mut Window, &mut App) + 'static>>,
    /// Track bounds from the last paint, used for mouse position calculations.
    /// Uses Rc<Cell<>> to allow sharing between paint callback and event handlers.
    track_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    /// Whether we're currently dragging
    is_dragging: bool,
}

impl SliderState {
    /// Get the current value.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Set the value programmatically.
    pub fn set_value(&mut self, value: f64) {
        self.value = value.clamp(*self.range.start(), *self.range.end());
    }

    /// Handle mouse down on the track - start dragging and update value.
    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.disabled {
            return;
        }

        self.is_dragging = true;
        self.update_value_from_mouse(event.position, window, cx);
    }

    /// Handle mouse move - update value if dragging.
    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if self.is_dragging && !self.disabled {
            self.update_value_from_mouse(event.position, window, cx);
        }
    }

    /// Handle mouse up - stop dragging.
    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_dragging = false;
        cx.notify();
    }

    /// Update the slider value based on mouse position.
    fn update_value_from_mouse(
        &mut self,
        mouse_position: Point<Pixels>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.track_bounds.get() {
            // Calculate relative position within the track
            // Pixels / Pixels gives f32
            let relative_x = mouse_position.x - bounds.origin.x;
            let position = (relative_x / bounds.size.width) as f64;
            let new_value = position_to_value(position, &self.range, self.step);

            if (new_value - self.value).abs() > f64::EPSILON {
                self.value = new_value;
                if let Some(ref handler) = self.on_change {
                    handler(&self.value, window, cx);
                }
                cx.notify();
            }
        }
    }
}

impl Render for SliderState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = if self.disabled {
            SliderColors::disabled()
        } else {
            SliderColors::new()
        };

        let position = value_to_position(self.value, &self.range);

        // Track dimensions
        let track_height = px(TRACK_HEIGHT);
        let thumb_size = px(THUMB_SIZE);
        let track_width = px(TRACK_WIDTH);

        // Build the track background (unfilled portion)
        let track_bg = div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .w_full()
            .h(track_height)
            .rounded(track_height)
            .bg(colors.track_bg)
            .border_1()
            .border_color(colors.track_border);

        // Build the filled portion of the track
        let track_fill = div()
            .absolute()
            .top(px(0.0))
            .left(px(0.0))
            .h(track_height)
            .rounded(track_height)
            .bg(colors.track_fill)
            .w(relative(position as f32));

        // Calculate thumb position
        let thumb_offset = track_height / 2.0 - thumb_size / 2.0;
        let thumb_left_percent = position as f32;

        // Build the thumb
        let thumb = div()
            .absolute()
            .top(thumb_offset)
            .left(relative(thumb_left_percent))
            .ml(-thumb_size / 2.0)
            .size(thumb_size)
            .rounded(thumb_size)
            .bg(colors.thumb_bg)
            .border_1()
            .border_color(colors.thumb_border)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.15),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        // Track container
        let mut track_container = div()
            .id(("slider-track", 0u32))
            .relative()
            .w(track_width)
            .h(thumb_size)
            .flex()
            .items_center()
            .child(track_bg)
            .child(track_fill)
            .child(thumb);

        // Make interactive if not disabled
        if !self.disabled {
            track_container = track_container
                .cursor_pointer()
                .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
                .on_mouse_move(cx.listener(Self::handle_mouse_move))
                .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
                .on_mouse_up_out(MouseButton::Left, cx.listener(Self::handle_mouse_up));
        } else {
            track_container = track_container.cursor_default();
        }

        // Add bounds tracking via canvas overlay
        let bounds_ref = self.track_bounds.clone();
        let bounds_tracker = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, _window, _cx| {
                bounds_ref.set(Some(bounds));
            },
        )
        .absolute()
        .size_full();

        // Wrap track container with bounds tracker
        let track_with_bounds = div()
            .relative()
            .w(track_width)
            .h(thumb_size)
            .child(track_container)
            .child(bounds_tracker);

        // Build complete slider with optional label
        let label_color = colors.label;

        if let Some(ref label_text) = self.label {
            div()
                .id(self.id.clone())
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_sm()
                        .text_color(label_color)
                        .child(label_text.clone()),
                )
                .child(track_with_bounds)
        } else {
            div()
                .id(self.id.clone())
                .flex()
                .flex_row()
                .items_center()
                .child(track_with_bounds)
        }
    }
}

/// Creates a SliderState from a Slider builder.
impl From<Slider> for SliderState {
    fn from(builder: Slider) -> Self {
        SliderState {
            id: builder.id,
            value: builder.value,
            range: builder.range,
            step: builder.step,
            label: builder.label,
            disabled: builder.disabled,
            on_change: builder.on_change.map(|handler| {
                Rc::new(move |value: &f64, window: &mut Window, cx: &mut App| {
                    handler(*value, window, cx);
                }) as Rc<dyn Fn(&f64, &mut Window, &mut App) + 'static>
            }),
            track_bounds: Rc::new(Cell::new(None)),
            is_dragging: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slider_creation() {
        let slider = Slider::new("test", 50.0, 0.0..=100.0);
        assert_eq!(slider.value, 50.0);
        assert!(!slider.disabled);
        assert!(slider.step.is_none());
    }

    #[test]
    fn test_slider_value_clamping() {
        let slider = Slider::new("test", 150.0, 0.0..=100.0);
        assert_eq!(slider.value, 100.0);

        let slider = Slider::new("test", -50.0, 0.0..=100.0);
        assert_eq!(slider.value, 0.0);
    }

    #[test]
    fn test_slider_with_step() {
        let slider = Slider::new("test", 50.0, 0.0..=100.0).step(10.0);
        assert_eq!(slider.step, Some(10.0));
    }

    #[test]
    fn test_slider_with_label() {
        let slider = Slider::new("test", 50.0, 0.0..=100.0).label("Volume");
        assert_eq!(slider.label, Some("Volume".into()));
    }

    #[test]
    fn test_slider_disabled() {
        let slider = Slider::new("test", 50.0, 0.0..=100.0).disabled(true);
        assert!(slider.disabled);
    }

    #[test]
    fn test_value_to_position() {
        let range = 0.0..=100.0;
        assert_eq!(value_to_position(50.0, &range), 0.5);
        assert_eq!(value_to_position(0.0, &range), 0.0);
        assert_eq!(value_to_position(100.0, &range), 1.0);
    }

    #[test]
    fn test_position_to_value() {
        let range = 0.0..=100.0;
        assert_eq!(position_to_value(0.5, &range, None), 50.0);
        assert_eq!(position_to_value(0.0, &range, None), 0.0);
        assert_eq!(position_to_value(1.0, &range, None), 100.0);
    }

    #[test]
    fn test_position_to_value_with_step() {
        let range = 0.0..=100.0;
        assert_eq!(position_to_value(0.45, &range, Some(10.0)), 50.0);
        assert_eq!(position_to_value(0.54, &range, Some(10.0)), 50.0);
        assert_eq!(position_to_value(0.56, &range, Some(10.0)), 60.0);
    }

    #[test]
    fn test_position_to_value_clamping() {
        let range = 0.0..=100.0;
        assert_eq!(position_to_value(-0.5, &range, None), 0.0);
        assert_eq!(position_to_value(1.5, &range, None), 100.0);
    }

    #[test]
    fn test_zero_range() {
        let range = 50.0..=50.0;
        assert_eq!(value_to_position(50.0, &range), 0.0);
    }
}
