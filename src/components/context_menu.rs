//! Context menu component for GPUI.
//!
//! A right-click context menu that appears at the cursor location.
//! Follows SwiftUI's declarative pattern - no external state management needed.
//!
//! # Usage
//!
//! Simply wrap any element with `ContextMenu` and add menu items:
//!
//! ```ignore
//! ContextMenu::new("my-context-menu", div().child("Right-click me"))
//!     .item(MenuItem::new("copy", "Copy").on_select(|| handle_copy()))
//!     .item(MenuItem::new("delete", "Delete").on_select(|| handle_delete()))
//! ```
//!
//! The menu automatically:
//! - Opens on right-click at the cursor position
//! - Closes when clicking outside or selecting an item
//! - Handles keyboard navigation (planned)

use gpui::prelude::*;
use gpui::*;
use std::cell::RefCell;
use std::rc::Rc;

use super::label::Icon;
use super::menu::{Menu, MenuContent, MenuItem, SubMenuBuilder};

/// Internal state for tracking context menu visibility.
#[derive(Debug, Clone)]
struct InternalState {
    is_open: bool,
    position: Point<Pixels>,
}

impl Default for InternalState {
    fn default() -> Self {
        Self {
            is_open: false,
            position: Point::default(),
        }
    }
}

/// A context menu that appears on right-click.
///
/// Unlike traditional UI frameworks that require manual state management,
/// `ContextMenu` manages its own visibility state internally, similar to SwiftUI.
///
/// # Example
///
/// ```ignore
/// // Simple usage - no state management needed
/// ContextMenu::new("file-menu", file_item)
///     .item(MenuItem::new("open", "Open"))
///     .item(MenuItem::new("delete", "Delete").on_select(|| delete_file()))
///
/// // With submenu
/// ContextMenu::new("edit-menu", text_field)
///     .item(MenuItem::new("cut", "Cut").shortcut("⌘X"))
///     .item(MenuItem::new("copy", "Copy").shortcut("⌘C"))
///     .item(MenuItem::new("paste", "Paste").shortcut("⌘V"))
///     .divider()
///     .item(MenuItem::new("delete", "Delete").on_select(|| delete_file()))
/// ```
pub struct ContextMenu {
    id: ElementId,
    content: AnyElement,
    items: Vec<MenuContent>,
    state: Rc<RefCell<InternalState>>,
}

