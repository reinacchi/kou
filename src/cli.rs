use crate::args::{help_arg, text_arg, version_arg};
use crate::commands::{handle_config_command, handle_root_command};
use crate::subcommands::config_cmd;
use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use clap::{ArgMatches, ColorChoice, Command};
use colored::Colorize;

pub fn build_cli() -> ArgMatches {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_CRATE_NAME");
    let author_name = "reinacchi"; // this is my command-line interface >:(

    Command::new(name)
        .about("a random command-line interface that does things")
        .author(author_name)
        .override_usage(format!("{} <options> <commands>", name).green().to_string())
        .version(version)
        .bin_name(name)
        .display_name(name)
        .color(ColorChoice::Always)
        .after_help(format!("made by {}", author_name.bright_cyan()))
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(help_arg())
        .arg(text_arg())
        .arg(version_arg())
        .subcommand(config_cmd())
        .get_matches()
}

pub fn handle_matches(matches: &ArgMatches, driver: SQLiteDriver) {
    if let Some(("config", sub_matches)) = matches.subcommand() {
        handle_config_command(sub_matches, driver);
    } else {
        handle_root_command(matches);        
    }
}
