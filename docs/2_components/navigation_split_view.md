# NavigationSplitView

A two-column layout for master-detail navigation interfaces.

## Overview

NavigationSplitView provides a sidebar-detail layout commonly used for navigation interfaces like file browsers, email clients, and settings panels. The sidebar contains navigation items while the detail column shows content for the selected item.

```rust
NavigationSplitView::new("file-browser")
    .sidebar_width(px(280.0))
    .sidebar(
        VStack::new()
            .child(SidebarItem::new("docs", "Documents").selected(true))
            .child(SidebarItem::new("downloads", "Downloads"))
    )
    .detail(
        div().child("Document contents here")
    )
```

Unlike `SplitView`, NavigationSplitView has a fixed sidebar width (not draggable) and applies macOS source list styling to the sidebar by default.

## Topics

### Creating a NavigationSplitView

- `new(_:)` - Creates a navigation split view with the given id.

### Configuring Content

- `sidebar(_:)` - Sets the sidebar content.
- `detail(_:)` - Sets the detail content.
- `sidebar_width(_:)` - Sets the sidebar width (default 240px).

### Styling

- `divider_color(_:)` - Sets the divider color between columns.
- `sidebar_background(_:)` - Sets the sidebar background color.
- `detail_background(_:)` - Sets the detail area background color.

## Sidebar Content

The sidebar typically contains navigation items. Use `List`, `VStack` with `SidebarItem`, or custom content:

```rust
// With List component
NavigationSplitView::new("nav")
    .sidebar(
        List::new("sidebar-list", items.len(), |idx, selected, w, cx| {
            SidebarItem::new(("item", idx), items[idx].name.clone())
                .selected(selected)
                .into_any_element()
        })
        .selection(selection_binding)
    )
    .detail(detail_content)
```

```rust
// With VStack
NavigationSplitView::new("nav")
    .sidebar(
        VStack::new()
            .child(SidebarItem::new("inbox", "Inbox").selected(true))
            .child(SidebarItem::new("sent", "Sent"))
            .child(SidebarItem::new("trash", "Trash"))
    )
```

## Detail Content

The detail area shows content based on the sidebar selection:

```rust
let detail = match self.selected_item {
    Some("inbox") => div().child("Inbox messages..."),
    Some("sent") => div().child("Sent messages..."),
    _ => div().child("Select an item"),
};

NavigationSplitView::new("mail")
    .sidebar(sidebar_content)
    .detail(detail)
```

## Custom Styling

Override the default macOS-style colors:

```rust
NavigationSplitView::new("settings")
    .sidebar_background(hsla(0.0, 0.0, 0.95, 1.0))
    .detail_background(hsla(0.0, 0.0, 1.0, 1.0))
    .divider_color(hsla(0.0, 0.0, 0.85, 1.0))
```

## Comparison with SplitView

| Feature | NavigationSplitView | SplitView |
|---------|--------------------:|----------:|
| Draggable divider | No | Yes |
| Sidebar styling | macOS source list | None |
| State management | Stateless | Entity-based |
| Orientation | Horizontal only | H or V |

Use `NavigationSplitView` for navigation interfaces with a fixed sidebar. Use `SplitView` when you need resizable panes or vertical splits.

## See Also

- SplitView
- SidebarItem
- List
