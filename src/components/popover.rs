//! Popover component for GPUI.
//!
//! A transient view that appears above content, attached to an anchor element.
//! Features arrow pointing to anchor, multiple edge attachments, and click-outside-to-dismiss.

use gpui::prelude::*;
use gpui::*;

/// Edge where the popover appears relative to the anchor element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PopoverEdge {
    /// Popover appears above the anchor.
    Top,
    /// Popover appears below the anchor (default).
    #[default]
    Bottom,
    /// Popover appears to the left of the anchor.
    Leading,
    /// Popover appears to the right of the anchor.
    Trailing,
}

/// A popover component that displays floating content attached to an anchor.
///
/// Popover is a transient view that appears above other content, with an optional arrow
/// pointing to its anchor element. It supports multiple edge attachments and
/// shows on hover (for simple tooltip-like behavior).
///
/// For click-triggered popovers with external state control, use `ControlledPopover`.
///
/// # Example
///
/// ```ignore
/// Popover::new("my-popover", trigger_button)
///     .edge(PopoverEdge::Bottom)
///     .content(|| {
///         div()
///             .p_4()
///             .child("Popover content here")
///     })
/// ```
pub struct Popover {
    id: ElementId,
    anchor: AnyElement,
    edge: PopoverEdge,
    content: Option<Box<dyn FnOnce() -> AnyElement>>,
    show_arrow: bool,
}

impl Popover {
    /// Creates a new popover attached to the given anchor element.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this popover
    /// * `anchor` - The element that triggers/anchors the popover
    pub fn new(id: impl Into<ElementId>, anchor: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            anchor: anchor.into_any_element(),
            edge: PopoverEdge::default(),
            content: None,
            show_arrow: true,
        }
    }

    /// Sets the edge where the popover appears relative to the anchor.
    pub fn edge(mut self, edge: PopoverEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Positions the popover above the anchor.
    pub fn top(mut self) -> Self {
        self.edge = PopoverEdge::Top;
        self
    }

    /// Positions the popover below the anchor.
    pub fn bottom(mut self) -> Self {
        self.edge = PopoverEdge::Bottom;
        self
    }

    /// Positions the popover to the left of the anchor.
    pub fn leading(mut self) -> Self {
        self.edge = PopoverEdge::Leading;
        self
    }

    /// Positions the popover to the right of the anchor.
    pub fn trailing(mut self) -> Self {
        self.edge = PopoverEdge::Trailing;
        self
    }

    /// Sets the content to display in the popover.
    ///
    /// The content builder receives a reference to the popover for context.
    pub fn content<F, E>(mut self, builder: F) -> Self
    where
        F: FnOnce() -> E + 'static,
        E: IntoElement,
    {
        self.content = Some(Box::new(move || builder().into_any_element()));
        self
    }

    /// Sets whether to show the arrow pointing to the anchor.
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Hides the arrow pointing to the anchor.
    pub fn without_arrow(mut self) -> Self {
        self.show_arrow = false;
        self
    }

    /// Builds the arrow element for the given edge.
    /// Uses a simple diamond/square shape rotated to create arrow effect.
    fn build_arrow(edge: PopoverEdge, panel_bg: Hsla, border_color: Hsla) -> Div {
        let arrow_size = 10.0;
        let half_size = arrow_size / 2.0;

        // Create a small square that will act as the arrow
        // Position it so half overlaps the panel
        let arrow = div()
            .absolute()
            .size(px(arrow_size))
            .bg(panel_bg)
            .border_1()
            .border_color(border_color);

        match edge {
            PopoverEdge::Top => {
                // Arrow points down, at bottom of popover
                arrow
                    .bottom(px(-half_size))
                    .left(px(16.0))
                    .border_t_0()
                    .border_l_0()
            }
            PopoverEdge::Bottom => {
                // Arrow points up, at top of popover
                arrow
                    .top(px(-half_size))
                    .left(px(16.0))
                    .border_b_0()
                    .border_r_0()
            }
            PopoverEdge::Leading => {
                // Arrow points right, at right of popover
                arrow
                    .right(px(-half_size))
                    .top(px(12.0))
                    .border_t_0()
                    .border_l_0()
            }
            PopoverEdge::Trailing => {
                // Arrow points left, at left of popover
                arrow
                    .left(px(-half_size))
                    .top(px(12.0))
                    .border_b_0()
                    .border_r_0()
            }
        }
    }
}

