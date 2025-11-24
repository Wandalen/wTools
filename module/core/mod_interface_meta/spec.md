# mod_interface_meta

Procedural macro implementation for module interface organization.

## Overview

`mod_interface_meta` is the proc-macro companion crate for `mod_interface`. It provides the `mod_interface!` macro that generates standardized module structures with namespace layers (own, orphan, exposed, prelude).

**Important**: This crate should not be used directly. Use the `mod_interface` crate which re-exports this functionality with documentation and utilities.

### Scope

#### Responsibility

mod_interface_meta is responsible for providing the procedural macro implementation that generates module organization code from the `mod_interface!` DSL.

#### In-Scope

- **`mod_interface!` macro**: Parse and expand module interface definitions
- **Namespace generation**: Create own/orphan/exposed/prelude modules
- **Layer handling**: Process `layer` directives for submodules
- **Re-export generation**: Generate `pub use` statements

#### Out-of-Scope

- **Runtime behavior**: Pure compile-time code generation
- **User-facing API**: Use `mod_interface` crate instead
- **Module content**: Only organizes structure, doesn't generate logic

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `mod_interface` crate
- **Compile-time only**: No runtime dependencies

## Architecture

### Module Structure

```
mod_interface_meta/
├── src/
│   └── lib.rs              # Proc-macro entry point
├── Cargo.toml
├── readme.md
└── spec.md
```

### Generated Structure

```rust
// Input
mod_interface!
{
  own use my_type;
  layer submodule;
}

// Generates (conceptual):
mod own { pub use super::my_type; }
mod orphan { pub use super::own::*; }
mod exposed { pub use super::orphan::*; }
mod prelude { }
pub use exposed::*;
// Plus submodule integration
```

## Public API

### Macro

#### `mod_interface!`

Define module interface with namespace layers.

```rust
use mod_interface::mod_interface;

mod private
{
  pub struct MyType;
  pub fn my_function() {}
}

mod_interface!
{
  // Export to own namespace
  own use private::MyType;

  // Export to exposed namespace (broader visibility)
  exposed use private::my_function;

  // Include submodule as layer
  layer submodule;
}
```

### Namespace Directives

- `own use`: Export to own namespace (most restrictive)
- `orphan use`: Export to orphan namespace
- `exposed use`: Export to exposed namespace
- `prelude use`: Export to prelude namespace (broadest)
- `layer`: Include submodule with its own namespace hierarchy

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
| `derive_tools` | Derive macro support |

### Consumers

- `mod_interface` - Re-exports this crate's macros

## Design Rationale

### Why Namespace Layers?

The four-layer namespace system provides:
1. **own**: Internal-only exports
2. **orphan**: Semi-public exports
3. **exposed**: Default public exports
4. **prelude**: Convenience re-exports

This enables fine-grained control over API visibility.

### Why Layer Directive?

The `layer` directive allows hierarchical module composition where submodules have their own namespace structure that integrates with the parent.

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `mod_interface` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
