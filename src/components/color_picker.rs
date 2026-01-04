//! ColorPicker component for selecting colors.
//!
//! A control for selecting colors with a color well display and picker popover.
//! Similar to SwiftUI's ColorPicker.

use gpui::prelude::*;
use gpui::*;
use std::cell::Cell;
use std::rc::Rc;

/// Preset colors for the color grid.
const PRESET_COLORS: &[Hsla] = &[
    // Row 1: Reds and oranges
    hsla(0.0 / 360.0, 0.85, 0.55, 1.0),        // Red
    hsla(15.0 / 360.0, 0.90, 0.55, 1.0),       // Red-orange
    hsla(30.0 / 360.0, 0.95, 0.55, 1.0),       // Orange
    hsla(45.0 / 360.0, 0.95, 0.55, 1.0),       // Yellow-orange
    hsla(60.0 / 360.0, 0.90, 0.50, 1.0),       // Yellow
    hsla(75.0 / 360.0, 0.70, 0.50, 1.0),       // Yellow-green
    // Row 2: Greens and cyans
    hsla(120.0 / 360.0, 0.70, 0.45, 1.0),      // Green
    hsla(150.0 / 360.0, 0.70, 0.45, 1.0),      // Spring green
    hsla(180.0 / 360.0, 0.70, 0.45, 1.0),      // Cyan
    hsla(195.0 / 360.0, 0.80, 0.50, 1.0),      // Light blue
    hsla(211.0 / 360.0, 0.95, 0.53, 1.0),      // Blue
    hsla(240.0 / 360.0, 0.70, 0.55, 1.0),      // Deep blue
    // Row 3: Purples and pinks
    hsla(270.0 / 360.0, 0.70, 0.55, 1.0),      // Purple
    hsla(285.0 / 360.0, 0.70, 0.55, 1.0),      // Violet
    hsla(300.0 / 360.0, 0.70, 0.55, 1.0),      // Magenta
    hsla(330.0 / 360.0, 0.80, 0.60, 1.0),      // Pink
    hsla(345.0 / 360.0, 0.80, 0.55, 1.0),      // Rose
    hsla(0.0 / 360.0, 0.70, 0.40, 1.0),        // Dark red
    // Row 4: Grayscale
    hsla(0.0, 0.0, 0.0, 1.0),                  // Black
    hsla(0.0, 0.0, 0.25, 1.0),                 // Dark gray
    hsla(0.0, 0.0, 0.50, 1.0),                 // Gray
    hsla(0.0, 0.0, 0.75, 1.0),                 // Light gray
    hsla(0.0, 0.0, 0.90, 1.0),                 // Very light gray
    hsla(0.0, 0.0, 1.0, 1.0),                  // White
];

const fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla { h, s, l, a }
}

/// A color picker component for selecting colors.
///
/// Features a color well that shows the current color and opens a picker
/// popover on click. The popover includes a grid of preset colors and
/// optional sliders for precise color adjustment.
///
/// # Example
///
/// ```ignore
/// ColorPicker::new("bg-color", selected_color)
///     .label("Background")
///     .supports_opacity(true)
///     .on_change(|new_color, window, cx| {
///         println!("Color changed: {:?}", new_color);
///     })
/// ```
pub struct ColorPicker {
    id: ElementId,
    color: Hsla,
    label: Option<SharedString>,
    supports_opacity: bool,
    disabled: bool,
    on_change: Option<Rc<dyn Fn(Hsla, &mut Window, &mut App) + 'static>>,
}

impl ColorPicker {
    /// Creates a new color picker with the given id and initial color.
    pub fn new(id: impl Into<ElementId>, color: Hsla) -> Self {
        Self {
            id: id.into(),
            color,
            label: None,
            supports_opacity: false,
            disabled: false,
            on_change: None,
        }
    }

