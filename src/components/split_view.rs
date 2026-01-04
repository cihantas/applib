//! Split view component for divided layouts.
//!
//! Provides a two-pane layout that can be oriented horizontally or vertically.

use gpui::prelude::*;
use gpui::*;

/// Orientation of the split view
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitOrientation {
    /// Panes arranged side by side (left/right)
    #[default]
    Horizontal,
    /// Panes arranged top to bottom
    Vertical,
}

/// A split view component that divides its space between two panes.
///
/// # Example
///
/// ```ignore
/// SplitView::horizontal()
///     .first(left_content)
///     .second(right_content)
///     .first_size(px(300.0))
/// ```
pub struct SplitView {
    orientation: SplitOrientation,
    first: Option<AnyElement>,
    second: Option<AnyElement>,
    first_size: Option<Pixels>,
    divider_color: Hsla,
}

impl SplitView {
    /// Creates a new horizontal split view (side by side panes).
    pub fn horizontal() -> Self {
        Self {
            orientation: SplitOrientation::Horizontal,
            first: None,
            second: None,
            first_size: None,
            divider_color: hsla(0.0, 0.0, 0.85, 1.0),
        }
    }

    /// Creates a new vertical split view (stacked panes).
    pub fn vertical() -> Self {
        Self {
            orientation: SplitOrientation::Vertical,
            first: None,
            second: None,
            first_size: None,
            divider_color: hsla(0.0, 0.0, 0.85, 1.0),
        }
    }

    /// Sets the first (left or top) pane content.
    pub fn first(mut self, content: impl IntoElement) -> Self {
        self.first = Some(content.into_any_element());
        self
    }

    /// Sets the second (right or bottom) pane content.
    pub fn second(mut self, content: impl IntoElement) -> Self {
        self.second = Some(content.into_any_element());
        self
    }

    /// Sets a fixed size for the first pane.
    /// For horizontal splits, this is the width.
    /// For vertical splits, this is the height.
    pub fn first_size(mut self, size: Pixels) -> Self {
        self.first_size = Some(size);
        self
    }

    /// Sets the divider color.
    pub fn divider_color(mut self, color: Hsla) -> Self {
        self.divider_color = color;
        self
    }
}

impl IntoElement for SplitView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let is_horizontal = self.orientation == SplitOrientation::Horizontal;

        let mut container = div().flex().size_full();

        if is_horizontal {
            container = container.flex_row();
        } else {
            container = container.flex_col();
        }

        // First pane
        if let Some(first_content) = self.first {
            let mut first_pane = div().flex().flex_col().overflow_hidden();

            if let Some(size) = self.first_size {
                if is_horizontal {
                    first_pane = first_pane.w(size).flex_shrink_0();
                } else {
                    first_pane = first_pane.h(size).flex_shrink_0();
                }
            } else {
                first_pane = first_pane.flex_1();
            }

            container = container.child(first_pane.child(first_content));
        }

        // Divider
        let divider = if is_horizontal {
            div()
                .w(px(1.0))
                .h_full()
                .bg(self.divider_color)
                .flex_shrink_0()
        } else {
            div()
                .h(px(1.0))
                .w_full()
                .bg(self.divider_color)
                .flex_shrink_0()
        };

        container = container.child(divider);

        // Second pane
        if let Some(second_content) = self.second {
            let second_pane = div().flex().flex_col().flex_1().overflow_hidden();
            container = container.child(second_pane.child(second_content));
        }

        container
    }
}
