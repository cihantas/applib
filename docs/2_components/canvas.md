# Canvas

A view for custom drawing with immediate mode graphics.

## Overview

Canvas provides a drawing context that allows for custom rendering of shapes, paths, and text. It collects drawing commands during a callback and renders them efficiently using GPUI's painting system.

Use Canvas when you need to create custom visualizations, charts, diagrams, or any custom graphics that cannot be easily composed from standard UI components.

```rust
Canvas::new("my-canvas", |ctx| {
    // Fill a blue rectangle
    ctx.fill_rect(
        Rect2D::new(10.0, 10.0, 100.0, 50.0),
        hsla(211.0 / 360.0, 0.95, 0.53, 1.0),
    );

    // Draw a red circle
    ctx.fill_circle(
        Point2D::new(80.0, 80.0),
        30.0,
        hsla(0.0, 0.85, 0.55, 1.0),
    );

    // Draw a line
    ctx.line(
        Point2D::new(0.0, 0.0),
        Point2D::new(100.0, 100.0),
        StrokeStyle::new(hsla(0.0, 0.0, 0.0, 1.0), 2.0),
    );
})
.size(px(200.0), px(200.0))
```

For path-based drawing:

```rust
Canvas::new("path-canvas", |ctx| {
    let path = PathBuilder::new(Point2D::new(50.0, 10.0))
        .line_to(Point2D::new(90.0, 90.0))
        .line_to(Point2D::new(10.0, 90.0))
        .close();

    ctx.fill_path(path, hsla(0.3, 0.8, 0.5, 1.0));
})
```

## Topics

### Creating a Canvas

- `new(_:_:)` — Creates a new canvas with a drawing callback.
- `empty(_:)` — Creates a new canvas without a drawing callback.

### Configuring Size

- `size(_:_:)` — Sets both width and height of the canvas.
- `width(_:)` — Sets the width of the canvas.
- `height(_:)` — Sets the height of the canvas.

### Configuring Appearance

- `background(_:)` — Sets the background color of the canvas.

### Drawing Context

- `DrawingContext` — Provides methods for drawing shapes, paths, and text.

### Drawing Shapes

- `fill_rect(_:_:)` — Fills a rectangle with a color.
- `fill_rounded_rect(_:_:_:)` — Fills a rounded rectangle.
- `stroke_rect(_:_:)` — Strokes a rectangle outline.
- `stroke_rounded_rect(_:_:_:)` — Strokes a rounded rectangle outline.
- `fill_ellipse(_:_:)` — Fills an ellipse inscribed in a rectangle.
- `stroke_ellipse(_:_:)` — Strokes an ellipse outline.
- `fill_circle(_:_:_:)` — Fills a circle.
- `stroke_circle(_:_:_:)` — Strokes a circle outline.
- `line(_:_:_:)` — Draws a line between two points.

### Drawing Paths

- `fill_path(_:_:)` — Fills a path.
- `stroke_path(_:_:)` — Strokes a path.

### Drawing Text

- `text(_:_:_:)` — Draws text at the given position.
- `text_sized(_:_:_:_:)` — Draws text with a custom size.

### Path Building

- `PathBuilder::new(_:)` — Creates a new path builder starting at the given point.
- `PathBuilder::new_at_origin()` — Creates a path builder starting at the origin.
- `PathBuilder::rect(_:)` — Creates a rectangular path.
- `PathBuilder::ellipse(_:)` — Creates an ellipse path.
- `PathBuilder::circle(_:_:)` — Creates a circular path.

### Path Operations

- `move_to(_:)` — Moves to a new point without drawing.
- `line_to(_:)` — Draws a line from the current point.
- `curve_to(_:_:)` — Draws a quadratic Bézier curve.
- `quad_to(_:_:)` — Draws a quadratic curve with a single control point.
- `arc(_:_:_:_:)` — Adds an arc segment.
- `close()` — Closes the path by drawing back to the start.

### Geometry Types

- `Point2D` — A point in 2D space.
- `Size2D` — A size in 2D space.
- `Rect2D` — A rectangle defined by origin and size.
- `CornerRadii` — Corner radii for rounded rectangles.

### Style Types

- `StrokeStyle` — A stroke style for drawing paths and shapes.
- `FillStyle` — A fill style for shapes (solid or gradient).

## See Also

- ColorView
- Image
