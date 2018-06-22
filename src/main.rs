extern crate chrono;
extern crate gdk;
extern crate gtk;
extern crate sys_info;
extern crate clap;
#[macro_use]
extern crate relm;
#[macro_use]
extern crate relm_derive;
extern crate xcb;
extern crate xcb_util;
extern crate gdk_sys;


use clap::Arg;

mod widgets;
mod modules;
mod application;

#[macro_use]
mod macros;


use relm::Widget;
use application::{App, AppConfig};


fn main() {
    let matches = clap::App::new("oxybar")
        .version("v0.0.1")
        .author("Kim Desrosiers <kimdesro@gmail.com>")
        .about("A simple status bar")
        .arg(Arg::with_name("monitor")
            .long("monitor")
            .value_name("monitor")
            .help("Set the monitor to show the bar")
            .takes_value(true))
        .get_matches();

    let (conn, screen_idx) = xcb::Connection::connect(None).expect("Failed to connect to X server");


    let screen = conn.get_setup().roots().nth(screen_idx as usize).unwrap();


    // Create the window
    let window = conn.generate_id();
    xcb::create_window(
        &conn,
        xcb::WINDOW_CLASS_COPY_FROM_PARENT as u8,
        window,
        screen.root(),
        0,
        0,
        1920,
        22,
        0,
        xcb::WINDOW_CLASS_INPUT_OUTPUT as u16,
        screen.root_visual(),
        &[
            (xcb::CW_BACK_PIXEL, 0x00000000 as u32),
            (
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_EXPOSURE | xcb::EVENT_MASK_POINTER_MOTION
                    | xcb::EVENT_MASK_BUTTON_PRESS | xcb::EVENT_MASK_BUTTON_RELEASE,
            ),
            (xcb::CW_OVERRIDE_REDIRECT, 0),
        ],
    );

    // Set all window properties
    let start_x = 0 as u32;
    let end_x = 255 as u32;
    let height = 22 as u32;
    let struts = [0, 0, height, 0, 0, 0, 0, 0, start_x, end_x, 0, 0];
    set_prop!(&conn, window, "_NET_WM_STRUT", &struts[0..4]);
    set_prop!(&conn, window, "_NET_WM_STRUT_PARTIAL", &struts);
    set_prop!(&conn, window, "_NET_WM_WINDOW_TYPE", @atom "_NET_WM_WINDOW_TYPE_DOCK");
    set_prop!(&conn, window, "_NET_WM_STATE", @atom "_NET_WM_STATE_STICKY");
    set_prop!(&conn, window, "_NET_WM_DESKTOP", &[-1]);

    // Request the WM to manage our window.
    xcb::map_window(&conn, window);
    conn.flush();

    let monitor = matches.value_of("monitor").unwrap_or("primary").to_string();
    let app_config = AppConfig {
        monitor
    };
//    App::run(app_config).unwrap();
    loop {

    }
}
