# src/

Production source code implementing procedural macro utilities and abstractions over syn/quote/proc-macro2.

## Organization

Flat module structure with selected submodules for complex subsystems. Each module provides focused utilities for specific procedural macro tasks.

## Module Categories

- **Attribute parsing**: attr, attr_prop - Parse and analyze procedural macro attributes
- **Type analysis**: typ, typed, container_kind - Extract and classify type information
- **Generic manipulation**: generic_params, generic_args - Handle generic parameters and arguments
- **Item utilities**: item, item_struct, struct_like - Parse and manipulate syn items
- **Identifier tools**: ident, name - Create and transform identifiers
- **Code generation**: tokens, phantom, derive - Generate token streams and derive utilities
- **Error handling**: diag - Format diagnostic messages and error reporting
- **Parsing primitives**: quantifier, punctuated, equation - Parse common patterns
- **Compile-time tools**: ct - Const-time string formatting integration
- **Component model**: components - Re-export component model types

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `lib.rs` | Define crate structure and public API surface |
| `attr.rs` | Extract and analyze procedural macro attributes |
| `attr_prop.rs` | Parse property-based attribute components |
| `attr_prop/` | Contain attribute property type implementations |
| `components.rs` | Re-export component model assignment traits |
| `container_kind.rs` | Detect and classify container types |
| `ct.rs` | Provide compile-time string formatting utilities |
| `ct/` | Contain compile-time formatting submodules |
| `derive.rs` | Provide derive macro parsing utilities |
| `diag.rs` | Format diagnostic messages and debugging output |
| `equation.rs` | Parse and manipulate equation expressions |
| `generic_args.rs` | Extract and process generic arguments |
| `generic_params.rs` | Parse and decompose generic parameters |
| `generic_params/` | Contain generic parameter manipulation algorithms |
| `ident.rs` | Create and transform identifier tokens |
| `item.rs` | Parse and manipulate general syn items |
| `item_struct.rs` | Parse and analyze struct items specifically |
| `iter.rs` | Provide iterator composition utilities |
| `kw.rs` | Define custom keyword parsing |
| `name.rs` | Transform and format identifier names |
| `phantom.rs` | Generate phantom type expressions |
| `punctuated.rs` | Manipulate punctuated sequences |
| `quantifier.rs` | Parse Pair and Many quantified patterns |
| `struct_like.rs` | Detect and parse struct-like items |
| `tokens.rs` | Provide token stream manipulation utilities |
| `typ.rs` | Extract type information and parameters |
| `typed.rs` | Wrap typed value extraction |
