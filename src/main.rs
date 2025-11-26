use std::io::{stdin, stdout, Write};
use colored::Colorize;
use dir::{get_curr, get_home};

mod dir;
mod command;

fn main() {
    loop {
        print_prefix();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        command::run(command, args);
    }
}

fn print_prefix() {
    let trail = ">";

    let home_dir = get_home();
    let curr_dir = get_curr();

    let dir_str = curr_dir.replace(&home_dir, "~");

    let full_prefix = format!("{} {} ", dir_str, trail).blue().bold();

    print!("{}", full_prefix);
    let _ = stdout().flush();
}
