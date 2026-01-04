# ScrollView

A scrollable container for content that exceeds its bounds.

## Overview

ScrollView provides scrolling capabilities for content larger than its container. You can configure scrolling along the vertical axis, horizontal axis, or both.

```rust
use applib::components::ScrollView;

ScrollView::vertical("content-scroll")
    .child(large_content_view)
```

Every ScrollView requires a unique element ID for GPUI to track its scroll state. Use descriptive IDs that reflect the content being scrolled.

## Choosing a Scroll Direction

Create a vertical scroll view (most common):

```rust
ScrollView::vertical("my-scroll")
    .child(content)
```

Create a horizontal scroll view:

```rust
ScrollView::horizontal("timeline-scroll")
    .child(wide_content)
```

Create a scroll view that scrolls in both directions:

```rust
ScrollView::both("canvas-scroll")
    .child(large_canvas)
```

## Topics

### Creating a ScrollView

- `vertical(_:)` — Creates a new vertical scroll view with the given ID.
- `horizontal(_:)` — Creates a new horizontal scroll view with the given ID.
- `both(_:)` — Creates a new scroll view that scrolls in both directions.

### Configuring Scroll Behavior

- `axis(_:)` — Sets the scroll axis.

### Managing Content

- `child(_:)` — Adds a child element to the scroll view.
- `children(_:)` — Adds multiple children to the scroll view.

## Usage Notes

The element ID is required because GPUI maintains scroll position state across renders. Using stable, unique IDs ensures scroll position is preserved correctly.

Content added via `child()` or `children()` is laid out in a flex column (for vertical/both) or flex row (for horizontal) within the scrollable container.

## See Also

- SplitView
- List
- Form
