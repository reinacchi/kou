use clap::ArgMatches;
use colored::Colorize;

pub fn handle_config_command(matches: &ArgMatches) {
    if let Some(config_name) = matches.get_one::<String>("name") {
        println!("name configured to: {}", config_name.green())
    } else {
        println!("{}", format!("{}", "run 'help config' for more information.").bright_magenta())
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
