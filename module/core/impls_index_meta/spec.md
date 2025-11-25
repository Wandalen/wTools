# impls_index_meta

Procedural macro implementation for function indexing in impl blocks.

## Overview

`impls_index_meta` is the proc-macro companion crate for `impls_index`. It provides macros that wrap each function in an impl block under a named macro, enabling function discovery and indexing at compile time.

**Important**: This crate should not be used directly. Use the `impls_index` crate which re-exports this functionality.

### Scope

#### Responsibility

impls_index_meta is responsible for providing procedural macros that generate wrapper macros for functions in impl blocks, enabling compile-time function indexing.

#### In-Scope

- **`impls!` macro**: Wrap impl block functions under named macros
- **`impls1!`, `impls2!`, `impls3!` macros**: Variants for different contexts
- **Function analysis**: Parse function signatures
- **Macro generation**: Generate wrapper macros for each function

#### Out-of-Scope

- **Runtime behavior**: Pure compile-time code generation
- **User-facing API**: Use `impls_index` crate instead
- **Function body modification**: Only wraps, doesn't transform

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `impls_index` crate
- **Compile-time only**: No runtime dependencies

## Architecture

### Module Structure

```
impls_index_meta/
├── src/
│   └── lib.rs              # Proc-macro entry point
├── Cargo.toml
├── readme.md
└── spec.md
```

### Macro Expansion

```rust
// Input
impls!
{
  fn add( a: i32, b: i32 ) -> i32 { a + b }
  fn sub( a: i32, b: i32 ) -> i32 { a - b }
}

// Generates (conceptual):
macro_rules! add { ... }
macro_rules! sub { ... }
fn add( a: i32, b: i32 ) -> i32 { a + b }
fn sub( a: i32, b: i32 ) -> i32 { a - b }
```

## Public API

### Macros

#### `impls!`

Index functions in impl block.

```rust
use impls_index::impls;

impls!
{
  fn greet( name: &str ) -> String
  {
    format!( "Hello, {}!", name )
  }
}

// Function is available both directly and via generated macro
let msg = greet( "World" );
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |

### Consumers

- `impls_index` - Re-exports this crate's macros

## Design Rationale

### Why Function Indexing?

Function indexing enables:
1. Compile-time function discovery
2. Macro-based function invocation
3. Metaprogramming over function sets

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `impls_index` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
