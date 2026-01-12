# ContextMenu

A popup menu that appears at the cursor location when right-clicking content.

## Overview

ContextMenu wraps any content and displays a menu when the user performs a secondary click (right-click) on that content. The menu appears at the cursor position and provides contextual actions relevant to the clicked element.

This is a **controlled component** - the caller manages the open/close state via `Option<Point<Pixels>>`:
- `None` means the menu is closed
- `Some(position)` means the menu is open at that position

```rust
struct MyView {
    context_menu_position: Option<Point<Pixels>>,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ContextMenu::new("file-context", file_item)
            .state(self.context_menu_position)
            .on_toggle(cx.listener(|this, pos, _window, cx| {
                this.context_menu_position = pos;
                cx.notify();
            }))
            .item(MenuItem::new("open", "Open"))
            .item(MenuItem::new("rename", "Rename"))
            .divider()
            .item(MenuItem::new("delete", "Delete"))
    }
}
```

## Topics

### Creating a ContextMenu

- `new(_:_:)` - Creates a context menu wrapping the given content.

### Managing State

- `state(_:)` - Sets the menu state: `None` = closed, `Some(position)` = open at position.
- `on_toggle(_:)` - Sets the handler called when the menu should open or close.

### Adding Menu Items

- `item(_:)` - Adds a menu item to the context menu.
- `divider()` - Adds a visual divider between menu items.
- `submenu(_:_:_:)` - Adds a nested submenu.
- `submenu_with_icon(_:_:_:_:)` - Adds a nested submenu with an icon.

## State Management

ContextMenu uses the controlled component pattern. Store the menu position in your view state:

```rust
struct FileView {
    // None = closed, Some(position) = open at cursor position
    menu_position: Option<Point<Pixels>>,
}

impl FileView {
    fn new() -> Self {
        Self {
            menu_position: None,
        }
    }
}
```

The `on_toggle` callback receives:
- `Some(position)` when the user right-clicks to open the menu
- `None` when the menu should close (clicking outside, selecting an item, etc.)

```rust
impl Render for FileView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ContextMenu::new("file", file_element)
            .state(self.menu_position)
            .on_toggle(cx.listener(|this, pos, _window, cx| {
                this.menu_position = pos;
                cx.notify();
            }))
            .item(
                MenuItem::new("delete", "Delete")
                    .on_select(cx.listener(|this, _window, cx| {
                        this.delete_file(cx);
                    }))
            )
    }
}
```

## Automatic Menu Closing

The context menu automatically closes when:
- An item is selected
- The user clicks outside the menu
- The user right-clicks elsewhere (opens menu at new position)

You do not need to manually close the menu in your item callbacks.

## Customizing Menu Items

Context menu items support all the same features as regular menu items:

```rust
ContextMenu::new("item-context", item)
    .state(self.menu_position)
    .on_toggle(toggle_handler)
    .item(
        MenuItem::new("edit", "Edit")
            .icon(Icon::Pencil)
            .shortcut("E")
    )
    .item(
        MenuItem::new("duplicate", "Duplicate")
            .icon(Icon::Document)
            .shortcut("D")
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
