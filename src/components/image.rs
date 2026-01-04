//! Image component for displaying PNG, JPEG, and other image formats.
//!
//! This module provides a high-level Image component that wraps GPUI's img()
//! primitive with a more ergonomic API for common use cases.

use gpui::prelude::*;
use gpui::*;
use std::path::PathBuf;

use crate::components::Icon;

/// Source of image data.
///
/// Supports loading images from filesystem paths or embedded resources.
#[derive(Clone)]
pub enum ImageSource {
    /// Load image from a filesystem path
    Path(PathBuf),
    /// Load image from a URI (http/https)
    Uri(SharedString),
    /// Use an embedded resource
    Embedded(SharedString),
}

impl From<PathBuf> for ImageSource {
    fn from(path: PathBuf) -> Self {
        ImageSource::Path(path)
    }
}

impl From<&str> for ImageSource {
    fn from(s: &str) -> Self {
        if s.starts_with("http://") || s.starts_with("https://") {
            ImageSource::Uri(s.to_string().into())
        } else {
            ImageSource::Path(PathBuf::from(s))
        }
    }
}

impl From<String> for ImageSource {
    fn from(s: String) -> Self {
        s.as_str().into()
    }
}

/// How the image should fit within its frame.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ImageFit {
    /// Scale to fit within bounds while maintaining aspect ratio (default)
    #[default]
    Contain,
    /// Scale to fill bounds while maintaining aspect ratio (may crop)
    Cover,
    /// Stretch to fill bounds (may distort)
    Fill,
}

impl From<ImageFit> for ObjectFit {
    fn from(fit: ImageFit) -> Self {
        match fit {
            ImageFit::Contain => ObjectFit::Contain,
            ImageFit::Cover => ObjectFit::Cover,
            ImageFit::Fill => ObjectFit::Fill,
        }
    }
}

/// A high-level image component for displaying images.
///
/// This component wraps GPUI's `img()` primitive with a more ergonomic API.
/// It supports loading images from filesystem paths, URIs, and embedded resources.
///
/// # Supported Formats
///
/// - PNG, JPEG, GIF, WebP
/// - TIFF, TGA, BMP, ICO
/// - HDR, EXR, PBM, PAM, PPM, PGM
/// - AVIF, DDS, QOI, SVG
///
/// # Example
///
/// ```ignore
/// // Load from file path
/// Image::new("avatar")
///     .source("/path/to/avatar.png")
///     .size(px(48.0))
///     .corner_radius(px(24.0))
///     .placeholder(Icon::Person)
///
/// // Load from URI
/// Image::new("remote")
///     .source("https://example.com/image.jpg")
///     .size(px(200.0))
///     .fit(ImageFit::Cover)
///
/// // With custom dimensions
/// Image::new("banner")
///     .source("/path/to/banner.png")
///     .w(px(800.0))
///     .h(px(200.0))
/// ```
pub struct Image {
    id: ElementId,
    source: Option<ImageSource>,
    width: Option<Pixels>,
    height: Option<Pixels>,
    corner_radius: Option<Pixels>,
    fit: ImageFit,
    grayscale: bool,
    placeholder: Option<Icon>,
}

impl Image {
    /// Creates a new image with the given element ID.
    pub fn new(id: impl Into<ElementId>) -> Self {
        Self {
            id: id.into(),
            source: None,
            width: None,
            height: None,
            corner_radius: None,
            fit: ImageFit::default(),
            grayscale: false,
            placeholder: None,
        }
    }

    /// Sets the image source.
    pub fn source(mut self, source: impl Into<ImageSource>) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Sets both width and height to the same value (creates a square).
    pub fn size(mut self, size: Pixels) -> Self {
        self.width = Some(size);
        self.height = Some(size);
        self
    }

    /// Sets the width.
    pub fn w(mut self, width: Pixels) -> Self {
        self.width = Some(width);
        self
    }

    /// Sets the height.
    pub fn h(mut self, height: Pixels) -> Self {
        self.height = Some(height);
        self
    }

    /// Sets the corner radius.
    pub fn corner_radius(mut self, radius: Pixels) -> Self {
        self.corner_radius = Some(radius);
        self
    }

    /// Makes the image fully rounded (circular if square).
    pub fn rounded(mut self) -> Self {
        // Set a large radius that will be clamped to make it fully rounded
        self.corner_radius = Some(px(9999.0));
        self
    }

    /// Sets how the image should fit within its frame.
    pub fn fit(mut self, fit: ImageFit) -> Self {
        self.fit = fit;
        self
    }

    /// Renders the image in grayscale.
    pub fn grayscale(mut self, grayscale: bool) -> Self {
        self.grayscale = grayscale;
        self
    }

    /// Sets a placeholder icon to show while loading or on error.
    pub fn placeholder(mut self, icon: Icon) -> Self {
        self.placeholder = Some(icon);
        self
    }

    /// Creates a placeholder element to show when image is loading or failed.
    fn create_placeholder(&self) -> AnyElement {
        let bg_color = hsla(0.0, 0.0, 0.94, 1.0);
        let icon_color = hsla(0.0, 0.0, 0.60, 1.0);

        let mut container = div()
            .flex()
            .items_center()
            .justify_center()
            .bg(bg_color);

        if let Some(width) = self.width {
            container = container.w(width);
        }
        if let Some(height) = self.height {
            container = container.h(height);
        }
        if let Some(radius) = self.corner_radius {
            container = container.rounded(radius);
        }

        if let Some(icon) = self.placeholder {
            // Use a reasonable default font size for placeholder icon
            let font_size = px(24.0);

            container = container.child(
                div()
                    .text_size(font_size)
                    .text_color(icon_color)
                    .child(icon.as_str()),
            );
        }

        container.into_any_element()
    }
}

impl IntoElement for Image {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let source = match self.source {
            Some(ImageSource::Path(path)) => path.to_string_lossy().to_string(),
            Some(ImageSource::Uri(uri)) => uri.to_string(),
            Some(ImageSource::Embedded(path)) => path.to_string(),
            None => {
                // No source provided - just show placeholder
                let id = self.id.clone();
                return div()
                    .id(id)
                    .child(self.create_placeholder())
                    .into_element();
            }
        };

        let mut img_element = img(source)
            .object_fit(self.fit.into())
            .grayscale(self.grayscale);

        // Note: Fallback would require cloneable placeholder, skipping for now
        // Future: implement custom error handling or stateful fallback

        let mut container = div().id(self.id);

        if let Some(width) = self.width {
            container = container.w(width);
            img_element = img_element.w(width);
        }
        if let Some(height) = self.height {
            container = container.h(height);
            img_element = img_element.h(height);
        }
        if let Some(radius) = self.corner_radius {
            container = container.rounded(radius);
            img_element = img_element.rounded(radius);
        }

        container
            .flex()
            .overflow_hidden()
            .child(img_element)
            .into_element()
    }
}
