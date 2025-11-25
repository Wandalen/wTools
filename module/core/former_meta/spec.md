# former_meta

Procedural macro implementation for the Former builder pattern.

## Overview

`former_meta` is the proc-macro companion crate for `former`. It provides the `#[derive(Former)]` macro that generates fluent builder APIs for structs, supporting nested builders, collection subformers, and customizable forming behavior.

**Important**: This crate should not be used directly. Use the `former` crate which re-exports this functionality with supporting types and traits.

### Scope

#### Responsibility

former_meta is responsible for providing the procedural macro implementation that generates builder pattern code for structs annotated with `#[derive(Former)]`.

#### In-Scope

- **`#[derive(Former)]` macro**: Generate builder pattern implementation
- **Field analysis**: Parse struct fields and their types
- **Subformer generation**: Create nested builders for collections
- **Attribute handling**: Process `#[former(...)]` configuration attributes
- **Generic support**: Handle structs with generic parameters

#### Out-of-Scope

- **Runtime types**: Provided by `former_types`
- **User-facing API**: Use `former` crate instead
- **Non-struct types**: Enum support limited

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing, `former_types` for runtime types
- **Downstream**: Re-exported by `former` crate
- **Compile-time only**: Macro expansion only

## Architecture

### Module Structure

```
former_meta/
├── src/
│   └── lib.rs              # Proc-macro entry point
├── Cargo.toml
├── readme.md
└── spec.md
```

### Generated Code Structure

```rust
// Input
#[derive(Former)]
struct Config
{
  name: String,
  values: Vec< i32 >,
}

// Generates (conceptual):
// - ConfigFormer<...> builder struct
// - ConfigFormerStorage storage type
// - Field setter methods (name(), values())
// - Collection subformer (values_subformer())
// - form()/end() completion methods
```

## Public API

### Derive Macro

#### `#[derive(Former)]`

Generate builder pattern for structs.

```rust
use former::Former;

#[derive(Former)]
pub struct UserProfile
{
  #[former(default = "Anonymous".to_string())]
  name: String,
  age: Option< u32 >,
  tags: Vec< String >,
}

let profile = UserProfile::former()
  .name( "Alice" )
  .age( 30 )
  .tags_subformer()
    .push( "admin" )
    .push( "active" )
    .end()
  .form();
```

### Field Attributes

#### `#[former(default = expr)]`

Set default value for field.

#### `#[former(setter = name)]`

Rename the setter method.

#### `#[former(subformer)]`

Enable subformer for collection fields.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `derive_former` | ✓ | Enable Former derive |
| `performance` | - | Benchmarking features |
| `proc-macro-debug` | - | Debug macro expansion |
| `former_diagnostics_print_generated` | - | Print generated code |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |
| `former_types` | Runtime type definitions |
| `iter_tools` | Iterator extensions |
| `component_model_types` | Component types |
| `convert_case` | Identifier case conversion |

### Consumers

- `former` - Re-exports this crate's macros

## Design Rationale

### Why Separate Crate?

Rust requires proc-macro crates to be separate. The `former` facade combines:
- This crate's derive macros
- `former_types` runtime types
- Additional utilities

### Why Complex Macro?

The Former pattern is inherently complex:
- Generates multiple types (Former, Storage)
- Handles nested builders
- Supports collections with subformers
- Manages type state for compile-time safety

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `former` | Parent facade crate |
| `former_types` | Runtime type definitions |
| `macro_tools` | Upstream syntax utilities |
| `typed-builder` | Alternative builder macro |
