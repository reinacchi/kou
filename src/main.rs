use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use chromoe_db::structure::SQLiteDriverOptions;

mod args;
mod cli;
mod commands;
mod subcommands;

fn main() {
    let matches = cli::build_cli();
    let driver_options = SQLiteDriverOptions {
        file_name: "kou_db.sqlite".to_string(),
        table_name: "kou_db".to_string(),
    };
    let driver = SQLiteDriver::new(Some(driver_options)).unwrap();

    cli::handle_matches(&matches, driver);
}
