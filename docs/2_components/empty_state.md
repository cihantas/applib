# EmptyState

A component for displaying "no data" scenarios with optional icon, message, and action.

## Overview

EmptyState communicates to users when content is unavailable or empty, providing context about why there's no content and optionally offering an action to resolve the situation. It displays a centered layout with an icon, title, description, and optional action button.

Use EmptyState to replace empty tables, lists, or content areas with helpful messaging that guides users toward meaningful actions.

```rust
EmptyState::new("no-results")
    .icon(Icon::MagnifyingGlass)
    .title("No Results")
    .description("Try a different search term")
```

With an action button to help users get started:

```rust
EmptyState::new("no-data")
    .icon(Icon::Document)
    .title("No Documents")
    .description("Create your first document to get started")
    .action(
        Button::new("create", "Create Document")
            .primary()
            .on_click(|_e, _w, cx| { /* ... */ })
    )
```

## Topics

### Creating an Empty State

- `new(_:)` — Creates a new empty state with the given element ID.

### Configuring Content

- `icon(_:)` — Sets the icon to display at the top.
- `title(_:)` — Sets the title text.
- `description(_:)` — Sets the description text.
- `action(_:)` — Sets an optional action button or element.

## See Also

- EmptyView
- ProgressView
- Icon
