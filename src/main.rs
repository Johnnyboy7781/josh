use std::path::Path;
use std::{env, process};
use std::io::{Write, stdout, stdin};
use std::str::SplitWhitespace;

fn main() {
    loop {
        print_prefix();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;
        
        run_command(command, args);
    }
}

fn print_prefix() {
    let home_dir = "/home/nixos";

    let curr_dir = env::current_dir().unwrap_or_default();
    let dir_str = curr_dir.to_str().unwrap_or("").replace(home_dir, "~");

    print!("{} > ", dir_str);
    let _ = stdout().flush();
}

fn run_command(command: &str, args: SplitWhitespace) {
    match command {
        "cd" => {
            let new_dir = args.peekable().peek().map_or("/", |x| *x);
            let root = Path::new(new_dir);
            if let Err(e) = env::set_current_dir(&root) {
                eprintln!("{}", e);
            }
        },
        "exit" => {
            println!("Bye bye!");
            process::exit(0);
        },
        command => {
            let child = process::Command::new(command)
                .args(args)
                .spawn();

            match child {
                Ok(mut child) => { let _ = child.wait(); },
                Err(e) => eprintln!("{}", e)
            }
        }
    }
}
