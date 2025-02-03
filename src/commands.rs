use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use clap::ArgMatches;
use colored::Colorize;

pub fn handle_config_command(matches: &ArgMatches, driver: SQLiteDriver) {
    if let Some(config_name) = matches.get_one::<String>("name") {
        driver.set("config.name", config_name).unwrap();
        println!("name configured to: {}", config_name.green());
    } else if matches.get_flag("remove-name") {
        driver.delete("config.name").ok();
        println!("{}", "your name has been removed".green());
    } else {
        match driver.get::<String>("config.name") {
            Ok(Some(cfg_name)) => {
                println!(
                    "config name: {}\n\n{}",
                    cfg_name.bright_cyan(),
                    "run 'help config' for more information".bright_magenta()
                );
            }
            Ok(None) | Err(_) => {
                println!(
                    "config name: {}\n\n{}",
                    "no config name has been set yet".bright_red(),
                    "run 'help config' for more information".bright_magenta()
                );
            }
        }
    }
}

pub fn handle_root_command(matches: &ArgMatches) {
    match (matches.args_present(), matches.contains_id("text")) {
        (false, _) => {
            println!(
                "{}",
                format!(
                    "welcome to kou v{}!\nuse -h for more information",
                    env!("CARGO_PKG_VERSION")
                )
                .bright_magenta()
            );
        }
        (_, true) => match matches.get_one::<String>("text") {
            Some(text) => println!("{}", text),
            None => {}
        },
        _ => {}
    }
}
