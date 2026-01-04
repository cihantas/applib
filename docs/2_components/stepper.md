# Stepper

A control for incrementing and decrementing numeric values.

## Overview

Stepper provides a segmented control with increment (+) and decrement (−) buttons for adjusting integer values within a specified range. The component follows modern design patterns with a clean segmented appearance.

```rust
Stepper::new("quantity-stepper", 5, 1..=10)
    .step(1)
    .label("Quantity")
    .on_change(|new_value, _window, _cx| {
        println!("Value changed to: {}", new_value);
    })
```

The stepper automatically disables increment and decrement buttons when the value reaches the range boundaries.

## Topics

### Creating a Stepper

- `new(_:_:_:)` — Creates a stepper with the given identifier, initial value, and range.

### Configuring Behavior

- `step(_:)` — Sets the increment/decrement amount.
- `label(_:)` — Sets the label text shown next to the stepper.
- `disabled(_:)` — Sets whether the stepper is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the value changes.

## Usage Guidelines

Use steppers for small numeric adjustments where the range is limited and the user needs precise control. For larger ranges or when fine control is less important, consider using a Slider instead.

The stepper ensures values never exceed the specified range—increment and decrement buttons automatically disable when limits are reached.

## Step Amount

The step parameter controls how much the value changes with each button press:

```rust
Stepper::new("rotation", 0, 0..=360)
    .step(15) // Adjusts in 15-degree increments
```

## See Also

- Slider
- TextField (for direct numeric input)
