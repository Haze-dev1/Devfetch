# devfetch

> A discovery engine for developer tools and project ecosystems

**devfetch** dynamically discovers developer tools, programming language runtimes, compilers, SDKs, and package managers on your system — without hardcoded language or package lists. It's a production-grade CLI tool written in Rust that adapts to any development environment.

#(V1 works with fedora)

## Features

- **Dynamic Discovery** - No hardcoded lists, discovers tools via PATH scanning and heuristics
- **Heuristic Classification** - Intelligently categorizes tools (language toolchains, package managers, build systems)
- **Project-Aware** - Detects ecosystem markers (package.json, Cargo.toml, etc.) and queries relevant tools
- **Universal** - Works with any language: Python, Node.js, Rust, Go, Java, Ruby, PHP, and more`
- **Fast & Safe** - Efficient PATH scanning with timeout protection and graceful error handling
- **Beautiful Output** - Clean, colorized terminal output or machine-readable JSON
- **Extensible** - Plugin-ready architecture for ecosystem-specific deep inspection

## Installation

### From Source

```bash
git clone https://github.com/yourusername/devfetch.git
cd devfetch
cargo build --release
sudo cp target/release/devfetch /usr/local/bin/
```

### Using Cargo

```bash
cargo install devfetch
```

### Basic Commands

```bash
# Inspect current directory (global + local)
devfetch

# Inspect specific directory
devfetch /path/to/project

# Show only global tools
devfetch --global

# Show only project info
devfetch --local

# Machine-readable JSON output
devfetch --json

# Verbose output for debugging
devfetch -v
```

## Supported Ecosystems

devfetch detects 20+ ecosystem markers:

| Language/Framework | Markers |
|-------------------|---------|
| JavaScript/Node.js | `package.json` |
| Python | `pyproject.toml`, `requirements.txt`, `Pipfile`, `poetry.lock` |
| Rust | `Cargo.toml` |
| Go | `go.mod` |
| Java | `pom.xml`, `build.gradle`, `build.gradle.kts` |
| Ruby | `Gemfile` |
| PHP | `composer.json` |
| Dart/Flutter | `pubspec.yaml` |
| Swift | `Package.swift` |
| Elixir | `mix.exs` |
| C/C++ | `CMakeLists.txt`, `Makefile`, `meson.build` |
| .NET | `*.csproj`, `*.fsproj` |


### Design Principles

1. **Discovery over Configuration** - No config files, everything is inferred
2. **Heuristics over Hardcoding** - Patterns and signals, not exhaustive lists
3. **Graceful Degradation** - Continue on errors, never crash
4. **Extensibility** - Easy to add ecosystem-specific plugins
5. **Performance** - Fast PATH scanning, timeout-protected execution


## Roadmap

- [ ] Parallel version probing
- [ ] Cache layer for repeated scans
- [ ] Plugin system for deep ecosystem inspection
- [ ] Container/VM detection
- [ ] CI/CD integration mode
- [ ] Diff mode (compare environments)

## Contributing

Contributions welcome! Please:

1. Maintain the no-hardcoding philosophy
2. Add heuristics, not lists
3. Keep error handling graceful
4. Include tests for new features

## License

MIT License - see LICENSE file for details

## Acknowledgments

Built with:
- [clap](https://github.com/clap-rs/clap) - Command line argument parsing
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [colored](https://github.com/mackwic/colored) - Terminal coloring
- [which](https://github.com/harryfei/which-rs) - PATH lookup

---

**devfetch** - Know your development environment.