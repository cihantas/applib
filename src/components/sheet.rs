//! Modal sheet (dialog) component.
//!
//! A modal overlay that displays content on top of the current view with
//! a semi-transparent backdrop.

use gpui::prelude::*;
use gpui::*;

/// A modal sheet (dialog) component.
///
/// Sheet displays content in a centered panel with a semi-transparent backdrop.
/// It's commonly used for dialogs, forms, and confirmation prompts.
///
/// # Example
///
/// ```ignore
/// Sheet::new("create-branch-sheet")
///     .title("Create New Branch")
///     .child(form_content)
///     .actions(button_row)
///     .on_dismiss(cx.listener(|this, _event, _window, cx| {
///         this.show_sheet = false;
///         cx.notify();
///     }))
/// ```
pub struct Sheet {
    id: ElementId,
    title: Option<SharedString>,
    width: Pixels,
    children: Vec<AnyElement>,
    actions: Option<AnyElement>,
    on_dismiss: Option<Box<dyn Fn(&ClickEvent, &mut Window, &mut App) + 'static>>,
}

impl Sheet {
    /// Creates a new sheet with the given id.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            title: None,
            width: px(400.0),
            children: Vec::new(),
            actions: None,
            on_dismiss: None,
        }
    }

    /// Sets the title displayed at the top of the sheet.
    pub fn title(mut self, title: impl Into<SharedString>) -> Self {
        self.title = Some(title.into());
        self
    }

    /// Sets the width of the sheet panel.
    ///
    /// Defaults to 400px.
    pub fn width(mut self, width: impl Into<Pixels>) -> Self {
        self.width = width.into();
        self
    }

    /// Adds a child element to the sheet content area.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the sheet content area.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    /// Sets the actions element (typically a row of buttons) at the bottom.
    pub fn actions(mut self, actions: impl IntoElement) -> Self {
        self.actions = Some(actions.into_any_element());
        self
    }

    /// Sets the dismiss handler, called when the backdrop is clicked.
    pub fn on_dismiss(
        mut self,
        handler: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_dismiss = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Sheet {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        // Colors
        let backdrop_color = hsla(0.0, 0.0, 0.0, 0.4);
        let panel_bg = hsla(0.0, 0.0, 0.97, 1.0);
        let title_color = hsla(0.0, 0.0, 0.15, 1.0);
        let border_color = hsla(0.0, 0.0, 0.78, 1.0);
        let separator_color = hsla(0.0, 0.0, 0.88, 1.0);

        // Build the title bar if title is provided
        let title_bar = self.title.map(|title| {
            div()
                .flex()
                .items_center()
                .justify_center()
                .px(px(20.0))
                .py(px(16.0))
                .border_b_1()
                .border_color(separator_color)
                .child(
                    div()
                        .text_base()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(title_color)
                        .child(title),
                )
        });

        // Build content area
        let mut content = div().flex().flex_col().px(px(20.0)).py(px(16.0));
        for child in self.children {
            content = content.child(child);
        }

        // Build actions bar if provided
        let actions_bar = self.actions.map(|actions| {
            div()
                .flex()
                .flex_row()
                .justify_end()
                .gap(px(8.0))
                .px(px(20.0))
                .py(px(16.0))
                .border_t_1()
                .border_color(separator_color)
                .bg(hsla(0.0, 0.0, 0.95, 1.0))
                .rounded_b(px(10.0))
                .child(actions)
        });

        // Build the dialog panel
        let mut panel = div()
            .flex()
            .flex_col()
            .w(self.width)
            .max_h(px(600.0))
            .bg(panel_bg)
            .rounded(px(10.0))
            .border_1()
            .border_color(border_color)
            .shadow(vec![
                // Outer shadow for depth
                BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.25),
                    offset: point(px(0.0), px(8.0)),
                    blur_radius: px(24.0),
                    spread_radius: px(0.0),
                },
                // Inner highlight at top
                BoxShadow {
                    color: hsla(0.0, 0.0, 1.0, 0.5),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(0.0),
                },
            ]);

        // Add title bar if present
        if let Some(title_bar) = title_bar {
            panel = panel.child(title_bar);
        }

        // Add content
        panel = panel.child(content);

        // Add actions bar if present
        if let Some(actions_bar) = actions_bar {
            panel = panel.child(actions_bar);
        }

        // Build the backdrop with click-to-dismiss
        let mut backdrop = div()
            .id(self.id)
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .bg(backdrop_color);

        // Add dismiss handler to backdrop
        if let Some(handler) = self.on_dismiss {
            backdrop = backdrop.on_click(move |event, window, cx| {
                handler(event, window, cx);
            });
        }

        // Add the panel to the backdrop
        // The panel stops click propagation to avoid dismissing when clicking inside
        backdrop.child(
            panel
                .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                    cx.stop_propagation();
                }),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sheet_creation() {
        let sheet = Sheet::new("test-sheet");
        assert!(sheet.title.is_none());
        assert!(sheet.actions.is_none());
        assert!(sheet.on_dismiss.is_none());
    }

    #[test]
    fn test_sheet_with_title() {
        let sheet = Sheet::new("test-sheet").title("Test Title");
        assert_eq!(sheet.title, Some("Test Title".into()));
    }

    #[test]
    fn test_sheet_width() {
        let sheet = Sheet::new("test-sheet").width(px(500.0));
        assert_eq!(sheet.width, px(500.0));
    }
}
