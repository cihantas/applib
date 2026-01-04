# AppLib

A modern UI component library built on GPUI for creating beautiful, high-performance Linux applications.

## Overview

AppLib provides a collection of reusable UI components with a polished, modern aesthetic. The library is built on GPUI and offers a declarative API similar to SwiftUI for building modern desktop applications.

Use AppLib to create consistent, polished user interfaces with minimal code. The library handles styling, state management, and interaction patterns while maintaining the flexibility to customize components for your specific needs.

```rust
use applib::prelude::*;

// Create a simple form layout
VStack::new()
    .gap_3()
    .child(
        TextField::new("username")
            .placeholder("Enter username")
    )
    .child(
        Button::new("submit", "Submit")
            .primary()
    )
```

The library organizes components into logical groups based on their purpose: layout containers, interactive controls, input fields, lists and tables, navigation elements, windows and containers, disclosure controls, and display components.

## Topics

### Layout

Arrange views and create flexible layouts.

- `HStack` — Arranges child views in a horizontal line.
- `VStack` — Arranges child views in a vertical stack.
- `ZStack` — Overlays child views along the z-axis.
- `Spacer` — Creates flexible space between views.

### Controls

Interactive elements for user actions and settings.

- `Button` — Triggers actions with primary and secondary styling.
- `IconButton` — Displays an icon-only interactive button.
- `Toggle` — Switches between on and off states.
- `Checkbox` — Represents a binary choice.
- `RadioGroup` — Selects a single option from multiple choices.
- `Slider` — Selects a value from a continuous range.
- `Stepper` — Increments or decrements a numeric value.
- `Picker` — Selects from a list of options in a dropdown.
- `DatePicker` — Selects dates using various presentation styles.
- `ColorPicker` — Selects colors from a color panel.

### Input

Text entry and editing components.

- `TextField` — Accepts single-line text input.
- `SecureField` — Accepts password input with masked characters.
- `TextArea` — Accepts multi-line text input.

### Lists & Tables

Display collections of data.

- `List` — Displays scrollable sections of items.
- `ListItem` — Represents an individual item with selection and hover states.
- `Table` — Displays data in rows and columns.
- `TableRow` — Represents a single row in a table.
- `LazyVStack` — Virtualizes vertical lists for efficient rendering.
- `LazyHStack` — Virtualizes horizontal lists for efficient rendering.
- `LazyVGrid` — Virtualizes vertical grids for efficient rendering.
- `LazyHGrid` — Virtualizes horizontal grids for efficient rendering.

### Navigation

Navigate between views and hierarchies.

- `TabView` — Switches between multiple child views using tabs.
- `Sidebar` — Displays a source list for navigation.
- `SidebarItem` — Represents an individual navigation item in a sidebar.

### Windows & Containers

Group and organize content.

- `WindowFrame` — Provides a window with title bar and traffic lights.
- `TitleBar` — Displays a window title bar with polished styling.
- `TrafficLights` — Displays window control buttons (close, minimize, maximize).
- `SplitView` — Divides space between two resizable views.
- `ScrollView` — Provides scrolling for content larger than the viewport.
- `Sheet` — Presents content as a modal sheet.
- `Alert` — Displays critical information requiring user response.
- `Panel` — Provides a floating utility window.
- `GroupBox` — Groups related content with an optional label.
- `Section` — Organizes content with an optional header.
- `Form` — Arranges labeled controls in a form layout.

### Disclosure & Menus

Show and hide content, display contextual options.

- `DisclosureGroup` — Shows or hides content with a disclosure triangle.
- `Menu` — Displays a menu of actions.
- `ContextMenu` — Displays context-sensitive actions.
- `Popover` — Presents content in a floating container.

### Display

Present information visually.

- `Text` — Displays styled text content.
- `Label` — Displays text with an optional icon.
- `Badge` — Displays a small rounded indicator for counts or labels.
- `Divider` — Separates content with a horizontal or vertical line.
- `ProgressView` — Indicates task progress or activity.
- `EmptyView` — Represents a view with no content.
- `EmptyState` — Displays a placeholder for empty content with icon and message.
- `Canvas` — Provides a surface for custom drawing operations.
- `ColorView` — Displays a color swatch.
- `Image` — Displays image content with various fit modes.
- `Link` — Displays clickable hyperlinks.
- `Tooltip` — Shows contextual information on hover.

