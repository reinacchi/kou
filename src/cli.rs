use clap::{ColorChoice, Command};
use colored::Colorize;
use crate::args::{help_arg, text_arg, version_arg};
use crate::commands;
use crate::utils;

pub fn build_cli() -> clap::ArgMatches {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_CRATE_NAME");
    let author_name = "reinacchi"; // this is my command-line interface >:(

    Command::new(name)
        .about("a random CLI that does things")
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
        .get_matches()
}

pub fn handle_help(matches: &clap::ArgMatches) -> bool {
    if let Some(help_target) = matches.get_one::<String>("help") {
        commands::show_help_for_command(help_target);
        return true;
    }

    matches.contains_id("help")
}

pub fn handle_main_logic(matches: &clap::ArgMatches, version: &str) {
    match (matches.args_present(), matches.contains_id("text")) {
        (false, _) => {
            println!("{}", format!("welcome to kou v{}!\nuse -h for more information.", version).magenta());
        }
        (_, true) => {
            match matches.get_one::<String>("text") {
                Some(text) => println!("{}", text),
                None => eprintln!("{}", utils::error_message("the -t flag requires a value.")),
            }
        }
        _ => {}
    }
}
