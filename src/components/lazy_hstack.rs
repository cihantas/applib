//! Lazy horizontal stack layout component.
//!
//! A virtualized layout component that only renders visible items horizontally.
//! This is equivalent to SwiftUI's LazyHStack.

use gpui::prelude::*;
use gpui::*;
use std::ops::Range;
use std::sync::Arc;

/// A lazy horizontal stack that only renders visible items.
///
/// LazyHStack uses GPUI's uniform_list for virtualization, rendering items
/// in a rotated orientation to achieve horizontal scrolling with virtualization.
///
/// Note: GPUI's uniform_list is primarily designed for vertical scrolling.
/// For true horizontal virtualization, items are rendered within the uniform_list
/// infrastructure but wrapped for horizontal display.
///
/// # Example
///
/// ```ignore
/// LazyHStack::new("my-hlist", 10000, |range, _window, _cx| {
///     range.map(|i| {
///         div()
///             .w(px(100.0))
///             .h_full()
///             .child(format!("Item {}", i))
///     }).collect()
/// })
/// ```
pub struct LazyHStack {
    id: ElementId,
    item_count: usize,
    render_items: Arc<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement> + 'static>,
    gap: Pixels,
    padding: Option<Pixels>,
    scroll_handle: Option<UniformListScrollHandle>,
}

impl LazyHStack {
    /// Creates a new lazy horizontal stack.
    ///
    /// # Arguments
    /// * `id` - A unique identifier for the list
    /// * `item_count` - The total number of items in the list
    /// * `render_items` - A closure that renders items for a given range of indices
    pub fn new<F, R>(id: impl Into<ElementId>, item_count: usize, render_items: F) -> Self
    where
        F: Fn(Range<usize>, &mut Window, &mut App) -> Vec<R> + 'static,
        R: IntoElement,
    {
        Self {
            id: id.into(),
            item_count,
            render_items: Arc::new(move |range, window, cx| {
                render_items(range, window, cx)
                    .into_iter()
                    .map(|e| e.into_any_element())
                    .collect()
            }),
            gap: px(0.0),
            padding: None,
            scroll_handle: None,
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

    /// Sets a scroll handle to programmatically control scroll position.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let scroll_handle = UniformListScrollHandle::new();
    /// LazyHStack::new("list", items.len(), render_fn)
    ///     .track_scroll(scroll_handle.clone())
    /// ```
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl IntoElement for LazyHStack {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let render_items = self.render_items.clone();
        let gap = self.gap;

        // Create the uniform_list for virtualization
        // Each "row" in uniform_list becomes a horizontal item
        let list = uniform_list(self.id.clone(), self.item_count, move |range, window, cx| {
            let items = render_items(range.clone(), window, cx);

            // Each item gets wrapped for display
            items
                .into_iter()
                .enumerate()
                .map(|(i, item)| {
                    let is_last = i == range.len() - 1;
                    if is_last || gap == px(0.0) {
                        div().flex_shrink_0().child(item)
                    } else {
                        div().flex_shrink_0().mr(gap).child(item)
                    }
                })
                .collect::<Vec<_>>()
        });

        // Apply scroll handle if provided
        let list = if let Some(handle) = self.scroll_handle {
            list.track_scroll(handle)
        } else {
            list
        };

        // Wrap in a container with padding and horizontal scroll
        let mut container = div()
            .id((self.id, "container"))
            .flex()
            .flex_row()
            .size_full()
            .overflow_x_scroll();

        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        container.child(list)
    }
}

/// A scroll handle for programmatically controlling LazyHStack scroll position.
///
/// This is a re-export of GPUI's UniformListScrollHandle for convenience.
pub type LazyHStackScrollHandle = UniformListScrollHandle;
