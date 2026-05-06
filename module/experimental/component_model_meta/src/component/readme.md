# Component Directory

This directory contains procedural macro implementations for the component model pattern.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `component_assign.rs` | Generate `Assign` trait implementations for mutable field assignment |
| `component_from.rs` | Generate `From<&Struct>` trait implementations for extracting field values |
| `component_model.rs` | Unified `ComponentModel` derive combining all component traits |
| `components_assign.rs` | Generate `ComponentsAssign` trait for bulk field assignment from tuples |
| `from_components.rs` | Generate `From<T>` trait for constructing struct from source type |

## Component Model Pattern

The component model pattern enables composition-based programming where structs expose individual fields through generated traits:

- **Field Extraction** (`component_from.rs`): Convert struct reference to individual field values
- **Field Assignment** (`component_assign.rs`): Mutably assign individual fields by type
- **Bulk Assignment** (`components_assign.rs`): Assign multiple fields from tuple source
- **Construction** (`from_components.rs`): Build struct from source implementing field conversions
- **Unified Interface** (`component_model.rs`): Single derive providing all capabilities

## Architecture

All macros follow consistent implementation patterns:
- Type deduplication via `HashSet` to prevent conflicting trait implementations
- Generic type parameter support via `generics.split_for_impl()`
- Debug attribute support for macro expansion debugging
- Proper error handling via `macro_tools::Result`
