# DisclosureGroup

A collapsible section with a header and expandable content.

## Overview

DisclosureGroup displays a header with a disclosure triangle indicator and child elements that can be shown or hidden. The expanded state can be managed externally via callbacks, making it suitable for stateful views where you need to control multiple disclosure groups.

Use disclosure groups in sidebars, navigation panels, or any hierarchical interface where content can be selectively revealed.

```rust
DisclosureGroup::new("branches", "Branches", is_expanded, move |expanded, cx| {
    on_toggle(expanded, cx);
})
    .child(SidebarItem::new("main", "main").bold(true))
    .child(SidebarItem::new("dev", "develop"))
```

The disclosure triangle automatically rotates to indicate the current state: pointing right when collapsed, pointing down when expanded.

## Topics

### Creating a DisclosureGroup

- `new(_:_:_:_:)` — Creates a disclosure group with external state management.
- `new_static(_:_:_:)` — Creates a disclosure group without toggle callback for static display.

### Adding Content

- `child(_:)` — Adds a single child element to the group.
- `children(_:)` — Adds multiple child elements to the group.

## State Management

DisclosureGroup requires external state management. Store the expanded state in your view and provide a callback that updates that state:

```rust
struct MyView {
    files_expanded: bool,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        DisclosureGroup::new(
            "files-section",
            "Files",
            self.files_expanded,
            cx.listener(|this, expanded, _window, cx| {
                this.files_expanded = expanded;
                cx.notify();
            })
        )
        .child(div().child("file1.txt"))
        .child(div().child("file2.txt"))
    }
}
```

For static display where user interaction is not needed, use `new_static`:

```rust
DisclosureGroup::new_static("read-only", "Items", true)
    .child(div().child("Item 1"))
    .child(div().child("Item 2"))
```

## See Also

- Sidebar
- SidebarItem
- List
