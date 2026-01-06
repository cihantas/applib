//! Two-way binding primitive for reactive UI components.
//!
//! `Binding<T>` provides a reference to a `State<T>` that can be passed to
//! components, enabling automatic two-way synchronization.
//!
//! # Example
//!
//! ```ignore
//! use applib::prelude::*;
//!
//! struct MyView {
//!     query: Entity<State<String>>,
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         TextField::new("search")
//!             .text(State::binding(&self.query, cx))
//!     }
//! }
//! ```

use gpui::{App, Entity};

use super::State;

/// A two-way binding to a `State<T>` value.
///
/// Bindings allow components to both read and write state without
/// requiring manual callback wiring. When a component updates a binding,
/// the underlying state is updated and observers are notified automatically.
#[derive(Clone)]
pub struct Binding<T: Clone + 'static> {
    entity: Entity<State<T>>,
}

impl<T: Clone + 'static> Binding<T> {
    /// Creates a new binding to the given state entity.
    ///
    /// Typically you'll use `State::binding(&entity, cx)` instead of calling this directly.
    pub fn new(entity: Entity<State<T>>, _cx: &App) -> Self {
        Self { entity }
    }

    /// Returns a clone of the current value.
    pub fn get(&self, cx: &App) -> T {
        self.entity.read(cx).get().clone()
    }

    /// Sets a new value, notifying observers.
    ///
    /// This updates the underlying `State<T>` and triggers re-renders
    /// for any views observing the state.
    pub fn set(&self, value: T, cx: &mut App) {
        self.entity.update(cx, |state, cx| {
            state.set(value, cx);
        });
    }

    /// Updates the value using a closure, notifying observers.
    pub fn update(&self, f: impl FnOnce(&mut T), cx: &mut App) {
        self.entity.update(cx, |state, cx| {
            state.update(f, cx);
        });
    }

    /// Returns a reference to the underlying entity.
    ///
    /// This is useful when you need direct access to the entity for
    /// observation or other GPUI operations.
    pub fn entity(&self) -> &Entity<State<T>> {
        &self.entity
    }
}

impl<T: Clone + Default + 'static> Binding<T> {
    /// Resets the value to its default.
    pub fn reset(&self, cx: &mut App) {
        self.set(T::default(), cx);
    }
}

// Convenience implementations for common types

impl Binding<String> {
    /// Returns true if the string is empty.
    pub fn is_empty(&self, cx: &App) -> bool {
        self.entity.read(cx).get().is_empty()
    }

    /// Clears the string value.
    pub fn clear(&self, cx: &mut App) {
        self.set(String::new(), cx);
    }
}

impl Binding<bool> {
    /// Toggles the boolean value.
    pub fn toggle(&self, cx: &mut App) {
        self.update(|v| *v = !*v, cx);
    }
}

impl<T: Clone + 'static> Binding<Option<T>> {
    /// Returns true if the option is None.
    pub fn is_none(&self, cx: &App) -> bool {
        self.entity.read(cx).get().is_none()
    }

    /// Returns true if the option is Some.
    pub fn is_some(&self, cx: &App) -> bool {
        self.entity.read(cx).get().is_some()
    }

    /// Clears the option to None.
    pub fn clear(&self, cx: &mut App) {
        self.set(None, cx);
    }
}
