use std::{env, io::{stdin, stdout, Write}, path::Path, process::{Command, Child, Stdio}};
use colored::Colorize;
use dir::{get_curr, get_home};

mod dir;
mod command;

fn main() {
    loop {
        print_prefix();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut commands = input.trim().split(" | ").peekable();
        let mut previous_command = None;

        while let Some(command) = commands.next() {
            let mut parts = command.trim().split_whitespace();
            let command = parts.next().unwrap_or("");
            let args = parts;

            match command {
                "cd" => {
                    let new_dir = args.peekable().peek().map_or("/", |x| *x).replace("~", &get_home());
                    let root = Path::new(new_dir.as_str());
                    if let Err(e) = env::set_current_dir(&root) {
                        eprintln!("{}", e);
                    }

                    previous_command = None;
                }
                "exit" => return,
                command => {
                    let stdin = previous_command.map_or(Stdio::inherit(), |output: Child| {
                        Stdio::from(output.stdout.unwrap())
                    });

                    let stdout = if commands.peek().is_some() {
                        // there is another command piped behind this one
                        // prepare to send output to the next command
                        Stdio::piped()
                    } else {
                        // there are no more commands piped behind this one
                        // send output to shell stdout
                        Stdio::inherit()
                    };

                    let output = Command::new(command)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        }
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        }
                    };
                }
            }
        }

        if let Some(mut final_command) = previous_command {
            // block until the final command has finished
            final_command.wait().unwrap();
        }
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
