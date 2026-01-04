//! Form container component for grouping form controls.
//!
//! This module provides a Form component that arranges form fields vertically
//! with consistent label alignment and spacing.

use gpui::prelude::*;
use gpui::*;

/// A form container for grouping form controls.
///
/// Form provides a vertical layout with consistent spacing between fields.
/// Forms typically display labels on the left with controls on the right,
/// but this component also supports inline labels within controls.
///
/// # Example
///
/// ```ignore
/// Form::new()
///     .child(TextField::new("name", cx).label("Name"))
///     .child(Toggle::new("subscribe", "Subscribe to newsletter", is_on))
///     .section("Preferences", |section| {
///         section.child(Picker::new(...))
///     })
/// ```
pub struct Form {
    children: Vec<AnyElement>,
    label_width: Option<Pixels>,
    spacing: Pixels,
    padding: Option<Pixels>,
}

impl Form {
    /// Creates a new form container.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            label_width: None,
            spacing: px(16.0), // Default spacing between form rows
            padding: Some(px(16.0)),
        }
    }

    /// Sets the fixed width for labels (for grid-style alignment).
    /// When set, form rows with FormRow components will align labels to this width.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self.label_width = Some(width.into());
        self
    }

    /// Sets the vertical spacing between form fields.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets padding around the form.
    pub fn padding(mut self, padding: impl Into<Pixels>) -> Self {
        self.padding = Some(padding.into());
        self
    }

    /// Removes padding from the form.
    pub fn no_padding(mut self) -> Self {
        self.padding = None;
        self
    }

    /// Adds a child element to the form.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the form.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }

    /// Adds a section to the form with a header and grouped fields.
    pub fn section<F>(self, title: impl Into<SharedString>, build: F) -> Self
    where
        F: FnOnce(FormSection) -> FormSection,
    {
        let section = FormSection::new(title);
        let section = build(section);
        self.child(section)
    }
}

impl Default for Form {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoElement for Form {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        // Form background
        let bg_color = hsla(0.0, 0.0, 0.98, 1.0);

        let mut container = div()
            .flex()
            .flex_col()
            .gap(self.spacing)
            .bg(bg_color)
            .rounded(px(6.0));

        // Apply padding if set
        if let Some(padding) = self.padding {
            container = container.p(padding);
        }

        // Add all children
        for child in self.children {
            container = container.child(child);
        }

        container
    }
}

/// A section within a form that groups related fields.
///
/// Sections provide visual grouping with a header and can contain
/// multiple form fields.
///
/// # Example
///
/// ```ignore
/// FormSection::new("Account Settings")
///     .child(TextField::new("email", cx).label("Email"))
///     .child(SecureField::new("password", cx).label("Password"))
/// ```
pub struct FormSection {
    title: SharedString,
    children: Vec<AnyElement>,
    spacing: Pixels,
    collapsed: bool,
}

impl FormSection {
    /// Creates a new form section with the given title.
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
            spacing: px(12.0), // Slightly tighter spacing within sections
            collapsed: false,
        }
    }

    /// Sets the vertical spacing between fields in this section.
    pub fn spacing(mut self, spacing: impl Into<Pixels>) -> Self {
        self.spacing = spacing.into();
        self
    }

    /// Sets whether the section is initially collapsed.
    pub fn collapsed(mut self, collapsed: bool) -> Self {
        self.collapsed = collapsed;
        self
    }

    /// Adds a child element to the section.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the section.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl IntoElement for FormSection {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        // Section header styling
        let header_color = hsla(0.0, 0.0, 0.40, 1.0);
        let divider_color = hsla(0.0, 0.0, 0.85, 1.0);

        // Build the section header
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(8.0))
            .pb(px(6.0))
            .child(
                div()
                    .text_xs()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(header_color)
                    .child(self.title.to_uppercase()),
            )
            .child(
                div()
                    .flex_1()
                    .h(px(1.0))
                    .bg(divider_color),
            );

        // Build the content container
        let mut content = div()
            .flex()
            .flex_col()
            .gap(self.spacing)
            .pl(px(0.0)); // Optional: indent section content

        if !self.collapsed {
            for child in self.children {
                content = content.child(child);
            }
        }

        div()
            .flex()
            .flex_col()
            .gap(px(8.0))
            .child(header)
            .child(content)
    }
}

/// A form row for label-control pairs with consistent alignment.
///
/// Use FormRow when you need labels aligned in a grid pattern
/// (labels on left, controls on right).
///
/// # Example
///
/// ```ignore
/// FormRow::new("Name")
///     .child(TextField::new("name", cx))
/// ```
pub struct FormRow {
    label: SharedString,
    label_width: Option<Pixels>,
    children: Vec<AnyElement>,
}

impl FormRow {
    /// Creates a new form row with the given label.
    pub fn new(label: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            label_width: Some(px(120.0)), // Default label width
            children: Vec::new(),
        }
    }

    /// Sets a custom width for the label.
    pub fn label_width(mut self, width: impl Into<Pixels>) -> Self {
        self.label_width = Some(width.into());
        self
    }

    /// Adds a child control to the row.
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    /// Adds multiple children to the row.
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(|c| c.into_any_element()));
        self
    }
}

impl IntoElement for FormRow {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let label_color = hsla(0.0, 0.0, 0.30, 1.0);

        // Build the label
        let mut label = div()
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .text_color(label_color)
            .flex_shrink_0();

        if let Some(width) = self.label_width {
            label = label.w(width);
        }

        label = label.child(self.label);

        // Build the control container
        let mut control_container = div().flex_1().flex().flex_col().gap(px(4.0));

        for child in self.children {
            control_container = control_container.child(child);
        }

        // Build the row
        div()
            .flex()
            .flex_row()
            .items_start()
            .gap(px(12.0))
            .child(label)
            .child(control_container)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_form_creation() {
        let form = Form::new();
        assert_eq!(form.spacing, px(16.0));
        assert!(form.padding.is_some());
    }

    #[test]
    fn test_form_no_padding() {
        let form = Form::new().no_padding();
        assert!(form.padding.is_none());
    }

    #[test]
    fn test_form_custom_spacing() {
        let form = Form::new().spacing(px(24.0));
        assert_eq!(form.spacing, px(24.0));
    }

    #[test]
    fn test_form_section_creation() {
        let section = FormSection::new("Test Section");
        assert_eq!(section.title.as_ref(), "Test Section");
        assert!(!section.collapsed);
    }

    #[test]
    fn test_form_section_collapsed() {
        let section = FormSection::new("Test").collapsed(true);
        assert!(section.collapsed);
    }

    #[test]
    fn test_form_row_creation() {
        let row = FormRow::new("Label");
        assert_eq!(row.label.as_ref(), "Label");
        assert!(row.label_width.is_some());
    }

    #[test]
    fn test_form_row_custom_width() {
        let row = FormRow::new("Label").label_width(px(200.0));
        assert_eq!(row.label_width, Some(px(200.0)));
    }
}
