# Button

A modern button component for primary and secondary actions.

## Overview

The Button component provides a standard clickable control for triggering actions in your interface. Buttons feature native styling with support for primary (blue) and secondary (gray) variants, disabled states, and hover effects.

```rust
Button::new("save-button", "Save")
    .primary()
    .on_click(|_event, _window, cx| {
        // Handle save action
    })
```

Buttons automatically handle visual feedback for hover and active states, following modern design patterns with subtle shadows and color transitions.

## Topics

### Creating a Button

- `new(_:_:)` — Creates a button with the given identifier and label.

### Configuring Style

- `style(_:)` — Sets the button style variant.
- `primary()` — Configures the button with primary (blue) styling.
- `secondary()` — Configures the button with secondary (gray) styling.

### Configuring State

- `disabled(_:)` — Sets whether the button is disabled.

### Handling Actions

- `on_click(_:)` — Registers a handler called when the button is clicked.

## Button Styles

### Primary

Blue buttons indicate the primary action in a context, such as saving changes or confirming an operation. Use sparingly—typically only one primary button per view.

```rust
Button::new("confirm-btn", "Confirm")
    .primary()
```

### Secondary

Gray buttons represent alternative or cancel actions. These are the default style.

```rust
Button::new("cancel-btn", "Cancel")
    .secondary()
```

## See Also

- IconButton
- Toggle
