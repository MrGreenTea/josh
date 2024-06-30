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

fn interpolate_arg(arg: &str) -> String {
    return arg.replace("~", &std::env::var("HOME").unwrap());
}

fn run_executable(cmd: &str, args: &[String]) -> Result<std::process::ExitStatus, std::io::Error> {
    // print output to stdout
    let mut child = std::process::Command::new(cmd).args(args).spawn()?;

    child.wait()
}

fn main() {
    // Wait for user input
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        let mut args = input.split_whitespace().map(|a| interpolate_arg(a));
        let cmd = args.next().unwrap();
        let args: Vec<String> = args.collect();
        match cmd.as_str() {
            "exit" => break,
            "echo" => println!("{}", args.join(" ")),
            "type" => {
                for arg in args {
                    match arg.as_str() {
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
                    let path = match path.as_str() {
                        "~" => std::env::var("HOME").unwrap(),
                        p => p.to_string(),
                    };
                    if std::env::set_current_dir(&path).is_err() {
                        println!("{}: No such file or directory", path);
                    }
                } else {
                    std::env::set_current_dir(std::env::var("HOME").unwrap()).unwrap();
                }
            }
            cmd => {
                if let Some(path) = find_command_in_path(cmd) {
                    if let Err(result) = run_executable(&path, &args) {
                        println!("{}", result);
                    }
                } else {
                    println!("{}: command not found", cmd)
                }
            }
        }
    }
}
