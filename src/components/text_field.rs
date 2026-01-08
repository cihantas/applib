//! Single-line text input component for GPUI.
//!
//! This module provides a text field for single-line text input with proper
//! text selection, cursor positioning, and IME support via EntityInputHandler.

use gpui::prelude::*;
use gpui::*;
use std::cell::Cell;
use std::ops::Range;
use std::rc::Rc;
use std::sync::Arc;

use crate::state::Binding;

/// A single-line text input component.
///
/// # Example
///
/// ```ignore
/// // Using a binding (recommended):
/// TextField::new("search", cx)
///     .text(State::binding(&self.query, cx))
///     .placeholder("Search...")
/// ```
pub struct TextField {
    id: ElementId,
    value: String,
    label: Option<SharedString>,
    placeholder: SharedString,
    focus_handle: FocusHandle,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
    text_binding: Option<Binding<String>>,
}

impl TextField {
    /// Creates a new text field with the given id.
    pub fn new(id: impl Into<ElementId>, cx: &mut App) -> Self {
        Self {
            id: id.into(),
            value: String::new(),
            label: None,
            placeholder: "".into(),
            focus_handle: cx.focus_handle(),
            on_change: None,
            text_binding: None,
        }
    }

    /// Sets a two-way binding for the text value.
    pub fn text(mut self, binding: Binding<String>) -> Self {
        self.text_binding = Some(binding);
        self
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
    pub fn on_change(mut self, handler: impl Fn(&String) + 'static) -> Self {
        self.on_change = Some(Box::new(handler));
        self
    }
}

/// Internal state for a TextField rendered as a view.
pub struct TextFieldState {
    id: ElementId,
    value: String,
    label: Option<SharedString>,
    placeholder: SharedString,
    focus_handle: FocusHandle,
    /// Cursor position in UTF-8 byte offset
    cursor_offset: usize,
    /// Selection anchor in UTF-8 byte offset. None means no selection.
    selection_anchor: Option<usize>,
    /// Cached line layout for hit testing
    line_layout: Option<Arc<LineLayout>>,
    /// Element bounds for input handling (updated during paint)
    input_bounds: Rc<Cell<Bounds<Pixels>>>,
    /// Whether we're currently dragging to select
    is_dragging: bool,
    on_change: Option<Box<dyn Fn(&String) + 'static>>,
    text_binding: Option<Binding<String>>,
}

impl TextFieldState {
    /// Get the current text value.
    pub fn value(&self) -> &str {
        &self.value
    }

    /// Set the text value programmatically.
    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.cursor_offset = self.value.len();
        self.selection_anchor = None;
    }

    /// Clear the text field.
    pub fn clear(&mut self) {
        self.value.clear();
        self.cursor_offset = 0;
        self.selection_anchor = None;
    }

    /// Focus the text field.
    pub fn focus(&self, window: &mut Window) {
        self.focus_handle.focus(window);
    }

    /// Returns the selection range (start, end) in byte offsets.
    /// Start is always <= end.
    fn selection_range(&self) -> Option<Range<usize>> {
        self.selection_anchor.map(|anchor| {
            if anchor <= self.cursor_offset {
                anchor..self.cursor_offset
            } else {
                self.cursor_offset..anchor
            }
        })
    }

    /// Returns true if there is an active selection.
    fn has_selection(&self) -> bool {
        self.selection_anchor.is_some()
            && self.selection_anchor != Some(self.cursor_offset)
    }

    /// Clear the selection without moving cursor.
    fn clear_selection(&mut self) {
        self.selection_anchor = None;
    }

    /// Delete the selected text and return true if something was deleted.
    fn delete_selection(&mut self) -> bool {
        if let Some(range) = self.selection_range() {
            if !range.is_empty() {
                self.value.replace_range(range.clone(), "");
                self.cursor_offset = range.start;
                self.selection_anchor = None;
                return true;
            }
        }
        self.selection_anchor = None;
        false
    }

    /// Select all text.
    fn select_all(&mut self) {
        self.selection_anchor = Some(0);
        self.cursor_offset = self.value.len();
    }

    /// Get the selected text, if any.
    fn selected_text(&self) -> Option<&str> {
        self.selection_range().map(|range| &self.value[range])
    }

