//! TabView component for switching between child views using tabs.
//!
//! This module provides a tab view component that switches between
//! child views using a tab bar.

use gpui::prelude::*;
use gpui::*;
use std::rc::Rc;

use super::label::Icon;
use super::Badge;

/// A single tab in a TabView.
///
/// Each tab has a label and can optionally include an icon and/or badge count.
/// The tab wraps a content element that will be displayed when the tab is selected.
///
/// # Example
///
/// ```ignore
/// Tab::new("First", first_content)
///     .icon(Icon::Folder)
///     .badge(5)
/// ```
pub struct Tab {
    label: SharedString,
    icon: Option<Icon>,
    badge: Option<u32>,
    content: AnyElement,
}

impl Tab {
    /// Creates a new tab with the given label and content.
    pub fn new(label: impl Into<SharedString>, content: impl IntoElement) -> Self {
        Self {
            label: label.into(),
            icon: None,
            badge: None,
            content: content.into_any_element(),
        }
    }

    /// Sets the icon for this tab.
    pub fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Sets a badge count for this tab.
    pub fn badge(mut self, count: u32) -> Self {
        self.badge = Some(count);
        self
    }
}

/// A tab view component.
///
/// TabView provides a tab bar with selectable tabs and a content area that displays
/// the currently selected tab's view. It supports icons, badges, and keyboard navigation.
///
/// # Example
///
/// ```ignore
/// TabView::new("main-tabs", selected_index)
///     .tab(Tab::new("First", first_view).icon(Icon::Folder))
///     .tab(Tab::new("Second", second_view).icon(Icon::Document).badge(3))
///     .on_selection_change(cx.listener(|this, index, _window, cx| {
///         this.selected_tab = index;
///         cx.notify();
///     }))
/// ```
pub struct TabView {
    id: ElementId,
    tabs: Vec<Tab>,
    selected_index: usize,
    on_selection_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
}

impl TabView {
    /// Creates a new tab view with the given id and selected index.
    pub fn new(id: impl Into<ElementId>, selected_index: usize) -> Self {
        Self {
            id: id.into(),
            tabs: Vec::new(),
            selected_index,
            on_selection_change: None,
        }
    }

    /// Adds a tab to the tab view.
    pub fn tab(mut self, tab: Tab) -> Self {
        self.tabs.push(tab);
        self
    }

    /// Sets the selection change handler.
    ///
    /// The handler receives the newly selected index.
    pub fn on_selection_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_selection_change = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for TabView {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let selected_index = self.selected_index;
        let on_selection_change = self.on_selection_change;
        let tab_count = self.tabs.len();

        // Colors
        let tab_bar_bg = hsla(0.0, 0.0, 0.93, 1.0); // Light gray tab bar
        let tab_bar_border = hsla(0.0, 0.0, 0.80, 1.0); // Border color
        let selected_tab_bg = hsla(0.0, 0.0, 1.0, 1.0); // White for selected
        let unselected_tab_bg = hsla(0.0, 0.0, 0.93, 1.0); // Same as bar for unselected
        let hover_tab_bg = hsla(0.0, 0.0, 0.97, 1.0); // Slightly lighter on hover
        let text_color = hsla(0.0, 0.0, 0.20, 1.0); // Dark gray text
        let selected_text_color = hsla(0.0, 0.0, 0.10, 1.0); // Darker text when selected
        let icon_color = hsla(0.0, 0.0, 0.40, 1.0); // Gray icons
        let selected_icon_color = hsla(211.0 / 360.0, 0.70, 0.45, 1.0); // Blue icons when selected

        // Split tabs into tab items (for the bar) and content
        let mut tab_items: Vec<(SharedString, Option<Icon>, Option<u32>)> = Vec::new();
        let mut contents: Vec<AnyElement> = Vec::new();

        for tab in self.tabs {
            tab_items.push((tab.label, tab.icon, tab.badge));
            contents.push(tab.content);
        }

        // Build the tab bar
        let mut tab_bar = div()
            .flex()
            .flex_row()
            .items_center()
            .bg(tab_bar_bg)
            .border_b_1()
            .border_color(tab_bar_border)
            .px(px(4.0))
            .py(px(4.0))
            .gap(px(2.0));

        // Add individual tab buttons
        for (index, (label, icon, badge)) in tab_items.into_iter().enumerate() {
            let is_selected = index == selected_index;

            // Build tab button content
            let mut tab_content = div()
                .flex()
                .flex_row()
                .items_center()
                .gap(px(4.0));

            // Add icon if present
            if let Some(icon) = icon {
                let icon_clr = if is_selected {
                    selected_icon_color
                } else {
                    icon_color
                };
                tab_content = tab_content.child(
                    div()
                        .text_sm()
                        .text_color(icon_clr)
                        .child(icon.as_str()),
                );
            }

            // Add label
            let txt_color = if is_selected {
                selected_text_color
            } else {
                text_color
            };
            tab_content = tab_content.child(
                div()
                    .text_sm()
                    .font_weight(if is_selected {
                        FontWeight::MEDIUM
                    } else {
                        FontWeight::NORMAL
                    })
                    .text_color(txt_color)
                    .child(label),
            );

            // Add badge if present
            if let Some(count) = badge {
                tab_content = tab_content.child(Badge::new(count.to_string()));
            }

            // Build tab button
            let mut tab_button = div()
                .id(("tab", index))
                .flex()
                .items_center()
                .justify_center()
                .px(px(12.0))
                .py(px(4.0))
                .rounded(px(4.0))
                .cursor_pointer();

            // Apply selected/unselected styling
            tab_button = if is_selected {
                tab_button
                    .bg(selected_tab_bg)
                    .shadow(vec![BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.08),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(2.0),
                        spread_radius: px(0.0),
                    }])
            } else {
                tab_button
                    .bg(unselected_tab_bg)
                    .hover(move |style| style.bg(hover_tab_bg))
            };

