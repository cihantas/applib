//! Canvas component for custom drawing with immediate mode graphics.
//!
//! This module provides a Canvas view that allows for custom drawing operations,
//! similar to SwiftUI's Canvas view.

use gpui::prelude::*;
use gpui::*;

/// A point in 2D space with floating-point coordinates.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point2D {
    pub x: f32,
    pub y: f32,
}

impl Point2D {
    /// Creates a new point at the given coordinates.
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a point at the origin (0, 0).
    pub fn zero() -> Self {
        Self { x: 0.0, y: 0.0 }
    }

    /// Converts to GPUI Point<Pixels>.
    pub fn to_pixels(self) -> Point<Pixels> {
        point(px(self.x), px(self.y))
    }
}

/// A size in 2D space with floating-point dimensions.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Size2D {
    pub width: f32,
    pub height: f32,
}

impl Size2D {
    /// Creates a new size with the given dimensions.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Creates a zero size.
    pub fn zero() -> Self {
        Self {
            width: 0.0,
            height: 0.0,
        }
    }

    /// Converts to GPUI Size<Pixels>.
    pub fn to_pixels(self) -> Size<Pixels> {
        size(px(self.width), px(self.height))
    }
}

/// A rectangle defined by its origin and size.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Rect2D {
    pub origin: Point2D,
    pub size: Size2D,
}

impl Rect2D {
    /// Creates a new rectangle with the given origin and size.
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self {
            origin: Point2D::new(x, y),
            size: Size2D::new(width, height),
        }
    }

    /// Creates a rectangle from origin point and size.
    pub fn from_origin_size(origin: Point2D, size: Size2D) -> Self {
        Self { origin, size }
    }

    /// Returns the center point of the rectangle.
    pub fn center(&self) -> Point2D {
        Point2D::new(
            self.origin.x + self.size.width / 2.0,
            self.origin.y + self.size.height / 2.0,
        )
    }

    /// Converts to GPUI Bounds<Pixels>.
    pub fn to_bounds(self) -> Bounds<Pixels> {
        Bounds {
            origin: self.origin.to_pixels(),
            size: self.size.to_pixels(),
        }
    }

    /// Creates a Rect2D from GPUI Bounds.
    pub fn from_bounds(bounds: Bounds<Pixels>) -> Self {
        Self {
            origin: Point2D::new(f32::from(bounds.origin.x), f32::from(bounds.origin.y)),
            size: Size2D::new(f32::from(bounds.size.width), f32::from(bounds.size.height)),
        }
    }
}

/// Corner radii for rounded rectangles.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CornerRadii {
    pub top_left: f32,
    pub top_right: f32,
    pub bottom_left: f32,
    pub bottom_right: f32,
}

impl CornerRadii {
    /// Creates corner radii with the same value for all corners.
    pub fn all(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_left: radius,
            bottom_right: radius,
        }
    }

    /// Creates corner radii with no rounding.
    pub fn none() -> Self {
        Self::all(0.0)
    }

    /// Converts to GPUI Corners<Pixels>.
    pub fn to_corners(self) -> Corners<Pixels> {
        Corners {
            top_left: px(self.top_left),
            top_right: px(self.top_right),
            bottom_left: px(self.bottom_left),
            bottom_right: px(self.bottom_right),
        }
    }
}

/// A stroke style for drawing paths and shapes.
#[derive(Debug, Clone, Copy)]
pub struct StrokeStyle {
    pub color: Hsla,
    pub width: f32,
}

impl StrokeStyle {
    /// Creates a new stroke style.
    pub fn new(color: Hsla, width: f32) -> Self {
        Self { color, width }
    }
}

impl Default for StrokeStyle {
    fn default() -> Self {
        Self {
            color: hsla(0.0, 0.0, 0.0, 1.0),
            width: 1.0,
        }
    }
}

/// A fill style for shapes.
#[derive(Debug, Clone, Copy)]
pub enum FillStyle {
    /// Solid color fill.
    Solid(Hsla),
    /// Linear gradient fill.
    LinearGradient {
        start: Point2D,
        end: Point2D,
        start_color: Hsla,
        end_color: Hsla,
    },
}

