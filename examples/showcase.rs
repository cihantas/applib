//! Component Showcase - Visual testing and demonstration of all AppLib components.
//!
//! Run with: cargo run --example showcase

use chrono::Local;
use applib::{
    Alert, AlertButton, AlertIcon, Badge, Button, Checkbox, ColorView, DatePicker,
    Divider, DisclosureGroup, Form, FormRow, GroupBox, HStack, Icon, IconButton,
    IconButtonStyle, Label, Link, List, ListItem, ListStyle, Menu, MenuItem,
    NavigationSplitView, Picker, ProgressStyle, ProgressView, RadioGroup, ScrollView,
    Section, Sheet, SidebarItem, Slider, Spacer, Stepper, Tab, TabView, Table, TableColumn,
    Text, TextAlign, TextStyle, TitleBar, Toggle, ToggleStyle, VStack,
    WindowFrame, ZStack, ZStackAlignment,
};
use gpui::*;

/// Showcase section identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum ShowcaseSection {
    Layout,
    Text,
    Controls,
    Inputs,
    Containers,
    Overlays,
    Progress,
}

impl ShowcaseSection {
    fn all() -> &'static [ShowcaseSection] {
        &[
            ShowcaseSection::Layout,
            ShowcaseSection::Text,
            ShowcaseSection::Controls,
            ShowcaseSection::Inputs,
            ShowcaseSection::Containers,
            ShowcaseSection::Overlays,
            ShowcaseSection::Progress,
        ]
    }

    fn name(&self) -> &'static str {
        match self {
            ShowcaseSection::Layout => "Layout",
            ShowcaseSection::Text => "Text & Labels",
            ShowcaseSection::Controls => "Controls",
            ShowcaseSection::Inputs => "Inputs",
            ShowcaseSection::Containers => "Containers",
            ShowcaseSection::Overlays => "Overlays",
            ShowcaseSection::Progress => "Progress",
        }
    }
}

/// View mode for the showcase.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ViewMode {
    All,
    BySection,
}

/// The showcase view displaying all component categories.
struct ShowcaseView {
    // View mode
    view_mode: ViewMode,
    // Selected section for navigation view
    selected_section: ShowcaseSection,
    // TabView state
    selected_tab: usize,
    // Toggle states
    toggle_checkbox: bool,
    toggle_switch: bool,
    // Slider state
    slider_value: f64,
    // Stepper state
    stepper_value: i32,
    // Radio group state
    selected_radio: String,
    // Checkbox states
    checkbox_checked: bool,
    // List state
    selected_list_item: Option<usize>,
    // Disclosure group states
    layout_expanded: bool,
    text_expanded: bool,
    controls_expanded: bool,
    inputs_expanded: bool,
    containers_expanded: bool,
    overlays_expanded: bool,
    progress_expanded: bool,
    // Sheet/Alert state
    show_sheet: bool,
    show_alert: bool,
}

impl ShowcaseView {
    fn new(_cx: &mut Context<Self>) -> Self {
        Self {
            view_mode: ViewMode::All,
            selected_section: ShowcaseSection::Layout,
            selected_tab: 0,
            toggle_checkbox: true,
            toggle_switch: false,
            slider_value: 50.0,
            stepper_value: 5,
            selected_radio: "option1".to_string(),
            checkbox_checked: true,
            selected_list_item: Some(0),
            layout_expanded: true,
            text_expanded: true,
            controls_expanded: true,
            inputs_expanded: true,
            containers_expanded: true,
            overlays_expanded: true,
            progress_expanded: true,
            show_sheet: false,
            show_alert: false,
        }
    }

    /// Renders a section header with title.
    fn section_header(title: impl Into<SharedString>) -> impl IntoElement {
        let title: SharedString = title.into();
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap_2()
            .py_2()
            .child(
                div()
                    .text_sm()
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(hsla(0.0, 0.0, 0.30, 1.0))
                    .child(title),
            )
            .child(
                div()
                    .flex_1()
                    .h(px(1.0))
                    .bg(hsla(0.0, 0.0, 0.85, 1.0)),
            )
    }

