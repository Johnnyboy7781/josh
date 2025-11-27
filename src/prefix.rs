use std::io::Write;
use std::io::stdout;
use colored::Colorize;
use crate::dir::{get_curr, get_home};

/// This method prints text before the shell cursor
/// Adds the current dir, replaces the home dir with '~'
pub fn print() {
    let trail = ">";

    let home_dir = get_home();
    let curr_dir = get_curr();

    let dir_str = curr_dir.replace(&home_dir, "~");

    let full_prefix = format!("{} {} ", dir_str, trail).blue().bold();

    print!("{}", full_prefix);
    let _ = stdout().flush();
}
