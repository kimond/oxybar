extern crate sys_info;
extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Box, Window, WindowType, Label};
use sys_info::loadavg;
use std::thread;
use std::time::Duration;

pub struct CpuModule {
    label: Label
}

impl CpuModule {
    fn new() -> CpuModule {
        let label = Label::new("...");
        match loadavg() {
            Ok(load) => {
                let load_value = load.one.to_string();
                label.set_text(&load_value);
            },
            Err(x) => {
                eprintln!("Cannot load cpu usage: {}", x);
                label.set_text("Error");
            }
        }

        CpuModule {label}
    }
}

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

        let cpu_module = CpuModule::new();
        right_widgets.pack_start(&cpu_module.label, true, true, 0);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        Bar {container, left_widgets, right_widgets }
    }
}

pub struct App {
    pub window: Window,
    pub bar: Bar,
}

impl App {
    fn new() -> App {
        let display = gdk::Display::get_default().expect("Unable to get default display");
        let monitor = display.get_primary_monitor().expect("Unable to get monitor");
        let monitor_rec = monitor.get_geometry();
        let window = Window::new(WindowType::Toplevel);

        window.set_role("oxybar");
        window.set_wmclass("oxybar", "Oxybar");
        window.set_border_width(1);
        window.set_position(gtk::WindowPosition::None);
        window.set_default_size(monitor_rec.width, 20);
        window.set_decorated(false);
        window.set_type_hint(gdk::WindowTypeHint::Dock);
        window.get_style_context().map(|c| c.add_class("oxybar-window"));

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        let bar = Bar::new();

        window.add(&bar.container);

        App { window, bar }
    }
}

fn main() {
    gtk::init().expect("Oxybar application initialization failed...");

    let app = App::new();

    app.window.show_all();

    gtk::main();
}
