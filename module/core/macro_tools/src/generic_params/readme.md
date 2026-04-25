# src/generic_params/

Generic parameter manipulation algorithms and utilities. Provides decomposition, filtering, merging, and classification of Rust generic parameters for procedural macros.

## Organization

Three focused modules implementing distinct generic parameter transformations: classify by kind, merge from multiple sources, filter by criteria.

## Algorithms

Each module implements specific transformation operations on `syn::Generics` for generating appropriate generic parameter lists in different contexts (impl blocks, type definitions, where clauses).

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `classification.rs` | Classify generic parameters by kind and decompose for different contexts |
| `combine.rs` | Merge generic parameters from multiple sources maintaining order |
| `filter.rs` | Filter generic parameters by type and constraints |
