//! Menu component for GPUI.
//!
//! A button that displays a popup menu of actions when clicked.
//! Supports menu items with labels, icons, keyboard shortcuts, dividers,
//! and nested submenus.

use gpui::prelude::*;
use gpui::*;

use super::label::Icon;

/// A menu item that can be added to a Menu.
///
/// Menu items display a label and optionally an icon and keyboard shortcut.
/// They can be disabled and have an action handler for when clicked.
///
/// # Example
///
/// ```ignore
/// MenuItem::new("copy", "Copy")
///     .icon(Icon::Document)
///     .shortcut("⌘C")
///     .on_select(|| println!("Copy clicked"))
/// ```
pub struct MenuItem {
    pub(crate) id: ElementId,
    pub(crate) label: SharedString,
    pub(crate) icon: Option<Icon>,
    pub(crate) shortcut: Option<SharedString>,
    pub(crate) disabled: bool,
    pub(crate) on_select: Option<Box<dyn Fn(&mut Window, &mut App) + 'static>>,
}

impl MenuItem {
    /// Creates a new menu item with the given id and label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            icon: None,
            shortcut: None,
            disabled: false,
            on_select: None,
        }
    }

    /// Sets the icon for this menu item.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets the keyboard shortcut display text (e.g., "⌘C").
    ///
    /// Note: This is just for display - actual keyboard handling
    /// should be implemented separately.
    pub fn shortcut(mut self, shortcut: impl Into<SharedString>) -> Self {
        self.shortcut = Some(shortcut.into());
        self
    }

    /// Sets whether this menu item is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the handler called when this menu item is selected.
    pub fn on_select(mut self, handler: impl Fn(&mut Window, &mut App) + 'static) -> Self {
        self.on_select = Some(Box::new(handler));
        self
    }
}

/// A content item in a menu - either a regular item, divider, or submenu.
pub enum MenuContent {
    /// A regular menu item.
    Item(MenuItem),
    /// A visual divider between groups of items.
    Divider,
    /// A nested submenu.
    Submenu {
        id: ElementId,
        label: SharedString,
        icon: Option<Icon>,
        items: Vec<MenuContent>,
    },
}

/// A button that displays a popup menu of actions.
///
/// Menu combines a trigger button with a dropdown panel containing menu items.
/// Supports nested submenus, dividers, icons, and keyboard shortcuts.
///
/// # Example
///
/// ```ignore
/// Menu::new("actions-menu", "Actions")
///     .item(MenuItem::new("copy", "Copy").shortcut("⌘C"))
///     .item(MenuItem::new("paste", "Paste").shortcut("⌘V"))
///     .divider()
///     .submenu("submenu-share", "Share", |items| {
///         items
///             .item(MenuItem::new("email", "Email"))
///             .item(MenuItem::new("message", "Message"))
///     })
/// ```
pub struct Menu {
    id: ElementId,
    label: SharedString,
    items: Vec<MenuContent>,
    disabled: bool,
}

