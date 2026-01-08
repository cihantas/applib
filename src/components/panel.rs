//! High-level Panel API for creating floating windows.
//!
//! This module provides a builder pattern API for creating floating panels (windows)
//! with GPUI, making it easy to create launchers, dialogs, and other popup interfaces.

use gpui::*;
use std::marker::PhantomData;
use anyhow::Result;

/// A builder for creating floating panel windows.
///
/// Panel provides a high-level API that wraps GPUI's window creation to make
/// floating panels easy to create with a fluent builder pattern.
///
/// # Example
///
/// ```ignore
/// Panel::new("launcher", |cx| LauncherView::new(cx))
///     .size(px(600.0), px(400.0))
///     .center_on_screen()
///     .floating(true)
///     .hide_titlebar()
///     .open(cx)
/// ```
/// The appearance of the window background.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum PanelBackground {
    /// Opaque background (default).
    #[default]
    Opaque,
    /// Transparent background with alpha.
    Transparent,
    /// Transparent with blur effect (may not be supported on all platforms).
    Blurred,
}

pub struct Panel<V, F>
where
    V: Render + 'static,
    F: FnOnce(&mut App) -> V + 'static,
{
    #[allow(dead_code)]
    id: &'static str,
    view_builder: F,
    width: Pixels,
    height: Pixels,
    centered: bool,
    floating: bool,
    titlebar_hidden: bool,
    background: PanelBackground,
    _phantom: PhantomData<V>,
}

