# clone_dyn_meta

Procedural macro implementation for cloning trait objects.

## Overview

`clone_dyn_meta` is the proc-macro companion crate for `clone_dyn`. It provides the `#[clone_dyn]` attribute macro that generates the necessary trait implementations to enable cloning of `Box<dyn Trait>` trait objects.

**Important**: This crate should not be used directly. Use the `clone_dyn` crate which re-exports this functionality with a more ergonomic API.

### Scope

#### Responsibility

clone_dyn_meta is responsible for providing the procedural macro implementation that generates clone-enabling code for trait definitions annotated with `#[clone_dyn]`.

#### In-Scope

- **`#[clone_dyn]` attribute macro**: Generate clone implementations for trait objects
- **Trait analysis**: Parse trait definitions to identify methods
- **Code generation**: Generate `CloneDyn` trait bounds and implementations
- **Generic support**: Handle traits with generic parameters

#### Out-of-Scope

- **Runtime behavior**: Pure compile-time code generation
- **User-facing API**: Use `clone_dyn` crate instead
- **Non-trait types**: Only works with trait definitions

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `clone_dyn` crate
- **Compile-time only**: No runtime dependencies

## Architecture

### Module Structure

```
clone_dyn_meta/
├── src/
│   └── lib.rs              # Proc-macro entry point
├── Cargo.toml
├── readme.md
└── spec.md
```

### Macro Expansion

```rust
// Input
#[clone_dyn]
trait MyTrait
{
  fn do_something( &self );
}

// Expanded (conceptual)
trait MyTrait: CloneDyn
{
  fn do_something( &self );
}

impl Clone for Box< dyn MyTrait >
{
  fn clone( &self ) -> Self { self.clone_dyn() }
}
```

## Public API

### Attribute Macro

#### `#[clone_dyn]`

Apply to trait definitions to enable cloning of trait objects.

```rust
use clone_dyn::clone_dyn;

#[clone_dyn]
pub trait Drawable
{
  fn draw( &self );
}

// Now Box<dyn Drawable> is Clone
let original: Box< dyn Drawable > = Box::new( Circle );
let cloned = original.clone();
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
| `macro_tools` | Syntax parsing (syn, quote, proc-macro2) |
| `component_model_types` | Component assignment types |

### Consumers

- `clone_dyn` - Re-exports this crate's macros

## Design Rationale

### Why Separate Crate?

Rust requires proc-macro crates to be separate due to compilation model constraints. The `clone_dyn` facade crate provides a user-friendly interface while this crate handles the macro implementation.

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `clone_dyn` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
| `dyn-clone` | Alternative approach |
