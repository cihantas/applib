# EmptyView

An invisible view that renders nothing and takes no space.

## Overview

EmptyView provides a way to represent the absence of content in a type-safe manner. It's particularly useful in conditional rendering scenarios where both branches of a condition must return the same type, but one branch has no content to display.

Use EmptyView when you need a placeholder that renders nothing, such as in the else branch of a conditional statement.

```rust
if show_content {
    content_view.into_any_element()
} else {
    EmptyView::new().into_any_element()
}
```

EmptyView is equivalent to an empty div but provides clearer semantic intent in your code.

## Topics

### Creating an Empty View

- `new()` — Creates a new empty view.

## See Also

- EmptyState
- Spacer
