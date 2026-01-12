# Table

A virtualized table component for displaying data in aligned columns with keyboard navigation.

## Overview

Table displays structured data in columns using GPUI's `uniform_list` for virtualized rendering. Rows are rendered on-demand via a callback function, providing efficient performance for large datasets.

```rust
struct CommitView {
    commits: Vec<Commit>,
    selected: Entity<State<Option<usize>>>,
    scroll_handle: UniformListScrollHandle,
    focus_handle: FocusHandle,
}

impl Render for CommitView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let commits = self.commits.clone();

        Table::new("commits", self.commits.len(), move |index, selected, _window, _cx| {
            let commit = &commits[index];
            vec![
                div().child(commit.message.clone()).into_any_element(),
                div().child(commit.author.clone()).into_any_element(),
                div().child(commit.hash.clone()).into_any_element(),
            ]
        })
        .columns([
            TableColumn::flex(),
            TableColumn::fixed(px(150.0)),
            TableColumn::fixed(px(80.0)),
        ])
        .selection(State::binding(&self.selected, cx))
        .focusable(self.focus_handle.clone())
        .track_scroll(self.scroll_handle.clone())
        .on_confirm(|index, _window, _cx| {
            println!("Confirmed row {}", index);
        })
    }
}
```

## Topics

### Creating a Table

- `new(_:_:_:)` - Creates a table with an id, row count, and cell render callback.

### Configuring Columns

- `columns(_:)` - Sets the column definitions for the table.

### Selection and Navigation

- `selection(_:)` - Sets a two-way binding for the selected row index.
- `focusable(_:)` - Makes the table focusable and enables keyboard navigation.
- `track_scroll(_:)` - Connects a scroll handle for scroll-to-selection support.

### Event Handling

- `on_confirm(_:)` - Sets the handler called when Enter is pressed on a selected row.
- `on_row_right_click(_:)` - Sets the handler called when right-clicking a row.

## Callback-Based Rendering

The table uses a callback to render cells for each visible row. The callback receives:
- `index` - The row index (0-based)
- `selected` - Whether this row is currently selected
- `window` - The window context
- `cx` - The app context

Return a `Vec<AnyElement>` with one element per column:

```rust
Table::new("items", items.len(), |index, selected, _window, _cx| {
    vec![
        div().child(items[index].name.clone()).into_any_element(),
        div().child(items[index].value.to_string()).into_any_element(),
    ]
})
```

## Column Definitions

Configure columns using `TableColumn`:

```rust
.columns([
    TableColumn::flex(),           // Grows to fill space
    TableColumn::fixed(px(150.0)), // Fixed 150px width
    TableColumn::fixed(px(80.0)),  // Fixed 80px width
])
```

- **Fixed columns** - Exact pixel width, won't shrink
- **Flex columns** - Grow to fill available space, share proportionally

## Selection with Bindings

Use a `Binding<Option<usize>>` for two-way selection state:

```rust
struct MyView {
    selected: Entity<State<Option<usize>>>,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Table::new("items", count, render)
            .selection(State::binding(&self.selected, cx))
    }
}
```

Clicking a row or using keyboard navigation updates the binding automatically.

## Keyboard Navigation

Enable keyboard navigation with `focusable()`:

```rust
Table::new("items", count, render)
    .selection(selection_binding)
    .focusable(self.focus_handle.clone())
```

Supported keys:
- **Up/Down** - Move selection
- **Cmd+Up/Cmd+Down** - Jump to first/last row
- **Enter** - Trigger `on_confirm` callback

## Scroll Tracking

Connect a scroll handle to automatically scroll to the selection:

```rust
struct MyView {
    scroll_handle: UniformListScrollHandle,
}

impl MyView {
    fn new() -> Self {
        Self {
            scroll_handle: UniformListScrollHandle::new(),
        }
    }
}

// In render:
Table::new("items", count, render)
    .track_scroll(self.scroll_handle.clone())
```

## Context Menu Support

Use `on_row_right_click` to handle right-clicks for context menus:

```rust
Table::new("items", count, render)
    .on_row_right_click(cx.listener(|this, (index, position), _window, cx| {
        this.context_menu_row = Some(index);
        this.context_menu_position = Some(position);
        cx.notify();
    }))
```

The handler receives the row index and click position in window coordinates.

## See Also

- TableRow
- List
- State
- Binding
