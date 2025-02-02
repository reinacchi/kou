use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use clap::ArgMatches;
use colored::Colorize;

pub fn handle_config_command(matches: &ArgMatches, driver: SQLiteDriver) {
    if let Some(config_name) = matches.get_one::<String>("name") {
        driver.set("config.name", config_name).unwrap();

        println!("name configured to: {}", config_name.green())
    } else {
        let cfg_name = driver.get::<String>("config.name").unwrap();

        println!("config name: {}\n\n{}", format!("{}", cfg_name.unwrap().bright_cyan()), format!("{}", "run 'help config' for more information.").bright_magenta())
    }
}

pub fn handle_root_command(matches: &ArgMatches) {
    match (matches.args_present(), matches.contains_id("text")) {
        (false, _) => {
            println!(
                "{}",
                format!("welcome to kou v{}!\nuse -h for more information.", env!("CARGO_PKG_VERSION")).magenta()
            );
        }
        (_, true) => match matches.get_one::<String>("text") {
            Some(text) => println!("{}", text),
            None => {}
        },
        _ => {}
    }
}
