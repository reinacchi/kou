mod args;
mod cli;
mod commands;
mod utils;

fn main() {
    let matches = cli::build_cli();

    if cli::handle_help(&matches) {
        return;
    }

    cli::handle_main_logic(&matches);
}
