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

        let command = input.trim();

        let mut child = Command::new(command)
            .spawn()
            .unwrap();

        let _ = child.wait();
    }
}
