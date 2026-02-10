use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Represents a discovered developer tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tool {
    pub name: String,
    pub path: PathBuf,
    pub version: Option<String>,
    pub category: ToolCategory,
}

/// Categories for discovered tools based on heuristics
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ToolCategory {
    LanguageToolchain,
    PackageManager,
    BuildSystem,
    DeveloperTool,
    Unknown,
}

impl ToolCategory {
    pub fn display_name(&self) -> &str {
        match self {
            ToolCategory::LanguageToolchain => "Language Toolchains",
            ToolCategory::PackageManager => "Package Managers",
            ToolCategory::BuildSystem => "Build Systems",
            ToolCategory::DeveloperTool => "Developer Tools",
            ToolCategory::Unknown => "Other Tools",
        }
    }
}

/// Represents a project marker file and its associated ecosystem
#[derive(Debug, Clone)]
pub struct ProjectMarker {
    pub file_name: String,
    pub ecosystem: String,
    pub commands: Vec<EcosystemCommand>,
}

/// Command to extract ecosystem information
#[derive(Debug, Clone)]
pub struct EcosystemCommand {
    pub tool: String,
    pub args: Vec<String>,
    pub parser: CommandParser,
}

/// How to parse command output
#[derive(Debug, Clone)]
pub enum CommandParser {
    Json,
    PlainText,
}

/// Detected project information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectInfo {
    pub path: PathBuf,
    pub markers: Vec<DetectedMarker>,
    pub ecosystems: HashMap<String, EcosystemInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DetectedMarker {
    pub file: String,
    pub ecosystem: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EcosystemInfo {
    pub name: String,
    pub tool_version: Option<String>,
    pub dependencies: Option<DependencyInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub count: usize,
    pub sample: Vec<String>,
}

/// Complete scan result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanResult {
    pub global_tools: Vec<Tool>,
    pub project_info: Option<ProjectInfo>,
}

impl ScanResult {
    pub fn new() -> Self {
        Self {
            global_tools: Vec::new(),
            project_info: None,
        }
    }
}

/// Version probe result
#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub success: bool,
    pub output: String,
    pub version: Option<String>,
}
