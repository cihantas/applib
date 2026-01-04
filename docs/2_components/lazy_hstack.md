# LazyHStack

A lazy horizontal stack that only renders visible items using virtualization.

## Overview

LazyHStack provides horizontal virtualization for large collections of items. It uses GPUI's `uniform_list` infrastructure to render only visible items, making it efficient for horizontally scrolling lists with hundreds or thousands of items.

Note that GPUI's uniform_list is primarily designed for vertical scrolling, so LazyHStack wraps items within this infrastructure for horizontal display with automatic overflow scrolling.

```rust
LazyHStack::new("my-hlist", 1000, |range, _window, _cx| {
    range.map(|i| {
        div()
            .w(px(100.0))
            .h_full()
            .child(format!("Item {}", i))
    }).collect()
})
.gap_3()
.p_4()
```

## Topics

### Creating a LazyHStack

- `new(_:_:_:)` — Creates a lazy horizontal stack with an identifier, item count, and render closure.

### Configuring Spacing

- `gap(_:)` — Sets custom gap between children.
- `gap_3()` — Sets gap to 12px (0.75rem).
- `gap_6()` — Sets gap to 24px (1.5rem).

### Configuring Padding

- `p(_:)` — Sets custom padding around all children.
- `p_1()` — Sets padding to 4px (0.25rem).
- `p_2()` — Sets padding to 8px (0.5rem).
- `p_3()` — Sets padding to 12px (0.75rem).
- `p_4()` — Sets padding to 16px (1rem).

### Controlling Scroll Position

- `track_scroll(_:)` — Sets a scroll handle to programmatically control scroll position.

## Rendering Items

The render closure receives a range of indices and returns a vector of elements. Each item is automatically wrapped with `flex_shrink_0` to prevent it from shrinking:

```rust
LazyHStack::new("images", images.len(), |range, _window, _cx| {
    range.map(|i| {
        let image = &images[i];
        div()
            .w(px(200.0))
            .h(px(150.0))
            .child(Image::new(image.url.clone()))
    }).collect()
})
```

## Programmatic Scrolling

Use `LazyHStackScrollHandle` (a re-export of GPUI's `UniformListScrollHandle`) to programmatically control scroll position:

```rust
let scroll_handle = UniformListScrollHandle::new();

LazyHStack::new("list", items.len(), render_fn)
    .track_scroll(scroll_handle.clone());

// Later, scroll to specific item
scroll_handle.scroll_to_item(index);
```

## Layout Behavior

The component creates a horizontal flex container with overflow-x scrolling enabled. Items are rendered within a uniform_list where each list entry represents a horizontal item rather than a vertical row.

Gaps between items are applied using margin-right, except for the last item in each rendered range.

## Performance Considerations

LazyHStack is optimized for uniform-width items. All items should have the same width for optimal performance and correct scrollbar behavior. For variable-width items, consider using standard layout components like HStack.

## See Also

- LazyVStack
- LazyHGrid
- List
