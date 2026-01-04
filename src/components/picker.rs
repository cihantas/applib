//! Picker component for selecting from multiple options.
//!
//! This module provides a picker component for selecting from options.
//! Supports multiple styles: Menu (dropdown), Segmented (horizontal button group),
//! and Inline (vertical list).

use gpui::prelude::*;
use gpui::*;
use std::rc::Rc;

/// A single option in a picker.
#[derive(Clone)]
struct PickerOption {
    label: SharedString,
}

/// Picker style variants.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum PickerStyle {
    /// Dropdown menu that shows options on click (default).
    #[default]
    Menu,
    /// Horizontal segmented button group.
    Segmented,
    /// Vertical list of options (inline).
    Inline,
}

/// A picker component for selecting from multiple options.
///
/// # Example
///
/// ```ignore
/// Picker::new("color-picker", selected_index)
///     .label("Color")
///     .option("Red")
///     .option("Blue")
///     .option("Green")
///     .style(PickerStyle::Menu)
///     .on_change(|new_index, window, cx| {
///         // Handle selection change
///     })
/// ```
pub struct Picker {
    id: ElementId,
    label: Option<SharedString>,
    options: Vec<PickerOption>,
    selected_index: usize,
    picker_style: PickerStyle,
    disabled: bool,
    is_open: bool,
    on_change: Option<Rc<dyn Fn(usize, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Box<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl Picker {
    /// Creates a new picker with the given id and selected index.
    pub fn new(id: impl Into<ElementId>, selected_index: usize) -> Self {
        Self {
            id: id.into(),
            label: None,
            options: Vec::new(),
            selected_index,
            picker_style: PickerStyle::default(),
            disabled: false,
            is_open: false,
            on_change: None,
            on_toggle: None,
        }
    }

    /// Sets the label displayed above the picker.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Adds an option to the picker.
    pub fn option(mut self, label: impl Into<SharedString>) -> Self {
        self.options.push(PickerOption {
            label: label.into(),
        });
        self
    }

    /// Sets the picker style.
    pub fn style(mut self, style: PickerStyle) -> Self {
        self.picker_style = style;
        self
    }

    /// Sets the picker to menu (dropdown) style.
    pub fn menu(mut self) -> Self {
        self.picker_style = PickerStyle::Menu;
        self
    }

    /// Sets the picker to segmented (horizontal button group) style.
    pub fn segmented(mut self) -> Self {
        self.picker_style = PickerStyle::Segmented;
        self
    }

    /// Sets the picker to inline (vertical list) style.
    pub fn inline(mut self) -> Self {
        self.picker_style = PickerStyle::Inline;
        self
    }

    /// Sets whether the picker is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the menu is open (only applies to Menu style).
    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    /// Sets the change handler for this picker.
    ///
    /// The handler receives the index of the newly selected option.
    pub fn on_change(
        mut self,
        handler: impl Fn(usize, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Sets the toggle handler for menu open/close state.
    ///
    /// Only applies to Menu style. The handler receives the new open state.
    pub fn on_toggle(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }

    /// Builds the Menu style picker (dropdown).
    fn build_menu(self) -> Stateful<Div> {
        let disabled = self.disabled;
        let is_open = self.is_open;
        let selected_index = self.selected_index;
        let on_change = self.on_change;
        let on_toggle = self.on_toggle;
        let options = self.options.clone();

        // Get the selected label
        let selected_label = self
            .options
            .get(selected_index)
            .map(|o| o.label.clone())
            .unwrap_or_else(|| "Select...".into());

        // Colors
        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0)
        } else {
            hsla(0.0, 0.0, 0.20, 1.0)
        };
        let label_color = hsla(0.0, 0.0, 0.35, 1.0);
        let bg_color = if disabled {
            hsla(0.0, 0.0, 0.90, 1.0)
        } else {
            hsla(0.0, 0.0, 0.97, 1.0)
        };
        let border_color = if disabled {
            hsla(0.0, 0.0, 0.82, 1.0)
        } else {
            hsla(0.0, 0.0, 0.78, 1.0)
        };
        let hover_bg = hsla(0.0, 0.0, 0.93, 1.0);

        // Build the container
        let mut container = div().id(self.id).flex().flex_col().gap(px(4.0));

        // Add label if provided
        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(label_color)
                    .child(label),
            );
        }

        // Build the trigger button
        let mut trigger = div()
            .id("picker-trigger")
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px_3()
            .py_1()
            .min_w(px(120.0))
            .h(px(24.0))
            .rounded(px(6.0))
            .text_sm()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .text_color(text_color)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        if !disabled {
            trigger = trigger.cursor_pointer().hover(|style| style.bg(hover_bg));
        }

        // Arrow indicator
        let arrow = div()
            .text_xs()
            .text_color(hsla(0.0, 0.0, 0.50, 1.0))
            .child(if is_open { "▲" } else { "▼" });

        trigger = trigger.child(selected_label).child(arrow);

        // Add click handler for toggle
        if !disabled {
            if let Some(handler) = on_toggle {
                let new_state = !is_open;
                trigger = trigger.on_click(move |_event, window, cx| {
                    handler(new_state, window, cx);
                });
            }
        }

        // Wrap trigger in relative container for dropdown positioning
        let mut trigger_container = div().relative().child(trigger);

        // Add dropdown menu if open
        if is_open && !disabled {
            let mut dropdown = div()
                .absolute()
                .top_full()
                .left(px(0.0))
                .mt(px(4.0))
                .min_w(px(120.0))
                .bg(hsla(0.0, 0.0, 1.0, 1.0))
                .rounded(px(6.0))
                .border_1()
                .border_color(hsla(0.0, 0.0, 0.78, 1.0))
                .shadow(vec![BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.15),
                    offset: point(px(0.0), px(4.0)),
                    blur_radius: px(12.0),
                    spread_radius: px(0.0),
                }])
                .overflow_hidden();

