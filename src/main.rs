#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;

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

fn run_executable(cmd: &str, args: &[&str]) {
    // print output to stdout
    let mut child = std::process::Command::new(cmd).args(args).spawn().unwrap();

    child.wait().unwrap();
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
                        "pwd" | "exit" | "echo" | "type" | "cd" => {
                            println!("{} is a shell builtin", arg)
                        }
                        a => {
                            if let Some(path) = find_command_in_path(a) {
                                println!("{} is {}", a, path);
                            } else {
                                println!("{}: not found", a);
                            }
                        }
                    }
                }
            }
            "pwd" => println!("{}", std::env::current_dir().unwrap().display()),
            "cd" => {
                if args.len() > 1 {
                    println!("cd: too many arguments");
                } else if let Some(path) = args.first() {
                    let path = Path::new(path)
                        .iter()
                        .map(|x| match x.to_str().unwrap() {
                            "~" => std::env::var("HOME").unwrap(),
                            p => p.to_string(),
                        })
                        .collect::<Vec<_>>()
                        .join("/");
                    if std::env::set_current_dir(&path).is_err() {
                        println!("{}: No such file or directory", path);
                    }
                } else {
                    std::env::set_current_dir(std::env::current_dir().unwrap()).unwrap();
                }
            }
            cmd => {
                if let Some(path) = find_command_in_path(cmd) {
                    run_executable(&path, args);
                } else {
                    println!("{}: command not found", cmd)
                }
            }
        }
    }
}
