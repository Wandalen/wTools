# API: ConfigManager Type

### Scope

- **Purpose**: Define the contract for the zero-cost manager type that composes the three configurable traits.
- **Responsibility**: Document all public operations, feature-gating, and the zero-runtime-cost guarantee.
- **In Scope**: All `ConfigManager` public operations; feature-gated method groups; type parameter constraints.
- **Out of Scope**: Individual trait contracts (→ api/001, api/002, api/003); resolution algorithm (→ algorithm/002); file format (→ format/001).

### Abstract

`ConfigManager` is the crate's primary public type. It is a zero-cost composition of three configurable traits: path config, defaults, and validation. The type carries no runtime data — all operations are stateless and resolved at compile time via the three type parameters. All public methods are associated functions, not instance methods.

### Operations

#### Resolution (always available)

| Operation | Purpose |
|-----------|---------|
| Resolve single value | Look up one parameter across all 6 priority levels; return value + source label |
| Resolve all values | Resolve all declared parameters plus any undeclared keys found in config files |

#### File I/O *(requires `file_ops` feature)*

| Operation | Purpose |
|-----------|---------|
| Load global config | Read key-value pairs from the global config file path |
| Save global config | Write the config map to the global config file, creating metadata |
| Load local config | Read key-value pairs from a specific discovered local config path |
| Save local config | Write to a specific local config path |
| Delete config | Remove a config file at a given path |
| Atomic modify | Read-lock, modify, write-unlock a config file in a single transaction |

#### Validation (always available)

| Operation | Purpose |
|-----------|---------|
| Validate parameter | Call the validator's per-parameter hook for a single resolved value |
| Validate all | Call the validator's cross-parameter hook for the full resolved map |

#### Display *(feature-gated)*

| Operation | Feature required | Purpose |
|-----------|-----------------|---------|
| Display as table | `display_table` | Format the config map as a columnar table |
| Display as JSON | `display_json` | Format the config map as a JSON document |
| Display as YAML | `display_yaml` | Format the config map as a YAML document |

### Zero-Cost Guarantee

`ConfigManager` has no fields. All type parameters are carried as phantom markers — zero bytes at runtime. There is no heap allocation from creating or using the manager type. All dispatch goes through compile-time monomorphization.

### Error Handling

File I/O operations return `Result` — callers must handle file-not-found, permission errors, and lock acquisition failures. Validation operations return a collected error list — never panic. Resolution operations are infallible — missing values resolve to null with default source label.

### Compatibility Guarantees

- Adding a new default-implemented method is a non-breaking change
- Changing the type parameters at a call site is a compile-time change only — no runtime cost
- Removing display or file_ops feature gates is a non-breaking change for callers not using those methods

### APIs

| File | Relationship |
|------|--------------|
| [api/001_config_paths_trait.md](001_config_paths_trait.md) | `P` type parameter — controls all path derivation |
| [api/002_config_defaults_trait.md](002_config_defaults_trait.md) | `D` type parameter — provides defaults and parameter list |
| [api/003_config_validator_trait.md](003_config_validator_trait.md) | `V` type parameter — validation hooks |

### Algorithms

| File | Relationship |
|------|--------------|
| [algorithm/002_resolution_waterfall.md](../algorithm/002_resolution_waterfall.md) | Algorithm called by resolution operations |

### Features

| File | Relationship |
|------|--------------|
| [feature/001_config_hierarchy.md](../feature/001_config_hierarchy.md) | Feature this type implements |

### Invariants

| File | Relationship |
|------|--------------|
| [invariant/001_resolution_hierarchy.md](../invariant/001_resolution_hierarchy.md) | Priority ordering all resolution operations must follow |
| [invariant/002_file_persistence_contracts.md](../invariant/002_file_persistence_contracts.md) | Contracts for file I/O operations |
| [invariant/003_app_name_constraints.md](../invariant/003_app_name_constraints.md) | App name validation triggered by path operations |

### Patterns

| File | Relationship |
|------|--------------|
| [pattern/001_zero_cost_composition.md](../pattern/001_zero_cost_composition.md) | Composition pattern this type uses |

### Sources

| File | Relationship |
|------|--------------|
| [src/manager.rs](../../src/manager.rs) | Complete `ConfigManager` implementation |
| [src/lib.rs](../../src/lib.rs) | Public re-export surface |

### Tests

| File | Relationship |
|------|--------------|
| [tests/basic_operations_tests.rs](../../tests/basic_operations_tests.rs) | Core operation tests |
| [tests/feature_tests.rs](../../tests/feature_tests.rs) | End-to-end feature tests |
| [tests/display_tests.rs](../../tests/display_tests.rs) | Display operation tests |
| [tests/configurability_tests.rs](../../tests/configurability_tests.rs) | Type parameter variation tests |
