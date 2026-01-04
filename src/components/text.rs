//! Text component for displaying styled text content.
//!
//! This module provides a SwiftUI-like Text component with support for various
//! text styles, colors, weights, alignment, and line limits.

use gpui::prelude::*;
use gpui::*;

/// Text style variants for different typography contexts.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextStyle {
    /// Large title text (22px)
    Title,
    /// Headline text (15px bold)
    Headline,
    /// Subheadline text (13px)
    Subheadline,
    /// Body text (13px) - default
    #[default]
    Body,
    /// Caption text (11px)
    Caption,
    /// Footnote text (10px)
    Footnote,
}

/// Text alignment options.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum TextAlign {
    #[default]
    Left,
    Center,
    Right,
}

/// A view for displaying styled text content.
///
/// # Example
///
/// ```ignore
/// Text::new("Hello, World!")
///     .style(TextStyle::Headline)
///     .color(hsla(0.0, 0.0, 0.5, 1.0))
///     .weight(FontWeight::BOLD)
///     .align(TextAlign::Center)
///     .line_limit(2)
/// ```
pub struct Text {
    content: SharedString,
    style: TextStyle,
    color: Option<Hsla>,
    weight: Option<FontWeight>,
    align: TextAlign,
    line_limit: Option<usize>,
    monospace: bool,
}

impl Text {
    /// Creates a new Text view with the given content.
    pub fn new(content: impl Into<SharedString>) -> Self {
        Self {
            content: content.into(),
            style: TextStyle::default(),
            color: None,
            weight: None,
            align: TextAlign::default(),
            line_limit: None,
            monospace: false,
        }
    }

    /// Sets the text style.
    pub fn style(mut self, style: TextStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the text style to title.
    pub fn title(mut self) -> Self {
        self.style = TextStyle::Title;
        self
    }

    /// Sets the text style to headline.
    pub fn headline(mut self) -> Self {
        self.style = TextStyle::Headline;
        self
    }

    /// Sets the text style to subheadline.
    pub fn subheadline(mut self) -> Self {
        self.style = TextStyle::Subheadline;
        self
    }

    /// Sets the text style to body.
    pub fn body(mut self) -> Self {
        self.style = TextStyle::Body;
        self
    }

    /// Sets the text style to caption.
    pub fn caption(mut self) -> Self {
        self.style = TextStyle::Caption;
        self
    }

    /// Sets the text style to footnote.
    pub fn footnote(mut self) -> Self {
        self.style = TextStyle::Footnote;
        self
    }

    /// Sets the text color.
    pub fn color(mut self, color: Hsla) -> Self {
        self.color = Some(color);
        self
    }

    /// Sets the font weight.
    pub fn weight(mut self, weight: FontWeight) -> Self {
        self.weight = Some(weight);
        self
    }

    /// Sets the text alignment.
    pub fn align(mut self, align: TextAlign) -> Self {
        self.align = align;
        self
    }

    /// Sets the maximum number of lines to display.
    /// Text will be truncated with ellipsis if it exceeds this limit.
    pub fn line_limit(mut self, limit: usize) -> Self {
        self.line_limit = Some(limit);
        self
    }

    /// Sets the font to monospace.
    pub fn monospace(mut self) -> Self {
        self.monospace = true;
        self
    }

    /// Returns the font size in pixels for the current style.
    fn font_size(&self) -> Pixels {
        match self.style {
            TextStyle::Title => px(22.0),
            TextStyle::Headline => px(15.0),
            TextStyle::Subheadline => px(13.0),
            TextStyle::Body => px(13.0),
            TextStyle::Caption => px(11.0),
            TextStyle::Footnote => px(10.0),
        }
    }

    /// Returns the default font weight for the current style.
    fn default_weight(&self) -> FontWeight {
        match self.style {
            TextStyle::Title => FontWeight::BOLD,
            TextStyle::Headline => FontWeight::SEMIBOLD,
            TextStyle::Subheadline => FontWeight::MEDIUM,
            TextStyle::Body => FontWeight::NORMAL,
            TextStyle::Caption => FontWeight::NORMAL,
            TextStyle::Footnote => FontWeight::NORMAL,
        }
    }

    /// Returns the default text color (dark gray).
    fn default_color(&self) -> Hsla {
        hsla(0.0, 0.0, 0.20, 1.0)
    }
}

impl IntoElement for Text {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let font_size = self.font_size();
        let weight = self.weight.unwrap_or_else(|| self.default_weight());
        let color = self.color.unwrap_or_else(|| self.default_color());

        let mut element = div()
            .text_size(font_size)
            .font_weight(weight)
            .text_color(color);

        // Apply alignment
        element = match self.align {
            TextAlign::Left => element,
            TextAlign::Center => element.text_center(),
            TextAlign::Right => element.text_right(),
        };

        // Apply monospace font
        if self.monospace {
            element = element.font_family("monospace");
        }

        // Apply line limit with truncation
        if let Some(limit) = self.line_limit {
            element = element
                .overflow_hidden()
                .text_ellipsis()
                .max_h(font_size * (limit as f32) * 1.4); // 1.4 line height factor
        }

        element.child(self.content)
    }
}

// Common color helpers for text

/// Primary text color (dark gray).
pub fn text_primary() -> Hsla {
    hsla(0.0, 0.0, 0.20, 1.0)
}

/// Secondary text color (medium gray).
pub fn text_secondary() -> Hsla {
    hsla(0.0, 0.0, 0.50, 1.0)
}

/// Tertiary text color (light gray).
pub fn text_tertiary() -> Hsla {
    hsla(0.0, 0.0, 0.65, 1.0)
}

/// Accent text color (muted blue).
pub fn text_accent() -> Hsla {
    hsla(211.0 / 360.0, 0.30, 0.50, 1.0)
}

/// Link text color (blue).
pub fn text_link() -> Hsla {
    hsla(211.0 / 360.0, 0.95, 0.53, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_creation() {
        let text = Text::new("Hello");
        assert_eq!(text.style, TextStyle::Body);
        assert_eq!(text.align, TextAlign::Left);
        assert!(text.color.is_none());
        assert!(text.weight.is_none());
        assert!(text.line_limit.is_none());
    }

    #[test]
    fn test_text_styles() {
        let text = Text::new("Test").headline();
        assert_eq!(text.style, TextStyle::Headline);

        let text = Text::new("Test").title();
        assert_eq!(text.style, TextStyle::Title);

        let text = Text::new("Test").caption();
        assert_eq!(text.style, TextStyle::Caption);
    }

    #[test]
    fn test_text_font_size() {
        assert_eq!(Text::new("").title().font_size(), px(22.0));
        assert_eq!(Text::new("").headline().font_size(), px(15.0));
        assert_eq!(Text::new("").body().font_size(), px(13.0));
        assert_eq!(Text::new("").caption().font_size(), px(11.0));
        assert_eq!(Text::new("").footnote().font_size(), px(10.0));
    }

    #[test]
    fn test_text_alignment() {
        let text = Text::new("Test").align(TextAlign::Center);
        assert_eq!(text.align, TextAlign::Center);
    }

    #[test]
    fn test_text_line_limit() {
        let text = Text::new("Test").line_limit(2);
        assert_eq!(text.line_limit, Some(2));
    }
}
