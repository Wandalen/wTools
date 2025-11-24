# willbe

Utility to publish multi-crate and multi-workspace environments and maintain their consistency.

## Overview

`willbe` is a comprehensive workspace management CLI tool designed for complex Rust projects with multiple crates. It handles the intricate dependencies between crates during publishing, automates CI/CD workflow generation, and provides tools for analyzing and maintaining workspace consistency.

The tool is exposed through the `will` binary (also available as `cargo will` and `willbe`) and uses the `wca` command-line argument parsing framework for its interface.

### Scope

#### Responsibility

willbe is responsible for managing multi-crate Rust workspaces, including publishing crates in correct dependency order, generating CI/CD workflows, analyzing dependencies, running comprehensive tests, and maintaining documentation consistency.

#### In-Scope

- **Multi-crate publishing**: Publish crates in topologically sorted dependency order
- **Version management**: Automatic version bumping across workspace
- **CI/CD generation**: Generate GitHub Actions workflows for workspace crates
- **Health tables**: Generate README health/status tables for all crates
- **Dependency analysis**: Analyze and report dependency relationships
- **Test execution**: Run tests with various feature combinations
- **Feature analysis**: Analyze and report crate feature configurations
- **Workspace listing**: List crates with filtering and formatting options
- **Header generation**: Update module headers and main README headers
- **Workspace scaffolding**: Initialize new workspace structures
- **Deploy configuration**: Generate deployment configurations

#### Out-of-Scope

- **Crate development**: No code generation or scaffolding beyond workspace structure
- **Version control**: No git operations beyond reading repository state
- **Package registry**: No crates.io account management
- **Build system**: Uses cargo, no custom build logic
- **IDE integration**: CLI-only tool

#### Boundaries

- **Upstream**: Depends on cargo_metadata for workspace introspection, cargo for building/publishing
- **Downstream**: Used by developers managing multi-crate workspaces
- **External services**: Interacts with crates.io for publishing, GitHub for CI/CD templates

## Architecture

### Module Structure

```
willbe/
├── src/
│   ├── lib.rs              # Crate root with mod_interface
│   ├── error.rs            # Error types and handling
│   ├── wtools.rs           # Internal tool re-exports
│   ├── bin/
│   │   ├── will.rs         # Main binary entry point
│   │   ├── willbe.rs       # Alternative binary name
│   │   └── cargo-will.rs   # Cargo subcommand entry
│   ├── entity/             # Core data structures
│   │   ├── workspace.rs    # Workspace representation
│   │   ├── workspace_graph.rs
│   │   ├── package.rs      # Package/crate representation
│   │   ├── manifest.rs     # Cargo.toml handling
│   │   ├── dependency.rs   # Dependency management
│   │   ├── version.rs      # Version handling
│   │   ├── features.rs     # Feature configuration
│   │   ├── test.rs         # Test configuration
│   │   ├── publish.rs      # Publish state tracking
│   │   ├── files/          # File type abstractions
│   │   └── ...
│   ├── command/            # CLI command definitions
│   │   ├── publish.rs      # Publish command
│   │   ├── test.rs         # Test command
│   │   ├── list.rs         # List command
│   │   ├── cicd_renew.rs   # CI/CD generation
│   │   └── ...
│   ├── action/             # Action implementations
│   │   ├── publish.rs      # Publishing logic
│   │   ├── test.rs         # Test execution logic
│   │   ├── cicd_renew.rs   # CI/CD generation logic
│   │   └── ...
│   └── tool/               # Utility functions
│       ├── cargo.rs        # Cargo command execution
│       ├── git.rs          # Git repository utilities
│       ├── graph.rs        # Dependency graph utilities
│       └── ...
├── tests/
├── Cargo.toml
├── readme.md
└── spec.md
```

