# Binding&lt;T&gt;

Two-way binding primitive for reactive UI components.

## Overview

`Binding<T>` provides a reference to a `State<T>` that can be passed to components, enabling automatic two-way synchronization. When a component updates a binding, the underlying state is updated and observers are notified automatically.

This eliminates the need for manual callback wiring - components can both read and write state through a binding.

## Example

```rust
use applib::prelude::*;

struct MyView {
    query: Entity<State<String>>,
}

impl Render for MyView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        TextField::new("search")
            .text(State::binding(&self.query, cx))  // Two-way binding
    }
}
```

When the user types in the TextField, the `query` state is automatically updated.

## Topics

### Creating Bindings

**`new(entity: Entity<State<T>>, cx: &App) -> Self`**

Creates a new binding to the given state entity:

```rust
let binding = Binding::new(state_entity.clone(), cx);
```

> **Tip**: Use `State::binding(&entity, cx)` instead - it's more ergonomic.

**`State::binding(entity: &Entity<State<T>>, cx: &App) -> Binding<T>`**

The preferred way to create bindings:

```rust
let query = cx.new(|_| State::new(String::new()));
let binding = State::binding(&query, cx);
```

### Reading Values

**`get(&self, cx: &App) -> T`**

Returns a clone of the current value:

```rust
let value = binding.get(cx);
```

### Updating Values

**`set(&self, value: T, cx: &mut App)`**

Sets a new value and notifies observers:

```rust
binding.set("new value".to_string(), cx);
```

**`update(&self, f: impl FnOnce(&mut T), cx: &mut App)`**

Updates the value using a closure:

```rust
binding.update(|s| s.push_str(" world"), cx);
```

**`reset(&self, cx: &mut App)`** *(requires `T: Default`)*

Resets the value to its default:

```rust
binding.reset(cx);  // Sets to T::default()
```

### Accessing the Entity

**`entity(&self) -> &Entity<State<T>>`**

Returns a reference to the underlying entity:

```rust
let entity = binding.entity();
// Use for observation or other GPUI operations
```

## String Bindings

`Binding<String>` has additional convenience methods:

**`is_empty(&self, cx: &App) -> bool`**

Returns true if the string is empty:

```rust
if query.is_empty(cx) {
    // Show placeholder
}
```

**`clear(&self, cx: &mut App)`**

Clears the string:

```rust
query.clear(cx);
```

## Boolean Bindings

`Binding<bool>` has a toggle method:

**`toggle(&self, cx: &mut App)`**

Toggles the boolean value:

```rust
enabled.toggle(cx);  // true -> false or false -> true
```

## Option Bindings

`Binding<Option<T>>` has convenience methods:

**`is_none(&self, cx: &App) -> bool`**

Returns true if the option is None:

```rust
if selection.is_none(cx) {
    // Nothing selected
}
```

**`is_some(&self, cx: &App) -> bool`**

Returns true if the option is Some:

```rust
if selection.is_some(cx) {
    // Something is selected
}
```

**`clear(&self, cx: &mut App)`**

Sets the option to None:

```rust
selection.clear(cx);
```

## See Also

- [State<T>](state.md) - Observable state container
- [TextField](../2_components/text_field.md) - Accepts text bindings
- [List](../2_components/list.md) - Accepts selection bindings
