pub fn show_help_for_command(help_target: &str) {
    match help_target {
        "text" => {
            println!(
                "usage: kou -t <TEXT>\n\nprint the given text.\n\nexample:\n  kou -t \"Hello, world!\""
            );
        }
        _ => {
            eprintln!("error: unknown command '{}'. Use 'kou -h' for a list of commands.", help_target);
        }
    }
}
