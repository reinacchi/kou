use clap::{Arg, ArgAction};

pub fn help_arg() -> Arg {
    Arg::new("help")
    .short('h')
    .long("help")
    .help("show list of all commands and options")
    .action(ArgAction::Help)
}

pub fn text_arg() -> Arg {
    Arg::new("text")
        .short('t')
        .long("text")
        .value_name("text")
        .help("print the given text")
        .num_args(1)
        .required(false)
}

pub fn version_arg() -> Arg {
    Arg::new("version")
        .short('v')
        .long("version")
        .help("print kou's version")
        .action(ArgAction::Version)
}
