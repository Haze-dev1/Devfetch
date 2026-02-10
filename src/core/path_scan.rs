use crate::core::probe;
use crate::types::Tool;
use std::collections::HashSet;
use std::env;
use std::fs;

/// Known developer tool prefixes/patterns to include
static DEVELOPER_TOOL_PATTERNS: &[&str] = &[
    // Language runtimes
    "python", "node", "ruby", "perl", "php", "lua", "java", "javac", "scala",
    "kotlin", "swift", "go", "rust", "cargo", "deno", "bun",
    // Compilers
    "gcc", "g++", "clang", "cc", "c++", "rustc", "ghc", "ocaml",
    // Package managers
    "npm", "yarn", "pnpm", "pip", "gem", "bundle", "composer", "maven", "mvn",
    "gradle", "mix", "hex", "cabal", "stack", "lein", "rebar", "sbt",
    "poetry", "pipenv", "conda", "mamba", "conan", "vcpkg", "brew",
    // Build tools
    "make", "cmake", "ninja", "meson", "bazel", "ant", "rake", "grunt", "gulp",
    "webpack", "vite", "rollup", "parcel", "esbuild", "turbo",
    // Version control
    "git", "hg", "svn", "fossil",
    // Containers & orchestration
    "docker", "podman", "kubectl", "helm", "kind", "minikube", "compose",
    // Infrastructure
    "terraform", "ansible", "vagrant", "packer",
    // Databases
    "psql", "mysql", "sqlite", "mongo", "redis",
    // .NET
    "dotnet", "csc", "fsc", "nuget",
    // Testing
    "jest", "mocha", "pytest", "rspec", "junit",
    // Linters/formatters
    "eslint", "prettier", "black", "flake8", "pylint", "rubocop", "rustfmt", "clippy",
    // Other dev tools
    "jq", "yq", "protoc", "thrift",
];

/// Check if a tool name matches developer tool patterns
fn is_likely_dev_tool(name: &str) -> bool {
    let name_lower = name.to_lowercase();
    
    // Check against known patterns
    for pattern in DEVELOPER_TOOL_PATTERNS {
        if name_lower.starts_with(pattern) || name_lower == *pattern {
            return true;
        }
    }
    
    // Check for version suffixes (python3, node18, etc.)
    if name_lower.chars().rev().take(2).all(|c| c.is_numeric()) {
        let base = name_lower.trim_end_matches(|c: char| c.is_numeric());
        for pattern in DEVELOPER_TOOL_PATTERNS {
            if base == *pattern || base.starts_with(pattern) {
                return true;
            }
        }
    }
    
    false
}

/// Scan PATH directories for developer tools
pub fn scan_path() -> Vec<String> {
    let path_var = match env::var("PATH") {
        Ok(p) => p,
        Err(_) => return Vec::new(),
    };

    let mut executables = HashSet::new();
    
    for dir in env::split_paths(&path_var) {
        if let Ok(entries) = fs::read_dir(&dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_file() {
                        if let Some(name) = entry.file_name().to_str() {
                            // Only include likely developer tools
                            if is_likely_dev_tool(name) {
                                executables.insert(name.to_string());
                            }
                        }
                    }
                }
            }
        }
    }

    executables.into_iter().collect()
}

/// Discover developer tools from PATH
pub fn discover_tools(verbose: bool) -> Vec<Tool> {
    let executables = scan_path();
    let mut tools = Vec::new();
    let mut seen_names = HashSet::new();

    if verbose {
        eprintln!("Found {} potential executables", executables.len());
    }

    for exe_name in executables {
        // Skip duplicates
        if seen_names.contains(&exe_name) {
            continue;
        }

        // Try to find the executable in PATH
        let exe_path = match which::which(&exe_name) {
            Ok(path) => path,
            Err(_) => continue,
        };

        // Probe for version
        let probe_result = probe::probe_version(exe_path.to_str().unwrap_or(&exe_name));

        // Only include tools that respond to version probes
        if probe_result.success && probe::looks_like_version(&probe_result.output) {
            // Classification will be done by classify module
            let version = probe_result.version.clone();
            tools.push(Tool {
                name: exe_name.clone(),
                path: exe_path,
                version: probe_result.version,
                category: crate::types::ToolCategory::Unknown, // Will be classified later
            });
            
            seen_names.insert(exe_name);

            if verbose {
                eprintln!("Discovered: {} {:?}", seen_names.len(), version);
            }
        }
    }

    tools
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_path() {
        let executables = scan_path();
        // PATH should have at least some executables
        assert!(!executables.is_empty());
    }

    #[test]
    fn test_discover_tools() {
        let tools = discover_tools(false);
        // Should find at least some developer tools
        assert!(!tools.is_empty());
    }
}