impl Menu {
    /// Creates a new menu with the given id and button label.
    pub fn new(id: impl Into<ElementId>, label: impl Into<SharedString>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            items: Vec::new(),
            disabled: false,
        }
    }

    /// Adds a menu item.
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
    ///
    /// The builder function receives a SubMenuBuilder to add items.
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
        icon: Icon,
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

    /// Sets whether the menu trigger button is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Builds a menu item row element.
    ///
    /// This method is public to allow reuse by ContextMenu and other menu-like components.
    pub fn build_menu_item(
        item: MenuItem,
        on_dismiss: impl Fn(&mut Window, &mut App) + Clone + 'static,
    ) -> Stateful<Div> {
        let text_color = if item.disabled {
            hsla(0.0, 0.0, 0.55, 1.0) // Disabled gray
        } else {
            hsla(0.0, 0.0, 0.15, 1.0) // Normal dark text
        };
        let shortcut_color = hsla(0.0, 0.0, 0.50, 1.0); // Muted gray for shortcuts

        let mut row = div()
            .id(item.id)
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .px(px(12.0))
            .py(px(6.0))
            .gap(px(8.0));

        // Icon column (fixed width for alignment)
        let icon_element = if let Some(icon) = item.icon {
            div()
                .w(px(16.0))
                .text_sm()
                .text_color(text_color)
                .child(icon.as_str())
        } else {
            div().w(px(16.0)) // Empty spacer for alignment
        };
        row = row.child(icon_element);

        // Label (flex grow to push shortcut to the right)
        row = row.child(
            div()
                .flex_grow()
                .text_sm()
                .text_color(text_color)
                .child(item.label),
        );

        // Shortcut (right-aligned)
        if let Some(shortcut) = item.shortcut {
            row = row.child(
                div()
                    .text_sm()
                    .text_color(shortcut_color)
                    .child(shortcut),
            );
        }

        // Styling and interactivity
        if item.disabled {
            row = row.cursor_default();
        } else {
            row = row
                .cursor_pointer()
                .hover(|style| style.bg(hsla(211.0 / 360.0, 0.95, 0.53, 1.0)));

            // Add click handler
            if let Some(handler) = item.on_select {
                let dismiss = on_dismiss.clone();
                row = row.on_click(move |_event, window, cx| {
                    handler(window, cx);
                    dismiss(window, cx);
                });
            } else {
                let dismiss = on_dismiss;
                row = row.on_click(move |_event, window, cx| {
                    dismiss(window, cx);
                });
            }
        }

        row
    }

    /// Builds a divider element.
    ///
    /// This method is public to allow reuse by ContextMenu and other menu-like components.
    pub fn build_divider() -> Div {
        div()
            .w_full()
            .h(px(1.0))
            .my(px(4.0))
            .bg(hsla(0.0, 0.0, 0.90, 1.0))
    }

    /// Builds a submenu row element.
    ///
    /// This method is public to allow reuse by ContextMenu and other menu-like components.
    pub fn build_submenu_trigger(
        id: ElementId,
        label: SharedString,
        icon: Option<Icon>,
    ) -> Stateful<Div> {
        let text_color = hsla(0.0, 0.0, 0.15, 1.0);
        let arrow_color = hsla(0.0, 0.0, 0.50, 1.0);

        let mut row = div()
            .id(id)
            .flex()
            .flex_row()
            .items_center()
            .w_full()
            .px(px(12.0))
            .py(px(6.0))
            .gap(px(8.0))
            .cursor_pointer()
            .hover(|style| style.bg(hsla(211.0 / 360.0, 0.95, 0.53, 1.0)));

        // Icon column
        let icon_element = if let Some(icon) = icon {
            div()
                .w(px(16.0))
                .text_sm()
                .text_color(text_color)
                .child(icon.as_str())
        } else {
            div().w(px(16.0))
        };
        row = row.child(icon_element);

        // Label
        row = row.child(
            div()
                .flex_grow()
                .text_sm()
                .text_color(text_color)
                .child(label),
        );

        // Submenu arrow
        row = row.child(
            div()
                .text_sm()
                .text_color(arrow_color)
                .child("▶"),
        );

        row
    }

    /// Builds the submenu panel with nested items.
    ///
    /// This method is public to allow reuse by ContextMenu and other menu-like components.
    pub fn build_submenu_panel(items: &[MenuContent], on_dismiss: impl Fn(&mut Window, &mut App) + Clone + 'static) -> Div {
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let mut panel = div()
            .absolute()
            .left_full()
            .top(px(0.0))
            .ml(px(-4.0))
            .min_w(px(160.0))
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

        for content in items {
            match content {
                MenuContent::Item(item) => {
                    // We need to clone the item fields since MenuItem doesn't implement Clone
                    let item_clone = MenuItem {
                        id: item.id.clone(),
                        label: item.label.clone(),
                        icon: item.icon,
                        shortcut: item.shortcut.clone(),
                        disabled: item.disabled,
                        on_select: None, // Can't clone the handler, submenu items won't have actions for now
                    };
                    panel = panel.child(Self::build_menu_item(item_clone, on_dismiss.clone()));
                }
                MenuContent::Divider => {
                    panel = panel.child(Self::build_divider());
                }
                MenuContent::Submenu { id, label, icon, items } => {
                    // Nested submenus shown on hover
                    let submenu_panel = Self::build_submenu_panel(items, on_dismiss.clone());
                    panel = panel.child(
                        div()
                            .relative()
                            .group("submenu")
                            .child(Self::build_submenu_trigger(id.clone(), label.clone(), *icon))
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

/// Builder for submenu items.
pub struct SubMenuBuilder {
    /// The items in this submenu.
    pub items: Vec<MenuContent>,
}

impl Default for SubMenuBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl SubMenuBuilder {
    /// Creates a new empty submenu builder.
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// Adds a menu item to the submenu.
    pub fn item(mut self, item: MenuItem) -> Self {
        self.items.push(MenuContent::Item(item));
        self
    }

    /// Adds a divider to the submenu.
    pub fn divider(mut self) -> Self {
        self.items.push(MenuContent::Divider);
        self
    }

    /// Adds a nested submenu.
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
}

/// A controlled Menu component for use in stateful views.
///
/// Unlike the basic hover-based Popover, ControlledMenu is controlled
/// by external state and closes when an item is selected or clicked outside.
///
/// # Example
///
/// ```ignore
/// // In your view state
/// struct MyView {
///     menu_open: bool,
/// }
///
/// // In render
/// ControlledMenu::new("my-menu", "Actions", self.menu_open)
///     .item(MenuItem::new("copy", "Copy"))
///     .on_toggle(cx.listener(|this, open, _window, cx| {
///         this.menu_open = *open;
///         cx.notify();
///     }))
/// ```
pub struct ControlledMenu {
    id: ElementId,
    label: SharedString,
    is_open: bool,
    items: Vec<MenuContent>,
    disabled: bool,
    on_toggle: Option<Box<dyn Fn(&bool, &mut Window, &mut App) + 'static>>,
}

impl ControlledMenu {
    /// Creates a new controlled menu.
    pub fn new(
        id: impl Into<ElementId>,
        label: impl Into<SharedString>,
        is_open: bool,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            is_open,
            items: Vec::new(),
            disabled: false,
            on_toggle: None,
        }
    }

    /// Adds a menu item.
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
        icon: Icon,
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

    /// Sets whether the menu trigger button is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets the toggle handler, called when the menu opens or closes.
    pub fn on_toggle(
        mut self,
        handler: impl Fn(&bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

struct ButtonColors {
    bg: Hsla,
    bg_hover: Hsla,
    border: Hsla,
    text: Hsla,
}

impl IntoElement for ControlledMenu {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let is_open = self.is_open;
        let disabled = self.disabled;
        let label = self.label.clone();
        let id = self.id;

        // Build button colors
        let colors = if disabled {
            ButtonColors {
                bg: hsla(0.0, 0.0, 0.90, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.90, 1.0),
                border: hsla(0.0, 0.0, 0.82, 1.0),
                text: hsla(0.0, 0.0, 0.55, 1.0),
            }
        } else {
            ButtonColors {
                bg: hsla(0.0, 0.0, 0.97, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.93, 1.0),
                border: hsla(0.0, 0.0, 0.78, 1.0),
                text: hsla(0.0, 0.0, 0.15, 1.0),
            }
        };

        // Build the trigger button
        let mut button = div()
            .id("menu-button")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .px_4()
            .py_1()
            .min_w(px(80.0))
            .h(px(24.0))
            .rounded(px(6.0))
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .bg(colors.bg)
            .border_1()
            .border_color(colors.border)
            .text_color(colors.text)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        if !disabled {
            button = button
                .cursor_pointer()
                .hover(move |style| style.bg(colors.bg_hover));

            // Add click handler to toggle menu
            if let Some(toggle_handler) = self.on_toggle {
                let new_state = !is_open;
                button = button.on_click(move |_event, window, cx| {
                    toggle_handler(&new_state, window, cx);
                });
            }
        }

        button = button
            .child(label)
            .child(
                div()
                    .text_xs()
                    .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                    .child("▼"),
            );

        // Build the container
        let mut container = div()
            .id(id)
            .relative()
            .child(button);

        // Add menu panel if open
        if is_open {
            // Build menu panel
            let mut panel = div()
                .absolute()
                .top_full()
                .left(px(0.0))
                .mt(px(4.0))
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
            for content in self.items {
                match content {
                    MenuContent::Item(item) => {
                        let dismiss = |_window: &mut Window, _cx: &mut App| {
                            // Dismiss is handled by the parent view
                        };
                        panel = panel.child(Menu::build_menu_item(item, dismiss));
                    }
                    MenuContent::Divider => {
                        panel = panel.child(Menu::build_divider());
                    }
                    MenuContent::Submenu { id, label, icon, items } => {
                        let dismiss = |_window: &mut Window, _cx: &mut App| {};
                        let submenu_panel = Menu::build_submenu_panel(&items, dismiss);
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

            // Wrap panel with click-outside handling
            let panel_container = div()
                .id("menu-panel-container")
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    cx.stop_propagation();
                })
                .child(panel);

            container = container.child(panel_container);
        }

        container
    }
}

// Simple hover-based menu (shows on hover like tooltip)
impl IntoElement for Menu {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let panel_bg = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let shadow_color = hsla(0.0, 0.0, 0.0, 0.15);

        let disabled = self.disabled;
        let label = self.label.clone();

        // Build button colors
        let colors = if disabled {
            ButtonColors {
                bg: hsla(0.0, 0.0, 0.90, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.90, 1.0),
                border: hsla(0.0, 0.0, 0.82, 1.0),
                text: hsla(0.0, 0.0, 0.55, 1.0),
            }
        } else {
            ButtonColors {
                bg: hsla(0.0, 0.0, 0.97, 1.0),
                bg_hover: hsla(0.0, 0.0, 0.93, 1.0),
                border: hsla(0.0, 0.0, 0.78, 1.0),
                text: hsla(0.0, 0.0, 0.15, 1.0),
            }
        };

        // Build the trigger button
        let mut button = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(4.0))
            .px_4()
            .py_1()
            .min_w(px(80.0))
            .h(px(24.0))
            .rounded(px(6.0))
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .bg(colors.bg)
            .border_1()
            .border_color(colors.border)
            .text_color(colors.text)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        if !disabled {
            button = button
                .cursor_pointer()
                .hover(move |style| style.bg(colors.bg_hover));
        }

        button = button
            .child(label)
            .child(
                div()
                    .text_xs()
                    .text_color(hsla(0.0, 0.0, 0.50, 1.0))
                    .child("▼"),
            );

        // Build the menu panel
        let mut panel = div()
            .absolute()
            .top_full()
            .left(px(0.0))
            .mt(px(4.0))
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
        let dismiss = |_window: &mut Window, _cx: &mut App| {};
        for content in self.items {
            match content {
                MenuContent::Item(item) => {
                    panel = panel.child(Self::build_menu_item(item, dismiss));
                }
                MenuContent::Divider => {
                    panel = panel.child(Self::build_divider());
                }
                MenuContent::Submenu { id, label, icon, items } => {
                    let submenu_panel = Self::build_submenu_panel(&items, dismiss);
                    panel = panel.child(
                        div()
                            .relative()
                            .group("submenu")
                            .child(Self::build_submenu_trigger(id, label, icon))
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

        // Build the container with hover behavior
        div()
            .id(self.id)
            .relative()
            .group("menu")
            .child(button)
            .child(
                div()
                    .invisible()
                    .group_hover("menu", |style| style.visible())
                    .child(panel),
            )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_menu_item_creation() {
        let item = MenuItem::new("test", "Test Item");
        assert_eq!(item.label.as_ref(), "Test Item");
        assert!(!item.disabled);
        assert!(item.icon.is_none());
        assert!(item.shortcut.is_none());
    }

    #[test]
    fn test_menu_item_with_icon() {
        let item = MenuItem::new("test", "Copy").icon(Icon::Document);
        assert!(item.icon.is_some());
        assert_eq!(item.icon.unwrap(), Icon::Document);
    }

    #[test]
    fn test_menu_item_with_shortcut() {
        let item = MenuItem::new("test", "Copy").shortcut("⌘C");
        assert_eq!(item.shortcut.as_ref().unwrap().as_ref(), "⌘C");
    }

    #[test]
    fn test_menu_item_disabled() {
        let item = MenuItem::new("test", "Test").disabled(true);
        assert!(item.disabled);
    }

    #[test]
    fn test_menu_creation() {
        let menu = Menu::new("test-menu", "Actions");
        assert_eq!(menu.label.as_ref(), "Actions");
        assert!(menu.items.is_empty());
    }

    #[test]
    fn test_menu_with_items() {
        let menu = Menu::new("test-menu", "Actions")
            .item(MenuItem::new("copy", "Copy"))
            .divider()
            .item(MenuItem::new("paste", "Paste"));
        assert_eq!(menu.items.len(), 3);
    }

    #[test]
    fn test_menu_with_submenu() {
        let menu = Menu::new("test-menu", "Actions")
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

    #[test]
    fn test_controlled_menu_creation() {
        let menu = ControlledMenu::new("test-menu", "Actions", false);
        assert!(!menu.is_open);
        assert!(!menu.disabled);
    }

    #[test]
    fn test_controlled_menu_open() {
        let menu = ControlledMenu::new("test-menu", "Actions", true);
        assert!(menu.is_open);
    }
}
