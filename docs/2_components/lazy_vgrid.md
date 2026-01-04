# LazyVGrid

A lazy vertical grid that only renders visible rows using virtualization.

## Overview

LazyVGrid efficiently handles large grids by only rendering rows currently visible in the viewport. Items are arranged in columns from left to right, then top to bottom. This makes it ideal for displaying large collections of uniformly-sized items like photo galleries or product catalogs.

The grid uses GPUI's `uniform_list` infrastructure where each list entry represents a complete row of grid items, ensuring efficient rendering and scrolling performance.

```rust
LazyVGrid::new(
    "photos",
    1000,
    vec![GridColumn::fixed(px(100.0)); 4],  // 4 columns
    |i, _window, _cx| {
        div()
            .h(px(100.0))
            .bg(hsla(0.0, 0.0, 0.95, 1.0))
            .child(format!("Photo {}", i))
    }
)
.spacing_3()
.p_4()
```

## Topics

### Creating a LazyVGrid

- `new(_:_:_:_:)` — Creates a lazy vertical grid with an identifier, item count, column definitions, and render closure.

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

### GridColumn

Defines how a grid column should be sized.

- `Fixed(Pixels)` — A column with a fixed width.
- `Flexible { minimum, maximum, weight }` — A column that flexes based on available space.
- `Adaptive { minimum, maximum }` — Creates as many columns as can fit between min and max width.

#### Creating Column Definitions

- `fixed(_:)` — Creates a fixed-width column.
- `flexible(_:)` — Creates a flexible column with the given weight.
- `flexible_min(_:_:)` — Creates a flexible column with a minimum width and weight.
- `adaptive(_:_:)` — Creates an adaptive column with minimum and maximum widths.

```rust
// Fixed width columns
vec![GridColumn::fixed(px(100.0)); 3]

// Flexible columns with different weights
vec![
    GridColumn::flexible(2.0),  // Takes twice as much space
    GridColumn::flexible(1.0),
]

// Adaptive columns (creates as many as can fit)
vec![GridColumn::adaptive(px(80.0), px(120.0))]
```

## Rendering Items

The render closure receives a single index and returns an element for that item. The grid calculates which row and column each item belongs to based on the total item count and column definitions:

```rust
LazyVGrid::new("products", products.len(), columns, |i, _window, _cx| {
    let product = &products[i];
    div()
        .h(px(200.0))
        .flex()
        .flex_col()
        .child(Image::new(product.image.clone()))
        .child(div().child(product.name.clone()))
})
```

## Layout Behavior

The grid arranges items in rows where each row contains as many items as there are column definitions. For example, with 3 columns and 10 items:

- Row 0: Items 0, 1, 2
- Row 1: Items 3, 4, 5
- Row 2: Items 6, 7, 8
- Row 3: Item 9 (partial row)

Each row is virtualized independently, so only visible rows are rendered. Horizontal spacing is applied between columns using margin-right, and vertical spacing between rows using margin-bottom.

## Column Flexibility

**Fixed columns** have exact pixel widths and do not shrink.

**Flexible columns** grow to fill available space based on their weight. A column with weight 2.0 takes twice as much space as a column with weight 1.0. Optional minimum and maximum constraints can be applied.

**Adaptive columns** automatically determine how many columns can fit in the available width while keeping items between the minimum and maximum sizes. This is useful for responsive layouts.

## Programmatic Scrolling

Use `LazyVGridScrollHandle` (a re-export of GPUI's `UniformListScrollHandle`) to programmatically control scroll position:

```rust
let scroll_handle = UniformListScrollHandle::new();

LazyVGrid::new("grid", items.len(), columns, render_fn)
    .track_scroll(scroll_handle.clone());

// Scroll to a specific row (not item index)
scroll_handle.scroll_to_item(row_index);
```

Note that the scroll handle controls row scrolling, not individual item scrolling.

## See Also

- LazyHGrid
- LazyVStack
- GridColumn
