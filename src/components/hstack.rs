//! Horizontal stack layout component.
//!
//! A semantic layout component that arranges children horizontally using flexbox.
//! This is equivalent to SwiftUI's HStack.

use gpui::prelude::*;
use gpui::*;

/// A horizontal stack layout component.
///
/// HStack arranges its children horizontally (left to right) using flexbox layout.
///
/// # Example
///
/// ```ignore
/// HStack::new()
///     .gap_3()
///     .child(Button::new("btn1", "First"))
///     .child(Button::new("btn2", "Second"))
///     .child(Button::new("btn3", "Third"))
/// ```
pub struct HStack {
    gap: Pixels,
    padding: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl HStack {
    /// Creates a new horizontal stack.
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

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for HStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let mut container = div()
            .flex()
            .flex_row()
            .size_full()
            .overflow_hidden()
            .gap(self.gap);

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