impl FillStyle {
    /// Creates a solid color fill.
    pub fn solid(color: Hsla) -> Self {
        FillStyle::Solid(color)
    }

    /// Creates a linear gradient fill.
    pub fn linear_gradient(
        start: Point2D,
        end: Point2D,
        start_color: Hsla,
        end_color: Hsla,
    ) -> Self {
        FillStyle::LinearGradient {
            start,
            end,
            start_color,
            end_color,
        }
    }

    /// Extracts the primary color (for solid fills or gradient start).
    fn primary_color(&self) -> Hsla {
        match self {
            FillStyle::Solid(color) => *color,
            FillStyle::LinearGradient { start_color, .. } => *start_color,
        }
    }
}

impl Default for FillStyle {
    fn default() -> Self {
        FillStyle::Solid(hsla(0.0, 0.0, 0.0, 1.0))
    }
}

impl From<Hsla> for FillStyle {
    fn from(color: Hsla) -> Self {
        FillStyle::Solid(color)
    }
}

/// Builder for constructing paths.
#[derive(Debug, Clone)]
pub struct PathBuilder {
    segments: Vec<PathSegment>,
    current_point: Point2D,
}

#[derive(Debug, Clone)]
enum PathSegment {
    MoveTo(Point2D),
    LineTo(Point2D),
    CurveTo {
        control: Point2D,
        end: Point2D,
    },
    QuadraticCurveTo {
        control: Point2D,
        end: Point2D,
    },
    Arc {
        center: Point2D,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
    },
    Close,
}

impl PathBuilder {
    /// Creates a new path builder starting at the given point.
    pub fn new(start: Point2D) -> Self {
        Self {
            segments: vec![PathSegment::MoveTo(start)],
            current_point: start,
        }
    }

    /// Creates a new path builder starting at the origin.
    pub fn new_at_origin() -> Self {
        Self::new(Point2D::zero())
    }

    /// Moves to a new point without drawing.
    pub fn move_to(mut self, point: Point2D) -> Self {
        self.segments.push(PathSegment::MoveTo(point));
        self.current_point = point;
        self
    }

    /// Draws a line from the current point to the given point.
    pub fn line_to(mut self, point: Point2D) -> Self {
        self.segments.push(PathSegment::LineTo(point));
        self.current_point = point;
        self
    }

    /// Draws a quadratic Bézier curve to the given point.
    pub fn curve_to(mut self, control: Point2D, end: Point2D) -> Self {
        self.segments.push(PathSegment::CurveTo { control, end });
        self.current_point = end;
        self
    }

    /// Draws a quadratic curve using a single control point.
    pub fn quad_to(mut self, control: Point2D, end: Point2D) -> Self {
        self.segments
            .push(PathSegment::QuadraticCurveTo { control, end });
        self.current_point = end;
        self
    }

    /// Adds an arc segment.
    pub fn arc(mut self, center: Point2D, radius: f32, start_angle: f32, end_angle: f32) -> Self {
        self.segments.push(PathSegment::Arc {
            center,
            radius,
            start_angle,
            end_angle,
        });
        // Update current point to end of arc
        self.current_point = Point2D::new(
            center.x + radius * end_angle.cos(),
            center.y + radius * end_angle.sin(),
        );
        self
    }

    /// Closes the path by drawing a line back to the start.
    pub fn close(mut self) -> Self {
        self.segments.push(PathSegment::Close);
        self
    }

