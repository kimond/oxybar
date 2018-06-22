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
        let disk_module_config = TextConfig {
            mod_type: ModuleType::Disk,
            prefix: "Disk ".to_string(),
            suffix: "GB".to_string(),
        };
        let cpu_module = right_widgets.add_widget::<Text>(cpu_module_config);
        let mem_module = right_widgets.add_widget::<Text>(mem_module_config);
        let disk_module = right_widgets.add_widget::<Text>(disk_module_config);
        let date_module = right_widgets.add_widget::<Text>(date_module_config);

        let container = gtk::Box::new(gtk::Orientation::Horizontal, 0);

        container.pack_start(&left_widgets, true, true, 0);
        container.pack_end(&right_widgets, true, true, 0);

        let blocks = vec!(cpu_module, mem_module, date_module, disk_module);

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
        let monitor = get_monitor_from_config(&display, model.monitor.clone())
            .expect("Unable to get monitor");
        let monitor_rec = monitor.get_geometry();
        let window = Window::new(WindowType::Toplevel);
        let screen = window.get_screen().unwrap();
        let style = gtk::CssProvider::new();
        let _ = style.load_from_data(CSS.as_bytes());
        gtk::StyleContext::add_provider_for_screen(&screen, &style, gtk::STYLE_PROVIDER_PRIORITY_USER);

        window.move_(monitor_rec.x, monitor_rec.y);
//        window.set_role("oxybar");
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

        let bar = Bar::new();

        window.add(&bar.container);

        window.show_all();

        let mut gdk_window = window.get_window().unwrap();
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("WM_NORMAL_HINTS"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("_GTK_THEME_VARIANT"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("_NET_WM_OPAQUE_REGION"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("WM_HINTS"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("WM_PROTOCOLS"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("_MOTIF_WM_HINTS"));
        gdk::property_delete(&gdk_window, &gdk::Atom::intern("XdndAware"));
        set_window_strut(&mut gdk_window);

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

fn set_window_strut(window: &mut gdk::Window) {
    use relm::ToGlibPtr;
    use gdk_sys as ffi;
    use std::ffi::CString;
    unsafe {
        let gwin = window.to_glib_none();
        let cardinal_char = CString::new("CARDINAL").unwrap();
        let cardinal_atom = ffi::gdk_atom_intern_static_string(cardinal_char.as_ptr());
        let strut_char = CString::new("_NET_WM_STRUT").unwrap();
        let strut_atom = ffi::gdk_atom_intern_static_string(strut_char.as_ptr());
        let strut_partial_char = CString::new("_NET_WM_STRUT_PARTIAL").unwrap();
        let strut_partial_atom = ffi::gdk_atom_intern_static_string(strut_partial_char.as_ptr());
        let strut_value: *const u8 = [0, 0, 22, 0].as_ptr();
//        let strut_partial_value = [0, 0, 22, 0, 0, 0, 0, 0, 1920, 3839, 0, 0];
        let strut_partial_value = [0, 0, 22, 0, 0, 0, 0, 0, 0, 255, 0, 0].as_ptr();
        ffi::gdk_property_change(gwin.0, strut_atom, cardinal_atom, 8, ffi::GDK_PROP_MODE_REPLACE, strut_value, 4);
        ffi::gdk_property_change(gwin.0, strut_partial_atom, cardinal_atom, 8, ffi::GDK_PROP_MODE_REPLACE, strut_partial_value, 12);
    }
}
