use clap::Command;
use colored::Colorize;

use crate::args::{config_name_arg, config_remove_name_arg};

pub fn config_cmd() -> Command {
    Command::new("config")
        .override_usage(format!("{} config <options>", env!("CARGO_CRATE_NAME")).green().to_string())
        .about("configure the preference")
        .arg(config_name_arg())
        .arg(config_remove_name_arg())
}

pub fn fetch_cmd() -> Command {
    Command::new("fetch")
        .about("fetch this software")
}
