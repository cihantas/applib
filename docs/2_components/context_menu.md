# ContextMenu

A popup menu that appears at the cursor location when right-clicking content.

## Overview

ContextMenu wraps any content and displays a menu when the user performs a secondary click (right-click) on that content. The menu appears at the exact cursor position and provides contextual actions relevant to the clicked element.

Context menus share the same menu item infrastructure as Menu, supporting icons, shortcuts, dividers, and nested submenus.

```rust
ContextMenu::new("file-context", file_item)
    .state(context_menu_state)
    .on_open(cx.listener(|this, pos, _window, cx| {
        this.context_menu_state.open(*pos);
        cx.notify();
    }))
    .item(MenuItem::new("open", "Open"))
    .item(MenuItem::new("rename", "Rename"))
    .divider()
    .item(MenuItem::new("delete", "Delete"))
```

## Topics

### Creating a ContextMenu

- `new(_:_:)` — Creates a context menu wrapping the given content.

### Managing State

- `ContextMenuState::new()` — Creates a new closed context menu state.
- `state(_:)` — Sets the context menu state.
- `on_open(_:)` — Sets the handler called when right-clicking to open the menu.

### Adding Menu Items

- `item(_:)` — Adds a menu item to the context menu.
- `divider()` — Adds a visual divider between menu items.
- `submenu(_:_:_:)` — Adds a nested submenu.
- `submenu_with_icon(_:_:_:_:)` — Adds a nested submenu with an icon.

### Managing State with ContextMenuState

- `open(_:)` — Opens the context menu at the specified position.
- `close()` — Closes the context menu.
- `toggle(_:)` — Toggles the context menu state at the specified position.

## State Management

ContextMenu requires a `ContextMenuState` to track whether the menu is visible and its position. Store this state in your view:

```rust
struct FileView {
    context_menu: ContextMenuState,
}

impl FileView {
    fn new() -> Self {
        Self {
            context_menu: ContextMenuState::new(),
        }
    }
}
```

Provide the state and an `on_open` callback to update it when the user right-clicks:

```rust
impl Render for FileView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ContextMenu::new("file", file_element)
            .state(self.context_menu.clone())
            .on_open(cx.listener(|this, pos, _window, cx| {
                this.context_menu.open(*pos);
                cx.notify();
            }))
            .item(
                MenuItem::new("delete", "Delete")
                    .on_select(cx.listener(|this, _window, cx| {
                        this.context_menu.close();
                        this.delete_file(cx);
                    }))
            )
    }
}
```

## Closing the Menu

The context menu does not automatically close when an item is selected. Close it explicitly in your item callbacks:

```rust
.item(
    MenuItem::new("copy", "Copy")
        .on_select(cx.listener(|this, _window, cx| {
            this.context_menu.close();
            this.copy_file(cx);
            cx.notify();
        }))
)
```

Clicking outside the menu area should also close it. Implement this in your view's mouse handling if needed.

## Customizing Menu Items

Context menu items support all the same features as regular menu items:

```rust
ContextMenu::new("item-context", item)
    .state(state)
    .on_open(open_handler)
    .item(
        MenuItem::new("edit", "Edit")
            .icon(Icon::Pencil)
            .shortcut("⌘E")
    )
    .item(
        MenuItem::new("duplicate", "Duplicate")
            .icon(Icon::Document)
            .shortcut("⌘D")
    )
    .divider()
    .item(
        MenuItem::new("delete", "Delete")
            .icon(Icon::Trash)
            .disabled(is_read_only)
    )
```

## See Also

- Menu
- MenuItem
- Popover
