# TextField

A single-line text input component for collecting short text values.

## Overview

TextField provides a focused input experience for single-line text entry. Use TextField for branch names, search queries, usernames, and other short text inputs where multi-line content is not appropriate.

```rust
// Using a binding (recommended):
TextField::new("search", cx)
    .text(State::binding(&self.query, cx))
    .placeholder("Search...")

// Using value + on_change (legacy):
TextField::new("branch-name", cx)
    .label("Branch name")
    .placeholder("feature/...")
    .value(&self.branch_name)
    .on_change(|text| {
        println!("Text changed: {}", text);
    })
```

The component provides keyboard navigation with standard shortcuts, displays an optional label above the input, and shows placeholder text when empty. When focused, TextField displays a cursor indicator and a blue focus ring following modern design aesthetics.

TextField supports two-way bindings via `text(_:)`, enabling automatic synchronization with [`State<String>`](../1_state/state.md). This is the recommended approach for modern reactive UIs.

## Topics

### Creating a TextField

- `new(_:_:)` — Creates a new text field with the given identifier.

### Configuring Content

- `text(_:)` — Sets a two-way binding for reactive state synchronization (recommended).
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

- [State<T>](../1_state/state.md) — Observable state for reactive bindings
- [Binding<T>](../1_state/binding.md) — Two-way binding primitive
- SecureField
- TextArea
