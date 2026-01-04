//! Context menu component for GPUI.
//!
//! A right-click context menu that appears at the cursor location.
//! Reuses menu item infrastructure from the Menu component.
//!
//! # Usage
//!
//! The ContextMenu is designed to be used with external state management.
//! You provide a `ContextMenuState` that tracks whether the menu is open
//! and its position, then use callbacks to update that state.
//!
//! ```ignore
//! struct MyView {
//!     context_menu_state: ContextMenuState,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         ContextMenu::new("my-context-menu", div().child("Right-click me"))
//!             .state(self.context_menu_state.clone())
//!             .on_open(cx.listener(|this, pos, _window, cx| {
//!                 this.context_menu_state.open(*pos);
//!                 cx.notify();
//!             }))
//!             .item(MenuItem::new("copy", "Copy")
//!                 .on_select(|| {
//!                     // Handle copy action
//!                 }))
//!             .item(MenuItem::new("delete", "Delete"))
//!     }
//! }
//! ```

use gpui::prelude::*;
use gpui::*;

use super::menu::{Menu, MenuContent, MenuItem, SubMenuBuilder};

/// State for a controlled context menu.
///
/// This struct tracks whether the context menu is open and where it should appear.
/// Use this with `ContextMenu` for stateful views that need to control
/// the context menu explicitly.
///
/// # Example
///
/// ```ignore
/// struct MyView {
///     context_menu: ContextMenuState,
/// }
///
/// impl MyView {
///     fn new() -> Self {
///         Self {
///             context_menu: ContextMenuState::new(),
///         }
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct ContextMenuState {
    /// Whether the context menu is currently visible.
    pub is_open: bool,
    /// The position where the context menu should appear (cursor position).
    pub position: Point<Pixels>,
}

impl Default for ContextMenuState {
    fn default() -> Self {
        Self::new()
    }
}

impl ContextMenuState {
    /// Creates a new closed context menu state.
    pub fn new() -> Self {
        Self {
            is_open: false,
            position: Point::default(),
        }
    }

    /// Opens the context menu at the specified position.
    pub fn open(&mut self, position: Point<Pixels>) {
        self.is_open = true;
        self.position = position;
    }

    /// Closes the context menu.
    pub fn close(&mut self) {
        self.is_open = false;
    }

    /// Toggles the context menu state at the specified position.
    pub fn toggle(&mut self, position: Point<Pixels>) {
        if self.is_open {
            self.close();
        } else {
            self.open(position);
        }
    }
}

/// A context menu that wraps content and shows on right-click.
///
/// ContextMenu wraps any content and displays a popup menu when the user
/// right-clicks (secondary click) on that content. The menu appears at the
/// cursor location and reuses the same menu item infrastructure as the Menu component.
///
/// # Example
///
/// ```ignore
/// ContextMenu::new("file-context", file_item)
///     .state(context_menu_state)
///     .on_open(cx.listener(|this, pos, _window, cx| {
///         this.context_menu_state.open(*pos);
///         cx.notify();
///     }))
///     .item(MenuItem::new("copy", "Copy").shortcut("⌘C"))
///     .item(MenuItem::new("paste", "Paste").shortcut("⌘V"))
///     .divider()
///     .item(MenuItem::new("delete", "Delete").on_select(|| delete_file()))
/// ```
pub struct ContextMenu {
    id: ElementId,
    content: AnyElement,
    items: Vec<MenuContent>,
    state: ContextMenuState,
    on_open: Option<Box<dyn Fn(&Point<Pixels>, &mut Window, &mut App) + 'static>>,
}

impl ContextMenu {
    /// Creates a new context menu wrapping the given content.
    ///
    /// # Arguments
    /// * `id` - Unique identifier for this context menu
    /// * `content` - The element that will trigger the context menu on right-click
    pub fn new(id: impl Into<ElementId>, content: impl IntoElement) -> Self {
        Self {
            id: id.into(),
            content: content.into_any_element(),
            items: Vec::new(),
            state: ContextMenuState::new(),
            on_open: None,
        }
    }

    /// Sets the context menu state.
    ///
    /// Use this to control whether the menu is visible and its position.
    pub fn state(mut self, state: ContextMenuState) -> Self {
        self.state = state;
        self
    }

