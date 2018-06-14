extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate sys_info;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::{Relm, Widget, Update};

mod widgets;
mod modules;

use gdk::prelude::*;
use gtk::{Box, Label, Window, WindowType};
use gtk::prelude::*;
use relm::{ContainerWidget, Component};
use widgets::Text;

use modules::{Module, LoadAvg, ModuleType, Config};


pub struct Bar {
    container: Box,
    _blocks: Vec<Component<Text>>,
}

impl Bar {
    fn new() -> Bar {
        let left_widgets = Box::new(gtk::Orientation::Horizontal, 0);
        left_widgets.set_halign(gtk::Align::Start);
        let right_widgets = Box::new(gtk::Orientation::Horizontal, 5);
        right_widgets.set_halign(gtk::Align::End);

        let cpu_module_config = Config {
            mod_type: ModuleType::LoadAvg,
            prefix: "CPU ".to_string(),
            suffix: "%".to_string()
        };
        let mem_module_config = Config {
            mod_type: ModuleType::Memory,
            prefix: "MEM ".to_string(),
            suffix: "GB".to_string()
        };
        let cpu_module = right_widgets.add_widget::<Text>(cpu_module_config);
        let mem_module = right_widgets.add_widget::<Text>(mem_module_config);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        let blocks = vec!(cpu_module, mem_module);

        Bar { container, _blocks: blocks }
    }
}

const CSS: &str = include_str!("../styles/app.css");

struct Model {}

#[derive(Msg)]
enum Msg {
    Quit,
}

struct App {
    _model: Model,
    window: Window,
    _bar: Bar,
}

impl Update for App {
    type Model = Model;
    type ModelParam = ();
    type Msg = Msg;

    fn model(_: &Relm<Self>, _: ()) -> Model {
        Model {
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
        let monitor = display.get_primary_monitor().expect("Unable to get monitor");
        let monitor_rec = monitor.get_geometry();
        let window = Window::new(WindowType::Toplevel);
        let screen = window.get_screen().unwrap();
        let style = gtk::CssProvider::new();
        let _ = style.load_from_data(CSS.as_bytes());
        gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

        window.set_role("oxybar");
        window.set_wmclass("oxybar", "Oxybar");
        window.set_border_width(1);
        window.set_position(gtk::WindowPosition::None);
        window.set_default_size(monitor_rec.width, 20);
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
            _bar: bar
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