    /// Builds the path into a GPUI Path.
    pub fn build(self) -> Path<Pixels> {
        let mut start_point = Point2D::zero();

        // Find first move_to for the path start
        for seg in &self.segments {
            if let PathSegment::MoveTo(p) = seg {
                start_point = *p;
                break;
            }
        }

        let mut path = Path::new(start_point.to_pixels());

        for segment in &self.segments {
            match segment {
                PathSegment::MoveTo(_) => {
                    // GPUI paths don't have move_to after creation
                }
                PathSegment::LineTo(p) => {
                    path.line_to(p.to_pixels());
                }
                PathSegment::CurveTo { control, end } => {
                    path.curve_to(control.to_pixels(), end.to_pixels());
                }
                PathSegment::QuadraticCurveTo { control, end } => {
                    // GPUI's curve_to takes a single control point (quadratic Bézier),
                    // so we can use the control point directly
                    path.curve_to(control.to_pixels(), end.to_pixels());
                }
                PathSegment::Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                } => {
                    // Approximate arc with line segments
                    let segments = 16;
                    let angle_step = (end_angle - start_angle) / segments as f32;
                    for i in 1..=segments {
                        let angle = start_angle + angle_step * i as f32;
                        let p =
                            Point2D::new(center.x + radius * angle.cos(), center.y + radius * angle.sin());
                        path.line_to(p.to_pixels());
                    }
                }
                PathSegment::Close => {
                    path.line_to(start_point.to_pixels());
                }
            }
        }

        path
    }

    /// Builds the path with an offset applied to all coordinates.
    pub fn build_with_offset(self, offset_x: f32, offset_y: f32) -> Path<Pixels> {
        let mut start_point = Point2D::zero();

        // Find first move_to for the path start
        for seg in &self.segments {
            if let PathSegment::MoveTo(p) = seg {
                start_point = Point2D::new(p.x + offset_x, p.y + offset_y);
                break;
            }
        }

        let mut path = Path::new(start_point.to_pixels());

        for segment in &self.segments {
            match segment {
                PathSegment::MoveTo(_) => {
                    // GPUI paths don't have move_to after creation
                }
                PathSegment::LineTo(p) => {
                    let offset_p = Point2D::new(p.x + offset_x, p.y + offset_y);
                    path.line_to(offset_p.to_pixels());
                }
                PathSegment::CurveTo { control, end } => {
                    let offset_control = Point2D::new(control.x + offset_x, control.y + offset_y);
                    let offset_end = Point2D::new(end.x + offset_x, end.y + offset_y);
                    path.curve_to(offset_control.to_pixels(), offset_end.to_pixels());
                }
                PathSegment::QuadraticCurveTo { control, end } => {
                    // GPUI's curve_to takes a single control point (quadratic Bézier)
                    let offset_control = Point2D::new(control.x + offset_x, control.y + offset_y);
                    let offset_end = Point2D::new(end.x + offset_x, end.y + offset_y);
                    path.curve_to(offset_control.to_pixels(), offset_end.to_pixels());
                }
                PathSegment::Arc {
                    center,
                    radius,
                    start_angle,
                    end_angle,
                } => {
                    let offset_center = Point2D::new(center.x + offset_x, center.y + offset_y);
                    let segments = 16;
                    let angle_step = (end_angle - start_angle) / segments as f32;
                    for i in 1..=segments {
                        let angle = start_angle + angle_step * i as f32;
                        let p = Point2D::new(
                            offset_center.x + radius * angle.cos(),
                            offset_center.y + radius * angle.sin(),
                        );
                        path.line_to(p.to_pixels());
                    }
                }
                PathSegment::Close => {
                    path.line_to(start_point.to_pixels());
                }
            }
        }

        path
    }

    /// Creates a rectangular path.
    pub fn rect(rect: Rect2D) -> Self {
        Self::new(rect.origin)
            .line_to(Point2D::new(
                rect.origin.x + rect.size.width,
                rect.origin.y,
            ))
            .line_to(Point2D::new(
                rect.origin.x + rect.size.width,
                rect.origin.y + rect.size.height,
            ))
            .line_to(Point2D::new(
                rect.origin.x,
                rect.origin.y + rect.size.height,
            ))
            .close()
    }

    /// Creates an ellipse path inscribed in the given rectangle.
    pub fn ellipse(rect: Rect2D) -> Self {
        let center = rect.center();
        let rx = rect.size.width / 2.0;
        let ry = rect.size.height / 2.0;

        // Approximate ellipse with Bézier curves
        // Using 4 cubic Bézier curves (one per quadrant)
        let k = 0.5522847498; // Magic number for cubic Bézier circle approximation

        Self::new(Point2D::new(center.x + rx, center.y))
            .curve_to(
                Point2D::new(center.x + rx, center.y + ry * k),
                Point2D::new(center.x + rx * k, center.y + ry),
            )
            .curve_to(
                Point2D::new(center.x, center.y + ry),
                Point2D::new(center.x - rx * k, center.y + ry),
            )
            .curve_to(
                Point2D::new(center.x - rx, center.y + ry * k),
                Point2D::new(center.x - rx, center.y),
            )
            .curve_to(
                Point2D::new(center.x - rx, center.y - ry * k),
                Point2D::new(center.x - rx * k, center.y - ry),
            )
            .curve_to(
                Point2D::new(center.x, center.y - ry),
                Point2D::new(center.x + rx * k, center.y - ry),
            )
            .curve_to(
                Point2D::new(center.x + rx, center.y - ry * k),
                Point2D::new(center.x + rx, center.y),
            )
            .close()
    }

    /// Creates a circular path with the given center and radius.
    pub fn circle(center: Point2D, radius: f32) -> Self {
        Self::ellipse(Rect2D::new(
            center.x - radius,
            center.y - radius,
            radius * 2.0,
            radius * 2.0,
        ))
    }
}

