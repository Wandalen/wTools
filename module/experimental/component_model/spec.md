# Specification: component_model

## Overview

**component_model** is the user-facing runtime crate for the component-based programming model, providing derive macros for type-safe component assignment. It aggregates component_model_meta (procedural macros) and component_model_types (trait definitions) into a unified API for building fluent APIs, configuration builders, and composable object systems.

**Version:** 0.12.0
**Status:** Experimental
**Category:** Design Patterns (Absorption Pattern)
**Dependents:** 0 workspace crates (external user-facing API)

### Scope

#### Responsibility

Provide the unified user-facing API for component-based programming by aggregating procedural macros (component_model_meta) and trait definitions (component_model_types), enabling zero-boilerplate, type-safe component assignment for configuration builders and fluent APIs.

#### In-Scope

1. **Derive Macro Re-exports**
   - `ComponentModel` - Unified derive macro combining all functionality
   - `Assign` - Type-driven component assignment derive
   - `ComponentsAssign` - Multiple component assignment
   - `ComponentFrom` - Component creation from single value
   - `FromComponents` - Component creation from multiple values
   - All re-exported from `component_model_meta`

2. **Trait System Re-exports**
   - `Assign<T, IntoT>` trait - Generic component assignment
   - `OptionExt<T>` trait - Option-aware assignment
   - `AssignWithType` trait - Explicit type assignment
   - `PopularType` marker - Standard library type support
   - All re-exported from `component_model_types`

3. **Absorption Pattern Implementation**
   - Runtime crate absorbs both meta and types crates
   - Prevents circular dependencies in ecosystem
   - Unified feature gating across all three crates
   - Single import point for users

4. **Popular Types Support**
   - Intelligent conversion for `Duration`, `PathBuf`, `SocketAddr`, etc.
   - Built-in implementations for common std library types
   - Exposed through `popular_types` module

5. **Feature Architecture**
   - `enabled`: Master feature switch
   - `full`: All features (default)
   - `derive_component_model`: Unified derive macro
   - `derive_component_assign`: Basic Assign derive
   - `derive_components_assign`: Multiple components
   - `derive_component_from`: Single value construction
   - `derive_from_components`: Multiple value construction
   - `types_component_assign`: Trait system

6. **No-std Support**
   - `no_std` feature flag
   - `use_alloc` feature for allocation-dependent functionality
   - Propagates to component_model_types and collection_tools

7. **Traditional Module Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Not using mod_interface! (absorption crate)

8. **Dependency Module**
   - Explicit `dependency` module re-exporting component_model_types and component_model_meta
   - Allows users to access underlying crates if needed

#### Out-of-Scope

