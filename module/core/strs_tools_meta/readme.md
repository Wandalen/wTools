# strs_tools_meta

Procedural macros for compile-time string optimizations.

## Overview

`strs_tools_meta` provides procedural macros that enable compile-time optimizations for string operations. This is the proc-macro companion crate for `strs_tools`.

**Important**: This crate should not be used directly. Use the `strs_tools` crate which re-exports this functionality.

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
strs_tools = { version = "*" }
```

## Features

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `optimize_split` | ✓ | Enable split optimization |
| `optimize_match` | ✓ | Enable match optimization |

## Usage

### Optimized String Splitting

```rust
use strs_tools::optimize_split;

// Pattern compiled at build time
let parts = optimize_split!( "hello,world", "," );
```

### Optimized String Matching

```rust
use strs_tools::optimize_match;

// Match patterns compiled at build time
let result = optimize_match!( input, "pattern1" | "pattern2" );
```

## Documentation

- [Specification](spec.md) - Complete technical specification
- [API Documentation](https://docs.rs/strs_tools_meta) - Full API reference

## Testing

```bash
# Run all tests
cargo nextest run --all-features

# With workspace context
w3 .test l::3
```

## License

Licensed under MIT license. See [license](license) file for details.
