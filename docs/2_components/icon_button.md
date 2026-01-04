# IconButton

A compact icon-only button for toolbars and inline actions.

## Overview

IconButton provides a minimal button control displaying only an icon, ideal for toolbar actions or inline controls next to text. The component supports ghost and filled styles, with optional tooltips for accessibility.

```rust
IconButton::new("add-btn", "+")
    .tooltip("Add item")
    .on_click(|_event, _window, cx| {
        // Handle add action
    })
```

Icon buttons automatically show hover feedback and support two size variants for different contexts.

## Topics

### Creating an Icon Button

- `new(_:_:)` — Creates an icon button with the given identifier and icon.

### Configuring Appearance

- `style(_:)` — Sets the button style variant.
- `ghost()` — Configures the button with ghost (minimal) styling.
- `filled()` — Configures the button with filled (prominent) styling.
- `size(_:)` — Sets the button size.

### Configuring Behavior

- `tooltip(_:)` — Sets the tooltip text shown on hover.
- `disabled(_:)` — Sets whether the button is disabled.

### Handling Actions

- `on_click(_:)` — Registers a handler called when the button is clicked.

## Button Sizes

### Small

20×20 pixels, suitable for inline use next to text elements.

```rust
IconButton::new("inline-btn", "×")
    .size(IconButtonSize::Small)
```

### Medium

24×24 pixels, appropriate for toolbar placement.

```rust
IconButton::new("toolbar-btn", "+")
    .size(IconButtonSize::Medium)
```

## Button Styles

### Ghost

Minimal chrome with transparent background, appearing only on hover. The default style.

```rust
IconButton::new("subtle-btn", "...")
    .ghost()
```

### Filled

More prominent with a visible background, suitable for primary toolbar actions.

```rust
IconButton::new("important-btn", "+")
    .filled()
```

## Icon Support

Currently accepts text characters like "+", "-", "×", "✓". This API is designed to evolve to support icon enums or SVG in future versions.

## See Also

- Button
- Stepper
