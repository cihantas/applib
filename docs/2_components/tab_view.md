# TabView

A modern tab view component for organizing content across multiple switchable panels.

## Overview

TabView provides a tab bar with selectable tabs and a content area that displays the currently selected tab's view. Each tab can include an icon, label, and optional badge count. The component follows modern design patterns with subtle shadows and rounded tab buttons.

```rust
TabView::new("main-tabs", 0)
    .tab(Tab::new("Files", files_view).icon(Icon::Folder))
    .tab(Tab::new("Messages", messages_view).icon(Icon::Message).badge(3))
    .tab(Tab::new("Settings", settings_view).icon(Icon::Gear))
    .on_selection_change(cx.listener(|this, index, _window, cx| {
        this.selected_tab = index;
        cx.notify();
    }))
```

The component implements lazy rendering—only the selected tab's content is rendered, optimizing performance for complex views. Tabs can be navigated using the mouse or keyboard arrow keys.

## Topics

### Creating a TabView

- `new(_:_:)` — Creates a tab view with the given identifier and selected index.

### Adding Tabs

- `tab(_:)` — Adds a tab to the tab view.

### Handling Selection

- `on_selection_change(_:)` — Registers a handler called when the selected tab changes.

## Supporting Types

### Tab

A single tab in a TabView, containing a label, optional icon, optional badge, and content element.

```rust
Tab::new("Inbox", inbox_view)
    .icon(Icon::Inbox)
    .badge(12)
```

#### Creating a Tab

- `new(_:_:)` — Creates a tab with the given label and content element.

#### Configuring a Tab

- `icon(_:)` — Sets an icon to display before the tab label.
- `badge(_:)` — Sets a numeric badge count to display after the tab label.

## Keyboard Navigation

TabView supports keyboard navigation when focused:

- **Left Arrow** — Selects the previous tab (if available).
- **Right Arrow** — Selects the next tab (if available).

Selection changes triggered by keyboard navigation invoke the `on_selection_change` handler, allowing your application to update state accordingly.

## Design Notes

TabView follows a polished aesthetic with:

- Light gray tab bar background with subtle border
- White background for selected tabs with soft shadow
- Blue icons when tabs are selected, gray when unselected
- Medium font weight for selected tab labels
- Smooth hover states for unselected tabs

Badges automatically use the Badge component styling, providing consistent visual treatment across the interface.

## See Also

- Badge
- Sidebar
- List
