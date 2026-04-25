# Invariant: Box-Only Restriction

### Scope

- **Purpose**: Constrain the clone_dyn ecosystem to `Box<dyn Trait>` exclusively.
- **Responsibility**: Prevent scope creep to `Rc<dyn Trait>`, `Arc<dyn Trait>`, or other smart pointers.
- **In Scope**: `Box<dyn Trait>` in all four `Send`/`Sync` variants.
- **Out of Scope**: `Rc`, `Arc`, raw pointers, pinned pointers, or any smart pointer other than `Box`.

### Invariant Statement

All `Clone` implementations generated or supported by the `clone_dyn` ecosystem operate exclusively on `Box<dyn Trait + [Send] + [Sync]>`. No other smart pointer type is supported.

### Enforcement Mechanism

- The `#[clone_dyn]` macro only emits `impl Clone for Box<dyn ...>` — the `Box` wrapper is hardcoded in the code generation logic in `clone_dyn_meta`.
- `clone_into_box` returns `Box<T>` — its return type enforces `Box` at the type level.
- Any attempt to use `Rc<dyn Trait>` results in a compile error (no `Clone` impl is generated).

### Violation Consequences

Adding `Rc`/`Arc` support would require: new generated impl variants in the macro, new `clone_into_rc`/`clone_into_arc` functions in `clone_dyn_types`, and a semver-breaking change. This must be a deliberate versioned decision, not an accidental addition.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| feature | `../feature/001_macro_usage.md` | Macro that generates Box-only impls |
| api | `../api/001_facade_api.md` | Public surface constrained to Box |
