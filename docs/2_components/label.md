# Label

A component combining an icon and text in a horizontal layout.

## Overview

Label displays an icon alongside text, similar to labeled buttons and menu items in standard interfaces. It provides a consistent way to present iconography with descriptive text.

Use Label when you need to combine an icon with text, such as in navigation items, buttons, or list entries. The component can display the icon and text together, or show only the icon or text individually.

```rust
Label::new("Favorites", Icon::Heart)
```

You can customize the display style to show only specific parts:

```rust
// Show only the icon
Label::new("Edit", Icon::Pencil)
    .icon_only()

// Show only the text
Label::new("Settings", Icon::Gear)
    .title_only()
```

## Topics

### Creating a Label

- `new(_:_:)` — Creates a new label with a title and icon.
- `with_icon_str(_:_:)` — Creates a new label with a custom icon string.
- `title_only_new(_:)` — Creates a label with only a title (no icon).

### Configuring Display Style

- `style(_:)` — Sets the label style (both, icon only, or title only).
- `icon_only()` — Sets the label to show only the icon.
- `title_only()` — Sets the label to show only the title.

### Configuring Colors

- `text_color(_:)` — Sets the text color for both icon and title.
- `icon_color(_:)` — Sets a separate color for the icon.

### Label Styles

- `LabelStyle::Both` — Show both icon and title (default).
- `LabelStyle::IconOnly` — Show only the icon.
- `LabelStyle::TitleOnly` — Show only the title.

### Icons

- `Icon` — An enum of common icons for use with labels and other components.

### Common Icons

- `Icon::Heart` — Heart icon (favorites).
- `Icon::Star` — Star icon (ratings).
- `Icon::Folder` — Folder icon (directories).
- `Icon::Document` — Document/file icon.
- `Icon::Gear` — Gear/settings icon.
- `Icon::Pencil` — Pencil/edit icon.
- `Icon::Trash` — Trash/delete icon.
- `Icon::Plus` — Plus/add icon.
- `Icon::Minus` — Minus/remove icon.
- `Icon::Checkmark` — Checkmark icon.
- `Icon::XMark` — X/close icon.
- `Icon::MagnifyingGlass` — Search icon.
- `Icon::Person` — Person/user icon.
- `Icon::Clock` — Clock/time icon.
- `Icon::Tag` — Tag icon.
- `Icon::Branch` — Branch icon (git).
- `Icon::Commit` — Commit icon (git).
- `Icon::ArrowUp` — Arrow up icon.
- `Icon::ArrowDown` — Arrow down icon.
- `Icon::ArrowLeft` — Arrow left icon.
- `Icon::ArrowRight` — Arrow right icon.
- `Icon::Refresh` — Refresh/reload icon.
- `Icon::Info` — Info icon.
- `Icon::Warning` — Warning icon.
- `Icon::Error` — Error/exclamation icon.

## See Also

- Text
- Badge
- Button
