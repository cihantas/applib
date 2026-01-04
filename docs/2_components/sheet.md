# Sheet

A modal overlay that displays content with a semi-transparent backdrop.

## Overview

Sheet displays content in a centered panel with polished styling, positioned over a darkened backdrop. Sheets are commonly used for dialogs, forms, and confirmation prompts that require user attention without fully blocking the application.

```rust
use applib::components::Sheet;

Sheet::new("create-item-sheet")
    .title("Create New Item")
    .child(form_content)
    .actions(button_row)
    .on_dismiss(cx.listener(|this, _event, _window, cx| {
        this.show_sheet = false;
        cx.notify();
    }))
```

The sheet panel is 400 pixels wide by default with a maximum height of 600 pixels. Content is scrollable if it exceeds this height. Clicking the backdrop dismisses the sheet by calling the `on_dismiss` handler.

## Sheet Structure

A complete sheet consists of three optional sections:

- **Title bar**: Displays a centered title with bottom border separator
- **Content area**: Contains the main sheet content with 20px padding
- **Actions bar**: Contains action buttons (typically Cancel and a primary action)

```rust
Sheet::new("example")
    .title("Title Bar")
    .child(content)
    .actions(
        div()
            .flex()
            .flex_row()
            .gap(px(8.0))
            .child(cancel_button)
            .child(confirm_button)
    )
```

## Topics

### Creating a Sheet

- `new(_:)` — Creates a new sheet with the given ID.

### Configuring Appearance

- `title(_:)` — Sets the title displayed at the top of the sheet.
- `width(_:)` — Sets the width of the sheet panel.

### Managing Content

- `child(_:)` — Adds a child element to the sheet content area.
- `children(_:)` — Adds multiple children to the sheet content area.
- `actions(_:)` — Sets the actions element (typically a row of buttons) at the bottom.

### Handling Dismissal

- `on_dismiss(_:)` — Sets the dismiss handler, called when the backdrop is clicked.

## Visual Design

The sheet follows a polished aesthetic:

- Semi-transparent dark backdrop (40% opacity black)
- Light panel background (#F7F7F7)
- 10px rounded corners
- Layered shadows for depth
- Subtle borders and separators

The panel prevents click propagation, so clicking inside the sheet content does not trigger dismissal.

## See Also

- Alert
- Form
- Panel
