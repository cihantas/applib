//! Split view component for divided layouts with resizable panes.
//!
//! Provides a two-pane layout that can be oriented horizontally or vertically,
//! with a draggable divider for resizing panes.
//!
//! # SwiftUI Comparison
//!
//! This component is analogous to SwiftUI's `HSplitView`/`VSplitView` which provide
//! draggable dividers out of the box. Like SwiftUI, the split view manages its own
//! divider position state internally, but can optionally report changes via callback.

use gpui::prelude::*;
use gpui::*;
use std::cell::Cell;
use std::rc::Rc;

/// Marker type for split view divider drag state
#[derive(Clone)]
struct SplitDividerDrag;

/// Invisible drag ghost view (required by GPUI's drag API)
struct DragGhost;

impl Render for DragGhost {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        // Invisible - we don't want a visual drag preview for split resizing
        div().w(px(0.0)).h(px(0.0))
    }
}

/// Orientation of the split view
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SplitOrientation {
    /// Panes arranged side by side (left/right)
    #[default]
    Horizontal,
    /// Panes arranged top to bottom
    Vertical,
}

/// Divider width/height in pixels
const DIVIDER_SIZE: f32 = 1.0;
/// Draggable hit area for the divider (wider than visual for easier grabbing)
const DIVIDER_HIT_AREA: f32 = 8.0;

/// Default constraints
const DEFAULT_MIN_SIZE: f32 = 100.0;
const DEFAULT_MAX_SIZE: f32 = 10000.0;

/// A split view builder for creating resizable two-pane layouts.
///
/// Use this builder to configure a split view, then convert it to `SplitViewState`
/// for use in a parent view that implements `Render`.
///
/// # Example
///
/// ```ignore
/// // In your view struct:
/// struct MyView {
///     split_view: Entity<SplitViewState>,
/// }
///
/// impl MyView {
///     fn new(cx: &mut Context<Self>) -> Self {
///         Self {
///             split_view: cx.new(|_| SplitViewState::from(
///                 SplitView::horizontal()
///                     .first_size(px(250.0))
///                     .min_first_size(px(150.0))
///                     .max_first_size(px(400.0))
///             )),
///         }
///     }
/// }
/// ```
pub struct SplitView {
    orientation: SplitOrientation,
    first_size: Pixels,
    min_first_size: Pixels,
    max_first_size: Pixels,
    divider_color: Hsla,
    on_resize: Option<Rc<dyn Fn(Pixels, &mut Window, &mut App) + 'static>>,
}

impl SplitView {
    /// Creates a new horizontal split view (side by side panes).
    pub fn horizontal() -> Self {
        Self {
            orientation: SplitOrientation::Horizontal,
            first_size: px(200.0),
            min_first_size: px(DEFAULT_MIN_SIZE),
            max_first_size: px(DEFAULT_MAX_SIZE),
            divider_color: hsla(0.0, 0.0, 0.85, 1.0),
            on_resize: None,
        }
    }

    /// Creates a new vertical split view (stacked panes).
    pub fn vertical() -> Self {
        Self {
            orientation: SplitOrientation::Vertical,
            first_size: px(200.0),
            min_first_size: px(DEFAULT_MIN_SIZE),
            max_first_size: px(DEFAULT_MAX_SIZE),
            divider_color: hsla(0.0, 0.0, 0.85, 1.0),
            on_resize: None,
        }
    }

    /// Sets the initial size for the first pane.
    /// For horizontal splits, this is the width.
    /// For vertical splits, this is the height.
    pub fn first_size(mut self, size: Pixels) -> Self {
        self.first_size = size;
        self
    }

    /// Sets the minimum size for the first pane.
    pub fn min_first_size(mut self, size: Pixels) -> Self {
        self.min_first_size = size;
        self
    }

    /// Sets the maximum size for the first pane.
    pub fn max_first_size(mut self, size: Pixels) -> Self {
        self.max_first_size = size;
        self
    }

    /// Sets the divider color.
    pub fn divider_color(mut self, color: Hsla) -> Self {
        self.divider_color = color;
        self
    }

