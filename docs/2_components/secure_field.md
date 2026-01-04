# SecureField

A secure text input component that masks characters for password entry.

## Overview

SecureField provides a single-line text input that displays bullet characters instead of the actual text. Use SecureField for passwords, API keys, tokens, and other sensitive information that should not be visible on screen.

```rust
SecureField::new("password", cx)
    .label("Password")
    .placeholder("Enter password")
    .value(&self.password)
    .on_change(|text| {
        println!("Password changed");
    })
    .show_toggle(true)
```

The component masks all characters as bullets (•) by default. An optional toggle button allows users to temporarily reveal the actual text for verification purposes.

## Topics

### Creating a SecureField

- `new(_:_:)` — Creates a new secure field with the given identifier.

### Configuring Content

- `value(_:)` — Sets the current text value.
- `placeholder(_:)` — Sets the placeholder text shown when empty.
- `label(_:)` — Sets the label text shown above the input.

### Configuring Visibility

- `show_toggle(_:)` — Enables the show/hide toggle button.

### Handling User Input

- `on_change(_:)` — Sets the change handler called when text changes.

### Managing State

The `SecureFieldState` view provides programmatic control:

- `value()` — Gets the current text value.
- `set_value(_:)` — Sets the text value programmatically.
- `clear()` — Clears the text field.
- `is_revealed()` — Checks if the password is currently revealed.
- `toggle_visibility()` — Toggles password visibility.

## Keyboard Shortcuts

- Left/Right Arrow — Move cursor
- Home/End — Move to start/end of text
- Backspace/Delete — Remove characters
- Cmd+A — Move cursor to end (select all)
- Tab — Move focus to next field

## See Also

- TextField
- TextArea
