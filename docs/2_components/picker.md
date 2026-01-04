# Picker

A control for selecting from multiple options with various display styles.

## Overview

Picker provides a versatile selection control supporting three distinct presentation styles: dropdown menu, segmented control, and inline list. The component adapts to different use cases while maintaining consistent the platform styling.

```rust
Picker::new("color-picker", selected_index)
    .label("Color")
    .option("Red")
    .option("Blue")
    .option("Green")
    .style(PickerStyle::Menu)
    .on_change(|new_index, window, cx| {
        // Handle selection change
    })
```

Each style serves different contexts—menus for space-constrained layouts, segmented controls for frequently accessed options, and inline lists for settings panels.

## Topics

### Creating a Picker

- `new(_:_:)` — Creates a picker with the given identifier and selected index.

### Configuring Options

- `label(_:)` — Sets the label displayed with the picker.
- `option(_:)` — Adds an option to the picker.

### Configuring Style

- `style(_:)` — Sets the display style variant.
- `menu()` — Configures the picker as a dropdown menu.
- `segmented()` — Configures the picker as a horizontal button group.
- `inline()` — Configures the picker as a vertical list.

### Configuring State

- `disabled(_:)` — Sets whether the picker is disabled.
- `is_open(_:)` — Sets whether the menu is open (Menu style only).

### Handling Changes

- `on_change(_:)` — Registers a handler called when the selection changes.
- `on_toggle(_:)` — Registers a handler called when the menu opens or closes (Menu style only).

## Picker Styles

### Menu

Dropdown menu that shows options on click, ideal for conserving space.

```rust
Picker::new("format", 0)
    .option("JPEG")
    .option("PNG")
    .option("GIF")
    .menu()
```

### Segmented

Horizontal button group with all options visible, suitable for frequently accessed choices.

```rust
Picker::new("view-mode", 0)
    .option("List")
    .option("Grid")
    .option("Column")
    .segmented()
```

### Inline

Vertical list showing all options, appropriate for settings panels.

```rust
Picker::new("theme", 0)
    .option("Light")
    .option("Dark")
    .option("Auto")
    .inline()
```

## Usage Guidelines

Choose the appropriate style based on context:
- Use **Menu** when space is limited or you have many options
- Use **Segmented** for 2-5 frequently toggled options
- Use **Inline** in settings panels where vertical space is available

## See Also

- RadioGroup
- DatePicker
- ColorPicker
