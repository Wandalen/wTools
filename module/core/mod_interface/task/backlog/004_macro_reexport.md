# 004 — Support macro re-export via mod_interface! directives

## Status: 📥 Ready

- **ID:** 004
- **Priority:** 3
- **Executor:** any
- **Advisability:** 108
- **Value:** 6 / Easiness:** 3 / Safety:** 6

## Purpose

The `mod_interface!` DSL cannot currently re-export macros through layer directives.
Directive forms like `exposed(crate) use macro_name;` are not supported; they produce
a parse or expansion error. Authors who want to re-export a macro through a layer must
use conventional `pub use` outside the macro invocation.

## Context

The test file `tests/inc/derive/layer_use_macro/layer_a.rs` originally contained
commented-out `exposed(crate) use macro*;` directives with a `// xxx:` marker. The
marker was removed during hygiene cleanup (2026-05-04); the unimplemented feature is
tracked here.

The `layer_use_macro` test currently only passes an empty-body `mod_interface!`
invocation. Full coverage of macro re-export requires this feature to be implemented.

## Scope

- `mod_interface_meta/src/impls.rs` — detect macro identifier form in `use` path and
  generate appropriate `#[macro_export]` / `pub use` expansion
- `mod_interface_meta/src/record.rs` — extend `RecordUse` to distinguish macro paths
- `tests/inc/derive/layer_use_macro/` — re-enable and expand positive test cases

## MOST Goals

1. `exposed(crate) use macro_name;` directive parses and expands without error
2. Re-exported macro is accessible via the crate root (standard `macro_export` semantics)
3. Existing tests continue passing

## Notes

- Macro re-export interacts with `macro_export` scoping rules in Rust; implementation
  must handle the difference between macro-rules macros and proc-macros
- `macro*` glob form may require separate handling from named macro re-export
