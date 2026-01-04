//! DatePicker component for GPUI.
//!
//! A control for selecting dates and/or times, with multiple display styles.

use chrono::{Datelike, Local, NaiveDate, NaiveDateTime, NaiveTime, Timelike, Weekday};
use gpui::prelude::*;
use gpui::*;
use std::ops::RangeInclusive;
use std::rc::Rc;

/// Components that can be displayed/selected in the date picker.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DateComponents {
    /// Date only (day, month, year).
    #[default]
    Date,
    /// Time only (hour, minute).
    Time,
    /// Both date and time.
    DateTime,
}

/// Display style for the date picker.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DatePickerStyle {
    /// Compact text field with popup calendar.
    #[default]
    Compact,
    /// Inline calendar grid display.
    Graphical,
}

/// A date picker component for selecting dates and times.
///
/// # Example
///
/// ```ignore
/// DatePicker::new("date-picker", selected_date)
///     .label("Start Date")
///     .components(DateComponents::Date)
///     .style(DatePickerStyle::Compact)
///     .range(start_date..=end_date)
///     .on_change(|new_date, _window, _cx| {
///         println!("Date changed: {:?}", new_date);
///     })
/// ```
pub struct DatePicker {
    id: ElementId,
    selected: NaiveDateTime,
    components: DateComponents,
    style: DatePickerStyle,
    label: Option<SharedString>,
    range: Option<RangeInclusive<NaiveDate>>,
    disabled: bool,
    is_open: bool,
    on_change: Option<Rc<dyn Fn(NaiveDateTime, &mut Window, &mut App) + 'static>>,
    on_toggle: Option<Rc<dyn Fn(bool, &mut Window, &mut App) + 'static>>,
}

impl DatePicker {
    /// Creates a new date picker with the given id and selected date/time.
    pub fn new(id: impl Into<ElementId>, selected: NaiveDateTime) -> Self {
        Self {
            id: id.into(),
            selected,
            components: DateComponents::default(),
            style: DatePickerStyle::default(),
            label: None,
            range: None,
            disabled: false,
            is_open: false,
            on_change: None,
            on_toggle: None,
        }
    }

    /// Creates a new date picker with the current date/time.
    pub fn now(id: impl Into<ElementId>) -> Self {
        let now = Local::now().naive_local();
        Self::new(id, now)
    }

    /// Sets the date/time components to display.
    pub fn components(mut self, components: DateComponents) -> Self {
        self.components = components;
        self
    }

    /// Sets the display style.
    pub fn style(mut self, style: DatePickerStyle) -> Self {
        self.style = style;
        self
    }