### Layer Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                         CLI Layer                            │
│    bin/will.rs, bin/willbe.rs, bin/cargo-will.rs            │
├─────────────────────────────────────────────────────────────┤
│                      Command Layer                           │
│    command/*.rs - CLI argument parsing via wca               │
├─────────────────────────────────────────────────────────────┤
│                      Action Layer                            │
│    action/*.rs - Business logic implementation               │
├─────────────────────────────────────────────────────────────┤
│                      Entity Layer                            │
│    entity/*.rs - Data structures and domain models           │
├─────────────────────────────────────────────────────────────┤
│                       Tool Layer                             │
│    tool/*.rs - Low-level utilities (cargo, git, http, etc.)  │
└─────────────────────────────────────────────────────────────┘
```

## Public API

### Binary Interface

willbe exposes three binary entry points that all delegate to the same logic:

- `will` - Primary binary name
- `willbe` - Alternative name
- `cargo will` - Cargo subcommand interface

### CLI Commands

#### `.publish`

Publish crates in dependency order.

```bash
# Publish all modified crates
will .publish

# Dry run (show what would be published)
will .publish dry:1

# Publish specific packages
will .publish packages:[ crate1, crate2 ]
```

#### `.test`

Run tests with various configurations.

```bash
# Run all tests
will .test

# Test with specific power level (feature combinations)
will .test power:2

# Test specific packages
will .test packages:[ crate1 ]
```

#### `.list`

List workspace crates.

```bash
# List all crates
will .list

# List with tree format
will .list format:tree

# Filter by path pattern
will .list path_glob:module/core/*
```

#### `.cicd.renew`

Generate CI/CD workflow files.

```bash
# Regenerate all CI/CD workflows
will .cicd.renew
```

#### `.readme.health.table.renew`

Generate health status table in README.

```bash
# Update health table
will .readme.health.table.renew
```

#### `.readme.headers.renew`

Update README headers across workspace.

```bash
# Regenerate all module headers
will .readme.headers.renew
```

#### `.features`

Analyze crate features.

```bash
# List features for all crates
will .features
```

#### `.publish.diff`

Show differences between local and published versions.

```bash
# Show diff for all crates
will .publish.diff
```

### Library API

```rust
use willbe::{ run, action, entity };

// Run CLI with arguments
fn main() -> Result< (), willbe::error::untyped::Error >
{
  let args: Vec< String > = std::env::args().collect();
  willbe::run( args )
}
```

### Key Entities

```rust
// Workspace representation
pub struct Workspace
{
  // Cargo metadata for workspace
  metadata: cargo_metadata::Metadata,
}

// Package/crate representation
pub struct Package
{
  manifest_path: AbsolutePath,
  name: String,
  version: semver::Version,
  // ...
}

// Dependency graph
pub struct WorkspaceGraph
{
  graph: petgraph::Graph< PackageId, DependencyKind >,
}

// Publishing report
pub struct PublishReport
{
  packages: Vec< PackagePublishReport >,
  // ...
}
```

## Usage Patterns

### Publishing Workflow

```bash
# 1. Check what would be published
will .publish dry:1

# 2. Review the publish order
will .list format:tree

# 3. Publish all modified crates
will .publish
```

### CI/CD Setup

```bash
# Generate GitHub Actions workflows
will .cicd.renew

# This creates:
# - .github/workflows/module_<crate>_push.yml for each crate
# - Proper test and publish jobs
```

### Health Table Generation

```bash
# Add to main README.md
will .readme.health.table.renew

# Generates a table showing:
# - Build status badges
# - Documentation links
# - Version information
```

### Comprehensive Testing

```bash
# Test with all feature combinations (high power)
will .test power:3

# Test only core modules
will .test path_glob:module/core/*
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `progress_bar` | ✓ | Show progress indicators during operations |
| `tracing` | - | Enable tracing/logging output |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `cargo_metadata` | Parse Cargo workspace structure |
| `petgraph` | Dependency graph construction |
| `toml_edit` | Manifest file editing |
| `semver` | Version parsing and comparison |
| `handlebars` | Template rendering for CI/CD |
| `ureq` | HTTP client for crates.io |
| `rayon` | Parallel test execution |
| `indicatif` | Progress bar display |
| `wca` | CLI argument parsing |
| `error_tools` | Error handling |
| `former` | Builder pattern |
| `mod_interface` | Module organization |

### Potential Consumers

- Multi-crate Rust workspace maintainers
- CI/CD pipeline automation
- Monorepo management
- Open source project maintainers

## Design Rationale

### Why Layer Architecture?

The command/action/entity/tool layering provides:
1. **Separation of concerns**: CLI parsing separate from business logic
2. **Testability**: Actions can be tested without CLI
3. **Reusability**: Entities and tools shared across commands
4. **Maintainability**: Clear boundaries between concerns

### Why Topological Publishing?

Crates in a workspace have dependencies on each other. Publishing must respect this order:
1. Build dependency graph
2. Topologically sort packages
3. Publish in order, ensuring each dependency is available

### Why Generate CI/CD?

Manual CI/CD configuration for multi-crate workspaces is:
1. Error-prone (many crates, many files)
2. Inconsistent (different patterns per crate)
3. Hard to maintain (changes require updating many files)

Generating workflows ensures consistency and reduces maintenance burden.

## Testing Strategy

### Test Categories

1. **Unit tests**: Entity and tool function tests
2. **Integration tests**: Full command execution
3. **Smoke tests**: Basic CLI invocation

### Running Tests

```bash
# Standard tests
cargo test

# Full test suite
cargo test --features full

# With progress bar
cargo test --features progress_bar
```

## Future Considerations

### Potential Enhancements

1. **Workspace templates**: Scaffolding for new crates
2. **Changelog generation**: Auto-generate changelogs from git
3. **Dependency updates**: Automated dependency version bumping
4. **Custom CI providers**: Support for GitLab, CircleCI, etc.
5. **Interactive mode**: TUI for complex operations

### Known Limitations

1. **GitHub-centric**: CI/CD generation targets GitHub Actions
2. **crates.io only**: No private registry support
3. **Sequential publishing**: Crates published one at a time
4. **No rollback**: Failed publishes require manual intervention

## Adoption Guidelines

### When to Use

- Managing workspaces with 5+ interconnected crates
- Need automated CI/CD for multi-crate projects
- Want consistent publishing across workspace
- Need visibility into dependency relationships

### When Not to Use

- Single-crate projects (use cargo directly)
- Workspaces without inter-crate dependencies
- Need private registry support
- Require non-GitHub CI/CD

### Integration Pattern

```bash
# Install willbe
cargo install willbe

# Initialize workspace CI/CD
cd my-workspace
will .cicd.renew

# Add to CI pipeline
# - Run `will .test` for testing
# - Run `will .publish` for releases
```

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `cargo-release` | Alternative publishing tool |
| `cargo-workspaces` | Similar workspace management |
| `release-plz` | Automated releases |
| `wca` | Internal - CLI framework |
| `crates_tools` | Internal - crates.io interaction |

## References

- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [crates.io API](https://crates.io/api-docs)
- [GitHub Actions](https://docs.github.com/en/actions)
