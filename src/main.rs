#[macro_use]
extern crate lazy_static;

extern crate clap;
extern crate config;
extern crate reqwest;
extern crate serde;

use clap::{App, AppSettings, SubCommand};
use serde::Deserialize;

mod api;
mod command;
mod gui;
mod cfg;

fn main() {
    let app = App::new("purr")
        .version("1.0")
        .author("SkyezerFox <skye@foxboy.sh>")
        .about("Command line interface and TUI for the Pterodactyl API")
        .subcommand(SubCommand::with_name("config").about("Modify purr configuration"))
        .subcommand(SubCommand::with_name("status").about("Fetches the status of active servers"))
        .subcommand(SubCommand::with_name("tui").about("Starts the terminal user interface"))
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    match matches.subcommand() {
        ("config", _) => command::config::get_config(),
        ("status", _) => command::status::get_status(),
        ("tui", _) => gui::run_gui(),
        _ => {}
    }
}

#[derive(Deserialize, Debug)]
struct StatusResponse {
    object: String,
    data: Vec<String>,
}
