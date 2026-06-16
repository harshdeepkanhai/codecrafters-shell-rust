#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("echo") {
            println!("{}", &command[5..]);
        } else if command.starts_with("type") {
            if let Some(arg) = command.get(5..) {
                if ["echo", "exit", "type"].contains(&arg) {
                    println!("{} is a shell builtin", arg);
                } else {
                    println!("{}: not found", arg);
                }
            }
        } else {
            println!("{}: command not found", command.trim());
        }
    }
}
