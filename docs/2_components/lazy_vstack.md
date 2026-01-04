# LazyVStack

A lazy vertical stack that only renders visible items using virtualization.

## Overview

LazyVStack efficiently handles large lists by only rendering items currently visible in the viewport, plus a small buffer. This makes it ideal for lists with thousands of items where rendering all items would be prohibitively expensive.

The component uses GPUI's `uniform_list` infrastructure to provide automatic virtualization and scroll handling. Items are rendered on-demand as they become visible during scrolling.

```rust
LazyVStack::new("my-list", 10000, |range, _window, _cx| {
    range.map(|i| {
        div()
            .h(px(44.0))
            .child(format!("Row {}", i))
    }).collect()
})
.gap_3()
.p_4()
```

## Topics

### Creating a LazyVStack

- `new(_:_:_:)` — Creates a lazy vertical stack with an identifier, item count, and render closure.

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

The render closure receives a range of indices and returns a vector of elements. The range represents the currently visible items plus a buffer for smooth scrolling:

```rust
LazyVStack::new("contacts", contacts.len(), |range, _window, _cx| {
    range.map(|i| {
        let contact = &contacts[i];
        div()
            .h(px(60.0))
            .flex()
            .items_center()
            .px_4()
            .child(contact.name.clone())
    }).collect()
})
```

The render function is called whenever the visible range changes due to scrolling or resizing.

## Programmatic Scrolling

Use `LazyVStackScrollHandle` (a re-export of GPUI's `UniformListScrollHandle`) to programmatically control scroll position:

```rust
let scroll_handle = UniformListScrollHandle::new();

LazyVStack::new("list", items.len(), render_fn)
    .track_scroll(scroll_handle.clone());

// Later, scroll to specific item
scroll_handle.scroll_to_item(index);
```

## Performance Considerations

LazyVStack is optimized for uniform-height items. All items should have the same height for optimal performance and correct scrollbar behavior. For variable-height items, consider using standard layout components like VStack or List.

The component applies gaps between items using margin-bottom, except for the last item in each rendered range.

## See Also

- LazyHStack
- LazyVGrid
- List
