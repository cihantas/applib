//! Multi-line text input component for GPUI.
//!
//! This module provides a text area for multi-line text input, such as commit messages.

use gpui::prelude::*;
use gpui::*;

/// A multi-line text input component.
///
/// # Example
///
/// ```ignore
/// TextArea::new("commit-message", cx)
///     .placeholder("Enter commit message...")
///     .rows(4)
///     .on_change(|text| {
///         println!("Text changed: {}", text);
///     })
/// ```
pub struct TextArea {
    id: ElementId,
    value: String,
    placeholder: SharedString,
    rows: u8,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
}

impl TextArea {
    /// Creates a new text area with the given id.
    pub fn new(id: impl Into<ElementId>, cx: &mut App) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            placeholder: "".into(),
            rows: 4,
            focus_handle: cx.focus_handle(),
            on_change: None,
        }
    }

    /// Sets the current text value.
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = value.into();
        self
    }

    /// Sets the placeholder text shown when empty.
    pub fn placeholder(mut self, placeholder: impl Into<SharedString>) -> Self {
        self.placeholder = placeholder.into();
        self
    }

    /// Sets the number of visible rows (height hint).
    pub fn rows(mut self, rows: u8) -> Self {
        self.rows = rows;
        self
    }

    /// Sets the change handler called when text changes.
    /// The handler receives the new text value.
    pub fn on_change(
        mut self,
        handler: impl Fn(&String) + 'static,
    ) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

/// Internal state for a TextArea rendered as a view.
pub struct TextAreaState {
    id: ElementId,
    value: String,
    placeholder: SharedString,
    rows: u8,
    focus_handle: FocusHandle,
    cursor_position: usize,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
}

impl TextAreaState {
    /// Get the current text value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the text value programmatically.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.cursor_position = self.value.len();
    }

    /// Clear the text area.
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_position = 0;
    }

    fn handle_key_down(&mut self, event: &KeyDownEvent, _window: &mut Window, cx: &mut Context<Self>) {
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
                self.insert_char('\n');
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
                // Move to start of current line
                let chars: Vec<_> = self.value.chars().collect();
                let mut pos = self.cursor_position;
                while pos > 0 && chars.get(pos - 1) != Some(&'\n') {
                    pos -= 1;
                }
                self.cursor_position = pos;
            }
            "end" => {
                // Move to end of current line
                let chars: Vec<_> = self.value.chars().collect();
                let mut pos = self.cursor_position;
                while pos < chars.len() && chars.get(pos) != Some(&'\n') {
                    pos += 1;
                }
                self.cursor_position = pos;
            }
            "tab" => {
                // Insert spaces for tab
                for _ in 0..4 {
                    self.insert_char(' ');
                }
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
                        self.insert_char(c);
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
}

impl Render for TextAreaState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let is_focused = self.focus_handle.is_focused(window);
        let is_empty = self.value.is_empty();
        let rows = self.rows;
        let placeholder = self.placeholder.clone();

        // Calculate line height and total height
        let line_height = 18.0;
        let padding = 8.0;
        let height = (rows as f32 * line_height) + (padding * 2.0);

        // Colors
        let bg_color = hsla(0.0, 0.0, 1.0, 1.0);
        let border_color = if is_focused {
            hsla(211.0 / 360.0, 0.80, 0.55, 1.0)
        } else {
            hsla(0.0, 0.0, 0.75, 1.0)
        };
        let text_color = hsla(0.0, 0.0, 0.15, 1.0);
        let placeholder_color = hsla(0.0, 0.0, 0.55, 1.0);

        // Build the display text with cursor
        let display_text = if is_empty && !is_focused {
            placeholder.to_string()
        } else {
            self.value.clone()
        };

        let text_element = if is_empty && !is_focused {
            div()
                .text_color(placeholder_color)
                .child(display_text)
        } else {
            // Show text with cursor indicator when focused
            if is_focused {
                let chars: Vec<_> = self.value.chars().collect();
                let before: String = chars[..self.cursor_position].iter().collect();
                let after: String = chars[self.cursor_position..].iter().collect();

                div()
                    .flex()
                    .text_color(text_color)
                    .child(before)
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(line_height))
                            .bg(hsla(0.0, 0.0, 0.0, 0.8))
                    )
                    .child(after)
            } else {
                div()
                    .text_color(text_color)
                    .child(display_text)
            }
        };

        div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .key_context("TextArea")
            .on_key_down(cx.listener(Self::handle_key_down))
            .w_full()
            .h(px(height))
            .p(px(padding))
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
            .child(text_element)
    }
}

/// Creates a TextAreaState from a TextArea builder.
impl From<TextArea> for TextAreaState {
    fn from(builder: TextArea) -> Self {
        let cursor_position = builder.value.len();
        TextAreaState {
            id: builder.id,
            value: builder.value,
            placeholder: builder.placeholder,
            rows: builder.rows,
            focus_handle: builder.focus_handle,
            cursor_position,
            on_change: builder.on_change,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: These tests would require a GPUI test harness to run properly
    // For now, they serve as documentation of expected behavior
}
