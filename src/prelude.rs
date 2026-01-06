//! Prelude for convenient imports.
//!
//! This module re-exports commonly used types for easy access.
//!
//! # Example
//!
//! ```ignore
//! use applib::prelude::*;
//! ```

// Re-export all components
pub use crate::components::*;

// Re-export state management primitives
pub use crate::state::{Binding, State};

// Re-export gpui prelude for convenience
pub use gpui::prelude::*;
