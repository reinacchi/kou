use crate::utils::bits_to_mb;
use chromoe_db::driver::sqlite_driver::SQLiteDriver;
use clap::ArgMatches;
use colored::Colorize;
use sysinfo::System;

pub struct Commands;

impl Commands {
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

    pub fn handle_fetch_command() {
        let mut sys = System::new_all();
        let ascii_art = "⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣀⠤⠤⢤⣄⡤⠤⣤⣀⣀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⠀⠀⢀⣠⠤⠀⡴⠋⠀⠀⠀⠀⠀⠉⠒⢌⠉⠛⣽⡲⣄⡀⠀⠀⠀⠀⠀⠀
    ⠀⠀⠀⠀⠀⣠⠾⠉⠀⠀⠀⠀⣄⠀⠀⠀⠀⠀⢀⣀⠀⣥⡤⠜⠊⣈⢻⣆⠀⠀⠀⠀⠀
    ⠀⠀⠀⣠⠾⠁⠔⠨⠂⠀⢀⠘⡜⡦⣀⡴⡆⠛⠒⠙⡴⡀⠘⡆⠀⠀⠛⡙⢷⡀⠀⠀⠀
    ⠀⠀⡴⠃⠀⠀⠀⠀⢀⣠⡼⠟⡏⡏⠙⣇⢸⡄⠀⠀⢹⠏⠁⢹⡳⣤⠀⠘⡌⣷⠀⠀⠀
    ⠀⣸⠃⠀⡠⠖⢲⠀⠀⣸⠃⢰⡇⡇⠀⢸⣌⣇⢀⠀⣸⣷⣀⡼⢣⡇⠀⠀⢹⣹⠀⠀⠀
    ⠀⡏⠀⡜⠁⠀⠁⠀⡰⢃⣴⣷⢟⣿⡟⡲⠟⠻⠊⠙⠃⣼⣿⣻⣾⡇⠀⠀⢸⡿⠀⠀⠀
    ⠀⡇⠰⡇⠀⢀⡠⠞⡗⢩⡟⢸⡏⠀⢹⡇⠀⠀⠀⠀⠀⢸⣿⠉⢱⣿⠠⢤⣟⠁⠀⠀⠀
    ⠀⣧⠀⠉⠉⠉⠀⢸⠦⡸⡅⢸⣏⠒⣱⠇⠀⠀⠀⠀⠀⠀⢿⣅⡽⠙⢦⠀⢈⣳⡄⠀⠀
    ⠀⡟⠀⠀⠀⠀⠀⠘⠀⣘⡌⣀⡉⠉⠁⠀⠀⠀⠀⠈⠀⠀⠀⠀⠀⠁⠀⡸⠛⠜⡷⣠⠀
    ⢸⠃⠀⠀⠀⠀⣀⡫⣿⣮⡀⠀⠀⠀⠀⠀⢠⠤⠶⡦⡤⠀⠀⠀⠀⠀⢠⠇⡀⠸⣧⣤⡆
    ⡟⠀⠀⠀⠀⠀⠀⡄⢠⠉⢇⠀⡄⠀⠀⠀⠘⢦⣀⡸⠃⠀⠀⠀⢀⡠⠋⠈⠛⢷⡖⠋⠀
    ⡇⢀⠀⠀⠀⠀⠀⢇⠀⢕⣺⣿⣅⡀⠀⠀⠀⠀⠀⠀⢀⣠⠤⠒⠉⠀⢠⣄⡶⠋⠀⠀⠀
    ⠻⢾⣼⣦⣀⠀⡄⠈⠓⢦⣼⣿⣍⠉⠻⣄⠀⢈⠏⠉⣿⣦⡀⠀⢀⣠⠾⠀⠀⠀⠀⠀⠀
    ⠀⠀⠈⠀⠉⠙⠓⠛⣦⡼⠘⣿⣿⣷⣤⣀⣹⠞⢤⣼⣿⣿⠈⢶⡋⠁⠀⠀⠀⠀⠀⠀⠀"
            .bright_yellow();

        sys.refresh_all();

        let system_name = System::name().unwrap_or_else(|| "unknown".to_string());
        let system_kernel = System::kernel_version().unwrap_or_else(|| "unknown".to_string());
        let system_os = System::os_version().unwrap_or_else(|| "unknown".to_string());
        let system_hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
        let total_memory = bits_to_mb(sys.total_memory()).to_string();
        let used_memory = bits_to_mb(sys.used_memory()).to_string();
        let total_swap = bits_to_mb(sys.total_swap()).to_string();
        let used_swap = bits_to_mb(sys.used_swap()).to_string();

        let system_info = format!(
            "{}\n\n\n{}\n\
            {:<22} {}\n\
            {:<22} {}\n\
            {:<22} {}\n\
            {:<22} {}",
            ascii_art,
            "> system info".bold().blue(),
            "system name:",
            system_name,
            "kernel version:",
            system_kernel,
            "OS version:",
            system_os,
            "host name:",
            system_hostname
        );

        let ram_swap_info = format!(
            "{}\n\
            {:<22} {} MB\n\
            {:<22} {} MB\n\
            {:<22} {} MB\n\
            {:<22} {} MB",
            "> RAM and swap info".bold().blue(),
            "total memory:",
            total_memory,
            "used memory:",
            used_memory,
            "total swap:",
            total_swap,
            "used swap:",
            used_swap
        );

        println!("{}\n\n{}", system_info, ram_swap_info);
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
}
