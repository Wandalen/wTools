# component_model_meta

Procedural macro implementation for component model patterns.

## Overview

`component_model_meta` is the proc-macro companion crate for `component_model`. It provides derive macros for component-based programming patterns, enabling automatic generation of component assignment, conversion, and composition code.

**Important**: This crate should not be used directly. Use the `component_model` crate which re-exports this functionality with supporting types.

### Scope

#### Responsibility

component_model_meta is responsible for providing procedural macro implementations that generate component-related trait implementations for structs.

#### In-Scope

- **`#[derive(ComponentModel)]` macro**: Generate component model implementation
- **`#[derive(Components)]` macro**: Generate component collection
- **`#[derive(ComponentFrom)]` macro**: Generate component conversion
- **`#[derive(ComponentAssign)]` macro**: Generate component assignment
- **`#[derive(ComponentsAssign)]` macro**: Generate bulk component assignment
- **`#[derive(FromComponents)]` macro**: Generate construction from components

#### Out-of-Scope

- **Runtime types**: Provided by `component_model_types`
- **User-facing API**: Use `component_model` crate instead
- **Non-struct types**: Focused on struct components

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `component_model` crate
- **Compile-time only**: Macro expansion only

## Architecture

### Module Structure

```
component_model_meta/
├── src/
│   └── lib.rs              # Proc-macro entry points
├── Cargo.toml
├── readme.md
└── spec.md
```

## Public API

### Derive Macros

#### `#[derive(ComponentModel)]`

Generate component model for struct.

```rust
use component_model::ComponentModel;

#[derive(ComponentModel)]
struct Config
{
  host: String,
  port: u16,
}
```

#### `#[derive(ComponentAssign)]`

Generate component assignment trait.

#### `#[derive(ComponentsAssign)]`

Generate bulk component assignment.

#### `#[derive(ComponentFrom)]`

Generate conversion from component.

#### `#[derive(FromComponents)]`

Generate construction from components.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | ✓ | All features |
| `derive_component_model` | ✓ | ComponentModel derive |
| `derive_components` | ✓ | Components derive |
| `derive_component_from` | ✓ | ComponentFrom derive |
| `derive_component_assign` | ✓ | ComponentAssign derive |
| `derive_components_assign` | ✓ | ComponentsAssign derive |
| `derive_from_components` | ✓ | FromComponents derive |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |
| `component_model_types` | Runtime type definitions |
| `convert_case` | Identifier case conversion |

### Consumers

- `component_model` - Re-exports this crate's macros

## Design Rationale

### Why Component Model?

Component-based programming enables:
1. Composition over inheritance
2. Fine-grained data access
3. Flexible struct construction

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `component_model` | Parent facade crate |
| `component_model_types` | Runtime type definitions |
| `macro_tools` | Upstream syntax utilities |
