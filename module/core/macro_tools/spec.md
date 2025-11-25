# Specification: macro_tools

## Overview

**macro_tools** is the canonical abstraction layer over `syn`, `quote`, and `proc-macro2` for procedural macro development within the wTools ecosystem. It provides high-level utilities, advanced parsers, and precise error reporting to simplify the creation of robust and maintainable procedural macros.

**Version:** 0.76.0
**Status:** Experimental
**Category:** Meta-programming Infrastructure
**Dependents:** 12 crates (former_meta, derive_tools_meta, mod_interface_meta, component_model_meta, reflect_tools_meta, impls_index_meta, strs_tools_meta, unilang_meta, variadic_from_meta, clone_dyn_meta, former, macro_make)

### Scope

#### Responsibility

Act as the canonical abstraction layer over `syn`, `quote`, and `proc-macro2` for procedural macro development within the wTools ecosystem, providing high-level utilities for token stream manipulation, attribute parsing, type analysis, and error reporting.

#### In-Scope

1. **Token Stream Manipulation**
   - High-level token stream utilities (`tokens` module)
   - Quote macro re-exports and extensions
   - Token tree navigation and transformation

2. **Attribute Parsing Framework**
   - Structured attribute parsing (`attr`, `attr_prop` modules)
   - Property-based attribute system with `AttributeComponent` and `AttributePropertyComponent` traits
   - Boolean, optional, and custom property types
   - Composable attribute parsing with `Assign` trait
   - Span-aware error messages for attribute parsing failures

3. **Type Analysis Tools**
   - Type parameter extraction (`typ` module)
   - Nested generic parameter analysis
   - Container kind detection (`container_kind` module)
   - Path type parsing and manipulation
   - Type expression quantifier extraction (`quantifier` module)

4. **Generic Parameter Utilities**
   - Generic parameter decomposition (`generic_params` module)
   - Merging generics from multiple sources
   - Filtering and transforming generic parameters
   - Generating appropriate tokens for impl blocks
   - Generic argument utilities (`generic_args` module)

5. **Syntax Tree Helpers**
   - Struct-like item parsing (`struct_like`, `item_struct` modules)
   - Item utilities (`item` module)
   - Name and path manipulation (`name`, `ident` modules)
   - Punctuated sequence utilities (`punctuated` module)
   - Phantom type helpers (`phantom` module)

6. **Error Reporting and Diagnostics**
   - Enhanced diagnostics with custom error formatting (`diag` module)
   - Span-aware error messages
   - Helpful error generation for macro users
   - `syn::Error` result type (`Result<T>`)

7. **Compile-Time Utilities**
   - Compile-time string formatting (`ct` module)
   - Keyword utilities (`kw` module)
   - Constant evaluation helpers

8. **Code Generation Support**
   - Derive macro utilities (`derive` module)
   - Equation parsing and generation (`equation` module)
   - Component-based code generation (`components` module)

9. **Dependency Re-exports**
   - Re-export `syn`, `quote`, `proc-macro2` for unified versioning
   - Re-export common macros (`quote!`, `parse_quote!`, etc.)
   - Provide `dependency` module for explicit dependency access

10. **Feature-Gated Architecture**
    - Fine-grained feature gates for each module
    - `enabled` feature as master switch
    - `full` feature for complete functionality

#### Out-of-Scope

1. **NOT Direct syn/quote/proc-macro2 Replacement**
   - Does not re-implement core syn types
   - Does not replace quote macro functionality
   - Builds on top of these libraries, not replaces them
   - **Rationale:** Leverage battle-tested parsing and code generation libraries

2. **NOT General-Purpose Utilities**
   - No runtime-only utilities
   - No non-macro-related functionality
   - Does not provide CLI tools or executables
   - **Rationale:** Focused exclusively on procedural macro development

3. **NOT Macro Execution Framework**
   - Does not provide macro execution or invocation
   - Does not handle proc-macro registration
   - Does not provide testing infrastructure for macros
   - **Rationale:** These are responsibilities of the Rust compiler and test frameworks