/// A drawing command to be executed on the canvas.
#[derive(Clone)]
pub enum DrawCommand {
    /// Fill a rectangle with a color.
    FillRect {
        rect: Rect2D,
        fill: FillStyle,
        corner_radii: CornerRadii,
    },
    /// Stroke a rectangle outline.
    StrokeRect {
        rect: Rect2D,
        stroke: StrokeStyle,
        corner_radii: CornerRadii,
    },
    /// Fill an ellipse inscribed in the given rectangle.
    FillEllipse { rect: Rect2D, fill: FillStyle },
    /// Stroke an ellipse outline.
    StrokeEllipse { rect: Rect2D, stroke: StrokeStyle },
    /// Fill a circle.
    FillCircle {
        center: Point2D,
        radius: f32,
        fill: FillStyle,
    },
    /// Stroke a circle outline.
    StrokeCircle {
        center: Point2D,
        radius: f32,
        stroke: StrokeStyle,
    },
    /// Draw a line between two points.
    Line {
        from: Point2D,
        to: Point2D,
        stroke: StrokeStyle,
    },
    /// Fill a path.
    FillPath { path: PathBuilder, fill: FillStyle },
    /// Stroke a path.
    StrokePath {
        path: PathBuilder,
        stroke: StrokeStyle,
    },
    /// Draw text at a position.
    Text {
        text: SharedString,
        position: Point2D,
        color: Hsla,
        size: f32,
    },
}

/// Drawing context that collects drawing commands.
///
/// The DrawingContext provides a high-level API for drawing operations.
/// Commands are collected and then rendered when the canvas paints.
pub struct DrawingContext {
    /// The size of the canvas in pixels.
    pub size: Size2D,
    /// The bounds of the canvas.
    pub bounds: Rect2D,
    /// Collected drawing commands.
    commands: Vec<DrawCommand>,
}

impl DrawingContext {
    /// Creates a new drawing context for the given bounds.
    fn new(bounds: Bounds<Pixels>) -> Self {
        Self {
            size: Size2D::new(f32::from(bounds.size.width), f32::from(bounds.size.height)),
            bounds: Rect2D::from_bounds(bounds),
            commands: Vec::new(),
        }
    }

    /// Fills a rectangle with a color.
    pub fn fill_rect(&mut self, rect: Rect2D, fill: impl Into<FillStyle>) {
        self.commands.push(DrawCommand::FillRect {
            rect,
            fill: fill.into(),
            corner_radii: CornerRadii::none(),
        });
    }

    /// Fills a rounded rectangle.
    pub fn fill_rounded_rect(
        &mut self,
        rect: Rect2D,
        fill: impl Into<FillStyle>,
        corner_radii: CornerRadii,
    ) {
        self.commands.push(DrawCommand::FillRect {
            rect,
            fill: fill.into(),
            corner_radii,
        });
    }

