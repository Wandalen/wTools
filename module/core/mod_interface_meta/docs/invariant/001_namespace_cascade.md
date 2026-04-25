# Invariant: Namespace Cascade

### Scope

- **Purpose**: Guarantee that items exported to a lower visibility layer are always accessible from all higher layers.
- **Responsibility**: Defines the cascade propagation rule enforced by the generated namespace modules.
- **In Scope**: Layer ordering, re-export structure, and cascade direction.
- **Out of Scope**: Which items are assigned to which layer; that is governed by the mod_interface! DSL directives.

### Invariant Statement

For every item I declared with a layer directive, I is accessible from all layers of equal or higher visibility. Layer ordering from lowest to highest: `prelude ≤ exposed ≤ orphan ≤ own`. Equivalently: `own` ⊇ `orphan` ⊇ `exposed` ⊇ `prelude`.

### Enforcement Mechanism

Each generated namespace module contains a wildcard re-export of the module directly below it in the hierarchy:
- `own` re-exports `orphan`
- `orphan` re-exports `exposed`
- `exposed` re-exports `prelude`

All four modules are generated unconditionally for every `mod_interface!` invocation. The cascade is enforced at compile time by this structural re-export chain; no runtime logic is involved.

### Violation Consequences

If any re-export in the chain were absent, items declared at a lower layer would become inaccessible at higher layers. An `orphan use X` declaration would be invisible from `own` — silently shrinking the public API surface of any crate using the module, with no compile error to indicate the regression.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/impls.rs` | Generates the cascade re-exports in the four namespace module declarations |
| source | `src/visibility.rs` | Layer kind classification and valid name enumeration |
| test | `tests/propagation_bug_test.rs` | Verifies cascade propagation correctness across all four layers |
| doc | `docs/api/001_mod_interface_macro.md` | DSL operations whose semantics depend on this cascade |
| doc | `docs/feature/001_mod_interface.md` | Feature hub for the mod_interface! macro |

