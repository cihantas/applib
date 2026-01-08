# State Management

Reactive state primitives for GPUI applications, inspired by SwiftUI.

## Overview

The state module provides two core primitives for reactive state management:

- **[`State<T>`](state.md)** - Observable value container with automatic change notification
- **[`Binding<T>`](binding.md)** - Two-way reference for component bindings

These primitives eliminate manual `cx.notify()` calls and enable declarative, reactive UI patterns.

## Basic Example

```rust
use applib::prelude::*;

struct Counter {
    count: Entity<State<i32>>,
}

impl Counter {
    fn new(cx: &mut App) -> Self {
        Self {
            count: cx.new(|_| State::new(0)),
        }
    }
}

impl Render for Counter {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let count = self.count.read(cx).get();

        VStack::new()
            .child(Text::new(format!("Count: {}", count)))
            .child(
                Button::new("increment", "Increment")
                    .on_click(cx.listener(|this, _, _, cx| {
                        this.count.update(cx, |state, cx| {
                            state.update(|n| *n += 1, cx);
                        });
                    }))
            )
    }
}
```

## Topics

### Core Primitives

- [State<T>](state.md) - Observable state container
- [Binding<T>](binding.md) - Two-way bindings for components

### Patterns

**Local State**: Use `State<T>` for component-local reactive values:

```rust
struct MyView {
    selected: Entity<State<bool>>,
}
```

**Form Bindings**: Use `Binding<T>` to connect inputs to state:

```rust
TextField::new("name")
    .text(self.name.binding(cx))
```

**Shared State**: Pass `Entity<State<T>>` between components:

```rust
fn parent(cx: &mut App) -> impl IntoElement {
    let shared = cx.new(|_| State::new(0));

    VStack::new()
        .child(ChildA::new(shared.clone()))
        .child(ChildB::new(shared.clone()))
}
```

## See Also

- [TextField](../2_components/text_field.md) - Uses bindings for text input
- [List](../2_components/list.md) - Uses bindings for selection
