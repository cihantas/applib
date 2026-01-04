# Sidebar

A vertical navigation container with polished source list styling.

## Overview

Sidebar provides a container for vertical navigation panels, featuring the characteristic light blue-gray gradient background and proper spacing for sidebar items. It serves as the foundation for navigation sidebars, file browsers, and organizational views.

```rust
Sidebar::new()
    .width(px(220.0))
    .child(SidebarItem::new("local", "Local Changes").selected(true))
    .child(SidebarItem::new("main", "main").bold(true))
    .child(SidebarItem::new("develop", "develop"))
```

The sidebar automatically handles layout, spacing, and background styling. It includes a right border to visually separate it from adjacent content areas.

## Topics

### Creating a Sidebar

- `new()` — Creates a sidebar with default width (200px).

### Configuring Dimensions

- `width(_:)` — Sets the sidebar width in pixels.

### Adding Content

- `child(_:)` — Adds a single child element to the sidebar.
- `children(_:)` — Adds multiple children to the sidebar.

## Design Notes

Sidebar implements a polished source list appearance:

- Light blue-gray background (hsl 210°, 8%, 93%)
- Right border for content separation
- 8px padding with 4px gap between items
- Full height layout with vertical flex direction

The component is designed to work seamlessly with SidebarItem components but accepts any child elements, allowing for sections, headers, or custom content.

## Usage Patterns

### Navigation Sidebar

Combine with SidebarItem components for application navigation:

```rust
Sidebar::new()
    .children(vec![
        SidebarItem::new("files", "Files").icon(Icon::Folder),
        SidebarItem::new("search", "Search").icon(Icon::Search),
        SidebarItem::new("settings", "Settings").icon(Icon::Gear),
    ])
```

### File Browser

Use for hierarchical file or folder navigation:

```rust
Sidebar::new()
    .width(px(250.0))
    .child(div().child("FOLDERS").text_xs().text_color(gray))
    .children(folders.iter().map(|folder| {
        SidebarItem::new(folder.id, folder.name.clone())
    }))
```

## See Also

- SidebarItem
- TabView
- List
