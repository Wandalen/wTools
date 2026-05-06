# 003 — Support `priv use super::child` private-visibility directive

## Status: 📥 Ready

- **ID:** 003
- **Priority:** 3
- **Executor:** any
- **Advisability:** 108
- **Value:** 6 / Easiness:** 3 / Safety:** 6

## Purpose

The `mod_interface!` DSL has no mechanism for a `use` directive that places an item
exclusively in the private namespace. A `priv use super::child` form would let authors
import a child module without exposing it through any generated layer.

## Context

The test file `tests/inc/derive/use_private_layers/mod.rs` originally contained a
commented-out `mod_interface!` invocation with a `priv use super::child` form alongside
a `// xxx: qqq: make it working` marker. The marker was removed during hygiene cleanup
(2026-05-04); the unimplemented feature is tracked here.

The `use_private_layers` test currently exercises only the absence of a layer directive —
verifying the module compiles without any mod_interface body. Full positive testing of
private-visibility import requires this feature to be implemented first.

## Scope

- `mod_interface_meta/src/impls.rs` — add `private`/`priv` clause-kind handling
- `mod_interface_meta/src/record.rs` — add `RecordUsePrivate` variant
- `mod_interface_meta/src/visibility.rs` — add `ClauseKind::Private`
- `tests/inc/derive/use_private_layers/` — re-enable and expand the trybuild test

## MOST Goals

1. `priv use super::child` directive parses without error
2. Item is accessible in the private namespace only; absent from own, orphan, exposed, prelude
3. Existing tests continue passing

## Notes

- No existing test covers private-visibility directives; no regression risk from adjacent code
- Consider whether `private` and `priv` should both be accepted as aliases
