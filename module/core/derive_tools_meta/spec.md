# derive_tools_meta

Procedural macro implementations for common derive macros extending std.

## Overview

`derive_tools_meta` is the proc-macro companion crate for `derive_tools`. It provides derive macro implementations for common traits like `Deref`, `DerefMut`, `From`, `Index`, `AsRef`, `AsMut`, `New`, and more.

**Important**: This crate should not be used directly. Use the `derive_tools` crate which re-exports this functionality with additional convenience features.

### Scope

#### Responsibility

derive_tools_meta is responsible for providing procedural macro implementations that generate trait implementations for structs, reducing boilerplate code.

#### In-Scope

- **Derive macros**: `Deref`, `DerefMut`, `From`, `InnerFrom`, `New`, `Index`, `IndexMut`, `AsRef`, `AsMut`, `VariadicFrom`, `Not`, `Phantom`
- **Struct analysis**: Parse struct definitions and field types
- **Code generation**: Generate trait implementations
- **Attribute support**: Handle derive attributes and configuration

#### Out-of-Scope

- **Runtime behavior**: Pure compile-time code generation
- **User-facing API**: Use `derive_tools` crate instead
- **Enum support**: Most derives are struct-focused

#### Boundaries

- **Upstream**: Uses `macro_tools` for syntax parsing
- **Downstream**: Re-exported by `derive_tools` crate
- **Compile-time only**: No runtime dependencies

## Architecture

### Module Structure

```
derive_tools_meta/
├── src/
│   └── lib.rs              # Proc-macro entry points
├── Cargo.toml
├── readme.md
└── spec.md
```

## Public API

### Derive Macros

#### `#[derive(Deref)]`

Auto-implement `Deref` for newtype structs.

```rust
#[derive(Deref)]
struct Wrapper( String );

let w = Wrapper( "hello".into() );
assert_eq!( w.len(), 5 ); // Derefs to String
```

#### `#[derive(DerefMut)]`

Auto-implement `DerefMut` for newtype structs.

#### `#[derive(From)]`

Auto-implement `From<Inner>` for newtype structs.

```rust
#[derive(From)]
struct UserId( u64 );

let id: UserId = 42u64.into();
```

#### `#[derive(InnerFrom)]`

Auto-implement `From<Wrapper>` returning inner type.

#### `#[derive(New)]`

Generate `new()` constructor.

```rust
#[derive(New)]
struct Point { x: f32, y: f32 }

let p = Point::new( 1.0, 2.0 );
```

#### `#[derive(Index)]` / `#[derive(IndexMut)]`

Auto-implement indexing traits.

#### `#[derive(AsRef)]` / `#[derive(AsMut)]`

Auto-implement reference conversion traits.

#### `#[derive(VariadicFrom)]`

Implement variadic `From1`, `From2`, `From3` traits.

#### `#[derive(Not)]`

Auto-implement logical `Not` trait.

#### `#[derive(Phantom)]`

Generate phantom data handling.

## Feature Flags

| Feature | Default | Description |
|---------|---------|-------------|
| `enabled` | ✓ | Enable the crate |
| `full` | - | All features |
| `derive_deref` | ✓ | Enable Deref derive |
| `derive_deref_mut` | ✓ | Enable DerefMut derive |
| `derive_from` | ✓ | Enable From derive |
| `derive_new` | ✓ | Enable New derive |
| `derive_index` | ✓ | Enable Index derive |
| `derive_index_mut` | ✓ | Enable IndexMut derive |
| `derive_inner_from` | ✓ | Enable InnerFrom derive |
| `derive_as_ref` | ✓ | Enable AsRef derive |
| `derive_as_mut` | ✓ | Enable AsMut derive |
| `derive_variadic_from` | ✓ | Enable VariadicFrom derive |
| `derive_not` | ✓ | Enable Not derive |
| `derive_phantom` | ✓ | Enable Phantom derive |

## Dependencies and Consumers

### Dependencies

| Dependency | Purpose |
|------------|---------|
| `macro_tools` | Syntax parsing utilities |
| `iter_tools` | Iterator extensions |
| `component_model_types` | Component types |

### Consumers

- `derive_tools` - Re-exports this crate's macros

## Design Rationale

### Why Separate Crate?

Rust requires proc-macro crates to be separate. The `derive_tools` facade provides the user-facing API while this crate handles implementations.

### Why Feature-Gated Derives?

Not all projects need all derives. Feature flags allow:
- Faster compilation by excluding unused code
- Reduced dependency footprint
- Selective functionality

## Related Crates

| Crate | Relationship |
|-------|--------------|
| `derive_tools` | Parent facade crate |
| `macro_tools` | Upstream syntax utilities |
| `derive_more` | Alternative derive collection |