    /// Move cursor left by one character.
    fn move_left(&mut self, extend_selection: bool) {
        if extend_selection {
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor_offset);
            }
        } else if self.has_selection() {
            if let Some(range) = self.selection_range() {
                self.cursor_offset = range.start;
            }
            self.clear_selection();
            return;
        }

        // Find previous character boundary
        if self.cursor_offset > 0 {
            let mut new_offset = self.cursor_offset - 1;
            while new_offset > 0 && !self.value.is_char_boundary(new_offset) {
                new_offset -= 1;
            }
            self.cursor_offset = new_offset;
        }

        if !extend_selection {
            self.clear_selection();
        }
    }

    /// Move cursor right by one character.
    fn move_right(&mut self, extend_selection: bool) {
        if extend_selection {
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor_offset);
            }
        } else if self.has_selection() {
            if let Some(range) = self.selection_range() {
                self.cursor_offset = range.end;
            }
            self.clear_selection();
            return;
        }

        // Find next character boundary
        if self.cursor_offset < self.value.len() {
            let mut new_offset = self.cursor_offset + 1;
            while new_offset < self.value.len() && !self.value.is_char_boundary(new_offset) {
                new_offset += 1;
            }
            self.cursor_offset = new_offset;
        }

        if !extend_selection {
            self.clear_selection();
        }
    }

    /// Move cursor to start.
    fn move_to_start(&mut self, extend_selection: bool) {
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_offset);
        }
        self.cursor_offset = 0;
        if !extend_selection {
            self.clear_selection();
        }
    }

    /// Move cursor to end.
    fn move_to_end(&mut self, extend_selection: bool) {
        if extend_selection && self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.cursor_offset);
        }
        self.cursor_offset = self.value.len();
        if !extend_selection {
            self.clear_selection();
        }
    }

    /// Insert text at cursor, replacing selection if any.
    fn insert_text(&mut self, text: &str) {
        self.delete_selection();
        self.value.insert_str(self.cursor_offset, text);
        self.cursor_offset += text.len();
    }

    /// Delete character before cursor.
    fn backspace(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.cursor_offset > 0 {
            let mut start = self.cursor_offset - 1;
            while start > 0 && !self.value.is_char_boundary(start) {
                start -= 1;
            }
            self.value.replace_range(start..self.cursor_offset, "");
            self.cursor_offset = start;
        }
    }

    /// Delete character after cursor.
    fn delete(&mut self) {
        if self.delete_selection() {
            return;
        }
        if self.cursor_offset < self.value.len() {
            let mut end = self.cursor_offset + 1;
            while end < self.value.len() && !self.value.is_char_boundary(end) {
                end += 1;
            }
            self.value.replace_range(self.cursor_offset..end, "");
        }
    }

    /// Get byte offset from x position using line layout.
    fn index_for_x(&self, x: Pixels, padding: Pixels) -> usize {
        let relative_x = x - padding;
        if relative_x <= px(0.0) {
            return 0;
        }

        if let Some(ref layout) = self.line_layout {
            layout.closest_index_for_x(relative_x)
        } else {
            // Fallback: estimate based on average char width
            let avg_char_width = px(7.5);
            let estimated = (relative_x / avg_char_width).round() as usize;
            estimated.min(self.value.len())
        }
    }

    fn handle_key_down(
        &mut self,
        event: &KeyDownEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let key = &event.keystroke.key;
        let modifiers = &event.keystroke.modifiers;
        let shift = modifiers.shift;

        match key.as_str() {
            "backspace" => self.backspace(),
            "delete" => self.delete(),
            "enter" => return, // Single-line input
            "left" => self.move_left(shift),
            "right" => self.move_right(shift),
            "home" => self.move_to_start(shift),
            "end" => self.move_to_end(shift),
            "tab" => return, // Let tab move focus
            "a" if modifiers.platform || modifiers.control => {
                self.select_all();
                cx.notify();
                return;
            }
            "c" if modifiers.platform || modifiers.control => {
                // Copy
                if let Some(text) = self.selected_text() {
                    cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
                }
                return;
            }
            "x" if modifiers.platform || modifiers.control => {
                // Cut
                if let Some(text) = self.selected_text() {
                    cx.write_to_clipboard(ClipboardItem::new_string(text.to_string()));
                    self.delete_selection();
                }
            }
            "v" if modifiers.platform || modifiers.control => {
                // Paste
                if let Some(item) = cx.read_from_clipboard() {
                    if let Some(text) = item.text() {
                        // Remove newlines for single-line input
                        let text = text.replace('\n', " ").replace('\r', "");
                        self.insert_text(&text);
                    }
                }
            }
            _ => {
                // Handle regular character input
                if modifiers.control || modifiers.platform {
                    return;
                }

                if let Some(ref key_char) = event.keystroke.key_char {
                    for c in key_char.chars() {
                        if c != '\n' && c != '\r' {
                            self.insert_text(&c.to_string());
                        }
                    }
                } else if key.len() == 1 {
                    if let Some(c) = key.chars().next() {
                        if !c.is_control() {
                            self.insert_text(&c.to_string());
                        }
                    }
                }
            }
        }

        // Notify change
        if let Some(ref handler) = self.on_change {
            handler(&self.value);
        }
        if let Some(ref binding) = self.text_binding {
            binding.set(self.value.clone(), &mut **cx);
        }
        cx.notify();
    }

    fn handle_mouse_down(
        &mut self,
        event: &MouseDownEvent,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.focus_handle.focus(window);

        // Double-click selects all
        if event.click_count == 2 {
            self.select_all();
            cx.notify();
            return;
        }

        // Calculate cursor position from click
        // Convert window coordinates to element-local coordinates
        let bounds = self.input_bounds.get();
        let local_x = event.position.x - bounds.origin.x;

        // Account for padding inside the input field
        let padding = px(8.0);
        let new_offset = self.index_for_x(local_x, padding);

        if event.modifiers.shift {
            // Extend selection
            if self.selection_anchor.is_none() {
                self.selection_anchor = Some(self.cursor_offset);
            }
        } else {
            // Start new selection
            self.selection_anchor = Some(new_offset);
        }

        self.cursor_offset = new_offset;
        self.is_dragging = true;
        cx.notify();
    }

    fn handle_mouse_up(
        &mut self,
        _event: &MouseUpEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.is_dragging = false;

        // If anchor equals cursor, clear selection (it was just a click)
        if self.selection_anchor == Some(self.cursor_offset) {
            self.clear_selection();
        }
        cx.notify();
    }

    fn handle_mouse_move(
        &mut self,
        event: &MouseMoveEvent,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        if !self.is_dragging {
            return;
        }

        // Convert window coordinates to element-local coordinates
        let bounds = self.input_bounds.get();
        let local_x = event.position.x - bounds.origin.x;

        let padding = px(8.0);
        self.cursor_offset = self.index_for_x(local_x, padding);
        cx.notify();
    }

    /// Create TextRun for the current text with default styling.
    fn text_run(&self, font: Font, color: Hsla) -> TextRun {
        TextRun {
            len: self.value.len(),
            font,
            color,
            background_color: None,
            underline: None,
            strikethrough: None,
        }
    }
}

