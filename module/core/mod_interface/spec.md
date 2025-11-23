# Specification: mod_interface

## Overview

**mod_interface** provides the `mod_interface!` procedural macro to define structured module interfaces with controlled visibility and propagation through exposure levels. It introduces a layered modularity system where items are organized into five predefined namespaces (`private`, `own`, `orphan`, `exposed`, `prelude`), each with specific visibility and propagation rules.

**Version:** 0.53.0
**Status:** Experimental
**Category:** Module Organization Pattern
**Dependents:** 10 crates (former, component_model, reflect_tools, implements, strs_tools, unilang, error_tools, meta_tools, diagnostics_tools, derive_tools)

### Scope

#### Responsibility

Provide a procedural macro system for organizing Rust modules into layered architectures with controlled item visibility and propagation, establishing a standardized modularity protocol for the wTools ecosystem.

#### In-Scope

1. **mod_interface! Macro**
   - Procedural macro for defining module layers
   - Automatic generation of exposure level namespaces
   - Controlled item propagation between layers
   - Syntax directives: `layer`, `use`, `reuse`, `<level> use`, `<level> mod`

2. **Exposure Levels (Five Layers)**
   - `private`: Internal implementation (no propagation)
   - `own`: Layer-specific public items (no propagation)
   - `orphan`: Items for immediate parent only
   - `exposed`: Items for all ancestors
   - `prelude`: Core interface essentials (glob use)

3. **Propagation Rules**
   - `private` → no propagation (internal only)
   - `own` → no propagation (layer only)
   - `orphan` → immediate parent's root and `own` only
   - `exposed` → all ancestor levels except `prelude`
   - `prelude` → all ancestor levels including `prelude`

4. **Layer Directives**
   - `layer <name>`: Define and include `<name>.rs` as child layer
   - `use <path>`: Integrate existing module as layer
   - `reuse <path>`: Similar to `use`, for reusing common interfaces
   - `<level> use <item>`: Re-export item into specific exposure level
   - `<level> mod <name>`: Define micro module in exposure level

5. **Debugging Support**
   - `#![debug]` directive in `mod_interface!` macro
   - Generates expanded macro code output for inspection
   - Helpful for understanding propagation behavior

6. **Convention-Based Architecture**
   - Standardized namespace structure across wTools crates
   - Clear boundaries and relationships between modules
   - Explicit control over API surface area

7. **No-std Compatibility**
   - Crate is `#![no_std]` compatible
   - Macro generation works in embedded environments

8. **Runtime Re-export Crate**
   - Re-exports `mod_interface_meta` procedural macro
   - Provides convenient import point for users
   - Absorption pattern (runtime absorbs meta)

#### Out-of-Scope

1. **NOT Runtime Functionality**
   - No runtime behavior beyond macro expansion
   - No dynamic module loading or manipulation
   - **Rationale:** Pure compile-time code generation system

2. **NOT Module Content Validation**
   - Does not validate module structure or contents
   - Does not enforce naming conventions
   - Does not check for missing exposure levels
   - **Rationale:** Trust developers to organize code correctly

3. **NOT Documentation Generation**
   - Does not generate module documentation
   - Does not extract or process doc comments
   - **Rationale:** Documentation is separate concern

4. **NOT Alternative Module Patterns**
   - Does not support custom exposure levels
   - Does not allow renaming standard levels
   - Does not provide opt-out of specific levels
   - **Rationale:** Standardization requires consistency

5. **NOT Visibility Enforcement**
   - Does not prevent direct access to private modules
   - Does not enforce use of public API
   - Relies on Rust's standard visibility rules
   - **Rationale:** Rust's visibility system handles enforcement

6. **NOT Dependency Management**
   - Does not manage module dependencies
   - Does not resolve circular dependencies
   - Does not provide dependency injection
   - **Rationale:** Dependencies are resolved by Rust compiler

7. **NOT Performance Optimization**
   - Does not optimize generated code
   - Does not inline or transform for performance
   - **Rationale:** Optimization is compiler's responsibility