4. **NOT Domain-Specific Macro Implementations**
   - Does not implement specific derive macros
   - Does not provide builder pattern macros (that's `former`)
   - Does not provide reflection macros (that's `reflect_tools_meta`)
   - **Rationale:** Provides tools for building macros, not specific macro implementations

5. **NOT Rust AST Manipulation**
   - Does not modify or transform existing ASTs
   - Does not provide AST validation
   - Does not enforce Rust language rules
   - **Rationale:** AST manipulation is syn's responsibility; macro_tools provides higher-level utilities

6. **NOT Code Formatting**
   - Does not format generated code
   - Does not enforce code style
   - Relies on rustfmt for output formatting
   - **Rationale:** Code formatting is the responsibility of rustfmt

7. **NOT Optimization**
   - Does not optimize generated code
   - Does not perform dead code elimination
   - Does not inline or transform for performance
   - **Rationale:** Code optimization is the compiler's responsibility

8. **NOT Documentation Generation**
   - Does not generate documentation comments
   - Does not extract or process doc comments
   - Does not provide doc testing utilities
   - **Rationale:** Documentation generation is rustdoc's responsibility

#### Boundaries

- **macro_tools vs syn**: macro_tools provides higher-level abstractions and patterns over syn's low-level AST types
- **macro_tools vs quote**: macro_tools extends quote with utilities but does not replace its code generation capabilities
- **macro_tools vs meta crates**: macro_tools provides the toolkit; meta crates (_meta suffix) use it to implement specific macros
- **macro_tools vs former**: former is a derive macro implementation; macro_tools provides the tools to build such macros
- **macro_tools vs derive_tools**: derive_tools is a general derive macro framework; macro_tools is the underlying toolkit

## Architecture

### Dependency Structure

```
macro_tools
├── External Dependencies
│   ├── syn ~2.0 (AST parsing)
│   ├── quote ~1.0 (code generation)
│   ├── proc-macro2 ~1.0 (token streams)
│   ├── const_format 0.2 (compile-time formatting)
│   └── convert_case (case conversion)
└── Internal Dependencies
    ├── interval_adapter (range utilities)
    ├── iter_tools (iterator composition)
    ├── clone_dyn_types (clone trait objects)
    └── component_model_types (component system types)
```

### Module Organization

macro_tools uses the traditional module organization pattern (own/orphan/exposed/prelude):

- **own**: All items from the crate, including re-exports
- **orphan**: Items without parent namespace (exposed + internal)
- **exposed**: Public API items
- **prelude**: Essential items for `use macro_tools::prelude::*`

### Feature Architecture

```
enabled (master switch)
├── attr (attribute parsing)
│   ├── diag
│   └── quantifier
├── attr_prop (attribute properties)
│   └── components
├── ct (compile-time utilities)
├── container_kind (container detection)
│   └── typ
├── derive (derive utilities)
├── diag (diagnostics)
├── equation (equation parsing)
├── generic_args (generic arguments)
├── generic_params (generic parameters)
│   └── punctuated
├── ident (identifier utilities)
│   └── kw
├── item (item utilities)
│   └── punctuated
├── item_struct (struct utilities)
├── kw (keyword utilities)
├── name (name utilities)
├── phantom (phantom types)
│   └── item
├── punctuated (punctuated sequences)
├── quantifier (type quantifiers)
├── struct_like (struct-like items)
│   └── item_struct
├── tokens (token stream utilities)
├── typ (type utilities)
└── typed (typed utilities)
```

**Default Features:** Most features enabled by default except `diag` (reverted from default)

### Core Abstractions

1. **Attribute Parsing System**
   - `AttributeComponent`: Trait for parsing attributes from `syn::Attribute`
   - `AttributePropertyComponent`: Marker trait for attribute properties
   - `Assign`: Trait for field assignment in attribute parsing
   - Property types: `AttributePropertyBoolean`, `AttributePropertyOptionalSyn`, `AttributePropertySingletone`

2. **Type Analysis System**
   - `typ::type_parameters`: Extract type parameters from `syn::Type`
   - `container_kind`: Detect container types (Option, Vec, HashMap, etc.)
   - `quantifier`: Extract quantifiers from type expressions

3. **Generic Parameter System**
   - `generic_params`: Decompose and merge `syn::Generics`
   - `generic_args`: Work with generic arguments

4. **Error Handling**
   - `Result<T>`: Type alias for `core::result::Result<T, syn::Error>`
   - `syn_err!`: Macro for creating span-aware errors
   - `return_syn_err!`: Macro for early return with error

## Public API

### Core Types

```rust
/// Result with `syn::Error`
pub type Result<T> = core::result::Result<T, syn::Error>;
```

### Attribute Parsing

```rust
/// Trait for parsing attributes from `syn::Attribute`
pub trait AttributeComponent
{
  const KEYWORD : &'static str;
  fn from_meta( attr : &syn::Attribute ) -> Result<Self>;
}

/// Marker trait for attribute properties
pub trait AttributePropertyComponent
{
  const KEYWORD : &'static str;
}

/// Trait for assigning fields in attribute parsing
pub trait Assign<TypeMarker, IntoType>
{
  fn assign( &mut self, component : IntoType );
}
```

### Type Analysis

```rust
/// Extract type parameters from a type within a given range
pub fn type_parameters( ty : &syn::Type, range : impl RangeBounds<usize> ) -> Vec<&syn::Type>;
```

### Re-exports

```rust
// Essential re-exports in prelude
pub use syn;
pub use proc_macro2;
pub use quote;
pub use quote::{quote, quote as qt, quote_spanned, format_ident};
pub use syn::{
  parse::ParseStream, Token, spanned::Spanned,
  braced, bracketed, custom_keyword, custom_punctuation, parenthesized,
  parse_macro_input, parse_quote, parse_quote as parse_qt,
  parse_quote_spanned, parse_quote_spanned as parse_qt_spanned,
};
```

## Usage Patterns

### Pattern 1: Extract Type Parameters

```rust
use macro_tools::{ typ, qt };

let code = qt!( Option<i32> );
let tree_type = syn::parse2::<syn::Type>( code ).unwrap();
let params = typ::type_parameters( &tree_type, 0..=0 );
params.iter().for_each( |param| println!( "{}", qt!( #param ) ) );
// Output: i32
```

### Pattern 2: Parse Structured Attributes

```rust
use macro_tools::exposed::*;

#[derive(Debug)]
pub struct CustomAttribute
{
  pub enabled : AttributePropertyBoolean<EnabledMarker>,
  pub name : AttributePropertyOptionalSyn<syn::LitStr, NameMarker>,
}

impl AttributeComponent for CustomAttribute
{
  const KEYWORD : &'static str = "custom";

  fn from_meta( attr : &syn::Attribute ) -> Result<Self>
  {
    match attr.meta
    {
      syn::Meta::List( ref meta_list ) =>
      {
        syn::parse2::<CustomAttribute>( meta_list.tokens.clone() )
      },
      _ => return_syn_err!( attr, "Expected attribute format: #[custom(...)]" ),
    }
  }
}
```

### Pattern 3: Component-Based Attribute Assignment

```rust
use macro_tools::{ Assign, AttributePropertyBoolean };

impl<IntoT> Assign<AttributePropertyDebug, IntoT> for AttributeMutator
where
  IntoT : Into<AttributePropertyDebug>,
{
  fn assign( &mut self, component : IntoT )
  {
    self.debug = component.into();
  }
}
```

### Pattern 4: Span-Aware Error Reporting

```rust
use macro_tools::{ syn_err, return_syn_err, qt };

let error = | attr : &syn::Attribute | -> syn::Error
{
  syn_err!( attr, "Expected format '#[custom(...)]'\nGot: '{}'", qt! { #attr } )
};

if invalid_condition {
  return_syn_err!( attr, "Invalid attribute" );
}
```

## Dependencies and Consumers

### Direct Dependencies

**External:**
- `syn` ~2.0 - Rust AST parsing
- `quote` ~1.0 - Code generation
- `proc-macro2` ~1.0 - Token stream manipulation
- `const_format` 0.2 - Compile-time string formatting
- `convert_case` - Case conversion utilities

**Internal:**
- `interval_adapter` - Range and interval utilities
- `iter_tools` - Iterator composition and utilities
- `clone_dyn_types` - Clone trait objects
- `component_model_types` - Component assignment system

### Consumers (12 crates)

**Meta Crates (10):**
1. `former_meta` - Builder pattern derive macro implementation
2. `derive_tools_meta` - General derive macro framework
3. `mod_interface_meta` - Module organization macro
4. `component_model_meta` - Component model derive macro
5. `reflect_tools_meta` - Reflection macro implementation
6. `impls_index_meta` - Implementation index macro
7. `strs_tools_meta` - String tools macro implementation
8. `unilang_meta` - Unilang macro implementation
9. `variadic_from_meta` - Variadic from macro
10. `clone_dyn_meta` - Clone dynamic trait objects macro

**Runtime Crates (2):**
11. `former` - Uses macro_tools in build scripts
12. `macro_make` - Macro composition utilities

## Design Rationale

### Why Abstraction Over syn/quote/proc-macro2?

1. **Unified Versioning**: All wTools macros use the same versions of syn/quote/proc-macro2
2. **Common Patterns**: Reduces duplication of parsing patterns across 10+ meta crates
3. **Consistent Error Messages**: Span-aware errors with consistent formatting
4. **Type Safety**: Property-based attribute system prevents runtime errors
5. **Ecosystem Integration**: Deep integration with wTools component system

### Why Property-Based Attribute System?

Traditional attribute parsing requires custom parsing logic for each attribute. The property-based system:

1. **Composability**: Properties can be mixed and matched
2. **Type Safety**: Each property has a unique type, ensuring compile-time validation
3. **Reusability**: Common properties (boolean, optional) provided by library
4. **Clarity**: Attribute structure matches field structure
5. **Error Handling**: Automatic span-aware error generation

### Why Fine-Grained Features?

1. **Compile Time**: Dependent crates only compile what they need
2. **Documentation**: Feature gates clearly show module dependencies
3. **Testing**: Each feature can be tested in isolation
4. **Evolution**: Features can be deprecated independently

### Why Traditional Module Pattern (Not mod_interface)?

1. **Established Before mod_interface**: macro_tools predates mod_interface
2. **Complexity**: mod_interface itself depends on macro_tools (circular dependency risk)
3. **Stability**: Traditional pattern is stable and well-understood
4. **Migration Risk**: Changing module structure is a breaking change

## Testing Strategy

### Test Coverage

- **Examples**: 5 comprehensive examples demonstrating core functionality
- **Doc Tests**: Embedded in readme.md and lib.rs
- **Integration Tests**: test_tools used for integration testing

### Example Programs

1. `macro_tools_trivial.rs` - Type parameter extraction
2. `macro_tools_extract_type_parameters.rs` - Advanced type analysis
3. `macro_tools_parse_attributes.rs` - Attribute parsing
4. `macro_tools_attr_prop.rs` - Property-based attributes
5. Additional examples in `examples/` directory

## Future Considerations

### Potential Enhancements

1. **Stabilization**: Move from experimental to maturing status
2. **Documentation**: Add formal specification sections to all modules
3. **Feature Optimization**: Audit and optimize feature dependencies
4. **syn Feature Binding**: Bind syn features to macro_tools features
5. **Procedural Macro Testing**: Utilities for testing procedural macros

### Breaking Changes to Consider

1. **Module Organization**: Potential migration to mod_interface pattern
2. **Feature Structure**: Consolidation of related features
3. **API Simplification**: Reduce prelude surface area
4. **Error Types**: Custom error type instead of syn::Error

## Related Crates

- **syn**: Low-level AST parsing (external dependency)
- **quote**: Code generation (external dependency)
- **proc-macro2**: Token stream manipulation (external dependency)
- **derive_tools**: General derive macro framework (consumer)
- **former_meta**: Builder pattern macros (consumer)
- **mod_interface_meta**: Module organization macros (consumer)
- **component_model_types**: Component assignment system (dependency)

## References

- [API Documentation](https://docs.rs/macro_tools)
- [Repository](https://github.com/Wandalen/wTools/tree/master/module/core/macro_tools)
- [Examples](https://github.com/Wandalen/wTools/tree/master/module/core/macro_tools/examples)
- [readme.md](./readme.md)
