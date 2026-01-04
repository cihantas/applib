# Toggle

A control for boolean on/off states with checkbox and switch variants.

## Overview

Toggle provides a two-state control for binary choices. The component supports both checkbox style (square box with checkmark) and switch style (sliding pill), following modern design patterns.

```rust
Toggle::new("notifications", "Enable notifications", is_enabled)
    .on_change(cx.listener(|this, is_on, _window, cx| {
        this.notifications_enabled = *is_on;
        cx.notify();
    }))
```

The toggle automatically manages its visual state and provides clear feedback for the current selection.

## Topics

### Creating a Toggle

- `new(_:_:_:)` — Creates a toggle with the given identifier, label, and initial state.

### Configuring Style

- `style(_:)` — Sets the visual style variant.

### Configuring State

- `disabled(_:)` — Sets whether the toggle is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the state changes.

## Toggle Styles

### Checkbox

Square box with checkmark, the classic the platform style. This is the default.

```rust
Toggle::new("option", "Enable feature", false)
    .style(ToggleStyle::Checkbox)
```

### Switch

Sliding pill with circular indicator, suitable for settings that take effect immediately.

```rust
Toggle::new("dark-mode", "Dark mode", is_dark)
    .style(ToggleStyle::Switch)
```

## Usage Guidelines

Use checkbox style for options in forms or dialogs where changes are applied on confirmation. Use switch style for settings that apply immediately, such as enabling or disabling a feature.

## See Also

- Checkbox
- RadioGroup