    /// Sets the label text shown next to the picker.
    pub fn label(mut self, label: impl Into<SharedString>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Sets the valid date range for selection.
    pub fn range(mut self, range: RangeInclusive<NaiveDate>) -> Self {
        self.range = Some(range);
        self
    }

    /// Sets whether the picker is disabled.
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Sets whether the popup is open (for compact style).
    pub fn is_open(mut self, is_open: bool) -> Self {
        self.is_open = is_open;
        self
    }

    /// Sets the change handler called when the date/time changes.
    pub fn on_change(
        mut self,
        handler: impl Fn(NaiveDateTime, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_change = Some(Rc::new(handler));
        self
    }

    /// Sets the toggle handler called when the popup opens/closes.
    pub fn on_toggle(
        mut self,
        handler: impl Fn(bool, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_toggle = Some(Rc::new(handler));
        self
    }

    /// Check if a date is within the valid range.
    fn is_date_in_range(&self, date: NaiveDate) -> bool {
        match &self.range {
            Some(range) => range.contains(&date),
            None => true,
        }
    }

    /// Format the date for display based on components.
    fn format_display(&self) -> String {
        match self.components {
            DateComponents::Date => self.selected.format("%b %d, %Y").to_string(),
            DateComponents::Time => self.selected.format("%I:%M %p").to_string(),
            DateComponents::DateTime => self.selected.format("%b %d, %Y at %I:%M %p").to_string(),
        }
    }

    /// Build the compact text field trigger.
    fn build_compact_trigger(&self, colors: &DatePickerColors) -> Stateful<Div> {
        let display_text = self.format_display();
        let disabled = self.disabled;
        let is_open = self.is_open;

        let border_color = if is_open && !disabled {
            colors.border_focused
        } else if disabled {
            colors.border_disabled
        } else {
            colors.border
        };

        let bg_color = if disabled {
            colors.bg_disabled
        } else {
            colors.bg
        };

        let text_color = if disabled {
            colors.text_disabled
        } else {
            colors.text
        };

        let mut trigger = div()
            .id("date-picker-trigger")
            .flex()
            .flex_row()
            .items_center()
            .gap(px(6.0))
            .h(px(24.0))
            .px(px(8.0))
            .bg(bg_color)
            .border_1()
            .border_color(border_color)
            .rounded(px(4.0))
            .text_sm()
            .text_color(text_color)
            .shadow(vec![BoxShadow {
                color: hsla(0.0, 0.0, 0.0, 0.05),
                offset: point(px(0.0), px(1.0)),
                blur_radius: px(2.0),
                spread_radius: px(0.0),
            }])
            .child(display_text)
            .child(
                // Calendar icon indicator
                div()
                    .text_xs()
                    .text_color(colors.icon)
                    .child("▼"),
            );

        if !disabled {
            trigger = trigger
                .cursor_pointer()
                .hover(|style| style.bg(hsla(0.0, 0.0, 0.96, 1.0)));
        }

        // Focus ring when open
        if is_open && !disabled {
            trigger = trigger.shadow(vec![
                BoxShadow {
                    color: hsla(0.0, 0.0, 0.0, 0.05),
                    offset: point(px(0.0), px(1.0)),
                    blur_radius: px(2.0),
                    spread_radius: px(0.0),
                },
                BoxShadow {
                    color: hsla(211.0 / 360.0, 0.80, 0.55, 0.3),
                    offset: point(px(0.0), px(0.0)),
                    blur_radius: px(0.0),
                    spread_radius: px(3.0),
                },
            ]);
        }

        trigger
    }

    /// Build the calendar grid for date selection.
    fn build_calendar(&self, colors: &DatePickerColors) -> Div {
        let selected_date = self.selected.date();
        let current_year = selected_date.year();
        let current_month = selected_date.month();

        // Get first day of month and calculate grid offset
        let first_of_month = NaiveDate::from_ymd_opt(current_year, current_month, 1)
            .unwrap_or(selected_date);
        let days_in_month = days_in_month(current_year, current_month);
        let first_weekday = first_of_month.weekday();
        let start_offset = weekday_offset(first_weekday);

        // Month/year header
        let month_name = month_name(current_month);
        let header = div()
            .flex()
            .flex_row()
            .items_center()
            .justify_between()
            .px(px(8.0))
            .py(px(6.0))
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(self.build_nav_button("prev-month", "◀", -1, colors))
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text)
                    .child(format!("{} {}", month_name, current_year))
            )
            .child(
                div()
                    .flex()
                    .items_center()
                    .gap(px(4.0))
                    .child(self.build_nav_button("next-month", "▶", 1, colors))
            );

        // Weekday labels
        let weekday_labels = div()
            .flex()
            .flex_row()
            .children(["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].iter().map(|day| {
                div()
                    .w(px(28.0))
                    .h(px(20.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(colors.text_secondary)
                    .child(*day)
            }));

        // Build the day grid (6 rows x 7 columns)
        let mut grid = div().flex().flex_col();
        let mut day = 1;
        let today = Local::now().naive_local().date();

        for week in 0..6 {
            let mut row = div().flex().flex_row();

            for weekday in 0..7 {
                let cell_idx = week * 7 + weekday;

                if cell_idx < start_offset || day > days_in_month {
                    // Empty cell
                    row = row.child(
                        div()
                            .w(px(28.0))
                            .h(px(28.0))
                    );
                } else {
                    let current_day = day;
                    let date = NaiveDate::from_ymd_opt(current_year, current_month, current_day as u32)
                        .unwrap_or(selected_date);
                    let is_selected = date == selected_date;
                    let is_today = date == today;
                    let in_range = self.is_date_in_range(date);

                    let mut cell = div()
                        .id(("day", week * 7 + weekday))
                        .w(px(28.0))
                        .h(px(28.0))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_sm()
                        .rounded(px(4.0));

                    if is_selected {
                        cell = cell
                            .bg(colors.selected_bg)
                            .text_color(colors.selected_text)
                            .font_weight(FontWeight::SEMIBOLD);
                    } else if is_today {
                        cell = cell
                            .text_color(colors.today_text)
                            .font_weight(FontWeight::SEMIBOLD);
                    } else if in_range {
                        cell = cell.text_color(colors.text);
                    } else {
                        cell = cell.text_color(colors.text_disabled);
                    }

                    if in_range && !is_selected {
                        cell = cell
                            .cursor_pointer()
                            .hover(|style| style.bg(colors.hover_bg));

                        let on_change = self.on_change.clone();
                        let selected_time = self.selected.time();
                        cell = cell.on_click(move |_event, window, cx| {
                            if let Some(ref handler) = on_change {
                                let new_datetime = NaiveDateTime::new(date, selected_time);
                                handler(new_datetime, window, cx);
                            }
                        });
                    }

                    cell = cell.child(format!("{}", current_day));
                    row = row.child(cell);
                    day += 1;
                }
            }

            grid = grid.child(row);

            // Stop if we've displayed all days and completed the week
            if day > days_in_month {
                break;
            }
        }

        div()
            .flex()
            .flex_col()
            .p(px(8.0))
            .w(px(220.0))
            .child(header)
            .child(weekday_labels)
            .child(grid)
    }

    /// Build a navigation button for month navigation.
    fn build_nav_button(&self, id: &'static str, label: &'static str, delta: i32, colors: &DatePickerColors) -> Stateful<Div> {
        let on_change = self.on_change.clone();
        let selected = self.selected;

        div()
            .id(id)
            .w(px(24.0))
            .h(px(24.0))
            .flex()
            .items_center()
            .justify_center()
            .text_xs()
            .text_color(colors.text_secondary)
            .rounded(px(4.0))
            .cursor_pointer()
            .hover(|style| style.bg(colors.hover_bg))
            .on_click(move |_event, window, cx| {
                if let Some(ref handler) = on_change {
                    let new_date = add_months(selected.date(), delta);
                    let new_datetime = NaiveDateTime::new(new_date, selected.time());
                    handler(new_datetime, window, cx);
                }
            })
            .child(label)
    }

    /// Build the time picker section.
    fn build_time_picker(&self, colors: &DatePickerColors) -> Div {
        let hour = self.selected.hour();
        let minute = self.selected.minute();
        let is_pm = hour >= 12;
        let display_hour = if hour == 0 { 12 } else if hour > 12 { hour - 12 } else { hour };

        div()
            .flex()
            .flex_row()
            .items_center()
            .justify_center()
            .gap(px(4.0))
            .p(px(8.0))
            .border_t_1()
            .border_color(colors.border)
            .child(
                // Hour spinner
                self.build_time_spinner("hour", display_hour, colors)
            )
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(colors.text)
                    .child(":")
            )
            .child(
                // Minute spinner
                self.build_time_spinner("minute", minute, colors)
            )
            .child(
                // AM/PM toggle
                self.build_ampm_toggle(is_pm, colors)
            )
    }

    /// Build a time spinner for hour or minute.
    fn build_time_spinner(&self, id: &'static str, value: u32, colors: &DatePickerColors) -> Div {
        let on_change = self.on_change.clone();
        let selected = self.selected;
        let is_hour = id == "hour";

        let up_handler = {
            let on_change = on_change.clone();
            move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                if let Some(ref handler) = on_change {
                    let new_datetime = if is_hour {
                        add_hours(selected, 1)
                    } else {
                        add_minutes(selected, 1)
                    };
                    handler(new_datetime, window, cx);
                }
            }
        };

        let down_handler = {
            move |_event: &ClickEvent, window: &mut Window, cx: &mut App| {
                if let Some(ref handler) = on_change {
                    let new_datetime = if is_hour {
                        add_hours(selected, -1)
                    } else {
                        add_minutes(selected, -1)
                    };
                    handler(new_datetime, window, cx);
                }
            }
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap(px(2.0))
            .child(
                div()
                    .id((id, 0u32))
                    .w(px(28.0))
                    .h(px(18.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(colors.text_secondary)
                    .rounded(px(2.0))
                    .cursor_pointer()
                    .hover(|style| style.bg(colors.hover_bg))
                    .on_click(up_handler)
                    .child("▲")
            )
            .child(
                div()
                    .w(px(32.0))
                    .h(px(24.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_sm()
                    .font_weight(FontWeight::MEDIUM)
                    .text_color(colors.text)
                    .bg(colors.bg)
                    .border_1()
                    .border_color(colors.border)
                    .rounded(px(4.0))
                    .child(format!("{:02}", value))
            )
            .child(
                div()
                    .id((id, 1u32))
                    .w(px(28.0))
                    .h(px(18.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .text_xs()
                    .text_color(colors.text_secondary)
                    .rounded(px(2.0))
                    .cursor_pointer()
                    .hover(|style| style.bg(colors.hover_bg))
                    .on_click(down_handler)
                    .child("▼")
            )
    }

    /// Build AM/PM toggle.
    fn build_ampm_toggle(&self, is_pm: bool, colors: &DatePickerColors) -> Div {
        let on_change = self.on_change.clone();
        let selected = self.selected;

        div()
            .flex()
            .flex_col()
            .ml(px(8.0))
            .gap(px(2.0))
            .child(
                div()
                    .id("am-btn")
                    .px(px(8.0))
                    .py(px(4.0))
                    .text_xs()
                    .font_weight(if !is_pm { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                    .text_color(if !is_pm { colors.selected_text } else { colors.text })
                    .bg(if !is_pm { colors.selected_bg } else { colors.bg })
                    .rounded_t(px(4.0))
                    .border_1()
                    .border_b_0()
                    .border_color(colors.border)
                    .cursor_pointer()
                    .when(is_pm, |div| {
                        let on_change = on_change.clone();
                        div.on_click(move |_event, window, cx| {
                            if let Some(ref handler) = on_change {
                                let new_datetime = toggle_ampm(selected);
                                handler(new_datetime, window, cx);
                            }
                        })
                    })
                    .child("AM")
            )
            .child(
                div()
                    .id("pm-btn")
                    .px(px(8.0))
                    .py(px(4.0))
                    .text_xs()
                    .font_weight(if is_pm { FontWeight::SEMIBOLD } else { FontWeight::NORMAL })
                    .text_color(if is_pm { colors.selected_text } else { colors.text })
                    .bg(if is_pm { colors.selected_bg } else { colors.bg })
                    .rounded_b(px(4.0))
                    .border_1()
                    .border_color(colors.border)
                    .cursor_pointer()
                    .when(!is_pm, |div| {
                        div.on_click(move |_event, window, cx| {
                            if let Some(ref handler) = on_change {
                                let new_datetime = toggle_ampm(selected);
                                handler(new_datetime, window, cx);
                            }
                        })
                    })
                    .child("PM")
            )
    }
}

/// Colors for the date picker component.
struct DatePickerColors {
    bg: Hsla,
    bg_disabled: Hsla,
    border: Hsla,
    border_focused: Hsla,
    border_disabled: Hsla,
    text: Hsla,
    text_secondary: Hsla,
    text_disabled: Hsla,
    icon: Hsla,
    selected_bg: Hsla,
    selected_text: Hsla,
    today_text: Hsla,
    hover_bg: Hsla,
}

impl DatePickerColors {
    fn new() -> Self {
        Self {
            bg: hsla(0.0, 0.0, 1.0, 1.0),
            bg_disabled: hsla(0.0, 0.0, 0.96, 1.0),
            border: hsla(0.0, 0.0, 0.75, 1.0),
            border_focused: hsla(211.0 / 360.0, 0.80, 0.55, 1.0),
            border_disabled: hsla(0.0, 0.0, 0.85, 1.0),
            text: hsla(0.0, 0.0, 0.15, 1.0),
            text_secondary: hsla(0.0, 0.0, 0.45, 1.0),
            text_disabled: hsla(0.0, 0.0, 0.65, 1.0),
            icon: hsla(0.0, 0.0, 0.50, 1.0),
            selected_bg: hsla(211.0 / 360.0, 0.95, 0.53, 1.0),
            selected_text: hsla(0.0, 0.0, 1.0, 1.0),
            today_text: hsla(211.0 / 360.0, 0.80, 0.50, 1.0),
            hover_bg: hsla(0.0, 0.0, 0.94, 1.0),
        }
    }
}

impl IntoElement for DatePicker {
    type Element = Stateful<Div>;

    fn into_element(self) -> Self::Element {
        let colors = DatePickerColors::new();
        let label = self.label.clone();
        let label_color = hsla(0.0, 0.0, 0.30, 1.0);

        let content = match self.style {
            DatePickerStyle::Compact => {
                // Compact style: trigger field with popup
                let trigger = self.build_compact_trigger(&colors);
                let is_open = self.is_open;
                let disabled = self.disabled;

                let mut container = div()
                    .id("date-picker-compact")
                    .relative();

                // Add click handler to trigger
                let trigger_with_click = if !disabled {
                    let on_toggle = self.on_toggle.clone();
                    trigger.on_click(move |_event, window, cx| {
                        if let Some(ref handler) = on_toggle {
                            handler(!is_open, window, cx);
                        }
                    })
                } else {
                    trigger
                };

                container = container.child(trigger_with_click);

                // Add popup if open
                if is_open && !disabled {
                    let mut popup_content = div()
                        .flex()
                        .flex_col()
                        .bg(colors.bg)
                        .rounded(px(8.0))
                        .border_1()
                        .border_color(colors.border)
                        .shadow(vec![
                            BoxShadow {
                                color: hsla(0.0, 0.0, 0.0, 0.15),
                                offset: point(px(0.0), px(4.0)),
                                blur_radius: px(12.0),
                                spread_radius: px(0.0),
                            },
                        ]);

                    // Add calendar if date component is enabled
                    if matches!(self.components, DateComponents::Date | DateComponents::DateTime) {
                        popup_content = popup_content.child(self.build_calendar(&colors));
                    }

                    // Add time picker if time component is enabled
                    if matches!(self.components, DateComponents::Time | DateComponents::DateTime) {
                        popup_content = popup_content.child(self.build_time_picker(&colors));
                    }

                    // Position the popup below the trigger
                    let popup = div()
                        .absolute()
                        .top_full()
                        .left(px(0.0))
                        .mt(px(4.0))
                        .id("date-picker-popup")
                        .on_mouse_down(MouseButton::Left, |_event, _window, cx| {
                            cx.stop_propagation();
                        })
                        .child(popup_content);

                    // Build the overlay with dismiss handling
                    let on_toggle = self.on_toggle.clone();
                    let popover_overlay = div()
                        .id("date-picker-dismiss")
                        .on_click(move |_event, window, cx| {
                            if let Some(ref handler) = on_toggle {
                                handler(false, window, cx);
                            }
                        })
                        .child(popup);

                    container = container.child(popover_overlay);
                }

                container
            }
            DatePickerStyle::Graphical => {
                // Graphical style: inline calendar
                let mut content = div()
                    .id("date-picker-graphical")
                    .bg(colors.bg)
                    .rounded(px(8.0))
                    .border_1()
                    .border_color(colors.border)
                    .shadow(vec![
                        BoxShadow {
                            color: hsla(0.0, 0.0, 0.0, 0.08),
                            offset: point(px(0.0), px(2.0)),
                            blur_radius: px(8.0),
                            spread_radius: px(0.0),
                        },
                    ]);

                // Add calendar if date component is enabled
                if matches!(self.components, DateComponents::Date | DateComponents::DateTime) {
                    content = content.child(self.build_calendar(&colors));
                }

                // Add time picker if time component is enabled
                if matches!(self.components, DateComponents::Time | DateComponents::DateTime) {
                    content = content.child(self.build_time_picker(&colors));
                }

                content
            }
        };

        // Wrap with label if provided
        if let Some(label_text) = label {
            div()
                .id(self.id)
                .flex()
                .flex_col()
                .gap(px(4.0))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::MEDIUM)
                        .text_color(label_color)
                        .child(label_text),
                )
                .child(content)
        } else {
            div()
                .id(self.id)
                .child(content)
        }
    }
}

// Helper functions for date calculations

/// Get the number of days in a month.
fn days_in_month(year: i32, month: u32) -> i32 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => {
            if is_leap_year(year) {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

/// Check if a year is a leap year.
fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
}

/// Get the offset for the first day of the month (0 = Sunday).
fn weekday_offset(weekday: Weekday) -> usize {
    match weekday {
        Weekday::Sun => 0,
        Weekday::Mon => 1,
        Weekday::Tue => 2,
        Weekday::Wed => 3,
        Weekday::Thu => 4,
        Weekday::Fri => 5,
        Weekday::Sat => 6,
    }
}

/// Get month name.
fn month_name(month: u32) -> &'static str {
    match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Unknown",
    }
}

/// Add months to a date (handles overflow properly).
fn add_months(date: NaiveDate, months: i32) -> NaiveDate {
    let total_months = date.year() * 12 + date.month() as i32 - 1 + months;
    let new_year = total_months / 12;
    let new_month = (total_months % 12 + 1) as u32;
    let max_day = days_in_month(new_year, new_month) as u32;
    let new_day = date.day().min(max_day);

    NaiveDate::from_ymd_opt(new_year, new_month, new_day).unwrap_or(date)
}

/// Add hours to a datetime.
fn add_hours(dt: NaiveDateTime, hours: i32) -> NaiveDateTime {
    let total_hours = dt.hour() as i32 + hours;
    let new_hour = ((total_hours % 24) + 24) % 24;
    let day_delta = if total_hours < 0 {
        (total_hours - 23) / 24
    } else {
        total_hours / 24
    };

    let new_date = if day_delta != 0 {
        dt.date() + chrono::Duration::days(day_delta as i64)
    } else {
        dt.date()
    };

    NaiveDateTime::new(
        new_date,
        NaiveTime::from_hms_opt(new_hour as u32, dt.minute(), dt.second()).unwrap_or(dt.time()),
    )
}

/// Add minutes to a datetime.
fn add_minutes(dt: NaiveDateTime, minutes: i32) -> NaiveDateTime {
    dt + chrono::Duration::minutes(minutes as i64)
}

/// Toggle AM/PM.
fn toggle_ampm(dt: NaiveDateTime) -> NaiveDateTime {
    let hour = dt.hour();
    let new_hour = if hour >= 12 { hour - 12 } else { hour + 12 };

    NaiveDateTime::new(
        dt.date(),
        NaiveTime::from_hms_opt(new_hour, dt.minute(), dt.second()).unwrap_or(dt.time()),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_date_picker_creation() {
        let date = NaiveDate::from_ymd_opt(2024, 3, 15).unwrap();
        let time = NaiveTime::from_hms_opt(14, 30, 0).unwrap();
        let datetime = NaiveDateTime::new(date, time);

        let picker = DatePicker::new("test", datetime);
        assert_eq!(picker.selected, datetime);
        assert_eq!(picker.components, DateComponents::Date);
        assert_eq!(picker.style, DatePickerStyle::Compact);
    }

    #[test]
    fn test_date_components() {
        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        );

        let picker = DatePicker::new("test", datetime)
            .components(DateComponents::DateTime);
        assert_eq!(picker.components, DateComponents::DateTime);

        let picker = DatePicker::new("test", datetime)
            .components(DateComponents::Time);
        assert_eq!(picker.components, DateComponents::Time);
    }

    #[test]
    fn test_date_picker_style() {
        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        );

        let picker = DatePicker::new("test", datetime)
            .style(DatePickerStyle::Graphical);
        assert_eq!(picker.style, DatePickerStyle::Graphical);
    }

    #[test]
    fn test_date_range() {
        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        );

        let start = NaiveDate::from_ymd_opt(2024, 1, 1).unwrap();
        let end = NaiveDate::from_ymd_opt(2024, 12, 31).unwrap();

        let picker = DatePicker::new("test", datetime)
            .range(start..=end);

        assert!(picker.is_date_in_range(NaiveDate::from_ymd_opt(2024, 6, 15).unwrap()));
        assert!(!picker.is_date_in_range(NaiveDate::from_ymd_opt(2023, 6, 15).unwrap()));
    }

    #[test]
    fn test_format_display() {
        let datetime = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(14, 30, 0).unwrap(),
        );

        let date_picker = DatePicker::new("test", datetime)
            .components(DateComponents::Date);
        assert_eq!(date_picker.format_display(), "Mar 15, 2024");

        let time_picker = DatePicker::new("test", datetime)
            .components(DateComponents::Time);
        assert_eq!(time_picker.format_display(), "02:30 PM");

        let datetime_picker = DatePicker::new("test", datetime)
            .components(DateComponents::DateTime);
        assert_eq!(datetime_picker.format_display(), "Mar 15, 2024 at 02:30 PM");
    }

    #[test]
    fn test_days_in_month() {
        assert_eq!(days_in_month(2024, 1), 31);
        assert_eq!(days_in_month(2024, 2), 29); // Leap year
        assert_eq!(days_in_month(2023, 2), 28); // Non-leap year
        assert_eq!(days_in_month(2024, 4), 30);
    }

    #[test]
    fn test_is_leap_year() {
        assert!(is_leap_year(2024));
        assert!(!is_leap_year(2023));
        assert!(!is_leap_year(2100));
        assert!(is_leap_year(2000));
    }

    #[test]
    fn test_add_months() {
        let date = NaiveDate::from_ymd_opt(2024, 1, 31).unwrap();

        // Adding 1 month from Jan 31 -> Feb 29 (leap year)
        let result = add_months(date, 1);
        assert_eq!(result, NaiveDate::from_ymd_opt(2024, 2, 29).unwrap());

        // Subtracting months
        let result = add_months(date, -1);
        assert_eq!(result, NaiveDate::from_ymd_opt(2023, 12, 31).unwrap());
    }

    #[test]
    fn test_toggle_ampm() {
        let am_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(9, 30, 0).unwrap(),
        );
        let pm_result = toggle_ampm(am_time);
        assert_eq!(pm_result.hour(), 21);

        let pm_time = NaiveDateTime::new(
            NaiveDate::from_ymd_opt(2024, 3, 15).unwrap(),
            NaiveTime::from_hms_opt(21, 30, 0).unwrap(),
        );
        let am_result = toggle_ampm(pm_time);
        assert_eq!(am_result.hour(), 9);
    }
}
