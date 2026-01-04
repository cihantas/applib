# Badge

A small rounded badge for displaying counts or short labels.

## Overview

Badge provides a compact way to display numeric counts or short labels, typically used to show notification counts, unread items, or other numerical indicators. It renders as a rounded rectangle with white text on a gray background.

Use Badge to show counts next to sidebar items, toolbar buttons, or anywhere you need a small, visually distinct indicator.

```rust
Badge::new("3")
```

Badges automatically adjust their width to fit the content while maintaining a minimum size and consistent padding.

## Topics

### Creating a Badge

- `new(_:)` — Creates a new badge with the given label.

## See Also

- Label
- Text
