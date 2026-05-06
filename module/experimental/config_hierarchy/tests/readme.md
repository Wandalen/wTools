# tests/

This directory contains all functional and integration tests for the crate.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `basic_operations_tests.rs` | Core configuration operations and functionality |
| `concurrent_access_tests.rs` | Concurrent configuration access and thread safety |
| `configurability_tests.rs` | Custom trait implementations and path/env var customization |
| `display_tests.rs` | Configuration display and output formatting |
| `dual_pattern_tests.rs` | Dual configuration pattern handling |
| `edge_cases_tests.rs` | Edge cases, special values, and boundary conditions |
| `feature_tests.rs` | Feature-gated functionality |
| `hierarchy_tests.rs` | Hierarchical configuration resolution and precedence |
| `path_standards_tests.rs` | Configuration file path standards and resolution |
| `scope_operations_tests.rs` | Scope-specific configuration operations and resolution |
| `type_detection_tests.rs` | Configuration value type detection and inference |
| `validator_tests.rs` | ConfigValidator trait and NoValidator no-op implementation |
| `docs/` | Test specification surface for all `docs/` entity instances |
| `manual/` | Manual testing plan and procedures |

### Scope

This test suite covers the config_hierarchy crate's hierarchical configuration system:

**In Scope:**
- Configuration hierarchy (hierarchical resolution and precedence)
- Basic configuration operations (core operations and functionality)
- ConfigPaths customization (custom trait implementations, path/env var customization)
- Scope-based operations (scope-specific configuration operations and resolution)
- Type detection and inference (configuration value type detection)
- Dual pattern support (dual configuration pattern handling)
- Path standardization (configuration file path standards and resolution)
- Display and formatting (configuration display and output)
- Feature-gated functionality (feature-specific configuration operations)
- Edge case handling (edge cases, special values, boundary conditions)
- Concurrency and thread safety (concurrent configuration access)
- Validator trait and no-op implementation

**Out of Scope:**
- Configuration file format parsing (YAML, TOML, JSON - uses external parsers)
- Configuration validation schemas
- Remote configuration sources
- Performance benchmarks
