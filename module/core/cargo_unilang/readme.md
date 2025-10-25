# cargo_unilang

Scaffolding and health check tool for unilang CLI projects - prevents common mistakes.

## Why This Tool Exists

A real-world project ([willbe/module/lib](https://github.com/Wandalen/wTools/tree/master/module/willbe/module/lib/build.rs)) wrote **220 lines of custom build.rs** duplicating exactly what unilang provides automatically, resulting in:

- **50x performance degradation** (fake PHF using `OnceLock<HashMap>` vs real compile-time PHF)
- **Duplicate dependencies** (serde_yaml, walkdir, phf_codegen)
- **Violated all spec requirements** for zero-overhead lookups
- **4+ hours of wasted development time**

cargo_unilang prevents these mistakes by:
1. Creating correct project structure (scaffolding)
2. Detecting common anti-patterns (health checks)
3. Teaching through examples

## Installation

```bash
# From source (cargo_unilang is part of wTools workspace)
cd /path/to/wTools
cargo install --path module/core/cargo_unilang

# Or use directly from workspace
cargo run --manifest-path module/core/cargo_unilang/Cargo.toml -- .help
```

## Quick Start

### Create a New Project

```bash
# Create minimal project (recommended)
cargo_unilang .new project::my-cli

# Create full-featured project
cargo_unilang .new project::my-api template::full

# With all options
cargo_unilang .new project::my-tool \
  template::full \
  author::YourName \
  license::Apache-2.0 \
  verbosity::2
```

**What it creates**:
```
my-cli/
├── Cargo.toml       # unilang = "0.30" with warnings
├── src/main.rs      # 15-line minimal example
└── commands.yaml    # Example command definitions
```

**What it does NOT create**:
- ❌ build.rs (unilang provides this automatically)

### Check an Existing Project

```bash
# Check current directory
cargo_unilang .check

# Check specific directory
cargo_unilang .check path::./my-project

# Verbose check with debug info
cargo_unilang .check verbosity::3

# Silent check (CI mode)
cargo_unilang .check verbosity::0
```

**What it checks**:
1. Custom build.rs with unilang keywords (duplicates unilang's functionality)
2. Duplicate dependencies (serde_yaml, walkdir, phf already in unilang)
3. Deprecated API usage (CommandRegistry::new() instead of ::with_static_commands())

## Commands

All commands use CLI rulebook compliant format:
- Dot-prefix: `.new`, `.check` (not `new`, `check`)
- param::value format: `project::my-cli` (not `--project my-cli`)

### General Help

```bash
cargo_unilang .
cargo_unilang .help
```

### Create New Project

```bash
cargo_unilang .new project::<name> [OPTIONS]
```

**Parameters**:
- `project::<name>` (required) - Project name (must be valid Rust package name)
- `template::<type>` (optional) - Template: `minimal` (default), `full`
- `author::<name>` (optional) - Author for Cargo.toml
- `license::<type>` (optional) - License type (default: MIT)
- `verbosity::<level>` (optional) - Output verbosity 0-5 (default: 2)

**Exit codes**:
- `0` - Project created successfully
- `1` - Failed to create (I/O error, permissions)
- `2` - Invalid parameters
- `3` - Project directory already exists

### Check Project

```bash
cargo_unilang .check [path::<dir>] [OPTIONS]
```

**Parameters**:
- `path::<dir>` (optional) - Path to project directory (default: `.`)
- `verbosity::<level>` (optional) - Output verbosity 0-5 (default: 2)
- `fix::<bool>` (optional) - Auto-fix issues (default: false, NOT YET IMPLEMENTED)

**Exit codes**:
- `0` - All checks passed
- `1` - Issues found
- `2` - Invalid parameters
- `3` - Cannot access path

### Command-Specific Help

```bash
cargo_unilang .new.help
cargo_unilang .check.help
```

## Verbosity Levels

All commands support verbosity control via `verbosity::<level>`:

- `verbosity::0` - Silent (exit code only, perfect for CI)
- `verbosity::1` - Single line output
- `verbosity::2` - Concise multi-line (DEFAULT, recommended)
- `verbosity::3` - With debug information
- `verbosity::4` - Lots of debug
- `verbosity::5` - Maximum debug

**Examples**:
```bash
# Silent mode for CI pipelines
cargo_unilang .check verbosity::0
echo $?  # Check exit code

# Single line for scripts
cargo_unilang .new project::test verbosity::1

# Debug mode for troubleshooting
cargo_unilang .check verbosity::3
```

## Examples

### Create and Verify Project

```bash
# Create project
cargo_unilang .new project::my-cli

# Verify structure
cd my-cli
ls -la
# Cargo.toml  commands.yaml  src/

# No build.rs! (unilang provides it)
ls build.rs
# ls: cannot access 'build.rs': No such file or directory

# Build and run
cargo build
cargo run -- .help
```

### Check for Common Mistakes

```bash
# Example: Project with issues
cd problematic-project

cargo_unilang .check

# Output:
# ❌ PROBLEMS DETECTED:
#
#   1. Custom build.rs found
#      Location: ./build.rs (220 lines)
#      Issue: Duplicates unilang's built-in build system
#      Fix: Delete build.rs - unilang provides this automatically
#
#   2. Duplicate dependencies
#      Location: Cargo.toml [dependencies]
#      Issue: serde_yaml, walkdir already provided by unilang
#      Fix: Remove serde_yaml, walkdir from Cargo.toml
#
# Summary: 2 issue(s) found
```

### CI Integration

```bash
#!/bin/bash
# In your CI pipeline

# Check project for unilang anti-patterns
cargo_unilang .check verbosity::0

if [ $? -ne 0 ]; then
  echo "Unilang project has issues - run 'cargo_unilang .check' for details"
  exit 1
fi

echo "Unilang project structure is correct"
```

## What Problems Does This Prevent?

### Problem 1: Custom build.rs (220-line greet_cli mistake)

**❌ Wrong** (what greet_cli did):
```rust
// build.rs - 220 lines of code duplicating unilang
fn main() {
  // Parse YAML files
  let files = discover_yaml_files();
  for file in files {
    let yaml = fs::read_to_string(file)?;
    let commands: Vec<Command> = serde_yaml::from_str(&yaml)?;
    // ... generate PHF maps manually
  }
}
```

**✅ Correct** (what cargo_unilang creates):
```
NO build.rs file at all!
```

Unilang's build.rs does this automatically.

### Problem 2: Duplicate Dependencies

**❌ Wrong**:
```toml
[dependencies]
unilang = "0.30"
serde_yaml = "0.9"    # ❌ Already in unilang
walkdir = "2.0"       # ❌ Already in unilang

[build-dependencies]
phf_codegen = "0.11"  # ❌ Already in unilang
```

**✅ Correct**:
```toml
[dependencies]
unilang = "0.30"      # ✅ Only dependency needed
```

### Problem 3: Deprecated API

**❌ Wrong**:
```rust
let registry = CommandRegistry::new();  // Deprecated
```

**✅ Correct**:
```rust
let registry = StaticCommandRegistry::from_commands(&STATIC_COMMANDS);
```

## CLI Rulebook Compliance

cargo_unilang follows all universal CLI rulebook standards:

- ✅ **Dot-prefix commands**: `.new`, `.check` (not `new`, `check`)
- ✅ **param::value format**: `project::my-cli` (not `--project my-cli`)
- ✅ **Help system**: `.`, `.help`, `.command.help`
- ✅ **Verbosity**: Levels 0-5 for all commands
- ✅ **Exit codes**: 0 (success), 1 (error), 2 (invalid params), 3 (validation failure)
- ✅ **Input validation**: Security-compliant parameter validation
- ✅ **Uses unilang framework**: Practices what it preaches

## Meta-Compliance

cargo_unilang itself is built using unilang and demonstrates correct usage:

- ✅ Has `commands.yaml` (processed by unilang's build.rs)
- ✅ NO custom build.rs
- ✅ NO duplicate dependencies (only unilang, toml_edit, walkdir)
- ✅ Uses correct API (not deprecated methods)
- ✅ Follows CLI rulebook standards

**Credibility**: The tool practices what it preaches.

## Testing

cargo_unilang has comprehensive test coverage:

```bash
# Run all tests (75 tests total)
cargo test

# Run only integration tests (38 tests)
cargo test --test integration_test

# Run specific test
cargo test test_new_creates_correct_structure
```

**Test Coverage**:
- 37 unit tests (validation, templates, checks)
- 38 integration tests (command format, help, verbosity, exit codes, functional)
- **Total**: 75 tests, 100% passing

## Architecture

```
cargo_unilang/
├── src/
│   ├── main.rs              # Entry point, command dispatcher
│   ├── commands/
│   │   ├── help.rs          # Help system (CLI rulebook compliant)
│   │   ├── new.rs           # .new command handler
│   │   └── check.rs         # .check command handler
│   ├── templates/
│   │   ├── cargo_toml.rs    # Cargo.toml template
│   │   ├── main_rs.rs       # main.rs templates (minimal/full)
│   │   └── commands_yaml.rs # commands.yaml templates
│   └── checks/
│       ├── build_rs.rs      # Detect custom build.rs
│       ├── deps.rs          # Detect duplicate dependencies
│       └── api.rs           # Detect deprecated API
├── tests/
│   └── integration_test.rs  # 38 integration tests
├── commands.yaml            # CLI command definitions
└── Cargo.toml
```

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run locally
cargo run -- .help

# Build release
cargo build --release
```

## Troubleshooting

### "Unknown command" error
Make sure you use dot-prefix: `.new` not `new`

### "Invalid parameter format" error
Use `param::value` format: `project::my-cli` not `--project my-cli`

### Check reports false positive
cargo_unilang uses proper TOML parsing - it won't trigger on comments. If you get a false positive, please file an issue.

### Project already exists
Use a different name or remove the existing directory first.

## License

MIT (same as unilang)

## Links

- [unilang documentation](https://docs.rs/unilang/)
- [wTools repository](https://github.com/Wandalen/wTools)
- [CLI rulebook](../../genai/cli/cli.rulebook.md)

## Contributing

Contributions welcome! Please ensure:
1. All tests pass (`cargo test`)
2. Follow CLI rulebook standards
3. Add tests for new functionality
4. Update documentation

---

**Made with unilang** - This tool uses unilang framework and serves as a reference implementation for correct usage.
