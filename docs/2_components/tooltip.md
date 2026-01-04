# Tooltip

A hover hint that appears near a wrapped element.

## Overview

Tooltip wraps any element and displays a text hint when the user hovers over it. The tooltip appears in a dark rounded rectangle with light text, following standard system tooltip styling. It can be positioned above, below, or to either side of the trigger element.

Use Tooltip to provide additional context or explanations for UI elements without cluttering the interface. Tooltips are particularly useful for icon-only buttons, truncated text, or any control that benefits from additional explanation.

```rust
Tooltip::new("save-btn-tooltip", save_button, "Save the current document")
```

With custom positioning:

```rust
Tooltip::new("tooltip-id", my_element, "Helpful hint")
    .position(TooltipPosition::Bottom)
```

Using convenience methods for position:

```rust
Tooltip::new("tooltip-id", my_element, "Top tooltip")
    .top()

Tooltip::new("tooltip-id", my_element, "Right tooltip")
    .right()
```

## Topics

### Creating a Tooltip

- `new(_:_:_:)` — Creates a new tooltip wrapping the given element.

### Configuring Position

- `position(_:)` — Sets the tooltip position relative to the wrapped element.
- `top()` — Positions the tooltip above the element.
- `bottom()` — Positions the tooltip below the element.
- `left()` — Positions the tooltip to the left of the element.
- `right()` — Positions the tooltip to the right of the element.

### Tooltip Positions

- `TooltipPosition::Top` — Tooltip appears above the element (default).
- `TooltipPosition::Bottom` — Tooltip appears below the element.
- `TooltipPosition::Left` — Tooltip appears to the left of the element.
- `TooltipPosition::Right` — Tooltip appears to the right of the element.

### Advanced Usage

- `TooltipState` — A stateful wrapper for elements that need hover delay tracking.
- `TooltipState::new()` — Creates a new tooltip state.
- `on_mouse_enter()` — Call when mouse enters the trigger element.
- `on_mouse_leave()` — Call when mouse leaves the trigger element.
- `update(_:)` — Check if tooltip should be shown based on hover duration.

## See Also

- Button
- Label
- Text
