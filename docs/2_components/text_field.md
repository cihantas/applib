# TextField

A single-line text input component for collecting short text values.

## Overview

TextField provides a focused input experience for single-line text entry. Use TextField for branch names, search queries, usernames, and other short text inputs where multi-line content is not appropriate.

```rust
TextField::new("branch-name", cx)
    .label("Branch name")
    .placeholder("feature/...")
    .value(&self.branch_name)
    .on_change(|text| {
        println!("Text changed: {}", text);
    })
```

The component provides keyboard navigation with standard shortcuts, displays an optional label above the input, and shows placeholder text when empty. When focused, TextField displays a cursor indicator and a blue focus ring following modern design aesthetics.

## Topics

### Creating a TextField

- `new(_:_:)` — Creates a new text field with the given identifier.

### Configuring Content

- `value(_:)` — Sets the current text value.
- `placeholder(_:)` — Sets the placeholder text shown when empty.
- `label(_:)` — Sets the label text shown above the input.

### Handling User Input

- `on_change(_:)` — Sets the change handler called when text changes.

### Managing State

The `TextFieldState` view provides programmatic control:

- `value()` — Gets the current text value.
- `set_value(_:)` — Sets the text value programmatically.
- `clear()` — Clears the text field.
- `focus(_:)` — Focuses the text field.

## Keyboard Shortcuts

- Left/Right Arrow — Move cursor
- Home/End — Move to start/end of text
- Backspace/Delete — Remove characters
- Cmd+A — Move cursor to end (select all)
- Tab — Move focus to next field

## See Also

- SecureField
- TextArea
