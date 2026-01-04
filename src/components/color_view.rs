//! A colored rectangle view component.
//!
//! This module provides a simple view that renders as a colored rectangle,
//! similar to SwiftUI's Color view.

use gpui::prelude::*;
use gpui::*;

/// A view that displays a solid color rectangle.
///
/// ColorView renders as a colored rectangle that expands to fill available space.
/// It's useful for backgrounds in ZStack layouts or as visual placeholders.
///
/// # Example
///
/// ```ignore
/// // Custom color
/// ColorView::new(hsla(0.6, 0.8, 0.5, 1.0))
///
/// // Predefined colors
/// ColorView::blue()
/// ColorView::red().opacity(0.5)
/// ```
pub struct ColorView {
    color: Hsla,
    opacity: f32,
}

impl ColorView {
    /// Creates a new color view with the given color.
    pub fn new(color: Hsla) -> Self {
        Self {
            color,
            opacity: 1.0,
        }
    }

    /// Sets the opacity of the color view.
    pub fn opacity(mut self, opacity: f32) -> Self {
        self.opacity = opacity;
        self
    }

    // Predefined color constructors

    /// Creates a blue color view.
    pub fn blue() -> Self {
        Self::new(hsla(211.0 / 360.0, 0.95, 0.53, 1.0))
    }

    /// Creates a red color view.
    pub fn red() -> Self {
        Self::new(hsla(0.0, 0.85, 0.55, 1.0))
    }

    /// Creates a green color view.
    pub fn green() -> Self {
        Self::new(hsla(120.0 / 360.0, 0.70, 0.45, 1.0))
    }

    /// Creates a yellow color view.
    pub fn yellow() -> Self {
        Self::new(hsla(45.0 / 360.0, 0.95, 0.55, 1.0))
    }

    /// Creates an orange color view.
    pub fn orange() -> Self {
        Self::new(hsla(30.0 / 360.0, 0.95, 0.55, 1.0))
    }

    /// Creates a purple color view.
    pub fn purple() -> Self {
        Self::new(hsla(270.0 / 360.0, 0.70, 0.55, 1.0))
    }

    /// Creates a pink color view.
    pub fn pink() -> Self {
        Self::new(hsla(330.0 / 360.0, 0.80, 0.65, 1.0))
    }

    /// Creates a gray color view.
    pub fn gray() -> Self {
        Self::new(hsla(0.0, 0.0, 0.55, 1.0))
    }

    /// Creates a white color view.
    pub fn white() -> Self {
        Self::new(hsla(0.0, 0.0, 1.0, 1.0))
    }

    /// Creates a black color view.
    pub fn black() -> Self {
        Self::new(hsla(0.0, 0.0, 0.0, 1.0))
    }

    /// Creates a clear (transparent) color view.
    pub fn clear() -> Self {
        Self::new(hsla(0.0, 0.0, 0.0, 0.0))
    }
}

impl IntoElement for ColorView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut color = self.color;
        color.a *= self.opacity;

        div().flex_grow().size_full().bg(color)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_view_creation() {
        let view = ColorView::new(hsla(0.5, 0.5, 0.5, 1.0));
        assert_eq!(view.opacity, 1.0);
    }

    #[test]
    fn test_color_view_opacity() {
        let view = ColorView::blue().opacity(0.5);
        assert_eq!(view.opacity, 0.5);
    }

    #[test]
    fn test_predefined_colors() {
        // Just ensure they create without panicking
        let _ = ColorView::blue();
        let _ = ColorView::red();
        let _ = ColorView::green();
        let _ = ColorView::yellow();
        let _ = ColorView::orange();
        let _ = ColorView::purple();
        let _ = ColorView::pink();
        let _ = ColorView::gray();
        let _ = ColorView::white();
        let _ = ColorView::black();
        let _ = ColorView::clear();
    }
}