## See Also

### Component Types

- `ButtonStyle` — Visual styling options for buttons.
- `IconButtonSize` — Size variants for icon buttons.
- `IconButtonStyle` — Visual styling for icon buttons.
- `ListStyle` — Presentation styles for lists.
- `SelectionMode` — Selection behavior for lists.
- `PickerStyle` — Presentation styles for pickers.
- `DatePickerStyle` — Presentation styles for date pickers.
- `ProgressStyle` — Visual styles for progress indicators.
- `ScrollAxis` — Scrolling directions for scroll views.
- `TextStyle` — Predefined text styles.
- `TextAlign` — Text alignment options.
- `ToggleStyle` — Visual styles for toggles.
- `TooltipPosition` — Positioning options for tooltips.
- `ZStackAlignment` — Alignment options for z-stacks.
- `ImageFit` — Content modes for images.

### Drawing Types

- `Canvas` — Custom drawing surface.
- `DrawingContext` — Context for drawing operations.
- `PathBuilder` — Constructs drawing paths.
- `Point2D` — Represents a point in 2D space.
- `Size2D` — Represents dimensions in 2D space.
- `Rect2D` — Represents a rectangle in 2D space.
- `CornerRadii` — Specifies corner radius values.
- `FillStyle` — Styling for filled shapes.
- `StrokeStyle` — Styling for stroked paths.

### State Management

- `ColorPickerState` — Manages color picker state.
- `ContextMenuState` — Manages context menu state.
- `SecureFieldState` — Manages secure field state.
- `SliderState` — Manages slider state.
- `TextAreaState` — Manages text area state.
- `TextFieldState` — Manages text field state.
- `TooltipState` — Manages tooltip state.

### Scroll Handles

- `LazyVStackScrollHandle` — Controls lazy vertical stack scrolling.
- `LazyHStackScrollHandle` — Controls lazy horizontal stack scrolling.
- `LazyVGridScrollHandle` — Controls lazy vertical grid scrolling.
- `LazyHGridScrollHandle` — Controls lazy horizontal grid scrolling.

### Form Components

- `Form` — Form container.
- `FormRow` — Individual form row.
- `FormSection` — Grouped form section.

### Menu Components

- `Menu` — Standard menu.
- `ControlledMenu` — Menu with external state control.
- `MenuContent` — Menu content builder.
- `MenuItem` — Individual menu item.
- `SubMenuBuilder` — Constructs nested submenus.

### Popover Components

- `Popover` — Standard popover.
- `ControlledPopover` — Popover with external state control.
- `PopoverEdge` — Edge positioning for popovers.

### Alert Components

- `Alert` — Alert dialog.
- `AlertButton` — Alert button configuration.
- `AlertButtonRole` — Semantic roles for alert buttons.
- `AlertIcon` — Icon styles for alerts.

### Grid Components

- `GridRow` — Row configuration for horizontal grids.
- `GridColumn` — Column configuration for vertical grids.

### Date Components

- `DateComponents` — Represents date values.
- `DatePicker` — Date selection control.
- `DatePickerStyle` — Date picker presentation styles.

### Label Components

- `Label` — Text with optional icon.
- `Icon` — Icon representation.
- `LabelStyle` — Label styling options.

### Image Components

- `Image` — Image display component.
- `ImageSource` — Image content source.
- `ImageFit` — Content scaling modes.

### Table Components

- `Table` — Table container.
- `TableColumn` — Column definition.
- `TableRow` — Row representation.

### Tab Components

- `TabView` — Tabbed interface.
- `Tab` — Individual tab configuration.

### Text Utilities

- `text_primary()` — Creates primary text style.
- `text_secondary()` — Creates secondary text style.
- `text_tertiary()` — Creates tertiary text style.
- `text_accent()` — Creates accent text style.
- `text_link()` — Creates link text style.
