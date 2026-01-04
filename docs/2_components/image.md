# Image

A component for displaying images from various sources.

## Overview

Image provides a high-level interface for displaying images loaded from filesystem paths, URIs, or embedded resources. It supports multiple image formats and provides controls for sizing, corner radius, aspect ratio fitting, and visual effects like grayscale.

Use Image when you need to display photos, icons, or any raster graphics in your interface. The component handles loading and provides optional placeholder fallbacks.

```rust
// Load from file path
Image::new("avatar")
    .source("/path/to/avatar.png")
    .size(px(48.0))
    .corner_radius(px(24.0))
    .placeholder(Icon::Person)
```

For remote images:

```rust
Image::new("remote")
    .source("https://example.com/image.jpg")
    .size(px(200.0))
    .fit(ImageFit::Cover)
```

With custom dimensions:

```rust
Image::new("banner")
    .source("/path/to/banner.png")
    .w(px(800.0))
    .h(px(200.0))
```

## Topics

### Creating an Image

- `new(_:)` — Creates a new image with the given element ID.

### Configuring Source

- `source(_:)` — Sets the image source (path, URI, or embedded).

### Configuring Size

- `size(_:)` — Sets both width and height to the same value (creates a square).
- `w(_:)` — Sets the width.
- `h(_:)` — Sets the height.

### Configuring Appearance

- `corner_radius(_:)` — Sets the corner radius.
- `rounded()` — Makes the image fully rounded (circular if square).
- `fit(_:)` — Sets how the image should fit within its frame.
- `grayscale(_:)` — Renders the image in grayscale.
- `placeholder(_:)` — Sets a placeholder icon to show while loading or on error.

### Image Sources

- `ImageSource::Path(_:)` — Load image from a filesystem path.
- `ImageSource::Uri(_:)` — Load image from a URI (http/https).
- `ImageSource::Embedded(_:)` — Use an embedded resource.

### Image Fit Modes

- `ImageFit::Contain` — Scale to fit within bounds while maintaining aspect ratio (default).
- `ImageFit::Cover` — Scale to fill bounds while maintaining aspect ratio (may crop).
- `ImageFit::Fill` — Stretch to fill bounds (may distort).

### Supported Formats

Image supports the following formats:

- PNG, JPEG, GIF, WebP
- TIFF, TGA, BMP, ICO
- HDR, EXR, PBM, PAM, PPM, PGM
- AVIF, DDS, QOI, SVG

## See Also

- Canvas
- ColorView
- Icon
