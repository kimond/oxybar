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


use clap::Arg;

mod widgets;
mod modules;
mod application;


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

    let monitor = matches.value_of("monitor").unwrap_or("primary").to_string();
    let app_config = AppConfig {
        monitor
    };
    App::run(app_config).unwrap();
}
