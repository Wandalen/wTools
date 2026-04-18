# Feature: Workspace Management

### Scope

- **Purpose**: Manage multi-crate Rust workspaces: publish crates in dependency order, generate CI/CD workflows, and maintain consistency across crates.
- **Responsibility**: Document the end-to-end workspace management capabilities exposed by the `will` CLI.
- **In Scope**: Multi-crate publishing with topological ordering, version bumping, CI/CD generation, health tables, dependency analysis, test execution with feature combinations, workspace listing and scaffolding.
- **Out of Scope**: Crate development/code generation, git version control operations, crates.io account management, IDE integration.

### Design

willbe manages multi-crate workspaces through five CLI commands targeting different workspace concerns:

| Command | Purpose |
|---------|---------|
| `.publish` | Publish crates in dependency order; supports `dry:1` for plan-only mode |
| `.test` | Run tests with varied feature combinations across workspace crates |
| `.list` | List workspace crates with optional tree format |
| `.cicd.renew` | Regenerate GitHub Actions workflow files for all workspace crates |
| `.readme.health.table.renew` | Update README health/status badge table |

The publish workflow builds a dependency graph, computes a topologically sorted publish order, checks which packages need publishing (local version absent from crates.io), and executes publish + git commit + git push per package.

**Known limitations (won't fix — legacy crate)**:

- `subgraph()` in `tool/graph.rs:212` follows outgoing edges (dependency direction) instead of incoming edges (dependent direction), producing a superset of packages. See `task/backlog/001_wrong_publish_set_graphs_tools.md`.
- `remove_not_required_to_publish()` cascade at `tool/graph.rs:323` skips `publish_need()` for packages whose dependency is already in the publish set, adding packages unconditionally.
- `publish_need()` at `entity/package.rs:251` checks whether the local version is on crates.io rather than whether the required version is available.
- `detect_stale_dependencies()` at `entity/staleness.rs:238` marks all dependents of a being-published package as stale regardless of version compatibility.

### Cross-References

| Type | File | Responsibility |
|------|------|----------------|
| source | `../../src/action/publish.rs` | Publish action: plan construction and execution |
| source | `../../src/tool/graph.rs` | Dependency graph: subgraph DFS, remove_not_required_to_publish |
| source | `../../src/entity/package.rs` | Package entity: publish_need() oracle |
| source | `../../src/entity/staleness.rs` | Staleness detection: detect_stale_dependencies() |
| source | `../../src/command/publish.rs` | CLI: .publish command definition |
| doc | `../../task/backlog/001_wrong_publish_set_graphs_tools.md` | Known algorithmic bugs (won't fix, legacy) |
| doc | `002_enhanced_publish_algorithm.md` | Planned staleness-aware algorithm (not implemented) |