8. **NOT Cross-Crate Module Organization**
   - Operates within single crate only
   - Does not manage inter-crate module relationships
   - **Rationale:** Cross-crate organization is handled by Cargo and visibility rules

#### Boundaries

- **mod_interface vs traditional modules**: mod_interface adds structured exposure levels on top of traditional modules
- **mod_interface vs mod_interface_meta**: mod_interface is runtime re-export crate, mod_interface_meta is procedural macro implementation
- **mod_interface vs pub/pub(crate)**: mod_interface provides finer-grained propagation control beyond Rust's standard visibility
- **Layer vs namespace**: All layers are namespaces, but not all namespaces are layers (layers use mod_interface! pattern)

## Architecture

### Dependency Structure

```
mod_interface (runtime crate, no_std)
└── Internal Dependencies
    └── mod_interface_meta (procedural macro, workspace)
```

### Absorption Pattern

mod_interface follows the absorption pattern:
- **Runtime crate** (`mod_interface`): Re-exports the macro, provides user-facing API
- **Meta crate** (`mod_interface_meta`): Implements procedural macro logic

This prevents circular dependencies and separates macro implementation from usage.

### Module Organization

mod_interface itself uses the traditional module organization pattern (not mod_interface! pattern):

```
mod_interface
├── lib.rs (re-exports mod_interface_meta)
├── dependency module (mod_interface_meta)
└── Standard namespaces: own, orphan, exposed, prelude
```

**Note:** mod_interface does not use its own macro to avoid bootstrap complexity.

### Exposure Level Architecture

When `mod_interface!` is applied, it generates five exposure level namespaces:

```
YourModule (uses mod_interface!)
├── private { /* all implementations */ }
├── own { pub use orphan::*; pub use <items from private>; }
├── orphan { pub use exposed::*; pub use <items from private>; }
├── exposed { pub use prelude::*; pub use <items from private>; }
└── prelude { pub use <items from private>; }
```

Root module includes: `pub use own::*;` (everything from own is available at module root)

### Propagation Flow

When a child layer is used in a parent layer via `use <child>`:

```
Child Layer                     Parent Layer
───────────                     ────────────
child::prelude::*        →      parent::prelude::*
child::exposed::*        →      parent::exposed::*
child::orphan::*         →      parent::own::* (not parent::orphan!)
child (module itself)    →      parent::own::*
```

**Key Insight:** Orphan items propagate to parent's `own` (and root via `pub use own::*`), but NOT to parent's `orphan`. This prevents uncontrolled propagation up the hierarchy.

## Public API

### Macro

```rust
/// Define module interface with controlled visibility and propagation
#[cfg(feature = "enabled")]
pub use mod_interface_meta::mod_interface;
```

### Usage Syntax

```rust
mod_interface!
{
  // Optional: Enable debug output
  #![debug]

  // Define child layer from file
  layer child_module;

  // Use existing module as layer
  use super::existing_module;

  // Reuse common interface
  reuse super::common_interface;

  // Re-export items into specific levels
  own use my_private_thing;
  orphan use parent_should_see_this;
  exposed use everyone_should_see_this;
  prelude use essential_item;

  // Define micro modules
  orphan mod helper_utilities;
  exposed mod public_utilities;
}
```

### Required Module Structure

For a module to be used as a layer, it must contain:

```rust
// 1. Private namespace with implementations
mod private
{
  pub fn my_function() {}
  pub struct MyType;
}

// 2. mod_interface! macro defining exposure levels
mod_interface!
{
  own use my_function;
  prelude use MyType;
}
```

This generates the five exposure levels automatically.

## Usage Patterns

### Pattern 1: Basic Layer with Items

