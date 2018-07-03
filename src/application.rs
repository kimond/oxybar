use gdk;
use gtk;
use gdk::prelude::*;
use gtk::prelude::*;
use gtk::{Window, WindowType};
use relm::{Relm, Widget, Update};

use config::Config;
use bar::Bar;


const CSS: &str = include_str!("../styles/app.css");

pub struct Model {
    monitor: String,
    bar_config: Config
}

pub struct AppConfig {
    pub monitor: String,
    pub bar_config: Config
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
            monitor: params.monitor,
            bar_config: params.bar_config
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
        let monitor = get_monitor_from_config(&display, model.monitor.clone())
            .expect("Unable to get monitor");
        let monitor_rec = monitor.get_geometry();
        let window = Window::new(WindowType::Toplevel);
        let screen = window.get_screen().unwrap();
        let style = gtk::CssProvider::new();
        let _ = style.load_from_data(CSS.as_bytes());
        gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

        window.move_(monitor_rec.x, monitor_rec.y);
        window.set_role("oxybar");
        window.set_wmclass("oxybar", "Oxybar");
        window.set_border_width(1);
        window.set_position(gtk::WindowPosition::None);
        window.set_default_size(monitor_rec.width, 22);
        window.set_decorated(false);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.get_style_context().map(|c| c.add_class("oxybar-window"));
        window.set_keep_above(true);
        window.stick();

        // Connect the signal `delete_event` to send the `Quit` message.
        connect!(relm, window, connect_delete_event(_, _), return (Some(Msg::Quit), Inhibit(false)));

        let bar = Bar::new(&model.bar_config);

        window.add(&bar.container);

        window.show_all();

        App {
            _model: model,
            window,
            _bar: bar,
        }
    }
}

fn get_monitor_from_config(display: &gdk::Display, monitor_value: String) -> Option<gdk::Monitor> {
    if monitor_value == "primary" {
        let m = display.get_primary_monitor().expect("Unable to get monitor");
        return Some(m);
    } else {
        for n in 0..display.get_n_monitors() {
            if let Some(m) = display.get_monitor(n) {
                if monitor_value == m.get_model().unwrap_or("".to_string()) {
                    return Some(m);
                }
            }
        }
    }
    return None;
}