    /// Sets the label text shown next to the color well.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets whether the picker supports opacity/alpha channel adjustment.
    pub fn supports_opacity(mut self, supports: bool) -> Self {
        self.supports_opacity = supports;
        self
    }

    /// Sets whether the color picker is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the change handler called when the color changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(Hsla, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }
}

/// Colors for the color picker component.
struct ColorPickerColors {
    well_border: Hsla,
    well_border_hover: Hsla,
    label: Hsla,
}

impl ColorPickerColors {
    fn new() -> Self {
        Self {
            well_border: hsla(0.0, 0.0, 0.70, 1.0),
            well_border_hover: hsla(0.0, 0.0, 0.55, 1.0),
            label: hsla(0.0, 0.0, 0.30, 1.0),
        }
    }

    fn disabled() -> Self {
        Self {
            well_border: hsla(0.0, 0.0, 0.82, 1.0),
            well_border_hover: hsla(0.0, 0.0, 0.82, 1.0),
            label: hsla(0.0, 0.0, 0.55, 1.0),
        }
    }
}

/// Size constants for the color well.
const WELL_SIZE: f32 = 24.0;
const WELL_BORDER_RADIUS: f32 = 4.0;

impl IntoElement for ColorPicker {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = if self.disabled {
            ColorPickerColors::disabled()
        } else {
            ColorPickerColors::new()
        };

        let disabled = self.disabled;
        let color = self.color;

        // Build the color well (small rectangle showing current color)
        let well = div()
            .size(px(WELL_SIZE))
            .rounded(px(WELL_BORDER_RADIUS))
            .bg(color)
            .border_1()
            .border_color(colors.well_border)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.10),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        let well = if !disabled {
            well.cursor_pointer().hover(move |style| {
                style.border_color(colors.well_border_hover)
            })
        } else {
            well.cursor_default()
        };

        // Build the complete picker with optional label
        let label_color = colors.label;

        if let Some(label_text) = self.label {
            div()
                .id(self.id)
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(well)
                .child(
                    div()
                        .text_sm()
                        .text_color(label_color)
                        .child(label_text),
                )
        } else {
            div()
                .id(self.id)
                .flex()
                .flex_row()
                .items_center()
                .child(well)
        }
    }
}

/// Internal state for a ColorPicker rendered as a stateful view with popover support.
///
/// This provides the full interactive experience including the color picker popover
/// with preset colors, sliders, and optional hex input.
///
/// # Example
///
/// ```ignore
/// // In your view struct:
/// struct MyView {
///     color_picker: Entity<ColorPickerState>,
/// }
///
/// impl MyView {
///     fn new(cx: &mut Context<Self>) -> Self {
///         Self {
///             color_picker: cx.new(|_| ColorPickerState::from(
///                 ColorPicker::new("my-color", hsla(0.5, 0.8, 0.5, 1.0))
///                     .label("Background")
///                     .on_change(cx.listener(|this, color, _window, cx| {
///                         this.bg_color = *color;
///                         cx.notify();
///                     }))
///             )),
///         }
///     }
/// }
/// ```
pub struct ColorPickerState {
    id: ElementId,
    color: Hsla,
    label: Option<SharedString>,
    supports_opacity: bool,
    disabled: bool,
    is_open: bool,
    on_change: Option<Rc<dyn Fn(&Hsla, &mut Window, &mut App) + 'static>>,
    /// Hex input value (for text field)
    hex_input: String,
    /// Track bounds for slider positioning
    hue_slider_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    sat_slider_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    light_slider_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
    opacity_slider_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl ColorPickerState {
    /// Get the current color.
    pub fn color(&self) -> Hsla {
        self.color
    }

    /// Set the color programmatically.
    pub fn set_color(&mut self, color: Hsla) {
        self.color = color;
        self.hex_input = color_to_hex(color);
    }

    /// Open the color picker popover.
    pub fn open(&mut self) {
        if !self.disabled {
            self.is_open = true;
        }
    }

    /// Close the color picker popover.
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Handle clicking on the color well.
    fn handle_well_click(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.disabled {
            self.is_open = !self.is_open;
            cx.notify();
        }
    }

    /// Handle selecting a preset color.
    fn select_color(&mut self, color: Hsla, window: &mut Window, cx: &mut Context<Self>) {
        // Preserve alpha if not supporting opacity
        let new_color = if self.supports_opacity {
            color
        } else {
            Hsla { a: self.color.a, ..color }
        };

        self.color = new_color;
        self.hex_input = color_to_hex(new_color);

        if let Some(ref handler) = self.on_change {
            handler(&self.color, window, cx);
        }
        cx.notify();
    }

    /// Handle hue slider mouse down.
    fn handle_hue_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.hue_slider_bounds.get() {
            let relative_x = event.position.x - bounds.origin.x;
            let position = (relative_x / bounds.size.width) as f64;
            let position = position.clamp(0.0, 1.0);
            self.color.h = position as f32;
            self.hex_input = color_to_hex(self.color);

            if let Some(ref handler) = self.on_change {
                handler(&self.color, window, cx);
            }
            cx.notify();
        }
    }

