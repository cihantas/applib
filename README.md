# AppLib

A native application framework for building Linux desktop applications.

> [!WARNING]
> **This project is experimental and in alpha.** I'm actively building and iterating on AppLib, so expect things to change—sometimes quickly. If you run into rough edges or missing features, know that I'm working on it. Things should improve rapidly as development continues.

AppLib provides the foundation for creating polished, high-performance desktop apps with a complete set of UI components, window management, and system integration.

## Features

- **60+ Components** - Buttons, lists, tables, forms, navigation, and more
- **Beautiful by Default** - Polished, cohesive design out of the box
- **Built on GPUI** - Leverages Zed's high-performance GPU-accelerated UI framework
- **Declarative API** - Intuitive, composable patterns for rapid development

## Quick Start

Add AppLib to your `Cargo.toml`:

```toml
[dependencies]
applib = "0.1"
gpui = "0.2"
```

Build your first UI:

```rust
use applib::prelude::*;

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |cx| {
            cx.new_view(|_cx| MyApp)
        });
    });
}

struct MyApp;

impl Render for MyApp {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        WindowFrame::new(cx.view().clone())
            .child(
                VStack::new()
                    .gap_4()
                    .p_4()
                    .child(Text::new("Hello, AppLib!").size_xl().bold())
                    .child(
                        HStack::new()
                            .gap_2()
                            .child(Button::new("cancel", "Cancel"))
                            .child(Button::new("ok", "OK").primary())
                    )
            )
    }
}
```

## Available Components

### Layout
`VStack` · `HStack` · `ZStack` · `Spacer` · `Divider`

### Controls
`Button` · `IconButton` · `Toggle` · `Checkbox` · `RadioGroup` · `Slider` · `Stepper` · `Picker` · `DatePicker` · `ColorPicker`

### Input
`TextField` · `SecureField` · `TextArea`

### Lists & Tables
`List` · `ListItem` · `Table` · `TableRow` · `LazyVStack` · `LazyHStack` · `LazyVGrid` · `LazyHGrid`

### Navigation
`TabView` · `Sidebar` · `SidebarItem` · `Menu` · `ContextMenu`

### Windows & Containers
`WindowFrame` · `TitleBar` · `TrafficLights` · `SplitView` · `ScrollView` · `Panel` · `Sheet` · `Alert` · `Popover` · `GroupBox` · `Section` · `Form` · `DisclosureGroup`

### Display
`Text` · `Label` · `Badge` · `Image` · `ProgressView` · `EmptyState` · `EmptyView` · `Canvas` · `ColorView` · `Link` · `Tooltip`

## Documentation

- [Online Documentation](https://applib.dev/docs)
- [API Reference](https://docs.rs/applib)
- [Examples](https://github.com/cihantas/applib/tree/main/examples)

## License

AppLib is dual-licensed:

### Open Source (Free)

**LGPL-3.0-or-later with Additional Terms**

- ✅ Build any application (commercial or open source)
- ✅ Sell your applications
- ✅ Keep your application code proprietary
- ✅ Create plugins, themes, and extensions
- 📤 Share modifications to AppLib itself (LGPL requirement)
- ❌ Cannot create competing UI frameworks

See [LICENSE](LICENSE) for complete terms.

### Commercial License

For organizations that prefer:
- Not sharing modifications to AppLib
- Enterprise support and SLAs
- Additional features and priority fixes
- Indemnification and warranties

Contact: cihan@tas.fm

## Contributing

Contributions are welcome. Please read the [Contributor License Agreement](CLA.md) before submitting pull requests.

```bash
# Clone the repository
git clone https://github.com/cihantas/applib.git

# Build
cargo build

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Community

- [GitHub Discussions](https://github.com/cihantas/applib/discussions)