            tab_button = tab_button.child(tab_content);

            // Add click handler
            if let Some(ref handler) = on_selection_change {
                let handler = handler.clone();
                tab_button = tab_button.on_click(move |_event, window, cx| {
                    handler(index, window, cx);
                });
            }

            tab_bar = tab_bar.child(tab_button);
        }

        // Build content area - only render the selected tab's content (lazy rendering)
        let content_area = div()
            .flex_1()
            .size_full()
            .overflow_hidden();

        // Get the selected content
        let content_area = if selected_index < contents.len() {
            // We need to consume contents, so we iterate with into_iter and enumerate
            let mut selected_content: Option<AnyElement> = None;
            for (index, content) in contents.into_iter().enumerate() {
                if index == selected_index {
                    selected_content = Some(content);
                    break;
                }
            }
            if let Some(content) = selected_content {
                content_area.child(content)
            } else {
                content_area
            }
        } else {
            content_area
        };

        // Build main container with keyboard navigation
        div()
            .id(self.id)
            .flex()
            .flex_col()
            .size_full()
            .child(tab_bar)
            .child(content_area)
            .on_key_down({
                let on_selection_change = on_selection_change.clone();
                move |event, window, cx| {
                    if let Some(ref handler) = on_selection_change {
                        match event.keystroke.key.as_str() {
                            "left" => {
                                if selected_index > 0 {
                                    handler(selected_index - 1, window, cx);
                                }
                            }
                            "right" => {
                                if selected_index + 1 < tab_count {
                                    handler(selected_index + 1, window, cx);
                                }
                            }
                            _ => {}
                        }
                    }
                }
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tab_creation() {
        let tab = Tab::new("Test", div());
        assert_eq!(tab.label.as_ref(), "Test");
        assert!(tab.icon.is_none());
        assert!(tab.badge.is_none());
    }

    #[test]
    fn test_tab_with_icon() {
        let tab = Tab::new("Files", div()).icon(Icon::Folder);
        assert!(tab.icon.is_some());
        assert_eq!(tab.icon.unwrap(), Icon::Folder);
    }

    #[test]
    fn test_tab_with_badge() {
        let tab = Tab::new("Inbox", div()).badge(5);
        assert!(tab.badge.is_some());
        assert_eq!(tab.badge.unwrap(), 5);
    }

    #[test]
    fn test_tabview_creation() {
        let tabview = TabView::new("test-tabs", 0);
        assert!(tabview.tabs.is_empty());
        assert_eq!(tabview.selected_index, 0);
    }

    #[test]
    fn test_tabview_with_tabs() {
        let tabview = TabView::new("test-tabs", 1)
            .tab(Tab::new("First", div()))
            .tab(Tab::new("Second", div()))
            .tab(Tab::new("Third", div()));
        assert_eq!(tabview.tabs.len(), 3);
        assert_eq!(tabview.selected_index, 1);
    }

    #[test]
    fn test_tabview_tab_labels() {
        let tabview = TabView::new("test-tabs", 0)
            .tab(Tab::new("Documents", div()).icon(Icon::Document))
            .tab(Tab::new("Downloads", div()).badge(3));
        assert_eq!(tabview.tabs[0].label.as_ref(), "Documents");
        assert_eq!(tabview.tabs[1].label.as_ref(), "Downloads");
    }
}
