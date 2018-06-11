extern crate gdk;
extern crate gio;
extern crate gtk;

use gio::prelude::*;
use gtk::prelude::*;
use gdk::prelude::*;
use gtk::{Button, Window, WindowType};

use std::env::args;

fn build_ui(application: &gtk::Application) {
    let display = gdk::Display::get_default().expect("Unable to get default display");
    let monitor = display.get_primary_monitor().expect("Unable to get monitor");
    let monitor_rec = monitor.get_geometry();
    let window = gtk::ApplicationWindow::new(application);
    let hbox = gtk::Box::new(gtk::Orientation::Horizontal, 0);
    let label = gtk::Label::new("Test");
    hbox.add(&label);

    window.set_role("oxybar");
    window.set_border_width(1);
    window.set_position(gtk::WindowPosition::None);
    window.set_default_size(monitor_rec.width, 20);
    window.set_decorated(false);
    window.set_type_hint(gdk::WindowTypeHint::Dock);

    window.connect_delete_event(move |window, _| {
        window.destroy();
        Inhibit(false)
    });

    window.add(&hbox);

    window.show_all();
}

fn main() {
    let application = gtk::Application::new("com.kimond.oxybar", gio::ApplicationFlags::empty())
        .expect("Initialization failed...");

    application.connect_startup(move |app| {
        build_ui(app);
    });
    application.connect_activate(|_| {});

    application.run(&args().collect::<Vec<_>>());
}
