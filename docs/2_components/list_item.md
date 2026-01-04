# ListItem

A list item component with selection states, hover effects, and polished styling.

## Overview

ListItem represents a single row in a List component. It provides built-in styling for selected and unselected states, hover effects, and click handling following modern design aesthetics.

When selected, list items display a blue gradient background. Unselected items show a white background with a subtle hover state. All items include a bottom border for visual separation.

```rust
ListItem::new("item-1")
    .selected(true)
    .on_click(cx.listener(|this, _event, _window, cx| {
        this.selected_item = Some(1);
        cx.notify();
    }))
    .child(
        div()
            .flex()
            .items_center()
            .child(div().child("Item content"))
    )
```

## Topics

### Creating a ListItem

- `new(_:)` — Creates a list item with the given identifier.

### Configuring Appearance

- `selected(_:)` — Sets whether the item is selected.

### Handling Interaction

- `on_click(_:)` — Registers a click handler for the list item.

### Adding Content

- `child(_:)` — Adds a child element to the list item.

## Visual States

ListItem automatically applies different styling based on its state:

- **Selected** — Blue gradient background (HSL: 211°, 95%, 53%) with pointer cursor.
- **Unselected** — White background with hover effect (98% lightness on hover) and pointer cursor.
- **Border** — All items have a 1px bottom border in light gray (90% lightness).

The item has a minimum height of 48px and 16px horizontal padding with 12px vertical padding.

## See Also

- List
- ListSection
- TableRow
