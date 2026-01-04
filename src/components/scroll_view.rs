//! Scroll view component for scrollable content.
//!
//! Provides a scrollable container for content that exceeds its bounds.

use gpui::prelude::*;
use gpui::*;

/// A scroll view component that provides scrollable content.
///
/// # Example
///
/// ```ignore
/// ScrollView::vertical("my-scroll")
///     .child(content)
/// ```
pub struct ScrollView {
    id: ElementId,
    axis: ScrollAxis,
    children: Vec<AnyElement>,
}

/// The axis along which content can scroll.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScrollAxis {
    /// Scroll vertically only
    #[default]
    Vertical,
    /// Scroll horizontally only
    Horizontal,
    /// Scroll in both directions
    Both,
}

impl ScrollView {
    /// Creates a new vertical scroll view with the given id.
    pub fn vertical(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            axis: ScrollAxis::Vertical,
            children: Vec::new(),
        }
    }

    /// Creates a new horizontal scroll view with the given id.
    pub fn horizontal(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            axis: ScrollAxis::Horizontal,
            children: Vec::new(),
        }
    }

    /// Creates a new scroll view that scrolls in both directions.
    pub fn both(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            axis: ScrollAxis::Both,
            children: Vec::new(),
        }
    }

    /// Sets the scroll axis.
    pub fn axis(mut self, axis: ScrollAxis) -> Self {
        self.axis = axis;
        self
    }

    /// Adds a child element to the scroll view.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the scroll view.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl IntoElement for ScrollView {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let mut container = div().id(self.id).flex().size_full();

        // Apply scroll behavior based on axis
        container = match self.axis {
            ScrollAxis::Vertical => container.flex_col().overflow_y_scroll(),
            ScrollAxis::Horizontal => container.flex_row().overflow_x_scroll(),
            ScrollAxis::Both => container.flex_col().overflow_scroll(),
        };

        // Add all children
        for child in self.children {
            container = container.child(child);
        }

        container
    }
}
