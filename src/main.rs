use std::path::Path;
use std::env;
use std::io::Write;
use std::io::stdout;
use std::process::Command;
use std::io::stdin;

fn main() {
    loop {
        print!("> ");
        let _ = stdout().flush();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap();
        let args = parts;

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
                return;
            },
            command => {
                let child = Command::new(command)
                    .args(args)
                    .spawn();

                match child {
                    Ok(mut child) => { let _ = child.wait(); },
                    Err(e) => eprintln!("{}", e)
                }
            }
        }
    }
}