    /// Handle saturation slider mouse down.
    fn handle_sat_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.sat_slider_bounds.get() {
            let relative_x = event.position.x - bounds.origin.x;
            let position = (relative_x / bounds.size.width) as f64;
            let position = position.clamp(0.0, 1.0);
            self.color.s = position as f32;
            self.hex_input = color_to_hex(self.color);

            if let Some(ref handler) = self.on_change {
                handler(&self.color, window, cx);
            }
            cx.notify();
        }
    }

    /// Handle lightness slider mouse down.
    fn handle_light_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.light_slider_bounds.get() {
            let relative_x = event.position.x - bounds.origin.x;
            let position = (relative_x / bounds.size.width) as f64;
            let position = position.clamp(0.0, 1.0);
            self.color.l = position as f32;
            self.hex_input = color_to_hex(self.color);

            if let Some(ref handler) = self.on_change {
                handler(&self.color, window, cx);
            }
            cx.notify();
        }
    }

    /// Handle opacity slider mouse down.
    fn handle_opacity_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if let Some(bounds) = self.opacity_slider_bounds.get() {
            let relative_x = event.position.x - bounds.origin.x;
            let position = (relative_x / bounds.size.width) as f64;
            let position = position.clamp(0.0, 1.0);
            self.color.a = position as f32;
            self.hex_input = color_to_hex(self.color);

            if let Some(ref handler) = self.on_change {
                handler(&self.color, window, cx);
            }
            cx.notify();
        }
    }

    /// Handle clicking outside the popover to dismiss.
    fn handle_dismiss(
        &mut self,
        _event: &ClickEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_open = false;
        cx.notify();
    }

    /// Build a color swatch for the preset grid.
    fn build_swatch(color: Hsla, index: usize, is_selected: bool) -> Stateful<Div> {
        let swatch = div()
            .id(("color-swatch", index))
            .size(px(20.0))
            .rounded(px(3.0))
            .bg(color)
            .border_1()
            .cursor_pointer();

        if is_selected {
            swatch
                .border_color(hsla(0.0, 0.0, 0.0, 0.8))
                .shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.3),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(2.0),
                }])
        } else {
            swatch
                .border_color(hsla(0.0, 0.0, 0.0, 0.15))
                .hover(|style| {
                    style.border_color(hsla(0.0, 0.0, 0.0, 0.4))
                })
        }
    }

    /// Build a slider track with gradient for visual feedback.
    /// Returns the track div that can have bounds tracking and event handlers attached.
    fn build_slider_track(
        id: impl Into<ElementId>,
        value: f32,
        gradient_colors: Vec<Hsla>,
    ) -> Stateful<Div> {
        let track_height = px(12.0);
        let track_width = px(180.0);
        let thumb_size = px(14.0);

        // Build gradient stops
        let mut track = div()
            .w(track_width)
            .h(track_height)
            .rounded(track_height)
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.70, 1.0))
            .overflow_hidden();

        // Simple gradient representation using segments
        let segment_count = gradient_colors.len();
        if segment_count > 0 {
            let mut gradient_div = div().flex().flex_row().size_full();
            for color in gradient_colors {
                gradient_div = gradient_div.child(
                    div().flex_grow().h_full().bg(color)
                );
            }
            track = track.child(gradient_div);
        }

        // Build the thumb - position using relative (percentage)
        // Adjust for thumb centering: thumb_size/2 = 7px offset at edges
        let thumb = div()
            .absolute()
            .top(px(-1.0))
            .left(relative(value))
            .ml(-thumb_size / 2.0) // Center the thumb
            .size(thumb_size)
            .rounded(thumb_size)
            .bg(hsla(0.0, 0.0, 1.0, 1.0))
            .border_1()
            .border_color(hsla(0.0, 0.0, 0.50, 1.0))
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.20),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        div()
            .id(id)
            .relative()
            .w(track_width)
            .h(thumb_size)
            .cursor_pointer()
            .child(track)
            .child(thumb)
    }
}

