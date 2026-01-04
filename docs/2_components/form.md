# Form

A container for grouping and arranging form controls.

## Overview

Form provides a vertical layout with consistent spacing between fields, following the platform form design patterns. Use Form to create settings panels, dialogs, and data entry interfaces with proper visual hierarchy and alignment.

```rust
use applib::components::Form;

Form::new()
    .child(name_field)
    .child(email_field)
    .child(subscribe_toggle)
```

Forms automatically space children vertically with 16-pixel gaps and include subtle background styling. Controls are laid out in a flex column for natural vertical stacking.

## Organizing with Sections

Use FormSection to group related fields with headers:

```rust
Form::new()
    .section("Account", |section| {
        section
            .child(username_field)
            .child(password_field)
    })
    .section("Preferences", |section| {
        section
            .child(theme_picker)
            .child(language_picker)
    })
```

Sections display uppercase header text with a divider line and slightly tighter spacing (12px) than the main form.

## Label-Control Alignment with FormRow

For grid-style forms with labels on the left and controls on the right, use FormRow:

```rust
Form::new()
    .label_width(px(120.0))
    .child(
        FormRow::new("Name")
            .child(text_field)
    )
    .child(
        FormRow::new("Email")
            .child(email_field)
    )
```

FormRow aligns labels to a fixed width (120px by default) with controls taking remaining horizontal space.

## Topics

### Creating a Form

- `new()` — Creates a new form container.

### Configuring Layout

- `label_width(_:)` — Sets the fixed width for labels in FormRow components.
- `spacing(_:)` — Sets the vertical spacing between form fields.
- `padding(_:)` — Sets padding around the form.
- `no_padding()` — Removes padding from the form.

### Managing Content

- `child(_:)` — Adds a child element to the form.
- `children(_:)` — Adds multiple children to the form.
- `section(_:_:)` — Adds a section to the form with a header and grouped fields.

### FormSection Methods

- `FormSection::new(_:)` — Creates a new form section with the given title.
- `spacing(_:)` — Sets the vertical spacing between fields in the section.
- `collapsed(_:)` — Sets whether the section is initially collapsed.
- `child(_:)` — Adds a child element to the section.
- `children(_:)` — Adds multiple children to the section.

### FormRow Methods

- `FormRow::new(_:)` — Creates a new form row with the given label.
- `label_width(_:)` — Sets a custom width for the label.
- `child(_:)` — Adds a child control to the row.
- `children(_:)` — Adds multiple children to the row.

## Complete Example

```rust
Form::new()
    .padding(px(20.0))
    .section("User Information", |section| {
        section
            .child(
                FormRow::new("Full Name")
                    .child(TextField::new("name", cx))
            )
            .child(
                FormRow::new("Email")
                    .child(TextField::new("email", cx))
            )
    })
    .section("Settings", |section| {
        section
            .child(Toggle::new("notifications", "Enable notifications", enabled))
            .child(Toggle::new("newsletter", "Subscribe to newsletter", enabled))
    })
```

## See Also

- FormSection
- FormRow
- Section
- GroupBox
