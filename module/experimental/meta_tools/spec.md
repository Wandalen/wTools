# Specification: meta_tools

## Overview

**meta_tools** is a facade crate aggregating meta-programming utilities from workspace crates (for_each, impls_index, mod_interface) and external sources (paste), providing a unified interface for compile-time code generation, macro iteration, trait implementation generation, and identifier manipulation. It serves as the workspace's central hub for meta-programming infrastructure, enabling powerful compile-time abstractions through declarative and procedural macros.

**Version:** 0.12.0
**Status:** Experimental
**Category:** Development Tools (Meta-Programming)
**Dependents:** Unknown (likely most workspace crates using advanced patterns)

### Scope

#### Responsibility

Aggregate and re-export meta-programming utilities from workspace crates (for_each, impls_index, impls_index_meta, mod_interface, mod_interface_meta) and external sources (paste), providing a unified feature-gated interface for compile-time code generation and macro utilities across the workspace.

#### In-Scope

1. **Macro Iteration (for_each)**
   - `for_each!` macro - Apply macro to each element
   - Function-style: `for_each!(dbg, "a", "b")`
   - Map-style invocation
   - Comma-delimited element lists
   - Compile-time iteration

2. **Trait Implementation Generation (impls_index)**
   - `index!` - Generate Index trait implementations
   - `impls1!`, `impls2!`, `impls3!` - Multi-level impl generation
   - `impls!` - Unified impl macro (alias for impls3)
   - `impls_optional!` - Optional trait impls
   - `tests_index!`, `tests_impls!` - Test generation
   - Tuple indexing utilities

3. **Module Interface Pattern (mod_interface)**
   - `mod_interface!` - Module organization macro
   - Layer-based visibility control
   - Namespace management (own, orphan, exposed, prelude)
   - Automatic re-exports
   - Declarative module structure

4. **Identifier Concatenation (paste)**
   - `paste!` macro (re-exported as `meta_idents_concat!`)
   - Token pasting at compile-time
   - Identifier generation
   - Hygiene-safe concatenation

5. **Function Utilities (impls_index)**
   - `fn_name!` - Function name extraction
   - `fn_rename!` - Function renaming
   - `fns!`, `fns2!` - Function generation helpers

6. **Feature Architecture**
   - `enabled` - Master switch (default)
   - `meta_for_each` - for_each macro (default)
   - `meta_impls_index` - impls generation (default)
   - `mod_interface` - module pattern (default)
   - `meta_idents_concat` - paste integration (default)
   - `no_std` / `use_alloc` support

7. **Traditional Namespace Organization**
   - Standard namespaces: own, orphan, exposed, prelude
   - Dependency namespace for explicit access
   - Feature-gated re-exports
   - Meta-specific namespace

8. **Unified Import Point**
   - Single `use meta_tools::*;` for all tools
   - Consistent macro naming
   - Cross-crate compatibility

#### Out-of-Scope

1. **NOT Runtime Code Generation**
   - All tools are compile-time only
   - No dynamic code generation
   - **Rationale:** Meta-programming is compile-time

2. **NOT Full Proc Macro Framework**
   - Aggregates specific utilities, not general framework
   - No custom proc macro development tools
   - **Rationale:** Focused on specific utilities

3. **NOT AST Manipulation Library**
   - No syn/quote wrappers
   - No direct AST access
   - **Rationale:** Use macro_tools for AST work

4. **NOT Template Engine**
   - No text template expansion
   - Only token-level manipulation
   - **Rationale:** Rust macros not template engine

5. **NOT Code Formatting**
   - No formatting utilities
   - Generated code uses default formatting
   - **Rationale:** Use rustfmt

6. **NOT Build Script Utilities**
   - No build.rs helpers
   - Compile-time only
   - **Rationale:** Focused on macro usage

7. **NOT Reflection Framework**
   - No runtime type information
   - Compile-time code generation only
   - **Rationale:** Use reflect_tools for reflection

8. **NOT Custom Syntax**
   - Standard Rust macro syntax only
   - No custom parsers
   - **Rationale:** Compatibility and simplicity

#### Boundaries

- **meta_tools vs for_each**: meta_tools aggregates for_each; for_each is standalone iteration utility
- **meta_tools vs macro_tools**: meta_tools provides high-level macros; macro_tools provides AST utilities
- **meta_tools vs paste**: meta_tools re-exports paste; paste is external token pasting crate

## Architecture

### Dependency Structure

```
meta_tools (facade, aggregation)
├── Internal Dependencies (workspace)
│   ├── for_each (macro iteration)
│   ├── impls_index (trait impl generation, declarative)
│   ├── impls_index_meta (trait impl generation, procedural)
│   ├── mod_interface (module pattern, declarative)
│   └── mod_interface_meta (module pattern, procedural)
├── External Dependencies (crates.io)
│   └── paste (optional, identifier concatenation)
└── Dev Dependencies
    └── test_tools (workspace, testing)
```