    /// Strokes a rectangle outline.
    pub fn stroke_rect(&mut self, rect: Rect2D, stroke: StrokeStyle) {
        self.commands.push(DrawCommand::StrokeRect {
            rect,
            stroke,
            corner_radii: CornerRadii::none(),
        });
    }

    /// Strokes a rounded rectangle outline.
    pub fn stroke_rounded_rect(
        &mut self,
        rect: Rect2D,
        stroke: StrokeStyle,
        corner_radii: CornerRadii,
    ) {
        self.commands.push(DrawCommand::StrokeRect {
            rect,
            stroke,
            corner_radii,
        });
    }

    /// Fills an ellipse inscribed in the given rectangle.
    pub fn fill_ellipse(&mut self, rect: Rect2D, fill: impl Into<FillStyle>) {
        self.commands.push(DrawCommand::FillEllipse {
            rect,
            fill: fill.into(),
        });
    }

    /// Strokes an ellipse outline.
    pub fn stroke_ellipse(&mut self, rect: Rect2D, stroke: StrokeStyle) {
        self.commands.push(DrawCommand::StrokeEllipse { rect, stroke });
    }

    /// Fills a circle.
    pub fn fill_circle(&mut self, center: Point2D, radius: f32, fill: impl Into<FillStyle>) {
        self.commands.push(DrawCommand::FillCircle {
            center,
            radius,
            fill: fill.into(),
        });
    }

    /// Strokes a circle outline.
    pub fn stroke_circle(&mut self, center: Point2D, radius: f32, stroke: StrokeStyle) {
        self.commands.push(DrawCommand::StrokeCircle {
            center,
            radius,
            stroke,
        });
    }

    /// Draws a line between two points.
    pub fn line(&mut self, from: Point2D, to: Point2D, stroke: StrokeStyle) {
        self.commands.push(DrawCommand::Line { from, to, stroke });
    }

    /// Fills a path.
    pub fn fill_path(&mut self, path: PathBuilder, fill: impl Into<FillStyle>) {
        self.commands.push(DrawCommand::FillPath {
            path,
            fill: fill.into(),
        });
    }

    /// Strokes a path.
    pub fn stroke_path(&mut self, path: PathBuilder, stroke: StrokeStyle) {
        self.commands.push(DrawCommand::StrokePath { path, stroke });
    }

    /// Draws text at the given position.
    pub fn text(&mut self, text: impl Into<SharedString>, position: Point2D, color: Hsla) {
        self.commands.push(DrawCommand::Text {
            text: text.into(),
            position,
            color,
            size: 14.0,
        });
    }

    /// Draws text with a custom size.
    pub fn text_sized(
        &mut self,
        text: impl Into<SharedString>,
        position: Point2D,
        color: Hsla,
        size: f32,
    ) {
        self.commands.push(DrawCommand::Text {
            text: text.into(),
            position,
            color,
            size,
        });
    }

    /// Returns the drawing commands.
    fn into_commands(self) -> Vec<DrawCommand> {
        self.commands
    }
}

/// A canvas view for custom drawing with immediate mode graphics.
///
/// Canvas provides a drawing context that allows for custom rendering of shapes,
/// paths, and text. It's similar to SwiftUI's Canvas view.
///
/// # Example
///
/// ```ignore
/// Canvas::new("my-canvas", |ctx| {
///     // Fill a blue rectangle
///     ctx.fill_rect(
///         Rect2D::new(10.0, 10.0, 100.0, 50.0),
///         hsla(211.0 / 360.0, 0.95, 0.53, 1.0),
///     );
///
///     // Draw a red circle
///     ctx.fill_circle(
///         Point2D::new(80.0, 80.0),
///         30.0,
///         hsla(0.0, 0.85, 0.55, 1.0),
///     );
///
///     // Draw a line
///     ctx.line(
///         Point2D::new(0.0, 0.0),
///         Point2D::new(100.0, 100.0),
///         StrokeStyle::new(hsla(0.0, 0.0, 0.0, 1.0), 2.0),
///     );
/// })
/// .size(px(200.0), px(200.0))
/// ```
pub struct Canvas {
    id: ElementId,
    width: Pixels,
    height: Pixels,
    background: Option<Hsla>,
    on_draw: Option<Box<dyn Fn(&mut DrawingContext) + 'static>>,
}

