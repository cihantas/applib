# TrafficLights

The iconic red, yellow, and green window control buttons from the platform.

## Overview

TrafficLights displays the three colored circular buttons that appear in the top-left corner of the platform windows. Each button is styled with a polished glass-ball aesthetic, featuring subtle shadows, highlights, and inset effects.

```rust
use applib::components::TrafficLights;

TrafficLights::new()
    .on_close(cx.listener(|_, _, _, cx| cx.quit()))
    .on_minimize(cx.listener(|_, _, window, _| window.minimize()))
    .on_maximize(cx.listener(|_, _, window, _| window.toggle_fullscreen()))
```

The buttons follow the platform color conventions: red for close, yellow for minimize, and green for maximize/zoom. Each button is 12x12 pixels with 8 pixels of spacing between them.

## Visual Design

Each button features layered styling to achieve the glass-ball effect:

- Base color fill with appropriate hue (red, yellow, or green)
- Dark border ring for depth
- Outer shadow for inset appearance
- Bottom white highlight
- Top inner shadow (darker at top)
- Top shine/highlight (glass reflection)
- Bottom highlight (brighter at bottom)

## Topics

### Creating TrafficLights

- `new()` — Creates new traffic light buttons.

### Handling Button Actions

- `on_close(_:)` — Sets the handler for the close button (red).
- `on_minimize(_:)` — Sets the handler for the minimize button (yellow).
- `on_maximize(_:)` — Sets the handler for the maximize/zoom button (green).

## Color Values

The component uses authentic platform colors:

- **Close (Red)**: HSL 4°, 70% saturation, 52% lightness
- **Minimize (Yellow)**: HSL 42°, 85% saturation, 55% lightness
- **Maximize (Green)**: HSL 120°, 55% saturation, 45% lightness

## Usage Notes

Button handlers are optional. If a handler is not provided, the button will still render but clicking it will have no effect. This allows you to selectively enable only the window controls you need.

All handlers receive `&ClickEvent`, `&mut Window`, and `&mut App` parameters for maximum flexibility in responding to button clicks.

## See Also

- TitleBar
- WindowFrame
