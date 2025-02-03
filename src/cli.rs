use crate::args::Args;
use crate::commands::Commands;
use crate::subcommands::SubCommands;
use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use clap::{ArgMatches, ColorChoice, Command};
use colored::Colorize;

pub fn build_cli() -> ArgMatches {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_CRATE_NAME");
    let author_name = "reinacchi"; // this is my command-line interface >:(

    Command::new(name)
        .about(format!("{}", "a random command-line interface that does things").blue().to_string())
        .author(author_name)
        .override_usage(format!("{} <options> <commands>", name).green().to_string())
        .version(version)
        .bin_name(name)
        .display_name(name)
        .color(ColorChoice::Always)
        .after_help(format!("made by {}", author_name.bright_cyan()))
        .disable_version_flag(true)
        .disable_help_flag(true)
        .arg(Args::help_arg())
        .arg(Args::text_arg())
        .arg(Args::version_arg())
        .subcommand(SubCommands::config_cmd())
        .subcommand(SubCommands::fetch_cmd())
        .get_matches()
}

pub fn handle_matches(matches: &ArgMatches, driver: SQLiteDriver) {
    if let Some(("config", sub_matches)) = matches.subcommand() {
        Commands::handle_config_command(sub_matches, driver);
    } else if let Some(("fetch", _)) = matches.subcommand() {
        Commands::handle_fetch_command();
    } else {
        Commands::handle_root_command(matches);        
    }
}
