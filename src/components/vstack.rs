//! Vertical stack layout component.
//!
//! A semantic layout component that arranges children vertically using flexbox.
//! This is equivalent to SwiftUI's VStack.

use gpui::prelude::*;
use gpui::*;

/// A vertical stack layout component.
///
/// VStack arranges its children vertically (top to bottom) using flexbox layout.
///
/// # Example
///
/// ```ignore
/// VStack::new()
///     .gap_3()
///     .child(div().child("First"))
///     .child(div().child("Second"))
///     .child(div().child("Third"))
/// ```
pub struct VStack {
    gap: Pixels,
    padding: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl VStack {
    /// Creates a new vertical stack.
    pub fn new() -> Self {
        Self {
            gap: px(0.0),
            padding: None,
            children: Vec::new(),
        }
    }

    /// Sets the gap between children to 0.75rem (12px).
    pub fn gap_3(mut self) -> Self {
        self.gap = px(12.0);
        self
    }

    /// Sets the gap between children to 1.5rem (24px).
    pub fn gap_6(mut self) -> Self {
        self.gap = px(24.0);
        self
    }

    /// Sets custom gap between children.
    pub fn gap(mut self, gap: impl Into<Pixels>) -> Self {
        self.gap = gap.into();
        self
    }

    /// Sets padding around all children to 0.25rem (4px).
    pub fn p_1(mut self) -> Self {
        self.padding = Some(px(4.0));
        self
    }

    /// Sets padding around all children to 0.5rem (8px).
    pub fn p_2(mut self) -> Self {
        self.padding = Some(px(8.0));
        self
    }

    /// Sets padding around all children to 0.75rem (12px).
    pub fn p_3(mut self) -> Self {
        self.padding = Some(px(12.0));
        self
    }

    /// Sets padding around all children to 1rem (16px).
    pub fn p_4(mut self) -> Self {
        self.padding = Some(px(16.0));
        self
    }

    /// Sets custom padding around all children.
    pub fn p(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Adds a child element to the stack.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the stack.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for VStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut container = div().flex().flex_col().gap(self.gap);

        // Apply padding if set
        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        // Add all children
        for child in self.children {
            container = container.child(child);
        }

        container
    }
}
