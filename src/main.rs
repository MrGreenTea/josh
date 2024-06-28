#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut args = input.split_whitespace();
        let cmd = args.next().unwrap();
        match cmd {
            "exit" => break,
            cmd => println!("{}: command not found", cmd.trim()),
        }
    }
}
