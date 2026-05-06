# Pattern: Absorption Pattern

### Scope

- **Purpose**: Resolve the bootstrap constraint preventing a proc-macro crate from using its own macro while keeping a single user-facing import point.
- **Responsibility**: Documents the two-crate split problem, solution structure, applicability, and consequences.
- **In Scope**: Crate boundary design for proc-macro crates with user-facing facades.
- **Out of Scope**: Code generation internals; those belong in the meta crate's own documentation.

### Problem

A procedural macro crate cannot invoke its own macro — the macro is not yet available at the time the crate itself is compiled. This creates a bootstrap constraint:
- The macro implementation must live in a proc-macro crate (a special crate type with compiler access).
- That proc-macro crate cannot use `mod_interface!` to organize its own modules.
- Yet users should import a single crate and not need to know about the internal split.

Without a solution, users either import the proc-macro crate directly (exposing internal structure) or work around the bootstrap constraint with fragile in-crate hacks.

### Solution

Split into two crates with distinct roles:

1. **Meta crate** (`mod_interface_meta`): contains the proc-macro implementation. Crate type is `proc-macro`. Organizes its own modules conventionally (no mod_interface!). Not intended for direct use.

2. **Runtime facade crate** (`mod_interface`): re-exports the macro from the meta crate. Provides the canonical import point. No logic of its own. Users depend on this crate only.

The runtime crate "absorbs" the meta crate — making it invisible to users who simply import `mod_interface` and use `mod_interface!`.

### Applicability

Apply this pattern when:
- A proc-macro crate needs to expose a public API that users import by a clean name.
- The macro itself would be used to organize the crate if the bootstrap constraint did not apply.
- Hiding the implementation crate from end users is desirable.

This pattern applies to all proc-macro crates in the wTools workspace.

### Consequences

**Benefits**:
- Users see one crate with one name; the implementation split is invisible.
- Meta crate can evolve independently without affecting the user-facing API contract.
- Circular dependency prevented: meta crate has no dependency on the facade.

**Liabilities**:
- Two crates must be versioned in lockstep; a breaking change in the meta crate requires a major bump in both.
- Contribution requires understanding the two-crate structure before editing the proc-macro.
- Proc-macro crate type restrictions mean the meta crate cannot use `mod_interface!` for its own organization.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Runtime facade; re-exports mod_interface! from mod_interface_meta |
| source | `../mod_interface_meta/src/lib.rs` | Proc-macro entry point in the meta crate |
| doc | `docs/feature/001_layered_module_interface.md` | Feature hub noting the bootstrap constraint and absorption pattern |
| doc | `docs/pattern/001_exposure_level_cascade.md` | Five-layer cascade that motivates this two-crate structure |
