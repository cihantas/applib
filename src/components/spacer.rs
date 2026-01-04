//! Flexible space layout component.
//!
//! A component that expands to fill available space in stack layouts.
//! This is equivalent to SwiftUI's Spacer.

use gpui::prelude::*;
use gpui::*;

/// A flexible space that expands to fill available space in stack layouts.
///
/// Spacer uses `flex_grow(1.0)` to expand within flex containers,
/// pushing sibling elements apart.
///
/// # Example
///
/// ```ignore
/// HStack::new()
///     .child(div().child("Left"))
///     .child(Spacer::new())
///     .child(div().child("Right"))
/// ```
///
/// With minimum length:
///
/// ```ignore
/// HStack::new()
///     .child(div().child("Left"))
///     .child(Spacer::new().min_length(px(20.0)))
///     .child(div().child("Right"))
/// ```
pub struct Spacer {
    min_length: Option<Pixels>,
}

impl Spacer {
    /// Creates a new spacer that expands to fill available space.
    pub fn new() -> Self {
        Self { min_length: None }
    }

    /// Sets the minimum length of the spacer.
    ///
    /// This ensures the spacer takes at least this much space,
    /// even if there's not enough room for it to fully expand.
    pub fn min_length(mut self, length: impl Into<Pixels>) -> Self {
        self.min_length = Some(length.into());
        self
    }
}

impl Default for Spacer {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Spacer {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut spacer = div().flex_grow();

        if let Some(min_length) = self.min_length {
            spacer = spacer.min_w(min_length).min_h(min_length);
        }

        spacer
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacer_creation() {
        let spacer = Spacer::new();
        assert!(spacer.min_length.is_none());
    }

    #[test]
    fn test_spacer_with_min_length() {
        let spacer = Spacer::new().min_length(px(20.0));
        assert_eq!(spacer.min_length, Some(px(20.0)));
    }
}
