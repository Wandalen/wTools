# Invariant: Zero-Copy Contract

### Scope

- **Purpose**: Guarantee that split and isolation operations return string slices that borrow from the original source, not independently allocated copies.
- **Responsibility**: Defines the zero-copy borrowing invariant and the conditions under which it holds or is deliberately broken.
- **In Scope**: Slice lifetime relationship to source, conditions for owned allocation (stripping, unescaping, number parsing).
- **Out of Scope**: Algorithm selection (`algorithm/`); feature gating (`invariant/002`); SIMD fallback correctness (`invariant/003`).

### Invariant

Split segments and isolation results that do not apply content transformation borrow from the source string slice. Their lifetimes are bound to the source lifetime, and no heap allocation occurs for the content bytes themselves.

This invariant holds when: stripping is disabled, unescaping is disabled, and no postprocessing transform is registered. Under these conditions the yielded slices are subslices of the original input.

The invariant is intentionally broken when: stripping or unescaping is enabled (the content bytes differ from the source bytes), or when the number parsing operation converts bytes to a numeric value (which by definition requires a different representation). In these cases the operation returns an owned value and the caller is responsible for the allocation.

### Sources

- `../../architecture.md` — Memory Efficiency section; zero-copy and `Cow<str>` patterns migrated to this invariant.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `src/string/split.rs` | Split iterator lifetime and slice yield |
| source | `src/string/isolate.rs` | Isolation slice return |
| source | `src/string/zero_copy.rs` | Zero-copy utilities and slice helpers |
| doc | `docs/feature/001_string_splitting.md` | Split feature design |
| doc | `docs/api/001_split_api.md` | Split API contract referencing this invariant |
