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
extern crate toml;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use clap::Arg;

mod widgets;
mod modules;
mod application;
mod bar;
#[macro_use]
mod macros;
mod config;

mod utils;

use relm::Widget;
use application::{App, AppConfig};
use utils::create_strut_window;
use config::parse_config;


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
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("config")
            .help("Set the path to the config file")
            .takes_value(true))
        .get_matches();

    let (conn, screen_idx) = xcb::Connection::connect(None).expect("Failed to connect to X server");
    create_strut_window(&conn, screen_idx as usize);


    let config_path = matches.value_of("config").unwrap_or("./tests/config.toml");
    let bar_config = parse_config(config_path.to_string()).unwrap();

    let monitor = matches.value_of("monitor").unwrap_or("primary").to_string();
    let app_config = AppConfig {
        monitor,
        bar_config
    };

    App::run(app_config).unwrap();
}