```rust
pub mod my_module
{
  mod private
  {
    pub fn internal_only() -> bool { true }
    pub fn module_only() -> bool { true }
    pub fn parent_only() -> bool { true }
    pub fn all_ancestors() -> bool { true }
    pub fn core_api() -> bool { true }
  }

  mod_interface!
  {
    // Does not propagate
    own use module_only;

    // Propagates to immediate parent's root and own
    orphan use parent_only;

    // Propagates to all ancestors (except their prelude)
    exposed use all_ancestors;

    // Propagates everywhere, intended for glob use
    prelude use core_api;
  }
}
```

### Pattern 2: Parent Using Child Layer

```rust
mod private {}

mod_interface!
{
  // Include child layer
  use super::my_module;
}

// Now accessible:
// - my_module::core_api() via prelude propagation
// - my_module::all_ancestors() via exposed propagation
// - my_module::parent_only() via orphan propagation (in root and own)
// - my_module::module_only() only in child, not propagated
```

### Pattern 3: Hierarchical Layers (Grandparent-Parent-Child)

```rust
// Child layer
pub mod child
{
  mod private { pub fn child_fn() -> i32 { 1 } }
  mod_interface! { prelude use child_fn; }
}

// Parent layer
pub mod parent
{
  mod private {}
  mod_interface! { use super::child; }
}

// Grandparent layer
mod private {}
mod_interface! { use super::parent; }

// child_fn() is accessible in grandparent via prelude propagation:
// child::prelude::child_fn() → parent::prelude::child_fn() → grandparent::prelude::child_fn()
```

### Pattern 4: Micro Modules

```rust
mod private
{
  pub fn main_function() {}
}

mod_interface!
{
  // Define utilities.rs directly in orphan namespace
  orphan mod utilities;

  // Re-export from private
  exposed use main_function;
}

// Generates: orphan::utilities module from utilities.rs
```

### Pattern 5: Debugging Macro Expansion

```rust
mod_interface!
{
  #![debug] // Prints generated code to console during compilation

  layer my_layer;
}
```

## Dependencies and Consumers

### Direct Dependencies

**Internal:**
- `mod_interface_meta` (workspace) - Procedural macro implementation

### Consumers (10 crates)

1. **former** - Builder pattern with structured module interfaces
2. **component_model** - Component system with layered architecture
3. **reflect_tools** - Reflection utilities with controlled exposure
4. **implements** - Implementation detection with module organization
5. **strs_tools** - String tools with layered API
6. **unilang** - Language utilities with structured modules
7. **error_tools** - Error handling with exposure levels
8. **meta_tools** - Meta-programming utilities
9. **diagnostics_tools** - Diagnostics with controlled visibility
10. **derive_tools** - Derive macro framework with layers

## Design Rationale

### Why Five Exposure Levels?

The five-level system addresses common modularity patterns:

1. **private**: Implementation details (every module needs internal code)
2. **own**: Module-specific API (some items should not propagate)
3. **orphan**: Parent-only items (avoid deep propagation for tightly-coupled items)
4. **exposed**: Public API for hierarchy (general-purpose public items)
5. **prelude**: Essential imports (glob-use-friendly core API)

**Tradeoff:** More levels = more complexity, but provides precise control over propagation.

### Why Not Use Standard pub/pub(crate)?

Standard Rust visibility has limitations:

