// AppLib - A Native Application Framework for Linux
// Copyright (C) 2025 AppLib Contributors
//
// This library is dual-licensed under:
// - LGPL-3.0-or-later with Additional Terms (see LICENSE file)
// - Commercial License (contact cihan@tas.fm)
//
// See LICENSE file for the complete open source license terms.
// For commercial licensing options, visit: https://applib.dev/licensing

#![recursion_limit = "512"]
//! AppLib - A Native Application Framework for Linux
//!
//! AppLib provides the foundation for building polished, high-performance Linux
//! desktop applications. It includes a complete set of UI components, window
//! management, and system integration.
//!
//! ## Quick Start
//!
//! ```ignore
//! use applib::prelude::*;
//!
//! // Create a primary button
//! Button::new("submit", "Submit").primary()
//!
//! // Create a list item
//! ListItem::new("item-1").selected(true)
//!
//! // Layout with VStack
//! VStack::new()
//!     .gap_3()
//!     .child(Button::new("btn1", "Button 1"))
//!     .child(Button::new("btn2", "Button 2"))
//! ```
//!
//! ## Available Components
//!
//! **Layout:**
//! - [`VStack`] - Vertical stack layout
//! - [`HStack`] - Horizontal stack layout
//! - [`ZStack`] - Layered stack layout
//! - [`Spacer`] - Flexible spacing
//!
//! **Controls:**
//! - [`Button`] - Buttons with primary/secondary variants
//! - [`IconButton`] - Icon-only buttons
//! - [`Toggle`] - Toggle switch
//! - [`Checkbox`] - Checkbox control
//! - [`RadioGroup`] - Radio button group
//! - [`Slider`] - Slider control
//! - [`Stepper`] - Increment/decrement stepper
//! - [`Picker`] - Dropdown picker
//! - [`DatePicker`] - Date selection control
//! - [`ColorPicker`] - Color selection control
//!
//! **Input:**
//! - [`TextField`] - Text input field
//! - [`SecureField`] - Password input field
//! - [`TextArea`] - Multi-line text input
//!
//! **Lists & Tables:**
//! - [`List`] - Scrollable list with sections
//! - [`ListItem`] - Generic list item with selection and hover states
//! - [`Table`] - Data table
//! - [`LazyVStack`] - Virtualized vertical list
//! - [`LazyHStack`] - Virtualized horizontal list
//! - [`LazyVGrid`] - Virtualized vertical grid
//! - [`LazyHGrid`] - Virtualized horizontal grid
//!
//! **Navigation:**
//! - [`TabView`] - Tab-based navigation
//! - [`Sidebar`] - Source list container
//! - [`SidebarItem`] - Individual sidebar navigation item
//!
//! **Windows & Containers:**
//! - [`WindowFrame`] - Window frame with title bar and traffic lights
//! - [`TitleBar`] - Window title bar
//! - [`TrafficLights`] - Window control buttons
//! - [`SplitView`] - Resizable split view
//! - [`ScrollView`] - Scrollable container
//! - [`Sheet`] - Modal sheet
//! - [`Alert`] - Alert dialog
//! - [`GroupBox`] - Grouped content container
//! - [`Section`] - Content section with optional header
//! - [`Form`] - Form layout container
//!
//! **Disclosure & Menus:**
//! - [`DisclosureGroup`] - Collapsible section
//! - [`Menu`] - Context menu
//! - [`ContextMenu`] - Contextual menu
//! - [`Popover`] - Popover overlay
//!
//! **Display:**
//! - [`Text`] - Styled text
//! - [`Label`] - Text with optional icon
//! - [`Badge`] - Small rounded badge for counts and labels
//! - [`Divider`] - Visual separator
//! - [`ProgressView`] - Progress indicator
//! - [`EmptyView`] - Empty state placeholder
//! - [`Canvas`] - Custom drawing surface
//! - [`ColorView`] - Color display
//! - [`Link`] - Clickable link
//! - [`Tooltip`] - Hover tooltip

pub mod components;
pub mod prelude;
pub mod state;
pub mod utils;

// Re-export all components at crate root for convenience
pub use components::*;
pub use state::{Binding, State};