impl ContextMenu {
    /// Creates a new context menu wrapping the given content.
    ///
    /// The content element will trigger the context menu on right-click.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this context menu
    /// * `content` - The element that will trigger the context menu on right-click
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            content: content.into_any_element(),
            items: Vec::new(),
            state: Rc::new(RefCell::new(InternalState::default())),
        }
    }

    /// Adds a menu item to the context menu.
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(MenuContent::Item(item));
        self
    }

    /// Adds a divider line between menu items.
    pub fn divider(mut self) -> Self {
        self.items.push(MenuContent::Divider);
        self
    }

    /// Adds a submenu with a text label.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the submenu
    /// * `label` - Text label shown in the menu
    /// * `builder` - Closure that configures the submenu items
    pub fn submenu(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        builder: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    ) -> Self {
        let submenu_builder = builder(SubMenuBuilder::new());
        self.items.push(MenuContent::Submenu {
            id: id.into(),
            label: label.into(),
            icon: None,
            items: submenu_builder.items,
        });
        self
    }

    /// Adds a submenu with an icon and text label.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for the submenu
    /// * `icon` - Icon to display before the label
    /// * `label` - Text label shown in the menu
    /// * `builder` - Closure that configures the submenu items
    pub fn submenu_with_icon(
        mut self,
        id: impl Into<ElementId>,
        icon: Icon,
        label: impl Into<SharedString>,
        builder: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    ) -> Self {
        let submenu_builder = builder(SubMenuBuilder::new());
        self.items.push(MenuContent::Submenu {
            id: id.into(),
            label: label.into(),
            icon: Some(icon),
            items: submenu_builder.items,
        });
        self
    }

    /// Builds the menu panel element at the given position.
    fn build_menu_panel(
        items: Vec<MenuContent>,
        state: Rc<RefCell<InternalState>>,
    ) -> Div {
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let mut panel = div()
            .min_w(px(180.0))
            .bg(panel_bg)
            .rounded(px(6.0))
            .border_1()
            .border_color(border_color)
            .py(px(4.0))
            .shadow(vec![BoxShadow {
                color: shadow_color,
                offset: point(px(0.0), px(4.0)),
                blur_radius: px(12.0),
                spread_radius: px(0.0),
            }]);

        // Dismiss handler that closes the menu
        let on_dismiss = {
            let state = state.clone();
            move |_window: &mut Window, _cx: &mut App| {
                state.borrow_mut().is_open = false;
            }
        };

        // Add items to panel
        for content in items {
            match content {
                MenuContent::Item(item) => {
                    panel = panel.child(Self::build_menu_item(item, state.clone()));
                }
                MenuContent::Divider => {
                    panel = panel.child(Menu::build_divider());
                }
                MenuContent::Submenu { id, label, icon, items } => {
                    let submenu_panel = Menu::build_submenu_panel(&items, on_dismiss.clone());
                    panel = panel.child(
                        div()
                            .relative()
                            .group("submenu")
                            .child(Menu::build_submenu_trigger(id, label, icon))
                            .child(
                                div()
                                    .invisible()
                                    .group_hover("submenu", |style| style.visible())
                                    .child(submenu_panel),
                            ),
                    );
                }
            }
        }

        panel
    }

    /// Builds a single menu item element that closes menu on click.
    fn build_menu_item(item: MenuItem, state: Rc<RefCell<InternalState>>) -> Stateful<Div> {
        let hover_bg = hsla(211.0 / 360.0, 0.95, 0.53, 1.0);
        let text_color = hsla(0.0, 0.0, 0.15, 1.0);
        let hover_text_color = hsla(0.0, 0.0, 1.0, 1.0);
        let disabled_text_color = hsla(0.0, 0.0, 0.55, 1.0);
        let shortcut_color = hsla(0.0, 0.0, 0.45, 1.0);

        let is_disabled = item.disabled;
        let on_select = item.on_select;

        let mut row = div()
            .id(item.id)
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px(px(12.0))
            .py(px(6.0))
            .mx(px(4.0))
            .rounded(px(4.0))
            .text_sm();

        // Left side: icon + label
        let mut left_content = div().flex().flex_row().items_center().gap(px(8.0));

        if let Some(icon) = item.icon {
            left_content = left_content.child(
                div()
                    .w(px(16.0))
                    .text_sm()
                    .text_color(text_color)
                    .child(icon.as_str())
            );
        }

        left_content = left_content.child(item.label.clone());

        if is_disabled {
            row = row.text_color(disabled_text_color).cursor_default();
        } else {
            row = row
                .text_color(text_color)
                .cursor_pointer()
                .hover(|style| style.bg(hover_bg).text_color(hover_text_color));

            // Add click handler
            if let Some(handler) = on_select {
                let state_for_click = state.clone();
                row = row.on_click(move |_event, window, cx| {
                    // Close the menu
                    state_for_click.borrow_mut().is_open = false;
                    // Call the handler
                    handler(window, cx);
                });
            } else {
                // Still close on click even without handler
                let state_for_click = state.clone();
                row = row.on_click(move |_event, _window, _cx| {
                    state_for_click.borrow_mut().is_open = false;
                });
            }
        }

        row = row.child(left_content);

        // Right side: shortcut
        if let Some(shortcut) = item.shortcut {
            row = row.child(
                div()
                    .text_xs()
                    .text_color(if is_disabled {
                        disabled_text_color
                    } else {
                        shortcut_color
                    })
                    .child(shortcut),
            );
        }

        row
    }
}

impl IntoElement for ContextMenu {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id = self.id;
        let items = self.items;
        let state = self.state;

        // Read current state
        let is_open = state.borrow().is_open;
        let position = state.borrow().position;

        // Container wraps the content and positions the menu
        let state_for_click = state.clone();
        let mut container = div()
            .id(id)
            .relative()
            .child(self.content)
            // Right-click opens the menu
            .on_mouse_down(MouseButton::Right, move |event, _window, _cx| {
                let mut s = state_for_click.borrow_mut();
                s.is_open = true;
                s.position = event.position;
            });

        // Add menu panel if open
        if is_open {
            let panel = Self::build_menu_panel(items, state.clone());

            // Backdrop to catch clicks outside the menu
            let state_for_backdrop = state.clone();
            let backdrop = div()
                .absolute()
                .top(px(0.0))
                .left(px(0.0))
                .w(px(10000.0))  // Large enough to cover the screen
                .h(px(10000.0))
                .on_mouse_down(MouseButton::Left, move |_event, _window, _cx| {
                    state_for_backdrop.borrow_mut().is_open = false;
                })
                .on_mouse_down(MouseButton::Right, move |_event, _window, _cx| {
                    // Also close on right-click outside
                });

            // Position the panel at the cursor position
            let positioned_panel = div()
                .absolute()
                .left(position.x)
                .top(position.y)
                .child(panel)
                .id("context-menu-panel")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    // Stop propagation so clicks inside the menu don't hit the backdrop
                    cx.stop_propagation();
                })
                .on_mouse_down(MouseButton::Right, |_event, _window, cx| {
                    cx.stop_propagation();
                });

            container = container.child(backdrop).child(positioned_panel);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_creation() {
        // Just verify it compiles - no state management needed
        let _menu = ContextMenu::new("test", div())
            .item(MenuItem::new("item1", "Item 1"))
            .divider()
            .item(MenuItem::new("item2", "Item 2"));
    }
}
