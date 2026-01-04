# VStack

A layout component that arranges child views in a vertical line.

## Overview

VStack uses flexbox to arrange its children vertically from top to bottom. Use VStack when you need to stack elements vertically, such as form fields, list items, or sections of content.

```rust
VStack::new()
    .gap_3()
    .child(div().child("Header"))
    .child(div().child("Content"))
    .child(div().child("Footer"))
```

The stack automatically sizes itself to fit its children and provides convenient methods for controlling spacing and padding.

## Topics

### Creating a VStack

- `new()` — Creates a new vertical stack with no gap or padding.

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

- HStack
- ZStack
- Spacer
