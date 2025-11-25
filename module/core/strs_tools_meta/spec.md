# strs_tools_meta

Procedural macros for compile-time string optimizations.

## Overview

`strs_tools_meta` is the proc-macro companion crate for `strs_tools`. It provides procedural macros that enable compile-time optimizations for string operations, particularly for split and match operations.

**Important**: This crate should not be used directly. Use the `strs_tools` crate which re-exports this functionality.

### Scope

#### Responsibility

strs_tools_meta is responsible for providing procedural macros that generate optimized string handling code at compile time.

#### In-Scope

- **`optimize_split!` macro**: Compile-time optimized string splitting
- **`optimize_match!` macro**: Compile-time optimized string matching
- **Pattern analysis**: Parse string patterns at compile time
- **Code generation**: Generate optimized implementations

#### Out-of-Scope

- **Runtime utilities**: Provided by `strs_tools`
- **User-facing API**: Use `strs_tools` crate instead
- **General string utilities**: Only optimization macros

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `strs_tools` crate
- **Compile-time only**: Pattern compilation at build time

## Architecture

### Module Structure

```
strs_tools_meta/
├── src/
│   └── lib.rs              # Proc-macro entry points
├── Cargo.toml
└── spec.md
```

## Public API

### Macros

#### `optimize_split!`

Compile-time optimized string splitting.

```rust
use strs_tools::optimize_split;

// Pattern compiled at build time
let parts = optimize_split!( "hello,world", "," );
```

#### `optimize_match!`

Compile-time optimized string matching.

```rust
use strs_tools::optimize_match;

// Match patterns compiled at build time
let result = optimize_match!( input, "pattern1" | "pattern2" );
```

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `optimize_split` | ✓ | Enable split optimization |
| `optimize_match` | ✓ | Enable match optimization |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |

### Consumers

- `strs_tools` - Re-exports this crate's macros

## Design Rationale

### Why Compile-Time Optimization?

String operations benefit from compile-time processing:
1. Pattern validation at build time
2. Pre-computed lookup tables
3. Reduced runtime overhead

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `strs_tools` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
