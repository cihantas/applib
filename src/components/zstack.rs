//! Z-axis stack layout component.
//!
//! A semantic layout component that overlays children on top of each other.
//! This is equivalent to SwiftUI's ZStack.

use gpui::prelude::*;
use gpui::*;

/// Alignment options for ZStack children.
///
/// Determines where children are positioned within the stack.
#[derive(Debug, Clone, Copy, Default)]
pub enum ZStackAlignment {
    /// Align to top-left corner.
    TopLeading,
    /// Align to top center.
    Top,
    /// Align to top-right corner.
    TopTrailing,
    /// Align to center-left.
    Leading,
    /// Align to center (default).
    #[default]
    Center,
    /// Align to center-right.
    Trailing,
    /// Align to bottom-left corner.
    BottomLeading,
    /// Align to bottom center.
    Bottom,
    /// Align to bottom-right corner.
    BottomTrailing,
}

/// A z-axis stack layout component.
///
/// ZStack overlays its children on top of each other along the z-axis.
/// The first child is at the bottom, and subsequent children are layered on top.
///
/// # Example
///
/// ```ignore
/// ZStack::new()
///     .alignment(ZStackAlignment::Center)
///     .child(div().size_full().bg(blue))
///     .child(Text::new("Hello"))
/// ```
pub struct ZStack {
    alignment: ZStackAlignment,
    padding: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl ZStack {
    /// Creates a new z-axis stack with center alignment.
    pub fn new() -> Self {
        Self {
            alignment: ZStackAlignment::Center,
            padding: None,
            children: Vec::new(),
        }
    }

    /// Sets the alignment for all children.
    pub fn alignment(mut self, alignment: ZStackAlignment) -> Self {
        self.alignment = alignment;
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
    ///
    /// Children are layered in order: first child at bottom, last child on top.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the stack.
    ///
    /// Children are layered in order: first child at bottom, last child on top.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for ZStack {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for ZStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        // Container uses relative positioning so children can be absolute
        let mut container = div().relative().size_full();

        // Apply padding if set
        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        // Add each child as an absolutely positioned layer
        for child in self.children {
            let mut layer = div().absolute().size_full();

            // Apply alignment using flexbox on the layer
            layer = layer.flex();

            // Apply horizontal alignment (justify-content)
            layer = match self.alignment {
                ZStackAlignment::TopLeading
                | ZStackAlignment::Leading
                | ZStackAlignment::BottomLeading => layer.justify_start(),
                ZStackAlignment::Top | ZStackAlignment::Center | ZStackAlignment::Bottom => {
                    layer.justify_center()
                }
                ZStackAlignment::TopTrailing
                | ZStackAlignment::Trailing
                | ZStackAlignment::BottomTrailing => layer.justify_end(),
            };

            // Apply vertical alignment (align-items)
            layer = match self.alignment {
                ZStackAlignment::TopLeading | ZStackAlignment::Top | ZStackAlignment::TopTrailing => {
                    layer.items_start()
                }
                ZStackAlignment::Leading | ZStackAlignment::Center | ZStackAlignment::Trailing => {
                    layer.items_center()
                }
                ZStackAlignment::BottomLeading
                | ZStackAlignment::Bottom
                | ZStackAlignment::BottomTrailing => layer.items_end(),
            };

            container = container.child(layer.child(child));
        }

        container
    }
}