impl Render for ColorPickerState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let colors = if self.disabled {
            ColorPickerColors::disabled()
        } else {
            ColorPickerColors::new()
        };

        let color = self.color;
        let is_open = self.is_open;
        let supports_opacity = self.supports_opacity;

        // Build the color well
        let well = div()
            .id("color-well")
            .size(px(WELL_SIZE))
            .rounded(px(WELL_BORDER_RADIUS))
            .bg(color)
            .border_1()
            .border_color(if is_open {
                colors.well_border_hover
            } else {
                colors.well_border
            })
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.10),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        let well = if !self.disabled {
            well.cursor_pointer()
                .hover(move |style| style.border_color(colors.well_border_hover))
                .on_click(cx.listener(Self::handle_well_click))
        } else {
            well.cursor_default()
        };

        // Build the main container
        let mut container = div()
            .id(self.id.clone())
            .relative();

        // Add label if present
        let label_color = colors.label;
        let main_row = if let Some(ref label_text) = self.label {
            div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(well)
                .child(
                    div()
                        .text_sm()
                        .text_color(label_color)
                        .child(label_text.clone()),
                )
        } else {
            div()
                .flex()
                .flex_row()
                .items_center()
                .child(well)
        };

        container = container.child(main_row);

        // Build the popover if open
        if is_open {
            let current_color = self.color;

            // Build the preset color grid (6 columns)
            let mut grid = div()
                .flex()
                .flex_wrap()
                .gap(px(4.0))
                .w(px(148.0)); // 6 * 20 + 5 * 4 = 140 + 8 padding margin

            for (index, &preset_color) in PRESET_COLORS.iter().enumerate() {
                let is_selected = colors_approximately_equal(preset_color, current_color);
                let swatch = Self::build_swatch(preset_color, index, is_selected);

                // We need to capture the color for the click handler
                let preset = preset_color;
                grid = grid.child(
                    swatch.on_click(cx.listener(move |this, _event, window, cx| {
                        this.select_color(preset, window, cx);
                    }))
                );
            }

            // Build HSL sliders
            let hue_value = current_color.h;
            let sat_value = current_color.s;
            let light_value = current_color.l;
            let alpha_value = current_color.a;

            // Hue gradient (rainbow)
            let hue_colors: Vec<Hsla> = (0..12)
                .map(|i| hsla(i as f32 / 12.0, 0.8, 0.5, 1.0))
                .collect();

            // Saturation gradient (current hue, varying saturation)
            let sat_colors = vec![
                hsla(current_color.h, 0.0, current_color.l, 1.0),
                hsla(current_color.h, 0.5, current_color.l, 1.0),
                hsla(current_color.h, 1.0, current_color.l, 1.0),
            ];

            // Lightness gradient
            let light_colors = vec![
                hsla(current_color.h, current_color.s, 0.0, 1.0),
                hsla(current_color.h, current_color.s, 0.5, 1.0),
                hsla(current_color.h, current_color.s, 1.0, 1.0),
            ];

            // Build sliders with bounds tracking
            let track_width = px(180.0);
            let thumb_size = px(14.0);

            // Hue slider with bounds tracking
            let hue_bounds_ref = self.hue_slider_bounds.clone();
            let hue_bounds_tracker = canvas(
                |bounds, _window, _cx| bounds,
                move |bounds, _, _window, _cx| {
                    hue_bounds_ref.set(Some(bounds));
                },
            )
            .absolute()
            .size_full();

            let hue_slider_base = Self::build_slider_track("hue-slider", hue_value, hue_colors)
                .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_hue_mouse_down));
            let hue_slider = div()
                .relative()
                .w(track_width)
                .h(thumb_size)
                .child(hue_slider_base)
                .child(hue_bounds_tracker);

            // Saturation slider with bounds tracking
            let sat_bounds_ref = self.sat_slider_bounds.clone();
            let sat_bounds_tracker = canvas(
                |bounds, _window, _cx| bounds,
                move |bounds, _, _window, _cx| {
                    sat_bounds_ref.set(Some(bounds));
                },
            )
            .absolute()
            .size_full();

            let sat_slider_base = Self::build_slider_track("sat-slider", sat_value, sat_colors)
                .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_sat_mouse_down));
            let sat_slider = div()
                .relative()
                .w(track_width)
                .h(thumb_size)
                .child(sat_slider_base)
                .child(sat_bounds_tracker);

            // Lightness slider with bounds tracking
            let light_bounds_ref = self.light_slider_bounds.clone();
            let light_bounds_tracker = canvas(
                |bounds, _window, _cx| bounds,
                move |bounds, _, _window, _cx| {
                    light_bounds_ref.set(Some(bounds));
                },
            )
            .absolute()
            .size_full();

            let light_slider_base = Self::build_slider_track("light-slider", light_value, light_colors)
                .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_light_mouse_down));
            let light_slider = div()
                .relative()
                .w(track_width)
                .h(thumb_size)
                .child(light_slider_base)
                .child(light_bounds_tracker);

            let sliders = div()
                .flex()
                .flex_col()
                .gap(px(8.0))
                .child(
                    div().flex().flex_row().items_center().gap(px(8.0))
                        .child(div().w(px(12.0)).text_xs().text_color(hsla(0.0, 0.0, 0.40, 1.0)).child("H"))
                        .child(hue_slider)
                )
                .child(
                    div().flex().flex_row().items_center().gap(px(8.0))
                        .child(div().w(px(12.0)).text_xs().text_color(hsla(0.0, 0.0, 0.40, 1.0)).child("S"))
                        .child(sat_slider)
                )
                .child(
                    div().flex().flex_row().items_center().gap(px(8.0))
                        .child(div().w(px(12.0)).text_xs().text_color(hsla(0.0, 0.0, 0.40, 1.0)).child("L"))
                        .child(light_slider)
                );

            // Opacity slider if supported
            let sliders = if supports_opacity {
                let opacity_colors = vec![
                    hsla(current_color.h, current_color.s, current_color.l, 0.0),
                    hsla(current_color.h, current_color.s, current_color.l, 1.0),
                ];

                // Opacity slider with bounds tracking
                let opacity_bounds_ref = self.opacity_slider_bounds.clone();
                let opacity_bounds_tracker = canvas(
                    |bounds, _window, _cx| bounds,
                    move |bounds, _, _window, _cx| {
                        opacity_bounds_ref.set(Some(bounds));
                    },
                )
                .absolute()
                .size_full();

                let opacity_slider_base = Self::build_slider_track("opacity-slider", alpha_value, opacity_colors)
                    .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_opacity_mouse_down));
                let opacity_slider = div()
                    .relative()
                    .w(track_width)
                    .h(thumb_size)
                    .child(opacity_slider_base)
                    .child(opacity_bounds_tracker);

                sliders.child(
                    div().flex().flex_row().items_center().gap(px(8.0))
                        .child(div().w(px(12.0)).text_xs().text_color(hsla(0.0, 0.0, 0.40, 1.0)).child("A"))
                        .child(opacity_slider)
                )
            } else {
                sliders
            };

            // Hex color display
            let hex_display = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(8.0))
                .child(
                    div()
                        .text_xs()
                        .text_color(hsla(0.0, 0.0, 0.40, 1.0))
                        .child("Hex:")
                )
                .child(
                    div()
                        .px(px(6.0))
                        .py(px(2.0))
                        .bg(hsla(0.0, 0.0, 0.95, 1.0))
                        .border_1()
                        .border_color(hsla(0.0, 0.0, 0.80, 1.0))
                        .rounded(px(3.0))
                        .text_xs()
                        .font_family("monospace")
                        .text_color(hsla(0.0, 0.0, 0.20, 1.0))
                        .child(self.hex_input.clone())
                );

            // Color preview (larger swatch)
            let preview = div()
                .size(px(32.0))
                .rounded(px(4.0))
                .bg(current_color)
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.70, 1.0));

            // Build the popover panel content
            let panel_content = div()
                .p(px(12.0))
                .flex()
                .flex_col()
                .gap(px(12.0))
                .child(grid)
                .child(div().h(px(1.0)).bg(hsla(0.0, 0.0, 0.88, 1.0)))
                .child(sliders)
                .child(div().h(px(1.0)).bg(hsla(0.0, 0.0, 0.88, 1.0)))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .items_center()
                        .justify_between()
                        .child(hex_display)
                        .child(preview)
                );

            // Build the popover panel
            let panel = div()
                .absolute()
                .top_full()
                .left(px(0.0))
                .mt(px(4.0))
                .bg(hsla(0.0, 0.0, 1.0, 1.0))
                .rounded(px(8.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.78, 1.0))
                .shadow(vec![
                    BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.15),
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                    },
                ])
                .child(panel_content)
                .id("popover-panel")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    cx.stop_propagation();
                });

            // Dismiss overlay
            let overlay = div()
                .id("dismiss-overlay")
                .on_click(cx.listener(Self::handle_dismiss))
                .child(panel);

            container = container.child(overlay);
        }

        container
    }
}

