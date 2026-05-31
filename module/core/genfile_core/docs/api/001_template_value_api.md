# API: Template Value

### Scope

- **Purpose**: Defines the contract for custom value types and the built-in default implementation.
- **Responsibility**: Documents the template value trait surface and built-in value type variants available to callers.
- **In Scope**: Trait methods, built-in value variants, string conversion contract.
- **Out of Scope**: Value storage at runtime (→ `feature/005`), rendering mechanics (→ `feature/006`).

### Design

Callers either use the built-in value type directly or provide a custom type implementing the template value trait. The trait requires three operations: conversion to a template string, construction from a string, and an emptiness check. The built-in type supports String, Number (integer), Bool, and List variants with predictable string conversions. Callers interact with this surface when building a value storage map prior to generation.

### Features

| File | Relationship |
|------|--------------|
| [feature/001_template_value_trait.md](../feature/001_template_value_trait.md) | Trait being contracted here |
| [feature/002_default_value_type.md](../feature/002_default_value_type.md) | Built-in implementation of this contract |

### Sources

| File | Relationship |
|------|--------------|
| `src/value.rs` | Trait and built-in type definitions |
