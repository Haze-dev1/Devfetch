use crate::types::{Tool, ToolCategory};
use std::path::Path;

/// Classify tools based on heuristics
pub fn classify_tools(tools: &mut [Tool]) {
    for tool in tools.iter_mut() {
        tool.category = classify_tool(&tool.name, &tool.path);
    }
}

/// Classify a single tool based on name, path, and output patterns
fn classify_tool(name: &str, path: &Path) -> ToolCategory {
    let name_lower = name.to_lowercase();
    let path_str = path.to_string_lossy().to_lowercase();

    // Language Toolchains - compilers, interpreters, runtimes
    if is_language_toolchain(&name_lower, &path_str) {
        return ToolCategory::LanguageToolchain;
    }

    // Package Managers
    if is_package_manager(&name_lower) {
        return ToolCategory::PackageManager;
    }

    // Build Systems
    if is_build_system(&name_lower) {
        return ToolCategory::BuildSystem;
    }

    // Developer Tools
    if is_developer_tool(&name_lower, &path_str) {
        return ToolCategory::DeveloperTool;
    }

    ToolCategory::Unknown
}

/// Detect language toolchains based on patterns
fn is_language_toolchain(name: &str, path: &str) -> bool {
    // Common language runtime/compiler patterns
    let language_patterns = [
        // Python
        "python", "python2", "python3", "pypy",
        // Node/JavaScript
        "node", "deno", "bun",
        // Ruby
        "ruby", "irb",
        // Java
        "java", "javac", "jshell",
        // Go
        "go",
        // Rust
        "rustc", "rust",
        // C/C++
        "gcc", "g++", "clang", "clang++", "cc", "c++",
        // .NET
        "dotnet", "csc", "fsc",
        // PHP
        "php",
        // Perl
        "perl",
        // Lua
        "lua", "luajit",
        // Kotlin
        "kotlinc", "kotlin",
        // Scala
        "scala", "scalac",
        // Swift
        "swift", "swiftc",
        // Dart
        "dart",
        // R
        "rscript",
        // Erlang/Elixir
        "erl", "erlc", "elixir", "iex",
        // Haskell
        "ghc", "ghci", "runhaskell",
        // Zig
        "zig",
        // Nim
        "nim",
        // Crystal
        "crystal",
        // V
        "vlang",
        // Julia
        "julia",
        // OCaml
        "ocaml", "ocamlc",
        // F#
        "fsharp", "fsharpc",
        // Clojure
        "clojure", "clj",
        // Racket
        "racket",
        // Scheme
        "scheme", "guile",
    ];

    for pattern in &language_patterns {
        if name.starts_with(pattern) || name == *pattern {
            return true;
        }
    }

    // Path-based detection (e.g., .sdkman, .nvm, .rbenv)
    let path_indicators = [
        ".sdkman", ".nvm", ".rbenv", ".pyenv", ".asdf", 
        ".rustup", ".cargo", ".local/share/virtualenvs"
    ];

    for indicator in &path_indicators {
        if path.contains(indicator) {
            return true;
        }
    }

    false
}

/// Detect package managers
fn is_package_manager(name: &str) -> bool {
    let package_managers = [
        // Node
        "npm", "yarn", "pnpm", "bun",
        // Python
        "pip", "pip3", "pipenv", "poetry", "conda", "mamba", "uv",
        // Ruby
        "gem", "bundle", "bundler",
        // PHP
        "composer",
        // Rust
        "cargo",
        // Go
        // "go" - already classified as toolchain
        // Java/JVM
        "mvn", "gradle", "ant", "sbt",
        // .NET
        "nuget",
        // Swift
        "swift", // Package manager aspect
        // Dart/Flutter
        "pub",
        // Elixir
        "mix", "hex",
        // Perl
        "cpan", "cpanm",
        // C/C++
        "conan", "vcpkg",
        // General
        "brew", "apt", "yum", "dnf", "pacman", "zypper",
        // Nix
        "nix", "nix-env",
    ];

    package_managers.contains(&name)
}

/// Detect build systems
fn is_build_system(name: &str) -> bool {
    let build_systems = [
        "make", "cmake", "ninja", "meson", "bazel", "buck",
        "gradle", "maven", "ant", "sbt",
        "rake", "grunt", "gulp", "webpack", "vite", "rollup", "parcel",
        "cargo", // Also a package manager
        "dotnet", // Also a toolchain
        "xcodebuild",
        "msbuild",
        "nant",
        "waf",
        "scons",
        "tup",
        "b2", "bjam",
    ];

    build_systems.contains(&name)
}

/// Detect general developer tools
fn is_developer_tool(name: &str, path: &str) -> bool {
    let dev_tool_patterns = [
        // Version control
        "git", "svn", "hg", "mercurial", "fossil",
        // Containers
        "docker", "podman", "kubectl", "helm", "kind", "minikube",
        // Infrastructure
        "terraform", "ansible", "vagrant", "packer",
        // Database
        "psql", "mysql", "sqlite3", "mongosh", "redis-cli",
        // Editors/IDEs
        "code", "emacs", "nvim", "neovim",
        // Linters/Formatters
        "eslint", "prettier", "black", "flake8", "pylint", "rubocop",
        "rustfmt", "clippy", "gofmt",
        // Testing
        "jest", "mocha", "pytest", "rspec",
        // Cloud CLI
        "aws", "gcloud", "az", "heroku", "netlify",
        // Debuggers
        "gdb", "lldb",
        // Profilers
        "perf", "valgrind",
    ];

    for pattern in &dev_tool_patterns {
        if name.starts_with(pattern) || name == *pattern {
            return true;
        }
    }

    // IDE/editor paths
    if path.contains("visual studio code") || path.contains("jetbrains") {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_classify_language_toolchains() {
        assert_eq!(classify_tool("python3", Path::new("/usr/bin/python3")), ToolCategory::LanguageToolchain);
        assert_eq!(classify_tool("node", Path::new("/usr/bin/node")), ToolCategory::LanguageToolchain);
        assert_eq!(classify_tool("rustc", Path::new("/usr/bin/rustc")), ToolCategory::LanguageToolchain);
        assert_eq!(classify_tool("java", Path::new("/usr/bin/java")), ToolCategory::LanguageToolchain);
    }

    #[test]
    fn test_classify_package_managers() {
        assert_eq!(classify_tool("npm", Path::new("/usr/bin/npm")), ToolCategory::PackageManager);
        assert_eq!(classify_tool("pip", Path::new("/usr/bin/pip")), ToolCategory::PackageManager);
        assert_eq!(classify_tool("cargo", Path::new("/usr/bin/cargo")), ToolCategory::PackageManager);
    }

    #[test]
    fn test_classify_build_systems() {
        assert_eq!(classify_tool("cmake", Path::new("/usr/bin/cmake")), ToolCategory::BuildSystem);
        assert_eq!(classify_tool("make", Path::new("/usr/bin/make")), ToolCategory::BuildSystem);
    }

    #[test]
    fn test_classify_developer_tools() {
        assert_eq!(classify_tool("git", Path::new("/usr/bin/git")), ToolCategory::DeveloperTool);
        assert_eq!(classify_tool("docker", Path::new("/usr/bin/docker")), ToolCategory::DeveloperTool);
    }
}