/// Creates a ColorPickerState from a ColorPicker builder.
impl From<ColorPicker> for ColorPickerState {
    fn from(builder: ColorPicker) -> Self {
        let hex_input = color_to_hex(builder.color);
        ColorPickerState {
            id: builder.id,
            color: builder.color,
            label: builder.label,
            supports_opacity: builder.supports_opacity,
            disabled: builder.disabled,
            is_open: false,
            on_change: builder.on_change.map(|handler| {
                Rc::new(move |color: &Hsla, window: &mut Window, cx: &mut App| {
                    handler(*color, window, cx);
                }) as Rc<dyn Fn(&Hsla, &mut Window, &mut App) + 'static>
            }),
            hex_input,
            hue_slider_bounds: Rc::new(Cell::new(None)),
            sat_slider_bounds: Rc::new(Cell::new(None)),
            light_slider_bounds: Rc::new(Cell::new(None)),
            opacity_slider_bounds: Rc::new(Cell::new(None)),
        }
    }
}

/// Convert HSLA color to hex string.
fn color_to_hex(color: Hsla) -> String {
    // Convert HSL to RGB
    let (r, g, b) = hsl_to_rgb(color.h, color.s, color.l);

    if color.a < 1.0 {
        format!("#{:02X}{:02X}{:02X}{:02X}",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8,
            (color.a * 255.0) as u8
        )
    } else {
        format!("#{:02X}{:02X}{:02X}",
            (r * 255.0) as u8,
            (g * 255.0) as u8,
            (b * 255.0) as u8
        )
    }
}

