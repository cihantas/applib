# HStack

A layout component that arranges child views in a horizontal line.

## Overview

HStack uses flexbox to arrange its children horizontally from left to right. Use HStack when you need to position elements side-by-side, such as buttons in a toolbar, labels with values, or navigation items.

```rust
HStack::new()
    .gap_3()
    .child(Button::new("save", "Save"))
    .child(Button::new("cancel", "Cancel"))
```

The stack automatically sizes itself to fit its children and provides convenient methods for controlling spacing and padding.

## Topics

### Creating an HStack

- `new()` — Creates a new horizontal stack with no gap or padding.

### Configuring Spacing

- `gap_3()` — Sets 12px spacing between children.
- `gap_6()` — Sets 24px spacing between children.
- `gap(_:)` — Sets custom spacing between children.

### Configuring Padding

- `p_1()` — Applies 4px padding around all children.
- `p_2()` — Applies 8px padding around all children.
- `p_3()` — Applies 12px padding around all children.
- `p_4()` — Applies 16px padding around all children.
- `p(_:)` — Applies custom padding around all children.

### Adding Content

- `child(_:)` — Adds a single child element to the stack.
- `children(_:)` — Adds multiple child elements to the stack.

## See Also

- VStack
- ZStack
- Spacer
