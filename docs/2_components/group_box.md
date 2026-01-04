# GroupBox

A labeled container for visually grouping related content.

## Overview

GroupBox provides a bordered container with optional title that groups related controls together, following a subtle design aesthetic. It's similar to SwiftUI's GroupBox or HTML's fieldset/legend pattern.

```rust
use applib::components::GroupBox;

GroupBox::new()
    .title("Display Settings")
    .child(brightness_slider)
    .child(contrast_slider)
    .child(auto_adjust_toggle)
```

Group boxes can also be used without titles for simple bordered containers:

```rust
GroupBox::new()
    .child(content_one)
    .child(content_two)
```

## Visual Design

GroupBox features polished styling:

- Light background (#FAFAFA)
- Subtle border (#D1D1D1)
- 6px rounded corners
- Soft shadow for depth
- Optional title bar with separator

When a title is present, it appears in a distinct header section with a bottom border. Content is laid out vertically with 8-pixel gaps and 12-pixel padding by default.

## Topics

### Creating a GroupBox

- `new()` — Creates a new group box.

### Configuring Appearance

- `title(_:)` — Sets the title displayed at the top of the group box.
- `padding(_:)` — Sets the padding inside the content area.

### Managing Content

- `child(_:)` — Adds a child element to the group box.
- `children(_:)` — Adds multiple children to the group box.

## Usage Patterns

Group boxes work well for organizing settings panels:

```rust
GroupBox::new()
    .title("Notifications")
    .padding(px(16.0))
    .child(Toggle::new("email", "Email notifications", enabled))
    .child(Toggle::new("sound", "Sound alerts", enabled))
```

Or for creating visual sections in forms:

```rust
Form::new()
    .child(
        GroupBox::new()
            .title("Account")
            .child(name_field)
            .child(email_field)
    )
    .child(
        GroupBox::new()
            .title("Privacy")
            .child(privacy_controls)
    )
```

## See Also

- Section
- Form
- Panel
