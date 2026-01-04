//! Secure text input component for GPUI.
//!
//! This module provides a password/secure text field that masks input with bullets.

use gpui::prelude::*;
use gpui::*;

/// A secure text input component that masks characters.
///
/// # Example
///
/// ```ignore
/// SecureField::new("password", cx)
///     .label("Password")
///     .placeholder("Enter password")
///     .value(&self.password)
///     .on_change(|text| {
///         println!("Password changed");
///     })
///     .show_toggle(true)  // Optional: adds reveal button
/// ```
pub struct SecureField {
    id: ElementId,
    value: String,
    label: Option<SharedString>,
    placeholder: SharedString,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
    show_toggle: bool,
}

impl SecureField {
    /// Creates a new secure field with the given id.
    pub fn new(id: impl Into<ElementId>, cx: &mut App) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            label: None,
            placeholder: "".into(),
            focus_handle: cx.focus_handle(),
            on_change: None,
            show_toggle: false,
        }
    }

    /// Sets the current text value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the label text shown above the input.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the placeholder text shown when empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the change handler called when text changes.
    /// The handler receives the new text value.
    pub fn on_change(mut self, handler: impl Fn(&String) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }

    /// Enables the show/hide toggle button (eye icon).
    pub fn show_toggle(mut self, show: bool) -> Self {
        self.show_toggle = show;
        self
    }
}

/// Internal state for a SecureField rendered as a view.
pub struct SecureFieldState {
    id: ElementId,
    value: String,
    label: Option<SharedString>,
    placeholder: SharedString,
    focus_handle: FocusHandle,
    cursor_position: usize,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
    show_toggle: bool,
    is_revealed: bool,
}

impl SecureFieldState {
    /// Get the current text value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the text value programmatically.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.cursor_position = self.value.chars().count();
    }

    /// Clear the text field.
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
    }

    /// Check if the password is currently revealed.
    pub fn is_revealed(&self) -> bool {
        self.is_revealed
    }

    /// Toggle password visibility.
    pub fn toggle_visibility(&mut self) {
        self.is_revealed = !self.is_revealed;
    }

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = &event.keystroke.key;
        let modifiers = &event.keystroke.modifiers;

        // Handle special keys
        match key.as_str() {
            "backspace" => {
                if self.cursor_position > 0 {
                    // Find the byte position for the character before cursor
                    let char_indices: Vec<_> = self.value.char_indices().collect();
                    if let Some(&(byte_pos, _)) = char_indices.get(self.cursor_position - 1) {
                        let end_pos = char_indices
                            .get(self.cursor_position)
                            .map(|&(pos, _)| pos)
                            .unwrap_or(self.value.len());
                        self.value.replace_range(byte_pos..end_pos, "");
                        self.cursor_position -= 1;
                    }
                }
            }
            "delete" => {
                let char_indices: Vec<_> = self.value.char_indices().collect();
                if self.cursor_position < char_indices.len() {
                    let start_pos = char_indices[self.cursor_position].0;
                    let end_pos = char_indices
                        .get(self.cursor_position + 1)
                        .map(|&(pos, _)| pos)
                        .unwrap_or(self.value.len());
                    self.value.replace_range(start_pos..end_pos, "");
                }
            }
            "enter" => {
                // Single-line input - do nothing on enter
                return;
            }
            "left" => {
                if self.cursor_position > 0 {
                    self.cursor_position -= 1;
                }
            }
            "right" => {
                let char_count = self.value.chars().count();
                if self.cursor_position < char_count {
                    self.cursor_position += 1;
                }
            }
            "home" => {
                self.cursor_position = 0;
            }
            "end" => {
                self.cursor_position = self.value.chars().count();
            }
            "tab" => {
                // Don't handle tab - let it move focus
                return;
            }
            _ => {
                // Handle regular character input
                // Skip if control/command is held (except for standard shortcuts)
                if modifiers.control || modifiers.platform {
                    // Handle Ctrl+A (select all - just move cursor to end for now)
                    if key == "a" && modifiers.platform {
                        self.cursor_position = self.value.chars().count();
                        return;
                    }
                    return;
                }

                // Use key_char if available, otherwise use key for printable chars
                if let Some(ref key_char) = event.keystroke.key_char {
                    for c in key_char.chars() {
                        // Skip newlines in single-line input
                        if c != '\n' && c != '\r' {
                            self.insert_char(c);
                        }
                    }
                } else if key.len() == 1 {
                    // Single character key
                    if let Some(c) = key.chars().next() {
                        if !c.is_control() {
                            let c = if modifiers.shift {
                                c.to_uppercase().next().unwrap_or(c)
                            } else {
                                c
                            };
                            self.insert_char(c);
                        }
                    }
                }
            }
        }

        // Notify change
        if let Some(ref handler) = self.on_change {
            handler(&self.value);
        }
        cx.notify();
    }

    fn insert_char(&mut self, c: char) {
        let char_indices: Vec<_> = self.value.char_indices().collect();
        let byte_pos = char_indices
            .get(self.cursor_position)
            .map(|&(pos, _)| pos)
            .unwrap_or(self.value.len());
        self.value.insert(byte_pos, c);
        self.cursor_position += 1;
    }

    /// Convert the actual value to masked bullets for display.
    fn masked_value(&self) -> String {
        "•".repeat(self.value.chars().count())
    }
}

