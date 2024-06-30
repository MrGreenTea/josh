#[allow(unused_imports)]
use std::io::{self, Write};

fn find_command_in_path(cmd: &str) -> Option<String> {
    let paths = std::env::var_os("PATH")?;
    for path in std::env::split_paths(&paths) {
        let full_path = format!("{}/{}", path.display(), cmd);
        if std::fs::metadata(&full_path).is_ok() {
            return Some(full_path);
        }
    }
    None
}

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
        let args = &mut args.collect::<Vec<_>>();
        match cmd {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                for arg in args {
                    match *arg {
                        "exit" | "echo" | "type" => println!("{} is a shell builtin", arg),
                        a => {
                            if let Some(path) = find_command_in_path(a) {
                                println!("{}: {}", a, path);
                            } else {
                                println!("{}: not found", a);
                            }
                        }
                    }
                }
            }
            cmd => println!("{}: command not found", cmd.trim()),
        }
    }
}
