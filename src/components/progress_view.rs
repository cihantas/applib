//! Progress indicator component.
//!
//! A view showing progress of a task, equivalent to SwiftUI's ProgressView.
//! Supports both indeterminate (spinning) and determinate (progress bar) styles.

use gpui::prelude::*;
use gpui::*;

/// Style variants for the progress indicator.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum ProgressStyle {
    /// Horizontal progress bar.
    #[default]
    Linear,
    /// Circular progress indicator.
    Circular,
}

/// A progress indicator component.
///
/// ProgressView can show either determinate progress (a percentage from 0.0 to 1.0)
/// or indeterminate progress (for operations with unknown duration).
///
/// # Example
///
/// ```ignore
/// // Indeterminate progress (spinning)
/// ProgressView::indeterminate()
///     .style(ProgressStyle::Circular)
///
/// // Determinate progress (50%)
/// ProgressView::new(0.5)
///     .label("Downloading...")
///
/// // With total value
/// ProgressView::new_with_total(50.0, 100.0)
///     .style(ProgressStyle::Linear)
/// ```
pub struct ProgressView {
    /// Progress value from 0.0 to 1.0, or None for indeterminate.
    value: Option<f32>,
    /// Optional label to display.
    label: Option<SharedString>,
    /// Visual style (linear or circular).
    style: ProgressStyle,
}

impl ProgressView {
    /// Creates a new determinate progress view with the given value (0.0 to 1.0).
    pub fn new(value: f32) -> Self {
        Self {
            value: Some(value.clamp(0.0, 1.0)),
            label: None,
            style: ProgressStyle::default(),
        }
    }

    /// Creates a new determinate progress view with value and total.
    pub fn new_with_total(value: f32, total: f32) -> Self {
        let progress = if total > 0.0 { value / total } else { 0.0 };
        Self::new(progress)
    }

    /// Creates a new indeterminate progress view.
    pub fn indeterminate() -> Self {
        Self {
            value: None,
            label: None,
            style: ProgressStyle::default(),
        }
    }

    /// Sets the visual style of the progress view.
    pub fn style(mut self, style: ProgressStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets an optional label to display alongside the progress indicator.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Renders a linear (bar) progress indicator.
    fn render_linear(&self) -> Div {
        let track_color = hsla(0.0, 0.0, 0.85, 1.0); // Light gray track
        let fill_color = hsla(211.0 / 360.0, 0.95, 0.53, 1.0); // Blue fill
        let indeterminate_color = hsla(211.0 / 360.0, 0.70, 0.60, 1.0); // Lighter blue for indeterminate

        let bar_height = px(6.0);
        let border_radius = px(3.0);

        let track = div()
            .w_full()
            .h(bar_height)
            .rounded(border_radius)
            .bg(track_color)
            .overflow_hidden();

        let fill = match self.value {
            Some(progress) => {
                // Determinate: show filled portion
                div()
                    .h_full()
                    .rounded(border_radius)
                    .bg(fill_color)
                    .w(relative(progress))
            }
            None => {
                // Indeterminate: show animated stripe pattern
                // Since GPUI doesn't have built-in animations, we show a static stripe
                div()
                    .h_full()
                    .rounded(border_radius)
                    .bg(indeterminate_color)
                    .w(relative(0.3)) // 30% width stripe
            }
        };

        track.child(fill)
    }

    /// Renders a circular progress indicator.
    fn render_circular(&self) -> Div {
        let size = px(20.0);
        let track_color = hsla(0.0, 0.0, 0.85, 1.0);
        let fill_color = hsla(211.0 / 360.0, 0.95, 0.53, 1.0);

        match self.value {
            Some(progress) => {
                // Determinate circular: show as a filled circle proportional to progress
                // Since GPUI doesn't support circular progress natively, we approximate
                // with a simple circular indicator
                let indicator_size = size * progress;
                div()
                    .size(size)
                    .rounded_full()
                    .bg(track_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .size(indicator_size)
                            .rounded_full()
                            .bg(fill_color),
                    )
            }
            None => {
                // Indeterminate: show a static indicator (animation not supported)
                // Represented as a partial arc using an inner offset circle
                div()
                    .size(size)
                    .rounded_full()
                    .bg(track_color)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .size(size - px(4.0))
                            .rounded_full()
                            .bg(fill_color),
                    )
            }
        }
    }
}

impl Default for ProgressView {
    fn default() -> Self {
        Self::indeterminate()
    }
}

impl IntoElement for ProgressView {
    type Element = Div;

    fn into_element(self) -> Self::Element {
        let indicator = match self.style {
            ProgressStyle::Linear => self.render_linear(),
            ProgressStyle::Circular => self.render_circular(),
        };

        match self.label {
            Some(label_text) => {
                // Container with label and indicator
                div()
                    .flex()
                    .flex_col()
                    .gap_1()
                    .child(
                        div()
                            .text_sm()
                            .text_color(hsla(0.0, 0.0, 0.40, 1.0)) // Secondary text
                            .child(label_text),
                    )
                    .child(indicator)
            }
            None => indicator,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_determinate_progress() {
        let progress = ProgressView::new(0.5);
        assert_eq!(progress.value, Some(0.5));
        assert_eq!(progress.style, ProgressStyle::Linear);
    }

    #[test]
    fn test_indeterminate_progress() {
        let progress = ProgressView::indeterminate();
        assert_eq!(progress.value, None);
    }

    #[test]
    fn test_progress_with_total() {
        let progress = ProgressView::new_with_total(25.0, 100.0);
        assert_eq!(progress.value, Some(0.25));
    }

    #[test]
    fn test_progress_clamping() {
        let progress = ProgressView::new(1.5);
        assert_eq!(progress.value, Some(1.0));

        let progress = ProgressView::new(-0.5);
        assert_eq!(progress.value, Some(0.0));
    }

    #[test]
    fn test_style_setting() {
        let progress = ProgressView::indeterminate().style(ProgressStyle::Circular);
        assert_eq!(progress.style, ProgressStyle::Circular);
    }

    #[test]
    fn test_label_setting() {
        let progress = ProgressView::new(0.5).label("Loading...");
        assert_eq!(progress.label, Some("Loading...".into()));
    }

    #[test]
    fn test_zero_total() {
        let progress = ProgressView::new_with_total(50.0, 0.0);
        assert_eq!(progress.value, Some(0.0));
    }
}
