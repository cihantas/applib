//! Lazy vertical stack layout component.
//!
//! A virtualized layout component that only renders visible items.
//! This is equivalent to SwiftUI's LazyVStack.

use gpui::prelude::*;
use gpui::*;
use std::ops::Range;
use std::sync::Arc;

/// A lazy vertical stack that only renders visible items.
///
/// LazyVStack efficiently handles large lists by only rendering items
/// currently visible in the viewport, plus a small buffer. This is
/// ideal for lists with thousands of items.
///
/// # Example
///
/// ```ignore
/// LazyVStack::new("my-list", 10000, |range, _window, _cx| {
///     range.map(|i| {
///         div()
///             .h(px(44.0))
///             .child(format!("Row {}", i))
///     }).collect()
/// })
/// ```
pub struct LazyVStack {
    id: ElementId,
    item_count: usize,
    render_items: Arc<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<AnyElement> + 'static>,
    gap: Pixels,
    padding: Option<Pixels>,
    scroll_handle: Option<UniformListScrollHandle>,
}

impl LazyVStack {
    /// Creates a new lazy vertical stack.
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
    /// LazyVStack::new("list", items.len(), render_fn)
    ///     .track_scroll(scroll_handle.clone())
    /// ```
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl IntoElement for LazyVStack {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let render_items = self.render_items.clone();
        let gap = self.gap;

        // Create the uniform_list with virtualization
        let list = uniform_list(self.id, self.item_count, move |range, window, cx| {
            let items = render_items(range.clone(), window, cx);

            // Wrap each item to apply gap (margin-bottom for all but last)
            items
                .into_iter()
                .enumerate()
                .map(|(i, item)| {
                    let is_last = i == range.len() - 1;
                    if is_last || gap == px(0.0) {
                        div().child(item)
                    } else {
                        div().mb(gap).child(item)
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

        // Wrap in a container with padding
        let mut container = div().flex().flex_col().size_full();

        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        container.child(list)
    }
}

/// A scroll handle for programmatically controlling LazyVStack scroll position.
///
/// This is a re-export of GPUI's UniformListScrollHandle for convenience.
pub type LazyVStackScrollHandle = UniformListScrollHandle;
