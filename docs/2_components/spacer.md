# Spacer

A flexible space that expands to fill available space in stack layouts.

## Overview

Spacer uses flexbox growth to push elements apart within HStack and VStack containers. Use Spacer when you need to create flexible spacing that adapts to the available space, such as pushing a button to the right edge of a toolbar or centering content between fixed elements.

```rust
HStack::new()
    .child(div().child("Left"))
    .child(Spacer::new())
    .child(div().child("Right"))
```

This positions "Left" at the start and "Right" at the end, with the Spacer expanding to fill the space between them.

You can set a minimum length to ensure the Spacer maintains at least a specific size:

```rust
VStack::new()
    .child(div().child("Top"))
    .child(Spacer::new().min_length(px(20.0)))
    .child(div().child("Bottom"))
```

## Topics

### Creating a Spacer

- `new()` — Creates a spacer that expands to fill available space.

### Configuring Size

- `min_length(_:)` — Sets the minimum width and height of the spacer.

## See Also

- HStack
- VStack