impl<V, F> Panel<V, F>
where
    V: Render + 'static,
    F: FnOnce(&mut App) -> V + 'static,
{
    /// Creates a new panel builder with the given id and view builder function.
    ///
    /// The view builder function is called when the panel is opened to create
    /// the root view for the window.
    ///
    /// # Arguments
    ///
    /// * `id` - A static string identifier for the panel (used for debugging)
    /// * `view_builder` - A closure that creates the root view for the panel
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("my-panel", |cx| MyPanelView::new(cx))
    /// ```
    pub fn new(id: &'static str, view_builder: F) -> Self {
        Self {
            id,
            view_builder,
            width: px(400.0),
            height: px(300.0),
            centered: false,
            floating: true,
            titlebar_hidden: false,
            background: PanelBackground::default(),
            _phantom: PhantomData,
        }
    }

    /// Sets the size of the panel window.
    ///
    /// # Arguments
    ///
    /// * `width` - The width of the panel in pixels
    /// * `height` - The height of the panel in pixels
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("panel", |cx| view)
    ///     .size(px(600.0), px(400.0))
    /// ```
    pub fn size(mut self, width: Pixels, height: Pixels) -> Self {
        self.width = width;
        self.height = height;
        self
    }

    /// Centers the panel on the screen when opened.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("panel", |cx| view)
    ///     .center_on_screen()
    /// ```
    pub fn center_on_screen(mut self) -> Self {
        self.centered = true;
        self
    }

    /// Sets whether the panel should be a floating window.
    ///
    /// When `true`, the panel uses `WindowKind::PopUp` which makes it float
    /// above other windows. When `false`, it uses `WindowKind::Normal`.
    ///
    /// Defaults to `true`.
    ///
    /// # Arguments
    ///
    /// * `floating` - Whether the panel should float above other windows
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("panel", |cx| view)
    ///     .floating(true)
    /// ```
    pub fn floating(mut self, floating: bool) -> Self {
        self.floating = floating;
        self
    }

    /// Hides the system titlebar for the panel.
    ///
    /// When called, the panel will use client-side decorations with no titlebar,
    /// allowing for custom window chrome.
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("panel", |cx| view)
    ///     .hide_titlebar()
    /// ```
    pub fn hide_titlebar(mut self) -> Self {
        self.titlebar_hidden = true;
        self
    }

    /// Sets the window background appearance.
    ///
    /// - `Opaque`: Standard opaque window (default)
    /// - `Transparent`: Transparent background with alpha
    /// - `Blurred`: Transparent with blur effect (platform-dependent)
    ///
    /// # Example
    ///
    /// ```ignore
    /// Panel::new("panel", |cx| view)
    ///     .background(PanelBackground::Blurred)
    /// ```
    pub fn background(mut self, background: PanelBackground) -> Self {
        self.background = background;
        self
    }

    /// Opens the panel window with the configured options.
    ///
    /// This consumes the builder and creates the window, returning a handle
    /// to the created window or an error if window creation fails.
    ///
    /// # Arguments
    ///
    /// * `cx` - The application context
    ///
    /// # Returns
    ///
    /// A `Result` containing the `WindowHandle` on success, or an error on failure.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let handle = Panel::new("launcher", |cx| LauncherView::new(cx))
    ///     .size(px(600.0), px(400.0))
    ///     .center_on_screen()
    ///     .floating(true)
    ///     .hide_titlebar()
    ///     .open(cx)?;
    /// ```
    pub fn open(self, cx: &mut App) -> Result<WindowHandle<V>> {
        let window_size = size(self.width, self.height);

        let bounds = if self.centered {
            Bounds::centered(None, window_size, cx)
        } else {
            Bounds::new(Point::default(), window_size)
        };

        let window_background = match self.background {
            PanelBackground::Opaque => WindowBackgroundAppearance::Opaque,
            PanelBackground::Transparent => WindowBackgroundAppearance::Transparent,
            PanelBackground::Blurred => WindowBackgroundAppearance::Blurred,
        };

        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            kind: if self.floating {
                WindowKind::PopUp
            } else {
                WindowKind::Normal
            },
            titlebar: if self.titlebar_hidden {
                None
            } else {
                Some(TitlebarOptions::default())
            },
            window_decorations: if self.titlebar_hidden {
                Some(WindowDecorations::Client)
            } else {
                None
            },
            window_background,
            focus: true,
            ..Default::default()
        };

        // Store the view builder to move into the closure
        let view_builder = self.view_builder;

        cx.open_window(options, |_, cx| cx.new(|cx| (view_builder)(cx)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Note: Full integration tests require a running GPUI application context,
    // so we only test the builder configuration here.

    struct MockView;

    impl Render for MockView {
        fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
            div()
        }
    }

    #[test]
    fn test_panel_defaults() {
        let panel = Panel::new("test", |_cx| MockView);
        assert_eq!(panel.width, px(400.0));
        assert_eq!(panel.height, px(300.0));
        assert!(!panel.centered);
        assert!(panel.floating);
        assert!(!panel.titlebar_hidden);
    }

    #[test]
    fn test_panel_size() {
        let panel = Panel::new("test", |_cx| MockView).size(px(800.0), px(600.0));
        assert_eq!(panel.width, px(800.0));
        assert_eq!(panel.height, px(600.0));
    }

    #[test]
    fn test_panel_center_on_screen() {
        let panel = Panel::new("test", |_cx| MockView).center_on_screen();
        assert!(panel.centered);
    }

    #[test]
    fn test_panel_floating() {
        let panel = Panel::new("test", |_cx| MockView).floating(false);
        assert!(!panel.floating);

        let panel = Panel::new("test", |_cx| MockView).floating(true);
        assert!(panel.floating);
    }

    #[test]
    fn test_panel_hide_titlebar() {
        let panel = Panel::new("test", |_cx| MockView).hide_titlebar();
        assert!(panel.titlebar_hidden);
    }

    #[test]
    fn test_panel_builder_chain() {
        let panel = Panel::new("launcher", |_cx| MockView)
            .size(px(600.0), px(400.0))
            .center_on_screen()
            .floating(true)
            .hide_titlebar();

        assert_eq!(panel.width, px(600.0));
        assert_eq!(panel.height, px(400.0));
        assert!(panel.centered);
        assert!(panel.floating);
        assert!(panel.titlebar_hidden);
    }
}
