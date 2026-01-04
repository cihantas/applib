# TableRow

A table row component that holds cell content for tabular display.

## Overview

TableRow is designed to be used with the Table component. It stores cell content and styling information, while the Table component handles the actual layout based on column definitions.

TableRow provides selection state and click handling. The visual rendering of cells is managed by the parent Table component to ensure consistent column widths across all rows.

```rust
TableRow::new("row-1")
    .selected(true)
    .on_click(cx.listener(|this, _event, _window, cx| {
        this.selected_row = Some(1);
        cx.notify();
    }))
    .cell(div().child("Cell 1"))
    .cell(div().child("Cell 2"))
    .cell(div().child("Cell 3"))
```

## Topics

### Creating a TableRow

- `new(_:)` — Creates a table row with the given identifier.

### Configuring Appearance

- `selected(_:)` — Sets whether the row is selected.

### Handling Interaction

- `on_click(_:)` — Registers a click handler for the row.

### Adding Content

- `cell(_:)` — Adds a cell to the row.

## Cell Order

Cells are rendered in the order they are added, with widths determined by the parent Table's column definitions. The first cell added corresponds to the first column, the second cell to the second column, and so on.

```rust
Table::new("data")
    .column(TableColumn::flex())
    .column(TableColumn::fixed(px(100.0)))
    .row(
        TableRow::new("row-1")
            .cell(div().child("This fills available space"))
            .cell(div().child("This is 100px wide"))
    )
```

## Visual States

TableRow applies different styling based on its state (managed by the parent Table):

- **Selected** — Blue gradient background (HSL: 211°, 95%, 53%) with darker blue border.
- **Unselected** — White background with hover effect (98% lightness on hover) and light gray border.

Rows have 16px horizontal padding, 2px vertical padding, a minimum height of 22px, and a 1px bottom border.

## See Also

- Table
- TableColumn
- ListItem
