# Slider

A control for selecting a value from a continuous range.

## Overview

Slider provides an interactive control for selecting numeric values within a specified range. The component features a draggable thumb along a track with optional step increments and visual feedback.

```rust
Slider::new("volume-slider", 50.0, 0.0..=100.0)
    .step(1.0)
    .label("Volume")
    .on_change(|new_value, _window, _cx| {
        println!("Value changed to: {}", new_value);
    })
```

For full mouse drag support, convert the builder to `SliderState`:

```rust
let slider_entity = cx.new(|_| SliderState::from(
    Slider::new("volume", self.volume, 0.0..=100.0)
        .on_change(cx.listener(|this, value, _window, cx| {
            this.volume = *value;
            cx.notify();
        }))
));
```

The slider displays a filled track indicating the current value and provides smooth dragging interaction.

## Topics

### Creating a Slider

- `new(_:_:_:)` — Creates a slider with the given identifier, initial value, and range.

### Configuring Behavior

- `step(_:)` — Sets the step increment for value snapping.
- `label(_:)` — Sets the label text shown next to the slider.
- `disabled(_:)` — Sets whether the slider is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the value changes.

## Using Slider State

The basic `Slider` type provides visual display but limited interaction. For full drag support, use `SliderState`:

```rust
// In your view
struct MyView {
    slider: Entity<SliderState>,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            slider: cx.new(|_| SliderState::from(
                Slider::new("my-slider", 50.0, 0.0..=100.0)
            )),
        }
    }
}
```

`SliderState` provides:
- `value()` — Gets the current value.
- `set_value(_:)` — Sets the value programmatically.

## Step Increments

When a step value is set, the slider snaps to the nearest step increment, making it easier to select precise values:

```rust
Slider::new("opacity", 1.0, 0.0..=1.0)
    .step(0.1) // Snaps to 0.0, 0.1, 0.2, etc.
```

## See Also

- Stepper
- Picker
