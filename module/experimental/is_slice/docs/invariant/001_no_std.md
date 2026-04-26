# Invariant: No Standard Library Required

### Scope

- **Purpose**: Guarantee that is_slice is usable in any Rust environment, including embedded systems, kernels, and other no_std targets, by relying exclusively on the core library.
- **Responsibility**: Documents the no_std invariant — its precise statement, how it is enforced, and what breaks if it is violated.
- **In Scope**: The set of library facilities used by the is_slice macro in its public implementation.
- **Out of Scope**: Dev-only test dependencies, the caller's environment constraints.

### Invariant Statement

For all versions: the crate is unconditionally marked no_std at the crate root. The is_slice macro uses only the zero-sized marker type from the core library — no allocator, no collections, no I/O. The crate introduces no standard library dependency into any project that depends on it.

### Enforcement Mechanism

The no_std attribute is applied unconditionally at the crate root, not behind a feature flag. This means the compiler enforces the constraint for all build configurations — there is no opt-in or opt-out path that could accidentally introduce a standard library dependency. The macro body references only the core marker module by its fully qualified core path. Any accidental use of a std-only type would produce a compile-time error when building in a no_std context.

### Violation Consequences

Removing the no_std attribute or introducing any standard library dependency would silently exclude the crate from embedded and kernel targets. Callers on those targets would receive a compile error with no clear indication that the invariant changed. Any project that selected this crate over alternatives specifically for its no_std property would be broken.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/lib.rs` | Unconditional no_std declaration at crate root |
| doc | `docs/feature/001_slice_detection.md` | Feature whose portability this invariant enables |
| doc | `docs/api/001_is_slice.md` | API whose compatibility guarantee depends on this invariant |
