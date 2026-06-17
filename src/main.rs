use std::io::{self, Write};
use std::path::{Path, PathBuf};

const BUILTINS: &[&str] = &["echo", "exit", "type"];

fn main() {
    let stdin = io::stdin();
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        if stdin.read_line(&mut line).unwrap() == 0 {
            break; // EOF (Ctrl-D)
        }

        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        let (command, args) = match line.split_once(' ') {
            Some((cmd, rest)) => (cmd, rest.trim()),
            None => (line, ""),
        };

        match command {
            "exit" => break,
            "echo" => println!("{args}"),
            "type" => type_builtin(args),
            _ => println!("{line}: command not found"),
        }
    }
}

/// Implements the `type` builtin: identify whether `name` is a builtin,
/// an executable found in `PATH`, or unknown.
fn type_builtin(name: &str) {
    if BUILTINS.contains(&name) {
        println!("{name} is a shell builtin");
    } else if let Some(path) = find_in_path(name) {
        println!("{name} is {}", path.display());
    } else {
        println!("{name}: not found");
    }
}

/// Search each directory in `PATH` for an executable file named `name`.
fn find_in_path(name: &str) -> Option<PathBuf> {
    let path_var = std::env::var_os("PATH")?;
    std::env::split_paths(&path_var)
        .map(|dir| dir.join(name))
        .find(|candidate| is_executable(candidate))
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
