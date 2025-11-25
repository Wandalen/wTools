# reflect_tools_meta

Procedural macro implementation for reflection mechanisms.

## Overview

`reflect_tools_meta` is the proc-macro companion crate for `reflect_tools`. It provides the `#[derive(Reflect)]` macro that generates reflection capabilities for types, enabling runtime type introspection.

**Important**: This crate should not be used directly. Use the `reflect_tools` crate which re-exports this functionality with supporting types.

### Scope

#### Responsibility

reflect_tools_meta is responsible for providing the procedural macro implementation that generates reflection code for types annotated with `#[derive(Reflect)]`.

#### In-Scope

- **`#[derive(Reflect)]` macro**: Generate reflection implementations
- **Type analysis**: Parse struct/enum definitions
- **Metadata generation**: Generate type, field, and variant information
- **Code generation**: Generate Entity trait implementations

#### Out-of-Scope

- **Runtime types**: Provided by `reflect_tools`
- **User-facing API**: Use `reflect_tools` crate instead
- **Deep reflection**: Limited to structural metadata

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `reflect_tools` crate
- **Compile-time only**: Macro expansion only

## Architecture

### Module Structure

```
reflect_tools_meta/
├── src/
│   └── lib.rs              # Proc-macro entry point
├── Cargo.toml
├── readme.md
└── spec.md
```

### Generated Code

```rust
// Input
#[derive(Reflect)]
struct Person
{
  name: String,
  age: u32,
}

// Generates (conceptual):
impl Entity for Person
{
  fn type_name() -> &'static str { "Person" }
  fn fields() -> Vec<FieldDescriptor> { ... }
}
```

## Public API

### Derive Macro

#### `#[derive(Reflect)]`

Generate reflection implementation for types.

```rust
use reflect_tools::Reflect;

#[derive(Reflect)]
struct Config
{
  host: String,
  port: u16,
}

// Access type information at runtime
let type_name = Config::type_name();
let fields = Config::fields();
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `reflect_derive` | ✓ | Enable Reflect derive |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |

### Consumers

- `reflect_tools` - Re-exports this crate's macros

## Design Rationale

### Why Reflection?

Rust lacks runtime reflection. This crate provides:
1. Type metadata at runtime
2. Field enumeration
3. Structural introspection

Useful for serialization, debugging, and generic programming.

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `reflect_tools` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
| `bevy_reflect` | Alternative reflection system |
