#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::{Path, PathBuf};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = command.trim();
        if command == "exit" {
            break;
        } else if let Some(arg) = command.strip_prefix("echo ") {
            println!("{}", arg);
        } else if let Some(arg) = command.strip_prefix("type ") {
            let arg = arg.trim();
            if ["echo", "exit", "type"].contains(&arg) {
                println!("{} is a shell builtin", arg);
            } else if let Some(path) = find_in_path(arg) {
                println!("{} is {}", arg, path.display());
            } else {
                println!("{}: not found", arg);
            }
        } else {
            println!("{}: command not found", command);
        }
    }
}

/// Search each directory in `PATH` for an executable file named `name`.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    for dir in std::env::split_paths(&path_var) {
        let candidate = dir.join(name);
        if is_executable(&candidate) {
            return Some(candidate);
        }
    }
    None
}

#[cfg(unix)]
fn is_executable(path: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    path.metadata()
        .map(|m| m.is_file() && m.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}

#[cfg(not(unix))]
fn is_executable(path: &Path) -> bool {
    path.is_file()
}