/// Convert HSL to RGB.
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (f32, f32, f32) {
    if s == 0.0 {
        return (l, l, l);
    }

    let q = if l < 0.5 {
        l * (1.0 + s)
    } else {
        l + s - l * s
    };
    let p = 2.0 * l - q;

    let r = hue_to_rgb(p, q, h + 1.0 / 3.0);
    let g = hue_to_rgb(p, q, h);
    let b = hue_to_rgb(p, q, h - 1.0 / 3.0);

    (r, g, b)
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

/// Check if two colors are approximately equal (ignoring small floating point differences).
fn colors_approximately_equal(a: Hsla, b: Hsla) -> bool {
    const EPSILON: f32 = 0.01;
    (a.h - b.h).abs() < EPSILON
        && (a.s - b.s).abs() < EPSILON
        && (a.l - b.l).abs() < EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_picker_creation() {
        let color = hsla(0.5, 0.5, 0.5, 1.0);
        let picker = ColorPicker::new("test", color);
        assert_eq!(picker.color, color);
        assert!(!picker.disabled);
        assert!(!picker.supports_opacity);
    }

    #[test]
    fn test_color_picker_with_label() {
        let picker = ColorPicker::new("test", hsla(0.0, 0.0, 0.0, 1.0))
            .label("Background");
        assert_eq!(picker.label, Some("Background".into()));
    }

    #[test]
    fn test_color_picker_supports_opacity() {
        let picker = ColorPicker::new("test", hsla(0.0, 0.0, 0.0, 1.0))
            .supports_opacity(true);
        assert!(picker.supports_opacity);
    }

    #[test]
    fn test_color_picker_disabled() {
        let picker = ColorPicker::new("test", hsla(0.0, 0.0, 0.0, 1.0))
            .disabled(true);
        assert!(picker.disabled);
    }

    #[test]
    fn test_color_to_hex() {
        // Black
        let hex = color_to_hex(hsla(0.0, 0.0, 0.0, 1.0));
        assert_eq!(hex, "#000000");

        // White
        let hex = color_to_hex(hsla(0.0, 0.0, 1.0, 1.0));
        assert_eq!(hex, "#FFFFFF");

        // With alpha
        let hex = color_to_hex(hsla(0.0, 0.0, 0.0, 0.5));
        assert_eq!(hex, "#0000007F");
    }

    #[test]
    fn test_hsl_to_rgb() {
        // Black
        let (r, g, b) = hsl_to_rgb(0.0, 0.0, 0.0);
        assert_eq!((r, g, b), (0.0, 0.0, 0.0));

        // White
        let (r, g, b) = hsl_to_rgb(0.0, 0.0, 1.0);
        assert_eq!((r, g, b), (1.0, 1.0, 1.0));

        // Gray (no saturation)
        let (r, g, b) = hsl_to_rgb(0.0, 0.0, 0.5);
        assert_eq!((r, g, b), (0.5, 0.5, 0.5));
    }

    #[test]
    fn test_colors_approximately_equal() {
        let a = hsla(0.5, 0.5, 0.5, 1.0);
        let b = hsla(0.5, 0.5, 0.5, 1.0);
        assert!(colors_approximately_equal(a, b));

        let c = hsla(0.6, 0.5, 0.5, 1.0);
        assert!(!colors_approximately_equal(a, c));
    }

    #[test]
    fn test_color_picker_state_from_builder() {
        let color = hsla(0.5, 0.8, 0.5, 1.0);
        let picker = ColorPicker::new("test", color)
            .label("Test Color")
            .supports_opacity(true);

        let state: ColorPickerState = picker.into();
        assert_eq!(state.color, color);
        assert_eq!(state.label, Some("Test Color".into()));
        assert!(state.supports_opacity);
        assert!(!state.is_open);
    }
}