**Note:** All production dependencies are optional and feature-gated

### Module Organization

```
meta_tools
├── lib.rs (facade aggregation)
├── meta/ - Meta-programming namespace
│   └── mod.rs - Internal organization
├── dependency.rs - Explicit dependency access
│   ├── for_each
│   ├── impls_index + impls_index_meta
│   ├── mod_interface + mod_interface_meta
│   └── paste
├── own.rs - Own namespace
├── orphan.rs - Orphan namespace
├── exposed.rs - Exposed namespace
└── prelude.rs - Prelude namespace
```

**Pattern:** Pure facade with traditional namespace organization

### Feature Architecture

```
enabled (master switch, default)
├── for_each/enabled
├── impls_index/enabled
└── mod_interface/enabled
│
├── meta_for_each (default)
│   └── for_each! - Macro iteration
│
├── meta_impls_index (default)
│   ├── index! - Index trait impls
│   ├── impls1!, impls2!, impls3! - Impl generation
│   ├── impls! - Unified impl (alias)
│   ├── impls_optional! - Optional impls
│   ├── tests_index!, tests_impls! - Test generation
│   └── fn_name!, fn_rename!, fns!, fns2! - Function utils
│
├── mod_interface (default)
│   └── mod_interface! - Module pattern
│
└── meta_idents_concat (default)
    └── paste! (as meta_idents_concat!) - Token pasting

full (all features)
no_std (embedded support)
use_alloc (no_std + allocation)
```

**Default Features:** `enabled`, `meta_for_each`, `meta_impls_index`, `mod_interface`, `meta_idents_concat`

### Macro Flow

#### for_each Macro Flow

```
for_each!(dbg, "a", "b", "c")
  ↓
Declarative macro expansion
  ↓
Generates:
  dbg!("a");
  dbg!("b");
  dbg!("c");
  ↓
Compiled into user's crate
```

#### mod_interface Macro Flow

```
mod_interface! {
  layer use_basic;
  layer use_tools;
}
  ↓
Procedural macro expands
  ↓
Generates:
  - own namespace
  - orphan namespace
  - exposed namespace
  - prelude namespace
  - Appropriate re-exports
  ↓
Compiled module structure
```

#### impls Macro Flow

```
impls! {
  impl Index<usize> for MyTuple { ... }
}
  ↓
Procedural macro generates implementations
  ↓
impl Index<usize> for (T0,) { ... }
impl Index<usize> for (T0, T1) { ... }
impl Index<usize> for (T0, T1, T2) { ... }
// ... up to specified arity
```

## Public API

### Macro Iteration (for_each)

```rust
#[cfg(feature = "meta_for_each")]
pub use ::for_each::*;

// Primary macro:
// for_each!(macro_name, elem1, elem2, ...)
```

### Trait Implementation Generation (impls_index)

```rust
#[cfg(feature = "meta_impls_index")]
pub use ::impls_index::{
  index,           // Index trait impl generation
  tests_index,     // Index tests generation
  impls1,          // Level 1 impl generation
  impls_optional,  // Optional trait impls
  tests_impls,     // Impl tests
  tests_impls_optional, // Optional impl tests
  impls2,          // Level 2 impl generation
  fn_name,         // Function name extraction
  fn_rename,       // Function renaming
  fns,             // Function generation
  fns2,            // Function generation v2
};

#[cfg(feature = "meta_impls_index")]
pub use ::impls_index_meta::{
  impls3,          // Level 3 impl generation (proc macro)
};

// Alias for convenience
#[cfg(feature = "meta_impls_index")]
pub use ::impls_index_meta::impls3 as impls;
```

### Module Interface Pattern (mod_interface)

```rust
#[cfg(feature = "mod_interface")]
pub use ::mod_interface_meta::mod_interface;

// Usage:
// mod_interface! {
//   layer use_basic;
//   layer use_tools;
// }
```

### Identifier Concatenation (paste)

```rust
#[cfg(feature = "meta_idents_concat")]
pub use ::paste::paste as meta_idents_concat;

// Usage:
// meta_idents_concat! {
//   fn [<get_ $field>]() { ... }
// }
```

### Dependency Namespace

```rust
pub mod dependency {
  #[cfg(feature = "meta_for_each")]
  pub use ::for_each::*;

  #[cfg(feature = "meta_impls_index")]
  pub use ::impls_index::*;

  #[cfg(feature = "meta_impls_index")]
  pub use ::impls_index_meta::*;

  #[cfg(feature = "mod_interface")]
  pub use ::mod_interface::*;

  #[cfg(feature = "mod_interface")]
  pub use ::mod_interface_meta::*;

  #[cfg(feature = "meta_idents_concat")]
  pub use ::paste::*;
}
```

