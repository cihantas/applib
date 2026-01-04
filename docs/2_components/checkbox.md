# Checkbox

A control for boolean selection with a label.

## Overview

Checkbox provides a standard checkbox control with an associated label. The component follows modern design aesthetics with a blue checkmark when selected.

```rust
Checkbox::new("checkout-after-create", "Check out after create")
    .checked(self.checkout_after_create)
    .on_change(cx.listener(|this, checked, _window, cx| {
        this.checkout_after_create = *checked;
        cx.notify();
    }))
```

Checkboxes display clear visual feedback for checked, unchecked, and disabled states.

## Topics

### Creating a Checkbox

- `new(_:_:)` — Creates a checkbox with the given identifier and label.

### Configuring State

- `checked(_:)` — Sets whether the checkbox is checked.
- `disabled(_:)` — Sets whether the checkbox is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the checked state changes.

## Usage Guidelines

Use checkboxes for individual binary choices in forms, settings, or lists. For mutually exclusive options, use RadioGroup instead. For settings that apply immediately, consider using Toggle with switch style.

## See Also

- Toggle
- RadioGroup
