use crate::types::{ScanResult, Tool, ToolCategory};
use colored::*;
use std::collections::HashMap;

/// Output scan results in pretty terminal format
pub fn print_pretty(result: &ScanResult) {
    // Print global tools
    if !result.global_tools.is_empty() {
        println!("\n{}", "═══════════════════════════════════════════════════════".bright_blue().bold());
        println!("{}", "  GLOBAL DEVELOPER TOOLS".bright_blue().bold());
        println!("{}", "═══════════════════════════════════════════════════════".bright_blue().bold());
        
        print_tools_by_category(&result.global_tools);
    }

    // Print project information
    if let Some(project) = &result.project_info {
        println!("\n{}", "═══════════════════════════════════════════════════════".bright_green().bold());
        println!("{}", "  PROJECT INFORMATION".bright_green().bold());
        println!("{}", "═══════════════════════════════════════════════════════".bright_green().bold());
        
        println!("\n{} {}", "📁 Path:".bold(), project.path.display().to_string().cyan());
        
        if !project.markers.is_empty() {
            println!("\n{}", "Detected Ecosystems:".bold().yellow());
            for marker in &project.markers {
                println!("  {} {} ({})", 
                    "▸".green(),
                    marker.ecosystem.bright_white(),
                    marker.file.dimmed()
                );
            }
        }

        if !project.ecosystems.is_empty() {
            println!("\n{}", "Ecosystem Details:".bold().yellow());
            for (name, info) in &project.ecosystems {
                print!("  {} {}", "◆".cyan(), name.bright_white());
                
                if let Some(version) = &info.tool_version {
                    print!(" {}", format!("v{}", version).green());
                }
                println!();

                if let Some(deps) = &info.dependencies {
                    println!("    {} {} dependencies", "├─".dimmed(), deps.count.to_string().yellow());
                    if !deps.sample.is_empty() {
                        println!("    {} {}", "└─".dimmed(), "Sample:".dimmed());
                        for dep in &deps.sample {
                            println!("       {} {}", "•".dimmed(), dep.bright_white());
                        }
                    }
                }
            }
        }
    }

    println!();
}

/// Print tools grouped by category
fn print_tools_by_category(tools: &[Tool]) {
    let mut by_category: HashMap<ToolCategory, Vec<&Tool>> = HashMap::new();
    
    for tool in tools {
        by_category.entry(tool.category.clone()).or_default().push(tool);
    }

    // Sort categories for consistent output
    let category_order = [
        ToolCategory::LanguageToolchain,
        ToolCategory::PackageManager,
        ToolCategory::BuildSystem,
        ToolCategory::DeveloperTool,
        ToolCategory::Unknown,
    ];

    for category in &category_order {
        if let Some(tools_in_cat) = by_category.get(category) {
            if tools_in_cat.is_empty() {
                continue;
            }

            let icon = match category {
                ToolCategory::LanguageToolchain => "",
                ToolCategory::PackageManager => "",
                ToolCategory::BuildSystem => " ",
                ToolCategory::DeveloperTool => " ",
                ToolCategory::Unknown => "",
            };

            println!("\n{} {}", icon, category.display_name().bold().yellow());
            
            let mut sorted_tools = tools_in_cat.clone();
            sorted_tools.sort_by(|a, b| a.name.cmp(&b.name));

            for tool in sorted_tools {
                print!("  {} {}", "▸".green(), tool.name.bright_white());
                
                if let Some(version) = &tool.version {
                    print!(" {}", format!("v{}", version).green());
                }
                
                println!(" {}", format!("({})", tool.path.display()).dimmed());
            }
        }
    }
}

/// Output scan results in JSON format
pub fn print_json(result: &ScanResult) -> anyhow::Result<()> {
    let json = serde_json::to_string_pretty(result)?;
    println!("{}", json);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_print_json() {
        let mut result = ScanResult::new();
        result.global_tools.push(Tool {
            name: "python3".to_string(),
            path: PathBuf::from("/usr/bin/python3"),
            version: Some("3.11.0".to_string()),
            category: ToolCategory::LanguageToolchain,
        });

        assert!(print_json(&result).is_ok());
    }
}
