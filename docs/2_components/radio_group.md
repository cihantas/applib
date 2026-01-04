# RadioGroup

A control for mutually exclusive option selection.

## Overview

RadioGroup provides a set of radio buttons where only one option can be selected at a time. The component follows modern design patterns with circular indicators and blue selection highlighting.

```rust
RadioGroup::new("local-changes")
    .label("Local changes")
    .option("none", "Don't change")
    .option("stash", "Stash and reapply")
    .option("discard", "Discard")
    .selected("none")
    .on_change(cx.listener(|this, value, _window, cx| {
        this.local_changes = value.to_string();
        cx.notify();
    }))
```

Radio buttons display clear visual feedback showing which option is currently selected.

## Topics

### Creating a Radio Group

- `new(_:)` — Creates a radio group with the given identifier.

### Configuring Options

- `label(_:)` — Sets the label displayed above the options.
- `option(_:_:)` — Adds an option to the group.
- `selected(_:)` — Sets the currently selected value.

### Configuring State

- `disabled(_:)` — Sets whether the radio group is disabled.

### Handling Changes

- `on_change(_:)` — Registers a handler called when the selection changes.

## Usage Guidelines

Use radio groups when the user must choose exactly one option from a set of mutually exclusive choices. All options should be visible at once. If you have many options or if multiple selections are allowed, consider using a Picker or Checkbox list instead.

## See Also

- Checkbox
- Picker
- Toggle