impl IntoElement for Popover {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Colors
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0); // White background
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let edge = self.edge;
        let show_arrow = self.show_arrow;

        // Build the content if provided
        let content_element = self
            .content
            .map(|builder| builder())
            .unwrap_or_else(|| div().into_any_element());

        // Build the popover panel
        let mut panel = div()
            .absolute()
            .overflow_hidden()
            .bg(panel_bg)
            .rounded(px(8.0))
            .border_1()
            .border_color(border_color)
            .shadow(vec![
                BoxShadow {
                    color: shadow_color,
                    offset: point(px(0.0), px(4.0)),
                    blur_radius: px(12.0),
                    spread_radius: px(0.0),
                },
                // Subtle inner highlight
                BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.5),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                },
            ])
            .child(content_element);

        // Add arrow if enabled
        if show_arrow {
            panel = panel.child(Self::build_arrow(edge, panel_bg, border_color));
        }

        // Position panel based on edge
        let positioned_panel = match edge {
            PopoverEdge::Top => panel.bottom_full().left(px(0.0)).mb(px(8.0)),
            PopoverEdge::Bottom => panel.top_full().left(px(0.0)).mt(px(8.0)),
            PopoverEdge::Leading => panel.right_full().top(px(0.0)).mr(px(8.0)),
            PopoverEdge::Trailing => panel.left_full().top(px(0.0)).ml(px(8.0)),
        };

        // The popover container with the anchor and overlay
        // Uses group hover pattern for showing on hover
        div()
            .id(self.id)
            .relative()
            .group("")
            .child(self.anchor)
            .child(
                // Popover that appears on group hover
                div()
                    .invisible()
                    .group_hover("", |style| style.visible())
                    .child(positioned_panel),
            )
    }
}

/// A controlled popover component for use in stateful views.
///
/// Unlike the basic `Popover` which shows on hover, `ControlledPopover` is
/// controlled by an external `is_open` state, making it suitable for click-triggered
/// popovers or complex interaction patterns.
///
/// # Example
///
/// ```ignore
/// // In your view state
/// struct MyView {
///     popover_open: bool,
/// }
///
/// // In render
/// ControlledPopover::new("my-popover", self.popover_open, button)
///     .edge(PopoverEdge::Bottom)
///     .content(|| {
///         div().p_4().child("Controlled content")
///     })
///     .on_dismiss(cx.listener(|this, _event, _window, cx| {
///         this.popover_open = false;
///         cx.notify();
///     }))
/// ```
pub struct ControlledPopover {
    id: ElementId,
    is_open: bool,
    anchor: AnyElement,
    edge: PopoverEdge,
    content: Option<Box<dyn FnOnce() -> AnyElement>>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    show_arrow: bool,
}

impl ControlledPopover {
    /// Creates a new controlled popover.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this popover
    /// * `is_open` - Whether the popover is currently visible
    /// * `anchor` - The element that anchors the popover
    pub fn new(id: impl Into<ElementId>, is_open: bool, anchor: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            is_open,
            anchor: anchor.into_any_element(),
            edge: PopoverEdge::default(),
            content: None,
            on_dismiss: None,
            show_arrow: true,
        }
    }

    /// Sets the edge where the popover appears relative to the anchor.
    pub fn edge(mut self, edge: PopoverEdge) -> Self {
        self.edge = edge;
        self
    }

    /// Positions the popover above the anchor.
    pub fn top(mut self) -> Self {
        self.edge = PopoverEdge::Top;
        self
    }

    /// Positions the popover below the anchor.
    pub fn bottom(mut self) -> Self {
        self.edge = PopoverEdge::Bottom;
        self
    }

    /// Positions the popover to the left of the anchor.
    pub fn leading(mut self) -> Self {
        self.edge = PopoverEdge::Leading;
        self
    }

    /// Positions the popover to the right of the anchor.
    pub fn trailing(mut self) -> Self {
        self.edge = PopoverEdge::Trailing;
        self
    }

    /// Sets the content to display in the popover.
    pub fn content<F, E>(mut self, builder: F) -> Self
    where
        F: FnOnce() -> E + 'static,
        E: IntoElement,
    {
        self.content = Some(Box::new(move || builder().into_any_element()));
        self
    }

    /// Sets whether to show the arrow pointing to the anchor.
    pub fn show_arrow(mut self, show: bool) -> Self {
        self.show_arrow = show;
        self
    }

    /// Hides the arrow pointing to the anchor.
    pub fn without_arrow(mut self) -> Self {
        self.show_arrow = false;
        self
    }

    /// Sets the dismiss handler, called when clicking outside the popover.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }

    /// Builds the arrow element for the given edge.
    fn build_arrow(edge: PopoverEdge, panel_bg: Hsla, border_color: Hsla) -> Div {
        let arrow_size = 10.0;
        let half_size = arrow_size / 2.0;

        let arrow = div()
            .absolute()
            .size(px(arrow_size))
            .bg(panel_bg)
            .border_1()
            .border_color(border_color);

        match edge {
            PopoverEdge::Top => arrow
                .bottom(px(-half_size))
                .left(px(16.0))
                .border_t_0()
                .border_l_0(),
            PopoverEdge::Bottom => arrow
                .top(px(-half_size))
                .left(px(16.0))
                .border_b_0()
                .border_r_0(),
            PopoverEdge::Leading => arrow
                .right(px(-half_size))
                .top(px(12.0))
                .border_t_0()
                .border_l_0(),
            PopoverEdge::Trailing => arrow
                .left(px(-half_size))
                .top(px(12.0))
                .border_b_0()
                .border_r_0(),
        }
    }
}