            // Add options
            for (index, option) in options.into_iter().enumerate() {
                let is_selected = index == selected_index;
                let option_bg = if is_selected {
                    hsla(211.0 / 360.0, 0.95, 0.53, 1.0)
                } else {
                    hsla(0.0, 0.0, 1.0, 1.0)
                };
                let option_text = if is_selected {
                    gpui::white()
                } else {
                    hsla(0.0, 0.0, 0.20, 1.0)
                };
                let option_hover_bg = if is_selected {
                    hsla(211.0 / 360.0, 0.95, 0.48, 1.0)
                } else {
                    hsla(0.0, 0.0, 0.95, 1.0)
                };

                let mut option_row = div()
                    .id(("picker-option", index))
                    .flex()
                    .items_center()
                    .px_3()
                    .py_1()
                    .text_sm()
                    .bg(option_bg)
                    .text_color(option_text)
                    .cursor_pointer()
                    .hover(move |style| style.bg(option_hover_bg))
                    .child(option.label);

                // Add click handler
                if let Some(ref handler) = on_change {
                    let handler = handler.clone();
                    option_row = option_row.on_click(move |_event, window, cx| {
                        handler(index, window, cx);
                    });
                }

                dropdown = dropdown.child(option_row);
            }

            trigger_container = trigger_container.child(dropdown);
        }

        container.child(trigger_container)
    }

    /// Builds the Segmented style picker (horizontal button group).
    fn build_segmented(self) -> Stateful<Div> {
        let disabled = self.disabled;
        let selected_index = self.selected_index;
        let on_change = self.on_change;

        // Colors
        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0)
        } else {
            hsla(0.0, 0.0, 0.20, 1.0)
        };
        let label_color = hsla(0.0, 0.0, 0.35, 1.0);

        // Build the container
        let mut container = div().id(self.id).flex().flex_col().gap(px(4.0));

        // Add label if provided
        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(label_color)
                    .child(label),
            );
        }

        // Build the segmented control
        let mut segments = div()
            .flex()
            .flex_row()
            .rounded(px(6.0))
            .border_1()
            .border_color(if disabled {
                hsla(0.0, 0.0, 0.82, 1.0)
            } else {
                hsla(0.0, 0.0, 0.78, 1.0)
            })
            .overflow_hidden()
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.08),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }]);

        let option_count = self.options.len();

        for (index, option) in self.options.into_iter().enumerate() {
            let is_selected = index == selected_index;
            let is_last = index == option_count - 1;

            // Segment colors
            let segment_bg = if disabled {
                hsla(0.0, 0.0, 0.90, 1.0)
            } else if is_selected {
                hsla(211.0 / 360.0, 0.95, 0.53, 1.0)
            } else {
                hsla(0.0, 0.0, 0.97, 1.0)
            };

            let segment_text = if is_selected && !disabled {
                gpui::white()
            } else {
                text_color
            };

            let segment_hover_bg = if disabled {
                segment_bg
            } else if is_selected {
                hsla(211.0 / 360.0, 0.95, 0.48, 1.0)
            } else {
                hsla(0.0, 0.0, 0.93, 1.0)
            };

            let mut segment = div()
                .id(("picker-segment", index))
                .flex()
                .items_center()
                .justify_center()
                .px_3()
                .py_1()
                .min_w(px(60.0))
                .text_sm()
                .font_weight(FontWeight::MEDIUM)
                .bg(segment_bg)
                .text_color(segment_text)
                .child(option.label);

            // Add right border separator (except for last item)
            if !is_last {
                segment = segment.border_r_1().border_color(if disabled {
                    hsla(0.0, 0.0, 0.82, 1.0)
                } else {
                    hsla(0.0, 0.0, 0.78, 1.0)
                });
            }

            if !disabled {
                segment = segment.cursor_pointer().hover(move |style| style.bg(segment_hover_bg));

                // Add click handler
                if let Some(ref handler) = on_change {
                    let handler = handler.clone();
                    segment = segment.on_click(move |_event, window, cx| {
                        handler(index, window, cx);
                    });
                }
            }

            segments = segments.child(segment);
        }

        container.child(segments)
    }

    /// Builds the Inline style picker (vertical list).
    fn build_inline(self) -> Stateful<Div> {
        let disabled = self.disabled;
        let selected_index = self.selected_index;
        let on_change = self.on_change;

        // Colors
        let text_color = if disabled {
            hsla(0.0, 0.0, 0.55, 1.0)
        } else {
            hsla(0.0, 0.0, 0.20, 1.0)
        };
        let label_color = hsla(0.0, 0.0, 0.35, 1.0);

        // Build the container
        let mut container = div().id(self.id).flex().flex_col().gap(px(4.0));

        // Add label if provided
        if let Some(label) = self.label {
            container = container.child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(label_color)
                    .mb(px(4.0))
                    .child(label),
            );
        }

        // Build the options list
        for (index, option) in self.options.into_iter().enumerate() {
            let is_selected = index == selected_index;

            // Option colors
            let option_bg = if is_selected && !disabled {
                hsla(211.0 / 360.0, 0.95, 0.53, 1.0)
            } else {
                hsla(0.0, 0.0, 0.0, 0.0) // Transparent
            };

            let option_text = if is_selected && !disabled {
                gpui::white()
            } else {
                text_color
            };

            let option_hover_bg = if disabled {
                option_bg
            } else if is_selected {
                hsla(211.0 / 360.0, 0.95, 0.48, 1.0)
            } else {
                hsla(0.0, 0.0, 0.95, 1.0)
            };

            let mut option_row = div()
                .id(("picker-inline-option", index))
                .flex()
                .flex_row()
                .items_center()
                .px_3()
                .py_1()
                .rounded(px(4.0))
                .text_sm()
                .bg(option_bg)
                .text_color(option_text);

            // Add checkmark for selected option
            if is_selected {
                option_row = option_row.child(
                    div()
                        .mr(px(6.0))
                        .text_xs()
                        .child("✓"),
                );
            } else {
                // Placeholder for alignment
                option_row = option_row.child(
                    div()
                        .mr(px(6.0))
                        .text_xs()
                        .w(px(12.0))
                        .child(""),
                );
            }

            option_row = option_row.child(option.label);

            if !disabled {
                option_row = option_row.cursor_pointer().hover(move |style| style.bg(option_hover_bg));

                // Add click handler
                if let Some(ref handler) = on_change {
                    let handler = handler.clone();
                    option_row = option_row.on_click(move |_event, window, cx| {
                        handler(index, window, cx);
                    });
                }
            }

            container = container.child(option_row);
        }

        container
    }
}

