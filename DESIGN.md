# Design Documentation

## Architecture Overview

devfetch follows a clean, modular architecture that separates concerns and enables extensibility.

## Core Modules

### 1. `main.rs` - Orchestration
- Entry point
- Coordinates scanning workflow
- Delegates to specialized modules

### 2. `cli.rs` - Command-Line Interface
- Uses clap's derive API for type-safe argument parsing
- Validates flag combinations
- Provides sensible defaults

### 3. `types.rs` - Data Structures
- Central type definitions
- Serde-compatible for JSON serialization
- Self-documenting through strong typing

### 4. `core/` - Business Logic

#### `path_scan.rs`
- Scans `$PATH` directories
- Filters system utilities
- Deduplicates executables
- Returns raw tool list

#### `probe.rs`
- Attempts version detection with multiple strategies
- Uses regex for version extraction
- Validates that output looks version-like
- Timeout-protected

#### `classify.rs`
- Heuristic-based categorization
- Multiple signal types:
  - Binary name patterns
  - Installation path hints
  - Output keyword analysis
- Extensible category system

#### `project_detect.rs`
- Marker file detection (package.json, etc.)
- Ecosystem-specific command execution
- Dependency parsing where available
- Graceful degradation on missing tools

#### `exec.rs`
- Safe command execution primitives
- Null stdin, captured stdout/stderr
- Error isolation
- Foundation for timeout support

#### `output.rs`
- Dual output modes (pretty/JSON)
- Category-grouped display
- Colorized terminal output
- Structured JSON for automation

## Data Flow

```
User Input (CLI)
      ↓
main.rs (orchestration)
      ↓
   ┌──┴──┐
   ↓     ↓
Global  Local
Scan    Scan
   ↓     ↓
   └──┬──┘
      ↓
ScanResult
      ↓
Output (Pretty/JSON)
```

## Extension Points

### Adding Ecosystem Support

1. Add marker to `project_detect.rs::get_project_markers()`
2. Define commands for version/dependency extraction
3. Add parser logic if needed
4. No other changes required

### Adding Classification Heuristics

1. Update patterns in `classify.rs`
2. Add new ToolCategory if needed
3. Update display logic in `output.rs`

### Adding Tool Probing Strategies

1. Add new flag pattern to `probe.rs::probe_version()`
2. Update version extraction regex if needed

## Design Philosophy

### Discovery Over Configuration
No config files. Everything inferred from environment.

### Heuristics Over Hardcoding
Patterns and signals, not exhaustive lists. Maintainable and adaptable.

### Fail Gracefully
Missing tools or failed probes never crash. Always produce partial results.

### Fast by Default
Minimize I/O. No deep recursion. Timeout protection.

### Extensible Core
Plugin-ready architecture. Easy to add ecosystems without modifying core.

## Performance Characteristics

- **PATH Scanning**: O(n) where n = number of files in PATH
- **Version Probing**: O(m) where m = number of executables (serialized currently)
- **Project Detection**: O(k) where k = number of marker types (constant)
- **Classification**: O(m) based on pattern matching

## Future Optimizations

1. **Parallel Probing**: Use tokio/rayon for concurrent version checks
2. **Caching**: Store probe results with timestamps
3. **Incremental Scanning**: Only re-probe changed executables
4. **Lazy Loading**: Defer ecosystem queries until needed

## Security Considerations

- Never executes arbitrary code
- Only runs tools found in PATH
- Timeout protection prevents hangs
- No network requests
- Read-only operations

## Testing Strategy

- Unit tests for core logic (parsing, classification)
- Integration tests for full workflows
- Snapshot tests for output formats
- Property-based tests for heuristics

## Error Handling

- `anyhow::Result` for recoverable errors
- `Option` for optional data
- Explicit error propagation
- User-friendly error messages

## Code Style

- Idiomatic Rust
- Type-driven design
- Minimal dependencies
- Clear function signatures
- Self-documenting code
