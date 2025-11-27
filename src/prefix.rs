use std::io::Write;
use std::io::stdout;
use colored::Colorize;
use crate::dir::get_curr;

/// This method prints text before the shell cursor
/// Adds the current dir, replaces the home dir with '~'
pub fn print(home_dir: &String) {
    let trail = "$".bright_black();

    let curr_dir = get_curr();

    let dir_str = curr_dir.replace(home_dir, "~").blue().bold();

    let full_prefix = format!("{} {} ", dir_str, trail);

    print!("{}", full_prefix);
    let _ = stdout().flush();
}
