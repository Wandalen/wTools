# Feature: Template Value Trait

### Scope

- **Purpose**: Provides a trait that any value type implements to participate in template rendering.
- **Responsibility**: Documents the `TemplateValue` trait contract and its role in the library.
- **In Scope**: Trait definition, required methods, and custom implementation support.
- **Out of Scope**: Concrete value types (→ 002), rendering mechanics (→ 006, 007).

### Design

The library exposes a template value trait with three required methods: conversion to a template string, construction from a string, and emptiness check. Any custom value type implements this trait to participate in template rendering without coupling to a specific CLI framework or value representation. The built-in value type implements this trait as the default option.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/value.rs` | Trait definition and built-in value type implementation |
| test | `tests/` | Value trait contract tests |
| doc | `docs/feature/002_default_value_type.md` | Built-in value type that implements this trait |

### Sources

| File | Notes |
|------|-------|
| [`../../spec.md`](../../spec.md) | FR1 in original spec; combined source migrated to feature/. spec.md has been deleted — Sources entry retained as migration record. |