    /// Sets a callback that's called when the user resizes the panes.
    ///
    /// This is useful for programmatic control or persisting the split position.
    /// The callback receives the new first pane size.
    ///
    /// # Example
    ///
    /// ```ignore
    /// SplitView::horizontal()
    ///     .first_size(px(250.0))
    ///     .on_resize(|new_size, _window, _cx| {
    ///         println!("Split resized to: {:?}", new_size);
    ///     })
    /// ```
    pub fn on_resize(
        mut self,
        handler: impl Fn(Pixels, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_resize = Some(Rc::new(handler));
        self
    }
}

/// Stateful split view that manages its own divider position and handles drag events.
///
/// This is the runtime state for a split view. Create it from a `SplitView` builder
/// and use it as an `Entity<SplitViewState>` in your parent view.
///
/// # Example
///
/// ```ignore
/// // In parent view:
/// struct MyView {
///     split: Entity<SplitViewState>,
///     left_content: String,
///     right_content: String,
/// }
///
/// impl Render for MyView {
///     fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
///         let left = div().child(&self.left_content);
///         let right = div().child(&self.right_content);
///
///         self.split.update(cx, |split, _| {
///             split.set_first(left);
///             split.set_second(right);
///         });
///
///         self.split.clone()
///     }
/// }
/// ```
pub struct SplitViewState {
    orientation: SplitOrientation,
    first_size: Pixels,
    min_first_size: Pixels,
    max_first_size: Pixels,
    divider_color: Hsla,
    on_resize: Option<Rc<dyn Fn(Pixels, &mut Window, &mut App) + 'static>>,

    // Content (set each render)
    first_content: Option<AnyElement>,
    second_content: Option<AnyElement>,

    // Container bounds for coordinate calculation
    container_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
}

impl SplitViewState {
    /// Gets the current first pane size.
    pub fn first_size(&self) -> Pixels {
        self.first_size
    }

    /// Sets the first pane size programmatically.
    pub fn set_first_size(&mut self, size: Pixels) {
        self.first_size = size.clamp(self.min_first_size, self.max_first_size);
    }

    /// Sets the first pane content. Call this each render.
    pub fn set_first(&mut self, content: impl IntoElement) {
        self.first_content = Some(content.into_any_element());
    }

    /// Sets the second pane content. Call this each render.
    pub fn set_second(&mut self, content: impl IntoElement) {
        self.second_content = Some(content.into_any_element());
    }
}

impl Render for SplitViewState {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_horizontal = self.orientation == SplitOrientation::Horizontal;
        let orientation = self.orientation;
        let min_size = self.min_first_size;
        let max_size = self.max_first_size;
        let on_resize = self.on_resize.clone();

        // Container with bounds tracking
        let bounds_ref = self.container_bounds.clone();
        let bounds_ref_for_drag = self.container_bounds.clone();
        let bounds_tracker = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, _window, _cx| {
                bounds_ref.set(Some(bounds));
            },
        )
        .absolute()
        .size_full();

        // First pane
        let mut first_pane = div()
            .flex()
            .flex_col()
            .overflow_hidden();

        if is_horizontal {
            first_pane = first_pane.w(self.first_size).h_full().flex_shrink_0();
        } else {
            first_pane = first_pane.h(self.first_size).w_full().flex_shrink_0();
        }

        if let Some(content) = self.first_content.take() {
            first_pane = first_pane.child(content);
        }

        // Divider - visual line
        let divider_visual = if is_horizontal {
            div()
                .w(px(DIVIDER_SIZE))
                .h_full()
                .bg(self.divider_color)
        } else {
            div()
                .h(px(DIVIDER_SIZE))
                .w_full()
                .bg(self.divider_color)
        };

        // Divider - hit area (wider for easier grabbing)
        let cursor_style = if is_horizontal {
            CursorStyle::ResizeLeftRight
        } else {
            CursorStyle::ResizeUpDown
        };

