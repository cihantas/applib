# Prelude

A module providing convenient re-exports of commonly used types and traits.

## Overview

The prelude module simplifies imports by re-exporting frequently used components and utilities in a single namespace. Import the prelude to gain access to all AppLib components and GPUI traits without needing individual import statements.

```rust
use applib::prelude::*;

fn build_ui(cx: &mut App) -> impl IntoElement {
    VStack::new()
        .child(Label::new("Hello"))
        .child(Button::new("Click me"))
}
```

The prelude includes all UI components from the `components` module and the GPUI prelude, giving you immediate access to element builders, styling traits, and state management utilities.

## What's Included

### Components

All UI components are available through the prelude:

**Layout Containers:**
- `VStack`, `HStack`, `ZStack` — Vertical, horizontal, and depth-based stacks.
- `ScrollView` — Scrollable content container.
- `SplitView` — Resizable split pane layout.
- `LazyVStack`, `LazyHStack`, `LazyVGrid`, `LazyHGrid` — Lazy-loading containers for large datasets.

**Form Controls:**
- `Button`, `IconButton` — Clickable buttons with various styles.
- `TextField`, `SecureField`, `TextArea` — Text input controls.
- `Checkbox`, `Toggle`, `RadioGroup` — Selection controls.
- `Slider`, `Stepper` — Numeric input controls.
- `Picker`, `DatePicker`, `ColorPicker` — Specialized pickers.

**Data Display:**
- `Text`, `Label` — Text display with styling.
- `List`, `ListItem`, `ListSection` — List views with sections.
- `Table`, `TableRow`, `TableColumn` — Tabular data display.
- `Badge` — Status indicators.
- `Image` — Image display with various fit modes.
- `ProgressView` — Progress indicators.

**Navigation:**
- `Sidebar`, `SidebarItem` — Sidebar navigation.
- `TabView`, `Tab` — Tab-based navigation.
- `Link` — Clickable links.

**Overlays:**
- `Alert` — Modal alert dialogs.
- `Sheet` — Modal sheets.
- `Popover`, `ControlledPopover` — Popover overlays.
- `ContextMenu`, `ControlledMenu` — Context menus.
- `Tooltip` — Hover tooltips.
- `Menu`, `MenuItem` — Menu bars and items.

**Organization:**
- `Form`, `FormRow`, `FormSection` — Form layouts.
- `GroupBox` — Grouped content.
- `Section` — Content sections.
- `DisclosureGroup` — Collapsible content.
- `Panel` — Content panels.

**Window Management:**
- `WindowFrame` — Window container.
- `TitleBar` — Custom title bars.
- `TrafficLights` — Window control buttons.

**Utilities:**
- `Divider` — Visual separators.
- `Spacer` — Flexible spacing.
- `Canvas` — Custom drawing surface.
- `EmptyView`, `EmptyState` — Empty state placeholders.
- `ColorView` — Color display.

### Enums and Types

The prelude also exports associated types and enums:

**Styles:**
- `ButtonStyle`, `IconButtonStyle`, `ToggleStyle`
- `LabelStyle`, `ListStyle`, `PickerStyle`, `DatePickerStyle`, `ProgressStyle`
- `TextStyle`, `TextAlign`

**State Management:**
- `TextFieldState`, `SecureFieldState`, `TextAreaState`
- `ColorPickerState`, `ContextMenuState`, `TooltipState`, `SliderState`

**Configuration:**
- `Icon`, `ImageSource`, `ImageFit`
- `AlertButton`, `AlertButtonRole`, `AlertIcon`
- `PopoverEdge`, `TooltipPosition`
- `ScrollAxis`, `SelectionMode`
- `ZStackAlignment`

**Canvas Drawing:**
- `DrawingContext`, `PathBuilder`
- `Point2D`, `Size2D`, `Rect2D`
- `FillStyle`, `StrokeStyle`, `CornerRadii`

**Grid Layouts:**
- `GridRow`, `GridColumn`
- `LazyHGridScrollHandle`, `LazyVGridScrollHandle`, `LazyHStackScrollHandle`, `LazyVStackScrollHandle`

**Menu Building:**
- `MenuContent`, `SubMenuBuilder`

**Date Handling:**
- `DateComponents`

**Text Utilities:**
- `text_primary()`, `text_secondary()`, `text_tertiary()`
- `text_accent()`, `text_link()`

### GPUI Traits

The prelude re-exports the GPUI prelude, which includes essential traits:

- `IntoElement` — Converts types into renderable elements.
- `RenderOnce` — Enables one-time rendering.
- `Render` — Enables component rendering.
- `Styled` — Provides styling methods.
- `InteractiveElement` — Adds interaction handlers.
- `ParentElement` — Enables child element management.
- `StatefulInteractiveElement` — Adds stateful interactions.

## Usage

### Basic Import

Import everything from the prelude:

```rust
use applib::prelude::*;
```

### Building a Simple View

```rust
use applib::prelude::*;

fn welcome_screen(cx: &mut App) -> impl IntoElement {
    VStack::new()
        .gap_4()
        .p_6()
        .child(
            Label::new("Welcome")
                .style(LabelStyle::Title)
        )
        .child(
            Text::new("Get started by creating your first project.")
                .color(text_secondary())
        )
        .child(
            Button::new("Create Project")
                .style(ButtonStyle::Primary)
        )
}
```

### Building a Form

```rust
use applib::prelude::*;

fn settings_form(cx: &mut App) -> impl IntoElement {
    Form::new()
        .child(
            FormSection::new("Account")
                .child(
                    FormRow::new()
                        .label("Email")
                        .child(TextField::new())
                )
                .child(
                    FormRow::new()
                        .label("Notifications")
                        .child(Toggle::new(false))
                )
        )
}
```

### Building a List View

```rust
use applib::prelude::*;

fn task_list(tasks: Vec<String>, cx: &mut App) -> impl IntoElement {
    List::new()
        .style(ListStyle::Inset)
        .child(
            ListSection::new()
                .header("Today")
                .children(
                    tasks.iter().map(|task| {
                        ListItem::new(task.clone())
                    })
                )
        )
}
```

## Selective Imports

While the prelude is convenient for most cases, you can selectively import specific items when needed:

```rust
// Import only what you need
use applib::components::{Button, VStack, Label};
use gpui::prelude::*;
```

This approach can help reduce naming conflicts or make dependencies more explicit in larger codebases.

## See Also

- [Components Documentation](/docs/components/)
- [GPUI Prelude](https://www.gpui.rs/prelude)
- [Debouncer](/docs/utils/debounce.md)
