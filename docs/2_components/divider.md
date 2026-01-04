# Divider

A visual separator line component.

## Overview

Divider creates a thin line to separate content visually. It can be oriented horizontally for use in vertical stacks, or vertically for use in horizontal stacks.

Use Divider to create clear visual separation between sections of your interface, such as between items in a menu, sections in a sidebar, or groups in a toolbar.

```rust
VStack::new()
    .child(div().child("Above"))
    .child(Divider::horizontal())
    .child(div().child("Below"))
```

For vertical separation in horizontal layouts:

```rust
HStack::new()
    .child(div().child("Left"))
    .child(Divider::vertical())
    .child(div().child("Right"))
```

## Topics

### Creating a Divider

- `horizontal()` — Creates a new horizontal divider.
- `vertical()` — Creates a new vertical divider.

### Configuring Appearance

- `color(_:)` — Sets a custom color for the divider.
- `thickness(_:)` — Sets a custom thickness for the divider.

### Divider Orientation

- `DividerOrientation::Horizontal` — Horizontal line (for use in VStack).
- `DividerOrientation::Vertical` — Vertical line (for use in HStack).

## See Also

- VStack
- HStack
