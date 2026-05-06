# API: Macro API

### Scope

- **Purpose**: Define the public macro interface of the `for_each` crate for external callers.
- **Responsibility**: Documents the three exported macros — their invocation conventions, parameter semantics, error conditions, and compatibility guarantees.
- **In Scope**: `for_each!`, `braces_unwrap!`, and `identity!` — calling conventions, parameters, and behavioural contracts.
- **Out of Scope**: Internal expansion mechanics (→ `feature/001_for_each_iteration.md`); future API extensions not yet committed.

### Abstract

The `for_each` crate exports three `#[macro_export]` macros that together form a compile-time token-list iteration facility. All three are unconditionally visible to external crates once the `enabled` feature gate is active. The API has no runtime component — all expansion occurs at compile time with zero heap allocation and zero I/O side-effects.

### Operations

**`for_each!`** — the primary iteration macro.

| Convention | Syntax | When to use |
|------------|--------|-------------|
| Function-style | `for_each!(Callback, elem1, elem2, …)` | Simple per-element dispatch; no prefix or postfix |
| Map-style | `for_each! { Callback where @Prefix P @Postfix Q @Each e1 e2 … }` | Needs prefix or postfix tokens around each element |
| Callbackless | `for_each! { @Prefix P @Postfix Q @Each e1 e2 … }` | Prefix/postfix generation without an explicit callback |

`@Prefix` and `@Postfix` are optional in map-style and callbackless forms; `@Each` is mandatory. Multi-token elements must be wrapped in braces; the outermost brace pair is stripped before dispatch.

**`braces_unwrap!`** — brace-stripping dispatch helper.

| Convention | Effect |
|------------|--------|
| `braces_unwrap!(Callback, { a, b })` | Strips outer braces; calls `Callback!(a, b)` |
| `braces_unwrap!(Callback, a, b)` | No braces to strip; calls `Callback!(a, b)` |
| Map-style with `@SRC`, `@Prefix`, `@Postfix` | Handles all eight combinations of braced/unbraced prefix, postfix, and source |

Covers sixteen permutations of braced vs. unbraced prefix, postfix, and source tokens.

**`identity!`** — transparent pass-through.

Accepts any token sequence and returns it unchanged. Used internally by the callbackless form of `for_each!`; also available directly to callers who need a no-op macro slot.

### Error Handling

All three macros are purely compile-time. Errors manifest as compile-time diagnostics, not runtime panics or results. Error conditions:

- Missing mandatory parameter (`@Each` absent from map-style) — compile error at macro call site.
- Unknown `@`-prefixed keyword — compile error; no silent fallback.
- Callback macro rejects the tokens it receives — compile error propagated from the callback's own expansion.
- Mismatched brace structure — compile error from the Rust macro parser before the macro body runs.

No recovery or fallback is possible; all errors are hard compilation failures.

### Compatibility Guarantees

Macro names, calling conventions, and token-tree semantics are stable across patch and minor releases within the same major version. New optional named parameters or invocation variants may be introduced in minor releases without breaking existing call sites. Removing or renaming existing parameters requires a major version bump. The crate carries zero transitive dependencies; API compatibility is unaffected by upstream crate changes.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | [`src/lib.rs`](../../src/lib.rs) | All three macro implementations |
| test | [`tests/inc/for_each_test.rs`](../../tests/inc/for_each_test.rs) | Behavioural tests for all calling conventions and edge cases |
| test | [`tests/example_output_quality_test.rs`](../../tests/example_output_quality_test.rs) | Example correctness and output quality validation |
| feature/001 | [`feature/001_for_each_iteration.md`](../feature/001_for_each_iteration.md) | Feature-level design decisions and invocation mode rationale |
