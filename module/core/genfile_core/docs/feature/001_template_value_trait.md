# Feature: Template Value Trait

### Scope

- **Purpose**: Provides a trait that any value type implements to participate in template rendering.
- **Responsibility**: Documents the template value trait contract and its role in the library.
- **In Scope**: Trait definition, required methods, and custom implementation support.
- **Out of Scope**: Concrete value types (→ 002), rendering mechanics (→ 006, 007).

### Design

The library exposes a template value trait with three required methods: conversion to a template string, construction from a string, and emptiness check. Any custom value type implements this trait to participate in template rendering without coupling to a specific CLI framework or value representation. The built-in value type implements this trait as the default option.

### Features

| File | Relationship |
|------|--------------|
| [`feature/002_default_value_type.md`](002_default_value_type.md) | Built-in value type that implements this trait |

### Sources

| File | Relationship |
|------|--------------|
| [`src/value.rs`](../../src/value.rs) | Trait definition |

### Tests

| File | Relationship |
|------|--------------|
| [`tests/inc/value_test.rs`](../../tests/inc/value_test.rs) | Value trait contract tests |
