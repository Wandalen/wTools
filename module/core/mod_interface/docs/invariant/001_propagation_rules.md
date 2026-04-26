# Invariant: Propagation Rules

### Scope

- **Purpose**: Guarantee that each layer's propagation to parent modules is exactly what the layer contract promises — no more, no less.
- **Responsibility**: Defines the four propagation rules enforced during child-to-parent namespace wiring.
- **In Scope**: Child-to-parent propagation rules for all four generated layers (own, orphan, exposed, prelude).
- **Out of Scope**: Within-module cascade (own ⊇ orphan ⊇ exposed ⊇ prelude); that is governed by the Namespace Cascade invariant in mod_interface_meta docs.

### Invariant Statement

For any child module C wired into parent module P via a layer directive:

1. Items in C's prelude namespace appear in P's prelude namespace.
2. Items in C's exposed namespace appear in P's exposed namespace.
3. Items in C's orphan namespace appear in P's own namespace only — NOT in P's orphan namespace.
4. C itself (the module reference) appears in P's own namespace.
5. Items in C's own namespace do NOT propagate to P at all.

Rule 3 is the critical constraint: orphan propagation stops at the immediate parent's own layer and does not continue further up the hierarchy. This prevents uncontrolled propagation of tightly-coupled items.

### Enforcement Mechanism

The code generation function responsible for wiring child layers into parent namespaces (`record_use_implicit` in `mod_interface_meta/src/impls.rs`) branches on the type of path being processed:

- For simple identifier paths (child modules named in the current scope), it generates all four wiring clauses according to the rules above.
- For paths that already carry an explicit scope qualifier (super::, crate::, or leading ::), it performs layer-aware propagation matching the child's own layer structure.

The branch point is determined by `private_prefix_is_needed()` in `mod_interface_meta/src/use_tree.rs`, which detects whether the path requires a scope prefix to resolve correctly.

### Violation Consequences

Violation of rule 1 or 2 (prelude or exposed not propagated): items expected in ancestor modules become unreachable, causing compile errors in dependent code that imports via the ancestor's prelude or exposed namespace.

Violation of rule 3 (orphan propagated beyond own): tightly-coupled items intended only for the immediate parent appear in grandparent and higher namespaces, expanding the API surface unintentionally and violating library encapsulation boundaries.

Violation of rule 4 (child module not in own): the child module itself becomes inaccessible via the parent's own namespace, breaking direct module path navigation.

Violation of rule 5 (own items propagated): items marked own-only would leak to parent modules, contradicting the explicit no-propagation contract of the own layer.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../mod_interface_meta/src/impls.rs` | `record_use_implicit()`: generates child-to-parent wiring clauses |
| source | `../mod_interface_meta/src/use_tree.rs` | `private_prefix_is_needed()`: detects path type for branch selection |
| test | `../mod_interface_meta/tests/propagation_bug_test.rs` | Verifies propagation rules across all four layers |
| test | `../mod_interface_meta/tests/integration_test.rs` | Integration coverage of full layer wiring |
| doc | `docs/feature/001_layered_module_interface.md` | Feature hub for the mod_interface! macro |
| doc | `../mod_interface_meta/docs/invariant/001_namespace_cascade.md` | Within-module cascade invariant (own ⊇ orphan ⊇ exposed ⊇ prelude) |
| task | `task/completed/001_fix_use_layer_reexports.md` | Root cause and fix for violation of rules 1-4 in record_use_implicit |
