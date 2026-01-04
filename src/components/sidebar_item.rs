//! Sidebar item component.
//!
//! Individual items in a sidebar, such as navigation items or list entries.

use gpui::prelude::*;
use gpui::*;

/// A sidebar item component.
///
/// Used for navigation items, branch names, and other sidebar entries.
/// Supports selection state, bold text, and optional badges.
///
/// # Example
///
/// ```ignore
/// SidebarItem::new("main")
///     .selected(true)
///     .bold(true)
///     .on_click(handler)
/// ```
pub struct SidebarItem {
    id: ElementId,
    label: SharedString,
    selected: bool,
    bold: bool,
    badge: Option<AnyElement>,
    on_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
    on_double_click: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl SidebarItem {
    /// Creates a new sidebar item with the given id and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            selected: false,
            bold: false,
            badge: None,
            on_click: None,
            on_double_click: None,
        }
    }

    /// Sets whether the item is selected.
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Sets whether the item text is bold.
    pub fn bold(mut self, bold: bool) -> Self {
        self.bold = bold;
        self
    }

    /// Adds a badge to the item.
    pub fn badge(mut self, badge: impl IntoElement) -> Self {
        self.badge = Some(badge.into_any_element());
        self
    }

    /// Sets the click handler.
    pub fn on_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_click = Some(Box::new(handler));
        self
    }

    /// Sets the double-click handler.
    pub fn on_double_click(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_double_click = Some(Box::new(handler));
        self
    }
}

impl IntoElement for SidebarItem {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let (bg_color, text_color) = if self.selected {
            // Selected: blue gradient
            (
                hsla(211.0 / 360.0, 0.75, 0.58, 1.0),
                hsla(0.0, 0.0, 1.0, 1.0), // White text
            )
        } else {
            // Unselected: transparent
            (
                hsla(0.0, 0.0, 0.0, 0.0),
                hsla(0.0, 0.0, 0.20, 1.0), // Dark gray text
            )
        };

        let font_weight = if self.bold {
            FontWeight::BOLD
        } else {
            FontWeight::NORMAL
        };

        let mut item = div()
            .id(self.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px(px(8.0))
            .py(px(4.0))
            .rounded(px(4.0))
            .bg(bg_color)
            .cursor_pointer();

        // Add hover state for unselected items
        if !self.selected {
            item = item.hover(|style| style.bg(hsla(0.0, 0.0, 0.0, 0.05)));
        }

        // Left side: label
        item = item.child(
            div()
                .text_sm()
                .font_weight(font_weight)
                .text_color(text_color)
                .child(self.label),
        );

        // Right side: badge (if present)
        if let Some(badge) = self.badge {
            item = item.child(badge);
        }

        // Add click handler that dispatches based on click count
        let on_click = self.on_click;
        let on_double_click = self.on_double_click;

        item.on_click(move |event, window, app| {
            // Check click count from the mouse event
            let click_count = match event {
                ClickEvent::Mouse(mouse_event) => mouse_event.down.click_count,
                ClickEvent::Keyboard(_) => 1,
            };

            if click_count >= 2 {
                // Double-click
                if let Some(ref handler) = on_double_click {
                    handler(event, window, app);
                }
            } else {
                // Single click
                if let Some(ref handler) = on_click {
                    handler(event, window, app);
                }
            }
        })
    }
}