impl Canvas {
    /// Creates a new canvas with a drawing callback.
    ///
    /// The callback receives a `DrawingContext` which provides methods for
    /// drawing shapes, paths, and text.
    pub fn new(
        id: impl Into<ElementId>,
        on_draw: impl Fn(&mut DrawingContext) + 'static,
    ) -> Self {
        Self {
            id: id.into(),
            width: px(100.0),
            height: px(100.0),
            background: None,
            on_draw: Some(Box::new(on_draw)),
        }
    }

    /// Creates a new canvas without a drawing callback.
    pub fn empty(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            width: px(100.0),
            height: px(100.0),
            background: None,
            on_draw: None,
        }
    }

    /// Sets the size of the canvas.
    pub fn size(mut self, width: Pixels, height: Pixels) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Sets the width of the canvas.
    pub fn width(mut self, width: Pixels) -> Self {
        self.width = width;
        self
    }

    /// Sets the height of the canvas.
    pub fn height(mut self, height: Pixels) -> Self {
        self.height = height;
        self
    }

    /// Sets the background color of the canvas.
    pub fn background(mut self, color: Hsla) -> Self {
        self.background = Some(color);
        self
    }

    /// Execute the draw commands and render them.
    fn render_commands(bounds: Bounds<Pixels>, commands: Vec<DrawCommand>, window: &mut Window) {
        let offset_x = f32::from(bounds.origin.x);
        let offset_y = f32::from(bounds.origin.y);

        for command in commands {
            match command {
                DrawCommand::FillRect {
                    rect,
                    fill,
                    corner_radii,
                } => {
                    // Offset rect by canvas bounds origin
                    let absolute_bounds = Bounds {
                        origin: point(
                            bounds.origin.x + px(rect.origin.x),
                            bounds.origin.y + px(rect.origin.y),
                        ),
                        size: rect.size.to_pixels(),
                    };
                    window.paint_quad(PaintQuad {
                        bounds: absolute_bounds,
                        corner_radii: corner_radii.to_corners(),
                        background: fill.primary_color().into(),
                        border_widths: Edges::default(),
                        border_color: gpui::transparent_black(),
                        border_style: BorderStyle::default(),
                    });
                }
                DrawCommand::StrokeRect {
                    rect,
                    stroke,
                    corner_radii,
                } => {
                    let absolute_bounds = Bounds {
                        origin: point(
                            bounds.origin.x + px(rect.origin.x),
                            bounds.origin.y + px(rect.origin.y),
                        ),
                        size: rect.size.to_pixels(),
                    };
                    window.paint_quad(PaintQuad {
                        bounds: absolute_bounds,
                        corner_radii: corner_radii.to_corners(),
                        background: gpui::transparent_black().into(),
                        border_widths: Edges::all(px(stroke.width)),
                        border_color: stroke.color,
                        border_style: BorderStyle::default(),
                    });
                }
                DrawCommand::FillEllipse { rect, fill } => {
                    // Build path with absolute coordinates
                    let abs_rect = Rect2D::new(
                        rect.origin.x + offset_x,
                        rect.origin.y + offset_y,
                        rect.size.width,
                        rect.size.height,
                    );
                    let path = PathBuilder::ellipse(abs_rect);
                    let gpui_path = path.build();
                    window.paint_path(gpui_path, fill.primary_color());
                }
                DrawCommand::StrokeEllipse { rect, stroke } => {
                    // Build path with absolute coordinates
                    let abs_rect = Rect2D::new(
                        rect.origin.x + offset_x,
                        rect.origin.y + offset_y,
                        rect.size.width,
                        rect.size.height,
                    );
                    let path = PathBuilder::ellipse(abs_rect);
                    let gpui_path = path.build();
                    window.paint_path(gpui_path, stroke.color);
                }
                DrawCommand::FillCircle { center, radius, fill } => {
                    let abs_center = Point2D::new(center.x + offset_x, center.y + offset_y);
                    let path = PathBuilder::circle(abs_center, radius);
                    let gpui_path = path.build();
                    window.paint_path(gpui_path, fill.primary_color());
                }
                DrawCommand::StrokeCircle {
                    center,
                    radius,
                    stroke,
                } => {
                    let abs_center = Point2D::new(center.x + offset_x, center.y + offset_y);
                    let path = PathBuilder::circle(abs_center, radius);
                    let gpui_path = path.build();
                    window.paint_path(gpui_path, stroke.color);
                }
                DrawCommand::Line { from, to, stroke } => {
                    // Draw line as a thin filled path (triangle strip)
                    // Calculate perpendicular offset for line width
                    let dx = to.x - from.x;
                    let dy = to.y - from.y;
                    let len = (dx * dx + dy * dy).sqrt();
                    if len > 0.0 {
                        let half_width = stroke.width / 2.0;
                        let nx = -dy / len * half_width;
                        let ny = dx / len * half_width;

                        // Use absolute coordinates
                        let abs_from = Point2D::new(from.x + offset_x, from.y + offset_y);
                        let abs_to = Point2D::new(to.x + offset_x, to.y + offset_y);

                        let path =
                            PathBuilder::new(Point2D::new(abs_from.x + nx, abs_from.y + ny))
                                .line_to(Point2D::new(abs_from.x - nx, abs_from.y - ny))
                                .line_to(Point2D::new(abs_to.x - nx, abs_to.y - ny))
                                .line_to(Point2D::new(abs_to.x + nx, abs_to.y + ny))
                                .close();
                        let gpui_path = path.build();
                        window.paint_path(gpui_path, stroke.color);
                    }
                }
                DrawCommand::FillPath { path, fill } => {
                    // Build path with offset applied
                    let gpui_path = path.build_with_offset(offset_x, offset_y);
                    window.paint_path(gpui_path, fill.primary_color());
                }
                DrawCommand::StrokePath { path, stroke } => {
                    // Build path with offset applied
                    let gpui_path = path.build_with_offset(offset_x, offset_y);
                    window.paint_path(gpui_path, stroke.color);
                }
                DrawCommand::Text {
                    text,
                    position,
                    color,
                    size,
                } => {
                    // Text rendering requires more complex handling with GPUI's text system
                    // For now, we skip text rendering as it requires font loading
                    let _ = (text, position, color, size);
                }
            }
        }
    }
}