    /// Renders a component row with label and component.
    fn component_row(label: impl Into<SharedString>, component: impl IntoElement) -> impl IntoElement {
        let label: SharedString = label.into();
        div()
            .flex()
            .flex_row()
            .items_center()
            .gap(px(16.0))
            .py_1()
            .child(
                div()
                    .w(px(140.0))
                    .text_sm()
                    .text_color(hsla(0.0, 0.0, 0.40, 1.0))
                    .child(label),
            )
            .child(component)
    }

    /// Renders the Layout section content (without disclosure group wrapper).
    fn render_layout_content() -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("LAYOUT"))
            // HStack demo
            .child(Self::component_row(
                "HStack",
                HStack::new()
                    .gap_3()
                    .child(Badge::new("A"))
                    .child(Badge::new("B"))
                    .child(Badge::new("C")),
            ))
            // VStack demo
            .child(Self::component_row(
                "VStack",
                VStack::new()
                    .gap(px(4.0))
                    .child(Badge::new("1"))
                    .child(Badge::new("2"))
                    .child(Badge::new("3")),
            ))
            // ZStack demo
            .child(Self::component_row(
                "ZStack",
                ZStack::new()
                    .alignment(ZStackAlignment::Center)
                    .child(
                        div()
                            .size(px(60.0))
                            .rounded(px(8.0))
                            .bg(hsla(211.0 / 360.0, 0.50, 0.70, 1.0)),
                    )
                    .child(
                        div()
                            .size(px(40.0))
                            .rounded(px(6.0))
                            .bg(hsla(211.0 / 360.0, 0.70, 0.55, 1.0)),
                    )
                    .child(
                        div()
                            .size(px(20.0))
                            .rounded(px(4.0))
                            .bg(hsla(211.0 / 360.0, 0.95, 0.45, 1.0)),
                    ),
            ))
            // Spacer demo
            .child(Self::component_row(
                "Spacer",
                div()
                    .flex()
                    .flex_row()
                    .w(px(200.0))
                    .bg(hsla(0.0, 0.0, 0.95, 1.0))
                    .rounded(px(4.0))
                    .p_2()
                    .child(Text::new("Left"))
                    .child(Spacer::new())
                    .child(Text::new("Right")),
            ))
            // Divider demo
            .child(Self::component_row(
                "Divider",
                div()
                    .flex()
                    .flex_col()
                    .w(px(200.0))
                    .gap(px(8.0))
                    .child(Text::new("Above"))
                    .child(Divider::horizontal())
                    .child(Text::new("Below")),
            ))
    }

    /// Renders the Layout section with disclosure group.
    fn render_layout_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new("layout-section", "Layout Components", self.layout_expanded, {
            let entity = entity.clone();
            move |expanded, _window, app| {
                entity.update(app, |this, cx| {
                    this.layout_expanded = expanded;
                    cx.notify();
                });
            }
        })
        .child(Self::render_layout_content())
    }

    /// Renders the Text & Labels section content (without disclosure group wrapper).
    fn render_text_content() -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("TEXT STYLES"))
            // Text styles
            .child(Self::component_row(
                "Title",
                Text::new("Title Text").style(TextStyle::Title),
            ))
            .child(Self::component_row(
                "Headline",
                Text::new("Headline Text").style(TextStyle::Headline),
            ))
            .child(Self::component_row(
                "Subheadline",
                Text::new("Subheadline Text").style(TextStyle::Subheadline),
            ))
            .child(Self::component_row(
                "Body",
                Text::new("Body Text").style(TextStyle::Body),
            ))
            .child(Self::component_row(
                "Caption",
                Text::new("Caption Text").style(TextStyle::Caption),
            ))
            .child(Self::component_row(
                "Footnote",
                Text::new("Footnote Text").style(TextStyle::Footnote),
            ))
            .child(Self::section_header("TEXT ALIGNMENT"))
            .child(Self::component_row(
                "Left",
                Text::new("Left aligned").align(TextAlign::Left),
            ))
            .child(Self::component_row(
                "Center",
                div()
                    .w(px(200.0))
                    .child(Text::new("Center aligned").align(TextAlign::Center)),
            ))
            .child(Self::component_row(
                "Right",
                div()
                    .w(px(200.0))
                    .child(Text::new("Right aligned").align(TextAlign::Right)),
            ))
            .child(Self::section_header("LABELS"))
            .child(Self::component_row(
                "Label",
                Label::new("Favorites", Icon::Heart),
            ))
            .child(Self::component_row(
                "Label (folder)",
                Label::new("Documents", Icon::Folder),
            ))
            .child(Self::component_row(
                "Link",
                Link::new("link", "Visit Example", "https://example.com"),
            ))
            .child(Self::component_row("Badge", Badge::new("42")))
    }

    /// Renders the Text section with disclosure group.
    fn render_text_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new("text-section", "Text & Labels", self.text_expanded, {
            let entity = entity.clone();
            move |expanded, _window, app| {
                entity.update(app, |this, cx| {
                    this.text_expanded = expanded;
                    cx.notify();
                });
            }
        })
        .child(Self::render_text_content())
    }

    /// Renders the Controls section content (without disclosure group wrapper).
    fn render_controls_content(&self, cx: &Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("BUTTONS"))
            // Primary button
            .child(Self::component_row(
                "Primary",
                Button::new("btn-primary", "Primary Button").primary(),
            ))
            // Secondary button
            .child(Self::component_row(
                "Secondary",
                Button::new("btn-secondary", "Secondary Button").secondary(),
            ))
            // Disabled button
            .child(Self::component_row(
                "Disabled",
                Button::new("btn-disabled", "Disabled Button")
                    .primary()
                    .disabled(true),
            ))
            // Icon buttons
            .child(Self::component_row(
                "IconButton",
                HStack::new()
                    .gap_3()
                    .child(IconButton::new("ib-1", "+"))
                    .child(IconButton::new("ib-2", "-"))
                    .child(IconButton::new("ib-3", "*").style(IconButtonStyle::Filled)),
            ))
            .child(Self::section_header("TOGGLES"))
            // Checkbox toggle
            .child(Self::component_row(
                "Checkbox",
                Toggle::new(
                    "toggle-checkbox",
                    "Checkbox style",
                    self.toggle_checkbox,
                )
                .style(ToggleStyle::Checkbox)
                .on_change(cx.listener(|this, checked: &bool, _window, cx| {
                    this.toggle_checkbox = *checked;
                    cx.notify();
                })),
            ))
            // Switch toggle
            .child(Self::component_row(
                "Switch",
                Toggle::new("toggle-switch", "Switch style", self.toggle_switch)
                    .style(ToggleStyle::Switch)
                    .on_change(cx.listener(|this, checked: &bool, _window, cx| {
                        this.toggle_switch = *checked;
                        cx.notify();
                    })),
            ))
            // Checkbox component
            .child(Self::component_row(
                "Checkbox",
                Checkbox::new("checkbox-demo", "Check me")
                    .checked(self.checkbox_checked)
                    .on_change(cx.listener(|this, checked: &bool, _window, cx| {
                        this.checkbox_checked = *checked;
                        cx.notify();
                    })),
            ))
            .child(Self::section_header("STEPPER"))
            // Stepper
            .child(Self::component_row(
                "Stepper",
                HStack::new()
                    .gap_3()
                    .child(Text::new(format!("Value: {}", self.stepper_value)))
                    .child(Stepper::new(
                        "stepper-demo",
                        self.stepper_value,
                        0..=10,
                    ).on_change({
                        let entity = cx.entity().clone();
                        move |value, _window, app| {
                            entity.update(app, |this, cx| {
                                this.stepper_value = value;
                                cx.notify();
                            });
                        }
                    })),
            ))
            .child(Self::section_header("SLIDER"))
            // Slider
            .child(Self::component_row(
                "Slider",
                HStack::new()
                    .gap_3()
                    .child(Text::new(format!("{:.0}", self.slider_value)))
                    .child(Slider::new("slider-demo", self.slider_value, 0.0..=100.0)
                        .on_change({
                            let entity = cx.entity().clone();
                            move |value, _window, app| {
                                entity.update(app, |this, cx| {
                                    this.slider_value = value;
                                    cx.notify();
                                });
                            }
                        })),
            ))
    }

    /// Renders the Controls section with disclosure group.
    fn render_controls_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new(
            "controls-section",
            "Controls",
            self.controls_expanded,
            {
                let entity = entity.clone();
                move |expanded, _window, app| {
                    entity.update(app, |this, cx| {
                        this.controls_expanded = expanded;
                        cx.notify();
                    });
                }
            },
        )
        .child(self.render_controls_content(cx))
    }

    /// Renders the Inputs section content (without disclosure group wrapper).
    fn render_inputs_content(&self, cx: &Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("SELECTION"))
            // Picker
            .child(Self::component_row(
                "Picker",
                Picker::new("picker-demo", 0usize)
                    .option("Option One")
                    .option("Option Two")
                    .option("Option Three"),
            ))
            // Radio group
            .child(Self::component_row(
                "RadioGroup",
                RadioGroup::new("radio-demo")
                    .option("option1", "First")
                    .option("option2", "Second")
                    .option("option3", "Third")
                    .selected(&self.selected_radio)
                    .on_change(cx.listener(|this, value: &SharedString, _window, cx| {
                        this.selected_radio = value.to_string();
                        cx.notify();
                    })),
            ))
            .child(Self::section_header("COLOR"))
            // ColorView
            .child(Self::component_row(
                "ColorView",
                HStack::new()
                    .gap_3()
                    .child(ColorView::new(hsla(0.0, 0.80, 0.50, 1.0)))
                    .child(ColorView::new(hsla(120.0 / 360.0, 0.80, 0.40, 1.0)))
                    .child(ColorView::new(hsla(211.0 / 360.0, 0.95, 0.53, 1.0))),
            ))
            .child(Self::section_header("DATE"))
            // DatePicker (simple display)
            .child(Self::component_row(
                "DatePicker",
                DatePicker::new("date-picker-demo", Local::now().naive_local()),
            ))
    }

    /// Renders the Inputs section with disclosure group.
    fn render_inputs_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new("inputs-section", "Inputs", self.inputs_expanded, {
            let entity = entity.clone();
            move |expanded, _window, app| {
                entity.update(app, |this, cx| {
                    this.inputs_expanded = expanded;
                    cx.notify();
                });
            }
        })
        .child(self.render_inputs_content(cx))
    }

    /// Renders the Containers section content (without disclosure group wrapper).
    fn render_containers_content(&self, cx: &Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("LISTS"))
            // List demo
            .child(Self::component_row(
                "List",
                div()
                    .h(px(120.0))
                    .w(px(200.0))
                    .overflow_hidden()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.85, 1.0))
                    .child({
                        let items = ["First Item", "Second Item", "Third Item"];
                        let selected = self.selected_list_item;
                        List::new("list-demo", items.len(), move |index, _is_selected, _window, _cx| {
                            ListItem::new(("list-item", index))
                                .selected(selected == Some(index))
                                .child(Text::new(items[index]))
                                .into_any_element()
                        })
                        .style(ListStyle::Plain)
                    }),
            ))
            .child(Self::section_header("GROUPBOX & SECTION"))
            // GroupBox demo
            .child(Self::component_row(
                "GroupBox",
                GroupBox::new()
                    .title("Settings")
                    .child(
                        VStack::new()
                            .gap(px(8.0))
                            .child(Text::new("Content inside"))
                            .child(Text::new("a GroupBox")),
                    ),
            ))
            // Section demo
            .child(Self::component_row(
                "Section",
                Section::new()
                    .header("Section Title")
                    .child(Text::new("Content inside a section")),
            ))
            .child(Self::section_header("FORM"))
            // Form demo
            .child(Self::component_row(
                "Form",
                Form::new()
                    .child(FormRow::new("Name").child(Text::new("John Doe")))
                    .child(FormRow::new("Email").child(Text::new("john@example.com"))),
            ))
            .child(Self::section_header("TABLE"))
            // Table demo
            .child(Self::component_row(
                "Table",
                div()
                    .w(px(300.0))
                    .h(px(100.0))
                    .overflow_hidden()
                    .child(
                        Table::new("table-demo", 2, |index, _selected, _window, _cx| {
                            let items = [("Item A", "$10.00"), ("Item B", "$25.00")];
                            vec![
                                Text::new(items[index].0).into_any_element(),
                                Text::new(items[index].1).into_any_element(),
                            ]
                        })
                        .columns([TableColumn::flex(), TableColumn::fixed(px(80.0))]),
                    ),
            ))
            .child(Self::section_header("TABVIEW"))
            // TabView demo
            .child(Self::component_row(
                "TabView",
                div()
                    .w(px(300.0))
                    .h(px(100.0))
                    .overflow_hidden()
                    .rounded(px(6.0))
                    .border_1()
                    .border_color(hsla(0.0, 0.0, 0.85, 1.0))
                    .child(
                        TabView::new("tabview-demo", self.selected_tab)
                            .tab(Tab::new(
                                "First",
                                div()
                                    .p_2()
                                    .child(Text::new("First tab content")),
                            ))
                            .tab(Tab::new(
                                "Second",
                                div()
                                    .p_2()
                                    .child(Text::new("Second tab content")),
                            ).badge(3))
                            .on_selection_change({
                                let entity = cx.entity().clone();
                                move |index, _window, app| {
                                    entity.update(app, |this, cx| {
                                        this.selected_tab = index;
                                        cx.notify();
                                    });
                                }
                            }),
                    ),
            ))
    }

    /// Renders the Containers section with disclosure group.
    fn render_containers_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new(
            "containers-section",
            "Containers",
            self.containers_expanded,
            {
                let entity = entity.clone();
                move |expanded, _window, app| {
                    entity.update(app, |this, cx| {
                        this.containers_expanded = expanded;
                        cx.notify();
                    });
                }
            },
        )
        .child(self.render_containers_content(cx))
    }

    /// Renders the Overlays section content (without disclosure group wrapper).
    fn render_overlays_content(&self, cx: &Context<Self>) -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("MODAL TRIGGERS"))
            .child(Self::component_row(
                "Sheet",
                Button::new("show-sheet-btn", "Show Sheet")
                    .secondary()
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.show_sheet = true;
                        cx.notify();
                    })),
            ))
            .child(Self::component_row(
                "Alert",
                Button::new("show-alert-btn", "Show Alert")
                    .secondary()
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.show_alert = true;
                        cx.notify();
                    })),
            ))
            .child(Self::section_header("MENU"))
            .child(Self::component_row(
                "Menu",
                Menu::new("menu-demo", "Actions")
                    .item(MenuItem::new("cut", "Cut").shortcut("Ctrl+X"))
                    .item(MenuItem::new("copy", "Copy").shortcut("Ctrl+C"))
                    .item(MenuItem::new("paste", "Paste").shortcut("Ctrl+V"))
                    .divider()
                    .item(MenuItem::new("select-all", "Select All").shortcut("Ctrl+A")),
            ))
    }

    /// Renders the Overlays section with disclosure group.
    fn render_overlays_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new(
            "overlays-section",
            "Overlays & Modals",
            self.overlays_expanded,
            {
                let entity = entity.clone();
                move |expanded, _window, app| {
                    entity.update(app, |this, cx| {
                        this.overlays_expanded = expanded;
                        cx.notify();
                    });
                }
            },
        )
        .child(self.render_overlays_content(cx))
    }

    /// Renders the Progress section content (without disclosure group wrapper).
    fn render_progress_content() -> impl IntoElement {
        VStack::new()
            .gap_3()
            .child(Self::section_header("PROGRESS"))
            // Linear progress
            .child(Self::component_row(
                "Linear (50%)",
                div()
                    .w(px(200.0))
                    .child(ProgressView::new(0.5).style(ProgressStyle::Linear)),
            ))
            // Linear progress with label
            .child(Self::component_row(
                "With Label",
                div().w(px(200.0)).child(
                    ProgressView::new(0.75)
                        .style(ProgressStyle::Linear)
                        .label("Downloading..."),
                ),
            ))
            // Circular progress
            .child(Self::component_row(
                "Circular",
                HStack::new()
                    .gap(px(16.0))
                    .child(ProgressView::new(0.25).style(ProgressStyle::Circular))
                    .child(ProgressView::new(0.50).style(ProgressStyle::Circular))
                    .child(ProgressView::new(0.75).style(ProgressStyle::Circular))
                    .child(ProgressView::new(1.0).style(ProgressStyle::Circular)),
            ))
            // Indeterminate
            .child(Self::component_row(
                "Indeterminate",
                HStack::new()
                    .gap(px(16.0))
                    .child(
                        div()
                            .w(px(100.0))
                            .child(ProgressView::indeterminate().style(ProgressStyle::Linear)),
                    )
                    .child(ProgressView::indeterminate().style(ProgressStyle::Circular)),
            ))
    }

    /// Renders the Progress section with disclosure group.
    fn render_progress_section(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        DisclosureGroup::new(
            "progress-section",
            "Progress & Indicators",
            self.progress_expanded,
            {
                let entity = entity.clone();
                move |expanded, _window, app| {
                    entity.update(app, |this, cx| {
                        this.progress_expanded = expanded;
                        cx.notify();
                    });
                }
            },
        )
        .child(Self::render_progress_content())
    }

    /// Renders the sheet overlay if visible.
    fn render_sheet(&self, cx: &Context<Self>) -> Option<impl IntoElement> {
        if !self.show_sheet {
            return None;
        }

        Some(
            Sheet::new("demo-sheet")
                .title("Demo Sheet")
                .width(px(350.0))
                .child(
                    VStack::new()
                        .gap_3()
                        .child(Text::new("This is a sheet overlay."))
                        .child(Text::new("Sheets are useful for forms and dialogs.")),
                )
                .actions(
                    Button::new("close-sheet-btn", "Close")
                        .primary()
                        .on_click(cx.listener(|this, _event, _window, cx| {
                            this.show_sheet = false;
                            cx.notify();
                        })),
                )
                .on_dismiss(cx.listener(|this, _event, _window, cx| {
                    this.show_sheet = false;
                    cx.notify();
                })),
        )
    }

    /// Renders the alert overlay if visible.
    fn render_alert(&self, cx: &Context<Self>) -> Option<impl IntoElement> {
        if !self.show_alert {
            return None;
        }

        Some(
            Alert::new("Demo Alert")
                .id("demo-alert")
                .message("This is an alert dialog with icon and buttons.")
                .icon(AlertIcon::Warning)
                .button(
                    AlertButton::cancel("Cancel").on_click(cx.listener(
                        |this, _event, _window, cx| {
                            this.show_alert = false;
                            cx.notify();
                        },
                    )),
                )
                .button(AlertButton::new("Confirm").on_click(cx.listener(
                    |this, _event, _window, cx| {
                        this.show_alert = false;
                        cx.notify();
                    },
                )))
                .on_dismiss(cx.listener(|this, _event, _window, cx| {
                    this.show_alert = false;
                    cx.notify();
                })),
        )
    }

    /// Renders the selected section's content for the detail view.
    fn render_section_detail(&self, section: ShowcaseSection, cx: &Context<Self>) -> impl IntoElement {
        ScrollView::vertical("section-detail-scroll").child(
            div().p_4().child(match section {
                ShowcaseSection::Layout => Self::render_layout_content().into_any_element(),
                ShowcaseSection::Text => Self::render_text_content().into_any_element(),
                ShowcaseSection::Controls => self.render_controls_content(cx).into_any_element(),
                ShowcaseSection::Inputs => self.render_inputs_content(cx).into_any_element(),
                ShowcaseSection::Containers => self.render_containers_content(cx).into_any_element(),
                ShowcaseSection::Overlays => self.render_overlays_content(cx).into_any_element(),
                ShowcaseSection::Progress => Self::render_progress_content().into_any_element(),
            })
        )
    }

    /// Renders the sidebar for navigation mode.
    fn render_sidebar(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        VStack::new()
            .p_2()
            .gap(px(4.0))
            .children(ShowcaseSection::all().iter().enumerate().map(|(idx, section)| {
                let section = *section;
                let is_selected = self.selected_section == section;
                let entity = entity.clone();

                SidebarItem::new(("sidebar", idx), section.name())
                    .selected(is_selected)
                    .on_click(move |_event, _window, app| {
                        entity.update(app, |this, cx| {
                            this.selected_section = section;
                            cx.notify();
                        });
                    })
            }))
    }

    /// Renders the "All Components" view with all sections.
    fn render_all_sections_view(&self, cx: &Context<Self>) -> impl IntoElement {
        ScrollView::vertical("showcase-scroll").child(
            VStack::new()
                .p_4()
                .gap(px(16.0))
                .child(self.render_layout_section(cx))
                .child(self.render_text_section(cx))
                .child(self.render_controls_section(cx))
                .child(self.render_inputs_section(cx))
                .child(self.render_containers_section(cx))
                .child(self.render_overlays_section(cx))
                .child(self.render_progress_section(cx)),
        )
    }

    /// Renders the "By Section" view with NavigationSplitView.
    fn render_navigation_view(&self, cx: &Context<Self>) -> impl IntoElement {
        NavigationSplitView::new("showcase-nav")
            .sidebar_width(px(180.0))
            .sidebar(self.render_sidebar(cx))
            .detail(self.render_section_detail(self.selected_section, cx))
    }

    /// Renders the view mode toggle.
    fn render_view_mode_toggle(&self, cx: &Context<Self>) -> impl IntoElement {
        let entity = cx.entity().clone();

        let all_button = {
            let entity = entity.clone();
            let btn = Button::new("view-all", "All")
                .on_click(move |_event, _window, app| {
                    entity.update(app, |this, cx| {
                        this.view_mode = ViewMode::All;
                        cx.notify();
                    });
                });
            if self.view_mode == ViewMode::All {
                btn.primary()
            } else {
                btn.secondary()
            }
        };

        let by_section_button = {
            let entity = entity.clone();
            let btn = Button::new("view-by-section", "By Section")
                .on_click(move |_event, _window, app| {
                    entity.update(app, |this, cx| {
                        this.view_mode = ViewMode::BySection;
                        cx.notify();
                    });
                });
            if self.view_mode == ViewMode::BySection {
                btn.primary()
            } else {
                btn.secondary()
            }
        };

        HStack::new()
            .gap(px(8.0))
            .p_2()
            .child(all_button)
            .child(by_section_button)
    }
}

