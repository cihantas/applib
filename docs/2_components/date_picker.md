# DatePicker

A control for selecting dates and times with calendar and time picker interfaces.

## Overview

DatePicker provides an interactive calendar and time selection interface following modern design patterns. The component supports date-only, time-only, or combined date/time selection with optional range constraints.

```rust
DatePicker::new("start-date", selected_date)
    .label("Start Date")
    .components(DateComponents::Date)
    .style(DatePickerStyle::Compact)
    .range(start_date..=end_date)
    .on_change(|new_date, _window, _cx| {
        println!("Date changed: {:?}", new_date);
    })
```

The picker presents a calendar grid for date selection and spinner controls for time adjustment, with automatic validation against the specified range.

## Topics

### Creating a Date Picker

- `new(_:_:)` — Creates a date picker with the given identifier and selected date/time.
- `now(_:)` — Creates a date picker initialized to the current date and time.

### Configuring Components

- `components(_:)` — Sets which date/time components to display.

### Configuring Display

- `style(_:)` — Sets the display style variant.
- `label(_:)` — Sets the label text shown with the picker.

### Configuring Range

- `range(_:)` — Sets the valid date range for selection.

### Configuring State

- `disabled(_:)` — Sets whether the picker is disabled.
- `is_open(_:)` — Sets whether the popup is open (Compact style only).

### Handling Changes

- `on_change(_:)` — Registers a handler called when the date/time changes.
- `on_toggle(_:)` — Registers a handler called when the popup opens or closes.

## Date Components

### Date

Displays only the calendar for date selection.

```rust
DatePicker::new("date", selected)
    .components(DateComponents::Date)
```

### Time

Displays only the time picker with hour, minute, and AM/PM controls.

```rust
DatePicker::new("time", selected)
    .components(DateComponents::Time)
```

### DateTime

Displays both calendar and time picker.

```rust
DatePicker::new("datetime", selected)
    .components(DateComponents::DateTime)
```

## Display Styles

### Compact

Text field with popup calendar, ideal for forms and space-constrained layouts.

```rust
DatePicker::new("compact", selected)
    .style(DatePickerStyle::Compact)
```

### Graphical

Inline calendar display, suitable for scheduling interfaces.

```rust
DatePicker::new("graphical", selected)
    .style(DatePickerStyle::Graphical)
```

## Date Range Constraints

Restrict selectable dates to a specific range:

```rust
use chrono::NaiveDate;

let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

DatePicker::new("date", selected)
    .range(start..=end)
```

Dates outside the range appear dimmed and cannot be selected.

## See Also

- Picker
- Slider (for time-of-day selection)
