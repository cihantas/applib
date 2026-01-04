# LazyHGrid

A lazy horizontal grid that only renders visible columns using virtualization.

## Overview

LazyHGrid efficiently handles large grids by only rendering columns currently visible in the viewport. Items are arranged in rows from top to bottom, then left to right. This is useful for horizontally scrolling collections arranged in multiple rows.

The grid uses GPUI's `uniform_list` infrastructure where each list entry represents a complete column of grid items, enabling efficient rendering and horizontal scrolling performance.

```rust
LazyHGrid::new(
    "timeline",
    events.len(),
    vec![GridRow::fixed(px(60.0)); 3],  // 3 rows
    |i, _window, _cx| {
        let event = &events[i];
        div()
            .w(px(120.0))
            .h(px(60.0))
            .child(event.title.clone())
    }
)
.spacing_3()
.p_4()
```

## Topics

### Creating a LazyHGrid

- `new(_:_:_:_:)` — Creates a lazy horizontal grid with an identifier, item count, row definitions, and render closure.

### Configuring Spacing

- `spacing(_:)` — Sets both horizontal and vertical spacing.
- `horizontal_spacing(_:)` — Sets spacing between columns.
- `vertical_spacing(_:)` — Sets spacing between rows.
- `spacing_3()` — Sets spacing to 12px (0.75rem).
- `spacing_6()` — Sets spacing to 24px (1.5rem).

### Configuring Padding

- `p(_:)` — Sets custom padding around all children.
- `p_1()` — Sets padding to 4px (0.25rem).
- `p_2()` — Sets padding to 8px (0.5rem).
- `p_3()` — Sets padding to 12px (0.75rem).
- `p_4()` — Sets padding to 16px (1rem).

### Controlling Scroll Position

- `track_scroll(_:)` — Sets a scroll handle to programmatically control scroll position.

## Supporting Types

### GridRow

Defines how a grid row should be sized. This is an alias for `GridColumn` from LazyVGrid, but semantically represents rows in the horizontal grid context.

- `Fixed(Pixels)` — A row with a fixed height.
- `Flexible { minimum, maximum, weight }` — A row that flexes based on available space.
- `Adaptive { minimum, maximum }` — Creates as many rows as can fit between min and max height.

```rust
// Fixed height rows
vec![GridRow::fixed(px(80.0)); 2]

// Flexible rows with different weights
vec![
    GridRow::flexible(1.0),
    GridRow::flexible(2.0),  // Takes twice as much space
]
```

## Rendering Items

The render closure receives a single index and returns an element for that item. The grid calculates which column and row each item belongs to based on the total item count and row definitions:

```rust
LazyHGrid::new("messages", messages.len(), rows, |i, _window, _cx| {
    let message = &messages[i];
    div()
        .w(px(200.0))
        .flex()
        .items_center()
        .child(div().child(message.text.clone()))
})
```

## Layout Behavior

The grid arranges items in columns where each column contains as many items as there are row definitions. Items flow top-to-bottom within each column, then left-to-right across columns.

For example, with 3 rows and 10 items:

- Column 0: Items 0, 1, 2 (top to bottom)
- Column 1: Items 3, 4, 5 (top to bottom)
- Column 2: Items 6, 7, 8 (top to bottom)
- Column 3: Item 9 (partial column)

Each column is virtualized independently, so only visible columns are rendered. Vertical spacing is applied between rows using margin-bottom, and horizontal spacing between columns using margin-right.

The container has horizontal overflow scrolling enabled automatically.

## Row Flexibility

**Fixed rows** have exact pixel heights and do not shrink.

**Flexible rows** grow to fill available space based on their weight. A row with weight 2.0 takes twice as much vertical space as a row with weight 1.0. Optional minimum and maximum constraints can be applied.

**Adaptive rows** automatically determine how many rows can fit in the available height while keeping items between the minimum and maximum sizes.

## Programmatic Scrolling

Use `LazyHGridScrollHandle` (a re-export of GPUI's `UniformListScrollHandle`) to programmatically control scroll position:

```rust
let scroll_handle = UniformListScrollHandle::new();

LazyHGrid::new("grid", items.len(), rows, render_fn)
    .track_scroll(scroll_handle.clone());

// Scroll to a specific column (not item index)
scroll_handle.scroll_to_item(column_index);
```

Note that the scroll handle controls column scrolling, not individual item scrolling.

## See Also

- LazyVGrid
- LazyHStack
- GridRow
