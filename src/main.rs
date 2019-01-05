use clap::Arg;
use relm::Widget;

use crate::application::{App, AppConfig};
use crate::config::parse_config;
use crate::utils::create_strut_window;

mod widgets;
mod modules;
mod application;
mod bar;
#[macro_use]
mod macros;
mod config;

mod utils;

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
        bar_config,
    };

    App::run(app_config).unwrap();
}

