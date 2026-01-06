//! Reactive state management primitives.
//!
//! This module provides SwiftUI-like state management for GPUI applications:
//!
//! - [`State<T>`] - Observable value container with automatic change notification
//! - [`Binding<T>`] - Two-way reference for component bindings
//!
//! # Example
//!
//! ```ignore
//! use applib::prelude::*;
//!
//! struct Counter {
//!     count: Entity<State<i32>>,
//! }
//!
//! impl Counter {
//!     fn new(cx: &mut App) -> Self {
//!         Self {
//!             count: cx.new(|_| State::new(0)),
//!         }
//!     }
//! }
//!
//! impl Render for Counter {
//!     fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
//!         let count = self.count.read(cx).get();
//!
//!         VStack::new()
//!             .child(Text::new(format!("Count: {}", count)))
//!             .child(
//!                 Button::new("increment", "Increment")
//!                     .on_click(cx.listener(|this, _, _, cx| {
//!                         this.count.update(cx, |state, cx| {
//!                             state.update(|n| *n += 1, cx);
//!                         });
//!                     }))
//!             )
//!     }
//! }
//! ```

mod binding;
mod state;

pub use binding::Binding;
pub use state::State;