impl Render for ShowcaseView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Title bar
        let title_bar = TitleBar::new("Component Showcase")
            .on_close(cx.listener(|_this, _event, _window, cx| {
                cx.quit();
            }))
            .on_minimize(|_event, window, _cx| {
                window.minimize_window();
            })
            .on_maximize(|_event, window, _cx| {
                window.zoom_window();
            });

        // View mode toggle bar
        let toggle_bar = div()
            .flex()
            .flex_row()
            .justify_center()
            .border_b_1()
            .border_color(hsla(0.0, 0.0, 0.85, 1.0))
            .bg(hsla(0.0, 0.0, 0.98, 1.0))
            .child(self.render_view_mode_toggle(cx));

        // Main content based on view mode
        let content = match self.view_mode {
            ViewMode::All => self.render_all_sections_view(cx).into_any_element(),
            ViewMode::BySection => self.render_navigation_view(cx).into_any_element(),
        };

        // Main window
        let mut window_content = div()
            .flex()
            .flex_col()
            .size_full()
            .child(title_bar)
            .child(toggle_bar)
            .child(
                div()
                    .flex_1()
                    .overflow_hidden()
                    .bg(hsla(0.0, 0.0, 0.97, 1.0))
                    .child(content),
            );

        // Add sheet if visible
        if let Some(sheet) = self.render_sheet(cx) {
            window_content = window_content.child(sheet);
        }

        // Add alert if visible
        if let Some(alert) = self.render_alert(cx) {
            window_content = window_content.child(alert);
        }

        WindowFrame::new().child(window_content).into_element()
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(700.0), px(800.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_decorations: Some(WindowDecorations::Client),
                titlebar: None,
                ..Default::default()
            },
            |_, cx| cx.new(|cx| ShowcaseView::new(cx)),
        )
        .unwrap();

        cx.activate(true);
    });
}
