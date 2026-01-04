//! Label component combining an icon and text.
//!
//! This module provides a label component similar to SwiftUI's Label,
//! combining an icon with text in a horizontal layout.

use gpui::prelude::*;
use gpui::*;

/// Common icons for use with Label and other components.
///
/// These map to Unicode characters or emoji that work well as icons.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Icon {
    /// Heart icon - for favorites
    Heart,
    /// Star icon - for ratings/starred items
    Star,
    /// Folder icon - for directories
    Folder,
    /// Document/file icon
    Document,
    /// Gear/settings icon
    Gear,
    /// Pencil/edit icon
    Pencil,
    /// Trash/delete icon
    Trash,
    /// Plus/add icon
    Plus,
    /// Minus/remove icon
    Minus,
    /// Checkmark icon
    Checkmark,
    /// X/close icon
    XMark,
    /// Search/magnifying glass icon
    MagnifyingGlass,
    /// Person/user icon
    Person,
    /// Clock/time icon
    Clock,
    /// Tag icon
    Tag,
    /// Branch icon - for git branches
    Branch,
    /// Commit icon - for git commits
    Commit,
    /// Arrow up icon
    ArrowUp,
    /// Arrow down icon
    ArrowDown,
    /// Arrow left icon
    ArrowLeft,
    /// Arrow right icon
    ArrowRight,
    /// Refresh/reload icon
    Refresh,
    /// Info icon
    Info,
    /// Warning icon
    Warning,
    /// Error/exclamation icon
    Error,
}

impl Icon {
    /// Returns the Unicode character representation of this icon.
    pub fn as_str(&self) -> &'static str {
        match self {
            Icon::Heart => "♥",
            Icon::Star => "★",
            Icon::Folder => "📁",
            Icon::Document => "📄",
            Icon::Gear => "⚙",
            Icon::Pencil => "✎",
            Icon::Trash => "🗑",
            Icon::Plus => "+",
            Icon::Minus => "−",
            Icon::Checkmark => "✓",
            Icon::XMark => "✕",
            Icon::MagnifyingGlass => "🔍",
            Icon::Person => "👤",
            Icon::Clock => "🕐",
            Icon::Tag => "🏷",
            Icon::Branch => "⑂",
            Icon::Commit => "●",
            Icon::ArrowUp => "↑",
            Icon::ArrowDown => "↓",
            Icon::ArrowLeft => "←",
            Icon::ArrowRight => "→",
            Icon::Refresh => "↻",
            Icon::Info => "ℹ",
            Icon::Warning => "⚠",
            Icon::Error => "⊘",
        }
    }
}

/// Display style for labels.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum LabelStyle {
    /// Show both icon and title (default)
    #[default]
    Both,
    /// Show only the icon
    IconOnly,
    /// Show only the title
    TitleOnly,
}

/// A label component combining an icon and text.
///
/// Similar to SwiftUI's Label, this component displays an icon alongside
/// text in a consistent horizontal layout. It can be configured to show
/// just the icon, just the title, or both.
///
/// # Example
///
/// ```ignore
/// // Basic label with icon and text
/// Label::new("Favorites", Icon::Heart)
///
/// // With custom icon character
/// Label::with_icon_str("Custom", "🎨")
///
/// // Icon only
/// Label::new("Edit", Icon::Pencil).icon_only()
///
/// // Title only
/// Label::new("Settings", Icon::Gear).title_only()
/// ```
pub struct Label {
    title: SharedString,
    icon: Option<SharedString>,
    style: LabelStyle,
    text_color: Option<Hsla>,
    icon_color: Option<Hsla>,
}

impl Label {
    /// Creates a new label with a title and icon.
    pub fn new(title: impl Into<SharedString>, icon: Icon) -> Self {
        Self {
            title: title.into(),
            icon: Some(icon.as_str().into()),
            style: LabelStyle::default(),
            text_color: None,
            icon_color: None,
        }
    }

    /// Creates a new label with a custom icon string.
    ///
    /// Use this when you need an icon that isn't in the Icon enum.
    pub fn with_icon_str(title: impl Into<SharedString>, icon: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: Some(icon.into()),
            style: LabelStyle::default(),
            text_color: None,
            icon_color: None,
        }
    }

    /// Creates a label with only a title (no icon).
    pub fn title_only_new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            icon: None,
            style: LabelStyle::TitleOnly,
            text_color: None,
            icon_color: None,
        }
    }

    /// Sets the label style.
    pub fn style(mut self, style: LabelStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the label to show only the icon.
    pub fn icon_only(mut self) -> Self {
        self.style = LabelStyle::IconOnly;
        self
    }

    /// Sets the label to show only the title.
    pub fn title_only(mut self) -> Self {
        self.style = LabelStyle::TitleOnly;
        self
    }

    /// Sets the text color for both icon and title.
    pub fn text_color(mut self, color: Hsla) -> Self {
        self.text_color = Some(color);
        self.icon_color = Some(color);
        self
    }

    /// Sets a separate color for the icon.
    pub fn icon_color(mut self, color: Hsla) -> Self {
        self.icon_color = Some(color);
        self
    }
}

impl IntoElement for Label {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let default_text_color = hsla(0.0, 0.0, 0.20, 1.0);
        let text_color = self.text_color.unwrap_or(default_text_color);
        let icon_color = self.icon_color.unwrap_or(text_color);

        let mut container = div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0));

        // Add icon if showing
        if self.style != LabelStyle::TitleOnly {
            if let Some(icon) = self.icon {
                container = container.child(
                    div()
                        .text_color(icon_color)
                        .child(icon),
                );
            }
        }

        // Add title if showing
        if self.style != LabelStyle::IconOnly {
            container = container.child(
                div()
                    .text_color(text_color)
                    .child(self.title),
            );
        }

        container
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_label_creation() {
        let label = Label::new("Favorites", Icon::Heart);
        assert_eq!(label.style, LabelStyle::Both);
        assert!(label.icon.is_some());
    }

    #[test]
    fn test_label_icon_only() {
        let label = Label::new("Edit", Icon::Pencil).icon_only();
        assert_eq!(label.style, LabelStyle::IconOnly);
    }

    #[test]
    fn test_label_title_only() {
        let label = Label::new("Settings", Icon::Gear).title_only();
        assert_eq!(label.style, LabelStyle::TitleOnly);
    }

    #[test]
    fn test_label_with_custom_icon() {
        let label = Label::with_icon_str("Custom", "🎨");
        assert_eq!(label.icon, Some("🎨".into()));
    }

    #[test]
    fn test_icon_as_str() {
        assert_eq!(Icon::Heart.as_str(), "♥");
        assert_eq!(Icon::Plus.as_str(), "+");
        assert_eq!(Icon::Checkmark.as_str(), "✓");
    }
}