## Usage Patterns

### Pattern 1: Macro Iteration with for_each

```rust
use meta_tools::*;

// Function-style call
for_each!(dbg, "a", "b", "c");
// Expands to:
// dbg!("a");
// dbg!("b");
// dbg!("c");

// Works with any macro
for_each!(println, "Hello", "World", "!");
```

### Pattern 2: Module Interface Pattern

```rust
use meta_tools::*;

mod my_module {
  mod_interface! {
    // Define layers for different functionality
    layer use_basic;
    layer use_advanced;
  }

  // Generates:
  // - pub mod own { ... }
  // - pub mod orphan { ... }
  // - pub mod exposed { ... }
  // - pub mod prelude { ... }
}

// Use the generated namespaces
use my_module::prelude::*;
```

### Pattern 3: Trait Implementation Generation

```rust
use meta_tools::*;

// Generate Index implementations for tuples
impls! {
  impl<T> Index<usize> for MyTuple<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
      match index {
        0 => &self.0,
        1 => &self.1,
        _ => panic!("Index out of bounds"),
      }
    }
  }
}

// Generates implementations for various tuple sizes
```

### Pattern 4: Identifier Concatenation

```rust
use meta_tools::*;

struct Person {
  name: String,
  age: u32,
}

meta_idents_concat! {
  impl Person {
    fn [<get_ name>](&self) -> &str {
      &self.name
    }
    fn [<set_ name>](&mut self, value: String) {
      self.name = value;
    }
  }
}

// Generates: get_name() and set_name()
```

### Pattern 5: Function Name Extraction

```rust
use meta_tools::*;

fn_name!(my_function);
// Expands to: "my_function"

fn_rename!(old_name => new_name);
// Renames function at compile-time
```

### Pattern 6: Optional Trait Implementations

```rust
use meta_tools::*;

impls_optional! {
  impl MyTrait for MyType {
    // Only generated if MyTrait is in scope
  }
}
```

### Pattern 7: Test Generation

```rust
use meta_tools::*;

tests_index! {
  // Generates index tests for various sizes
}

tests_impls! {
  // Generates impl tests
}
```

### Pattern 8: Multi-Level Implementations

```rust
use meta_tools::*;

// Level 1: Basic impls
impls1! { /* ... */ }

// Level 2: Intermediate impls
impls2! { /* ... */ }

// Level 3: Advanced impls (proc macro)
impls3! { /* ... */ }
```

## Dependencies and Consumers

### Direct Dependencies

**Workspace:**
- `for_each` (optional) - Macro iteration utilities
- `impls_index` (optional) - Trait impl generation (declarative)
- `impls_index_meta` (always) - Trait impl generation (procedural)
- `mod_interface` (optional) - Module pattern (declarative)
- `mod_interface_meta` (always) - Module pattern (procedural)

**External:**
- `paste` (optional) - Identifier concatenation

**Dev:**
- `test_tools` (workspace) - Testing utilities

### Consumers (Unknown)

**Likely used by:**
- Workspace crates using mod_interface pattern
- Crates generating repetitive trait implementations
- Meta-programming heavy code
- Framework code

**Usage Pattern:** Workspace crates use meta_tools for advanced compile-time patterns, module organization, and reducing boilerplate through code generation.

## Design Rationale

### Why Facade Pattern?

Aggregates multiple meta-programming tools:

**Benefits:**
1. **Single Import**: One `use meta_tools::*;` for all tools
2. **Unified Documentation**: Centralized meta-programming reference
3. **Feature Control**: Granular dependency management
4. **Consistency**: Common interface across tools

**Tradeoff:** Indirection layer, but provides simplicity

### Why Mix Declarative and Procedural?

Combines declarative macros (impls_index) with procedural (impls_index_meta):

**Rationale:**
1. **Simplicity**: Declarative macros for simple cases
2. **Power**: Procedural macros for complex generation
3. **Performance**: Declarative compiles faster
4. **Flexibility**: Choose right tool for task

**Pattern:** Start declarative, upgrade to procedural when needed

### Why Re-export paste?

Includes external crate instead of reimplementing:

**Rationale:**
1. **Quality**: paste is battle-tested
2. **Maintenance**: Don't maintain token pasting logic
3. **Compatibility**: Standard tool in ecosystem
4. **Focus**: Focus on workspace-specific tools

**Alias:** Re-exported as `meta_idents_concat!` for consistency

### Why mod_interface Pattern?

Standardizes module organization:

**Rationale:**
1. **Consistency**: All workspace crates use same pattern
2. **Automation**: Reduce boilerplate namespace declarations
3. **Correctness**: Prevent re-export errors
4. **Documentation**: Self-documenting module structure

**Benefit:** Enforces workspace conventions

### Why Multiple impl Levels?

