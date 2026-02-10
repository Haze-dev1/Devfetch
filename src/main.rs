mod cli;
mod core;
mod types;

use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use cli::Cli;
use types::ScanResult;

fn main() -> Result<()> {
    let args = Cli::parse();

    // Perform the scan
    let result = perform_scan(&args)?;

    // Output results
    if args.json {
        core::output::print_json(&result)?;
    } else {
        core::output::print_pretty(&result);
    }

    Ok(())
}

/// Orchestrate the complete scan operation
fn perform_scan(args: &Cli) -> Result<ScanResult> {
    let mut result = ScanResult::new();
    
    // Scan for global tools if requested
    if args.should_scan_global() {
        if args.verbose {
            eprintln!("Scanning PATH for developer tools...");
        }

        let mut tools = core::path_scan::discover_tools(args.verbose);
        
        // Classify discovered tools
        core::classify::classify_tools(&mut tools);

        result.global_tools = tools;

        if args.verbose {
            eprintln!("Found {} developer tools", result.global_tools.len());
        }
    }

    // Scan for project-specific information if requested
    if args.should_scan_local() {
        let target_path = args.target_path();
        
        if args.verbose {
            eprintln!("Scanning project directory: {}", target_path.display());
        }

        result.project_info = core::project_detect::detect_project(&target_path, args.verbose);

        if args.verbose {
            if result.project_info.is_some() {
                eprintln!("Project ecosystems detected");
            } else {
                eprintln!("No project markers found");
            }
        }
    }

    Ok(result)
}
