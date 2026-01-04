# ColorPicker

A control for selecting colors with a popover picker interface.

## Overview

ColorPicker provides a comprehensive color selection interface featuring a color well display, preset color grid, and HSL sliders for precise adjustment. The component supports optional opacity control and follows modern design aesthetics.

```rust
ColorPicker::new("bg-color", selected_color)
    .label("Background")
    .supports_opacity(true)
    .on_change(|new_color, window, cx| {
        println!("Color changed: {:?}", new_color);
    })
```

For full interactive support with popover, convert to `ColorPickerState`:

```rust
let color_picker = cx.new(|_| ColorPickerState::from(
    ColorPicker::new("my-color", hsla(0.5, 0.8, 0.5, 1.0))
        .label("Background")
        .on_change(cx.listener(|this, color, _window, cx| {
            this.bg_color = *color;
            cx.notify();
        }))
));
```

The picker displays a color well showing the current color and opens a popover with preset colors, sliders, and a hex value display.

## Topics

### Creating a Color Picker

- `new(_:_:)` — Creates a color picker with the given identifier and initial color.

### Configuring Behavior

- `label(_:)` — Sets the label text shown next to the color well.
- `supports_opacity(_:)` — Sets whether opacity adjustment is enabled.
- `disabled(_:)` — Sets whether the picker is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the color changes.

## Using Color Picker State

The basic `ColorPicker` type provides the color well display. For the full interactive popover experience, use `ColorPickerState`:

```rust
struct MyView {
    color_picker: Entity<ColorPickerState>,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            color_picker: cx.new(|_| ColorPickerState::from(
                ColorPicker::new("color", hsla(0.5, 0.8, 0.5, 1.0))
            )),
        }
    }
}
```

`ColorPickerState` provides:
- `color()` — Gets the current color.
- `set_color(_:)` — Sets the color programmatically.
- `open()` — Opens the color picker popover.
- `close()` — Closes the color picker popover.

## Picker Interface

The popover includes:
- **Preset Grid**: Common colors for quick selection
- **HSL Sliders**: Hue, saturation, and lightness controls
- **Opacity Slider**: Alpha channel control (when enabled)
- **Hex Display**: Current color as hexadecimal
- **Preview Well**: Large swatch showing the current color

## Opacity Support

Enable opacity control to allow alpha channel adjustment:

```rust
ColorPicker::new("overlay-color", color)
    .supports_opacity(true)
```

When disabled, the alpha channel remains at its current value while other components can be adjusted.

## Color Format

Colors use the HSLA (Hue, Saturation, Lightness, Alpha) format:

```rust
use gpui::*;

let color = hsla(
    211.0 / 360.0,  // Hue (0.0 - 1.0)
    0.95,           // Saturation (0.0 - 1.0)
    0.53,           // Lightness (0.0 - 1.0)
    1.0             // Alpha (0.0 - 1.0)
);
```

## See Also

- Picker
- Slider
