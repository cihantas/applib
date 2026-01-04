# ColorView

A view that displays a solid color rectangle.

## Overview

ColorView renders as a colored rectangle that expands to fill available space. It's useful for backgrounds in layered layouts, visual placeholders, or anywhere you need a simple block of color.

Use ColorView when you need a solid color background or a color swatch. The component provides convenient constructors for common colors and supports custom opacity.

```rust
// Custom color
ColorView::new(hsla(0.6, 0.8, 0.5, 1.0))

// Predefined colors
ColorView::blue()
ColorView::red().opacity(0.5)
```

ColorView works particularly well in ZStack layouts for creating layered backgrounds:

```rust
ZStack::new()
    .child(ColorView::blue().opacity(0.1))
    .child(content)
```

## Topics

### Creating a Color View

- `new(_:)` — Creates a new color view with the given color.

### Configuring Appearance

- `opacity(_:)` — Sets the opacity of the color view.

### Predefined Colors

- `blue()` — Creates a blue color view.
- `red()` — Creates a red color view.
- `green()` — Creates a green color view.
- `yellow()` — Creates a yellow color view.
- `orange()` — Creates an orange color view.
- `purple()` — Creates a purple color view.
- `pink()` — Creates a pink color view.
- `gray()` — Creates a gray color view.
- `white()` — Creates a white color view.
- `black()` — Creates a black color view.
- `clear()` — Creates a clear (transparent) color view.

## See Also

- Canvas
- Divider