impl IntoElement for Canvas {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let background = self.background;

        // Collect draw commands by calling the callback
        let commands = if let Some(on_draw) = self.on_draw {
            // Create a temporary context with zero-sized bounds for command collection
            // The actual bounds will be used during rendering
            let mut ctx = DrawingContext::new(Bounds {
                origin: point(px(0.0), px(0.0)),
                size: size(self.width, self.height),
            });
            on_draw(&mut ctx);
            ctx.into_commands()
        } else {
            Vec::new()
        };

        let base = div()
            .id(self.id)
            .w(self.width)
            .h(self.height)
            .relative()
            .overflow_hidden();

        // Apply background if set
        let base = if let Some(bg) = background {
            base.bg(bg)
        } else {
            base
        };

        // Add canvas overlay for custom painting
        let canvas_overlay = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, window, _cx| {
                Canvas::render_commands(bounds, commands.clone(), window);
            },
        )
        .absolute()
        .size_full();

        base.child(canvas_overlay)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d() {
        let p = Point2D::new(10.0, 20.0);
        assert_eq!(p.x, 10.0);
        assert_eq!(p.y, 20.0);
    }

    #[test]
    fn test_size2d() {
        let s = Size2D::new(100.0, 50.0);
        assert_eq!(s.width, 100.0);
        assert_eq!(s.height, 50.0);
    }

    #[test]
    fn test_rect2d() {
        let r = Rect2D::new(10.0, 20.0, 100.0, 50.0);
        assert_eq!(r.origin.x, 10.0);
        assert_eq!(r.origin.y, 20.0);
        assert_eq!(r.size.width, 100.0);
        assert_eq!(r.size.height, 50.0);

        let center = r.center();
        assert_eq!(center.x, 60.0);
        assert_eq!(center.y, 45.0);
    }

    #[test]
    fn test_corner_radii() {
        let radii = CornerRadii::all(5.0);
        assert_eq!(radii.top_left, 5.0);
        assert_eq!(radii.top_right, 5.0);
        assert_eq!(radii.bottom_left, 5.0);
        assert_eq!(radii.bottom_right, 5.0);
    }

    #[test]
    fn test_stroke_style() {
        let stroke = StrokeStyle::new(hsla(0.0, 1.0, 0.5, 1.0), 2.0);
        assert_eq!(stroke.width, 2.0);
    }

    #[test]
    fn test_fill_style() {
        let solid = FillStyle::solid(hsla(0.0, 1.0, 0.5, 1.0));
        assert!(matches!(solid, FillStyle::Solid(_)));

        let gradient = FillStyle::linear_gradient(
            Point2D::zero(),
            Point2D::new(100.0, 0.0),
            hsla(0.0, 1.0, 0.5, 1.0),
            hsla(0.5, 1.0, 0.5, 1.0),
        );
        assert!(matches!(gradient, FillStyle::LinearGradient { .. }));
    }

    #[test]
    fn test_path_builder() {
        let path = PathBuilder::new(Point2D::zero())
            .line_to(Point2D::new(100.0, 0.0))
            .line_to(Point2D::new(100.0, 100.0))
            .close();

        // Just verify it builds without panicking
        let _gpui_path = path.build();
    }

    #[test]
    fn test_path_rect() {
        let rect = Rect2D::new(10.0, 10.0, 100.0, 50.0);
        let path = PathBuilder::rect(rect);
        let _gpui_path = path.build();
    }

    #[test]
    fn test_path_ellipse() {
        let rect = Rect2D::new(10.0, 10.0, 100.0, 50.0);
        let path = PathBuilder::ellipse(rect);
        let _gpui_path = path.build();
    }

    #[test]
    fn test_path_circle() {
        let path = PathBuilder::circle(Point2D::new(50.0, 50.0), 30.0);
        let _gpui_path = path.build();
    }

    #[test]
    fn test_drawing_context() {
        let bounds = Bounds {
            origin: point(px(0.0), px(0.0)),
            size: size(px(200.0), px(200.0)),
        };
        let mut ctx = DrawingContext::new(bounds);

        ctx.fill_rect(Rect2D::new(10.0, 10.0, 50.0, 50.0), hsla(0.0, 1.0, 0.5, 1.0));
        ctx.stroke_rect(
            Rect2D::new(70.0, 10.0, 50.0, 50.0),
            StrokeStyle::new(hsla(0.5, 1.0, 0.5, 1.0), 2.0),
        );
        ctx.fill_circle(Point2D::new(100.0, 100.0), 30.0, hsla(0.3, 1.0, 0.5, 1.0));
        ctx.line(
            Point2D::zero(),
            Point2D::new(200.0, 200.0),
            StrokeStyle::default(),
        );

        let commands = ctx.into_commands();
        assert_eq!(commands.len(), 4);
    }

    #[test]
    fn test_canvas_creation() {
        let canvas = Canvas::new("test-canvas", |ctx| {
            ctx.fill_rect(Rect2D::new(0.0, 0.0, 50.0, 50.0), hsla(0.0, 1.0, 0.5, 1.0));
        });

        assert_eq!(canvas.width, px(100.0));
        assert_eq!(canvas.height, px(100.0));
    }

    #[test]
    fn test_canvas_size() {
        let canvas = Canvas::empty("test")
            .size(px(200.0), px(150.0))
            .background(hsla(0.0, 0.0, 1.0, 1.0));

        assert_eq!(canvas.width, px(200.0));
        assert_eq!(canvas.height, px(150.0));
        assert!(canvas.background.is_some());
    }
}