Three levels (impls1, impls2, impls3):

**Rationale:**
1. **Complexity**: Different levels for different needs
2. **Performance**: Lower levels compile faster
3. **Features**: Higher levels have more capabilities
4. **Migration**: Upgrade path as needs grow

**Pattern:** Start simple (impls1), progress to advanced (impls3/impls)

### Why Feature-Gate Everything?

Each tool has own feature flag:

**Rationale:**
1. **Compile Time**: Only compile needed tools
2. **Dependencies**: Minimize dependency tree
3. **Binary Size**: Exclude unused macro code
4. **Flexibility**: Fine-grained control

**Default:** Enable all common tools

### Why Separate Meta Crates?

for_each, impls_index, mod_interface as separate crates:

**Rationale:**
1. **Reusability**: Can use individually if needed
2. **Testing**: Isolated test suites
3. **Development**: Independent development cycles
4. **Clarity**: Clear responsibility boundaries

**Aggregation:** meta_tools provides convenient unified access

## Testing Strategy

### Test Coverage

**test_tools Available:**
- Can use test_tools for comprehensive testing
- Macro expansion testing

### Test Focus

1. **Macro Expansion**: Verify macros expand correctly
2. **Edge Cases**: Empty lists, single elements
3. **Nesting**: Nested macro invocations
4. **Error Messages**: Helpful compilation errors
5. **Feature Combinations**: Different feature sets

### Known Test Limitations

1. **Expansion Testing**: Cannot easily test intermediate expansion
2. **Hygiene**: Macro hygiene hard to test directly
3. **Error Testing**: Compile-fail tests for bad invocations
4. **Performance**: Compile-time performance not measured

## Future Considerations

### Potential Enhancements

1. **More Iteration Macros**: for_each variants (enumerate, filter, etc.)
2. **Better Error Messages**: Improve procedural macro diagnostics
3. **Documentation Generation**: Auto-generate macro docs
4. **Macro Composition**: Combine macros more easily
5. **Debug Utilities**: Macro expansion debugging tools
6. **Performance**: Optimize compile-time performance
7. **Type-Level Programming**: Add type-level utilities

### Breaking Changes to Consider

1. **Rename Features**: Shorter feature names
2. **Change Defaults**: Adjust default feature set
3. **Unify impls**: Single impl macro instead of levels
4. **Namespace Simplification**: Flatten module structure
5. **Remove External Deps**: Replace paste with workspace impl

### Known Limitations

1. **Macro Limitations**: Subject to Rust macro system constraints
2. **Hygiene**: Macro hygiene can be tricky
3. **Error Messages**: Procedural macro errors can be cryptic
4. **Compile Time**: Many macros can slow compilation
5. **No Runtime**: All tools are compile-time only

## Adoption Guidelines

### When to Use meta_tools

**Good Candidates:**
- Repetitive trait implementations
- Module organization (mod_interface)
- Compile-time iteration
- Code generation patterns
- Identifier manipulation
- Workspace-standard patterns

**Poor Candidates:**
- Simple one-off code
- Runtime code generation
- Complex AST manipulation (use macro_tools)
- Performance-critical macros (minimize usage)

### Choosing Which Tools

```rust
// Module organization
use meta_tools::*;
mod_interface! {
  layer use_basic;
}

// Macro iteration
for_each!(dbg, val1, val2, val3);

// Trait impls for tuples
impls! {
  impl MyTrait for Tuple { ... }
}

// Identifier concatenation
meta_idents_concat! {
  fn [<get_ $field>]() { ... }
}
```

### Best Practices

1. **Start Simple**: Use declarative macros first
2. **Test Expansion**: Verify macro output
3. **Document Usage**: Comment macro invocations
4. **Minimize Nesting**: Keep macro nesting shallow
5. **Check Errors**: Ensure good error messages
6. **Feature Selection**: Only enable needed features

## Related Crates

**Dependencies:**
- **for_each**: Macro iteration utilities (workspace)
- **impls_index**: Trait impl generation declarative (workspace)
- **impls_index_meta**: Trait impl generation procedural (workspace)
- **mod_interface**: Module pattern declarative (workspace)
- **mod_interface_meta**: Module pattern procedural (workspace)
- **paste**: Identifier concatenation (external)

**Alternatives:**
- **macro_tools**: AST manipulation utilities (workspace, different level)
- **quote**: Code generation (for proc macros)
- **syn**: Parsing (for proc macros)

## References

- [API Documentation](https://docs.rs/meta_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/meta_tools)
- [readme.md](./readme.md)
- [for_each](../for_each/readme.md) - Macro iteration
- [impls_index](../impls_index/readme.md) - Impl generation declarative
- [mod_interface](../mod_interface/readme.md) - Module pattern
- [paste](https://docs.rs/paste) - Identifier concatenation