impl IntoElement for Picker {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        match self.picker_style {
            PickerStyle::Menu => self.build_menu(),
            PickerStyle::Segmented => self.build_segmented(),
            PickerStyle::Inline => self.build_inline(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_picker_creation() {
        let picker = Picker::new("test", 0);
        assert_eq!(picker.selected_index, 0);
        assert_eq!(picker.picker_style, PickerStyle::Menu);
        assert!(!picker.disabled);
        assert!(picker.options.is_empty());
    }

    #[test]
    fn test_picker_with_options() {
        let picker = Picker::new("test", 1)
            .option("Red")
            .option("Blue")
            .option("Green");
        assert_eq!(picker.options.len(), 3);
        assert_eq!(picker.selected_index, 1);
    }

    #[test]
    fn test_picker_styles() {
        let picker = Picker::new("test", 0).menu();
        assert_eq!(picker.picker_style, PickerStyle::Menu);

        let picker = Picker::new("test", 0).segmented();
        assert_eq!(picker.picker_style, PickerStyle::Segmented);

        let picker = Picker::new("test", 0).inline();
        assert_eq!(picker.picker_style, PickerStyle::Inline);

        let picker = Picker::new("test", 0).style(PickerStyle::Segmented);
        assert_eq!(picker.picker_style, PickerStyle::Segmented);
    }

    #[test]
    fn test_picker_with_label() {
        let picker = Picker::new("test", 0).label("Choose a color");
        assert!(picker.label.is_some());
    }

    #[test]
    fn test_picker_disabled() {
        let picker = Picker::new("test", 0).disabled(true);
        assert!(picker.disabled);
    }

    #[test]
    fn test_picker_is_open() {
        let picker = Picker::new("test", 0).is_open(true);
        assert!(picker.is_open);
    }
}