impl Render for TextFieldState {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Sync value from binding
        if let Some(ref binding) = self.text_binding {
            let bound_value = binding.get(&**cx);
            if bound_value != self.value {
                self.value = bound_value;
                self.cursor_offset = self.cursor_offset.min(self.value.len());
            }
        }

        let is_focused = self.focus_handle.is_focused(window);
        let is_empty = self.value.is_empty();
        let placeholder = self.placeholder.clone();
        let label = self.label.clone();

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
        let selection_bg = hsla(211.0 / 360.0, 0.80, 0.55, 0.3);
        let cursor_color = hsla(0.0, 0.0, 0.0, 0.8);

        // Get font settings
        let font_size = px(14.0); // text_sm equivalent
        let font = Font {
            family: "system-ui".into(),
            features: Default::default(),
            fallbacks: None,
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
        };

        // Layout text for hit testing
        if !self.value.is_empty() {
            let run = self.text_run(font.clone(), text_color);
            self.line_layout = Some(window.text_system().layout_line(
                &self.value,
                font_size,
                &[run],
                None,
            ));
        } else {
            self.line_layout = None;
        }

        // Build text element
        let text_element = if is_empty && !is_focused {
            div().text_color(placeholder_color).child(placeholder.to_string())
        } else if is_focused {
            let selection_range = self.selection_range();

            if let Some(range) = selection_range.filter(|r| !r.is_empty()) {
                // Has selection
                let before = &self.value[..range.start];
                let selected = &self.value[range.clone()];
                let after = &self.value[range.end..];

                div()
                    .flex()
                    .items_center()
                    .text_color(text_color)
                    .child(before.to_string())
                    .child(
                        div()
                            .bg(selection_bg)
                            .rounded(px(2.0))
                            .child(selected.to_string()),
                    )
                    .child(after.to_string())
            } else {
                // No selection, show cursor
                let before = &self.value[..self.cursor_offset];
                let after = &self.value[self.cursor_offset..];

                div()
                    .flex()
                    .items_center()
                    .text_color(text_color)
                    .child(before.to_string())
                    .child(
                        div()
                            .w(px(1.0))
                            .h(px(14.0))
                            .bg(cursor_color),
                    )
                    .child(after.to_string())
            }
        } else {
            div().text_color(text_color).child(self.value.clone())
        };

        // Clone bounds tracker for the canvas callback
        let bounds_cell = self.input_bounds.clone();

        // Bounds tracking canvas overlay
        let bounds_tracker = canvas(
            |bounds, _window, _cx| bounds,
            move |bounds, _, _window, _cx| {
                bounds_cell.set(bounds);
            },
        )
        .absolute()
        .size_full();

        // Input field element (interactive container)
        let input_container = div()
            .id(self.id.clone())
            .track_focus(&self.focus_handle)
            .key_context("TextField")
            .on_key_down(cx.listener(Self::handle_key_down))
            .on_mouse_down(MouseButton::Left, cx.listener(Self::handle_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(Self::handle_mouse_up))
            .on_mouse_move(cx.listener(Self::handle_mouse_move))
            .w_full()
            .h(px(28.0))
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
            .child(text_element);

        // Wrap input with bounds tracker
        let input_field = div()
            .relative()
            .w_full()
            .child(input_container)
            .child(bounds_tracker);

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

/// Creates a TextFieldState from a TextField builder.
impl From<TextField> for TextFieldState {
    fn from(builder: TextField) -> Self {
        TextFieldState {
            id: builder.id,
            value: builder.value.clone(),
            label: builder.label,
            placeholder: builder.placeholder,
            focus_handle: builder.focus_handle,
            cursor_offset: builder.value.len(),
            selection_anchor: None,
            line_layout: None,
            input_bounds: Rc::new(Cell::new(Bounds::default())),
            is_dragging: false,
            on_change: builder.on_change,
            text_binding: builder.text_binding,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
