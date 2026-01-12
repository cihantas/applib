//! Context menu component for GPUI.
//!
//! A right-click context menu that appears at the cursor location.
//! Uses a controlled component pattern - the caller manages open state.
//!
//! # Usage
//!
//! The caller manages the context menu state through `Option<Point<Pixels>>`:
//!
//! ```ignore
//! // In your view state:
//! struct MyView {
//!     context_menu_state: Option<Point<Pixels>>,  // None = closed, Some(pos) = open at pos
//! }
//!
//! // In render:
//! ContextMenu::new("my-context-menu", div().child("Right-click me"))
//!     .state(self.context_menu_state)  // Pass the current state
//!     .on_toggle(cx.listener(|this, pos, _window, cx| {
//!         this.context_menu_state = pos;  // pos is Some(Point) to open, None to close
//!         cx.notify();
//!     }))
//!     .item(MenuItem::new("copy", "Copy").on_select(|| handle_copy()))
//!     .item(MenuItem::new("delete", "Delete").on_select(|| handle_delete()))
//! ```
//!
//! The menu:
//! - Opens on right-click at the cursor position (calls on_toggle with Some(position))
//! - Closes when clicking outside or selecting an item (calls on_toggle with None)
//! - Handles keyboard navigation (planned)

use gpui::prelude::*;
use gpui::*;
use std::rc::Rc;

use super::label::Icon;
use super::menu::{Menu, MenuContent, MenuItem, SubMenuBuilder};

/// A context menu that appears on right-click.
///
/// This is a controlled component - the caller manages the open/close state.
///
/// # Example
///
/// ```ignore
/// // View state stores context menu position (None = closed)
/// struct MyView {
///     menu_position: Option<Point<Pixels>>,
/// }
///
/// // In render:
/// ContextMenu::new("file-menu", file_item)
///     .state(self.menu_position)
///     .on_toggle(cx.listener(|this, pos, _window, cx| {
///         this.menu_position = pos;
///         cx.notify();
///     }))
///     .item(MenuItem::new("open", "Open"))
///     .item(MenuItem::new("delete", "Delete").on_select(|| delete_file()))
///
/// // With submenu
/// ContextMenu::new("edit-menu", text_field)
///     .state(self.menu_position)
///     .on_toggle(cx.listener(|this, pos, _window, cx| {
///         this.menu_position = pos;
///         cx.notify();
///     }))
///     .item(MenuItem::new("cut", "Cut").shortcut("⌘X"))
///     .item(MenuItem::new("copy", "Copy").shortcut("⌘C"))
///     .item(MenuItem::new("paste", "Paste").shortcut("⌘V"))
///     .divider()
///     .item(MenuItem::new("delete", "Delete"))
/// ```
pub struct ContextMenu {
    id: ElementId,
    content: AnyElement,
    items: Vec<MenuContent>,
    /// Current menu state: None = closed, Some(position) = open at position
    state: Option<Point<Pixels>>,
    /// Callback when menu should open or close
    on_toggle: Option<Rc<dyn Fn(Option<Point<Pixels>>, &mut Window, &mut App) + 'static>>,
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
            state: None,
            on_toggle: None,
        }
    }

    /// Sets the menu state: None = closed, Some(position) = open at position.
    pub fn state(mut self, state: Option<Point<Pixels>>) -> Self {
        self.state = state;
        self
    }

    /// Sets the toggle handler called when the menu should open or close.
    ///
    /// The handler receives:
    /// - `Some(position)` when the menu should open at the given cursor position
    /// - `None` when the menu should close
    pub fn on_toggle(
        mut self,
        handler: impl Fn(Option<Point<Pixels>>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
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
        on_close: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
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

        // Add items to panel
        for content in items {
            match content {
                MenuContent::Item(item) => {
                    let on_close_clone = on_close.clone();
                    panel = panel.child(Self::build_menu_item(item, on_close_clone));
                }
                MenuContent::Divider => {
                    panel = panel.child(Menu::build_divider());
                }
                MenuContent::Submenu { id, label, icon, items } => {
                    let on_close_clone = on_close.clone();
                    let on_dismiss = move |window: &mut Window, cx: &mut App| {
                        on_close_clone(window, cx);
                    };
                    let submenu_panel = Menu::build_submenu_panel(&items, on_dismiss);
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
    fn build_menu_item(
        item: MenuItem,
        on_close: Rc<dyn Fn(&mut Window, &mut App) + 'static>,
    ) -> Stateful<Div> {
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
                let on_close_for_click = on_close.clone();
                row = row.on_click(move |_event, window, cx| {
                    // Close the menu first
                    on_close_for_click(window, cx);
                    // Call the handler
                    handler(window, cx);
                });
            } else {
                // Still close on click even without handler
                let on_close_for_click = on_close.clone();
                row = row.on_click(move |_event, window, cx| {
                    on_close_for_click(window, cx);
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
        let on_toggle = self.on_toggle;

        // Container wraps the content and positions the menu
        let mut container = div()
            .id(id)
            .relative()
            .child(self.content);

        // Add right-click handler if we have a toggle callback
        if let Some(ref toggle) = on_toggle {
            let toggle_for_open = toggle.clone();
            container = container.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                toggle_for_open(Some(event.position), window, cx);
            });
        }

        // Add menu panel if open
        if let Some(_position) = state {
            // Create close handler
            let on_close: Rc<dyn Fn(&mut Window, &mut App) + 'static> = if let Some(ref toggle) = on_toggle {
                let toggle_for_close = toggle.clone();
                Rc::new(move |window, cx| {
                    toggle_for_close(None, window, cx);
                })
            } else {
                Rc::new(|_window, _cx| {})
            };

            let on_close_for_backdrop = on_close.clone();
            let panel = Self::build_menu_panel(items, on_close);

            // Backdrop to catch clicks outside the menu
            let backdrop = div()
                .absolute()
                .top(px(0.0))
                .left(px(0.0))
                .w(px(10000.0))  // Large enough to cover the screen
                .h(px(10000.0))
                .on_mouse_down(MouseButton::Left, move |_event, window, cx| {
                    on_close_for_backdrop(window, cx);
                })
                .on_mouse_down(MouseButton::Right, {
                    let toggle = on_toggle.clone();
                    move |event, window, cx| {
                        // Close current menu, open at new position
                        if let Some(ref toggle) = toggle {
                            toggle(Some(event.position), window, cx);
                        }
                    }
                });

            // Position the panel just below and to the right of the element
            // Note: We can't use the stored position directly as it's in window coordinates
            // and we're positioning relative to the container. Instead, we position at the
            // bottom-left of the triggering element.
            let positioned_panel = div()
                .absolute()
                .left(px(0.0))
                .top_full()  // Position at 100% of parent height (just below the element)
                .child(panel)
                .id("context-menu-panel")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    // Stop propagation so clicks inside the menu don't hit the backdrop
                    cx.stop_propagation();
                })
                .on_mouse_down(MouseButton::Right, |_event, _window, cx| {
                    cx.stop_propagation();
                });

            // Use deferred drawing with high priority to ensure menu appears above all other content
            container = container
                .child(deferred(backdrop).with_priority(999))
                .child(deferred(positioned_panel).with_priority(1000));
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_creation() {
        // Just verify it compiles
        let _menu = ContextMenu::new("test", div())
            .item(MenuItem::new("item1", "Item 1"))
            .divider()
            .item(MenuItem::new("item2", "Item 2"));
    }
}
