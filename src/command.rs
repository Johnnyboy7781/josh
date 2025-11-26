use std::path::Path;
use std::{env, process};
use std::str::SplitWhitespace;

pub fn run(command: &str, args: SplitWhitespace) {
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
