mod args;
mod cli;
mod commands;
mod subcommands;

fn main() {
    let matches = cli::build_cli();

    cli::handle_matches(&matches);
}
