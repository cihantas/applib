# List

A scrollable list component for displaying rows of data with built-in selection management and keyboard navigation.

## Overview

List provides a container for displaying items with built-in scrolling, selection management, and styling following modern design patterns. It integrates with `ListItem` components and supports sectioned content via `ListSection`.

The list supports three visual styles (`Plain`, `Inset`, and `Sidebar`) and multiple selection modes (`None`, `Single`, and `Multiple`). When configured with a focus handle, lists respond to keyboard input including arrow keys for navigation and Enter to confirm selections.

```rust
List::new("my-list")
    .style(ListStyle::Inset)
    .selection_mode(SelectionMode::Single)
    .selected_index(Some(0))
    .on_selection_change(|indices, window, cx| {
        println!("Selected: {:?}", indices);
    })
    .children(items.iter().enumerate().map(|(i, item)| {
        ListItem::new(("item", i))
            .child(div().child(item.name.clone()))
    }))
```

Lists can be made searchable by adding a search field component at the top:

```rust
struct MyView {
    search_query: String,
    search_field: View<TextFieldState>,
    all_items: Vec<Item>,
}

impl Render for MyView {
    fn render(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let filtered_items = self.all_items.iter()
            .filter(|item| {
                self.search_query.is_empty() ||
                item.name.to_lowercase().contains(&self.search_query.to_lowercase())
            })
            .collect::<Vec<_>>();

        List::new("items-list")
            .search_field(self.search_field.clone())
            .children(filtered_items.iter().map(|item| {
                ListItem::new(("item", item.id))
                    .child(item.name.clone())
            }))
    }
}
```

## Topics

### Creating a List

- `new(_:)` — Creates a list with the given identifier.

### Configuring Appearance

- `style(_:)` — Sets the visual style (Plain, Inset, or Sidebar).
- `show_separators(_:)` — Controls whether row separators are displayed.

### Managing Selection

- `selection_mode(_:)` — Sets the selection mode (None, Single, or Multiple).
- `selected(_:)` — Sets the currently selected indices.
- `selected_index(_:)` — Sets a single selected index for single selection mode.
- `on_selection_change(_:)` — Registers a handler called when selection changes.

### Handling Keyboard Navigation

- `focusable(_:)` — Makes the list focusable and enables keyboard navigation.
- `on_confirm(_:)` — Registers a handler called when Enter is pressed on a selected item.

### Adding Content

- `child(_:)` — Adds a single child element to the list.
- `children(_:)` — Adds multiple children to the list.
- `search_field(_:)` — Adds a search field element at the top of the list.

## Supporting Types

### ListStyle

Visual style options for the list.

- `Plain` — Full-width rows with minimal styling.
- `Inset` — Rounded corners with inset margins.
- `Sidebar` — Optimized styling for navigation sidebars.

### SelectionMode

Selection behavior options for the list.

- `None` — No selection allowed.
- `Single` — Only one item can be selected at a time.
- `Multiple` — Multiple items can be selected.

### ListSection

A section within a list for grouping related items.

```rust
List::new("my-list")
    .child(
        ListSection::new("favorites")
            .header("Favorites")
            .children(favorites.iter().map(|f| {
                ListItem::new(("fav", f.id)).child(f.name.clone())
            }))
    )
    .child(
        ListSection::new("recent")
            .header("Recent")
            .children(recent.iter().map(|r| {
                ListItem::new(("recent", r.id)).child(r.name.clone())
            }))
    )
```

#### Creating a Section

- `new(_:)` — Creates a section with the given identifier.

#### Configuring a Section

- `header(_:)` — Sets the header text for the section.
- `footer(_:)` — Sets the footer text for the section.
- `collapsed(_:)` — Sets whether the section is collapsed.

#### Adding Content

- `child(_:)` — Adds a single child element to the section.
- `children(_:)` — Adds multiple children to the section.

## Keyboard Navigation

When configured with a focus handle using `focusable(_:)`, lists support keyboard navigation:

- **Arrow Up/Down** — Move selection up or down by one item.
- **Cmd+Arrow Up** or **Home** — Jump to the first item.
- **Cmd+Arrow Down** or **End** — Jump to the last item.
- **Enter** — Trigger the `on_confirm` callback for the selected item.

Keyboard navigation is only active when the selection mode is not `None`.

## See Also

- ListItem
- TextField
- Table
