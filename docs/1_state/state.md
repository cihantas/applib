# State&lt;T&gt;

Observable state container that automatically notifies on changes.

## Overview

`State<T>` wraps a value and automatically triggers UI re-renders when it changes. When you call `set()` or `update()`, it automatically calls `cx.notify()` behind the scenes.

This is designed to work with GPUI's `Entity` system - wrap your `State<T>` in an `Entity` for reactive updates.

## Example

```rust
use applib::prelude::*;

struct MyView {
    counter: Entity<State<i32>>,
}

impl MyView {
    fn new(cx: &mut App) -> Self {
        Self {
            counter: cx.new(|_| State::new(0)),
        }
    }

    fn increment(&mut self, cx: &mut App) {
        self.counter.update(cx, |state, cx| {
            state.set(state.get() + 1, cx);  // Auto-notifies
        });
    }
}
```

## Topics

### Creating State

**`new(value: T) -> Self`**

Creates a new state with an initial value:

```rust
let state = cx.new(|_| State::new(0));
```

**`default() -> Self`** *(requires `T: Default`)*

Creates a state with the default value:

```rust
let state = cx.new(|_| State::<String>::default());
```

### Reading Values

**`get(&self) -> &T`**

Returns a reference to the current value:

```rust
let value = state.read(cx).get();
```

**`get_mut(&mut self) -> &mut T`**

Returns a mutable reference without notifying:

```rust
state.update(cx, |state, _cx| {
    let value = state.get_mut();
    // Direct mutation - no notification
});
```

> **Note**: `get_mut()` does NOT trigger notifications. Use `set()` or `update()` for reactive updates.

### Updating Values

**`set(&mut self, value: T, cx: &mut Context<Self>)`**

Sets a new value and notifies observers:

```rust
self.counter.update(cx, |state, cx| {
    state.set(42, cx);  // Triggers re-render
});
```

**`update(&mut self, f: impl FnOnce(&mut T), cx: &mut Context<Self>)`**

Updates the value in place and notifies:

```rust
self.counter.update(cx, |state, cx| {
    state.update(|n| *n += 1, cx);  // Increment and notify
});
```

### Creating Bindings

**`binding(entity: &Entity<Self>, cx: &App) -> Binding<T>`** *(requires `T: Clone`)*

Creates a two-way binding to this state:

```rust
let query: Entity<State<String>> = cx.new(|_| State::new(String::new()));

TextField::new("search")
    .text(State::binding(&query, cx))
```

See [Binding<T>](binding.md) for more details.

### Cloning

**`clone(&self) -> Self`** *(requires `T: Clone`)*

Clones the state value (not the Entity):

```rust
let state1 = State::new(42);
let state2 = state1.clone();  // Independent copy
```

> **Note**: Cloning a `State<T>` creates a new independent state. To share state between components, pass the `Entity<State<T>>` instead.

## See Also

- [Binding<T>](binding.md) - Two-way bindings for components
- [TextField](../2_components/text_field.md) - Component that accepts bindings
- [List](../2_components/list.md) - Component that accepts bindings
