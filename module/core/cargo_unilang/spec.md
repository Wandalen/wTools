# cargo_unilang

Scaffolding and health check tool for unilang CLI projects - prevents common mistakes.

## Overview

`cargo_unilang` is a developer tool that creates correctly structured unilang CLI projects and detects common anti-patterns in existing projects. It was created in response to real-world cases where developers duplicated unilang's functionality (e.g., writing 220-line custom build.rs files), resulting in significant performance degradation and wasted development time.

The tool follows CLI rulebook standards using unilang itself, serving as both a utility and a reference implementation.

### Scope

#### Responsibility

cargo_unilang is responsible for scaffolding new unilang CLI projects with correct structure and validating existing projects for common anti-patterns that duplicate or conflict with unilang's built-in functionality.

#### In-Scope

- **Project scaffolding**: Create new unilang projects with correct structure
- **Health checks**: Detect custom build.rs that duplicates unilang
- **Dependency analysis**: Find duplicate dependencies already in unilang
- **API validation**: Detect deprecated API usage patterns
- **Template generation**: Cargo.toml, main.rs, commands.yaml templates
- **Verbosity control**: Configurable output levels (0-5)
- **CLI compliance**: Follows universal CLI rulebook standards

#### Out-of-Scope

- **Code generation beyond scaffolding**: No complex code synthesis
- **Auto-fix**: Detection only, manual fixes required
- **Build system**: Uses cargo, no custom build logic
- **unilang development**: Tool for users, not unilang maintenance
- **IDE integration**: CLI-only tool

#### Boundaries

- **Upstream**: Depends on unilang for CLI framework
- **Downstream**: Used by developers creating unilang CLI projects
- **Meta-compliance**: Tool uses unilang itself (practices what it preaches)

## Architecture

### Module Structure

```
cargo_unilang/
├── src/
│   ├── main.rs              # Entry point
│   ├── lib.rs               # Library exports
│   ├── commands/            # CLI command handlers
│   │   ├── help.rs          # Help system
│   │   ├── new.rs           # .new command
│   │   └── check.rs         # .check command
│   ├── templates/           # Project templates
│   │   ├── cargo_toml.rs    # Cargo.toml generation
│   │   ├── main_rs.rs       # main.rs templates
│   │   └── commands_yaml.rs # commands.yaml templates
│   └── checks/              # Health check implementations
│       ├── build_rs.rs      # Detect custom build.rs
│       ├── deps.rs          # Detect duplicate dependencies
│       └── api.rs           # Detect deprecated API
├── tests/
│   └── integration_test.rs  # Integration tests
├── commands.yaml            # CLI command definitions
├── Cargo.toml
├── readme.md
└── spec.md
```

### Check Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Input: Project Path                     │
├─────────────────────────────────────────────────────────────┤
│                      Check Pipeline                          │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────┐          │
│  │ build_rs.rs │  │   deps.rs   │  │   api.rs    │          │
│  │ Custom      │  │ Duplicate   │  │ Deprecated  │          │
│  │ build.rs?   │  │ deps?       │  │ API usage?  │          │
│  └─────────────┘  └─────────────┘  └─────────────┘          │
├─────────────────────────────────────────────────────────────┤
│                      Output: Report + Exit Code              │
│  - Issues found (with locations and fixes)                  │
│  - Exit code (0 = pass, 1 = issues, 2 = error)              │
└─────────────────────────────────────────────────────────────┘
```

## Public API

### CLI Commands

#### `.new`

Create a new unilang project with correct structure.

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

**Parameters**:
- `project::<name>` (required) - Project name
- `template::<type>` (optional) - `minimal` (default) or `full`
- `author::<name>` (optional) - Author for Cargo.toml
- `license::<type>` (optional) - License (default: MIT)
- `verbosity::<level>` (optional) - Output level 0-5 (default: 2)

**Exit codes**:
- `0` - Project created successfully
- `1` - Failed to create
- `2` - Invalid parameters
- `3` - Project directory exists

#### `.check`

Validate an existing project for anti-patterns.

```bash
# Check current directory
cargo_unilang .check

# Check specific directory
cargo_unilang .check path::./my-project

# Verbose mode
cargo_unilang .check verbosity::3

