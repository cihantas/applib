# Popover

A transient view that appears above content, anchored to a specific element.

## Overview

Popover displays floating content attached to an anchor element, with an optional arrow pointing to the anchor. It can appear on any edge of the anchor (top, bottom, leading, or trailing) and supports both hover-triggered and click-triggered interactions.

Two variants are available:
- **Popover** — Hover-triggered popover that appears when hovering over the anchor
- **ControlledPopover** — Click-triggered popover controlled by external state

```rust
Popover::new("info-popover", info_button)
    .edge(PopoverEdge::Bottom)
    .content(|| {
        div()
            .p_4()
            .child("Additional information about this feature")
    })
```

Use popovers for supplementary information, tool palettes, or temporary controls that relate to a specific element without requiring a modal dialog.

## Topics

### Creating a Popover

- `new(_:_:)` — Creates a hover-triggered popover.
- `ControlledPopover::new(_:_:_:)` — Creates a popover controlled by external state.

### Positioning the Popover

- `edge(_:)` — Sets which edge of the anchor the popover appears on.
- `top()` — Positions the popover above the anchor.
- `bottom()` — Positions the popover below the anchor.
- `leading()` — Positions the popover to the left of the anchor.
- `trailing()` — Positions the popover to the right of the anchor.

### Configuring Content

- `content(_:)` — Sets the content to display in the popover.
- `show_arrow(_:)` — Sets whether to show the arrow pointing to the anchor.
- `without_arrow()` — Hides the arrow pointing to the anchor.

### Handling Dismissal

- `on_dismiss(_:)` — Sets the dismiss handler for ControlledPopover.

## Positioning Popovers

Popovers can appear on any edge of the anchor element. The default position is bottom:

```rust
// Default (bottom)
Popover::new("default", button)
    .content(|| div().child("Content"))

// Explicit positioning
Popover::new("top", button)
    .top()
    .content(|| div().child("Appears above"))

Popover::new("trailing", button)
    .trailing()
    .content(|| div().child("Appears to the right"))
```

Use `PopoverEdge` for dynamic positioning:

```rust
let edge = if has_space_below {
    PopoverEdge::Bottom
} else {
    PopoverEdge::Top
};

Popover::new("adaptive", button)
    .edge(edge)
    .content(|| div().child("Content"))
```

## Customizing Appearance

Control the arrow indicator:

```rust
// With arrow (default)
Popover::new("with-arrow", button)
    .content(|| div().child("Content"))

// Without arrow
Popover::new("no-arrow", button)
    .without_arrow()
    .content(|| div().child("Content"))
```

The popover automatically applies polished styling with rounded corners, borders, and shadows. The arrow is a small diamond shape that connects the popover to its anchor.

## Popover Content

The content builder receives no arguments and returns any element:

```rust
Popover::new("rich-content", trigger)
    .bottom()
    .content(|| {
        div()
            .flex()
            .flex_col()
            .gap_2()
            .p_4()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .child("Details")
            )
            .child(
                div()
                    .text_xs()
                    .text_color(hsla(0.0, 0.0, 0.5, 1.0))
                    .child("Additional information here")
            )
            .child(
                Button::new("action", "Take Action")
                    .on_click(|_window, cx| {
                        // Handle action
                    })
            )
    })
```

## Using ControlledPopover

For click-triggered popovers with explicit state management:

```rust
struct ToolPalette {
    popover_open: bool,
}

impl Render for ToolPalette {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ControlledPopover::new("tools", self.popover_open, tool_button)
            .bottom()
            .content(|| {
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .p_2()
                    .child(IconButton::new("brush", Icon::Brush))
                    .child(IconButton::new("eraser", Icon::Eraser))
                    .child(IconButton::new("fill", Icon::Fill))
            })
            .on_dismiss(cx.listener(|this, _event, _window, cx| {
                this.popover_open = false;
                cx.notify();
            }))
    }
}
```

The `on_dismiss` handler is called when clicking outside the popover, allowing you to update your state and close the popover.

## Guidelines

- Use popovers for lightweight, temporary content related to a specific UI element
- Keep popover content concise and focused on a single task
- Avoid nesting popovers or showing multiple popovers simultaneously
- For modal interactions or complex forms, use Sheet or Alert instead
- Position popovers to avoid covering critical interface elements
- The arrow helps users understand the relationship between the popover and its anchor

## See Also

- Menu
- ContextMenu
- Sheet
- Alert
