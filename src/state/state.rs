//! Observable state primitive for reactive UI.
//!
//! `State<T>` wraps a value and automatically notifies GPUI when it changes,
//! triggering UI re-renders. This eliminates manual `cx.notify()` calls.
//!
//! # Example
//!
//! ```ignore
//! use applib::prelude::*;
//!
//! struct MyView {
//!     counter: Entity<State<i32>>,
//! }
//!
//! impl MyView {
//!     fn new(cx: &mut App) -> Self {
//!         Self {
//!             counter: cx.new(|_| State::new(0)),
//!         }
//!     }
//!
//!     fn increment(&mut self, cx: &mut App) {
//!         self.counter.update(cx, |state, cx| {
//!             state.set(state.get() + 1, cx);  // Auto-notifies
//!         });
//!     }
//! }
//! ```

use gpui::{App, Context, Entity};

use super::Binding;

/// Observable state container that auto-notifies on changes.
///
/// `State<T>` is designed to be used with GPUI's `Entity` system.
/// When you call `set()` or `update()`, it automatically calls `cx.notify()`
/// to trigger a re-render of any views observing this state.
#[derive(Debug)]
pub struct State<T> {
    value: T,
}

impl<T> State<T> {
    /// Creates a new state with the given initial value.
    pub fn new(value: T) -> Self {
        Self { value }
    }

    /// Returns a reference to the current value.
    pub fn get(&self) -> &T {
        &self.value
    }

    /// Returns a mutable reference to the current value.
    ///
    /// Note: This does NOT trigger a notification. Use `set()` or `update()`
    /// if you want automatic re-renders.
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }
}

impl<T: 'static> State<T> {
    /// Sets a new value and notifies observers.
    ///
    /// This automatically calls `cx.notify()` to trigger re-renders.
    pub fn set(&mut self, value: T, cx: &mut Context<Self>) {
        self.value = value;
        cx.notify();
    }

    /// Updates the value using a closure and notifies observers.
    ///
    /// This is useful for modifying the value in place.
    ///
    /// # Example
    ///
    /// ```ignore
    /// state.update(|value| *value += 1, cx);
    /// ```
    pub fn update(&mut self, f: impl FnOnce(&mut T), cx: &mut Context<Self>) {
        f(&mut self.value);
        cx.notify();
    }
}

impl<T: Clone + 'static> State<T> {
    /// Creates a two-way binding to this state.
    ///
    /// The binding can be passed to components that support it,
    /// enabling automatic synchronization between the component and this state.
    ///
    /// # Example
    ///
    /// ```ignore
    /// let query: Entity<State<String>> = cx.new(|_| State::new(String::new()));
    ///
    /// TextField::new("search")
    ///     .text(query.binding(cx))  // Two-way binding
    /// ```
    pub fn binding(entity: &Entity<Self>, cx: &App) -> Binding<T> {
        Binding::new(entity.clone(), cx)
    }
}

impl<T: Default> Default for State<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T: Clone> Clone for State<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
        }
    }
}
