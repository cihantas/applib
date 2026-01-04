# Text

A view for displaying styled text content.

## Overview

Text provides a flexible way to display text with customizable typography, including predefined styles for titles, headlines, body text, and more. It supports various customization options including color, font weight, alignment, line limits, and monospace fonts.

Use Text when you need to display any textual content in your interface. The component provides semantic text styles that maintain visual hierarchy and consistency across your application.

```rust
Text::new("Hello, World!")
    .headline()
    .color(text_primary())
```

For multi-line text with overflow control:

```rust
Text::new("This is a longer piece of text that might need to be truncated.")
    .line_limit(2)
    .align(TextAlign::Center)
```

## Topics

### Creating a Text View

- `new(_:)` — Creates a new Text view with the given content.

### Configuring Text Style

- `style(_:)` — Sets the text style using a TextStyle variant.
- `title()` — Sets the text style to title (22px).
- `headline()` — Sets the text style to headline (15px bold).
- `subheadline()` — Sets the text style to subheadline (13px).
- `body()` — Sets the text style to body (13px, default).
- `caption()` — Sets the text style to caption (11px).
- `footnote()` — Sets the text style to footnote (10px).

### Configuring Appearance

- `color(_:)` — Sets the text color.
- `weight(_:)` — Sets the font weight.
- `monospace()` — Sets the font to monospace.

### Configuring Layout

- `align(_:)` — Sets the text alignment (left, center, right).
- `line_limit(_:)` — Sets the maximum number of lines to display.

### Text Styles

- `TextStyle::Title` — Large title text (22px).
- `TextStyle::Headline` — Headline text (15px bold).
- `TextStyle::Subheadline` — Subheadline text (13px).
- `TextStyle::Body` — Body text (13px, default).
- `TextStyle::Caption` — Caption text (11px).
- `TextStyle::Footnote` — Footnote text (10px).

### Text Alignment

- `TextAlign::Left` — Align text to the left (default).
- `TextAlign::Center` — Center align text.
- `TextAlign::Right` — Align text to the right.

### Color Helpers

- `text_primary()` — Primary text color (dark gray).
- `text_secondary()` — Secondary text color (medium gray).
- `text_tertiary()` — Tertiary text color (light gray).
- `text_accent()` — Accent text color (muted blue).
- `text_link()` — Link text color (blue).

## See Also

- Label
- Link
