# Pattern: Autoref Specialization

### Scope

- **Purpose**: Simulate trait-conditional branching on stable Rust without unstable specialization, proc-macros, or runtime overhead.
- **Responsibility**: Documents the autoref specialization pattern — the problem it solves, the resolution mechanism, its applicability, and its tradeoffs.
- **In Scope**: The mechanism by which two competing method implementations produce different boolean results depending on whether a type satisfies a trait bound.
- **Out of Scope**: The specific trait bounds checked (→ api/), the invariants imposed by the pattern (→ invariant/), the user-facing feature built on it (→ feature/).

### Problem

Determining at compile time whether a type satisfies a given trait bound, with the result exposed as a runtime boolean, requires conditional code generation. Stable Rust provides no direct mechanism for this: trait specialization is unstable, const trait evaluation is unstable, and procedural macros add an unwanted dependency. A solution must work on stable Rust with zero external dependencies.

### Solution

Two traits are defined within a local scope — one with an unconditional implementation (always returns false) and one with a conditional implementation (returns true only when the inspected type satisfies the required bound). The unconditional implementation is placed on a reference-wrapped phantom type; the conditional implementation is placed on the phantom type directly. A helper constructs the phantom type from a shared reference to the inspected value. Because Rust's method resolution prefers less-indirected receivers, the conditional implementation is selected when the bound is satisfied and the unconditional implementation is selected otherwise. The result is a compile-time constant emitted as a runtime boolean.

### Applicability

Apply this pattern when all of the following hold:
- The result of a trait bound check must be available as a runtime boolean value.
- Stable Rust is required (no nightly-only features).
- Zero additional dependencies are acceptable.
- The check must not consume or alter the inspected value.

Do not apply when a compile-time constant is needed (const contexts), when checking callable traits on named function items (see invariant/003_fn_trait_limitation.md), or when the check must be reusable across unrelated crates without inlining.

### Consequences

**Benefits**: no dependencies; the boolean result optimizes to a compile-time constant in release builds; the inspected value is never moved; works with any stable trait bound including compound bounds.

**Tradeoffs**: checking callable traits against named function items (as opposed to closures or function pointers) triggers a compile error rather than returning false — this is a structural consequence of how function item types interact with the conditional implementation; the technique is non-obvious to readers unfamiliar with autoref method resolution.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/implements_impl.rs` | Direct implementation of this pattern — the two-trait mechanism and helper function |
| doc | `docs/feature/001_trait_implementation_check.md` | Feature built entirely on this pattern |
| doc | `docs/api/001_implements.md` | Primary macro whose mechanism is this pattern |
| doc | `docs/api/002_instance_of.md` | Alias macro using the same mechanism |
| doc | `docs/invariant/003_fn_trait_limitation.md` | Known consequence of this pattern with callable traits |
