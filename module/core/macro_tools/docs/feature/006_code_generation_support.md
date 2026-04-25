# Feature: Code Generation Support

### Scope
- **Purpose**: Provide utilities for token stream construction, derive macro scaffolding, and compile-time operations.
- **Responsibility**: Navigate all artifacts for token stream, derive, iterator, and compile-time utility capabilities.
- **In Scope**: Token stream utilities, derive macro helpers, typed generation helpers, iterator composition, compile-time string formatting, keyword definitions.
- **Out of Scope**: Attribute parsing → feature/001; error reporting → feature/005; direct quoting via prelude re-exports.

### Design
Token stream utilities extend the base quoting primitives with higher-level operations
for patterns common in derive macro implementation. Derive helpers provide scaffolding
that reduces boilerplate when generating impl blocks — for example, composing where
clauses or assembling associated type assignments. Typed generation helpers produce
tokens for specific kinds of constructs in a reusable form. Iterator composition
utilities support processing field lists and parameter sequences during code generation
without rewriting iteration logic per consumer. Compile-time string formatting uses
a constant-evaluation approach to produce error text without heap allocation, enabling
diagnostics inside const contexts. Keyword definitions provide a uniform way to declare
and recognise custom syntax keywords across consumers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/tokens.rs` | Token stream utilities |
| source | `src/derive.rs` | Derive macro scaffolding helpers |
| source | `src/typed.rs` | Typed code generation utilities |
| source | `src/iter.rs` | Iterator composition for code generation |
| source | `src/ct.rs` | Compile-time string formatting module router |
| source | `src/ct/str.rs` | Compile-time string operations |
| source | `src/kw.rs` | Custom keyword definitions |
| test | `tests/inc/tokens_test.rs` | Token stream utility correctness |
| test | `tests/inc/derive_test.rs` | Derive helper correctness |
| test | `tests/inc/compile_time_test.rs` | Compile-time utility correctness |
