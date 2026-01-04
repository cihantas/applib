//! Visual separator line component.
//!
//! A simple divider for separating content, equivalent to SwiftUI's Divider.

use gpui::prelude::*;
use gpui::*;

/// Orientation of the divider.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    /// Horizontal line (for use in VStack).
    #[default]
    Horizontal,
    /// Vertical line (for use in HStack).
    Vertical,
}

/// A visual separator line component.
///
/// Divider creates a thin line to separate content visually.
/// By default, it creates a horizontal line suitable for use in a VStack.
///
/// # Example
///
/// ```ignore
/// VStack::new()
///     .child(div().child("Above"))
///     .child(Divider::horizontal())
///     .child(div().child("Below"))
/// ```
pub struct Divider {
    orientation: DividerOrientation,
    color: Hsla,
    thickness: Pixels,
}

impl Divider {
    /// Creates a new horizontal divider.
    pub fn horizontal() -> Self {
        Self {
            orientation: DividerOrientation::Horizontal,
            color: hsla(0.0, 0.0, 0.90, 1.0), // Default separator color
            thickness: px(1.0),
        }
    }

    /// Creates a new vertical divider.
    pub fn vertical() -> Self {
        Self {
            orientation: DividerOrientation::Vertical,
            color: hsla(0.0, 0.0, 0.90, 1.0), // Default separator color
            thickness: px(1.0),
        }
    }

    /// Sets a custom color for the divider.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = color;
        self
    }

    /// Sets a custom thickness for the divider.
    pub fn thickness(mut self, thickness: impl Into<Pixels>) -> Self {
        self.thickness = thickness.into();
        self
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::horizontal()
    }
}

impl IntoElement for Divider {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        match self.orientation {
            DividerOrientation::Horizontal => div()
                .w_full()
                .h(self.thickness)
                .bg(self.color),
            DividerOrientation::Vertical => div()
                .h_full()
                .w(self.thickness)
                .bg(self.color),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horizontal_divider() {
        let divider = Divider::horizontal();
        assert_eq!(divider.orientation, DividerOrientation::Horizontal);
    }

    #[test]
    fn test_vertical_divider() {
        let divider = Divider::vertical();
        assert_eq!(divider.orientation, DividerOrientation::Vertical);
    }

    #[test]
    fn test_custom_color() {
        let custom_color = hsla(0.5, 0.5, 0.5, 1.0);
        let divider = Divider::horizontal().color(custom_color);
        assert_eq!(divider.color, custom_color);
    }

    #[test]
    fn test_custom_thickness() {
        let divider = Divider::horizontal().thickness(px(2.0));
        assert_eq!(divider.thickness, px(2.0));
    }
}
