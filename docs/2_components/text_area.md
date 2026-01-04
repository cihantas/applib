# TextArea

A multi-line text input component for collecting longer text content.

## Overview

TextArea provides a resizable input area for multi-line text entry. Use TextArea for commit messages, descriptions, comments, and other content where users need multiple lines and paragraph breaks.

```rust
TextArea::new("commit-message", cx)
    .placeholder("Enter commit message...")
    .rows(4)
    .on_change(|text| {
        println!("Text changed: {}", text);
    })
```

The component supports line breaks with the Enter key, handles Tab key insertion as spaces, and provides line-aware Home/End navigation. The visible height is controlled by the `rows` parameter, which acts as a height hint based on line height.

## Topics

### Creating a TextArea

- `new(_:_:)` — Creates a new text area with the given identifier.

### Configuring Content

- `value(_:)` — Sets the current text value.
- `placeholder(_:)` — Sets the placeholder text shown when empty.

### Configuring Appearance

- `rows(_:)` — Sets the number of visible rows (height hint).

### Handling User Input

- `on_change(_:)` — Sets the change handler called when text changes.

### Managing State

The `TextAreaState` view provides programmatic control:

- `value()` — Gets the current text value.
- `set_value(_:)` — Sets the text value programmatically.
- `clear()` — Clears the text area.

## Keyboard Shortcuts

- Left/Right Arrow — Move cursor
- Home/End — Move to start/end of current line
- Backspace/Delete — Remove characters
- Enter — Insert line break
- Tab — Insert four spaces
- Cmd+A — Move cursor to end (select all)

## See Also

- TextField
- SecureField
