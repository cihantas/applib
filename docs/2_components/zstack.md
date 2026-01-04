# ZStack

A layout component that overlays child views on the z-axis.

## Overview

ZStack layers its children on top of each other, with the first child at the bottom and subsequent children stacked above. Use ZStack when you need to overlay elements, such as placing text on an image, showing badges on icons, or creating card overlays.

```rust
ZStack::new()
    .alignment(ZStackAlignment::Center)
    .child(
        div()
            .size_full()
            .bg(gpui::blue())
    )
    .child(Text::new("Overlay"))
```

Children are positioned according to the stack's alignment. By default, all children are centered within the stack's bounds.

## Topics

### Creating a ZStack

- `new()` — Creates a new z-axis stack with center alignment.

### Configuring Alignment

- `alignment(_:)` — Sets the alignment for all children using `ZStackAlignment`.

### ZStackAlignment Options

- `TopLeading` — Aligns children to the top-left corner.
- `Top` — Aligns children to the top center.
- `TopTrailing` — Aligns children to the top-right corner.
- `Leading` — Aligns children to the center-left.
- `Center` — Aligns children to the center (default).
- `Trailing` — Aligns children to the center-right.
- `BottomLeading` — Aligns children to the bottom-left corner.
- `Bottom` — Aligns children to the bottom center.
- `BottomTrailing` — Aligns children to the bottom-right corner.

### Configuring Padding

- `p_1()` — Applies 4px padding around all children.
- `p_2()` — Applies 8px padding around all children.
- `p_3()` — Applies 12px padding around all children.
- `p_4()` — Applies 16px padding around all children.
- `p(_:)` — Applies custom padding around all children.

### Adding Content

- `child(_:)` — Adds a child element to the stack, layered on top of previous children.
- `children(_:)` — Adds multiple child elements to the stack in order.

## See Also

- HStack
- VStack
