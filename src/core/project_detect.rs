use crate::core::exec;
use crate::types::*;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Project markers and their associated ecosystems
fn get_project_markers() -> Vec<ProjectMarker> {
    vec![
        // JavaScript/Node
        ProjectMarker {
            file_name: "package.json".to_string(),
            ecosystem: "Node.js".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "node".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
                EcosystemCommand {
                    tool: "npm".to_string(),
                    args: vec!["list".to_string(), "--depth=0".to_string(), "--json".to_string()],
                    parser: CommandParser::Json,
                },
            ],
        },
        // Python
        ProjectMarker {
            file_name: "pyproject.toml".to_string(),
            ecosystem: "Python".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "python3".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
                EcosystemCommand {
                    tool: "pip".to_string(),
                    args: vec!["list".to_string(), "--format=json".to_string()],
                    parser: CommandParser::Json,
                },
            ],
        },
        ProjectMarker {
            file_name: "requirements.txt".to_string(),
            ecosystem: "Python".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "python3".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "Pipfile".to_string(),
            ecosystem: "Python (Pipenv)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "pipenv".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "poetry.lock".to_string(),
            ecosystem: "Python (Poetry)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "poetry".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Rust
        ProjectMarker {
            file_name: "Cargo.toml".to_string(),
            ecosystem: "Rust".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "rustc".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
                EcosystemCommand {
                    tool: "cargo".to_string(),
                    args: vec!["metadata".to_string(), "--no-deps".to_string(), "--format-version=1".to_string()],
                    parser: CommandParser::Json,
                },
            ],
        },
        // Go
        ProjectMarker {
            file_name: "go.mod".to_string(),
            ecosystem: "Go".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "go".to_string(),
                    args: vec!["version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Java (Maven)
        ProjectMarker {
            file_name: "pom.xml".to_string(),
            ecosystem: "Java (Maven)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "mvn".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Java/Kotlin (Gradle)
        ProjectMarker {
            file_name: "build.gradle".to_string(),
            ecosystem: "JVM (Gradle)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "gradle".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "build.gradle.kts".to_string(),
            ecosystem: "JVM (Gradle/Kotlin)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "gradle".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Ruby
        ProjectMarker {
            file_name: "Gemfile".to_string(),
            ecosystem: "Ruby".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "ruby".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
                EcosystemCommand {
                    tool: "bundle".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // PHP
        ProjectMarker {
            file_name: "composer.json".to_string(),
            ecosystem: "PHP".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "php".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
                EcosystemCommand {
                    tool: "composer".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Dart/Flutter
        ProjectMarker {
            file_name: "pubspec.yaml".to_string(),
            ecosystem: "Dart/Flutter".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "dart".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Swift
        ProjectMarker {
            file_name: "Package.swift".to_string(),
            ecosystem: "Swift".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "swift".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // Elixir
        ProjectMarker {
            file_name: "mix.exs".to_string(),
            ecosystem: "Elixir".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "elixir".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // C/C++
        ProjectMarker {
            file_name: "CMakeLists.txt".to_string(),
            ecosystem: "C/C++ (CMake)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "cmake".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "Makefile".to_string(),
            ecosystem: "C/C++ (Make)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "make".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "meson.build".to_string(),
            ecosystem: "C/C++ (Meson)".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "meson".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        // .NET
        ProjectMarker {
            file_name: "*.csproj".to_string(),
            ecosystem: ".NET/C#".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "dotnet".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
        ProjectMarker {
            file_name: "*.fsproj".to_string(),
            ecosystem: ".NET/F#".to_string(),
            commands: vec![
                EcosystemCommand {
                    tool: "dotnet".to_string(),
                    args: vec!["--version".to_string()],
                    parser: CommandParser::PlainText,
                },
            ],
        },
    ]
}

/// Detect project markers and ecosystem information
pub fn detect_project(path: &Path, verbose: bool) -> Option<ProjectInfo> {
    if !path.is_dir() {
        return None;
    }

    let markers = get_project_markers();
    let mut detected_markers = Vec::new();
    let mut ecosystems = HashMap::new();

    // Scan for marker files
    for marker in &markers {
        let marker_path = path.join(&marker.file_name);
        
        // Handle glob patterns for .NET projects
        let exists = if marker.file_name.contains('*') {
            check_glob_pattern(path, &marker.file_name)
        } else {
            marker_path.exists()
        };

        if exists {
            if verbose {
                eprintln!("Found marker: {}", marker.file_name);
            }

            detected_markers.push(DetectedMarker {
                file: marker.file_name.clone(),
                ecosystem: marker.ecosystem.clone(),
            });

            // Try to get ecosystem info
            if let Some(eco_info) = probe_ecosystem(marker, verbose) {
                ecosystems.insert(marker.ecosystem.clone(), eco_info);
            }
        }
    }

    if detected_markers.is_empty() {
        return None;
    }

    Some(ProjectInfo {
        path: path.to_path_buf(),
        markers: detected_markers,
        ecosystems,
    })
}

/// Check if files matching a glob pattern exist
fn check_glob_pattern(path: &Path, pattern: &str) -> bool {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            if let Some(name) = entry.file_name().to_str() {
                if pattern.contains("*.csproj") && name.ends_with(".csproj") {
                    return true;
                }
                if pattern.contains("*.fsproj") && name.ends_with(".fsproj") {
                    return true;
                }
            }
        }
    }
    false
}

/// Probe ecosystem for version and dependency information
fn probe_ecosystem(marker: &ProjectMarker, verbose: bool) -> Option<EcosystemInfo> {
    let mut tool_version = None;
    let mut dependencies = None;

    for cmd in &marker.commands {
        if !exec::command_exists(&cmd.tool) {
            if verbose {
                eprintln!("Tool not found: {}", cmd.tool);
            }
            continue;
        }

        let args: Vec<&str> = cmd.args.iter().map(|s| s.as_str()).collect();
        
        if let Some(output) = exec::execute_for_output(&cmd.tool, &args) {
            match cmd.parser {
                CommandParser::PlainText => {
                    // Extract version from first command (usually --version)
                    if tool_version.is_none() {
                        tool_version = crate::core::probe::extract_version(&output);
                    }
                }
                CommandParser::Json => {
                    // Try to parse dependency information
                    dependencies = parse_dependencies_json(&output, &marker.ecosystem);
                }
            }
        }
    }

    if tool_version.is_some() || dependencies.is_some() {
        Some(EcosystemInfo {
            name: marker.ecosystem.clone(),
            tool_version,
            dependencies,
        })
    } else {
        None
    }
}

/// Parse dependency information from JSON output
fn parse_dependencies_json(json_str: &str, ecosystem: &str) -> Option<DependencyInfo> {
    let parsed: serde_json::Value = serde_json::from_str(json_str).ok()?;

    let deps = if ecosystem.contains("Node") {
        // npm list format
        parsed.get("dependencies")
            .and_then(|d| d.as_object())
            .map(|obj| obj.keys().map(|k| k.to_string()).collect::<Vec<_>>())
    } else if ecosystem.contains("Python") {
        // pip list format
        parsed.as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(String::from))
                    .collect::<Vec<_>>()
            })
    } else if ecosystem.contains("Rust") {
        // cargo metadata format
        parsed.get("packages")
            .and_then(|p| p.as_array())
            .map(|arr| {
                arr.iter()
                    .filter_map(|v| v.get("name").and_then(|n| n.as_str()).map(String::from))
                    .collect::<Vec<_>>()
            })
    } else {
        None
    };

    deps.and_then(|mut dep_list| {
        if dep_list.is_empty() {
            return None;
        }

        let count = dep_list.len();
        dep_list.truncate(5); // Sample first 5

        Some(DependencyInfo {
            count,
            sample: dep_list,
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_project_markers() {
        let markers = get_project_markers();
        assert!(!markers.is_empty());
        assert!(markers.iter().any(|m| m.file_name == "package.json"));
        assert!(markers.iter().any(|m| m.file_name == "Cargo.toml"));
    }
}
