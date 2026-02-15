use anyhow::Result;
use std::process::{Command, Output, Stdio};
use std::time::{Duration, Instant};

/// Maximum time to wait for a command to execute (in milliseconds)
const COMMAND_TIMEOUT_MS: u64 = 1500;

/// Safely execute a command with timeout and error handling.
/// Properly kills the child process if it exceeds the timeout.
pub fn execute_command(program: &str, args: &[&str]) -> Result<Output> {
    let mut child = Command::new(program)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let start = Instant::now();
    let timeout = Duration::from_millis(COMMAND_TIMEOUT_MS);

    loop {
        match child.try_wait() {
            Ok(Some(_status)) => {
                // Process finished — collect output
                return child.wait_with_output().map_err(Into::into);
            }
            Ok(None) => {
                // Still running
                if start.elapsed() >= timeout {
                    // Kill the child process to avoid zombies
                    let _ = child.kill();
                    let _ = child.wait(); // Reap the process
                    return Err(anyhow::anyhow!("Command timed out: {}", program));
                }
                std::thread::sleep(Duration::from_millis(25));
            }
            Err(e) => return Err(e.into()),
        }
    }
}

/// Execute command and return combined stdout+stderr as string if successful.
/// Many tools (e.g. `java -version`) write version info to stderr,
/// so we capture both streams.
pub fn execute_for_output(program: &str, args: &[&str]) -> Option<String> {
    match execute_command(program, args) {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);

            if output.status.success() {
                // Prefer stdout if it has content, otherwise fall back to stderr
                let text = if stdout.trim().is_empty() {
                    stderr.to_string()
                } else {
                    stdout.to_string()
                };
                if text.trim().is_empty() { None } else { Some(text) }
            } else {
                // Some tools exit non-zero for --version but still print useful output
                // (e.g. some versions of java). Try stderr then stdout.
                let text = if !stderr.trim().is_empty() {
                    stderr.to_string()
                } else {
                    stdout.to_string()
                };
                if text.trim().is_empty() { None } else { Some(text) }
            }
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
