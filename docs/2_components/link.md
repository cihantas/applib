# Link

A clickable hyperlink component that opens URLs in the default browser.

## Overview

Link displays text or custom content as a clickable hyperlink with standard link styling. When clicked, it opens the specified URL in the system's default browser. The component automatically applies appropriate visual styling including blue color and underline on hover.

Use Link whenever you need to direct users to external web pages or resources. It handles the platform-specific details of opening URLs in the browser.

```rust
// Simple text link
Link::new("visit-website", "Visit Website", "https://example.com")
```

For links with custom content:

```rust
Link::with_content(
    "custom-link",
    "https://example.com",
    HStack::new()
        .child(Text::new("Open"))
        .child(Text::new("→"))
)
```

## Topics

### Creating a Link

- `new(_:_:_:)` — Creates a new link with an ID, text label, and URL.
- `with_content(_:_:_:)` — Creates a link with custom content.

## See Also

- Text
- Button
- Label
