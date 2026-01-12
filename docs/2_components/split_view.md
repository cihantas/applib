# SplitView

A stateful two-pane layout with a draggable divider for resizing.

## Overview

SplitView provides a resizable two-pane layout with a draggable divider. It manages its own internal state via `SplitViewState` and is used as an `Entity` in your parent view.

```rust
struct MyView {
    split: Entity<SplitViewState>,
}

impl MyView {
    fn new(cx: &mut Context<Self>) -> Self {
        Self {
            split: cx.new(|_| SplitViewState::from(
                SplitView::horizontal()
                    .first_size(px(250.0))
                    .min_first_size(px(150.0))
                    .max_first_size(px(400.0))
            )),
        }
    }
}

impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.split.update(cx, |split, _| {
            split.set_first(sidebar_content);
            split.set_second(main_content);
        });

        self.split.clone()
    }
}
```

## Topics

### Creating a SplitView

- `SplitView::horizontal()` - Creates a horizontal split (side-by-side panes).
- `SplitView::vertical()` - Creates a vertical split (stacked panes).

### Configuring Size

- `first_size(_:)` - Sets the initial size for the first pane.
- `min_first_size(_:)` - Sets the minimum size for the first pane.
- `max_first_size(_:)` - Sets the maximum size for the first pane.

### Styling

- `divider_color(_:)` - Sets the divider color.

### Events

- `on_resize(_:)` - Sets a callback when the user resizes the panes.

### SplitViewState Methods

- `first_size()` - Gets the current first pane size.
- `set_first_size(_:)` - Sets the first pane size programmatically.
- `set_first(_:)` - Sets the first pane content (call each render).
- `set_second(_:)` - Sets the second pane content (call each render).

## Builder Pattern

Use `SplitView` as a builder, then convert to `SplitViewState` for use as an Entity:

```rust
// Create the builder
let builder = SplitView::horizontal()
    .first_size(px(250.0))
    .min_first_size(px(150.0))
    .max_first_size(px(400.0));

// Convert to state and wrap in Entity
let split = cx.new(|_| SplitViewState::from(builder));
```

## Setting Content Each Render

Because `SplitViewState` is an Entity, you must set content each render cycle:

```rust
impl Render for MyView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Update content each render
        self.split.update(cx, |split, _| {
            split.set_first(div().child("Left pane"));
            split.set_second(div().child("Right pane"));
        });

        // Return the entity (it implements IntoElement via Render)
        self.split.clone()
    }
}
```

## Resize Callback

Get notified when the user drags the divider:

```rust
SplitView::horizontal()
    .first_size(px(250.0))
    .on_resize(|new_size, _window, _cx| {
        println!("Resized to: {:?}", new_size);
        // Persist the size, update other state, etc.
    })
```

## Programmatic Resizing

Update the split size from code:

```rust
self.split.update(cx, |split, cx| {
    split.set_first_size(px(300.0));
    cx.notify();
});
```

## Nested Split Views

Split views can be nested. Each SplitView instance manages its own drag state independently - dragging a nested divider won't affect parent split views.

```rust
// Inner split (vertical)
let inner_split = cx.new(|_| SplitViewState::from(
    SplitView::vertical().first_size(px(200.0))
));

// Outer split (horizontal) with inner split as second pane
self.outer_split.update(cx, |split, _| {
    split.set_first(sidebar);
    split.set_second(inner_split.clone());
});
```

## See Also

- NavigationSplitView
- Panel
- ScrollView