    /// Adds a menu item to the context menu.
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(MenuContent::Item(item));
        self
    }

    /// Adds a divider between menu items.
    pub fn divider(mut self) -> Self {
        self.items.push(MenuContent::Divider);
        self
    }

    /// Adds a submenu with the given label and items.
    pub fn submenu(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        builder: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    ) -> Self {
        let sub = builder(SubMenuBuilder::new());
        self.items.push(MenuContent::Submenu {
            id: id.into(),
            label: label.into(),
            icon: None,
            items: sub.items,
        });
        self
    }

    /// Adds a submenu with an icon.
    pub fn submenu_with_icon(
        mut self,
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        icon: super::label::Icon,
        builder: impl FnOnce(SubMenuBuilder) -> SubMenuBuilder,
    ) -> Self {
        let sub = builder(SubMenuBuilder::new());
        self.items.push(MenuContent::Submenu {
            id: id.into(),
            label: label.into(),
            icon: Some(icon),
            items: sub.items,
        });
        self
    }

    /// Sets a handler called when the context menu opens (on right-click).
    ///
    /// The handler receives the cursor position where the menu will appear.
    /// Use this to update your state to show the menu at the given position.
    ///
    /// # Example
    ///
    /// ```ignore
    /// .on_open(cx.listener(|this, pos, _window, cx| {
    ///     this.context_menu_state.open(*pos);
    ///     cx.notify();
    /// }))
    /// ```
    pub fn on_open(
        mut self,
        handler: impl Fn(&Point<Pixels>, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_open = Some(Box::new(handler));
        self
    }

    /// Builds the menu panel element at the given position.
    fn build_menu_panel(items: Vec<MenuContent>, on_dismiss: impl Fn(&mut Window, &mut App) + Clone + 'static) -> Div {
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
                    panel = panel.child(Menu::build_menu_item(item, on_dismiss.clone()));
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
}

impl IntoElement for ContextMenu {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let id = self.id;
        let is_open = self.state.is_open;
        let position = self.state.position;
        let items = self.items;
        let on_open = self.on_open;

        // Container wraps the content and positions the menu
        let mut container = div()
            .id(id)
            .relative()
            .child(self.content);

        // Add right-click handler if we have an on_open handler
        if let Some(open_handler) = on_open {
            container = container.on_mouse_down(MouseButton::Right, move |event, window, cx| {
                let pos = event.position;
                open_handler(&pos, window, cx);
            });
        }

        // Add menu panel if open
        if is_open {
            // Create a simple dismiss handler
            // Dismissal is handled by the parent view responding to item selection
            let on_dismiss = |_window: &mut Window, _cx: &mut App| {
                // No-op: parent view manages menu state via item on_select callbacks
            };

            let panel = Self::build_menu_panel(items, on_dismiss);

            // Position the panel at the cursor position
            let positioned_panel = div()
                .absolute()
                .left(position.x)
                .top(position.y)
                .child(panel)
                .id("context-menu-panel")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    // Stop propagation so clicks inside the menu don't dismiss it
                    cx.stop_propagation();
                });

            container = container.child(positioned_panel);
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_context_menu_state_new() {
        let state = ContextMenuState::new();
        assert!(!state.is_open);
        assert_eq!(state.position.x, px(0.0));
        assert_eq!(state.position.y, px(0.0));
    }

    #[test]
    fn test_context_menu_state_open() {
        let mut state = ContextMenuState::new();
        state.open(Point { x: px(100.0), y: px(200.0) });
        assert!(state.is_open);
        assert_eq!(state.position.x, px(100.0));
        assert_eq!(state.position.y, px(200.0));
    }

    #[test]
    fn test_context_menu_state_close() {
        let mut state = ContextMenuState::new();
        state.open(Point { x: px(100.0), y: px(200.0) });
        state.close();
        assert!(!state.is_open);
    }

    #[test]
    fn test_context_menu_state_toggle() {
        let mut state = ContextMenuState::new();
        let pos = Point { x: px(50.0), y: px(75.0) };

        state.toggle(pos);
        assert!(state.is_open);
        assert_eq!(state.position, pos);

        state.toggle(pos);
        assert!(!state.is_open);
    }

    #[test]
    fn test_context_menu_creation() {
        let menu = ContextMenu::new("test-menu", div().child("Test content"));
        assert!(menu.items.is_empty());
        assert!(!menu.state.is_open);
    }

    #[test]
    fn test_context_menu_with_items() {
        let menu = ContextMenu::new("test-menu", div().child("Test content"))
            .item(MenuItem::new("copy", "Copy"))
            .divider()
            .item(MenuItem::new("paste", "Paste"));
        assert_eq!(menu.items.len(), 3);
    }

    #[test]
    fn test_context_menu_with_state() {
        let mut state = ContextMenuState::new();
        state.open(Point { x: px(100.0), y: px(100.0) });

        let menu = ContextMenu::new("test-menu", div().child("Test"))
            .state(state);
        assert!(menu.state.is_open);
    }

    #[test]
    fn test_context_menu_with_submenu() {
        let menu = ContextMenu::new("test-menu", div().child("Test"))
            .submenu("share", "Share", |sub| {
                sub.item(MenuItem::new("email", "Email"))
                    .item(MenuItem::new("message", "Message"))
            });
        assert_eq!(menu.items.len(), 1);
        match &menu.items[0] {
            MenuContent::Submenu { items, .. } => {
                assert_eq!(items.len(), 2);
            }
            _ => panic!("Expected submenu"),
        }
    }
}
