# Explore type-checking marker traits for collection type discrimination

## Execution State

- **Executor Type:** any
- **Actor:** null
- **Claimed At:** null
- **Status:** 📥 (Backlog)

## Goal

Evaluate whether `collection_tools` should provide pre-defined marker traits (e.g., `IsSlice`, `IsVec`, `IsHashMap`) that enable compile-time type-shape checks via `implements!`, and if justified, implement a minimal set behind a feature flag (Motivated: `is_slice` crate is being deprecated as a redundant single-purpose crate; its functionality is achievable with `implements!` + a 2-line marker trait, but no pre-built traits exist anywhere in the workspace; Observable: new `type_markers` feature in `collection_tools` with marker traits + integration tests; Scoped: marker traits only for types `collection_tools` already handles, no changes to `implements!`; Testable: `cargo test -p collection_tools --features type_markers`).

The `is_slice!` macro duplicates the trait-specialization mechanism that `implements!` already provides generically. A 2-line marker trait + `implements!` call produces identical results. However, the workspace currently offers zero pre-built marker traits, forcing every consumer to define their own.

This task explores whether a curated set of marker traits in `collection_tools` provides meaningful value or violates YAGNI. The decision gate is: do any workspace crates or realistic external consumers need compile-time type-shape checks for collection types? Current evidence shows zero usage of `is_slice!` outside its own tests, and zero `is_vec!`/`is_hashmap!` calls anywhere. The task must produce a concrete recommendation: implement, defer, or reject.

## In Scope

- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/src/` — new marker traits module (if implementing)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/Cargo.toml` — new `type_markers` feature flag (if implementing)
- `/home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/` — integration tests showing `implements!( expr => IsVec )` pattern (if implementing)
- Marker traits for types collection_tools already handles: `&[T]`, `Vec<T>`, `HashMap<K,V>`, `HashSet<T>`, `BTreeMap<K,V>`, `BTreeSet<T>`, `LinkedList<T>`, `VecDeque<T>`, `BinaryHeap<T>`
- Usage audit: search workspace for real need (type discrimination patterns)
- Decision document: implement vs defer vs reject

## Out of Scope

- Changes to the `implements` crate (its API is fixed)
- Changes to `inspect_type` (different domain: runtime debugging)
- Deprecation of `is_slice` (tracked separately in `doc/layers.md`)
- Marker traits for non-collection types (String, Box, Rc, Arc, etc.)
- Runtime type checking mechanisms (TypeId, Any, downcast)
- Documentation updates to `is_slice` docs/

## Requirements

- All work must strictly adhere to all applicable rulebooks (discover via `kbase .rulebooks`)
- Marker traits (if created) must be `no_std` compatible — use only `core` types
- Marker traits must be behind an opt-in feature flag (`type_markers`) to avoid bloating default builds
- Each marker trait must be implementable in 2-3 lines (trait definition + blanket impl)
- Integration tests must demonstrate usage with `implements!` crate, not standalone
- YAGNI gate: if usage audit finds zero concrete consumers, the recommendation must be "defer" unless a compelling external-consumer argument exists

## Work Procedure

Execute in order. Do not skip or reorder steps.

