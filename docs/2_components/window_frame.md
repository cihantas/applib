# WindowFrame

A container that provides resize handles for client-side decorated windows.

## Overview

WindowFrame wraps your window content with invisible resize handles positioned at all edges and corners, enabling users to resize windows that use client-side decorations. The resize handles have a 6-pixel hit area and display appropriate cursor styles when hovered.

When a user clicks and drags a resize handle, the window manager receives the appropriate resize edge signal (Top, Bottom, Left, Right, TopLeft, TopRight, BottomLeft, or BottomRight).

```rust
use applib::components::WindowFrame;

WindowFrame::new()
    .child(your_window_content)
```

Use WindowFrame as the root container for windows that need resize functionality without system-provided window chrome. The component handles all resize interaction logic internally, including cursor changes and edge detection.

## Topics

### Creating a WindowFrame

- `new()` — Creates a new window frame.

### Managing Content

- `child(_:)` — Adds a child element to the frame content.
- `children(_:)` — Adds multiple children to the frame content.

## Implementation Details

The component creates eight invisible resize handles:

- Four edge handles (top, bottom, left, right) with 6px width/height
- Four corner handles (top-left, top-right, bottom-left, bottom-right) with 6x6px dimensions

Each handle is positioned absolutely and responds to left mouse button presses by calling the GPUI window's `start_window_resize()` method with the appropriate edge.

## See Also

- TitleBar
- Panel
