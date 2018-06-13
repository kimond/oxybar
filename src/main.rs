extern crate gdk;
extern crate glib;
extern crate gtk;
extern crate sys_info;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;

use relm::{Relm, Widget, Update};

mod block;

use gdk::prelude::*;
use gtk::{Box, Label, Window, WindowType};
use gtk::prelude::*;
use relm::init;
use block::CpuModule;


pub struct Bar {
    container: Box,
    left_widgets: Box,
    right_widgets: Box,
}

impl Bar {
    fn new() -> Bar {
        let left_widgets = Box::new(gtk::Orientation::Horizontal, 0);
        left_widgets.set_halign(gtk::Align::Start);
        let right_widgets = Box::new(gtk::Orientation::Horizontal, 0);
        right_widgets.set_halign(gtk::Align::End);

        let test_label = Label::new("Oxybar");

        left_widgets.pack_start(&test_label, true, true, 0);

        let cpu_module = init::<CpuModule>(()).unwrap();
        right_widgets.pack_start(cpu_module.widget(), true, true, 0);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        Bar { container, left_widgets, right_widgets }
    }
}

const CSS: &str = include_str!("../styles/app.css");

struct Model {}

#[derive(Msg)]
enum Msg {
    Quit,
}

struct App {
    model: Model,
    window: Window,
    bar: Bar,
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
            model,
            window,
            bar
        }
    }
}

fn main() {
    App::run(()).unwrap();
}
