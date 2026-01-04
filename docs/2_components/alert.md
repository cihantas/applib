# Alert

A modal dialog for displaying important messages with action buttons.

## Overview

Alert displays critical information in a centered dialog with polished styling. Alerts include an optional icon, title, message text, and action buttons arranged following the platform conventions (Cancel on left, primary/destructive actions on right).

```rust
use applib::components::{Alert, AlertButton, AlertIcon};

Alert::new("Delete Item?")
    .message("This action cannot be undone.")
    .icon(AlertIcon::Warning)
    .button(AlertButton::cancel("Cancel"))
    .button(AlertButton::destructive("Delete", |_, _, _| {
        // Handle delete
    }))
```

Alerts are 320 pixels wide and automatically size vertically to fit their content. Clicking the backdrop dismisses the alert.

## Alert Icons

Alerts support three icon types, each with appropriate color and symbol:

```rust
Alert::new("Information")
    .icon(AlertIcon::Info)    // Blue ℹ

Alert::new("Warning")
    .icon(AlertIcon::Warning) // Yellow ⚠

Alert::new("Error Occurred")
    .icon(AlertIcon::Error)   // Red ⊘
```

Icons appear in a 48x48 pixel circular badge with 15% opacity background tint.

## Alert Buttons

AlertButton provides three role-based styles:

```rust
// Default button (blue, for primary actions)
AlertButton::new("OK")
    .on_click(|_, _, _| { /* ... */ })

// Cancel button (gray, secondary styling)
AlertButton::cancel("Cancel")
    .on_click(|_, _, _| { /* ... */ })

// Destructive button (red, for dangerous actions)
AlertButton::destructive("Delete", |_, _, _| {
    // Handler is required
})
```

## Topics

### Creating an Alert

- `new(_:)` — Creates a new alert with the given title.

### Configuring Appearance

- `id(_:)` — Sets the element ID for the alert.
- `message(_:)` — Sets the message text displayed below the title.
- `icon(_:)` — Sets the icon displayed in the alert.

### Adding Buttons

- `button(_:)` — Adds a button to the alert.

### Handling Dismissal

- `on_dismiss(_:)` — Sets the dismiss handler, called when the backdrop is clicked.

### AlertButton Methods

- `AlertButton::new(_:)` — Creates a new alert button with the given label.
- `AlertButton::cancel(_:)` — Creates a cancel button.
- `AlertButton::destructive(_:_:)` — Creates a destructive action button.
- `on_click(_:)` — Sets the click handler for the button.

## Button Ordering

Following the platform conventions, arrange buttons from least to most important, left to right:

```rust
Alert::new("Save changes?")
    .button(AlertButton::cancel("Don't Save"))
    .button(AlertButton::new("Cancel"))
    .button(AlertButton::new("Save").on_click(|_, _, _| { /* save */ }))
```

## See Also

- Sheet
- AlertIcon
- AlertButton