impl IntoElement for ControlledPopover {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let edge = self.edge;
        let show_arrow = self.show_arrow;

        // Build the container with anchor
        let mut container = div().id(self.id).relative().child(self.anchor);

        // Only add popover content if open
        if self.is_open {
            let content_element = self
                .content
                .map(|builder| builder())
                .unwrap_or_else(|| div().into_any_element());

            let mut panel = div()
                .absolute()
                .overflow_hidden()
                .bg(panel_bg)
                .rounded(px(8.0))
                .border_1()
                .border_color(border_color)
                .shadow(vec![
                    BoxShadow {
                        color: shadow_color,
                        offset: point(px(0.0), px(4.0)),
                        blur_radius: px(12.0),
                        spread_radius: px(0.0),
                    },
                    BoxShadow {
                        color: hsla(0.0, 0.0, 1.0, 0.5),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(0.0),
                    },
                ])
                .child(content_element);

            if show_arrow {
                panel = panel.child(Self::build_arrow(edge, panel_bg, border_color));
            }

            let positioned_panel = match edge {
                PopoverEdge::Top => panel.bottom_full().left(px(0.0)).mb(px(8.0)),
                PopoverEdge::Bottom => panel.top_full().left(px(0.0)).mt(px(8.0)),
                PopoverEdge::Leading => panel.right_full().top(px(0.0)).mr(px(8.0)),
                PopoverEdge::Trailing => panel.left_full().top(px(0.0)).ml(px(8.0)),
            };

            // Stop propagation when clicking inside the panel
            let panel_with_events = positioned_panel
                .id("popover-panel")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    cx.stop_propagation();
                });

            // Build the overlay with dismiss handling
            let popover_overlay = if let Some(handler) = self.on_dismiss {
                div()
                    .id("popover-dismiss-area")
                    .on_click(move |event, window, cx| {
                        handler(event, window, cx);
                    })
                    .child(panel_with_events)
            } else {
                div().id("popover-no-dismiss").child(panel_with_events)
            };

            container = container.child(popover_overlay);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_popover_edge() {
        let popover = Popover::new("test", div()).top();
        assert_eq!(popover.edge, PopoverEdge::Top);

        let popover = Popover::new("test", div()).bottom();
        assert_eq!(popover.edge, PopoverEdge::Bottom);

        let popover = Popover::new("test", div()).leading();
        assert_eq!(popover.edge, PopoverEdge::Leading);

        let popover = Popover::new("test", div()).trailing();
        assert_eq!(popover.edge, PopoverEdge::Trailing);
    }

    #[test]
    fn test_popover_edge_setter() {
        let popover = Popover::new("test", div()).edge(PopoverEdge::Top);
        assert_eq!(popover.edge, PopoverEdge::Top);

        let popover = Popover::new("test", div()).edge(PopoverEdge::Leading);
        assert_eq!(popover.edge, PopoverEdge::Leading);
    }

    #[test]
    fn test_popover_arrow() {
        let popover = Popover::new("test", div());
        assert!(popover.show_arrow);

        let popover = Popover::new("test", div()).without_arrow();
        assert!(!popover.show_arrow);

        let popover = Popover::new("test", div()).show_arrow(false);
        assert!(!popover.show_arrow);
    }

    #[test]
    fn test_controlled_popover_state() {
        let popover = ControlledPopover::new("test", true, div());
        assert!(popover.is_open);

        let popover = ControlledPopover::new("test", false, div());
        assert!(!popover.is_open);
    }

    #[test]
    fn test_controlled_popover_edge() {
        let popover = ControlledPopover::new("test", false, div()).top();
        assert_eq!(popover.edge, PopoverEdge::Top);

        let popover = ControlledPopover::new("test", false, div()).trailing();
        assert_eq!(popover.edge, PopoverEdge::Trailing);
    }

    #[test]
    fn test_popover_default_edge() {
        let popover = Popover::new("test", div());
        assert_eq!(popover.edge, PopoverEdge::Bottom);
    }
}
