# Panel

A builder for creating floating panel windows.

## Overview

Panel provides a high-level API for creating floating windows with GPUI, making it easy to build launchers, dialogs, and popup interfaces using a fluent builder pattern. Panels wrap GPUI's window creation with sensible defaults for floating windows.

```rust
use applib::components::Panel;

Panel::new("launcher", |cx| LauncherView::new(cx))
    .size(px(600.0), px(400.0))
    .center_on_screen()
    .floating(true)
    .hide_titlebar()
    .open(cx)?
```

Panels are configured before opening with chainable builder methods. The view builder function is called when `open()` is invoked to create the root view for the window.

## Creating Floating Windows

Panels default to floating (popup) windows that appear above other windows:

```rust
Panel::new("quick-open", |cx| QuickOpenView::new(cx))
    .size(px(500.0), px(350.0))
    .center_on_screen()
    .open(cx)?
```

For standard windows that participate in normal window stacking:

```rust
Panel::new("document", |cx| DocumentView::new(cx))
    .size(px(800.0), px(600.0))
    .floating(false)
    .open(cx)?
```

## Custom Window Chrome

Hide the system titlebar to implement custom window decorations:

```rust
Panel::new("custom-window", |cx| CustomView::new(cx))
    .size(px(600.0), px(400.0))
    .hide_titlebar()
    .open(cx)?
```

When the titlebar is hidden, the panel uses client-side decorations, allowing you to implement custom title bars with TitleBar and window frames with WindowFrame.

## Topics

### Creating a Panel

- `new(_:_:)` — Creates a new panel builder with the given ID and view builder function.

### Configuring Size and Position

- `size(_:_:)` — Sets the size of the panel window.
- `center_on_screen()` — Centers the panel on the screen when opened.

### Configuring Behavior

- `floating(_:)` — Sets whether the panel should be a floating window.
- `hide_titlebar()` — Hides the system titlebar for the panel.

### Opening the Panel

- `open(_:)` — Opens the panel window with the configured options.

## Default Configuration

Newly created panels have these defaults:

- Width: 400px
- Height: 300px
- Floating: true (uses WindowKind::PopUp)
- Centered: false
- Titlebar: visible
- Focus: true (window receives focus when opened)

## Usage Notes

The view builder closure receives a mutable App context (`&mut App`) and must return a type that implements the `Render` trait. The returned view becomes the root view for the panel window.

The `open()` method consumes the builder and returns `Result<WindowHandle<V>>`, where `V` is your view type. Handle errors appropriately:

```rust
match Panel::new("my-panel", |cx| MyView::new(cx))
    .center_on_screen()
    .open(cx)
{
    Ok(handle) => {
        // Store or use the window handle
    }
    Err(e) => {
        eprintln!("Failed to open panel: {}", e);
    }
}
```

## See Also

- WindowFrame
- TitleBar
- Sheet
- Alert
