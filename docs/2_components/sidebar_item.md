# SidebarItem

An individual navigation or list item within a Sidebar container.

## Overview

SidebarItem represents a single selectable entry in a sidebar, such as a navigation link, branch name, or file entry. Items support selection states, bold text emphasis, badges, and both single-click and double-click interactions.

```rust
SidebarItem::new("main-branch", "main")
    .selected(true)
    .bold(true)
    .on_click(|_event, _window, cx| {
        // Handle selection
    })
```

Selected items display with a blue gradient background and white text, following modern design patterns. Unselected items show subtle hover states to indicate interactivity.

## Topics

### Creating a SidebarItem

- `new(_:_:)` — Creates a sidebar item with the given identifier and label.

### Configuring Appearance

- `selected(_:)` — Sets whether the item appears selected.
- `bold(_:)` — Sets whether the item text is bold.
- `badge(_:)` — Adds a badge element to the right side of the item.

### Handling Interactions

- `on_click(_:)` — Registers a handler called when the item is clicked.
- `on_double_click(_:)` — Registers a handler called when the item is double-clicked.

## Selection States

### Selected

Selected items feature prominent visual styling to indicate the current selection:

```rust
SidebarItem::new("current", "Current Branch")
    .selected(true)
```

- Blue gradient background (hsl 211°, 75%, 58%)
- White text color
- No hover effect (already visually distinct)

### Unselected

Unselected items use minimal styling with interactive feedback:

```rust
SidebarItem::new("branch-1", "feature/new-ui")
    .selected(false)
```

- Transparent background
- Dark gray text (20% lightness)
- Subtle darkening on hover (5% opacity overlay)

## Click Handling

SidebarItem distinguishes between single and double clicks based on click count. The component automatically routes clicks to the appropriate handler:

```rust
SidebarItem::new("file", "README.md")
    .on_click(|_event, _window, cx| {
        // Select the file
    })
    .on_double_click(|_event, _window, cx| {
        // Open the file
    })
```

Double-clicks (click count ≥ 2) invoke the `on_double_click` handler if provided. Single clicks invoke the `on_click` handler. If only one handler is registered, items respond to that interaction type only.

## Usage Patterns

### Navigation Items

Use for application-level navigation:

```rust
SidebarItem::new("nav-home", "Home")
    .selected(current_view == "home")
    .on_click(|_event, _window, cx| {
        navigate_to("home");
    })
```

### Repository Branches

Display branches with emphasis on the current branch:

```rust
SidebarItem::new(branch.name, branch.name)
    .bold(branch.is_current)
    .selected(branch.is_selected)
```

### Items with Badges

Show counts or status indicators:

```rust
SidebarItem::new("inbox", "Inbox")
    .badge(Badge::new("24"))
    .on_click(show_inbox_handler)
```

## See Also

- Sidebar
- Badge
- ListItem
