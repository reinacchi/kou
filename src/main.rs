use chromoe_db::driver::sqlite_driver::SQLiteDriver;

mod args;
mod cli;
mod commands;
mod subcommands;

fn main() {
    let matches = cli::build_cli();
    let driver = SQLiteDriver::new(None).unwrap();

    cli::handle_matches(&matches, driver);
}
