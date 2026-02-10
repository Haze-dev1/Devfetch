use crate::types::ProbeResult;
use crate::core::exec;
use regex::Regex;
use std::sync::OnceLock;

/// Version patterns to extract version numbers from command output
static VERSION_REGEX: OnceLock<Regex> = OnceLock::new();

fn get_version_regex() -> &'static Regex {
    VERSION_REGEX.get_or_init(|| {
        // Match common version patterns: 1.2.3, v1.2.3, version 1.2.3, etc.
        Regex::new(r"(?i)(?:version\s+)?v?(\d+(?:\.\d+){1,3}(?:[.-][a-z0-9]+)?)").unwrap()
    })
}

/// Probe a binary for version information
pub fn probe_version(binary_path: &str) -> ProbeResult {
    // Try different version flags in order of likelihood
    let strategies = [
        vec!["--version"],
        vec!["-v"],
        vec!["version"],
        vec!["-V"],
    ];

    for args in &strategies {
        if let Some(output) = exec::execute_for_output(binary_path, args) {
            // Check if output looks like version info
            if let Some(version) = extract_version(&output) {
                return ProbeResult {
                    success: true,
                    output: output.trim().to_string(),
                    version: Some(version),
                };
            }
        }
    }

    ProbeResult {
        success: false,
        output: String::new(),
        version: None,
    }
}

/// Extract version number from output text
pub fn extract_version(text: &str) -> Option<String> {
    let re = get_version_regex();
    
    // Take first line for more reliable parsing
    let first_line = text.lines().next()?;
    
    re.captures(first_line)
        .and_then(|cap| cap.get(1))
        .map(|m| m.as_str().to_string())
}

/// Check if output looks like version information
pub fn looks_like_version(output: &str) -> bool {
    let output_lower = output.to_lowercase();
    
    // Short output is more likely to be version info
    if output.len() > 500 {
        return false;
    }
    
    // Must have version-like content
    let has_version_keyword = output_lower.contains("version")
        || output_lower.contains("copyright")
        || output_lower.contains("release");
    
    let has_version_pattern = get_version_regex().is_match(output);
    
    has_version_keyword || has_version_pattern
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_version() {
        assert_eq!(extract_version("Python 3.11.0"), Some("3.11.0".to_string()));
        assert_eq!(extract_version("version 1.2.3"), Some("1.2.3".to_string()));
        assert_eq!(extract_version("v2.0.0-alpha"), Some("2.0.0-alpha".to_string()));
        assert_eq!(extract_version("rustc 1.75.0"), Some("1.75.0".to_string()));
    }

    #[test]
    fn test_looks_like_version() {
        assert!(looks_like_version("Python 3.11.0"));
        assert!(looks_like_version("version 1.2.3"));
        assert!(!looks_like_version("This is a long help text that goes on and on..."));
    }
}
