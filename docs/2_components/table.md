# Table

A table component for displaying data in aligned columns with consistent widths.

## Overview

Table manages column definitions and renders rows with consistent column widths across all rows. It provides polished styling with selection states and hover effects, designed for displaying structured tabular data.

Columns can be configured as either fixed-width or flexible (growing to fill available space). Multiple flexible columns share space proportionally. The table works with `TableRow` components to render each row's content.

```rust
let mut table = Table::new("commits")
    .column(TableColumn::flex())           // Message column (grows)
    .column(TableColumn::fixed(px(150.0))) // Author column
    .column(TableColumn::fixed(px(80.0)))  // Hash column
    .column(TableColumn::fixed(px(120.0))); // Date column

for (index, commit) in commits.iter().enumerate() {
    table = table.row(
        TableRow::new(("commit", index))
            .selected(selected_index == Some(index))
            .on_click(cx.listener(move |this, _event, _window, cx| {
                this.selected_index = Some(index);
                cx.notify();
            }))
            .cell(div().child(commit.message.clone()))
            .cell(div().child(commit.author.clone()))
            .cell(div().child(commit.hash.clone()))
            .cell(div().child(commit.date.clone()))
    );
}
```

## Topics

### Creating a Table

- `new(_:)` — Creates a table with the given identifier.

### Configuring Columns

- `column(_:)` — Adds a column definition to the table.

### Adding Content

- `row(_:)` — Adds a row to the table.

## Supporting Types

### TableColumn

Defines how a column should be sized within the table.

- `Fixed(Pixels)` — Column with a fixed pixel width.
- `Flex` — Column that grows to fill available space.

#### Creating Column Definitions

- `fixed(_:)` — Creates a fixed-width column.
- `flex()` — Creates a flexible column that grows to fill available space.

Multiple flex columns in the same table will share the available space equally.

## Layout Behavior

The table applies column widths to cells in the order they are added. Each cell is wrapped in a container that applies the appropriate width constraint:

- **Fixed columns** — Set to exact pixel width with `flex_shrink_0` to prevent shrinking.
- **Flex columns** — Set to `flex_1` to grow and fill available space.

Cells automatically hide overflow content to maintain column boundaries.

The number of cells in each row should match the number of columns defined. If a row has fewer cells than columns, the missing cells are skipped. If a row has more cells than columns, extra cells are rendered as flexible.

## See Also

- TableRow
- List
- LazyVGrid
