# SplitView

A two-pane layout container with horizontal or vertical orientation.

## Overview

SplitView divides its space between two panes separated by a 1-pixel divider. Panes can be arranged side-by-side (horizontal) or stacked top-to-bottom (vertical).

```rust
use applib::components::{SplitView, SplitOrientation};

// Horizontal split with fixed left pane
SplitView::horizontal()
    .first(sidebar_content)
    .second(main_content)
    .first_size(px(250.0))
```

By default, when no first pane size is specified, both panes share the available space equally using flex layout. Setting a fixed size for the first pane makes it rigid while the second pane takes all remaining space.

## Choosing an Orientation

Create a horizontal split for side-by-side panes:

```rust
SplitView::horizontal()
    .first(left_pane)
    .second(right_pane)
```

Create a vertical split for stacked panes:

```rust
SplitView::vertical()
    .first(top_pane)
    .second(bottom_pane)
```

## Topics

### Creating a SplitView

- `horizontal()` — Creates a new horizontal split view (side-by-side panes).
- `vertical()` — Creates a new vertical split view (stacked panes).

### Configuring Content

- `first(_:)` — Sets the first (left or top) pane content.
- `second(_:)` — Sets the second (right or bottom) pane content.

### Controlling Size

- `first_size(_:)` — Sets a fixed size for the first pane.

### Styling the Divider

- `divider_color(_:)` — Sets the divider color.

## Size Behavior

When `first_size()` is set:
- The first pane has a fixed width (horizontal) or height (vertical)
- The second pane fills remaining space
- The first pane will not shrink when the container is resized

When `first_size()` is not set:
- Both panes share space equally using flex: 1
- Both panes resize proportionally when the container changes size

## See Also

- ScrollView
- Panel
- Form