# Silent mode (CI)
cargo_unilang .check verbosity::0
```

**Parameters**:
- `path::<dir>` (optional) - Path to project (default: `.`)
- `verbosity::<level>` (optional) - Output level 0-5 (default: 2)

**Exit codes**:
- `0` - All checks passed
- `1` - Issues found
- `2` - Invalid parameters
- `3` - Cannot access path

#### `.help`

Display help information.

```bash
cargo_unilang .
cargo_unilang .help
cargo_unilang .new.help
cargo_unilang .check.help
```

### Generated Project Structure

```
my-cli/
├── Cargo.toml       # unilang dependency, no build.rs
├── src/main.rs      # Minimal working example
└── commands.yaml    # Command definitions
```

**What is NOT generated** (intentionally):
- No `build.rs` - unilang provides this automatically

## Usage Patterns

### New Project Workflow

```bash
# Create project
cargo_unilang .new project::my-cli

# Verify structure
cd my-cli
ls -la  # Should see: Cargo.toml, commands.yaml, src/

# Verify no build.rs
ls build.rs  # Should fail - unilang provides it

# Build and run
cargo build
cargo run -- .help
```

### CI Integration

```bash
#!/bin/bash
# Check project for anti-patterns
cargo_unilang .check verbosity::0

if [ $? -ne 0 ]; then
  echo "Project has unilang anti-patterns"
  cargo_unilang .check  # Show details
  exit 1
fi
```

### Health Check Example

```bash
cd problematic-project
cargo_unilang .check

# Output:
# ❌ PROBLEMS DETECTED:
#
#   1. Custom build.rs found
#      Location: ./build.rs (220 lines)
#      Issue: Duplicates unilang's built-in build system
#      Fix: Delete build.rs
#
#   2. Duplicate dependencies
#      Location: Cargo.toml [dependencies]
#      Issue: serde_yaml, walkdir already in unilang
#      Fix: Remove from Cargo.toml
#
# Summary: 2 issue(s) found
```

## Feature Flags

No feature flags - cargo_unilang is a standalone binary tool.

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `unilang` | CLI framework (meta-compliance) |
| `toml_edit` | Cargo.toml parsing and generation |
| `walkdir` | File system traversal |

### Dev Dependencies

| Dependency | Purpose |
|------------|---------|
| `assert_cmd` | Command integration testing |
| `assert_fs` | File system testing |
| `predicates` | Test assertions |

### Potential Consumers

- Developers starting unilang CLI projects
- CI/CD pipelines validating project structure
- Teams enforcing unilang best practices

## Design Rationale

### Why This Tool Exists

A real-world project wrote 220 lines of custom build.rs duplicating unilang, resulting in:
- 50x performance degradation (fake PHF vs real compile-time PHF)
- Duplicate dependencies
- Violated spec requirements
- Wasted development time

cargo_unilang prevents these mistakes proactively.

### Why Meta-Compliance

The tool uses unilang itself, demonstrating:
1. Correct project structure
2. Proper dependency usage
3. CLI rulebook compliance
4. No custom build.rs

**Credibility**: The tool practices what it preaches.

### Why Detection-Only (No Auto-Fix)

Auto-fix is risky for build configurations:
1. Custom build.rs might have legitimate extensions
2. Dependencies might be needed for other purposes
3. Manual review ensures understanding

## Testing Strategy

### Test Categories

1. **Unit tests**: Validation, templates, check logic
2. **Integration tests**: Full command execution, exit codes

### Test Coverage

- 37 unit tests
- 38 integration tests
- 75 total tests

### Running Tests

```bash
# All tests
cargo test

# Integration tests only
cargo test --test integration_test

# Specific test
cargo test test_new_creates_correct_structure
```

## Future Considerations

### Potential Enhancements

1. **Auto-fix mode**: Optional automatic corrections
2. **More checks**: Additional anti-pattern detection
3. **Template library**: More project templates
4. **Watch mode**: Continuous validation during development

### Known Limitations

1. **Detection only**: No automatic fixes
2. **unilang-specific**: Not general-purpose scaffolding
3. **Binary only**: No library API

## Adoption Guidelines

### When to Use

- Starting new unilang CLI projects
- Validating existing unilang projects
- CI/CD pipeline checks
- Team onboarding for unilang development

### When Not to Use

- Non-unilang CLI projects
- Need auto-fix capabilities
- General Rust project scaffolding

### Integration Pattern

```bash
# Install
cargo install --path module/core/cargo_unilang

# New project workflow
cargo_unilang .new project::my-cli
cd my-cli
cargo build

# CI validation
cargo_unilang .check verbosity::0 || exit 1
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `unilang` | Upstream - CLI framework |
| `cargo-generate` | Alternative - general scaffolding |
| `willbe` | Related - workspace management |

## References

- [unilang documentation](https://docs.rs/unilang)
- [CLI rulebook](../../genai/cli/cli.rulebook.md)
- [wTools repository](https://github.com/Wandalen/wTools)
