//! Help command handlers
//!
//! Implements CLI rulebook compliant help system:
//! - Universal help via `.` and `.help`
//! - Command-specific help via `.command.help`

/// General help text (for `.` and `.help`)
pub fn general_help() -> String
{
r"cargo_unilang - Scaffolding and health check tool for unilang projects

Prevents common mistakes when using unilang CLI framework:
  ❌ Custom build.rs (unilang provides this automatically)
  ❌ Duplicate dependencies (serde_yaml, walkdir, phf)
  ❌ Deprecated API (CommandRegistry::new())

USAGE:
  cargo_unilang .                                # Show this help
  cargo_unilang .help                            # Show this help
  cargo_unilang .new project::<name> [OPTIONS]   # Create new project
  cargo_unilang .check [path::<dir>] [OPTIONS]   # Check existing project

COMMANDS:
  .new       Create new unilang project with correct structure
  .check     Validate existing project for common mistakes

For command-specific help:
  cargo_unilang .new.help
  cargo_unilang .check.help

EXAMPLES:
  # Create minimal project
  cargo_unilang .new project::my-cli

  # Create full-featured project
  cargo_unilang .new project::my-api template::full verbosity::1

  # Check current directory
  cargo_unilang .check

  # Check specific project with debug output
  cargo_unilang .check path::./my-project verbosity::3

DOCUMENTATION:
  https://docs.rs/unilang/
  https://github.com/Wandalen/wTools/tree/master/module/core/unilang

EXIT CODES:
  0  Success
  1  General error (issues found, I/O error)
  2  Invalid parameters
  3  Validation failure (path doesn't exist, project exists)
".to_string()
}

/// Help for `.new` command
pub fn new_help() -> String
{
r"cargo_unilang .new - Create new unilang project

Creates correct unilang project structure, preventing common mistakes.

USAGE:
  cargo_unilang .new project::<name> [OPTIONS]

PARAMETERS:
  project::<name>      Project name (required)
                       Must be valid Rust package name
                       1-64 characters, alphanumeric/underscore/hyphen
                       Must start with letter or underscore

  template::<type>     Template type (optional, default: minimal)
                       Options: minimal, full

  author::<name>       Author name for Cargo.toml (optional)
                       Format: Name <email@example.com>

  license::<type>      License type (optional, default: MIT)
                       Common: MIT, Apache-2.0, GPL-3.0

  verbosity::<level>   Output verbosity (optional, default: 2)
                       0 = silent (exit code only)
                       1 = single line
                       2 = concise (default)
                       3 = with debug info
                       4-5 = maximum debug

WHAT IT CREATES:
  my-cli/
  ├── Cargo.toml       # unilang = 0.30 with warnings
  ├── src/main.rs      # 15-line minimal example
  └── commands.yaml    # Example command

WHAT IT DOES NOT CREATE:
  ❌ build.rs          # unilang provides this automatically

EXAMPLES:
  # Minimal project (default)
  cargo_unilang .new project::my-cli

  # Full-featured project
  cargo_unilang .new project::my-api template::full

  # With all options
  cargo_unilang .new project::my-tool template::full author::John_Doe license::Apache-2.0 verbosity::1

EXIT CODES:
  0  Project created successfully
  1  Failed to create (I/O error, permissions)
  2  Invalid parameters (invalid project name)
  3  Project directory already exists
".to_string()
}

/// Help for `.check` command
pub fn check_help() -> String
{
r"cargo_unilang .check - Validate unilang project

Checks for common mistakes and anti-patterns in unilang projects.

USAGE:
  cargo_unilang .check [path::<dir>] [OPTIONS]

PARAMETERS:
  path::<dir>          Path to project directory (optional, default: .)
                       Must be existing directory

  verbosity::<level>   Output verbosity (optional, default: 2)
                       0 = silent (exit code only)
                       1 = single line summary
                       2 = concise (default)
                       3 = with debug info
                       4-5 = maximum debug

  fix::<bool>          Auto-fix issues if possible (optional, default: false)
                       WARNING: Will modify files

CHECKS PERFORMED:
  1. Custom build.rs with unilang keywords
     Issue: Duplicates unilang's built-in build system
     Fix: Delete build.rs

  2. Duplicate dependencies in Cargo.toml
     Issue: serde_yaml, walkdir, phf already in unilang
     Fix: Remove from [dependencies] or [build-dependencies]

  3. Deprecated API usage
     Issue: CommandRegistry::new() is deprecated
     Fix: Use CommandRegistry::with_static_commands()

  4. YAML syntax errors
     Issue: Invalid YAML in *.commands.yaml files
     Fix: Correct YAML syntax

EXAMPLES:
  # Check current directory
  cargo_unilang .check

  # Check specific project
  cargo_unilang .check path::./my-project

  # Silent check (CI mode)
  cargo_unilang .check verbosity::0

  # Verbose check with debug
  cargo_unilang .check verbosity::3

EXIT CODES:
  0  All checks passed
  1  Issues found
  2  Invalid parameters
  3  Cannot access path
".to_string()
}
