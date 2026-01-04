# Debouncer

A utility for delaying callback execution until a specified time has passed without new calls.

## Overview

Debouncing is a technique for limiting how often a function executes in response to high-frequency events. The `Debouncer` delays callback execution until activity has stopped for a specified duration, making it ideal for events like text input, window resizing, or scroll interactions where you want to wait until the user has finished before processing.

Each call to `call()` cancels any pending callback and starts a new timer. The callback only executes if no new calls arrive within the delay period.

```rust
use applib::utils::Debouncer;
use std::time::Duration;

struct SearchView {
    query: String,
    debouncer: Debouncer,
}

impl SearchView {
    fn new() -> Self {
        Self {
            query: String::new(),
            debouncer: Debouncer::from_millis(150),
        }
    }

    fn on_text_change(&mut self, text: String, cx: &mut App) {
        self.query = text.clone();
        self.debouncer.call(cx, move |cx| {
            // This only fires 150ms after the last keystroke
            perform_search(&text, cx);
        });
    }
}
```

The debouncer automatically manages task lifecycle, canceling pending callbacks when new ones are scheduled or when the debouncer is dropped.

## Topics

### Creating a Debouncer

- `new(_:)` — Creates a debouncer with the specified delay duration.
- `from_millis(_:)` — Creates a debouncer with a delay specified in milliseconds.
- `default()` — Creates a debouncer with a 150ms delay.

### Scheduling Callbacks

- `call(_:_:)` — Schedules a callback to execute after the delay, canceling any pending callback.

### Managing State

- `cancel()` — Cancels any pending callback.
- `is_pending()` — Returns true if a callback is waiting to execute.

## Creating a Debouncer

### `new(_:)`

Creates a debouncer with the specified delay.

```rust
let debouncer = Debouncer::new(Duration::from_millis(150));
```

**Parameters:**
- `delay`: The time to wait after the last call before executing the callback.

**Returns:** A new `Debouncer` instance.

---

### `from_millis(_:)`

Creates a debouncer with a delay specified in milliseconds.

```rust
// 150ms delay, good for search input
let search_debouncer = Debouncer::from_millis(150);

// 100ms delay, good for resize handlers
let resize_debouncer = Debouncer::from_millis(100);
```

**Parameters:**
- `millis`: The delay in milliseconds.

**Returns:** A new `Debouncer` instance.

---

### `default()`

Creates a debouncer with a default delay of 150ms.

```rust
let debouncer = Debouncer::default();
```

This is a reasonable default for search input debouncing.

**Returns:** A new `Debouncer` instance with 150ms delay.

## Scheduling Callbacks

### `call(_:_:)`

Schedules a callback to execute after the delay.

```rust
debouncer.call(cx, |cx| {
    // Expensive operation here
    perform_search(cx);
});
```

If a callback is already pending, it will be cancelled and replaced with the new one. The callback executes only if no new calls to `call()` arrive within the delay period.

**Parameters:**
- `cx`: The GPUI application context.
- `callback`: The function to execute after the delay.

## Managing State

### `cancel()`

Cancels any pending callback.

```rust
// Cancel pending search when user clears the input
if query.is_empty() {
    debouncer.cancel();
}
```

Use this when you want to stop a debounced operation from executing, for example when a component is being unmounted or when the operation is no longer relevant.

---

### `is_pending()`

Returns true if a callback is waiting to execute.

```rust
if debouncer.is_pending() {
    show_loading_indicator();
}
```

**Returns:** `true` if a callback is scheduled, `false` otherwise.

## Common Use Cases

### Search Input

Delay search queries until the user stops typing:

```rust
struct SearchBar {
    debouncer: Debouncer,
}

impl SearchBar {
    fn new() -> Self {
        Self {
            debouncer: Debouncer::from_millis(150),
        }
    }

    fn on_input(&mut self, query: String, cx: &mut App) {
        self.debouncer.call(cx, move |cx| {
            fetch_search_results(&query, cx);
        });
    }
}
```

### Window Resize

Delay layout recalculation until resizing stops:

```rust
struct ResponsiveView {
    debouncer: Debouncer,
}

impl ResponsiveView {
    fn new() -> Self {
        Self {
            debouncer: Debouncer::from_millis(100),
        }
    }

    fn on_resize(&mut self, size: Size, cx: &mut App) {
        self.debouncer.call(cx, move |cx| {
            recalculate_layout(size, cx);
        });
    }
}
```

### Auto-save

Delay saving until the user stops editing:

```rust
struct Editor {
    debouncer: Debouncer,
}

impl Editor {
    fn new() -> Self {
        Self {
            debouncer: Debouncer::from_millis(500),
        }
    }

    fn on_content_change(&mut self, content: String, cx: &mut App) {
        self.debouncer.call(cx, move |cx| {
            save_document(&content, cx);
        });
    }
}
```

## See Also

- [GPUI Documentation](https://www.gpui.rs/)
- [Throttling vs Debouncing](https://css-tricks.com/debouncing-throttling-explained-examples/)
