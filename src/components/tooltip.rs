//! Tooltip component for GPUI.
//!
//! A hover hint that appears on hover near the wrapped element.
//! Uses dark background with light text for visibility.

use gpui::prelude::*;
use gpui::*;

/// Position where the tooltip appears relative to the wrapped element.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TooltipPosition {
    /// Tooltip appears above the element (default).
    #[default]
    Top,
    /// Tooltip appears below the element.
    Bottom,
    /// Tooltip appears to the left of the element.
    Left,
    /// Tooltip appears to the right of the element.
    Right,
}

/// A tooltip component that wraps content and shows a hint on hover.
///
/// # Example
///
/// ```ignore
/// Tooltip::new("save-btn-tooltip", save_button, "Save the current document")
/// ```
///
/// Or using the builder pattern:
/// ```ignore
/// Tooltip::new("tooltip-id", my_element, "Tooltip text")
///     .position(TooltipPosition::Bottom)
/// ```
pub struct Tooltip {
    id: ElementId,
    child: AnyElement,
    text: SharedString,
    position: TooltipPosition,
}

impl Tooltip {
    /// Creates a new tooltip wrapping the given element.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this tooltip
    /// * `child` - The element to wrap (tooltip trigger)
    /// * `text` - The tooltip text to display
    pub fn new(
        id: impl Into<ElementId>,
        child: impl IntoElement,
        text: impl Into<SharedString>,
    ) -> Self {
        Self {
            id: id.into(),
            child: child.into_any_element(),
            text: text.into(),
            position: TooltipPosition::default(),
        }
    }

    /// Sets the tooltip position relative to the wrapped element.
    pub fn position(mut self, position: TooltipPosition) -> Self {
        self.position = position;
        self
    }

    /// Positions the tooltip above the element.
    pub fn top(mut self) -> Self {
        self.position = TooltipPosition::Top;
        self
    }

    /// Positions the tooltip below the element.
    pub fn bottom(mut self) -> Self {
        self.position = TooltipPosition::Bottom;
        self
    }

    /// Positions the tooltip to the left of the element.
    pub fn left(mut self) -> Self {
        self.position = TooltipPosition::Left;
        self
    }

    /// Positions the tooltip to the right of the element.
    pub fn right(mut self) -> Self {
        self.position = TooltipPosition::Right;
        self
    }
}

impl IntoElement for Tooltip {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Colors (dark background, light text)
        let tooltip_bg = hsla(0.0, 0.0, 0.15, 0.95);
        let tooltip_text_color = hsla(0.0, 0.0, 0.95, 1.0);
        let tooltip_border = hsla(0.0, 0.0, 0.25, 1.0);

        // Build the tooltip panel with position-specific offsets
        let tooltip_panel = div()
            .absolute()
            .px(px(8.0))
            .py(px(4.0))
            .bg(tooltip_bg)
            .text_color(tooltip_text_color)
            .text_xs()
            .rounded(px(4.0))
            .border_1()
            .border_color(tooltip_border)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.3),
                offset: point(px(0.0), px(2.0)),
                blur_radius: px(6.0),
                spread_radius: px(0.0),
            }])
            .whitespace_nowrap()
            .child(self.text.clone());

        // Position tooltip based on the chosen position
        // Offsets are relative to the trigger element
        let positioned_tooltip = match self.position {
            TooltipPosition::Top => tooltip_panel.bottom_full().left(px(0.0)).mb(px(4.0)),
            TooltipPosition::Bottom => tooltip_panel.top_full().left(px(0.0)).mt(px(4.0)),
            TooltipPosition::Left => tooltip_panel.right_full().top(px(0.0)).mr(px(4.0)),
            TooltipPosition::Right => tooltip_panel.left_full().top(px(0.0)).ml(px(4.0)),
        };

        // The tooltip container with hover logic using group hover pattern
        div()
            .id(self.id)
            .relative()
            .group("")
            .child(self.child)
            .child(
                // Tooltip that appears on group hover
                div()
                    .invisible()
                    .group_hover("", |style| style.visible())
                    .child(positioned_tooltip),
            )
    }
}

/// A stateful tooltip wrapper for elements that need hover delay tracking.
///
/// This is a more advanced version that tracks hover state and shows the tooltip
/// after a delay. It requires being used within a view that can handle state.
///
/// For simple use cases, prefer the stateless `Tooltip` component.
pub struct TooltipState {
    /// Whether the tooltip is currently visible
    pub visible: bool,
    /// Timer ID for tracking hover delay
    hover_start: Option<std::time::Instant>,
}

impl TooltipState {
    /// Creates a new tooltip state.
    pub fn new() -> Self {
        Self {
            visible: false,
            hover_start: None,
        }
    }

    /// Call this when mouse enters the trigger element.
    pub fn on_mouse_enter(&mut self) {
        self.hover_start = Some(std::time::Instant::now());
    }

    /// Call this when mouse leaves the trigger element.
    pub fn on_mouse_leave(&mut self) {
        self.visible = false;
        self.hover_start = None;
    }

    /// Check if tooltip should be shown based on hover duration.
    /// Call this on each render to update visibility.
    ///
    /// Returns true if visibility changed.
    pub fn update(&mut self, delay_ms: u64) -> bool {
        if let Some(start) = self.hover_start {
            let elapsed = start.elapsed().as_millis() as u64;
            if elapsed >= delay_ms && !self.visible {
                self.visible = true;
                return true;
            }
        }
        false
    }
}

impl Default for TooltipState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tooltip_position() {
        let text: SharedString = "test".into();
        let tooltip = Tooltip::new("test", div(), text.clone()).top();
        assert_eq!(tooltip.position, TooltipPosition::Top);

        let tooltip = Tooltip::new("test", div(), text.clone()).bottom();
        assert_eq!(tooltip.position, TooltipPosition::Bottom);

        let tooltip = Tooltip::new("test", div(), text.clone()).left();
        assert_eq!(tooltip.position, TooltipPosition::Left);

        let tooltip = Tooltip::new("test", div(), text).right();
        assert_eq!(tooltip.position, TooltipPosition::Right);
    }

    #[test]
    fn test_tooltip_state() {
        let mut state = TooltipState::new();
        assert!(!state.visible);

        state.on_mouse_enter();
        assert!(state.hover_start.is_some());

        state.on_mouse_leave();
        assert!(!state.visible);
        assert!(state.hover_start.is_none());
    }
}
