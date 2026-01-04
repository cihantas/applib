//! Window frame component with resize handles.
//!
//! Provides a container that adds invisible resize handles around the window edges
//! for client-side decorated windows.

use gpui::prelude::*;
use gpui::*;

/// The width of the resize handle hit area in pixels.
const RESIZE_HANDLE_SIZE: f32 = 6.0;

/// A window frame that provides resize handles for client-side decorated windows.
///
/// Wraps content with invisible resize handles at all edges and corners.
///
/// # Example
///
/// ```ignore
/// WindowFrame::new()
///     .child(your_content)
/// ```
pub struct WindowFrame {
    children: Vec<AnyElement>,
}

impl WindowFrame {
    /// Creates a new window frame.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
        }
    }

    /// Adds a child element to the frame content.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the frame content.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl Default for WindowFrame {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for WindowFrame {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let handle_size = px(RESIZE_HANDLE_SIZE);

        // Content area
        let mut content = div().flex().flex_col().size_full();
        for child in self.children {
            content = content.child(child);
        }

        div()
            .relative()
            .size_full()
            // Main content
            .child(content)
            // Top edge
            .child(
                div()
                    .id("resize-top")
                    .absolute()
                    .top_0()
                    .left(handle_size)
                    .right(handle_size)
                    .h(handle_size)
                    .cursor(CursorStyle::ResizeUpDown)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::Top);
                    }),
            )
            // Bottom edge
            .child(
                div()
                    .id("resize-bottom")
                    .absolute()
                    .bottom_0()
                    .left(handle_size)
                    .right(handle_size)
                    .h(handle_size)
                    .cursor(CursorStyle::ResizeUpDown)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::Bottom);
                    }),
            )
            // Left edge
            .child(
                div()
                    .id("resize-left")
                    .absolute()
                    .left_0()
                    .top(handle_size)
                    .bottom(handle_size)
                    .w(handle_size)
                    .cursor(CursorStyle::ResizeLeftRight)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::Left);
                    }),
            )
            // Right edge
            .child(
                div()
                    .id("resize-right")
                    .absolute()
                    .right_0()
                    .top(handle_size)
                    .bottom(handle_size)
                    .w(handle_size)
                    .cursor(CursorStyle::ResizeLeftRight)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::Right);
                    }),
            )
            // Top-left corner
            .child(
                div()
                    .id("resize-top-left")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size(handle_size)
                    .cursor(CursorStyle::ResizeUpLeftDownRight)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::TopLeft);
                    }),
            )
            // Top-right corner
            .child(
                div()
                    .id("resize-top-right")
                    .absolute()
                    .top_0()
                    .right_0()
                    .size(handle_size)
                    .cursor(CursorStyle::ResizeUpRightDownLeft)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::TopRight);
                    }),
            )
            // Bottom-left corner
            .child(
                div()
                    .id("resize-bottom-left")
                    .absolute()
                    .bottom_0()
                    .left_0()
                    .size(handle_size)
                    .cursor(CursorStyle::ResizeUpRightDownLeft)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::BottomLeft);
                    }),
            )
            // Bottom-right corner
            .child(
                div()
                    .id("resize-bottom-right")
                    .absolute()
                    .bottom_0()
                    .right_0()
                    .size(handle_size)
                    .cursor(CursorStyle::ResizeUpLeftDownRight)
                    .on_mouse_down(MouseButton::Left, |_event, window, _cx| {
                        window.start_window_resize(ResizeEdge::BottomRight);
                    }),
            )
    }
}
