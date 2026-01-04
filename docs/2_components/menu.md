# Menu

A button that displays a popup menu of actions when triggered.

## Overview

Menu combines a trigger button with a dropdown panel containing menu items. It supports icons, keyboard shortcuts, dividers, disabled items, and nested submenus, following modern design patterns.

Two variants are available:
- **Menu** — Hover-triggered menu that appears when hovering over the button
- **ControlledMenu** — Click-triggered menu controlled by external state

```rust
Menu::new("actions-menu", "Actions")
    .item(MenuItem::new("copy", "Copy").shortcut("⌘C"))
    .item(MenuItem::new("paste", "Paste").shortcut("⌘V"))
    .divider()
    .item(MenuItem::new("delete", "Delete"))
```

Menu items can be disabled, display icons, show keyboard shortcuts, and execute callbacks when selected. The menu automatically dismisses when an item is selected.

## Topics

### Creating a Menu

- `new(_:_:)` — Creates a hover-triggered menu.
- `ControlledMenu::new(_:_:_:)` — Creates a menu controlled by external state.

### Adding Menu Items

- `item(_:)` — Adds a menu item.
- `divider()` — Adds a visual divider between menu items.
- `submenu(_:_:_:)` — Adds a nested submenu.
- `submenu_with_icon(_:_:_:_:)` — Adds a nested submenu with an icon.

### Configuring Menu

- `disabled(_:)` — Sets whether the menu trigger button is disabled.
- `on_toggle(_:)` — Sets the toggle handler for ControlledMenu.

### Creating Menu Items

- `MenuItem::new(_:_:)` — Creates a menu item with an identifier and label.
- `icon(_:)` — Sets the icon for the menu item.
- `shortcut(_:)` — Sets the keyboard shortcut display text.
- `disabled(_:)` — Marks the menu item as disabled.
- `on_select(_:)` — Sets the handler called when the item is selected.

## Using Menu Items

Create menu items with descriptive labels and optional icons and shortcuts:

```rust
MenuItem::new("save", "Save")
    .icon(Icon::Document)
    .shortcut("⌘S")
    .on_select(|_window, cx| {
        // Save action
    })
```

Group related items with dividers:

```rust
Menu::new("file-menu", "File")
    .item(MenuItem::new("new", "New"))
    .item(MenuItem::new("open", "Open"))
    .divider()
    .item(MenuItem::new("save", "Save"))
    .item(MenuItem::new("save-as", "Save As..."))
    .divider()
    .item(MenuItem::new("close", "Close"))
```

Disabled items appear grayed out and cannot be selected:

```rust
MenuItem::new("undo", "Undo")
    .shortcut("⌘Z")
    .disabled(true)
```

## Working with Submenus

Create nested menu hierarchies using submenus:

```rust
Menu::new("edit-menu", "Edit")
    .submenu("transform", "Transform", |sub| {
        sub.item(MenuItem::new("uppercase", "Make Uppercase"))
            .item(MenuItem::new("lowercase", "Make Lowercase"))
            .item(MenuItem::new("capitalize", "Capitalize"))
    })
```

Submenus can be nested multiple levels deep and support the same features as top-level menus:

```rust
.submenu("share", "Share", |sub| {
    sub.item(MenuItem::new("email", "Email"))
        .submenu("social", "Social Media", |subsub| {
            subsub.item(MenuItem::new("twitter", "Twitter"))
                .item(MenuItem::new("facebook", "Facebook"))
        })
})
```

## Using ControlledMenu

For click-triggered menus, use ControlledMenu with external state:

```rust
struct MyView {
    menu_open: bool,
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        ControlledMenu::new("actions", "Actions", self.menu_open)
            .item(MenuItem::new("action1", "Action 1"))
            .item(MenuItem::new("action2", "Action 2"))
            .on_toggle(cx.listener(|this, open, _window, cx| {
                this.menu_open = *open;
                cx.notify();
            }))
    }
}
```

## See Also

- ContextMenu
- Popover
- MenuItem