- **pub**: All or nothing (can't limit to immediate parent)
- **pub(crate)**: Crate-wide visibility (can't limit to module hierarchy)
- **pub(in path)**: Verbose and doesn't support propagation rules

mod_interface provides:
- **Hierarchical propagation**: Items propagate up the hierarchy based on level
- **Convention over configuration**: Standardized structure across all modules
- **Clear API surface**: Prelude clearly indicates core API

### Why Absorption Pattern (mod_interface + mod_interface_meta)?

1. **Circular Dependency Prevention**: Macro implementation can't use the macro itself
2. **Clean User Experience**: Users import `mod_interface`, not `mod_interface_meta`
3. **Separation of Concerns**: Runtime re-export vs procedural macro logic
4. **Version Management**: Single version for both runtime and meta crates

### Why Not Customizable Exposure Levels?

Fixed levels ensure:
1. **Consistency**: All wTools crates use same structure
2. **Predictability**: Developers know what to expect
3. **Tooling**: Tools can assume standard structure
4. **Learning Curve**: Once learned, applies everywhere

**Tradeoff:** Less flexibility for specific use cases, but greater consistency across ecosystem.

### Why No Module Validation?

1. **Trust**: Developers are trusted to organize code correctly
2. **Flexibility**: Validation would restrict valid use cases
3. **Compile-Time Errors**: Rust's compiler already catches errors
4. **Simplicity**: Validation adds complexity without clear benefit

## Testing Strategy

### Test Coverage

- **Example Programs**: 2 comprehensive examples
  - `mod_interface_trivial` - Basic usage demonstration
  - `mod_interface_debug` - Debug directive demonstration
- **Doc Tests**: Embedded in readme.md demonstrating propagation rules
- **Integration Tests**: Uses `test_tools` for testing

### Test Focus

1. **Propagation Rules**: Verify each exposure level propagates correctly
2. **Layer Composition**: Test parent-child layer relationships
3. **Macro Expansion**: Verify generated code structure
4. **Edge Cases**: Empty layers, multiple children, deep hierarchies

## Future Considerations

### Potential Enhancements

1. **IDE Support**: Editor plugins for navigating exposure levels
2. **Visualization Tools**: Graphical representation of layer hierarchy
3. **Linting**: Detect anti-patterns (e.g., exposing too much in prelude)
4. **Migration Tool**: Convert traditional modules to mod_interface pattern
5. **Spec Integration**: Auto-generate specification sections from layers

### Breaking Changes to Consider

1. **Customizable Levels**: Allow custom exposure levels beyond the five standard ones
2. **Opt-Out**: Allow disabling specific levels for simpler modules
3. **Aliasing**: Rename exposure levels for specific domains
4. **Cross-Crate Layers**: Support layer composition across crate boundaries

### Known Limitations

1. **Bootstrap Complexity**: mod_interface itself can't use mod_interface! pattern
2. **Learning Curve**: Developers must understand propagation rules
3. **Verbosity**: Five levels can be overkill for simple modules
4. **Tool Support**: Limited IDE/tooling awareness of layer structure

## Adoption Guidelines

### When to Use mod_interface

**Good Candidates:**
- Crates with complex module hierarchies
- Projects requiring precise API control
- Libraries with public, semi-public, and internal APIs
- Codebases needing consistent module structure

**Poor Candidates:**
- Simple single-module crates
- Procedural macro crates (can't use own macro)
- Proof-of-concept or prototype code
- Projects with unconventional module structures

### Migration Strategy

1. **Start with prelude**: Move core API items to prelude level
2. **Add exposed**: Public API items that aren't in prelude
3. **Identify orphan**: Parent-only items (avoid deep propagation)
4. **Keep in own**: Module-specific items that shouldn't propagate
5. **Hide in private**: Implementation details

### Best Practices

1. **Minimal prelude**: Only truly essential items in prelude
2. **Consistent patterns**: Use same exposure levels for similar items across modules
3. **Document propagation**: Comment why items are in specific levels
4. **Avoid orphan overuse**: Most items should be exposed or prelude, not orphan
5. **Test visibility**: Verify propagation behavior with tests

## Related Crates

- **mod_interface_meta**: Procedural macro implementation (meta crate)
- **former**: Major consumer using mod_interface for builder pattern
- **component_model**: Uses mod_interface for component system architecture
- **All wTools meta crates**: Use traditional pattern, not mod_interface (can't use own macro)

## References

- [API Documentation](https://docs.rs/mod_interface)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/mod_interface)
- [Examples](https://github.com/Wandalen/wTools/tree/master/examples)
  - [mod_interface_trivial](https://github.com/Wandalen/wTools/tree/master/examples/mod_interface_trivial)
  - [mod_interface_debug](https://github.com/Wandalen/wTools/tree/master/examples/mod_interface_debug)
- [readme.md](./readme.md)
