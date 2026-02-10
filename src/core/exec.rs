use anyhow::Result;
use std::process::{Command, Output, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Maximum time to wait for a command to execute (in milliseconds)
const COMMAND_TIMEOUT_MS: u64 = 1000;

/// Safely execute a command with timeout and error handling
pub fn execute_command(program: &str, args: &[&str]) -> Result<Output> {
    let program = program.to_string();
    let args: Vec<String> = args.iter().map(|s| s.to_string()).collect();
    
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        let result = Command::new(&program)
            .args(&args)
            .stdin(Stdio::null())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output();
        let _ = tx.send(result);
    });
    
    match rx.recv_timeout(Duration::from_millis(COMMAND_TIMEOUT_MS)) {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(e)) => Err(e.into()),
        Err(_) => Err(anyhow::anyhow!("Command timed out")),
    }
}

/// Execute command and return stdout as string if successful
pub fn execute_for_output(program: &str, args: &[&str]) -> Option<String> {
    match execute_command(program, args) {
        Ok(output) if output.status.success() => {
            String::from_utf8(output.stdout).ok()
        }
        _ => None,
    }
}

/// Check if a command exists and is executable
pub fn command_exists(program: &str) -> bool {
    which::which(program).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_exists() {
        // These should exist on most systems
        assert!(command_exists("ls") || command_exists("dir"));
    }

    #[test]
    fn test_execute_command() {
        let result = execute_command("echo", &["test"]);
        assert!(result.is_ok());
    }
}
