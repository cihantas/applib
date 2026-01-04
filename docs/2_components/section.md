# Section

A container for grouping content with optional header and footer text.

## Overview

Section groups related content with optional header and footer text, similar to SwiftUI's Section within List or Form. Sections provide visual organization without heavy borders, using subtle text styling to delineate groups.

```rust
use applib::components::Section;

Section::new()
    .header("RECENT ITEMS")
    .footer("Showing 10 of 50")
    .child(list_item_1)
    .child(list_item_2)
    .child(list_item_3)
```

Sections are commonly used within List or Form views to organize content into logical groups. Unlike GroupBox, sections use lightweight header/footer text rather than borders and backgrounds.

## Headers and Footers

Headers are displayed in uppercase, small font, medium weight, and gray color following the platform list section styling:

```rust
Section::new()
    .header("Settings")  // Will display in small, gray, uppercase
    .child(content)
```

Footers use smaller font and gray color for supplementary information:

```rust
Section::new()
    .header("Files")
    .footer("Last updated 2 minutes ago")
    .children(file_items)
```

## Custom Header and Footer Views

For more control over appearance, use custom views:

```rust
Section::new()
    .header_view(
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .child(icon)
            .child("Custom Header")
    )
    .child(content)
```

Custom views override any text set via `header()` or `footer()`.

## Topics

### Creating a Section

- `new()` — Creates a new empty section.

### Configuring Headers

- `header(_:)` — Sets the header text for the section.
- `header_view(_:)` — Sets a custom header view for the section.

### Configuring Footers

- `footer(_:)` — Sets the footer text for the section.
- `footer_view(_:)` — Sets a custom footer view for the section.

### Managing Content

- `child(_:)` — Adds a child element to the section content.
- `children(_:)` — Adds multiple children to the section content.

## Usage in Forms

Sections integrate seamlessly with Form containers:

```rust
Form::new()
    .child(
        Section::new()
            .header("BASIC INFORMATION")
            .child(name_field)
            .child(email_field)
    )
    .child(
        Section::new()
            .header("ADVANCED OPTIONS")
            .child(advanced_settings)
    )
```

## See Also

- Form
- FormSection
- GroupBox
- List