impl Render for SecureFieldState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);
        let is_empty = self.value.is_empty();
        let placeholder = self.placeholder.clone();
        let label = self.label.clone();
        let is_revealed = self.is_revealed;
        let show_toggle = self.show_toggle;

        // Colors
        let bg_color = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = if is_focused {
            hsla(211.0 / 360.0, 0.80, 0.55, 1.0)
        } else {
            hsla(0.0, 0.0, 0.75, 1.0)
        };
        let text_color = hsla(0.0, 0.0, 0.15, 1.0);
        let placeholder_color = hsla(0.0, 0.0, 0.55, 1.0);
        let label_color = hsla(0.0, 0.0, 0.30, 1.0);
        let toggle_color = hsla(0.0, 0.0, 0.50, 1.0);
        let toggle_hover_color = hsla(0.0, 0.0, 0.30, 1.0);

        // Build the display text (masked or revealed)
        let display_text = if is_empty && !is_focused {
            placeholder.to_string()
        } else if is_revealed {
            self.value.clone()
        } else {
            self.masked_value()
        };

        let text_element = if is_empty && !is_focused {
            div().text_color(placeholder_color).child(display_text)
        } else {
            // Show text with cursor indicator when focused
            if is_focused {
                let display_chars: Vec<char> = if is_revealed {
                    self.value.chars().collect()
                } else {
                    std::iter::repeat('•').take(self.value.chars().count()).collect()
                };
                let before: String = display_chars[..self.cursor_position].iter().collect();
                let after: String = display_chars[self.cursor_position..].iter().collect();

                div()
                    .flex()
                    .items_center()
                    .text_color(text_color)
                    .child(before)
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(14.0))
                            .bg(hsla(0.0, 0.0, 0.0, 0.8)),
                    )
                    .child(after)
            } else {
                div().text_color(text_color).child(display_text)
            }
        };

        // Toggle button (eye icon)
        let toggle_button = if show_toggle {
            let icon_text = if is_revealed { "◉" } else { "○" };
            Some(
                div()
                    .id("toggle-visibility")
                    .ml(px(6.0))
                    .px(px(4.0))
                    .cursor_pointer()
                    .text_color(toggle_color)
                    .hover(|style| style.text_color(toggle_hover_color))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.toggle_visibility();
                        cx.notify();
                    }))
                    .child(icon_text),
            )
        } else {
            None
        };

        // Input field
        let input_field = div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .key_context("SecureField")
            .on_key_down(cx.listener(Self::handle_key_down))
            .w_full()
            .h(px(24.0))
            .px(px(8.0))
            .flex()
            .items_center()
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(4.0))
            .text_sm()
            .overflow_hidden()
            .cursor_text()
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.05),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }])
            // Focus ring
            .when(is_focused, |div| {
                div.shadow(vec![
                    BoxShadow {
                        color: hsla(0.0, 0.0, 0.0, 0.05),
                        offset: point(px(0.0), px(1.0)),
                        blur_radius: px(2.0),
                        spread_radius: px(0.0),
                    },
                    BoxShadow {
                        color: hsla(211.0 / 360.0, 0.80, 0.55, 0.3),
                        offset: point(px(0.0), px(0.0)),
                        blur_radius: px(0.0),
                        spread_radius: px(3.0),
                    },
                ])
            })
            .child(
                div()
                    .flex()
                    .flex_1()
                    .items_center()
                    .overflow_hidden()
                    .child(text_element),
            )
            .when_some(toggle_button, |div, button| div.child(button));

        // Wrap with label if provided
        if let Some(label_text) = label {
            div()
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(label_color)
                        .child(label_text),
                )
                .child(input_field)
        } else {
            div().child(input_field)
        }
    }
}

/// Creates a SecureFieldState from a SecureField builder.
impl From<SecureField> for SecureFieldState {
    fn from(builder: SecureField) -> Self {
        let cursor_position = builder.value.chars().count();
        SecureFieldState {
            id: builder.id,
            value: builder.value,
            label: builder.label,
            placeholder: builder.placeholder,
            focus_handle: builder.focus_handle,
            cursor_position,
            on_change: builder.on_change,
            show_toggle: builder.show_toggle,
            is_revealed: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a GPUI test harness to run properly
    // For now, they serve as documentation of expected behavior
}
