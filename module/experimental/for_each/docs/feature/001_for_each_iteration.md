# Feature: For-Each Macro Iteration

### Scope

- **Purpose**: Enable repetitive compile-time code generation by applying a callback macro to each element of a statically enumerated token list.
- **Responsibility**: Documents the invocation modes, named-parameter conventions, callbackless variant, token-tree unwrapping, and zero-dependency design of the for-each iteration facility.
- **In Scope**: Function-style invocation, map-style invocation with `@Prefix`/`@Postfix`/`@Each`, callbackless mode, and brace-unwrapping semantics.
- **Out of Scope**: Runtime iteration (→ standard library); conditional or indexed iteration; procedural macro alternatives (→ `macro_tools`).

### Design

The for-each facility dispatches a callback to each element of a compile-time list, producing one callback invocation per element. All expansion occurs during compilation; no runtime allocations or I/O occur.

**Function-style invocation** accepts a callback name followed by a comma-separated element list. Each element is dispatched to the callback independently. A trailing comma is permitted.

**Map-style invocation** uses three named parameters introduced by `@`-prefixed keywords: `@Prefix` supplies tokens prepended to every element, `@Postfix` supplies tokens appended, and `@Each` lists the elements. `@Prefix` and `@Postfix` are optional; `@Each` is mandatory. Parameters must appear in declaration order: `@Prefix`, then `@Postfix`, then `@Each`. A `where` keyword separates the callback name from the parameter block.

**Callbackless mode** omits the callback name and `where` keyword entirely. Only named parameters are given. An internal identity pass-through serves as the implicit callback, producing prefix + element + postfix tokens for each element without invoking any external macro.

**Token-tree unwrapping** applies when an element is wrapped in curly braces. The outermost brace pair is stripped before the element reaches the callback. This allows multi-token elements that would otherwise be ambiguous to the macro parser. Parenthesised and bracketed elements are passed through without modification.

**Zero-dependency design** — the entire facility is implemented as declarative `macro_rules!` macros. No external crates are required; no heap allocation occurs. The `no_std` and `use_alloc` feature flags are provided for embedded and allocation-constrained environments respectively.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/lib.rs`](../../src/lib.rs) | Complete macro implementation — `for_each!`, `braces_unwrap!`, `identity!` |
| test | [`tests/inc/for_each_test.rs`](../../tests/inc/for_each_test.rs) | All invocation modes, edge cases, and higher-order composition |
| test | [`tests/smoke_test.rs`](../../tests/smoke_test.rs) | Package-level smoke tests |
| api/001 | [`api/001_macro_api.md`](../api/001_macro_api.md) | Public macro interface — operations, parameters, and compatibility |
