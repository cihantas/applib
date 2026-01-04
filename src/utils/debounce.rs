//! Debouncing utility for delaying callback execution.
//!
//! Debouncing is useful for high-frequency events like text input, window resizing,
//! or scroll events where you want to wait until the user has stopped interacting
//! before processing.
//!
//! # Example
//!
//! ```ignore
//! use applib::utils::Debouncer;
//!
//! struct MyView {
//!     query: String,
//!     debouncer: Debouncer,
//! }
//!
//! impl MyView {
//!     fn new() -> Self {
//!         Self {
//!             query: String::new(),
//!             debouncer: Debouncer::new(Duration::from_millis(150)),
//!         }
//!     }
//!
//!     fn on_text_change(&mut self, text: String, cx: &mut App) {
//!         self.query = text.clone();
//!         self.debouncer.call(cx, move |cx| {
//!             // This only fires 150ms after the last keystroke
//!             println!("Searching for: {}", text);
//!         });
//!     }
//! }
//! ```

use gpui::{App, Task};
use std::time::Duration;

/// A debouncer that delays callback execution until a specified time has passed
/// without new calls.
///
/// Each call to `call()` cancels any pending callback and starts a new timer.
/// The callback only executes if no new calls arrive within the delay period.
///
/// # Example
///
/// ```ignore
/// let mut debouncer = Debouncer::new(Duration::from_millis(150));
///
/// // In your event handler:
/// debouncer.call(cx, |cx| {
///     // This fires 150ms after the last call
///     perform_search(cx);
/// });
/// ```
pub struct Debouncer {
    delay: Duration,
    pending_task: Option<Task<()>>,
}

impl Debouncer {
    /// Creates a new debouncer with the specified delay.
    ///
    /// # Arguments
    ///
    /// * `delay` - The time to wait after the last call before executing the callback.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // 150ms delay, good for search input
    /// let debouncer = Debouncer::new(Duration::from_millis(150));
    ///
    /// // 100ms delay, good for resize handlers
    /// let debouncer = Debouncer::new(Duration::from_millis(100));
    /// ```
    pub fn new(delay: Duration) -> Self {
        Self {
            delay,
            pending_task: None,
        }
    }

    /// Creates a new debouncer with a delay specified in milliseconds.
    ///
    /// This is a convenience constructor for the common case.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let debouncer = Debouncer::from_millis(150);
    /// ```
    pub fn from_millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }

    /// Schedules a callback to execute after the delay.
    ///
    /// If there's already a pending callback, it will be cancelled and
    /// replaced with this new one. The callback will only execute if
    /// no new calls to `call()` arrive within the delay period.
    ///
    /// # Arguments
    ///
    /// * `cx` - The GPUI application context
    /// * `callback` - The function to execute after the delay
    ///
    /// # Example
    ///
    /// ```ignore
    /// debouncer.call(cx, |cx| {
    ///     // Expensive operation here
    ///     perform_search(cx);
    /// });
    /// ```
    pub fn call<F>(&mut self, cx: &mut App, callback: F)
    where
        F: FnOnce(&mut App) + 'static,
    {
        // Cancel any pending task by dropping it
        self.pending_task = None;

        let delay = self.delay;

        // Spawn a new task that waits for the delay then executes the callback
        self.pending_task = Some(cx.spawn(async move |cx| {
            // Wait for the delay period
            cx.background_executor().timer(delay).await;

            // Execute the callback on the main thread
            let _ = cx.update(|cx| {
                callback(cx);
            });
        }));
    }

    /// Cancels any pending callback.
    ///
    /// Use this when you want to stop a debounced operation from executing,
    /// for example when a component is being unmounted.
    ///
    /// # Example
    ///
    /// ```ignore
    /// // Cancel pending search when user clears the input
    /// if query.is_empty() {
    ///     debouncer.cancel();
    /// }
    /// ```
    pub fn cancel(&mut self) {
        self.pending_task = None;
    }

    /// Returns true if there's a pending callback waiting to execute.
    ///
    /// # Example
    ///
    /// ```ignore
    /// if debouncer.is_pending() {
    ///     show_loading_indicator();
    /// }
    /// ```
    pub fn is_pending(&self) -> bool {
        self.pending_task.is_some()
    }
}

impl Default for Debouncer {
    /// Creates a debouncer with a default delay of 150ms.
    ///
    /// This is a reasonable default for search input debouncing.
    fn default() -> Self {
        Self::from_millis(150)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_debouncer_creation() {
        let debouncer = Debouncer::new(Duration::from_millis(100));
        assert!(!debouncer.is_pending());
    }

    #[test]
    fn test_debouncer_from_millis() {
        let debouncer = Debouncer::from_millis(200);
        assert_eq!(debouncer.delay, Duration::from_millis(200));
    }

    #[test]
    fn test_debouncer_default() {
        let debouncer = Debouncer::default();
        assert_eq!(debouncer.delay, Duration::from_millis(150));
    }
}