1. **NOT Actual Implementation Code**
   - Does not contain procedural macro implementations (that's component_model_meta)
   - Does not define traits (that's component_model_types)
   - **Rationale:** Pure re-export/aggregation crate following absorption pattern

2. **NOT Global Component Registry**
   - Does not provide global component registration
   - Does not implement dependency injection container
   - **Rationale:** Component model focuses on type-driven assignment, not DI

3. **NOT Runtime Component Loading**
   - Does not load components at runtime
   - Does not provide plugin system
   - **Rationale:** Compile-time only derive macros

4. **NOT Reflection System**
   - Does not discover components via reflection
   - Does not provide runtime type inspection
   - **Rationale:** Type-driven assignment is compile-time only

5. **NOT Builder Pattern Implementation**
   - Does not implement builder pattern logic (that's former crate)
   - Does not generate builder types
   - **Rationale:** Component model provides assignment traits; former provides builder scaffolding

6. **NOT Validation Framework**
   - Does not validate component assignments
   - Does not enforce constraints
   - **Rationale:** Validation is application-specific

7. **NOT Object Composition Framework**
   - Does not provide composition utilities beyond assignment
   - Does not implement component lifecycle
   - **Rationale:** Focused on assignment, not lifecycle management

8. **NOT Configuration Management**
   - Does not load/save configuration files
   - Does not provide config parsing
   - **Rationale:** Component model enables building config objects, not managing them

#### Boundaries

- **component_model vs component_model_types**: component_model re-exports types, types defines them
- **component_model vs component_model_meta**: component_model re-exports macros, meta implements them
- **component_model vs former**: component_model provides assignment traits, former provides builder pattern implementation
- **Absorption pattern**: Runtime crate absorbs meta and types to prevent circular dependencies

## Architecture

### Dependency Structure

```
component_model (runtime, absorption crate)
├── Internal Dependencies
│   ├── component_model_meta (workspace, proc macros)
│   └── component_model_types (workspace, trait definitions)
└── Dev Dependencies
    ├── test_tools (workspace)
    └── collection_tools (workspace, for tests)
```

### Absorption Pattern

component_model follows the absorption pattern with two absorbed crates:

```
Component Model Ecosystem
├── component_model_types (types only, no dependencies on others)
├── component_model_meta (proc macro, depends on types)
└── component_model (runtime, absorbs both meta and types)
```

**Prevents Circular Dependencies:**
- meta needs types for trait definitions
- runtime needs both meta (macros) and types (traits)
- No circular dependency possible

### Module Organization

```
component_model
├── lib.rs (traditional namespaces, re-exports only)
├── dependency module (component_model_types, component_model_meta)
└── Standard namespaces: own, orphan, exposed, prelude
```

**Note:** Uses traditional module pattern, not mod_interface! (absorption crate)

### Feature Architecture

```
enabled (master switch)
├── full (all features, default)
│   ├── derive_component_model (unified derive)
│   │   ├── derive_component_assign
│   │   ├── derive_components_assign
│   │   ├── derive_component_from
│   │   └── derive_from_components
│   ├── derive_components (alternative unified derive)
│   ├── derive_component_assign
│   ├── derive_components_assign
│   ├── derive_component_from
│   ├── derive_from_components
│   └── types_component_assign
│
no_std (embedded support)
└── use_alloc (requires alloc)
```

**Default Features:** `enabled`, `full`

**Feature Propagation:**
- `derive_*` features propagate to `component_model_meta`
- `types_*` features propagate to `component_model_types`
- `no_std`/`use_alloc` propagate to both

## Public API

### Derive Macros (Re-exported from component_model_meta)

```rust
#[cfg(feature = "derive_component_model")]
/// Unified derive macro combining all component model functionality
pub use component_model_meta::ComponentModel;

#[cfg(feature = "derive_component_assign")]
/// Derive Assign trait for type-driven component assignment
pub use component_model_meta::Assign;

#[cfg(feature = "derive_components_assign")]
/// Derive for assigning multiple components at once
pub use component_model_meta::ComponentsAssign;

#[cfg(feature = "derive_component_from")]
/// Derive for creating component from single value
pub use component_model_meta::ComponentFrom;

#[cfg(feature = "derive_from_components")]
/// Derive for creating component from multiple values
pub use component_model_meta::FromComponents;
```

### Traits (Re-exported from component_model_types)

```rust
#[cfg(feature = "types_component_assign")]
pub use component_model_types::Assign;
pub use component_model_types::OptionExt;
pub use component_model_types::AssignWithType;
pub use component_model_types::PopularType;
```

### Modules

```rust
/// Popular type support for std library types
pub use component_model_types::popular_types;

/// Explicit dependency access
pub mod dependency {
  pub use component_model_types;
  pub use component_model_meta;
}
```

## Usage Patterns

### Pattern 1: ComponentModel Derive (Recommended)

```rust
use component_model::{ComponentModel, Assign};

#[derive(Default, Debug, ComponentModel)]
struct Person {
  age: i32,
  name: String,
}

let mut person = Person::default();
person.assign(25);        // Sets age: i32
person.assign("Alice");   // Sets name: String

assert_eq!(person.age, 25);
assert_eq!(person.name, "Alice");
```

### Pattern 2: Fluent API with impute

```rust
use component_model::{ComponentModel, Assign};

#[derive(Default, ComponentModel)]
struct Config {
  host: String,
  port: i32,
}

// Fluent chaining
let config = Config::default()
  .impute("localhost")
  .impute(8080);

assert_eq!(config.host, "localhost");
assert_eq!(config.port, 8080);
```

### Pattern 3: Popular Types Support

```rust
use component_model::{ComponentModel, Assign};
use std::time::Duration;
use std::path::PathBuf;

#[derive(Default, ComponentModel)]
struct AppConfig {
  timeout: Duration,
  config_path: PathBuf,
}

let mut config = AppConfig::default();
config.assign(Duration::from_secs(30));
config.assign(PathBuf::from("/etc/app.conf"));
```

### Pattern 4: Multiple Component Assignment

```rust
use component_model::{ComponentsAssign, Assign};

#[derive(Default, ComponentsAssign)]
struct Server {
  host: String,
  port: i32,
  timeout: u64,
}

let mut server = Server::default();
server.components_assign(("localhost", 8080, 300u64));

assert_eq!(server.host, "localhost");
assert_eq!(server.port, 8080);
assert_eq!(server.timeout, 300);
```

### Pattern 5: Component Creation

```rust
use component_model::FromComponents;

#[derive(FromComponents)]
struct Point {
  x: i32,
  y: i32,
}

let point = Point::from_components((10, 20));
assert_eq!(point.x, 10);
assert_eq!(point.y, 20);
```

### Pattern 6: No-std Usage

```rust
#![no_std]
extern crate alloc;

use component_model::{ComponentModel, Assign};
use alloc::string::String;

#[derive(Default, ComponentModel)]
struct Data {
  value: i32,
  label: String,
}

let mut data = Data::default();
data.assign(42);
data.assign("embedded");
```

## Dependencies and Consumers

### Direct Dependencies

**Internal:**
- `component_model_meta` (workspace, optional) - Procedural macro implementations
- `component_model_types` (workspace, optional) - Trait definitions

**Dev Dependencies:**
- `test_tools` (workspace) - Testing utilities
- `collection_tools` (workspace) - Collection macros for tests

### Consumers (0 workspace crates)

component_model is a user-facing API crate designed for external consumption, not for use by other wTools crates. It serves as the entry point for the component model ecosystem.

**Usage:** Intended for application developers building:
- Configuration objects
- Fluent APIs
- Builder pattern implementations
- Composable systems

## Design Rationale

### Why Absorption Pattern?

**Problem:** Circular dependency risk:

```
component_model (runtime)
↓ depends on
component_model_meta (proc macro)
↓ wants to depend on (for types)
component_model (runtime)  ← CIRCULAR!
```

**Solution:** Three-crate architecture:

```
component_model_types (types only)
↑                   ↑
component_model_meta   component_model
(proc macro)          (runtime, absorbs both)
```

**Benefits:**
1. **No Circular Dependencies**: Meta depends on types, runtime depends on both
2. **User Simplicity**: Single import (`use component_model::*`)
3. **Ecosystem Coherence**: All parts versioned together
4. **Clear Separation**: Types vs macros vs aggregation

### Why Re-export Only?

component_model contains NO implementation code, only re-exports. This is intentional:

1. **Single Responsibility**: Aggregation is its only job
2. **Simplicity**: No logic = no bugs in aggregation layer
3. **Transparency**: Users get exactly what meta/types provide
4. **Maintenance**: Changes only in meta/types, not runtime

**Tradeoff:** Additional crate in workspace, but cleaner architecture.

### Why ComponentModel Derive?

The `ComponentModel` derive is a unified macro that combines:
- `Assign` - Component assignment
- `ComponentsAssign` - Multiple components
- `ComponentFrom` - Single value construction
- `FromComponents` - Multiple value construction

**Benefits:**
1. **Simplicity**: One derive for all functionality
2. **Discoverability**: Users find one macro, get everything
3. **Consistency**: Same macro across different use cases

**Tradeoff:** Larger generated code, but better UX.

### Why Popular Types?

Standard library types like `Duration`, `PathBuf`, `SocketAddr` need special handling because:

1. **Orphan Rule**: Can't implement Assign for foreign types
2. **Common Patterns**: These types appear frequently in configs
3. **User Expectation**: Should "just work" for std types

**Solution:** Generate implementations in macro for recognized types.

### Why No Validation?

Component model focuses on assignment, not validation:

1. **Single Responsibility**: Assignment is orthogonal to validation
2. **Flexibility**: Users can validate however they want
3. **Performance**: No runtime overhead for validation

Validation should be added separately (e.g., using validator crates or custom logic).

### Why Not Replace former?

component_model and former serve different purposes:

- **component_model**: Type-driven assignment mechanism
- **former**: Builder pattern scaffolding (type generation, end conditions, etc.)

former actually uses component_model internally for assignments, showing they're complementary.

## Testing Strategy

### Test Coverage

- **43 test files**: Comprehensive behavior validation
- **Doc Tests**: Embedded in readme.md
- **Integration Tests**: Uses test_tools for integration testing

### Test Focus

1. **Derive Macro Correctness**: Verify generated code works
2. **Popular Types**: Verify std library type support
3. **Feature Gating**: Verify each feature works independently
4. **No-std**: Verify functionality in embedded environments
5. **Type Safety**: Verify compile-time type checking

## Future Considerations

### Potential Enhancements

1. **Standalone Constructors**: Generate top-level constructor functions
2. **Argument Attributes**: Mark fields as constructor arguments
3. **Enum Support**: Component model for enum variants
4. **Custom Error Types**: Better error reporting in derives
5. **More Popular Types**: Expand std library type support

### Breaking Changes to Consider

1. **Unified API**: Consolidate all derives into ComponentModel only
2. **Feature Structure**: Simplify feature hierarchy
3. **Decouple from former**: Already decoupled, maintain separation

### Known Limitations

1. **Orphan Rule**: Cannot implement Assign for foreign types without macro
2. **Type Ambiguity**: Multiple fields of same type require workarounds
3. **No Validation**: Assignment doesn't validate component values

## Adoption Guidelines

### When to Use component_model

**Good Candidates:**
- Configuration objects with many fields
- Fluent API implementations
- Builder pattern with type-driven assignment
- Applications needing flexible object composition

**Poor Candidates:**
- Simple structs with few fields (use direct initialization)
- Performance-critical code (trait overhead)
- Foreign types (orphan rule limitations)

### Migration from Manual Builders

```rust
// Before: Manual builder
impl MyStruct {
  fn new() -> Self { Default::default() }
  fn with_field1(mut self, val: T1) -> Self { self.field1 = val; self }
  fn with_field2(mut self, val: T2) -> Self { self.field2 = val; self }
}

// After: ComponentModel
#[derive(Default, ComponentModel)]
struct MyStruct {
  field1: T1,
  field2: T2,
}
// Methods generated automatically!
```

### Best Practices

1. **Use ComponentModel Derive**: Prefer unified derive over individual derives
2. **Derive Default**: Always derive Default for fluent APIs
3. **Document Fields**: Component model works best with well-documented types
4. **Avoid Type Ambiguity**: Multiple fields of same type need special handling

## Related Crates

- **component_model_types**: Type definitions (absorbed crate)
- **component_model_meta**: Procedural macros (absorbed crate)
- **former**: Builder pattern implementation (uses component_model internally)
- **collection_tools**: Collection macros (used in tests)

## References

- [API Documentation](https://docs.rs/component_model)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/component_model)
- [component_model_types](https://docs.rs/component_model_types) - Trait definitions
- [component_model_meta](https://docs.rs/component_model_meta) - Procedural macros
- [readme.md](./readme.md)
