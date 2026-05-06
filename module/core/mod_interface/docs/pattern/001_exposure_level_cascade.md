# Pattern: Exposure Level Cascade

### Scope

- **Purpose**: Provide a reusable structural pattern for organizing module namespaces into a fixed hierarchy with downward cascade.
- **Responsibility**: Documents the five-layer cascade problem, solution structure, applicability, and consequences.
- **In Scope**: Layer ordering, cascade direction, and the architectural rationale for fixed versus configurable levels.
- **Out of Scope**: Code generation mechanics; those belong in algorithm/ doc instances.

### Problem

Library modules typically expose items either fully publicly (to all callers) or privately (internal only). This binary choice is insufficient for complex hierarchies where:
- Some items should be visible only to the immediate parent module but not grandparents.
- Some items should propagate freely to all ancestors.
- Some items are intended for glob import at the call site.
- Some items must not leave the module at all.

Standard visibility modifiers do not support hierarchical propagation — they express scope (crate, super, absolute path) but not propagation intent through module hierarchies.

### Solution

Define five fixed visibility layers forming a cascade from most to least restrictive:

1. **Private** — user-written implementation namespace; never propagated; not generated.
2. **Own** — generated namespace; items accessible only within this module.
3. **Orphan** — generated namespace; items propagated to the immediate parent only.
4. **Exposed** — generated namespace; items propagated to all ancestors.
5. **Prelude** — generated namespace; items propagated to all ancestors and intended for glob import.

The cascade flows upward: each generated namespace re-exports everything from the one below it. A prelude item is therefore accessible from all four generated namespaces (prelude, exposed, orphan, own). An own item is accessible only from own.

The layer count and names are fixed across all modules. This intentional constraint ensures every module in a codebase using this pattern has identical structure, enabling predictable navigation and tooling.

### Applicability

Apply this pattern when:
- A library crate has multi-level module hierarchies.
- Different callers at different levels of the hierarchy need different visibility.
- Consistent module structure across the entire codebase is more valuable than per-module customization.

Do not apply when:
- The module is a procedural macro implementation (bootstrap constraint prevents self-application).
- The module is a simple single-file utility with no sub-modules.
- Per-module customization of visibility levels is required.

### Consequences

**Benefits**:
- Predictable API surface: developers know what to expect from any module following this pattern.
- Explicit propagation control: no items leak to ancestors unintentionally.
- Conventional structure: once learned, the pattern applies everywhere in the codebase.

**Liabilities**:
- Learning curve: developers must understand all five layers before contributing.
- Bootstrap constraint: the macro implementation crate cannot use its own macro (addressed by the Absorption Pattern).
- Verbosity overhead: simple modules with few items still generate the full four-namespace structure.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../mod_interface_meta/src/impls.rs` | Generates the four namespace modules implementing the cascade |
| source | `../mod_interface_meta/src/visibility.rs` | ClauseKind enum defining the four layer values |
| doc | `docs/feature/001_layered_module_interface.md` | Feature hub for the mod_interface! macro |
| doc | `docs/invariant/001_propagation_rules.md` | Child-to-parent propagation invariants for the cascade |
| doc | `../mod_interface_meta/docs/invariant/001_namespace_cascade.md` | Within-module cascade invariant (own ⊇ orphan ⊇ exposed ⊇ prelude) |