1. **Read rulebooks** — `kbase .rulebooks`; note `code_design.rulebook.md` constraints on feature flags and `crate_distribution.rulebook.md` on crate scope.
2. **Read documentation** — Read `collection_tools/docs/` and `implements/docs/api/001_implements.md` to understand current API contracts.
3. **Audit usage patterns** — Search workspace for all compile-time type discrimination patterns: `is_slice!`, `implements!`, `TypeId::of`, manual type matching. Catalog real call sites, not test/example code.
4. **Write Test Matrix** — Define input scenarios for each proposed marker trait (see Test Matrix below).
5. **Design marker trait module** — Draft `src/type_markers.rs` with trait definitions. Each trait: `trait IsX {} impl<T> IsX for X<T> {}`. Ensure `no_std` compatibility.
6. **Write failing tests** — Create `tests/type_marker_tests.rs` with `implements!` integration tests for each marker trait. Tests should fail (traits don't exist yet).
7. **Implement marker traits** — Add `type_markers` feature to `Cargo.toml`, create `src/type_markers.rs`, wire into `mod_interface`.
8. **Validate** — Run `clear && RUSTFLAGS="-D warnings" cargo nextest run -p collection_tools --all-features && RUSTDOCFLAGS="-D warnings" cargo test -p collection_tools --doc --all-features && cargo clippy -p collection_tools --all-targets --all-features -- -D warnings`.
9. **Write decision document** — If usage audit (step 3) found zero consumers, document the recommendation as "defer" and skip steps 5-8. Create `-decision_type_markers.md` with evidence and rationale.
10. **Walk Validation Checklist** — check every item below. Every answer must be YES.

## Test Matrix

| Input Scenario | Config Under Test | Expected Behavior |
|----------------|-------------------|-------------------|
| `&[1,2,3][..]` (slice ref) | `implements!( expr => IsSlice )` | Returns `true` |
| `&[1,2,3]` (array ref) | `implements!( expr => IsSlice )` | Returns `false` |
| `vec![1,2,3]` (owned Vec) | `implements!( expr => IsVec )` | Returns `true` |
| `&[1,2,3]` (not a Vec) | `implements!( expr => IsVec )` | Returns `false` |
| `hmap!{ 1 => 2 }` (HashMap) | `implements!( expr => IsHashMap )` | Returns `true` |
| `bmap!{ 1 => 2 }` (BTreeMap) | `implements!( expr => IsHashMap )` | Returns `false` |
| `hset!{ 1, 2 }` (HashSet) | `implements!( expr => IsHashSet )` | Returns `true` |
| `bset!{ 1, 2 }` (BTreeSet) | `implements!( expr => IsBTreeSet )` | Returns `true` |
| `"hello"` (str ref) | `implements!( expr => IsSlice )` | Returns `false` |
| Empty `Vec::<i32>::new()` | `implements!( expr => IsVec )` | Returns `true` |

## Acceptance Criteria

- Usage audit produces a catalog of all compile-time type discrimination call sites in the workspace with file paths and line numbers
- A concrete recommendation (implement / defer / reject) is documented with evidence
- If implementing: all 9 marker traits compile and pass `implements!` integration tests under `--features type_markers`
- If implementing: `collection_tools` compiles clean without `type_markers` feature (no regressions to default build)
- If implementing: `cargo clippy -p collection_tools --all-features -- -D warnings` reports zero warnings
- If deferring: decision document explains why with reference to YAGNI and usage audit data

## Validation

### Checklist

Desired answer for every question is YES.

**Usage Audit**
- [ ] Does the usage audit cover all active crates (module/core/ + module/experimental/)?
- [ ] Does the audit catalog include file paths and line numbers for every type-discrimination call site?
- [ ] Is the audit data used as evidence in the decision document?

**Design (if implementing)**
- [ ] Is each marker trait defined in 2-3 lines (trait + impl)?
- [ ] Are all marker traits `no_std` compatible (use only `core` types)?
- [ ] Is the `type_markers` feature opt-in (not in `default` or `full`)?
- [ ] Does `collection_tools` compile clean with `default-features = false`?

**Tests (if implementing)**
- [ ] Does every Test Matrix row have a corresponding test case?
- [ ] Do tests use `implements!` from the `implements` crate (not inline trait tricks)?
- [ ] Do tests cover both positive (returns true) and negative (returns false) cases?

**Out of Scope confirmation**
- [ ] Is the `implements` crate unchanged?
- [ ] Is `inspect_type` unchanged?
- [ ] Are no marker traits defined for non-collection types (String, Box, Rc, Arc)?

### Measurements

**M1 — Test count**
Command: `cargo nextest run -p collection_tools --features type_markers 2>&1 | grep 'passed'`
Before: 0 type_marker tests exist. Expected: 10+ tests pass (if implementing) or 0 (if deferring). Deviation: fewer tests than Test Matrix rows.

**M2 — Feature isolation**
Command: `cargo check -p collection_tools 2>&1 | tail -1`
Before: compiles clean. Expected: still compiles clean without `type_markers`. Deviation: compilation error.

### Invariants

- [ ] I1 — test suite: `clear && RUSTFLAGS="-D warnings" cargo nextest run -p collection_tools --all-features && cargo clippy -p collection_tools --all-targets --all-features -- -D warnings` exits 0

### Anti-faking checks

**AF1 — Marker traits use implements! not inline tricks**
Check: `grep -c "implements!" /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/tests/type_marker_tests.rs`
Expected: 10+ (if implementing). Why: ensures tests prove integration with `implements!`, not standalone trait tricks that bypass the architecture.

**AF2 — Feature flag exists and is opt-in**
Check: `grep -c "type_markers" /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/Cargo.toml`
Expected: 1+ (if implementing). Why: confirms the feature exists and isn't silently merged into `default`.

**AF3 — YAGNI gate was applied**
Check: `ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/task/-decision_type_markers.md 2>/dev/null || ls /home/user1/pro/lib/wip_core/wtools/dev/module/core/collection_tools/task/backlog/-decision_type_markers.md 2>/dev/null`
Expected: file exists (regardless of implement/defer decision). Why: ensures the usage audit evidence was captured before any implementation.

## Outcomes

[Empty — populated upon task completion]
