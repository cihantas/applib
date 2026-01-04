# ProgressView

A progress indicator component for showing task completion.

## Overview

ProgressView displays the progress of a task, either as a determinate progress bar (when the completion percentage is known) or as an indeterminate indicator (when the duration is unknown). It supports both linear (bar) and circular styles.

Use ProgressView to provide feedback during long-running operations such as file downloads, data processing, or network requests.

```rust
// Indeterminate progress
ProgressView::indeterminate()
    .style(ProgressStyle::Circular)

// Determinate progress (50%)
ProgressView::new(0.5)
    .label("Downloading...")
```

For progress with absolute values:

```rust
ProgressView::new_with_total(50.0, 100.0)
    .style(ProgressStyle::Linear)
    .label("Processing files...")
```

## Topics

### Creating a Progress View

- `new(_:)` — Creates a determinate progress view with a value from 0.0 to 1.0.
- `new_with_total(_:_:)` — Creates a determinate progress view with value and total.
- `indeterminate()` — Creates an indeterminate progress view.

### Configuring Appearance

- `style(_:)` — Sets the visual style of the progress view.
- `label(_:)` — Sets an optional label to display alongside the indicator.

### Progress Styles

- `ProgressStyle::Linear` — Horizontal progress bar (default).
- `ProgressStyle::Circular` — Circular progress indicator.

## See Also

- EmptyState
- LoadingIndicator
