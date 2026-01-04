# TitleBar

A polished window title bar with traffic lights and drag support.

## Overview

TitleBar provides a window title bar with a polished appearance, featuring a subtle gradient background, traffic light buttons (close, minimize, maximize), and centered title text. The title bar supports window dragging and is 22 pixels in height.

```rust
use applib::components::TitleBar;

TitleBar::new("My Application")
    .on_close(cx.listener(|_, _, _, cx| cx.quit()))
    .on_minimize(cx.listener(|_, _, window, _| window.minimize()))
```

The title bar automatically handles window movement when users click and drag the central title area or right spacer. Traffic light buttons are positioned on the left following the platform conventions.

## Visual Design

The title bar features a polished design:

- Gradient background from #F3EEF0 (top) to #BDBDBD (bottom)
- 1px bright highlight line at the top edge
- Bottom border for depth
- Title text with embossed effect (dark text with white shadow 1px below)
- Traffic light buttons with glass-ball styling

## Topics

### Creating a TitleBar

- `new(_:)` — Creates a new title bar with the given title.

### Configuring Interaction

- `draggable(_:)` — Sets whether the title bar can be dragged to move the window.

### Handling Button Actions

- `on_close(_:)` — Sets the handler for the close button.
- `on_minimize(_:)` — Sets the handler for the minimize button.
- `on_maximize(_:)` — Sets the handler for the maximize button.

## Usage Notes

Window dragging is enabled by default. The draggable area includes both the center title region and the right spacer, but excludes the traffic light button area to prevent accidental window movement when clicking buttons.

Button handlers receive `&ClickEvent`, `&mut Window`, and `&mut App` parameters, allowing you to perform window operations or update application state.

## See Also

- TrafficLights
- WindowFrame
- Panel
