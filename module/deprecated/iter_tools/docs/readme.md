# Docs

### Scope

Design and API documentation for `iter_tools`.

### Responsibility Table

| File | Responsibility |
|------|----------------|
| `api/` | Formal public API contracts for traits, extensions, and re-exports. |
| `feature/` | User-facing capability documentation with design rationale. |
| `invariant/` | Non-negotiable behavioral contracts and enforcement details. |
| `entities.md` | Master registry of all doc entities and instances. |
| `doc_graph.yml` | Directed graph of cross-references across all doc instances. |

### System Actors

| Actor | Role |
|-------|------|
| crate author | Writes and maintains `iter_tools` source code. |
| API consumer | Uses `iter_tools` in downstream Rust crates. |
| workspace maintainer | Manages workspace-level dependency versions. |
| documentation author | Writes and updates doc instances in `docs/`. |
| code reviewer | Reviews PRs for correctness and doc compliance. |
| test author | Writes automated and manual test cases for `iter_tools`. |
| CI pipeline | Runs tests, clippy, and doc checks against all feature combinations. |
| rulebook system | Validates doc structure and format via `kbase .validate`. |

### Vocabulary

| Term | Definition |
|------|------------|
| `_IterTrait` | Infrastructure base trait with `Iterator + ExactSizeIterator + DoubleEndedIterator + CloneDyn` bounds; underscore prefix signals internal use. |
| `IterTrait` | User-facing supertrait of `_IterTrait` that additionally requires `Clone`. |
| `BoxedIter` | Type alias for `Box<dyn _IterTrait<'a, T> + 'a>`; the primary heap-allocated iterator type. |
| `IterExt` | Blanket extension trait providing `map_result` to any `Clone + Iterator`. |
| `CloneDyn` | Object-safe clone mechanism from `clone_dyn_types` enabling `Clone` on boxed trait objects. |
| manual namespace chain | The `own → orphan → exposed → prelude` re-export pattern implemented directly in source to avoid a circular `mod_interface` dependency. |
