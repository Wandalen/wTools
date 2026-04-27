# Feature: Enhanced Publish Algorithm

> **Status: Not Implemented — Legacy Crate**
> This feature was fully specified (2025-11-08, spec v2.0) but never implemented. willbe is a legacy crate; this enhancement will not be built. The specification is preserved here for historical reference and to inform any future replacement tool.

### Scope

- **Purpose**: Correct the current publish algorithm by adding dependency staleness detection and version-aware cascade publishing to avoid the wrong-publish-set bug.
- **Responsibility**: Document the planned 5-phase algorithm that would replace the current publish implementation.
- **In Scope**: Phase 1 (initial detection), Phase 2 (staleness detection), Phase 3 (transitive closure), Phase 4 (topological sort), Phase 5 (publish execution); correct `subgraph` direction; correct `publish_need` oracle.
- **Out of Scope**: Implementation — willbe is a legacy crate. See task/backlog/001 for the documented bugs and Won't Fix rationale.

### Design

The algorithm addresses four interacting defects in the current implementation (documented in `task/backlog/001_wrong_publish_set_graphs_tools.md`):

**Phase 1 — Initial Detection**: Detect packages with local changes or explicit version bumps. This is the only phase the current implementation handles correctly.

**Phase 2 — Dependency Staleness Detection**: For each package P not in Phase 1 output, check all dependencies D. Mark P stale if D is being published OR if the workspace version of D does not satisfy P's requirement for D. Current implementation uses a being-published state flag but ignores version compatibility.

**Phase 3 — Transitive Closure**: Repeat Phase 2 until no new packages are added. Current implementation uses `compute_transitive_closure` which is structurally correct but feeds from an already-inflated Phase 1 set.

**Phase 4 — Topological Sort**: Order the publish set using reverse topological sort of the dependency graph. This phase is correct in the current implementation.

**Phase 5 — Publish Execution**: Execute publish operations in order with git commit/push per package. This phase is correct in the current implementation.

**Correct `subgraph` direction**: The fixed algorithm traverses incoming edges (dependents, not dependencies) from the root package. The current implementation traverses outgoing edges, producing ~20 packages when ~4 are required.

**Correct `publish_need` oracle**: The fixed oracle asks "does crates.io have a version satisfying the requirement?" The current implementation asks "is the local version already on crates.io?" — different questions with different answers.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/action/publish.rs` | Current publish action (buggy, won't fix) |
| source | `../../src/tool/graph.rs` | Current graph tools (Defect 1 + 2 locations) |
| source | `../../src/entity/package.rs` | Current publish_need (Defect 3 location) |
| source | `../../src/entity/staleness.rs` | Current staleness detection (Defect 4 location) |
| doc | [feature/001_workspace_management.md](001_workspace_management.md) | Current implementation behavior and known bugs |
| doc | [api/001_cli_interface.md](../api/001_cli_interface.md) | CLI publish command reference |
| task | `../../task/backlog/001_wrong_publish_set_graphs_tools.md` | Root cause analysis of wrong publish set (Won't Fix) |
