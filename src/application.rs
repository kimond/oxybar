use gdk;
use gtk;
use gdk::prelude::*;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{Relm, Widget, Update};
use relm::{ContainerWidget, Component};

use widgets::{Text, TextConfig, Workspace};
use modules::ModuleType;

pub struct Bar {
    container: gtk::Box,
    _workspace: Component<Workspace>,
    _blocks: Vec<Component<Text>>,
}

impl Bar {
    fn new() -> Bar {
        let left_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 0);
        left_widgets.set_halign(gtk::Align::Start);
        let right_widgets = gtk::Box::new(gtk::Orientation::Horizontal, 10);
        right_widgets.set_halign(gtk::Align::End);

        let workspace_module = left_widgets.add_widget::<Workspace>(());

        let cpu_module_config = TextConfig {
            mod_type: ModuleType::LoadAvg,
            prefix: "CPU ".to_string(),
            suffix: "%".to_string(),
        };
        let mem_module_config = TextConfig {
            mod_type: ModuleType::Memory,
            prefix: "MEM ".to_string(),
            suffix: "GB".to_string(),
        };
        let date_module_config = TextConfig {
            mod_type: ModuleType::Date,
            prefix: "".to_string(),
            suffix: "".to_string(),
        };
        let cpu_module = right_widgets.add_widget::<Text>(cpu_module_config);
        let mem_module = right_widgets.add_widget::<Text>(mem_module_config);
        let date_module = right_widgets.add_widget::<Text>(date_module_config);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        let blocks = vec!(cpu_module, mem_module, date_module);

        Bar { container, _workspace: workspace_module, _blocks: blocks }
    }
}

const CSS: &str = include_str!("../styles/app.css");

pub struct Model {
    monitor: String
}

pub struct AppConfig {
    pub monitor: String
}

#[derive(Msg)]
pub enum Msg {
    Quit,
}

pub struct App {
    _model: Model,
    window: Window,
    _bar: Bar,
}

impl Update for App {
    type Model = Model;
    type ModelParam = AppConfig;
    type Msg = Msg;

    fn model(_: &Relm<Self>, params: Self::ModelParam) -> Model {
        Model {
            monitor: params.monitor
        }
    }

    fn update(&mut self, event: Msg) {
        match event {
            Msg::Quit => gtk::main_quit(),
        }
    }
}

impl Widget for App {
    type Root = Window;

    fn root(&self) -> Self::Root {
        self.window.clone()
    }

    // Create the widgets.
    fn view(relm: &Relm<Self>, model: Self::Model) -> Self {
        let display = gdk::Display::get_default().expect("Unable to get default display");
        let mut monitor = display.get_primary_monitor().expect("Unable to get monitor");
        if model.monitor == "primary" {
            monitor = display.get_primary_monitor().expect("Unable to get monitor");
        } else {
            let nb_monitors = display.get_n_monitors();
            for n in 0..nb_monitors {
                if let Some(m) = display.get_monitor(n) {
                    let monitor_model = m.get_model().unwrap_or("".to_string());
                    if monitor_model == model.monitor {
                        monitor = m;
                        break;
                    }
                }
            }
        }
        let monitor_rec = monitor.get_geometry();
        let window = Window::new(WindowType::Toplevel);
        let screen = window.get_screen().unwrap();
        let style = gtk::CssProvider::new();
        let _ = style.load_from_data(CSS.as_bytes());
        gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

        window.move_(monitor_rec.x, 0);
        window.set_role("oxybar");
        window.set_wmclass("oxybar", "Oxybar");
        window.set_border_width(1);
        window.set_position(gtk::WindowPosition::None);
        window.set_default_size(monitor_rec.width, 22);
        window.set_decorated(false);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.get_style_context().map(|c| c.add_class("oxybar-window"));

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        let bar = Bar::new();

        window.add(&bar.container);

        window.show_all();

        App {
            _model: model,
            window,
            _bar: bar,
        }
    }
}