        let divider_hit = if is_horizontal {
            div()
                .id("split-divider")
                .w(px(DIVIDER_HIT_AREA))
                .ml(px(-DIVIDER_HIT_AREA / 2.0 + DIVIDER_SIZE / 2.0))
                .h_full()
                .flex_shrink_0()
                .cursor(cursor_style)
                .flex()
                .justify_center()
                .child(divider_visual)
        } else {
            div()
                .id("split-divider")
                .h(px(DIVIDER_HIT_AREA))
                .mt(px(-DIVIDER_HIT_AREA / 2.0 + DIVIDER_SIZE / 2.0))
                .w_full()
                .flex_shrink_0()
                .cursor(cursor_style)
                .flex()
                .items_center()
                .child(divider_visual)
        };

        // Use GPUI's drag API for proper global mouse capture during drag
        let divider_hit = divider_hit
            .on_drag(SplitDividerDrag, |_drag, _offset, _window, cx| {
                cx.new(|_| DragGhost)
            })
            .on_drag_move(cx.listener(
                move |this, event: &DragMoveEvent<SplitDividerDrag>, window, cx| {
                    if let Some(bounds) = bounds_ref_for_drag.get() {
                        let new_size = match orientation {
                            SplitOrientation::Horizontal => {
                                event.event.position.x - bounds.origin.x
                            }
                            SplitOrientation::Vertical => {
                                event.event.position.y - bounds.origin.y
                            }
                        };

                        let clamped = new_size.clamp(min_size, max_size);

                        if clamped != this.first_size {
                            this.first_size = clamped;

                            if let Some(ref handler) = on_resize {
                                handler(this.first_size, window, cx);
                            }

                            cx.notify();
                        }
                    }
                },
            ));

        // Second pane
        let mut second_pane = div()
            .flex()
            .flex_col()
            .flex_1()
            .overflow_hidden();

        if let Some(content) = self.second_content.take() {
            second_pane = second_pane.child(content);
        }

        // Build container
        let mut container = div()
            .relative()
            .flex()
            .size_full();

        if is_horizontal {
            container = container.flex_row();
        } else {
            container = container.flex_col();
        }

        container
            .child(first_pane)
            .child(divider_hit)
            .child(second_pane)
            .child(bounds_tracker)
    }
}

impl From<SplitView> for SplitViewState {
    fn from(builder: SplitView) -> Self {
        let first_size = builder.first_size.clamp(builder.min_first_size, builder.max_first_size);

        SplitViewState {
            orientation: builder.orientation,
            first_size,
            min_first_size: builder.min_first_size,
            max_first_size: builder.max_first_size,
            divider_color: builder.divider_color,
            on_resize: builder.on_resize,
            first_content: None,
            second_content: None,
            container_bounds: Rc::new(Cell::new(None)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_view_horizontal() {
        let split = SplitView::horizontal();
        assert_eq!(split.orientation, SplitOrientation::Horizontal);
        assert_eq!(split.first_size, px(200.0));
    }

    #[test]
    fn test_split_view_vertical() {
        let split = SplitView::vertical();
        assert_eq!(split.orientation, SplitOrientation::Vertical);
    }

    #[test]
    fn test_split_view_first_size() {
        let split = SplitView::horizontal().first_size(px(300.0));
        assert_eq!(split.first_size, px(300.0));
    }

    #[test]
    fn test_split_view_constraints() {
        let split = SplitView::horizontal()
            .first_size(px(250.0))
            .min_first_size(px(150.0))
            .max_first_size(px(400.0));

        assert_eq!(split.min_first_size, px(150.0));
        assert_eq!(split.max_first_size, px(400.0));
    }

    #[test]
    fn test_split_view_state_clamping() {
        let state: SplitViewState = SplitView::horizontal()
            .first_size(px(50.0)) // Below min
            .min_first_size(px(100.0))
            .into();

        assert_eq!(state.first_size, px(100.0));
    }

    #[test]
    fn test_split_view_state_set_size() {
        let mut state: SplitViewState = SplitView::horizontal()
            .min_first_size(px(100.0))
            .max_first_size(px(400.0))
            .into();

        state.set_first_size(px(250.0));
        assert_eq!(state.first_size, px(250.0));

        // Test clamping
        state.set_first_size(px(50.0));
        assert_eq!(state.first_size, px(100.0));

        state.set_first_size(px(500.0));
        assert_eq!(state.first_size, px(400.0));
    }
}
