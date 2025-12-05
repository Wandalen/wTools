# tests/

This directory contains all functional and integration tests for the crate.

## Responsibility Table

| Test Suite | Responsibility | In Scope | Out of Scope (See) |
|------------|----------------|----------|-------------------|
| `hierarchy_*.rs` | Configuration hierarchy | Hierarchical configuration resolution and precedence | Basic operations (→ basic_operations tests), Scope operations (→ scope_operations tests) |
| `basic_operations_*.rs` | Basic configuration operations | Core configuration operations and functionality | Hierarchy (→ hierarchy tests), Type detection (→ type_detection tests) |
| `scope_operations_*.rs` | Scope-based operations | Scope-specific configuration operations and resolution | Hierarchy (→ hierarchy tests), Basic operations (→ basic_operations tests) |
| `type_detection_*.rs` | Type detection and inference | Configuration value type detection and inference | Basic operations (→ basic_operations tests) |
| `dual_pattern_*.rs` | Dual pattern support | Dual configuration pattern handling | Hierarchy (→ hierarchy tests), Basic operations (→ basic_operations tests) |
| `path_standards_*.rs` | Path standardization | Configuration file path standards and resolution | Hierarchy (→ hierarchy tests) |
| `display_*.rs` | Display and formatting | Configuration display and output formatting | Basic operations (→ basic_operations tests) |
| `feature_*.rs` | Feature-gated functionality | Feature-specific configuration operations | Hierarchy (→ hierarchy tests), Basic operations (→ basic_operations tests) |
| `edge_cases_*.rs` | Edge case handling | Edge cases, special values, boundary conditions | Basic operations (→ basic_operations tests), Hierarchy (→ hierarchy tests) |
| `concurrent_access_*.rs` | Concurrency and thread safety | Concurrent configuration access, thread safety | Basic operations (→ basic_operations tests) |

## Organization (10 test files)

Tests organized by domain (see Responsibility Table above).

### Scope

This test suite covers the config_hierarchy crate's hierarchical configuration system:

**In Scope:**
- Configuration hierarchy (hierarchical resolution and precedence)
- Basic configuration operations (core operations and functionality)
- Scope-based operations (scope-specific configuration operations and resolution)
- Type detection and inference (configuration value type detection)
- Dual pattern support (dual configuration pattern handling)
- Path standardization (configuration file path standards and resolution)
- Display and formatting (configuration display and output)
- Feature-gated functionality (feature-specific configuration operations)
- Edge case handling (edge cases, special values, boundary conditions)
- Concurrency and thread safety (concurrent configuration access)

**Out of Scope:**
- Configuration file format parsing (YAML, TOML, JSON - uses external parsers)
- Configuration validation schemas
- Remote configuration sources
- Performance benchmarks
